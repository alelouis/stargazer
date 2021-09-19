/// Stage for our systems
pub const APP_STATE_STAGE: &str = "app_state_stage";

/// States
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    Menu,
    Stars,
}
