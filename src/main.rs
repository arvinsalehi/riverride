use std::io::{stdout, Stdout, Write};

use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};

fn main() -> std::io::Result<()> {
    let mut sc: Stdout = stdout();
    sc.execute(Print("Styled Text Here"))?;
    Ok(())
}
