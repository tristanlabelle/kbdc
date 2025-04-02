use std::collections::{BTreeMap, HashMap};
use std::mem::transmute;
use std::ptr::null;
use windows_sys::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

use crate::model::*;

type FnKbdLayerDescriptor = extern "system" fn() -> *const KBDTABLES;


pub fn read_keyboard(path: String) -> KeyboardDesc {
    unsafe {
        let module = LoadLibraryW(string_to_rgwsz(path).as_ptr());
        if module.is_null() {
            panic!("Failed to load library");
        }

        let proc: FnKbdLayerDescriptor = transmute(GetProcAddress(module, b"KbdLayerDescriptor\0".as_ptr()).unwrap());
        let descriptor_ptr = proc();

        let mut descriptor = KeyboardDesc::new();
        descriptor.physical_keys = read_physical_keys(descriptor_ptr);
        descriptor.virtual_keys = read_virtual_keys(descriptor_ptr);
        descriptor.dead_keys = read_dead_keys(descriptor_ptr);

        let locale_flags = descriptor_ptr.deref().fLocaleFlags;
        descriptor.version = (locale_flags >> 16) as u16;
        descriptor.supports_altgr = (locale_flags & KLLF_ALTGR) != 0;
        descriptor.supports_shift_lock = (locale_flags & KLLF_SHIFTLOCK) != 0;
        descriptor.supports_directionality = (locale_flags & KLLF_LRM_RLM) != 0;

        if descriptor_ptr.deref().nLgMax > 0 {
            panic!("Ligatures are not implemented.")
        }

        descriptor.type_value = descriptor_ptr.deref().dwType;
        descriptor.subtype_value = descriptor_ptr.deref().dwSubType;

        descriptor
    }
}

unsafe fn read_physical_keys(descriptor_ptr: *const KBDTABLES) -> BTreeMap<ScanCode, PhysicalKeyDesc> {
    let mut result: BTreeMap<ScanCode, PhysicalKeyDesc> = BTreeMap::new();

    unsafe {
        for scan_code in 0..descriptor_ptr.deref().bMaxVSCtoVK {
            let virtual_key_bits = descriptor_ptr.deref().pusVSCtoVK.offset(scan_code as isize).read();
            let (virtual_key, virtual_key_flags) = VirtualKey::from_extended_bits(virtual_key_bits);
            if virtual_key.code == 0xFF { continue }
            result.insert(ScanCode::Unescaped(scan_code as u8), PhysicalKeyDesc {
                virtual_key: virtual_key,
                virtual_key_flags: virtual_key_flags,
                name: None
            });
        }

        for row_ptr in table(descriptor_ptr.deref().pVSCtoVK_E0, |row_ptr| row_ptr.deref().Vsc != 0) {
            let (virtual_key, virtual_key_flags) = VirtualKey::from_extended_bits(row_ptr.deref().Vk);
            result.insert(ScanCode::Extended0(row_ptr.deref().Vsc), PhysicalKeyDesc {
                virtual_key: virtual_key,
                virtual_key_flags: virtual_key_flags,
                name: None
            });
        }

        for row_ptr in table(descriptor_ptr.deref().pVSCtoVK_E1, |row_ptr| row_ptr.deref().Vsc != 0) {
            let (virtual_key, virtual_key_flags) = VirtualKey::from_extended_bits(row_ptr.deref().Vk);
            result.insert(ScanCode::Extended1(row_ptr.deref().Vsc), PhysicalKeyDesc {
                virtual_key: virtual_key,
                virtual_key_flags: virtual_key_flags,
                name: None
            });
        }

        // Populate physical key names
        for row_ptr in table(descriptor_ptr.deref().pKeyNames, |row_ptr| row_ptr.deref().vsc != 0) {
            let Some(entry_ref) = result.get_mut(&ScanCode::Unescaped(row_ptr.deref().vsc)) else {
                continue
            };
            entry_ref.name = Some(pwsz_to_string(row_ptr.deref().pwsz));
        }

        for row_ptr in table(descriptor_ptr.deref().pKeyNamesExt, |row_ptr| row_ptr.deref().vsc != 0) {
            let Some(entry_ref) = result.get_mut(&ScanCode::Extended0(row_ptr.deref().vsc)) else {
                continue
            };
            entry_ref.name = Some(pwsz_to_string(row_ptr.deref().pwsz));
        }
    }

    result
}

