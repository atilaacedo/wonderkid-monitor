use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::domain::activity::Activity;
use crate::domain::player::Player;


pub fn start_background_loop(app_handle: AppHandle, player_state: Arc<Mutex<Player>>) {
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
                // Escopo do Mutex: tranca, atualiza e destranca
                let mut player_lock = player_state.lock().unwrap();
                player_lock.process_activity(atividade);
                
                // Emite o evento silenciando possíveis erros de desconexão com o frontend (usando let _)
                let _ = app_handle.emit("player-updated", player_lock.clone());
            }

            thread::sleep(Duration::from_secs(3));
        }
    });
}