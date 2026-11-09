use crate::azure::account::AzureAccount;
use crate::azure::token::AccessToken;
use crate::domain::load_state::LoadState;

pub struct DashboardState {
    pub selected_subscription: usize,
    pub selector_open: bool,
    pub subscription_list: LoadState<Vec<AzureAccount>>,

    pub subscription_list_loading: bool,
    pub subscription_changing: bool,
    pub switch_error: Option<String>,
}

pub struct TokenState {
    pub token: LoadState<AccessToken>,
    pub selected_target: usize,
    pub token_fetching: bool,
}