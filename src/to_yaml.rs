use saphyr::{Hash, Yaml};

use crate::model::*;

impl KeyboardDesc {
    pub fn to_yaml(&self) -> Yaml {
        let mut document = Hash::new();
        {
            let mut physical_to_virtual = Hash::new();
            for (scan_code, physical_key) in &self.physical_keys {
                if physical_key.name.is_none() { continue; }
                physical_to_virtual.insert(scan_code.to_yaml(), Yaml::String(physical_key.name.clone().unwrap()));
            }
            document.insert(Yaml::String("physicalKeyNames".into()), Yaml::Hash(physical_to_virtual));
        }
        {
            let mut physical_to_virtual = Hash::new();
            for (scan_code, physical_key) in &self.physical_keys {
                physical_to_virtual.insert(scan_code.to_yaml(), physical_key.virtual_key.to_yaml());
            }
            document.insert(Yaml::String("physicalToVirtualKeys".into()), Yaml::Hash(physical_to_virtual));
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
            Self::Unescaped(code) => format!("{:02X}", code),
            Self::Extended0(code) => format!("E0{:02X}", code),
            Self::Extended1(code) => format!("E1{:02X}", code),
        };
        Yaml::String(str)
    }
}

impl VirtualKey {
    fn to_yaml(&self) -> Yaml {
        if *self == VirtualKey::NONE {
            return Yaml::Null;
        }
        if let Some(enum_name) = self.to_vk_enum(true) {
            return Yaml::String(enum_name);
        }
        Yaml::String(format!("{:02X}", self.code))
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