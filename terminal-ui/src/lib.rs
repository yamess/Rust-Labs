use std::{io};
use std::fmt::format;
use tui::{
    backend::Backend,
    widgets::{Block, Borders, BorderType, Tabs},
    layout::{Direction,Layout, Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    Terminal, Frame,
};
use crossterm::{
    event::{Event, KeyCode},
    event
};

pub struct App {
    pub titles: Vec<String>,
    pub index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            titles: vec![
                "Page 0".to_string(), "Page 1".to_string(),
                "Page 2".to_string(), "Page 3".to_string()
            ],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn previous(&mut self){
        if self.index == 0 {
            self.index = self.titles.len() - 1;
        } else {
            self.index -= 1;
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref()
        ).split(size);

    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .title("Main block with round corner")
    //     .title_alignment(Alignment::Center)
    //     .border_type(BorderType::Rounded);
    let block = Block::default().style(Style::default().bg(Color::White));
    f.render_widget(block, size);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first.to_uppercase(), Style::default().fg(Color::Yellow)),
                Span::styled(rest.to_lowercase(), Style::default().fg(Color::Green))
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black)
        );
    f.render_widget(tabs, chunks[0]);
    let inner = match app.index{
        v if v < 4 => Block::default().title(&*app.titles[0])
            .borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);

    // Top two inner blocks
    // let top_chunk = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(
    //         [
    //             Constraint::Percentage(50),
    //             Constraint::Percentage(50),
    //         ].as_ref()
    //     ).split(chunks[0]);

    // Top left inner block
    // let block = Block::default()
    //     .title(vec![
    //         Span::styled("With", Style::default().fg(Color::Yellow)),
    //         Span::from(" Background"),
    //     ])
    //     .style(Style::default().bg(Color::Red));
    // f.render_widget(block, top_chunk[0]);

    // Top right inner block
    // let block = Block::default()
    //     .title(Span::styled(
    //         "Styled title",
    //         Style::default()
    //             .fg(Color::White)
    //             .bg(Color::Red)
    //             .add_modifier(Modifier::ITALIC),
    //     ))
    //     .borders(Borders::ALL)
    //     .title_alignment(Alignment::Right);
    // f.render_widget(block, top_chunk[1]);

    // Bottom two inner blocks
    // let bottom_chunk = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(
    //         [
    //             Constraint::Percentage(50),
    //             Constraint::Percentage(50),
    //         ].as_ref()
    //     ).split(chunks[1]);

    // Bottom left block with all default borders
    // let block = Block::default()
    //     .title("With borders").borders(Borders::ALL);
    // f.render_widget(block, bottom_chunk[0]);

    // Bottom right block with styled left and right borders
    // let block = Block::default()
    //     .title("With styled borders and doubled borders")
    //     .border_style(Style::default().fg(Color::Cyan))
    //     .borders(Borders::LEFT | Borders::RIGHT)
    //     .border_type(BorderType::Double);
    // f.render_widget(block, bottom_chunk[1]);
}