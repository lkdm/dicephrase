use arboard::Clipboard;

pub struct ClipboardGuard {
    clipboard: Clipboard,
    generated: String,
    previous: Option<String>,
}

impl ClipboardGuard {
    pub fn new(value: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut clipboard = Clipboard::new()?;

        let previous = clipboard.get_text().ok();

        clipboard.set_text(value.clone())?;

        Ok(Self {
            clipboard,
            generated: value,
            previous,
        })
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        match self.clipboard.get_text() {
            Ok(current) if current == self.generated => {
                let previous = self.previous.take().unwrap_or_default();

                match self.clipboard.set_text(previous) {
                    Ok(()) => eprintln!("Clipboard restored."),
                    Err(err) => eprintln!("Failed to restore clipboard: {err}"),
                }
            }
            Ok(_) => {
                eprintln!("Clipboard changed externally; leaving it untouched.");
            }
            Err(err) => {
                eprintln!("Failed to read clipboard during cleanup: {err}");
            }
        }
    }
}
