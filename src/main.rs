pub mod tui;
pub mod app;
pub mod spinner;
use std::{iter::Iterator, future::Future, task::Poll};
//use anyhow::Result;
use color_eyre::eyre::{eyre, Result};
use app::{App, Message};
use tokio::time::{Instant, Interval};
use tui::Tui;
use ratatui::{backend::CrosstermBackend as Backend};


// Fix for VsCode https://stackoverflow.com/questions/75926539/unicode-not-showing-properly-in-git-bash-when-debugging-in-visual-studio-code

// impl Future for Spinner {
//     type Output = char;

//     fn poll(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         let spin = self.as_mut().next();
//         match spin {
//             Some(c_spin) => {
//                 return Poll::Ready(c_spin);
//             },
//             None => {
//                 return Poll::Pending
//             }
//         }
//     }
// }
// https://github.com/tokio-rs/tokio/blob/master/tokio/src/time/interval.rs
// https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html





#[tokio::main]
async fn main() -> Result<()> {

    let mut app = App::new();
    let mut tui = Tui::new()?;
    tui.enter()?; // >>> tui.start()



    while !app.should_quit {

        let e = tui.next().await?;
        let message = app.handle_event(e)?;
        app.update(message)?;
        tui.terminal.draw(|f| app.ui(f))?;
        // let event = tui.next().await.ok_or(eyre!("Unable to get event"))?; // blocks until next event (e.g. keystroke or tick)
        // let message = self.handle_event(event)?;
        // self.update(message)?; // Update App state
      }
      tui.exit()?;

    Ok(())
}
