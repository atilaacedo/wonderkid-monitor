<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
// Importamos o listen para ouvir eventos vindos do Rust
import { listen } from "@tauri-apps/api/event"; 

const player = ref(null);

async function setupWatchdog() {
  try {
    // 1. Pega o estado inicial assim que a tela abre
    player.value = await invoke("get_player_status");

    // 2. Fica escutando o evento emitido pela thread do Rust
    await listen("player-updated", (event) => {
      // O 'event.payload' contém o JSON atualizado que o Rust enviou
      player.value = event.payload; 
    });

  } catch (error) {
    console.error("Erro no watchdog:", error);
  }
}

onMounted(() => {
  setupWatchdog();
});
</script>

<template>
  <main class="container">
    <h1>Dashboard do Olheiro</h1>
    
    <div v-if="player" class="player-card">
      <h2>Jogador: {{ player.name }}</h2>
      <p><strong>CA (Current Ability):</strong> {{ player.current_ability.toFixed(2) }}</p>
      <p><strong>Pontos de Foco:</strong> {{ player.focus_points }}</p>
    </div>
    <div v-else>
      <p>Carregando dados do motor offline...</p>
    </div>
  </main>
</template>

<style scoped>
.container {
  padding: 20px;
  font-family: sans-serif;
}
.player-card {
  background: #f4f4f4;
  padding: 15px;
  border-radius: 8px;
  max-width: 300px;
  color: #333;
}
</style>