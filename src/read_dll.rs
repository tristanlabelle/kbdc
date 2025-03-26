use std::collections::HashMap;
use std::mem::transmute;
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

        descriptor
    }
}

unsafe fn read_physical_keys(descriptor_ptr: *const KBDTABLES) -> HashMap<ScanCode, PhysicalKeyDesc> {
    let mut result: HashMap<ScanCode, PhysicalKeyDesc> = HashMap::new();

    unsafe {
        for scan_code in 0..(*descriptor_ptr).bMaxVSCtoVK {
            let virtual_key_code = (*descriptor_ptr).pusVSCtoVK.offset(scan_code as isize).read();
            if virtual_key_code == 0xFF { continue }
            result.insert(ScanCode::Unescaped(scan_code as u8), PhysicalKeyDesc {
                virtual_key: VirtualKey { code: virtual_key_code as u8 },
                name: None
            });
        }

        for entry in table((*descriptor_ptr).pVSCtoVK_E0, |entry| entry.Vsc != 0) {
            result.insert(ScanCode::Extended0(entry.Vsc), PhysicalKeyDesc {
                // TODO: Consume virtual code flags
                virtual_key: VirtualKey { code: (entry.Vk & 0xFF) as u8 },
                name: None
            });
        }

        for entry in table((*descriptor_ptr).pVSCtoVK_E1, |entry| entry.Vsc != 0) {
            result.insert(ScanCode::Extended1(entry.Vsc), PhysicalKeyDesc {
                // TODO: Consume virtual code flags
                virtual_key: VirtualKey { code: (entry.Vk & 0xFF) as u8 },
                name: None
            });
        }

        // Populate physical key names
        for entry in table((*descriptor_ptr).pKeyNames, |entry| entry.vsc != 0) {
            let Some(entry_ref) = result.get_mut(&ScanCode::Unescaped(entry.vsc)) else {
                continue
            };
            entry_ref.name = Some(pwsz_to_string(entry.pwsz));
        }

        for entry in table((*descriptor_ptr).pKeyNamesExt, |entry| entry.vsc != 0) {
            let Some(entry_ref) = result.get_mut(&ScanCode::Extended0(entry.vsc)) else {
                continue
            };
            entry_ref.name = Some(pwsz_to_string(entry.pwsz));
        }
    }

    result
}


struct ZeroTerminatedTableIterator<V> {
    row: *const V,
    first: bool,
    predicate: fn(V) -> bool,
}

impl<V> Iterator for ZeroTerminatedTableIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.row.is_null() {
                return None;
            }

            if !(self.predicate)(self.row.read()) {
                return None;
            }

            if self.first {
                self.first = false;
                return Some(self.row.read());
            }

            self.row = self.row.add(1);
            if !(self.predicate)(self.row.read()) {
                return None;
            }

            Some(self.row.read())
        }
    }
}

fn table<V>(start: *const V, predicate: fn(V) -> bool) -> ZeroTerminatedTableIterator<V> {
    ZeroTerminatedTableIterator { row: start, first: true, predicate }
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
