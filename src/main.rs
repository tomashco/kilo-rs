use std::io::{stdin, stdout, Write};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message
    write!(
        stdout,
        "{}{}{}Esc to exit, ctrl + h to print \"Hello world!\"\r\n",
        termion::style::Bold,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();
    // let line_counter = stdin.keys().flatten().count() as u16;
    //detecting keydown events
    for c in stdin.keys() {
        //i reckon this speaks for itself

        match c.unwrap() {
            Key::Esc => break,
            Key::Ctrl('d') => println!("{}\rWrite something", termion::style::Reset),
            Key::Char('n') => write!(stdout, "{}", termion::cursor::Down(1)).unwrap(),
            Key::Char('t') => write!(stdout, "{:?}", termion::terminal_size().unwrap()).unwrap(),
            Key::Char('b') => println!("{}", termion::color::Bg(termion::color::Cyan)),
            Key::Char('r') => println!("{}", termion::color::Bg(termion::color::Reset)),
            Key::Ctrl('h') => println!("\rHello world!"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
