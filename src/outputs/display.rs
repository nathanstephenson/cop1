use console::Key;
use std::io::Write;

pub fn create_output() -> impl FnMut(Key) {
    let write = move |message: Key| {
        let mut stdout = std::io::stdout();
        let mut val = String::new();
        match message {
            Key::Char(c) => val.push(c),
            Key::Enter => val.push_str("[Enter]"),
            Key::Backspace => val.push_str("[Backspace]"),
            Key::ArrowUp => val.push_str("[ArrowUp]"),
            Key::ArrowDown => val.push_str("[ArrowDown]"),
            Key::ArrowLeft => val.push_str("[ArrowLeft]"),
            Key::ArrowRight => val.push_str("[ArrowRight]"),
            Key::Escape => val.push_str("[Esc]"),
            _ => val.push_str("[Other]"),
        }
        let _ = write!(stdout, "You pressed: {}", val);
        let _ = write!(stdout, "\n");
    };
    write
}
