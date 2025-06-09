<template>
  <div class="keyboard-heatmap inline-flex flex-col bg-gray-800 p-4 rounded-lg">
    <div v-for="(row, rowIndex) in keyboardLayout" :key="rowIndex" class="flex justify-start">
      <div
        v-for="(key, keyIndex) in row"
        :key="keyIndex"
        :title="getKeyClickCount(key)"
        class="key m-0.5 rounded text-center flex items-center justify-center select-none cursor-pointer"
        :class="[getKeySize(key), { 'text-xs': key.length > 1 }]"
        :style="{ backgroundColor: getHeatColor(key) }"
      >
        {{ key }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref, watch } from 'vue';

const props = defineProps<{
  selectedApp: string | null;
  selectedDate: string | null;
  scalingMethod: 'linear' | 'logarithmic';
}>();


const keyboardLayout = [
  ['Esc', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12', 'Del', 'Ins'],
  ['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Bck', 'PgUp'],
  ['Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\', 'PgDn'],
  ['Caps', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', "'", 'Return', 'Home'],
  ['LShift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'RShift', '↑', 'End'],
  ['LCtrl', 'Win', 'LAlt', 'Space', 'RAlt', 'Fn', 'RCtrl', '←', '↓', '→'],
];

const getKeyClickCount = (key: string) => {
  const count = keyHeatData.value[key] || 0;
  return `Clicks: ${count}`;
};


const keyHeatData = ref<Record<string, number>>({});

const maxHeat = computed(() => Math.max(...Object.values(keyHeatData.value), 1));

const getHeatColor = (key: string) => {
  const heat = keyHeatData.value[key] || 0;
  let intensity;
  
  if (props.scalingMethod === 'logarithmic') {
    // Logarithmic scaling
    intensity = Math.log(heat + 1) / Math.log(maxHeat.value + 1);
  } else {
    // Linear scaling
    intensity = heat / maxHeat.value;
  }
  
  return `rgba(255, 0, 0, ${intensity})`;
};


const getKeySize = (key: string) => {
  switch (key) {
    case 'Bck':
      return 'w-14 h-10'
    case 'Return':
      return 'w-20 h-10'
    case 'LShift':
      return 'w-20 h-10';
    case 'RShift':
      return 'w-14 h-10';
    case 'Caps':
      return 'w-14 h-10';
    case 'Tab':
      return 'w-14 h-10';
    case 'Space':
      return 'w-56 h-10';
    case 'LCtrl':
      return 'w-14 h-10';
    case 'Win':
      return 'w-14 h-10';
    case 'LAlt':
      return 'w-14 h-10';
    case 'Fn':
      return 'w-10 h-10';
    default:
      return 'w-10 h-10';
  }
};

const fetchKeyStats = async () => {
  try {
    const keyStats: { key: string; count: number }[] = await invoke('get_key_stats', { 
      appName: props.selectedApp,
      date: props.selectedDate ? props.selectedDate : null
    });
    const newHeatData: Record<string, number> = {};
    keyStats.forEach(({ key, count }) => {
      newHeatData[key] = count;
    });
    keyHeatData.value = newHeatData;
  } catch (error) {
    console.error('Error fetching key stats:', error);
  }
};

onMounted(() => {
  fetchKeyStats();
});

watch(() => [props.selectedApp, props.selectedDate], () => {
  fetchKeyStats();
});

defineExpose({ fetchKeyStats });
</script>

<style scoped>
.keyboard-heatmap {
  @apply bg-gray-800 p-4 rounded-lg inline-block;
}

.key {
  @apply bg-gray-700 border border-gray-600 font-bold text-sm text-white;
}
</style>