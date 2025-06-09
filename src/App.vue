<template>
  <div class="flex min-h-screen max-w-full relative">
    <!-- LEFT SIDEBAR -->
    <div v-if="showHeatmap" class="sidebar left-sidebar">
      <!-- Application counts -->
      <AppCountBar :selectedDate="selectedDate" @updateAppFilter="handleAppFilter" :resetTrigger="resetTrigger" />
    </div>

    <!-- MAIN CONTENT -->
    <div class="flex-grow flex flex-col min-h-screen w-full transition-all duration-300">
      <!-- TOP BAR -->
      <div class="top-bar flex items-center justify-center w-full h-fit flex-row p-4">
        <div class="flex gap-4">
          <button @click="toggleShowHeatmap" class="home-button p-2">
            <span class="material-symbols-outlined">home</span>
          </button>
          <button @click="refreshData" class="refresh-button p-2">
            <span class="material-symbols-outlined">sync</span>
          </button>
          <button @click="toggleShowSettings" class="settings-button p-2">
            <span class="material-symbols-outlined">settings</span>
          </button>
        </div>
      </div>

      <!-- HEATMAP AND DATE BAR -->
      <div class="flex flex-col flex-grow items-center w-full max-w-5xl mx-auto px-4">
        <!-- MAIN CONTENT -->
        <div v-if="showHeatmap" class="flex flex-col items-center justify-center gap-4 w-full flex-grow">
          <h1 class="text-2xl font-bold mb-4 text-white text-center">Keyboard Heatmap</h1>
          <div class="flex justify-center w-full">
            <KeyboardHeatmap ref="heatmapRef" :selectedApp="selectedApp" :selectedDate="selectedDate" :scalingMethod="scalingMethod" />
          </div>
          <div class="flex flex-row items-center justify-center w-full gap-8">
            <div>
              <label for="scaling-select" class="mr-2 text-white">Scaling Method:</label>
              <select id="scaling-select" v-model="scalingMethod" @change="updateHeatmap" class="p-2 rounded bg-gray-800 text-white">
                <option value="linear">Linear</option>
                <option value="logarithmic">Logarithmic</option>
              </select>
            </div>
          </div>
        </div>

        <!-- BOTTOM BAR -->
        <div v-if="showHeatmap" class="flex mt-auto bottom-bar">
          <!-- Date Counts -->
          <DateCountBar @updateDayFilter="handleDayFilter" :resetTrigger="resetTrigger" class="w-full h-52"></DateCountBar>
        </div>
      </div>

      <!-- SETTINGS VIEW -->
      <div v-if="showSettings" class="flex flex-grow flex-col items-center justify-center p-4">
        <SettingsView />
      </div>
    </div>

    <!-- RIGHT SIDEBAR -->
    <div v-if="showHeatmap" class="sidebar right-sidebar">
      <!-- Click Counts -->
      <ClickCountBar :selectedApp="selectedApp" :selectedDate="selectedDate"></ClickCountBar>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref } from 'vue';
import AppCountBar from './components/AppCountBar.vue';
import ClickCountBar from './components/ClickCountBar.vue';
import DateCountBar from './components/DateCountBar.vue';
import KeyboardHeatmap from './components/KeyboardHeatmap.vue';
import SettingsView from './views/SettingsView.vue';

const heatmapRef = ref<InstanceType<typeof KeyboardHeatmap> | null>(null);

const selectedApp = ref<string | null>(null);
const selectedDate = ref<string | null>(null);
const scalingMethod = ref<'linear' | 'logarithmic'>('logarithmic');

const resetTrigger = ref(0);

const updateHeatmap = () => {
  if (heatmapRef.value) {
    heatmapRef.value.fetchKeyStats();
  }
};

const handleAppFilter = (appName: string) => {
  selectedApp.value = appName;
};

const handleDayFilter = (day: string) => {
  selectedDate.value = day;
}

const refreshData = () => {
  selectedApp.value = null
  selectedDate.value = null
  updateHeatmap();
  resetSelection();
};

const resetSelection = () => {
  resetTrigger.value += 1;
};

const showHeatmap = ref(true)
const showSettings = ref(false);


const toggleShowHeatmap = () => {
  showSettings.value = false
  showHeatmap.value = true
}


const toggleShowSettings = () => {
  showSettings.value = true
  showHeatmap.value = false
}

</script>

<style>
#app {
  @apply min-h-screen;
}

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

<style scoped>
.top-bar button {
  @apply bg-gray-600 h-fit p-2 flex items-center justify-center text-white font-bold rounded-md;
  @apply hover:scale-110 transition;
}

.settings-button {
  @apply bg-gray-600 h-fit p-2 flex items-center justify-center text-white font-bold rounded-md;
  @apply hover:scale-110 transition;
}

.sidebar {
  @apply fixed top-0 bottom-0 w-64 transition-all duration-300 ease-in-out overflow-y-hidden z-10;
}

.left-sidebar {
  @apply left-0;
}

.right-sidebar {
  @apply right-0;
}

.bottom-bar {
  @apply w-full sm:w-11/12 md:w-10/12 lg:w-9/12 xl:w-8/12 mx-auto;
}

</style>