unsafe fn read_virtual_keys(descriptor_ptr: *const KBDTABLES) -> HashMap<VirtualKey, KeyEffect> {
    let mut result: HashMap<VirtualKey, KeyEffect> = HashMap::new();

    unsafe {
        // Populate modifier virtual keys
        let modifiers_ptr = descriptor_ptr.deref().pCharModifiers;
        for row_ptr in table(modifiers_ptr.deref().pVkToBit, |row_ptr| row_ptr.deref().Vk != 0) {
            result.insert(VirtualKey { code: row_ptr.deref().Vk }, KeyEffect::Modifier(KeyModifiers::from_bits(row_ptr.deref().ModBits)));
        }

        // Build modification number -> modifiers mapping
        let mut mod_numbers_to_mods: HashMap<u8, KeyModifiers> = HashMap::new();
        for modifier_bits in 0..(modifiers_ptr.deref().wMaxModBits + 1) {
            let mod_number = modifiers_ptr.deref().ModNumber.as_ptr().add(modifier_bits as usize).read();
            if mod_number as u32 == SHFT_INVALID { continue }
            mod_numbers_to_mods.insert(mod_number, KeyModifiers::from_bits(modifier_bits as u8));
        }

        // Populate virtual keys which type stuff
        for tables_row_ptr in table(descriptor_ptr.deref().pVkToWcharTable, |row_ptr| !row_ptr.deref().pVkToWchars.is_null()) {
            let key_mod_count = tables_row_ptr.deref().nModifications;
            let mut table_row_iterator = strided_table(
                tables_row_ptr.deref().pVkToWchars,
                tables_row_ptr.deref().cbSize as usize,
                |row_ptr| row_ptr.deref().VirtualKey != 0);
            loop {
                let Some(table_row_ptr) = table_row_iterator.next() else { break };

                // Read attributes
                let attribute_bits = table_row_ptr.deref().Attributes as u32;
                let mut key_typing = KeyTyping {
                    by_modifiers: HashMap::new(),
                    caps_lock_as_shift: (attribute_bits & CAPLOK) != 0,
                    caps_lock_as_uppercase: (attribute_bits & SGCAPS) != 0,
                    caps_lock_altgr_as_shift: (attribute_bits & CAPLOKALTGR) != 0,
                    kana_support: (attribute_bits & KANALOK) != 0,
                    grpseltap_support: (attribute_bits & GRPSELTAP) != 0,
                };

                let chars_ptr = table_row_ptr.deref().wch.as_ptr();

                // Read chars for each modifier
                let mut dead_row_chars_ptr: *const u16 = null();
                for mod_number in 0..key_mod_count {
                    let char = chars_ptr.add(mod_number as usize).read();
                    if char as u32 == WCH_NONE { continue }

                    let modifiers = mod_numbers_to_mods[&mod_number];

                    if char as u32 == WCH_DEAD {
                        // Read the dead row if we haven't already
                        if dead_row_chars_ptr.is_null() {
                            let dead_row_ptr = table_row_iterator.next();
                            if dead_row_ptr.is_none() || dead_row_ptr.unwrap().deref().VirtualKey != 0xFF {
                                panic!("Malformed virtual key to dead key mapping.")
                            }

                            dead_row_chars_ptr = dead_row_ptr.unwrap().deref().wch.as_ptr()
                        }

                        let dead_char = dead_row_chars_ptr.add(mod_number as usize).read();
                        key_typing.by_modifiers.insert(modifiers, TypingEffect::DeadKey(dead_char));
                        continue
                    }

                    if char as u32 == WCH_LGTR {
                        panic!("Ligatures are not implemented.")
                    }

                    key_typing.by_modifiers.insert(modifiers, TypingEffect::Char(char));
                }

                result.insert(VirtualKey { code: table_row_ptr.deref().VirtualKey }, KeyEffect::Typing(key_typing));
            }
        }
    }

    result
}

pub fn read_dead_keys(descriptor_ptr: *const KBDTABLES) -> HashMap<u16, DeadKeyDesc> {
    let mut result: HashMap<u16, DeadKeyDesc> = HashMap::new();

    unsafe {
        // Populate dead key combos
        for row_ptr in table(descriptor_ptr.deref().pDeadKey, |row_ptr| row_ptr.deref().wchComposed != 0) {
            let accent_and_base_char = row_ptr.deref().dwBoth;
            let base_char = (accent_and_base_char & 0xFFFF) as u16;
            let accent_char = (accent_and_base_char >> 16) as u16;

            let dead_key = result.entry(accent_char)
                .or_insert(DeadKeyDesc { name: None, combos: HashMap::new() });

            dead_key.combos.insert(base_char, DeadKeyCombo {
                composed_char: row_ptr.deref().wchComposed,
                flags: row_ptr.deref().uFlags,
            });
        }

        // Populate dead key names
        for row_ptr in table(descriptor_ptr.deref().pKeyNamesDead, |row_ptr| !row_ptr.deref().is_null()) {
            let pwsz = row_ptr.deref();
            let accent_char = pwsz.read();
            let name = pwsz_to_string(pwsz.add(1));

            let dead_key = result.entry(accent_char)
                .or_insert(DeadKeyDesc { name: None, combos: HashMap::new() });
            dead_key.name = Some(name);
        }
    }

    result
}

trait PtrDeref<T>{
    unsafe fn deref<'x>(self) -> &'x T;
}

impl<T> PtrDeref<T> for *const T{
    unsafe fn deref<'x>(self) -> &'x T{
        unsafe { &*self }
    }
}

struct TableIterator<V> {
    row: *const V,
    first: bool,
    stride: usize,
    predicate: fn(*const V) -> bool,
}

impl<V> Iterator for TableIterator<V> {
    type Item = *const V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.row.is_null() {
                return None;
            }

            if !(self.predicate)(self.row) {
                return None;
            }

            if self.first {
                self.first = false;
                return Some(self.row)
            }

            self.row = self.row.byte_add(self.stride);
            if !(self.predicate)(self.row) {
                return None;
            }

            Some(self.row)
        }
    }
}

fn table<V>(start: *const V, predicate: fn(*const V) -> bool) -> TableIterator<V> {
    TableIterator {
        row: start,
        first: true,
        stride: size_of::<V>(),
        predicate
    }
}

fn strided_table<V>(start: *const V, stride: usize, predicate: fn(*const V) -> bool) -> TableIterator<V> {
    TableIterator {
        row: start,
        first: true,
        stride: stride,
        predicate
    }
}

fn string_to_rgwsz(str: String) -> Vec<u16> {
    let mut utf16: Vec<u16> = str.encode_utf16().collect();
    utf16.push(0);
    return utf16
}

unsafe fn pwsz_to_string(ptr: *const u16) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf16_lossy(slice)
    }
}
