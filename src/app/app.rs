use crate::app::worker::AppMessage;
use crate::azure::account::{AzureAccount, get_account, get_account_list, set_account};
use crate::azure::token::{get_access_token, TokenTarget};
use crate::domain::errors::SwitchError;
use crate::domain::load_state::LoadState;
use crate::domain::states::{DashboardState, TokenState};
use crate::ui;
use crate::ui::navigation::Tab;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use std::sync::mpsc::{Receiver, Sender};

pub struct App {
    pub running: bool,
    pub active_tab: Tab,

    pub azure_account: LoadState<AzureAccount>,
    pub azure_account_loading: bool,

    pub dashboard: DashboardState,
    pub token: TokenState,

    pub worker_sender: Sender<AppMessage>,
    pub worker_receiver: Receiver<AppMessage>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut app = Self {
            running: true,
            azure_account: LoadState::Loading,
            azure_account_loading: false,
            active_tab: Tab::Dashboard,
            worker_sender: sender,
            worker_receiver: receiver,
            dashboard: DashboardState {
                selected_subscription: 0,
                selector_open: false,
                subscription_list: LoadState::Loading,

                subscription_list_loading: false,
                subscription_changing: false,
                switch_error: None,
            },
            token: TokenState {
                token: LoadState::Idle,
                selected_target: 0,
                token_fetching: false,
            },
        };
        app.refresh_account();
        Ok(app)
    }

    pub fn next_tab(&mut self) {
        let index = Tab::ALL
            .iter()
            .position(|t| *t == self.active_tab)
            .unwrap_or(0);

        self.active_tab = Tab::ALL[(index + 1) % Tab::ALL.len()];
    }

    pub fn previous_tab(&mut self) {
        let index = Tab::ALL
            .iter()
            .position(|t| *t == self.active_tab)
            .unwrap_or(0);

        self.active_tab = Tab::ALL[(index + Tab::ALL.len() - 1) % Tab::ALL.len()];
    }

    pub fn render(&self, frame: &mut Frame) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Active tab
                Constraint::Length(3),  // Footer
            ])
            .split(frame.area());

        ui::navigation::render_header(frame, areas[0], self.active_tab);
        frame.render_widget("[Ctrl+h/l] Switch tab    [q] Quit    [r] Refresh AZ Account    [s] Switch account", areas[2]);

        match self.active_tab {
            Tab::Dashboard => {
                ui::dashboard::render(frame, areas[1], self);
            }
            Tab::Settings => frame.render_widget("Settings coming soon", areas[1]),
            Tab::Tokens => ui::tokens::render(frame, areas[1], self),
            Tab::Vaults => frame.render_widget("Key Vaults coming soon", areas[1]),
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn refresh_account(&mut self) {
        if self.azure_account_loading || self.dashboard.subscription_changing {
            return;
        }
        self.azure_account_loading = true;
        self.azure_account = LoadState::Loading;
        let sender = self.worker_sender.clone();
        std::thread::spawn(move || {
            let result = get_account().map_err(|e| e.to_string());
            let _ = sender.send(AppMessage::AccountLoaded(result));
        });
    }

    fn set_account_list(&mut self) {
        if self.dashboard.subscription_list_loading {
            return;
        }
        self.dashboard.subscription_list_loading = true;
        self.dashboard.subscription_list = LoadState::Loading;
        let sender = self.worker_sender.clone();
        std::thread::spawn(move || {
            let result = get_account_list().map_err(|e| e.to_string());
            let _ = sender.send(AppMessage::AccountListLoaded(result));
        });
    }

    fn set_active_account(&mut self) {
        if self.dashboard.subscription_changing || self.azure_account_loading {
            return;
        }

        let Some(subscription_id) = self.dashboard.subscription_list
            .value()
            .and_then(|list| list.get(self.dashboard.selected_subscription))
            .map(|account| account.id.clone())
        else {
            return;
        };

        self.dashboard.subscription_changing = true;
        self.dashboard.switch_error = None;

        let sender = self.worker_sender.clone();
        std::thread::spawn(move || {
            let result = match set_account(subscription_id) {
                Ok(()) => get_account().map_err(|e| SwitchError::RefetchFailed(e.to_string())),
                Err(error) => Err(SwitchError::SetFailed(error.to_string())),
            };
            let _ = sender.send(AppMessage::AccountSet(result));
        });
    }

    fn fetch_access_token(&mut self) {
        if self.token.token_fetching {
            return;
        }
        let target = TokenTarget::ALL[self.token.selected_target];
        let subscription_id = match &self.azure_account {
            LoadState::Ready(value) => value.id.clone(),
            _ => {
                self.token.token = LoadState::Error(String::from("Subscription is not loaded"));
                return;
            }
        };

        self.token.token_fetching = true;
        self.token.token = LoadState::Loading;
        let sender = self.worker_sender.clone();
        std::thread::spawn(move || {
            let result = get_access_token(target, subscription_id.as_str()).map_err(|e| e.to_string());
            let _ = sender.send(AppMessage::TokenRequest(result));
        });
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.dashboard.selector_open {
            if self.dashboard.subscription_changing {
                return;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.dashboard.selector_open = false;
                }
                KeyCode::Char('j') => {
                    match &self.dashboard.subscription_list {
                        LoadState::Ready(list) => {
                            if list.is_empty() || self.dashboard.selected_subscription + 1 == list.len() {
                                return;
                            }
                            self.dashboard.selected_subscription = self.dashboard.selected_subscription + 1;
                        }
                        _ => {
                            return;
                        }
                    }
                }
                KeyCode::Char('k') => {
                    self.dashboard.selected_subscription = self.dashboard.selected_subscription.saturating_sub(1);
                }
                KeyCode::Enter => {
                    self.set_active_account();
                }
                _ => {}
            }
            return;
        }
        // Global navigation
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('h') => {
                    self.previous_tab();
                    return;
                }

                KeyCode::Char('l') => {
                    self.next_tab();
                    return;
                }

                _ => {
                    return;
                }
            }
        }
        if self.active_tab == Tab::Tokens {
            match key.code {
                KeyCode::Char('j') => {
                    if self.token.selected_target + 1 < TokenTarget::ALL.len()  {
                        self.token.selected_target = self.token.selected_target + 1;
                    }
                }
                KeyCode::Char('k') => {
                    self.token.selected_target = self.token.selected_target.saturating_sub(1);
                }
                KeyCode::Char('y') => {
                    // TODO Yank token
                }
                KeyCode::Enter => {
                    self.fetch_access_token();
                }
                _ => {}
            }
        }
        if self.active_tab == Tab::Dashboard {
            if key.code == KeyCode::Char('s') {
                self.set_account_list();
                self.dashboard.switch_error = None;
                self.dashboard.selector_open = true;
                return;
            }
        }
        match key.code {
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.previous_tab(),
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Char('r') => {
                self.refresh_account();
                return;
            },
            _ => {}
        }
    }
}
