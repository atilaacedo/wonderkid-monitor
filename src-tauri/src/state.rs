use std::sync::{Arc, Mutex};
use crate::domain::player::Player;

pub struct AppState {
    pub player: Arc<Mutex<Player>>,
}