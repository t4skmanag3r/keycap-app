<template>
  <div class="w-full h-screen overflow-y-auto relative overflow-x-hidden">
    <div class="flex justify-start gap-2 items-center my-4 mx-2">
      <h2 class="text-lg font-bold text-white select-none">Application Counts</h2>
      <button @click="toggleSidebar" class="flex items-center justify-center text-white p-2 rounded hover:bg-gray-700 transition-colors duration-200 pointer-events-auto">
        <span class="material-symbols-outlined">
          {{ isSidebarOpen ? 'chevron_left' : 'chevron_right' }}
        </span>
      </button>
    </div>
    <transition name="sidebar">
      <div v-if="isSidebarOpen" class="sidebar-content">
        <transition-group 
          name="app-list" 
          tag="div" 
          class="w-full flex flex-col gap-2 pointer-events-auto"
          @before-leave="onBeforeLeave"
          @after-leave="onAfterLeave"
        >
        <div 
          v-for="(app, _) in sorted_apps" 
          :key="app.app_name" 
          class="flex flex-row items-center cursor-pointer app-list-item"
          @click="toggleAppSelection(app.app_name)"
        >
        <div class="w-full h-6 bg-gray-700 hover:bg-gray-600 rounded-r relative overflow-hidden"
             :class="{ 'ring-2 ring-amber-500 bg-gray-600': selectedApp === app.app_name }">
          <div 
            class="h-full bg-emerald-500 rounded-r absolute left-0 top-0"
            :style="{ width: `${(app.click_count / maxCount) * 100}%` }"
          ></div>
          <span class="text-xs text-white absolute left-2 top-1/2 transform -translate-y-1/2 whitespace-nowrap select-none">
            {{ app.app_name }} ({{ app.click_count }})
          </span>
        </div>
      </div>
      </transition-group>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.app-list-item {
  transition: all 0.5s ease;
  position: relative;
}

.app-list-enter-active,
.app-list-leave-active {
  transition: all 0.5s ease;
}

.app-list-enter-from,
.app-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.app-list-move {
  transition: transform 0.5s ease;
}

.app-list-leave-active {
  position: absolute;
  width: 100%;
}

.overflow-y-auto {
  scrollbar-width: thin;
  scrollbar-color: #4a5568 #2d3748;
}

.overflow-y-auto {
  scrollbar-width: thin;
  scrollbar-color: #4a5568 #2d3748;
  direction: rtl;
}

.overflow-y-auto > * {
  direction: ltr;
}

.overflow-y-auto::-webkit-scrollbar {
  width: 8px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #2d3748;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: #4a5568;
  border-radius: 4px;
}

.sidebar-content {
  transition: all 0.3s ease-in-out;
  width: 100%;
  overflow: hidden;
}

.sidebar-enter-from,
.sidebar-leave-to {
  transform: translateX(-100%);
  opacity: 0;
}

.sidebar-enter-to,
.sidebar-leave-from {
  transform: translateX(0);
  opacity: 1;
}

</style>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';

interface AppCount {
  app_name: string;
  click_count: number;
}

// Define props
const props = defineProps<{
  selectedDate: string | null;
  resetTrigger: number;
}>();

const apps = ref<AppCount[]>([]);
const selectedApp = ref<string | null>(null);

const isSidebarOpen = ref(true);

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value;
};

const checkScreenSize = () => {
  isSidebarOpen.value = window.innerWidth >= 1240;
};

onMounted(() => {
  checkScreenSize();
  window.addEventListener('resize', checkScreenSize);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenSize);
});


const sorted_apps = computed(() => 
  [...apps.value].sort((a, b) => b.click_count - a.click_count)
);

const fetchAppCounts = async () => {
  try {
    const result = await invoke<AppCount[]>('get_app_counts', { date: props.selectedDate });
    apps.value = result;
  } catch (error) {
    console.error('Error fetching app counts:', error);
  }
};

onMounted(fetchAppCounts);

// Watch for changes in selectedDate
watch(() => props.selectedDate, fetchAppCounts);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...apps.value.map(app => app.click_count)));

// Define emits
const emit = defineEmits(['updateAppFilter', 'sidebarToggled']);

// Function to handle app selection and deselection
const toggleAppSelection = (appName: string) => {
  if (selectedApp.value === appName) {
    selectedApp.value = null;
    emit('updateAppFilter', null);
  } else {
    selectedApp.value = appName;
    emit('updateAppFilter', appName);
  }
};

watch(() => props.resetTrigger, () => {
  selectedApp.value = null;
});

const onBeforeLeave = (el: Element) => {
  const htmlEl = el as HTMLElement;
  const { marginLeft, marginTop, width, height } = window.getComputedStyle(htmlEl);
  htmlEl.style.left = `${htmlEl.offsetLeft - parseFloat(marginLeft)}px`;
  htmlEl.style.top = `${htmlEl.offsetTop - parseFloat(marginTop)}px`;
  htmlEl.style.width = width;
  htmlEl.style.height = height;
};

const onAfterLeave = (el: Element) => {
  const htmlEl = el as HTMLElement;
  htmlEl.style.removeProperty('left');
  htmlEl.style.removeProperty('top');
  htmlEl.style.removeProperty('width');
  htmlEl.style.removeProperty('height');
};

</script>