use saphyr::{Hash, Yaml};

use crate::model::*;

impl KeyboardDesc {
    pub fn to_yaml(&self) -> Yaml {
        let mut document = Hash::new();
        {
            let mut physical_keys = Hash::new();
            for (scan_code, physical_key) in &self.physical_keys {
                physical_keys.insert(scan_code.to_yaml(), physical_key.to_yaml());
            }
            document.insert(Yaml::String("physicalKeys".into()), Yaml::Hash(physical_keys));
        }
        // {
        //     let mut virtual_keys = Hash::new();
        //     for (virtual_key, key_effect) in &self.virtual_keys {
        //         virtual_keys.insert(virtual_key.to_yaml(), key_effect.to_yaml());
        //     }
        //     document.insert("virtual_keys".to_string(), Yaml::Hash(virtual_keys));
        // }
        // {
        //     let mut dead_keys = Hash::new();
        //     for (dead_key, dead_key_desc) in &self.dead_keys {
        //         dead_keys.insert(dead_key.to_yaml(), dead_key_desc.to_yaml());
        //     }
        //     document.insert("dead_keys".to_string(), Yaml::Hash(dead_keys));
        // }
        Yaml::Hash(document)
    }
}

impl ScanCode {
    fn to_yaml(&self) -> Yaml {
        let str: String = match self {
            Self::Unescaped(code) => format!("0x{:02X}", code),
            Self::Extended0(code) => format!("0xE0{:02X}", code),
            Self::Extended1(code) => format!("0xE1{:02X}", code),
        };
        Yaml::String(str)
    }
}

impl VirtualKey {
    fn to_yaml(&self) -> Yaml {
        // TODO: Leverage VirtualKey.to_vk_name() to get the name.
        Yaml::String(format!("0x{:02X}", self.code))
    }
}

impl PhysicalKeyDesc {
    fn to_yaml(&self) -> Yaml {
        let mut hash = Hash::new();
        hash.insert(Yaml::String("virtualKey".into()), self.virtual_key.to_yaml());
        if let Some(name) = &self.name {
            hash.insert(Yaml::String("name".into()), Yaml::String(name.clone()));
        }
        Yaml::Hash(hash)
    }
}