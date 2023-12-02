use std::time::Duration;
use color_eyre::eyre::{eyre, Result};
use ratatui::{
    backend::CrosstermBackend as Backend,
    prelude::*,
     widgets::*,
    Terminal,
    widgets::Paragraph,
};
use futures::{
    FutureExt, // fuse()
    StreamExt // next()
};
//use anyhow::Result;
use tokio_util;
use tokio::{sync::mpsc::{self, UnboundedReceiver, UnboundedSender}, task::JoinHandle};


#[derive(Clone, Debug)]
pub enum Event {
  Error,
  Tick,
  Key(crossterm::event::KeyEvent),
}
pub struct Tui {
    pub terminal: Terminal<Backend<std::io::Stderr>>,
    pub task: tokio::task::JoinHandle<()>,
    pub cancellation_token: tokio_util::sync::CancellationToken,
    pub event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    pub event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
}

impl Tui {
  pub fn new() -> Result<Tui> {
    let mut terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
    terminal.clear()?;
    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
    let cancellation_token = tokio_util::sync::CancellationToken::new();
    let task = tokio::spawn(async {});
    Ok(Self { terminal, task, cancellation_token, event_rx, event_tx })
  }

  pub async fn next(&mut self) -> Result<Event> {
    self.event_rx.recv().await.ok_or(eyre!("Unable to get event"))
  }
  
  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen, crossterm::cursor::Hide)?;
    self.start();
    Ok(())
  }

  pub fn exit(&self) -> Result<()> {
    self.stop()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen, crossterm::cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
  }

  pub fn cancel(&self) {
    self.cancellation_token.cancel();
  }

  pub fn stop(&self) -> Result<()> {
    self.cancel();
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_millis(250));
      counter += 1;
      if counter > 5 {
        self.task.abort();
      }
      if counter > 10 {
        //log::error!("Failed to abort task for unknown reason");
        return Err(eyre!("Unable to abort task"));
      }
    }
    Ok(())
  }

  pub fn start(&mut self) {
    let tick_rate = std::time::Duration::from_millis(500);
    self.cancel();
    self.cancellation_token = tokio_util::sync::CancellationToken::new();
    let _cancellation_token = self.cancellation_token.clone();
    let _event_tx = self.event_tx.clone();
    self.task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate); // Creates new Interval that yields with interval of duration. The first tick completes immediately.
      loop {
        let delay = interval.tick(); // The first tick completes immediately.
        let crossterm_event = reader.next().fuse();
        tokio::select! { // Waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.
          _ = _cancellation_token.cancelled() => { // [1.] cancelled() == future, gets fullfilled when cancellation is requested
            break; // break endless loop
          }
          maybe_event = crossterm_event => { // [2.] Keystroke
            match maybe_event {
              Some(Ok(crossterm::event::Event::Key(key))) => {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    _event_tx.send(Event::Key(key)).unwrap(); // Send keystroke back to rx_stream (-> next() method)
                }
              }
              Some(Ok(_)) => { }
              Some(Err(_)) => {
                _event_tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => { // [3. tick passed]
              _event_tx.send(Event::Tick).unwrap();
          },
        }
      }
    });
  }
}
