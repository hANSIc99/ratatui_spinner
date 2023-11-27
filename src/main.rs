pub mod tui;
pub mod app;
use std::{iter::Iterator, future::Future, task::Poll};
//use anyhow::Result;
use color_eyre::eyre::{eyre, Result};
use app::App;
use tokio::time::{Instant, Interval};
use tui::Tui;
use ratatui::{backend::CrosstermBackend as Backend};
struct Spinner {
    m_spinner   : Vec<char>,
    m_iter      : usize,
    m_interval  : Interval
}

impl Spinner {
    fn new() -> Self {
        Self {
            m_spinner : vec!['|', '/', '-', '\\'],
            m_iter : 0,
            m_interval : tokio::time::interval(std::time::Duration::from_millis(500))
        }
    }

    pub async fn tick(&mut self) -> Instant {
        println!("hello");
        self.m_interval.tick().await
    }
}

impl Iterator for Spinner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.m_iter += 1;
        self.m_iter &= 0x3;

        Some(self.m_spinner[self.m_iter])
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}



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



    loop {
        tui.terminal.draw(|f| app.ui(f))?;
        // let event = tui.next().await.ok_or(eyre!("Unable to get event"))?; // blocks until next event (e.g. keystroke or tick)
        // let message = self.handle_event(event)?;
        // self.update(message)?; // Update App state
      }
      tui.exit()?;
      //Ok(())










    let mut spinner: Spinner = Spinner::default();

    //let mut spin: char = spinner.await;
    //println!("{}", spin);
    // let mut spin: char = spinner.await;
    // println!("{}", spin);
    // let mut spin: char = spinner.await;
    // println!("{}", spin);
    // let mut spin: char = spinner.await;
    // println!("{}", spin);
    let tick_rate = std::time::Duration::from_millis(100);
    let mut interval = tokio::time::interval(tick_rate);

    loop {
        let delay = interval.tick(); // ticker() returns a future to wait on
        //let delay = spinner.tick(); // ticker() returns a future to wait on

        tokio::select! {
            _ = delay => {
                println!("tick");
            }
        }
    }
    // = spinner.await;
    
//     tokio::select! { 
//         test = spinner.poll() => {}
//     }
//     // let res = tokio::try_join!(
//     //     spin = spinner(),
//     // );

//     match res {
//         Ok((first, second)) => {
//             // do something with the values
//         }
//         Err(err) => {
//            println!("processing failed; error = {}", err);
//         }
//    }

    
    // 
    // while let Some(spin) = spinner.next() {
    //     println!("{}", spin);
    // }
    Ok(())
}
