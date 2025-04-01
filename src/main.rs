mod model;
mod read_dll;
mod json_format;

fn main() {
    let descriptor = read_dll::read_keyboard(r"C:\Windows\System32\KBDUS.DLL".to_owned());
    println!("Found {} scan codes, {} virtual keys, {} dead keys",
        descriptor.physical_keys.len(),
        descriptor.virtual_keys.len(),
        descriptor.dead_keys.len());

    println!("{}", descriptor.to_json());
}
