use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{List, ListState};
use crate::app::app::App;
use crate::azure::token::TokenTarget;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let titles = TokenTarget::ALL.iter().map(|target| target.title());
    let mut list_state = ListState::default()
        .with_selected(Some(app.token.selected_target));

    let list = List::new(titles)
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    frame.render_stateful_widget(list, area, &mut list_state)
}