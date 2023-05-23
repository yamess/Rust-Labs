use std::{io};
use tui::{
    backend::Backend,
    widgets::{Block, Borders, BorderType},
    layout::{Direction,Layout, Alignment},
    Terminal, Frame,
};
use crossterm::{
    event::{Event, KeyCode},
    event
};
use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;


pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            titles: vec!["Page 1", "Page 2", "Page 3"],
            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len()
    }
    pub fn previous(&mut self){
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>{
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code{
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => app.previous(),
                KeyCode::Right => app.next(),
                _ => {}
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App){
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main block with round corner")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        ).split(size);

    // Top two inner blocks
    let top_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        ).split(chunks[0]);

    // Top left inner block
    let block = Block::default()
        .title(vec![
            Span::styled("With", Style::default().fg(Color::Yellow)),
            Span::from(" Background"),
        ])
        .style(Style::default().bg(Color::Red));
    f.render_widget(block, top_chunk[0]);

    // Top right inner block
    let block = Block::default()
        .title(Span::styled(
            "Styled title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::ITALIC),
        ))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Right);
    f.render_widget(block, top_chunk[1]);

    // Bottom two inner blocks
    let bottom_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        ).split(chunks[1]);

    // Bottom left block with all default borders
    let block = Block::default()
        .title("With borders").borders(Borders::ALL);
    f.render_widget(block, bottom_chunk[0]);

    // Bottom right block with styled left and right borders
    let block = Block::default()
        .title("With styled borders and doubled borders")
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Double);
    f.render_widget(block, bottom_chunk[1]);
}