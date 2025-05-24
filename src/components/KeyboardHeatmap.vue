<template>
  <div class="keyboard-heatmap">
    <div v-for="(row, rowIndex) in keyboardLayout" :key="rowIndex" class="flex justify-center">
      <div
        v-for="(key, keyIndex) in row"
        :key="keyIndex"
        class="key m-0.5 rounded text-center flex items-center justify-center"
        :class="[getKeySize(key), { 'text-xs': key.length > 1 }]"
        :style="{ backgroundColor: getHeatColor(key) }"
      >
        {{ key }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

const keyboardLayout = [
  ['Esc', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12', 'Del', 'Ins'],
  ['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Bck', 'PgUp'],
  ['Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\', 'PgDn'],
  ['Caps', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', "'", 'Return', 'Home'],
  ['L-Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'R-Shift', '↑', 'End'],
  ['L-Ctrl', 'Win', 'L-Alt', 'Space', 'R-Alt', 'Fn', 'R-Ctrl', '←', '↓', '→'],
];

const keyHeatData = ref<Record<string, number>>({});

const maxHeat = computed(() => Math.max(...Object.values(keyHeatData.value), 1));

const getHeatColor = (key: string) => {
  const heat = keyHeatData.value[key] || 0;
  const intensity = heat / maxHeat.value;
  return `rgba(255, 0, 0, ${intensity})`;
};

const getKeySize = (key: string) => {
  switch (key) {
    case 'Bck':
      return 'w-14 h-10'
    case 'Return':
      return 'w-20 h-10'
    case 'L-Shift':
      return 'w-20 h-10';
    case 'R-Shift':
      return 'w-14 h-10';
    case 'Caps':
      return 'w-14 h-10';
    case 'Tab':
      return 'w-14 h-10';
    case 'Space':
      return 'w-56 h-10';
    case 'L-Ctrl':
      return 'w-14 h-10';
    case 'Win':
      return 'w-14 h-10';
    case 'L-Alt':
      return 'w-14 h-10';
    case 'Fn':
      return 'w-10 h-10';
    default:
      return 'w-10 h-10';
  }
};

const updateHeatData = (newData: Record<string, number>) => {
  keyHeatData.value = newData;
};

defineExpose({ updateHeatData });
</script>

<style scoped>
.keyboard-heatmap {
  @apply bg-gray-800 p-4 rounded-lg inline-block;
}

.key {
  @apply bg-gray-700 border border-gray-600 font-bold text-sm text-white;
}
</style>