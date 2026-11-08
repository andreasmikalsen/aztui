use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Tabs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Tokens,
    Vaults,
    Settings,
}

impl Tab {
    pub const ALL: [Tab; 4] = [Tab::Dashboard, Tab::Tokens, Tab::Vaults, Tab::Settings];

    pub fn title(&self) -> &str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Tokens => "Tokens",
            Tab::Vaults => "Key Vaults",
            Tab::Settings => "Settings",
        }
    }
}

pub fn render_header(frame: &mut Frame, area: Rect, active_tab: Tab) {
    let titles = Tab::ALL
        .iter()
        .map(|tab| tab.title())
        .collect::<Vec<&str>>();
    let current_index = Tab::ALL
        .iter()
        .position(|tab| *tab == active_tab)
        .unwrap_or(0);

    let tabs_widget = Tabs::new(titles)
        .select(current_index)
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs_widget, area);
}