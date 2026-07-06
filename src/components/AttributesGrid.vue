<script setup>
import { computed, defineProps } from 'vue';

const props = defineProps({
  player: {
    type: Object,
    required: true
  }
});

// Propriedades Computadas (computed)
// Como o focus_points vai subindo, calculamos atributos na escala 1 a 20.
const focusPoints = computed(() => props.player.focus_points || 0);

// Fórmulas fictícias para simular o crescimento dos atributos com base nos focus points
const tecnica = computed(() => Math.min(20, Math.floor(10 + (focusPoints.value * 0.1))));
const passe = computed(() => Math.min(20, Math.floor(12 + (focusPoints.value * 0.08))));
const finalizacao = computed(() => Math.min(20, Math.floor(9 + (focusPoints.value * 0.12))));

const concentracao = computed(() => Math.min(20, Math.floor(8 + (focusPoints.value * 0.15))));
const indiceTrabalho = computed(() => Math.min(20, Math.floor(11 + (focusPoints.value * 0.2))));
const visao = computed(() => Math.min(20, Math.floor(13 + (focusPoints.value * 0.05))));

const aceleracao = computed(() => Math.min(20, Math.floor(14 + (focusPoints.value * 0.07))));
const velocidade = computed(() => Math.min(20, Math.floor(13 + (focusPoints.value * 0.09))));
const folego = computed(() => Math.min(20, Math.floor(12 + (focusPoints.value * 0.1))));
const forca = computed(() => Math.min(20, Math.floor(10 + (focusPoints.value * 0.06))));

// Função para definir a cor do atributo com base no valor (Estilo FM)
function getAttributeClass(value) {
  if (value >= 16) return 'attr-excellent'; // verde neon
  if (value >= 11) return 'attr-good';      // verde claro
  if (value >= 6) return 'attr-average';    // amarelo
  return 'attr-poor';                       // vermelho
}
</script>

<template>
  <!-- Painel Principal: Grade de Atributos -->
  <section class="attributes-panel">
    <div class="attributes-header">
      <h2>Relatório de Olheiro</h2>
      <div class="badge">Nível de Conhecimento: 100%</div>
    </div>
    
    <div class="attributes-grid">
      <!-- Categoria: Técnicos -->
      <div class="attr-category">
        <h3>Técnicos</h3>
        <ul>
          <li>
            <span class="attr-name">Finalização</span>
            <span :class="['attr-value', getAttributeClass(finalizacao)]">{{ finalizacao }}</span>
          </li>
          <li>
            <span class="attr-name">Passe</span>
            <span :class="['attr-value', getAttributeClass(passe)]">{{ passe }}</span>
          </li>
          <li>
            <span class="attr-name">Técnica</span>
            <span :class="['attr-value', getAttributeClass(tecnica)]">{{ tecnica }}</span>
          </li>
        </ul>
      </div>
      
      <!-- Categoria: Mentais -->
      <div class="attr-category">
        <h3>Mentais</h3>
        <ul>
          <li>
            <span class="attr-name">Concentração</span>
            <span :class="['attr-value', getAttributeClass(concentracao)]">{{ concentracao }}</span>
          </li>
          <li>
            <span class="attr-name">Índice de Trabalho</span>
            <span :class="['attr-value', getAttributeClass(indiceTrabalho)]">{{ indiceTrabalho }}</span>
          </li>
          <li>
            <span class="attr-name">Visão de Jogo</span>
            <span :class="['attr-value', getAttributeClass(visao)]">{{ visao }}</span>
          </li>
        </ul>
      </div>

      <!-- Categoria: Físicos -->
      <div class="attr-category">
        <h3>Físicos</h3>
        <ul>
          <li>
            <span class="attr-name">Aceleração</span>
            <span :class="['attr-value', getAttributeClass(aceleracao)]">{{ aceleracao }}</span>
          </li>
          <li>
            <span class="attr-name">Fôlego</span>
            <span :class="['attr-value', getAttributeClass(folego)]">{{ folego }}</span>
          </li>
          <li>
            <span class="attr-name">Força</span>
            <span :class="['attr-value', getAttributeClass(forca)]">{{ forca }}</span>
          </li>
          <li>
            <span class="attr-name">Velocidade</span>
            <span :class="['attr-value', getAttributeClass(velocidade)]">{{ velocidade }}</span>
          </li>
        </ul>
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Painel Principal */
.attributes-panel {
  flex: 1;
  padding: 35px;
  display: flex;
  flex-direction: column;
}

.attributes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 25px;
  padding-bottom: 15px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.attributes-header h2 {
  margin: 0;
  font-size: 1.4rem;
  color: #fff;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.badge {
  background: rgba(94, 66, 166, 0.3);
  color: #d1baff;
  padding: 6px 14px;
  border-radius: 20px;
  font-size: 0.8rem;
  font-weight: 600;
  border: 1px solid rgba(209, 186, 255, 0.2);
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
}

.attributes-grid {
  display: flex;
  gap: 25px;
  flex: 1;
}

.attr-category {
  flex: 1;
  background: rgba(0, 0, 0, 0.15);
  border-radius: 10px;
  padding: 18px;
  border: 1px solid rgba(255,255,255,0.03);
}

.attr-category h3 {
  margin: 0 0 16px 0;
  font-size: 0.85rem;
  color: #8c7ba8;
  text-transform: uppercase;
  letter-spacing: 1.2px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  padding-bottom: 10px;
}

.attr-category ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.attr-category li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 6px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.attr-category li:hover {
  background-color: rgba(255, 255, 255, 0.04);
}

.attr-name {
  font-size: 0.95rem;
  color: #e0e0e0;
}

.attr-value {
  font-weight: 700;
  font-size: 1.1rem;
  width: 30px;
  text-align: center;
  background: rgba(0,0,0,0.2);
  padding: 2px 0;
  border-radius: 4px;
}

/* Cores dos Atributos Estilo FM */
.attr-poor {
  color: #ff5252; /* Vermelho */
}
.attr-average {
  color: #ffd740; /* Amarelo */
}
.attr-good {
  color: #69f0ae; /* Verde Claro */
}
.attr-excellent {
  color: #18ffff; /* Verde Néon / Ciano */
  text-shadow: 0 0 8px rgba(24, 255, 255, 0.5);
  box-shadow: inset 0 0 10px rgba(24, 255, 255, 0.1);
}
</style>
