use std::mem::transmute;
use windows_sys::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

use crate::model::*;

type FnKbdLayerDescriptor = extern "system" fn() -> *const KBDTABLES;

struct ZeroTerminatedTableIterator<V> {
    row: *const V,
    first: bool,
    until: fn(V) -> bool,
}

impl<V> Iterator for ZeroTerminatedTableIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.row.is_null() {
                return None;
            }

            if (self.until)(self.row.read()) {
                return None;
            }

            if self.first {
                self.first = false;
                return Some(self.row.read());
            }

            self.row = self.row.add(1);
            if (self.until)(self.row.read()) {
                return None;
            }

            Some(self.row.read())
        }
    }
}

fn table<V>(start: *const V, until: fn(V) -> bool) -> ZeroTerminatedTableIterator<V> {
    ZeroTerminatedTableIterator { row: start, first: true, until }
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
        for entry in table((*descriptor_ptr).pKeyNames, |entry| entry.vsc == 0) {
            descriptor.key_names.insert(ScanCode::Unescaped(entry.vsc), pwsz_to_string(entry.pwsz));
        }

        for entry in table((*descriptor_ptr).pKeyNamesExt, |entry| entry.vsc == 0) {
            descriptor.key_names.insert(ScanCode::Extended(entry.vsc), pwsz_to_string(entry.pwsz));
        }

        descriptor
    }
}