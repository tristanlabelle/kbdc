use std::collections::HashMap;
use std::mem::transmute;
use windows_sys::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use serde::{Serialize, Deserialize};

type FnKbdLayerDescriptor = extern "system" fn() -> *const KBDTABLES;

#[derive(Serialize, Deserialize)]
struct KeyboardDescriptor {
    key_names: HashMap<ScanCode, String>,
    dead_keys: Vec<DeadKey>,
}

struct KeyModifiers {
    shift: bool, // CAPOK
    caps_lock: bool, // SGCAPLOK
    altgr: bool, // CAPLOKALTGR
    kana: bool, // KANALOK
}

enum VirtualKeyEffect {
    Chars(Box<[u16]>),
    DeadKey(Box<[u16]>),
    Ligature
}

impl KeyboardDescriptor {
    fn new() -> Self {
        Self {
            key_names: HashMap::new(),
            dead_keys: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ScanCode {
    Unescaped(u8), // Most significant bit unused
    Extended(u8), // E0-escaped, most significant bit unused
    Pause, // E1-escaped
}

#[derive(Serialize, Deserialize)]
struct DeadKey {
    name: Option<String>,
    base_char: u16,
    diacritic: u16,
    composed_char: u16,
    flags: u16
}

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

fn main() {
    unsafe {
        let mut path_utf16: Vec<u16> = r"C:\Windows\System32\KBDUS.DLL".encode_utf16().collect();
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

        println!("{}", serde_json::to_string(&descriptor).unwrap());
    }
}
