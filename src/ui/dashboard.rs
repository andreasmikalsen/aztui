use crate::app::app::App;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Modifier;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Clear, List, ListState, Paragraph, Wrap};
use log::error;
use crate::domain::load_state::LoadState;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let text = match &app.azure_account {
        LoadState::Ready(account) => {
            format!("User: {}    Subscription: {}    Tenant: {}", account.user.name, account.name, account.tenant_id)
        }
        LoadState::Error(e) => {
            format!("Could not load Azure account. Error: {}", e)
        }
        LoadState::Loading => {
            "Loading account...".to_string()
        }
    };
    frame.render_widget(
        Paragraph::new(text),
        area,
    );

    if app.dashboard.selector_open {
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(50));
        frame.render_widget(Clear, centered_area);

        let popup_block = Block::bordered().title("Azure subscriptions");
        let inner = popup_block.inner(centered_area);

        frame.render_widget(popup_block, centered_area);

        let error_height = if app.dashboard.switch_error.is_some() {
            3
        } else {
            0
        };

        let [list_area, error_area] = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(error_height),
        ]).areas(inner);

        match &app.dashboard.subscription_list {
            LoadState::Ready(subscription_list) => {
                if app.dashboard.subscription_changing {
                    frame.render_widget(
                        Paragraph::new("Switching..."),
                        list_area,
                    );
                } else if subscription_list.is_empty() {
                    frame.render_widget(
                        Paragraph::new("No accounts to show"),
                        list_area,
                    );
                } else {
                    let names = subscription_list
                        .iter()
                        .map(|s| {
                            let is_current = if let Some(current_account) = app.azure_account.value() {
                                s.id == current_account.id
                            } else {
                                false
                            };

                            return format!("{}{}", if is_current { "* " } else { "" }, s.name.as_str());
                        });
                    let mut list_state = ListState::default()
                        .with_selected(Some(app.dashboard.selected_subscription));

                    let list = List::new(names)
                        .highlight_style(
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD))
                        .highlight_symbol("▶ ");
                    frame.render_stateful_widget(list, list_area, &mut list_state);
                }
            }
            LoadState::Error(e) => {
                frame.render_widget(
                    Paragraph::new(format!("Could not load Azure account. Error: {}", e)),
                    list_area,
                );
            }
            LoadState::Loading => {
                frame.render_widget(
                    Paragraph::new("Subscription list loading..."),
                    list_area
                )
            }
        };
        if let Some(error) = &app.dashboard.switch_error {
            frame.render_widget(
                Paragraph::new(error.as_str())
                    .style(Style::default().fg(Color::Red))
                    .wrap(Wrap { trim: true }),
                error_area
            )
        }
    }
}
