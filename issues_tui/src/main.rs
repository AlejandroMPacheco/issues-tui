use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    text::Text,
    widgets::{Block, List, ListDirection, ListItem, Widget},
};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}

#[derive(Default)]
struct App {
    should_quit: bool,
    text: String,
}

// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//     loop {
//         terminal.draw(render)?;
//         if matches!(event::read()?, Event::Key(q)) {
//             break Ok(());
//         }
//     }
// }
impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&mut self, tui: &mut DefaultTerminal) -> Result<()> {
        tui.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }
    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                _ => {}
            }
        }
        Ok(())
    }
}
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = Constraint::from_lengths([1, 1, 2, 1]);
        let [greeting, timer, squares, position] = Layout::vertical(constraints).areas(area);

        // render an ephemeral greeting widget

        // render a reference to the timer widget

        // render a boxed widget containing red and blue squares
        #[cfg(feature = "unstable-widget-ref")]
        self.boxed_squares.render(squares, buf);

        // render a mutable reference to the green square widget

        // Display the dynamically updated position of the green square

        // square_position.render(position, buf);
    }
}
// fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
//     let [area] = Layout::horizontal([horizontal])
//         .flex(Flex::Center):wa!
//         .areas(area);
//     let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
//     area
// }
// fn render(frame: &mut Frame) {
//     let text = Text::raw("hi");
//     let area = center(
//         frame.area(),
//         Constraint::Percentage(20),
//         Constraint::Length(3),
//     );
//     frame.render_widget(text, area);
// }
