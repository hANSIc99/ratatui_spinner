use ratatui::{prelude::*, widgets::*};

pub struct App {
    counter: i64,
    should_quit: bool,
    //action_tx: UnboundedSender<Action>,
}

impl App {
    pub fn new() -> Self {
      Self {
        counter : 0,
        should_quit : false
      }
    }

    pub fn ui(&mut self, f: &mut Frame<'_>) {
      let area = f.size();
      f.render_widget(
        Paragraph::new(format!("Spinner!.\n\n{}", self.counter,))
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
}

