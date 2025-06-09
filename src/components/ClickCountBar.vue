<template>
  <div class="w-60 h-screen overflow-y-auto relative overflow-x-hidden">
    <div class="flex justify-end gap-2 items-center my-4 mx-2">
      <button @click="toggleSidebar" class="text-white flex items-center justify-center p-2 rounded hover:bg-gray-700 transition-colors duration-200">
        <span class="material-symbols-outlined">
          {{ isSidebarOpen ? 'chevron_right' : 'chevron_left' }}
        </span>
      </button>
      <h2 class="text-lg font-bold text-white select-none">Click Counts</h2>
    </div>
    <transition name="sidebar">
      <div v-if="isSidebarOpen" class="sidebar-content">
        <transition-group 
          name="key-list" 
          tag="div" 
          class="w-full flex flex-col gap-2"
          @before-leave="onBeforeLeave"
          @after-leave="onAfterLeave"
        >
      <div 
        v-for="(key, _) in sorted_keys" 
        :key="key.key" 
        class="flex flex-row items-center key-list-item"
      >
        <div class="w-full h-6 bg-gray-700 rounded-l relative overflow-hidded">
          <div 
            class="h-full bg-emerald-500 rounded-l absolute right-0 top-0"
            :style="{ width: `${(key.count / maxCount) * 100}%` }"
          ></div>
          <span class="text-xs text-white absolute left-2 top-1/2 transform -translate-y-1/2 whitespace-nowrap select-none">
            {{ key.key }} ({{ key.count }})
          </span>
        </div>
      </div>
    </transition-group>
  </div>
  </transition>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';

// Define props
const props = defineProps<{
  selectedApp: string | null;
  selectedDate: string | null;
}>();

interface KeyStat {
  key: string;
  count: number;
}

const keyStats = ref<KeyStat[]>([]);

const sorted_keys = computed(() => 
  [...keyStats.value].sort((a, b) => b.count - a.count)
);

const fetchKeyStats = async () => {
  try {
    const result = await invoke<KeyStat[]>('get_key_stats', { appName: props.selectedApp,  date: props.selectedDate });
    keyStats.value = result;
  } catch (error) {
    console.error('Error fetching key stats:', error);
  }
};

const isSidebarOpen = ref(true);

const checkScreenSize = () => {
  isSidebarOpen.value = window.innerWidth >= 1240;
};

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value;
};

onMounted(() => {
  checkScreenSize();
  window.addEventListener('resize', checkScreenSize);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenSize);
});


const emit = defineEmits(['sidebarToggled']);

onMounted(fetchKeyStats);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...keyStats.value.map(key => key.count)));

// Watch for changes in selectedDate
watch(() => [props.selectedApp, props.selectedDate], fetchKeyStats);

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

};

</script>

<style scoped>
.key-list-item {
  transition: all 0.5s ease;
  position: relative;
}

.key-list-enter-from,
.key-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.key-list-move {
  transition: transform 0.5s ease;
}

.key-list-leave-active {
  position: absolute;
  width: 100%;
  max-width: 100%;
  overflow: hidden;
}

.overflow-y-auto {
  scrollbar-width: thin;
  scrollbar-color: #4a5568 #2d3748;
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
  transform: translateX(+100%);
  opacity: 0;
}

.sidebar-enter-to,
.sidebar-leave-from {
  transform: translateX(0);
  opacity: 1;
}
</style>