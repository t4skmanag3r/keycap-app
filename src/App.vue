<template>
  <div class="container mx-auto p-4 flex flex-col gap-4">
    <h1 class="text-2xl font-bold mb-4 text-white text-center">Keyboard Heatmap</h1>
    <KeyboardHeatmap ref="heatmapRef" :selectedApp="selectedApp" :selectedDate="selectedDate" :scalingMethod="scalingMethod" />
    <div class="mb-4 flex items-center justify-center space-x-4">
      <div>
        <label for="app-select" class="mr-2 text-white">Select Application:</label>
        <select id="app-select" v-model="selectedApp" @change="updateHeatmap" class="p-2 rounded bg-gray-800 text-white">
          <option :value="null">All Applications</option>
          <option v-for="app in applications" :key="app" :value="app">{{ app }}</option>
        </select>
      </div>
      <div>
        <label for="date-select" class="mr-2 text-white">Select Date:</label>
        <input type="date" id="date-select" v-model="selectedDate" @change="updateHeatmap" class="p-2 rounded bg-gray-800 text-white">
      </div>
      <div>
        <label for="scaling-select" class="mr-2 text-white">Scaling Method:</label>
        <select id="scaling-select" v-model="scalingMethod" @change="updateHeatmap" class="p-2 rounded bg-gray-800 text-white">
          <option value="linear">Linear</option>
          <option value="logarithmic">Logarithmic</option>
        </select>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
import KeyboardHeatmap from './components/KeyboardHeatmap.vue';

const heatmapRef = ref<InstanceType<typeof KeyboardHeatmap> | null>(null);

const selectedApp = ref<string | null>(null);
const selectedDate = ref<string | null>(null);
const applications = ref<string[]>([]);
const scalingMethod = ref<'linear' | 'logarithmic'>('linear');

const fetchApplications = async () => {
  try {
    applications.value = await invoke('get_applications');
  } catch (error) {
    console.error('Error fetching applications:', error);
  }
};

const updateHeatmap = () => {
  if (heatmapRef.value) {
    heatmapRef.value.fetchKeyStats();
  }
};

onMounted(() => {
  fetchApplications();
});
</script>

<style>
body {
  @apply bg-gray-900;
}

h1, h2, h3, h4, h5, h6 {
  font-family: monospace;
}

p {
  font-family: sans-serif;
}

select, input[type="date"] {
  @apply bg-gray-800 text-white border border-gray-700;
}

select option {
  @apply bg-gray-800;
}
</style>