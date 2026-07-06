<script setup>
import DashboardView from './views/DashboardView.vue';
</script>

<template>
  <DashboardView />
</template>

<style>
/* Estilos globais injetados para o body */
body, html {
  margin: 0;
  padding: 0;
  height: 100vh;
  background: #110c1c; /* Fundo roxo bem escuro, estilo FM */
  background-image: radial-gradient(circle at 50% 0%, #1f1435 0%, #110c1c 70%);
  font-family: 'Inter', 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  color: #e0e0e0;
  overflow: hidden;
}
#app {
  height: 100%;
}
</style>