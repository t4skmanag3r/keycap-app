<template>
  <div class="w-full h-screen overflow-y-auto py-12">
    <transition-group 
      name="app-list" 
      tag="div" 
      class="w-full flex flex-col gap-2"
      @before-leave="onBeforeLeave"
      @after-leave="onAfterLeave"
    >
      <div 
        v-for="(app, index) in sorted_apps" 
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
</style>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref, watch } from 'vue';

interface AppCount {
  app_name: string;
  click_count: number;
}

// Define props
const props = defineProps<{
  selectedDate: string | null;
}>();

const apps = ref<AppCount[]>([]);
const selectedApp = ref<string | null>(null);

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
const emit = defineEmits(['updateAppFilter']);

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

// Add these new functions for animation hooks
const onBeforeLeave = (el: Element) => {
  const { marginLeft, marginTop, width, height } = window.getComputedStyle(el);
  el.style.left = `${el.offsetLeft - parseFloat(marginLeft)}px`;
  el.style.top = `${el.offsetTop - parseFloat(marginTop)}px`;
  el.style.width = width;
  el.style.height = height;
};

const onAfterLeave = (el: Element) => {
  (el as HTMLElement).style.removeProperty('left');
  (el as HTMLElement).style.removeProperty('top');
  (el as HTMLElement).style.removeProperty('width');
  (el as HTMLElement).style.removeProperty('height');
};

</script>