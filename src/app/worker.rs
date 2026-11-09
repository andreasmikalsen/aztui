use crate::app::app::{App};
use crate::azure::account::AzureAccount;
use crate::azure::token::AccessToken;
use crate::domain::errors::SwitchError;
use crate::domain::load_state::LoadState;

pub enum AppMessage {
    // Dashboard
    AccountLoaded(Result<AzureAccount, String>),
    AccountListLoaded(Result<Vec<AzureAccount>, String>),
    AccountSet(Result<AzureAccount, SwitchError>),

    // Token
    TokenRequest(Result<AccessToken, String>),
}

pub fn process_messages(app: &mut App) {
    while let Ok(message) = app.worker_receiver.try_recv() {
        match message {
            AppMessage::AccountLoaded(result) => {
                app.azure_account = LoadState::from_result(result);
                app.azure_account_loading = false;
            }
            AppMessage::AccountListLoaded(result) => {
                let selected = match (&result, &app.azure_account) {
                    (Ok(result), LoadState::Ready(account)) => {
                        result
                            .iter()
                            .position(|item| item.id == account.id)
                            .unwrap_or(0)
                    }
                    _ => 0
                };
                app.dashboard.subscription_list = LoadState::from_result(result);
                app.dashboard.subscription_list_loading = false;
                app.dashboard.selected_subscription = selected;
            }
            AppMessage::AccountSet(result) => {
                app.dashboard.subscription_changing = false;
                match result {
                    Ok(account) => {
                        app.azure_account = LoadState::Ready(account);
                        app.dashboard.selector_open = false;
                    }
                    Err(SwitchError::SetFailed(error)) => {
                        app.dashboard.switch_error = Some(format!("Could not switch subscription. Error: {}", error));
                    }
                    Err(SwitchError::RefetchFailed(error)) => {
                        app.dashboard.switch_error = Some(format!("Switched, but could not load account. Error: {}", error));
                        app.azure_account = LoadState::Error(String::from("Could not load account after switching."));
                    }
                }
            }
            AppMessage::TokenRequest(result) => {
                app.token.token_fetching = false;
                app.token.token = LoadState::from_result(result);
            }
        }
    }
}