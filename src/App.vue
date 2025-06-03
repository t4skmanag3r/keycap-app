<template>
  <div class="flex min-h-screen max-w-full relative">
    <!-- LEFT SIDEBAR -->
    <transition name="slide-left">
      <div v-if="showHeatmap && leftSidebarOpen" class="sidebar left-sidebar">
        <!-- Application counts -->
        <AppCountBar :selectedDate="selectedDate" @updateAppFilter="handleAppFilter" :resetTrigger="resetTrigger" />
      </div>
    </transition>

    <!-- MAIN CONTENT -->
    <div class="flex-grow flex flex-col transition-all duration-300" :class="contentClass">
      <!-- TOP BAR -->
      <div class="top-bar flex items-center w-full h-fit flex-row p-4" :class="{'justify-center': showSettings, 'justify-between': showHeatmap}">
        <button v-if="showHeatmap" @click="toggleLeftSidebar" class="sidebar-toggle">
          <span class="material-symbols-outlined">{{ leftSidebarOpen ? 'chevron_left' : 'chevron_right' }}</span>
        </button>
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
        <button v-if="showHeatmap" @click="toggleRightSidebar" class="sidebar-toggle">
          <span class="material-symbols-outlined">{{ rightSidebarOpen ? 'chevron_right' : 'chevron_left' }}</span>
        </button>
      </div>

      <!-- MAIN CONTENT -->
      <div v-if="showHeatmap" class="flex flex-grow flex-col items-center justify-center gap-4 p-4">
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
      <div v-if="showSettings" class="flex flex-grow flex-col items-center justify-center p-4">
        <SettingsView />
      </div>

      <!-- BOTTOM BAR -->
      <div v-if="showHeatmap" class="flex w-full h-52 px-12 flex-row">
        <!-- Date Counts -->
        <DateCountBar @updateDayFilter="handleDayFilter" :resetTrigger="resetTrigger"></DateCountBar>
      </div>
    </div>

    <!-- RIGHT SIDEBAR -->
    <transition name="slide-right">
      <div v-if="showHeatmap && rightSidebarOpen" class="sidebar right-sidebar">
        <!-- Click Counts -->
        <ClickCountBar :selectedApp="selectedApp" :selectedDate="selectedDate"></ClickCountBar>
      </div>
    </transition>
  </div>
</template>


<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
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

const leftSidebarOpen = ref(true);
const rightSidebarOpen = ref(true);

const toggleLeftSidebar = () => {
  leftSidebarOpen.value = !leftSidebarOpen.value;
};

const toggleRightSidebar = () => {
  rightSidebarOpen.value = !rightSidebarOpen.value;
};

const checkScreenSize = () => {
  if (window.innerWidth < 1024) {
    leftSidebarOpen.value = false;
    rightSidebarOpen.value = false;
  } else {
    leftSidebarOpen.value = true;
    rightSidebarOpen.value = true;
  }
};

onMounted(() => {
  checkScreenSize();
  window.addEventListener('resize', checkScreenSize);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenSize);
});

const contentClass = computed(() => {
  return {
    'ml-60': leftSidebarOpen.value,
    'mr-60': rightSidebarOpen.value,
  };
});

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
  @apply fixed top-0 bottom-0 w-60 transition-all duration-300 ease-in-out overflow-y-hidden z-10;
}

.left-sidebar {
  @apply left-0;
}

.right-sidebar {
  @apply right-0;
}

.sidebar-toggle {
  @apply bg-gray-700 text-white p-2 rounded-full;
}

.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: transform 0.3s ease;
}

.slide-left-enter-from,
.slide-left-leave-to {
  transform: translateX(-100%);
}

.slide-right-enter-from,
.slide-right-leave-to {
  transform: translateX(100%);
}
</style>