use ratatui::{
    Frame,
    layout::{Constraint},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::azure::keyvault::Secret;

pub fn render_secrets(
    frame: &mut Frame,
    secrets: &[Secret],
) {
    let area = frame.area();

    let rows = secrets.iter().map(|secret| {
        Row::new(vec![
            Cell::from(secret.name.clone()),
            Cell::from("Available"),
            Cell::from("••••••••"),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
        .header(
            Row::new(["Secret", "Status", "Value"])
                .style(ratatui::style::Style::default().bold())
        )
        .block(
            Block::default()
                .title(" Key Vault Secrets ")
                .borders(Borders::ALL)
        );

    frame.render_widget(table, area);
}