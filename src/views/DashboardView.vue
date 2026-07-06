<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event"; 
import SidebarPlayer from "../components/SidebarPlayer.vue";
import AttributesGrid from "../components/AttributesGrid.vue";

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
  <main class="fm-dashboard">
    <div v-if="player" class="scout-container">
      <SidebarPlayer :player="player" />
      <AttributesGrid :player="player" />
    </div>
    
    <div v-else class="loading-state">
      <div class="spinner"></div>
      <p>Estabelecendo conexão com o motor Scout offline...</p>
    </div>
  </main>
</template>

<style scoped>
.fm-dashboard {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  box-sizing: border-box;
}

.scout-container {
  display: flex;
  width: 100%;
  max-width: 950px;
  height: 100%;
  max-height: 550px;
  background: rgba(30, 22, 50, 0.7);
  backdrop-filter: blur(15px);
  -webkit-backdrop-filter: blur(15px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.6), inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #9b8abf;
  height: 100%;
}

.spinner {
  width: 45px;
  height: 45px;
  border: 4px solid rgba(255, 255, 255, 0.05);
  border-top-color: #00ffcc;
  border-radius: 50%;
  animation: spin 1s cubic-bezier(0.55, 0.085, 0.68, 0.53) infinite;
  margin-bottom: 20px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
