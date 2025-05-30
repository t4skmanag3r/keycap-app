<template>
  <div class="w-full max-h-screen overflow-y-auto py-12">
    <div class="w-full flex flex-col gap-2">
      <div v-for="(key, index) in sorted_keys" :key="index" class="flex flex-row items-center">
        <div class="w-full h-6 bg-gray-700 rounded-l relative overflow-hidden">
          <div 
            class="h-full bg-emerald-500 rounded-l absolute right-0 top-0"
            :style="{ width: `${(key.count / maxCount) * 100}%` }"
          ></div>
          <span class="text-xs text-white absolute left-2 top-1/2 transform -translate-y-1/2 whitespace-nowrap">
            {{ key.key }} ({{ key.count }})
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref } from 'vue';

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
    const result = await invoke<KeyStat[]>('get_key_stats');
    keyStats.value = result;
  } catch (error) {
    console.error('Error fetching key stats:', error);
  }
};

onMounted(fetchKeyStats);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...keyStats.value.map(key => key.count)));
</script>

<style scoped>
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