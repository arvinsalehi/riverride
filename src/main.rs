use std::io::{stdout, Stdout, Write};
use std::time::Duration;

use crossterm::queue;
use crossterm::style::Color;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Print, SetForegroundColor},
    terminal::{self, size},
    QueueableCommand,
};

struct World {
    width: u16,
    height: u16,
    map: Vec<(u16, u16)>,
}

// Boat struct to hold boat state
struct Boat {
    position_x: u16,
    position_y: u16,
}

trait BoatMechanics {
    fn move_up(&mut self);

    fn move_down(&mut self);

    fn move_left(&mut self);

    fn move_right(&mut self);
}

impl Boat {
    /// Creates a new [`Boat`].
    fn new(x: u16, y: u16) -> Boat {
        Boat {
            position_x: x,
            position_y: y,
        }
    }

    fn display(&self) {
        print!("\x1B[2J\x1B[1;1H"); // Clear the terminal
        println!(
            "Current Boat Position: ({}, {})",
            self.position_x, self.position_y
        );
    }
}

impl BoatMechanics for Boat {
    fn move_up(&mut self) {
        self.position_y -= 1;
    }

    fn move_down(&mut self) {
        self.position_y += 1;
    }

    fn move_left(&mut self) {
        self.position_x -= 1;
    }

    fn move_right(&mut self) {
        self.position_x += 1;
    }
}

fn draw(mut sc: &Stdout, world: &World, boat: &Boat) -> std::io::Result<()> {
    queue!(sc, Clear(ClearType::All))?;
    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;
        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;
    }
    sc.queue(MoveTo(boat.position_x, boat.position_y))?;
    sc.queue(SetForegroundColor(Color::Yellow))?;
    sc.queue(Print('d'))?;
    sc.queue(SetForegroundColor(Color::Reset))?;
    let _ = sc.flush();
    Ok(())
}

fn mechanics(boat: &mut Boat, height: u16, width: u16) -> std::io::Result<bool> {
    if poll(Duration::from_millis(10))? {
        let key = read()?;
        match key {
            Event::Key(event) => match event.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => {
                    boat.position_y -= 1;
                    if boat.position_y == 0 {
                        boat.position_y = height - 1;
                    }
                }
                KeyCode::Down => {
                    boat.position_y += 1;
                    if boat.position_y == height {
                        boat.position_y = 1;
                    }
                }
                KeyCode::Left => {
                    boat.position_x -= 1;
                    if boat.position_x == 0 {
                        boat.position_x = width - 1;
                    }
                }
                KeyCode::Right => {
                    boat.position_x += 1;
                    if boat.position_x == width {
                        boat.position_x = 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(false)
}

fn main() -> std::io::Result<()> {
    // Initialize the terminal.
    terminal::enable_raw_mode()?;
    // Get the size of the terminal.
    let (width, height) = size()?;

    let mut sc = stdout();
    let mut world: World = World {
        width: width,
        height: height,
        map: vec![((width / 2) - 5, (width / 2) + 5); height as usize],
    };
    let mut boat = Boat::new(width / 2, height - 1);

    let mut stop: bool = false;
    execute!(sc, Clear(ClearType::All))?;

    while !stop {
        stop = mechanics(&mut boat, height, width)?;
        draw(&sc, &world, &boat).expect("Failed in draw function");
    }
    // Disable raw mode and show the cursor before exiting
    execute!(sc, Clear(ClearType::All))?;
    terminal::disable_raw_mode()?;
    execute!(sc, crossterm::cursor::Show)?;
    Ok(())
}
