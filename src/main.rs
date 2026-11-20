use color_eyre::{
    Result,
    eyre::{ContextCompat, Ok, WrapErr, bail},
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Layout,
    prelude::*,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_res = App::default().run(&mut terminal);
    //ratatui::run(|terminal| App::default().run(terminal));
    ratatui::restore();
    app_res
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed");
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),

            _ => Ok(()),
        };
        Ok(())
    }

    fn handle_event(&mut self, event: KeyEvent) -> Result<()> {
        match event.code {
            KeyCode::Up => self.increment_counter()?,
            KeyCode::Down => self.decrement_counter()?,
            KeyCode::Char('q') => self.exit()?,
            _ => {}
        }
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        self.counter = self
            .counter
            .checked_sub(1)
            .wrap_err("attempt to subtract with overflow")?;
        Ok(())
    }

    fn increment_counter(&mut self) -> Result<()> {
        self.counter = self.counter.checked_add(1).wrap_err("over the limit")?;
        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.exit = true;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titel = Line::from(" Counter app test app ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(titel.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render_test() {
        let app = App::default();
        let mut buff = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buff.area, &mut buff);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter app test app ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();

        expected.set_style(Rect::new(14, 0, 22, 1), title);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buff, expected);
    }

    #[test]
    fn handle_key_event_test() {
        let mut app = App::default();
        app.handle_event(KeyCode::Up.into()).unwrap();
        assert_eq!(app.counter, 1);

        app.handle_event(KeyCode::Down.into()).unwrap();
        assert_eq!(app.counter, 0);

        app.handle_event(KeyCode::Char('q').into()).unwrap();
        assert!(app.exit);
    }

    // #[test]
    // #[should_panic(expected = "attempt to subtract with overflow")]
    // fn handle_key_event_test_panic() {
    //     let mut app = App::default();
    //     let _ = app.handle_event(KeyCode::Down.into());
    // }

     #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.handle_event(KeyCode::Up.into()).is_ok());
        assert!(app.handle_event(KeyCode::Up.into()).is_ok());
    }
}
