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

#[derive(Clone, Copy)]
struct EnemyPlane {
    max_health: u16,
    health_point: u16,
    coordinate: (u16, u16),
}

#[tokio::main]
async fn main() {
    let std: Arc<Mutex<std::io::Stdout>> = Arc::new(Mutex::new(stdout()));
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
    let enemies_coors: Vec<(u16, u16)> = Vec::new();
    let enemies_coors_holder = Arc::new(Mutex::new(enemies_coors));
    let enemies_holder = Arc::new(Mutex::new(enemies));

    let mut x_holder = plane_holder.lock().unwrap().coordinate.0;
    let mut y_holder = plane_holder.lock().unwrap().coordinate.1;
    let _health_point = plane_holder.lock().unwrap().health_point;

    let mut cnt = 1;
    loop {
        cnt += 1;
        if poll(Duration::from_millis(350)).unwrap() {
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
        match print_friendly_plane(plane_holder.clone(), std.clone()) {
            Err(()) => {
                queue!(
                    stdout(),
                    crossterm::terminal::Clear(terminal::ClearType::All)
                )
                .unwrap();
                let mut out = stdout();
                let text = "GAME OVER";
                let x = (50 - text.len() as u16) / 2;
                let y = 20;

                queue!(
                    out,
                    cursor::MoveTo(x, y),
                    SetForegroundColor(Color::Red),
                    Print(text),
                    SetForegroundColor(Color::Reset)
                )
                .unwrap();

                out.flush().unwrap();

                disable_raw_mode().unwrap();
                return;
            }

            Ok(()) => {}
        }
        let enemy_holder_clone = enemies_holder.clone();
        let std_holder = std.clone();
        let enemy_coor_holder_clone = enemies_coors_holder.clone();
        let _update_enemy_handle = std::thread::spawn(move || {
            let changed_health =
                <Vec<EnemyPlane> as Clone>::clone(&enemy_holder_clone.lock().unwrap())
                    .into_iter()
                    .enumerate()
                    .filter(|(_, plane)| plane.health_point != 60)
                    .map(|(index, plane)| {
                        (
                            index,
                            plane.coordinate.0,
                            plane.coordinate.1,
                            plane.health_point,
                        )
                    });
            for (index, x, y, hp) in changed_health {
                let another_holder = std_holder.clone();
                update_enemy_plane((x, y), hp, another_holder);
                if hp == 0 {
                    enemy_holder_clone.lock().unwrap().remove(index);
                    enemy_coor_holder_clone.lock().unwrap().remove(index);
                }
            }
        });

        stdout().flush().unwrap();

        if cnt % 30 == 0 {
            let holder = plane_holder.clone();
            let coor_holder = enemies_coors_holder.clone();
            let enemy_holder_clone = enemies_holder.clone();
            let _ = std::thread::spawn(move || {
                friendly_plane_fireing(x_holder, y_holder, holder, coor_holder, enemy_holder_clone);
            });
        }
        if cnt % 50 == 0 && enemies_holder.lock().unwrap().len() < 4 {
            let holder = enemies_holder.clone();
            spawn_enemy_planes(holder, enemies_coors_holder.clone(), std.clone());
        }

        if cnt % 70 == 0 {
            let enemy_clone = enemies_holder.clone();
            for enemy in enemy_clone.lock().unwrap().iter() {
                let holder = plane_holder.clone();
                let std_holder = std.clone();
                let enemy = enemy.clone();
                tokio::spawn(async move {
                    enemye_plane_fireing(&enemy, holder, std_holder).await;
                });
            }
        }
    }
}

fn print_friendly_plane(
    plane: Arc<Mutex<Plane>>,
    std: Arc<Mutex<std::io::Stdout>>,
) -> Result<(), ()> {
    let (x, y, health) = {
        let plane_lock = plane.lock().unwrap();
        (
            plane_lock.coordinate.0,
            plane_lock.coordinate.1,
            plane_lock.health_point,
        )
    };

    if health == 0 {
        return Err(());
    }

    let mut out = std.lock().unwrap();

    queue!(out, cursor::MoveTo(x, y), Print("⍊⍊⏏⍊⍊")).unwrap();

    let bar_x = 55;
    let bar_y = 5;
    let bar_length = 20;

    let filled_len = (health as f32 / 100.0 * bar_length as f32).round() as usize;
    let empty_len = bar_length - filled_len;

    let color = if health > 66 {
        Color::Green
    } else if health > 33 {
        Color::Yellow
    } else {
        Color::Red
    };

    queue!(
        out,
        cursor::MoveTo(bar_x, bar_y - 1),
        SetForegroundColor(Color::White),
        Print("HP"),
        SetForegroundColor(color),
        cursor::MoveTo(bar_x, bar_y),
        Print("▉".repeat(filled_len)),
        SetForegroundColor(Color::Grey),
        Print("░".repeat(empty_len)),
        SetForegroundColor(Color::Reset),
    )
    .unwrap();

    out.flush().unwrap();
    Ok(())
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

fn friendly_plane_fireing(
    x: u16,
    y: u16,
    plane: Arc<Mutex<Plane>>,
    enemy_coors: Arc<Mutex<Vec<(u16, u16)>>>,
    enemies: Arc<Mutex<Vec<EnemyPlane>>>,
) {
    for j in 0..y {
        if (y - 1 - j) <= 20 {
            let res = enemy_coors.lock().unwrap().iter().position(|coor| {
                ((y - 1 - j) == coor.1) && (x + 2 >= coor.0 && x + 2 <= coor.0 + 2)
            });

            match res {
                Some(index) => {
                    if enemies.lock().unwrap()[index].health_point != 0 {
                        enemies.lock().unwrap()[index].health_point -= 20;
                    }
                    sleep(Duration::from_millis(300));
                    queue!(stdout(), cursor::MoveTo(x + 2, y - j), Print(" ")).unwrap();
                    break;
                }
                None => {}
            }
        }
        let (x_holder, y_holder) = plane.lock().unwrap().coordinate;
        if (y_holder == y - j - 1) && (x + 2 >= x_holder && x + 2 <= x_holder + 4) {
            sleep(Duration::from_millis(300));
            queue!(stdout(), cursor::MoveTo(x + 2, y - 2 - j), Print("⌂")).unwrap();
            if j != 0 {
                queue!(stdout(), cursor::MoveTo(x + 2, y - j), Print(" ")).unwrap();
            }
        } else if ((y_holder == y - j) && (x + 2 >= x_holder && x + 2 <= x_holder + 4)) && j != 0 {
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
        }
    }
    stdout().flush().unwrap();
    execute!(stdout(), cursor::MoveTo(x + 2, 0), Print(" ")).unwrap();
}
fn spawn_enemy_planes(
    array: Arc<Mutex<Vec<EnemyPlane>>>,
    coors: Arc<Mutex<Vec<(u16, u16)>>>,
    std: Arc<Mutex<std::io::Stdout>>,
) {
    let x;
    let y;
    loop {
        let x_in = rand::thread_rng().gen_range(1..48);
        let y_in = rand::thread_rng().gen_range(1..13);

        let array2 = array.lock().unwrap();
        let res = array2.iter().find(|plane| {
            (x_in + 2 >= plane.coordinate.0 && x_in + 2 <= plane.coordinate.0 + 2)
                || y_in == plane.coordinate.1
        });
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

    {
        let mut out = std.lock().unwrap();
        queue!(out, cursor::MoveTo(x, y), Print("-⌄-")).unwrap();
    }

    let pct = new_plane.health_point as f32 / new_plane.max_health as f32;
    let color = if pct > 0.66 {
        Color::Green
    } else if pct > 0.33 {
        Color::Yellow
    } else {
        Color::Red
    };

    let filled = (new_plane.health_point / 20) as usize;
    let empty = ((new_plane.max_health - new_plane.health_point) / 20) as usize;
    {
        let mut out = std.lock().unwrap();
        queue!(
            out,
            cursor::MoveTo(new_plane.coordinate.0, new_plane.coordinate.1 - 1),
            SetForegroundColor(color),
            Print("▮".repeat(filled)),
            SetForegroundColor(Color::Grey),
            Print("▯".repeat(empty)),
            SetForegroundColor(Color::Reset),
        )
        .unwrap();
    }

    let mut array2 = array.lock().unwrap();
    array2.push(new_plane);
    coors.lock().unwrap().push(new_plane.coordinate);
}

async fn enemye_plane_fireing(
    plane: &EnemyPlane,
    friendly_plane: Arc<Mutex<Plane>>,
    std: Arc<Mutex<std::io::Stdout>>,
) {
    let (x_holder, y_holder) = plane.coordinate;

    for y in y_holder + 2..37 {
        tokio::time::sleep(Duration::from_millis(400)).await;

        {
            let mut out = std.lock().unwrap();
            queue!(
                out,
                cursor::MoveTo(x_holder + 1, y),
                Print("●"),
                cursor::MoveTo(x_holder + 1, y - 1),
                Print(" ")
            )
            .unwrap();
            out.flush().unwrap();
        }

        let (fx, fy) = {
            let plane = friendly_plane.lock().unwrap();
            let coord = plane.coordinate;
            (coord.0, coord.1)
        };

        if (y == fy || y + 1 == fy || y - 1 == fy) && (x_holder + 1 >= fx && x_holder + 1 <= fx + 4)
        {
            {
                let mut plane = friendly_plane.lock().unwrap();
                if plane.health_point >= 20 {
                    plane.health_point -= 20;
                } else {
                    plane.health_point = 0;
                }
            }

            tokio::time::sleep(Duration::from_millis(400)).await;
            {
                let mut out = std.lock().unwrap();
                queue!(out, cursor::MoveTo(x_holder + 1, y), Print(" ")).unwrap();
                out.flush().unwrap();
            }

            break;
        }
    }

    {
        let mut out = std.lock().unwrap();
        queue!(out, cursor::MoveTo(x_holder + 1, 36), Print(" ")).unwrap();
        out.flush().unwrap();
    }
}

fn update_enemy_plane(coors: (u16, u16), health: u16, std: Arc<Mutex<std::io::Stdout>>) {
    if health == 0 {
        for j in 0..2 {
            for i in 0..3 {
                {
                    let out = std.clone();
                    queue!(
                        out.lock().unwrap(),
                        cursor::MoveTo(coors.0 + i, coors.1 - j),
                        Print(" ")
                    )
                    .unwrap();
                }
            }
        }
        return;
    }
    let pct = health as f32 / 60.0;
    let color = if pct > 0.66 {
        Color::Green
    } else if pct > 0.33 {
        Color::Yellow
    } else {
        Color::Red
    };

    let filled = (health / 20) as usize;
    let empty = ((60 - health) / 20) as usize;

    {
        let out = std.clone();
        queue!(
            out.lock().unwrap(),
            cursor::MoveTo(coors.0, coors.1 - 1),
            SetForegroundColor(color),
            Print("▮".repeat(filled)),
            SetForegroundColor(Color::Grey),
            Print("▯".repeat(empty)),
            SetForegroundColor(Color::Reset),
        )
        .unwrap();
    }
}
