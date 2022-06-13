use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{borrow::BorrowMut, error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

struct ColumnState<T> {
    items: [T; 3],
    state: ListState,
}

impl<T> ColumnState<T> {
    fn new(items: [T; 3]) -> ColumnState<T> {
        ColumnState {
            items,
            state: ListState::default(),
        }
    }

    pub fn next(&mut self, row: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < self.items.len() - 1 {
                    i + 1
                } else {
                    i
                }
            }
            None => row,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, row: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => row,
        };
        self.state.select(Some(i));
    }

    pub fn return_selected(&mut self) -> usize {
        let _i = match self.state.selected() {
            Some(_i) => return _i,
            None => return 9,
        };
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut left_column = ColumnState::new([
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
    ]);

    let mut central_column = ColumnState::new([
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
    ]);

    let mut right_column = ColumnState::new([
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
        ListItem::new(" ").style(Style::default().bg(Color::Red)),
    ]);

    let mut column_number: usize = 0;
    let mut row: usize = 0;
    loop {
        terminal.draw(|f| {
            ui(
                f,
                &mut [&mut left_column, &mut central_column, &mut right_column],
            )
        })?;
        let mut columns = [&mut left_column, &mut central_column, &mut right_column];
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    columns[column_number].next(row);
                    row = columns[column_number].return_selected();
                }
                KeyCode::Up => {
                    columns[column_number].previous(row);
                    row = columns[column_number].return_selected();
                }
                KeyCode::Right => {
                    if column_number < 2 {
                        columns[column_number].unselect();
                        column_number += 1;
                        columns[column_number].next(row);
                    }
                }
                KeyCode::Left => {
                    if column_number > 0 {
                        columns[column_number].unselect();
                        column_number -= 1;
                        columns[column_number].next(row);
                    }
                }
                KeyCode::Char('w') => toggle_white(&mut columns),
                _ => {}
            }
        }
    }
    exit(&mut terminal)?;
    return Ok(());
}

fn ui<B: Backend>(f: &mut Frame<B>, cols: &mut [&mut ColumnState<ListItem>; 3]) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let smaller_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);
    let grid_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[0]);
    let list_left = List::new(cols[0].items.clone())
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list_left, grid_layout[0], &mut cols[0].state);

    let list_center = List::new(cols[1].items.clone())
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list_center, grid_layout[1], &mut cols[1].state);

    let list_right = List::new(cols[2].items.clone())
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list_right, grid_layout[2], &mut cols[2].state);

    let commands_list = [
        ListItem::new("Use arrows to select slot"),
        ListItem::new("Press 'w' to select white"),
        ListItem::new("Press 'b' to select black"),
        ListItem::new("Press 'q' to exit"),
    ];
    let commands = List::new(commands_list)
        .block(
            Block::default()
                .title("Available Commands")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(commands, smaller_chunks[0]);
}

fn toggle_white(cols: &mut [&mut ColumnState<ListItem>; 3]) {
    for col in cols.into_iter() {
        let row = col.return_selected();
        if row != 9 {
            println!("{}", row);
            col.items[row].style(Style::default().bg(Color::White));
        }
    }
}

fn exit(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
