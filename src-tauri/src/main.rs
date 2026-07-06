#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod infra; 
mod state; 

use std::sync::{Arc, Mutex};
use tauri::{State};

use domain::player::Player;
use infra::monitor::start_background_loop;
use state::AppState;

#[tauri::command]
fn get_player_status(state: State<'_, AppState>) -> Player {
    let player_lock = state.player.lock().unwrap();
    player_lock.clone()
}

fn main() {
    let wonderkid = Arc::new(Mutex::new(Player::new("Átila")));
    let player_for_thread = Arc::clone(&wonderkid);

    tauri::Builder::default()
        .manage(AppState { player: wonderkid })
        .setup(move |app| {
            let app_handle = app.handle().clone();

            start_background_loop(app_handle, player_for_thread);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_player_status])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar a aplicação Tauri");
}