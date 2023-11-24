use ratatui::{
    backend::CrosstermBackend as Backend,
    Terminal,
    widgets::Paragraph,
};
use tokio_util;
use tokio::{sync::mpsc::{self, UnboundedReceiver, UnboundedSender}, task::JoinHandle};


#[derive(Clone, Debug)]
pub enum Event {
  Error,
  Tick,
  Key(crossterm::event::KeyEvent),
}
struct Tui {
    pub terminal: Terminal<Backend<std::io::Stderr>>,
    pub task: tokio::task::JoinHandle<()>,
    pub cancellation_token: tokio_util::sync::CancellationToken,
    pub event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    pub event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
}

impl Tui {
    
}