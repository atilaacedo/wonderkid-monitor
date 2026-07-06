#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{Manager, State, Emitter}; // <-- Emitter adicionado para o Tauri v2

use domain::activity::Activity;
use domain::player::Player;

// Nosso Estado Global seguro
struct AppState {
    player: Arc<Mutex<Player>>,
}

// O comando que o Vue chama na montagem da tela
#[tauri::command]
fn get_player_status(state: State<'_, AppState>) -> Player {
    let player_lock = state.player.lock().unwrap();
    player_lock.clone()
}

fn main() {
    let wonderkid = Arc::new(Mutex::new(Player::new("Átila")));
    let player_for_thread = Arc::clone(&wonderkid);

    // Encadeamento do Builder: repare que não há ponto e vírgula 
    // entre os métodos .manage(), .setup(), .invoke_handler() e .run()
    tauri::Builder::default()
        .manage(AppState { player: wonderkid })
        .setup(move |app| {
            let app_handle = app.handle().clone();

            thread::spawn(move || {
                let simulacao_janelas = vec![
                    "Visual Studio Code - projeto_rust",
                    "Mozilla Firefox - Vue Docs",
                    "Steam - Football Manager",
                    "Twitter",
                ];
                let mut ciclo = 0;

                loop {
                    let janela_atual = simulacao_janelas[ciclo % simulacao_janelas.len()];
                    ciclo += 1;

                    let atividade = Activity::classify(janela_atual);

                    {
                        // Escopo do Mutex: tranca, atualiza e destranca automaticamente no fim das chaves
                        let mut player_lock = player_for_thread.lock().unwrap();
                        player_lock.process_activity(atividade);
                        
                        // Emite o evento pro Vue atualizar a tela sozinho
                        app_handle.emit("player-updated", player_lock.clone()).unwrap();
                    }

                    // Espera 3 segundos e roda o ciclo de novo
                    thread::sleep(Duration::from_secs(3));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_player_status])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar a aplicação Tauri");
}