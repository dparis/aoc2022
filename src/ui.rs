use crate::app::App;
use crate::days::Day;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

fn header_row<'a>() -> Row<'a> {
    return Row::new(vec!["Day", "Part 1", "Part 2", "Stars"])
        .style(Style::default().add_modifier(Modifier::BOLD));
}

fn day_to_row<'a>(day: &mut Day) -> Row<'a> {
    let solution_1 = day.part_1.solution().unwrap_or_else(|| String::from(""));
    let solution_2 = day.part_2.solution().unwrap_or_else(|| String::from(""));

    Row::new(vec![day.label(), solution_1, solution_2, day.stars()])
}

fn table_rows<'a>(app: &mut App) -> Vec<Row<'a>> {
    let days = &mut app.day_table.items;

    return days.iter_mut().map(day_to_row).collect();
}

fn draw_table<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let app_wrapper = Block::default().borders(Borders::ALL).title(app.title);

    let rows = table_rows(app);

    let table = Table::new(rows)
        .header(header_row())
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
        .style(Style::default().fg(Color::White))
        .column_spacing(1)
        .highlight_style(Style::default().fg(Color::Green))
        .block(app_wrapper);

    f.render_stateful_widget(table, chunks[0], &mut app.day_table.state);
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    draw_table(f, app);
}
