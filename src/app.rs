use ratatui::{prelude::*, widgets::*};
use crate::tui::Event;
use crate::spinner::Spinner;
use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
  StartOrSplit,
  Stop,
  Tick,
  Quit,
}

pub struct App {
    counter: i64,
    spinner: Spinner,
    pub should_quit: bool,
    //action_tx: UnboundedSender<Action>,
}

impl App {
    pub fn new() -> Self {
      Self {
        counter : 0,
        spinner : Spinner::default(),
        should_quit : false
      }
    }

    pub fn ui(&mut self, f: &mut Frame<'_>) {
      let area = f.size();
      f.render_widget(
        Paragraph::new(format!("Spinner!.\n\n{}", self.spinner.next().unwrap_or(' '),))
          .block(
            Block::default()
              .title("ratatui async counter app")
              .title_alignment(Alignment::Center)
              .borders(Borders::ALL)
              .border_type(BorderType::Rounded),
          )
          .style(Style::default().fg(Color::Cyan))
          .alignment(Alignment::Center),
        area,
      );
    }

    pub fn handle_event(&self, event: Event) -> Result<Message> {
      let msg = match event {
        Event::Key(key) => {
          match key.code {
            crossterm::event::KeyCode::Char('q') => Message::Quit,
            crossterm::event::KeyCode::Char(' ') => Message::StartOrSplit,
            crossterm::event::KeyCode::Char('s') => Message::Stop,
            _ => Message::Tick,
          }
        },
        _ => Message::Tick,
      };
      Ok(msg)
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
      match message {
        Message::StartOrSplit => println!("Start or split"),
        Message::Quit => self.should_quit = true,
        Message::Tick => self.counter += 1,
        Message::Stop => println!("Stop")
      }
      Ok(())
    }
}

