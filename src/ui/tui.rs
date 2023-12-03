use ratatui::prelude::CrosstermBackend;

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;
pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;