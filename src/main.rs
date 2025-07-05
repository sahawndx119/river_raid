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
    style::{Color, Print, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

use rand::Rng;

struct Plane {
    health_point: u16,
    coordinate: (u16, u16),
}

struct EnemyPlane {
    max_health: u16,
    health_point: u16,
    coordinate: (u16, u16),
}

fn main() {
    let std:Arc<Mutex<std::io::Stdout>> = Arc::new(Mutex::new(stdout()));
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
        coordinate: (23, 25),
    };
    (plane.coordinate.0, plane.coordinate.1);

    let plane_holder: Arc<Mutex<Plane>> = Arc::new(Mutex::new(plane));
    // let x_holder = plane.x;
    // let y_holder = plane.y;

    stdout().flush().unwrap();

    let enemies: Vec<EnemyPlane> = Vec::new();
    let enemies_holder = Arc::new(Mutex::new(enemies));

    let mut x_holder = plane_holder.lock().unwrap().coordinate.0;
    let mut y_holder = plane_holder.lock().unwrap().coordinate.1;
    let mut health_point = plane_holder.lock().unwrap().health_point;

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
                        if y_holder > 20 {
                            plane_holder.lock().unwrap().coordinate.1 -= 1;
                            y_holder -= 1;
                        }
                    }

                    KeyCode::Char('s') | KeyCode::Down => {
                        if y_holder < 36 {
                            plane_holder.lock().unwrap().coordinate.1 += 1;
                            y_holder += 1;
                        }
                    }

                    KeyCode::Char('a') | KeyCode::Left => {
                        if x_holder > 1 {
                            plane_holder.lock().unwrap().coordinate.0 -= 1;
                            x_holder -= 1;
                        }
                    }

                    KeyCode::Char('d') | KeyCode::Right => {
                        if x_holder < 45 {
                            plane_holder.lock().unwrap().coordinate.0 += 1;
                            x_holder += 1;
                        }
                    }

                    _ => {}
                }
            } else {
            }
        } else {
        }
        let std_holder = std.clone();
        print_friendly_plane(x_holder, y_holder , std_holder);
        stdout().flush().unwrap();

        if cnt % 30 == 0 {
            let holder = plane_holder.clone();
            let std_holder = std.clone();
            let _ = std::thread::spawn(move || {
                friendly_plane_fireing(x_holder, y_holder, holder , std_holder);
            });
        }
        if cnt % 50 == 0 {
            let holder = enemies_holder.clone();
            spawn_enemy_planes(holder);
        }
    }
}

fn print_friendly_plane(x: u16, y: u16 , std : Arc<Mutex<std::io::Stdout>>) {
    queue!(std.lock().unwrap(), cursor::MoveTo(x, y), Print("⍊⍊⏏⍊⍊")).unwrap();
}

fn clear_friendly_plane(x: u16, y: u16) {
    for i in 0..5 {
        queue!(stdout(), cursor::MoveTo(x + i, y), Print(" ")).unwrap();
    }
}

fn print_playground() {
    for i in 0..51 {
        for j in 0..41 {
            if i == 0 || i == 50 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("✱")).unwrap();
            } else if j == 40 && i % 2 == 0 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("✱")).unwrap();
            }
        }
    }
    stdout().flush().unwrap();
}

fn friendly_plane_fireing(x: u16, y: u16, plane: Arc<Mutex<Plane>> , std : Arc<Mutex<std::io::Stdout>>) {
    for j in 0..y {
        let (x_holder, y_holder) = plane.lock().unwrap().coordinate;
        if (y_holder == y - j - 1) && (x + 2 >= x_holder && x + 2 <= x_holder + 4) {
            sleep(Duration::from_millis(300));
            queue!(std.lock().unwrap(), cursor::MoveTo(x + 2, y - 2 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(std.lock().unwrap(), cursor::MoveTo(x + 2, y - j), Print(" ")).unwrap();
            }
        } else if ((y_holder == y - j) && (x + 2 >= x_holder && x + 2 <= x_holder + 4))  && j != 0 {
            sleep(Duration::from_millis(300));
            queue!(std.lock().unwrap(), cursor::MoveTo(x + 2, y - 2 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(std.lock().unwrap(), cursor::MoveTo(x + 2, y - j - 1), Print(" ")).unwrap();
            }
        } else {
            sleep(Duration::from_millis(300));
            queue!(std.lock().unwrap(), cursor::MoveTo(x + 2, y - 1 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(std.lock().unwrap(), cursor::MoveTo(x + 2, y - j), Print(" ")).unwrap();
            }
            std.lock().unwrap().flush().unwrap();
        }
    }
    stdout().flush().unwrap();
    execute!(stdout(), cursor::MoveTo(x + 2, 0), Print(" ")).unwrap();
}
fn spawn_enemy_planes(array: Arc<Mutex<Vec<EnemyPlane>>>) {
    let x;
    let y;
    loop {
        let x_in = rand::thread_rng().gen_range(1..48);
        let y_in = rand::thread_rng().gen_range(1..13);

        let array2 = array.lock().unwrap();
        let res = array2
            .iter()
            .find(|plane| x_in == plane.coordinate.0 || y_in == plane.coordinate.1);
        match res {
            Some(_) => {}
            None => {
                x = x_in;
                y = y_in;
                break;
            }
        }
    }
    let new_plane = EnemyPlane {
        max_health: 60,
        health_point: 60,
        coordinate: (x, y),
    };

    execute!(stdout(), cursor::MoveTo(x, y), Print("-⌄-")).unwrap();

    let pct = new_plane.health_point as f32 / new_plane.max_health as f32;
    let color = if pct > 0.66 {
        Color::Green
    } else if pct > 0.33 {
        Color::Yellow
    } else {
        Color::Red
    };

    let filled = (new_plane.health_point / 20) as usize; // چند تا خانهٔ پر
    let empty = ((new_plane.max_health - new_plane.health_point) / 20) as usize;

    queue!(
        stdout(),
        cursor::MoveTo(new_plane.coordinate.0, new_plane.coordinate.1 - 1),
        SetForegroundColor(color),
        Print("▮".repeat(filled)),
        SetForegroundColor(Color::Grey),
        Print("▯".repeat(empty)),
        SetForegroundColor(Color::Reset),
    )
    .unwrap();

    let mut array2 = array.lock().unwrap();
    array2.push(new_plane);
}