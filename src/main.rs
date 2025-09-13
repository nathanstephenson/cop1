mod inputs;
mod outputs;
mod thread_manager;

fn main() {
    println!("Hello, world!");
    println!("Press keys (Ctrl+C to exit):");
    let keyboard_receiver = thread_manager::create_input_thread(inputs::keyboard::wait_for_input);
    let file_receiver = thread_manager::create_input_thread(inputs::file::start_and_read);

    thread_manager::create_output_thread(
        file_receiver,
        outputs::playback::create_output_track,
        false,
    );
    thread_manager::create_output_thread(keyboard_receiver, outputs::display::create_output, true);

    loop {
        std::thread::park();
    }
}
