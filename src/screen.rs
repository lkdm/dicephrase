use std::error::Error;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new(enabled: bool) -> Result<Option<Self>, Box<dyn Error>> {
        if enabled {
            let mut stdout = std::io::stdout();
            execute!(stdout, EnterAlternateScreen)?;
            execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
            Ok(Some(Self))
        } else {
            Ok(None)
        }
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
    }
}
