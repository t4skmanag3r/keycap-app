<template>
  <div class="container flex flex-row gap-24 justify-center min-h-screen max-w-full">
    <!-- LEFT SIDEBAR -->
     <div v-if="showHeatmap" class="min-h-full w-60">
      <!-- Application counts -->
       <AppCountBar :selectedDate="selectedDate" @updateAppFilter="handleAppFilter" :resetTrigger="resetTrigger" />
      <div>
      </div>
     </div>
    <!-- MIDDLE DIV -->
    <div class="min-h-full flex flex-col gap-4 flex-grow">
      <!-- TOP BAR -->
      <div class="top-bar flex justify-center gap-4 w-full h-fit flex-row mt-8">
        <button @click="toggleShowHeatmap" class="home-button p-0">
          <span class="material-symbols-outlined">
            home
          </span>
        </button>
        <button @click="refreshData" class="refresh-button p-0">
          <span class="material-symbols-outlined">
            sync
          </span>
        </button>
        <button @click="toggleShowSettings" class="settings-button p-0">
          <span class="material-symbols-outlined">
            settings
          </span>
        </button>
      </div>
       <!-- MAIN CONTENT -->
      <div v-if="showHeatmap" class="flex flex-grow flex-col items-center justify-center gap-4">
        <h1 class="text-2xl font-bold mb-4 text-white text-center">Keyboard Heatmap</h1>
        <KeyboardHeatmap ref="heatmapRef" :selectedApp="selectedApp" :selectedDate="selectedDate" :scalingMethod="scalingMethod" />
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
      <!-- SETTINGS VIEW -->
      <div v-if="showSettings" class="flex flex-grow flex-col items-center justify-center">
        <SettingsView />
      </div>
      <!-- BOTTOM BAR -->
       <div v-if="showHeatmap" class="flex w-full h-52 flex-row ">
        <!-- Date Counts -->
        <DateCountBar @updateDayFilter="handleDayFilter" :resetTrigger="resetTrigger"></DateCountBar>
      </div>
    </div>
    <!-- RIGHT SIDEBAR -->
     <div v-if="showHeatmap" class="min-h-full w-60">
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
</style>