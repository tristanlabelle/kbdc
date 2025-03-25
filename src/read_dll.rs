use std::mem::transmute;
use windows_sys::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

use crate::model::*;

type FnKbdLayerDescriptor = extern "system" fn() -> *const KBDTABLES;

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

unsafe fn pwsz_to_string(ptr: *const u16) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf16_lossy(slice)
    }
}

pub fn read_keyboard(path: String) -> KeyboardDescriptor {
    unsafe {
        let mut path_utf16: Vec<u16> = path.encode_utf16().collect();
        path_utf16.push(0);
        let module = LoadLibraryW(path_utf16.as_ptr());
        if module.is_null() {
            panic!("Failed to load library");
        }

        let proc: FnKbdLayerDescriptor = transmute(GetProcAddress(module, b"KbdLayerDescriptor\0".as_ptr()).unwrap());
        let descriptor_ptr = proc();

        let mut descriptor = KeyboardDescriptor::new();
        for scan_code in 0..(*descriptor_ptr).bMaxVSCtoVK {
            let virtual_key_code = (*descriptor_ptr).pusVSCtoVK.offset(scan_code as isize).read();
            if virtual_key_code == 0xFF { continue }
            descriptor.scan_codes.insert(ScanCode::Unescaped(scan_code as u8), ScanCodeEntry {
                virtual_key: VirtualKey { code: virtual_key_code as u8 },
                name: None
            });
        }

        for entry in table((*descriptor_ptr).pVSCtoVK_E0, |entry| entry.Vsc != 0) {
            descriptor.scan_codes.insert(ScanCode::Extended0(entry.Vsc), ScanCodeEntry {
                // TODO: Consume virtual code flags
                virtual_key: VirtualKey { code: (entry.Vk & 0xFF) as u8 },
                name: None
            });
        }

        for entry in table((*descriptor_ptr).pVSCtoVK_E1, |entry| entry.Vsc != 0) {
            descriptor.scan_codes.insert(ScanCode::Extended1(entry.Vsc), ScanCodeEntry {
                // TODO: Consume virtual code flags
                virtual_key: VirtualKey { code: (entry.Vk & 0xFF) as u8 },
                name: None
            });
        }

        for entry in table((*descriptor_ptr).pKeyNames, |entry| entry.vsc != 0) {
            let Some(entry_ref) = descriptor.scan_codes.get_mut(&ScanCode::Unescaped(entry.vsc)) else {
                continue
            };
            entry_ref.name = Some(pwsz_to_string(entry.pwsz));
        }

        for entry in table((*descriptor_ptr).pKeyNamesExt, |entry| entry.vsc != 0) {
            let Some(entry_ref) = descriptor.scan_codes.get_mut(&ScanCode::Extended0(entry.vsc)) else {
                continue
            };
            entry_ref.name = Some(pwsz_to_string(entry.pwsz));
        }

        descriptor
    }
}