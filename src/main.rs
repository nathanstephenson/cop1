mod inputs;

fn main() {
    println!("Hello, world!");
    println!("Press keys (Ctrl+C to exit):");
    inputs::file::start_and_read();
    inputs::keyboard::accept_input();
}
