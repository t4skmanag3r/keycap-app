<template>
  <div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4 text-white">Keyboard Heatmap</h1>
    <KeyboardHeatmap ref="heatmapRef" />
    <button @click="updateRandomHeatData" class="mt-4 bg-blue-500 text-white px-4 py-2 rounded">
      Generate Random Heat Data
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import KeyboardHeatmap from './components/KeyboardHeatmap.vue';

const heatmapRef = ref<InstanceType<typeof KeyboardHeatmap> | null>(null);

const updateRandomHeatData = () => {
  const newData: Record<string, number> = {};
  const keys = [
    'Esc', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12', 'Del', 'Ins',
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Bck', 'PgUp',
    'Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\', 'PgDn',
    'Caps', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', "'", 'Return', 'Home',
    'L-Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'R-Shift', '↑', 'End',
    'L-Ctrl', 'Win', 'L-Alt', 'Space', 'R-Alt', 'Fn', 'R-Ctrl', '←', '↓', '→'
  ];
  keys.forEach(key => {
    newData[key] = Math.floor(Math.random() * 100);
  });
  heatmapRef.value?.updateHeatData(newData);
};
</script>

<style scoped>
</style>
<style>
body {
  background-color: #252525;
}

h1, h2, h3, h4, h5, h6 {
  font-family: monospace
}
p {
  font-family: sans-serif
}
</style>