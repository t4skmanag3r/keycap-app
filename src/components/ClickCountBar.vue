<template>
  <div class="w-full h-screen overflow-y-auto py-12 relative">
    <transition-group 
      name="key-list" 
      tag="div" 
      class="w-full flex flex-col gap-2"
      @before-leave="onBeforeLeave"
      @after-leave="onAfterLeave"
    >
      <div 
        v-for="(key, index) in sorted_keys" 
        :key="key.key" 
        class="flex flex-row items-center key-list-item"
      >
        <div class="w-full h-6 bg-gray-700 rounded-l relative overflow-hidden">
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
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref, watch } from 'vue';

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

onMounted(fetchKeyStats);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...keyStats.value.map(key => key.count)));

// Watch for changes in selectedDate
watch(() => [props.selectedApp, props.selectedDate], fetchKeyStats);

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

<style scoped>
.key-list-item {
  transition: all 0.5s ease;
  position: relative;
}

.key-list-enter-active,
.key-list-leave-active {
  transition: all 0.5s ease;
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
</style>