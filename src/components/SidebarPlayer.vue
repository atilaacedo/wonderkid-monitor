<script setup>
import { defineProps } from 'vue';

const props = defineProps({
  player: {
    type: Object,
    required: true
  }
});
</script>

<template>
  <aside class="sidebar">
    <div class="profile-header">
      <div class="portrait-placeholder">
        <svg viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.4)" stroke-width="1.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" />
        </svg>
      </div>
      <h1 class="player-name">{{ player.name }}</h1>
      <p class="player-role">Meio-Campista (Centro)</p>
    </div>
    
    <div class="stats-card">
      <div class="stat-row">
        <span>Current Ability (CA)</span>
        <span class="stat-value ca-value">{{ player.current_ability.toFixed(2) }}</span>
      </div>
      <div class="stat-row highlight">
        <span>Pontos de Foco</span>
        <span class="stat-value focus-value">{{ player.focus_points }}</span>
      </div>
    </div>
  </aside>
</template>

<style scoped>
/* Painel Lateral */
.sidebar {
  width: 280px;
  background: rgba(15, 10, 25, 0.65);
  padding: 35px 25px;
  display: flex;
  flex-direction: column;
  border-right: 1px solid rgba(255, 255, 255, 0.05);
  box-shadow: 10px 0 20px rgba(0,0,0,0.15);
}

.profile-header {
  text-align: center;
  margin-bottom: 35px;
}

.portrait-placeholder {
  width: 110px;
  height: 110px;
  margin: 0 auto 15px;
  border-radius: 50%;
  background: linear-gradient(145deg, #2a1f42, #181126);
  border: 2px solid rgba(255, 255, 255, 0.15);
  box-shadow: 0 8px 15px rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
}

.portrait-placeholder svg {
  width: 60px;
  height: 60px;
}

.player-name {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: #fff;
  letter-spacing: 0.5px;
}

.player-role {
  margin: 4px 0 0;
  font-size: 0.85rem;
  color: #9b8abf;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.stats-card {
  background: rgba(0, 0, 0, 0.25);
  border-radius: 10px;
  padding: 16px;
  border: 1px solid rgba(255, 255, 255, 0.03);
}

.stat-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  font-size: 0.9rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  color: #c4b5e3;
}
.stat-row:last-child {
  border-bottom: none;
  padding-bottom: 4px;
}
.stat-row:first-child {
  padding-top: 4px;
}

.stat-value {
  font-weight: 700;
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 1.05rem;
}

.ca-value {
  color: #fff;
}

.highlight {
  color: #d1baff;
}

.focus-value {
  color: #00ffcc;
  text-shadow: 0 0 8px rgba(0, 255, 204, 0.4);
  font-size: 1.2rem;
}
</style>
