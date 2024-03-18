use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo, event::{self, read}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{self, size}, ExecutableCommand, QueueableCommand
};

struct World {
    player_c: u16,
    player_l: u16,
}

fn draw(mut sc: &Stdout, world: &World) {
    let _ = sc.queue(MoveTo(world.player_c, world.player_l));
    let _ = sc.queue(Print('P'));
    let _ = sc.flush();
}

fn main() -> std::io::Result<()> {
    // Initialize the terminal.
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    // Get the size of the terminal.
    let (width, height) = size().expect("Failed to get terminal size");

    let sc: Stdout = stdout();
    let mut world = World {
        player_c: width/ 2,
        player_l: height - 1,
    };

    loop {

        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let key = read().expect("Failed to read event");
            match key? {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                #[cfg(feature = "bracketed-paste")]
                Event::Paste(data) => println!("Pasted {:?}", data),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }

        draw(&sc, &world);
    }
    Ok(())
}
