<template>
  <div class="w-full flex flex-row gap-0 overflow-x-auto justify-end pt-4">
    <div v-for="(day, index) in sorted_days" :key="index" class="flex flex-col justify-end items-center">
        <span class="text-xs text-white transform -rotate-30 origin-top-left whitespace-nowrap">
          {{ formatDate(day.date) }}
        </span>
      <div class="h-full w-6 bg-emerald-500 rounded-t"
      :style="{ height: `${(day.click_count / maxCount) * 100}%` }">
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref } from 'vue';

interface DailyClickCount {
  date: string;
  click_count: number;
}

const days = ref<DailyClickCount[]>([]);

const sorted_days = computed(() => 
  [...days.value].sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
);

const fetchDailyClickCounts = async () => {
  try {
    const result = await invoke<DailyClickCount[]>('get_daily_click_counts');
    days.value = result;
  } catch (error) {
    console.error('Error fetching daily click counts:', error);
  }
};

onMounted(fetchDailyClickCounts);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...days.value.map(day => day.click_count)));

// Function to format the date (you can adjust this as needed)
const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
};
</script>

<style scoped>
.transform {
  transform: rotate(-30deg);
}
.origin-top-left {
  transform-origin: top left;
}

.overflow-x-auto {
  scrollbar-width: thin;
  scrollbar-color: #4a5568 #2d3748;
}

.overflow-x-auto::-webkit-scrollbar {
  height: 8px;
}

.overflow-x-auto::-webkit-scrollbar-track {
  background: #2d3748;
}

.overflow-x-auto::-webkit-scrollbar-thumb {
  background-color: #4a5568;
  border-radius: 4px;
}
</style>