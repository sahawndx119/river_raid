//in the name of God

use std::io::{stdout, Write};

use crossterm::{self, cursor, execute, queue, style::Print, terminal};

fn main() {
    queue!(
        stdout(),
        crossterm::terminal::Clear(terminal::ClearType::All)
    )
    .unwrap();
    queue!(stdout(), cursor::Hide).unwrap();
    stdout().flush().unwrap();
    print_playground();
    print_plane(15, 10);

}

fn print_plane(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y), Print("-⌃-")).unwrap();
}

fn print_playground() {
    for i in 0..41 {
        for j in 0..21 {
            if i == 0 || i == 40 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("✱")).unwrap();
            }else if j == 20 && i % 2 == 0{
                queue!(stdout(), cursor::MoveTo(i, j), Print("✱")).unwrap();
            }
        }
    }
    stdout().flush().unwrap();
}
