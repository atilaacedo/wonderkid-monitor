use crate::domain::activity::Activity;
use serde::Serialize; 

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    pub name: String,
    pub current_ability: f32,
    pub focus_points: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            current_ability: 45.0,
            focus_points: 0,
        }
    }

    pub fn process_activity(&mut self, activity: Activity) {
        match activity {
            Activity::Programming(app) => {
                println!("💪 Treino focado em {}! CA subindo.", app);
                self.focus_points += 15;
                self.current_ability += 0.05;
            }
            Activity::Studying(app) => {
                println!("📚 Estudando com {}. Evolução tática.", app);
                self.focus_points += 10;
                self.current_ability += 0.02;
            }
            Activity::Gaming(game) => {
                println!("🎮 Jogando {}. Descanso é importante.", game);
            }
            Activity::Distraction => {
                println!("📉 Faltou ao treino! Caiu na distração.");
                self.current_ability -= 0.1;
                if self.focus_points > 5 {
                    self.focus_points -= 5;
                }
            }
            Activity::Unknown => {
                println!("🕵️ Olheiro confuso.");
            }
        }
    }
}
