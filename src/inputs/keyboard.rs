use console::{Key, Term};

use crate::thread_manager::SendFunc;

pub fn wait_for_input(input_reader: SendFunc<Key>) {
    let term = Term::stdout();

    loop {
        let c = Term::read_key(&term);
        let input_result = match c {
            Ok(char) => input_reader(char),
            Err(_) => {
                println!("Error reading input, please try again.");
                continue;
            }
        };
        match input_result {
            Ok(_) => {}
            Err(_) => {
                println!("Error processing input, please try again.");
                continue;
            }
        }
    }
}

// fn reader() -> SendFunc<Key> {
//     fn reader_fun(message: Key) -> Result<(), Box<dyn std::error::Error>> {
//         let mut stdout = std::io::stdout();
//         let mut val = String::new();
//         match message {
//             Key::Char(c) => val.push(c),
//             Key::Enter => val.push_str("[Enter]"),
//             Key::Backspace => val.push_str("[Backspace]"),
//             Key::ArrowUp => val.push_str("[ArrowUp]"),
//             Key::ArrowDown => val.push_str("[ArrowDown]"),
//             Key::ArrowLeft => val.push_str("[ArrowLeft]"),
//             Key::ArrowRight => val.push_str("[ArrowRight]"),
//             Key::Escape => val.push_str("[Esc]"),
//             _ => val.push_str("[Other]"),
//         }
//         let _ = write!(stdout, "You pressed: {}", val);
//         let _ = write!(stdout, "\n");
//         Ok(())
//     }
//     Box::new(reader_fun)
// }

// pub fn accept_input() {
//     wait_for_input(reader());
// }
