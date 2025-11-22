use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState, Paragraph, Widget},
};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

struct App {
    should_quit: bool,
    title: String,
    items: Vec<String>,
    list_state: ListState,
    last_tick: Instant,
}

struct IssuesList {
    list: Vec<String>,
}

impl App {
    //elements
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            title: "Issues-Tui".to_string(),
            should_quit: false,
            items: vec![
                "First item".into(),
                "Second item".into(),
                "Third item".into(),
                "Fourth item".into(),
            ],
            list_state,
            last_tick: Instant::now(),
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, tui: &mut DefaultTerminal) -> Result<()> {
        tui.draw(|f| {
            let size = f.area();
            let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(size);

            let header = Paragraph::new(self.title.as_str())
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = self
                .items
                .iter()
                .map(|s| ListItem::new(s.as_str()))
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Items"))
                .highlight_symbol(">> ")
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

            // render a stateful list (requires ListState in App)
            f.render_stateful_widget(list, chunks[1], &mut self.list_state);
        })?;

        Ok(())
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                _ => {}
            }
        }
        Ok(())
    }
}
