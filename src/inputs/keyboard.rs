use console::Term;
use std::io::Write;

fn wait_for_input(input_reader: fn(console::Key)) {
    let term = Term::stdout();

    loop {
        let c = Term::read_key(&term);
        match c {
            Ok(char) => input_reader(char),
            Err(_) => {
                println!("Error reading input, please try again.");
                continue;
            }
        }
    }
}

fn reader() -> fn(console::Key) {
    fn reader_fun(message: console::Key) {
        let mut stdout = std::io::stdout();
        let mut val = String::new();
        match message {
            console::Key::Char(c) => val.push(c),
            console::Key::Enter => val.push_str("[Enter]"),
            console::Key::Backspace => val.push_str("[Backspace]"),
            console::Key::ArrowUp => val.push_str("[ArrowUp]"),
            console::Key::ArrowDown => val.push_str("[ArrowDown]"),
            console::Key::ArrowLeft => val.push_str("[ArrowLeft]"),
            console::Key::ArrowRight => val.push_str("[ArrowRight]"),
            console::Key::Escape => val.push_str("[Esc]"),
            _ => val.push_str("[Other]"),
        }
        let _ = write!(stdout, "You pressed: {}", val);
        let _ = write!(stdout, "\n");
    }
    reader_fun
}

pub fn accept_input() {
    wait_for_input(reader());
}
