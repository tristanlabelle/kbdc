mod model;
mod read_dll;
mod to_yaml;

use saphyr::YamlEmitter;

fn main() {
    let descriptor = read_dll::read_keyboard(r"C:\Windows\System32\KBDUS.DLL".to_owned());
    println!("Found {} scan codes, {} virtual keys, {} dead keys",
        descriptor.physical_keys.len(),
        descriptor.virtual_keys.len(),
        descriptor.dead_keys.len());

    let yaml = descriptor.to_yaml();
    let mut yaml_str = String::new();
    YamlEmitter::new(&mut yaml_str).dump(&yaml).unwrap();

    println!("{}", yaml_str);
}
