<template>
  <div class="w-full flex flex-row gap-2 overflow-x-auto overflow-y-hidden justify-end pt-4">
    <div v-for="(day, index) in sorted_days" :key="index" class="flex flex-col justify-end items-center cursor-pointer relative" @click="toggleDaySelection(day.date)">
      <span class="text-xs z-10 text-white transform -rotate-30 origin-top-left whitespace-nowrap select-none absolute top-2 -left-1">
        {{ formatDate(day.date) }}
      </span>
      <div class="h-full w-6 bg-emerald-500 rounded-t hover:bg-emerald-400 relative"
           :style="{ height: `${(day.click_count / maxCount) * 100}%` }"
           :class="{ 'ring-2 ring-amber-500': selectedDay === day.date }">
        <span class="absolute bottom-0 left-0 right-0 text-xs text-white vertical-text select-none">
          {{ formatClickCount(day.click_count) }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref, watch } from 'vue';

const props = defineProps<{
  resetTrigger: number;
  selectedApp: string | null;
}>();

interface DailyClickCount {
  date: string;
  click_count: number;
}

const days = ref<DailyClickCount[]>([]);

const selectedDay = ref<string | null>(null);

const sorted_days = computed(() => 
  [...days.value].sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
);

const fetchDailyClickCounts = async () => {
  try {
    const result = await invoke<DailyClickCount[]>('get_daily_click_counts', { appName: props.selectedApp });
    days.value = result;
  } catch (error) {
    console.error('Error fetching daily click counts:', error);
  }
};

onMounted(fetchDailyClickCounts);

watch(() => props.selectedApp, fetchDailyClickCounts);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...days.value.map(day => day.click_count)));

// Function to format the date (you can adjust this as needed)
const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
};

// Function to handle day selection and deselection
const toggleDaySelection = (day: string) => {
  if (selectedDay.value === day) {
    selectedDay.value = null;
    emit('updateDayFilter', null);
  } else {
    selectedDay.value = day;
    emit('updateDayFilter', day);
  }
};

// Define emits
const emit = defineEmits(['updateDayFilter']);

// Function to format the click count
const formatClickCount = (count: number): string => {
  if (count >= 1000000) {
    return (count / 1000000).toFixed(1) + 'M';
  } else if (count >= 1000) {
    return (count / 1000).toFixed(1) + 'K';
  } else {
    return count.toString();
  }
};

watch(() => props.resetTrigger, () => {
  selectedDay.value = null;
});

</script>

<style scoped>
.transform {
  transform: rotate(-35deg);
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

.vertical-text {
  writing-mode: vertical-rl;
  text-orientation: mixed;
  transform: rotate(180deg);
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>