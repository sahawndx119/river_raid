//in the name of God

use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    self, cursor,
    event::{self, poll, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

struct Plane {
    health_point: u16,
    coordinate: (u16, u16),
}

fn main() {
    enable_raw_mode().unwrap();
    queue!(
        stdout(),
        crossterm::terminal::Clear(terminal::ClearType::All)
    )
    .unwrap();
    queue!(stdout(), cursor::Hide).unwrap();
    stdout().flush().unwrap();
    print_playground();
    let plane = Plane {
        health_point: 100,
        coordinate: (23, 15),
    };
    print_friendly_plane(plane.coordinate.0, plane.coordinate.1);

    let plane_holder: Arc<Mutex<Plane>> = Arc::new(Mutex::new(plane));
    // let x_holder = plane.x;
    // let y_holder = plane.y;

    stdout().flush().unwrap();

    let mut x_holder = plane_holder.lock().unwrap().coordinate.0;
    let mut y_holder = plane_holder.lock().unwrap().coordinate.1;

    let mut cnt = 1;
    loop {
        cnt += 1;
        if poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                clear_friendly_plane(x_holder, y_holder);
                match key_event.code {
                    KeyCode::Esc => {
                        disable_raw_mode().unwrap();
                        return;
                    }

                    KeyCode::Char('w') | KeyCode::Up => {
                        // clear_friendly_plane(plane.x, plane.y);
                        plane_holder.lock().unwrap().coordinate.1 -= 1;
                        y_holder -= 1;
                        // print_friendly_plane(plane.x, plane.y);
                    }

                    KeyCode::Char('s') | KeyCode::Down => {
                        // clear_friendly_plane(plane.x, plane.y);
                        plane_holder.lock().unwrap().coordinate.1 += 1;
                        y_holder += 1;
                        // print_friendly_plane(plane.x, plane.y);
                    }

                    KeyCode::Char('a') | KeyCode::Left => {
                        // clear_friendly_plane(plane.x, plane.y);
                        plane_holder.lock().unwrap().coordinate.0 -= 1;
                        x_holder -= 1;
                        // print_friendly_plane(plane.x, plane.y);
                    }

                    KeyCode::Char('d') | KeyCode::Right => {
                        // clear_friendly_plane(plane.x, plane.y);
                        plane_holder.lock().unwrap().coordinate.0 += 1;
                        x_holder += 1;
                        // print_friendly_plane(plane.x, plane.y);
                    }

                    _ => {}
                }
            } else {
            }
        } else {
        }
        print_friendly_plane(x_holder, y_holder);
        stdout().flush().unwrap();

        if cnt % 30 == 0 {
            let holder = plane_holder.clone();
            let _ = std::thread::spawn(move || {
                friendly_plane_fireing(x_holder, y_holder, holder);
            });
        }
    }
}

fn print_friendly_plane(x: u16, y: u16) {
    queue!(stdout(), cursor::MoveTo(x, y), Print("⍊⍊⏏⍊⍊")).unwrap();
}

fn clear_friendly_plane(x: u16, y: u16) {
    for i in 0..5 {
        queue!(stdout(), cursor::MoveTo(x + i, y), Print(" ")).unwrap();
    }
}

fn print_playground() {
    for i in 0..51 {
        for j in 0..21 {
            if i == 0 || i == 50 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("✱")).unwrap();
            } else if j == 20 && i % 2 == 0 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("✱")).unwrap();
            }
        }
    }
    stdout().flush().unwrap();
}

fn friendly_plane_fireing(x: u16, y: u16, plane: Arc<Mutex<Plane>>) {
    for j in 0..y {
        let (x_holder, y_holder) = plane.lock().unwrap().coordinate;
        if (y_holder == y - j - 1) && (x + 2 >= x_holder && x + 2 <= x_holder + 4) {
            sleep(Duration::from_millis(300));
            queue!(stdout(), cursor::MoveTo(x + 2, y - 2 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(stdout(), cursor::MoveTo(x + 2, y - j), Print(" ")).unwrap();
            }
        } else if ((y_holder == y - j) && (x + 2 >= x_holder && x + 2 <= x_holder + 4))  && j != 0 {
            sleep(Duration::from_millis(300));
            queue!(stdout(), cursor::MoveTo(x + 2, y - 2 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(stdout(), cursor::MoveTo(x + 2, y - j - 1), Print(" ")).unwrap();
            }
        } else {
            sleep(Duration::from_millis(300));
            queue!(stdout(), cursor::MoveTo(x + 2, y - 1 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(stdout(), cursor::MoveTo(x + 2, y - j), Print(" ")).unwrap();
            }
            stdout().flush().unwrap();
        }
    }
    stdout().flush().unwrap();
    execute!(stdout(), cursor::MoveTo(x + 2, 0), Print(" ")).unwrap();
}
