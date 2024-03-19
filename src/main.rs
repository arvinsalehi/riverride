use std::io::{stdout, Stdout, Write};
use std::time::Duration;

use crossterm::style::Color;
use crossterm::{
    cursor::MoveTo,
    event::{self, poll, read, Event, KeyCode},
    execute,
    style::{Print, SetForegroundColor},
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};

struct World {
    player_c: u16,
    player_l: u16,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(MoveTo(world.player_c, world.player_l))?;
    sc.queue(SetForegroundColor(Color::Yellow))?;
    sc.queue(Print('P'))?;
    sc.queue(SetForegroundColor(Color::Reset))?;
    sc.flush();
    Ok(())
}

fn mechanics(
    sc: &mut Stdout,
    world: &mut World,
    height: u16,
    width: u16,
    stop: &mut bool,
) -> std::io::Result<()> {
    if poll(Duration::from_millis(10))? {
        let key = read()?;
        match key {
            Event::Key(event) => match event.code {
                KeyCode::Char('q') => *stop = !*stop,
                KeyCode::Up => {
                    world.player_l -= 1;
                    if world.player_l == 0 {
                        world.player_l = height - 1;
                    }
                }
                KeyCode::Down => {
                    world.player_l += 1;
                    if world.player_l == height {
                        world.player_l = 1;
                    }
                }
                KeyCode::Left => {
                    world.player_c -= 1;
                    if world.player_c == 0 {
                        world.player_c = width - 1;
                    }
                }
                KeyCode::Right => {
                    world.player_c += 1;
                    if world.player_c == width {
                        world.player_c = 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Initialize the terminal.
    terminal::enable_raw_mode()?;
    // Get the size of the terminal.
    let (width, height) = size()?;

    let mut sc = stdout();
    let mut world = World {
        player_c: width / 2,
        player_l: height - 1,
    };

    let mut stop: bool = false;
    while !stop {
        let _ = mechanics(&mut sc, &mut world, height, width, &mut stop);
        draw(&sc, &world)?;
    }
    // Disable raw mode and show the cursor before exiting
    terminal::disable_raw_mode()?;
    execute!(sc, crossterm::cursor::Show)?;
    Ok(())
}
