mod model;
mod read_dll;

fn main() {
    let descriptor = read_dll::read_keyboard(r"C:\Windows\System32\KBDUS.DLL".to_owned());
    println!("Found {} key names, {} dead keys", descriptor.key_names.len(), descriptor.dead_keys.len())
}
