<template>
  <div class="w-full flex flex-col gap-2 overflow-y-auto py-12">
    <div v-for="(app, index) in sorted_apps" :key="index" class="flex flex-row items-center">
      <div class="w-full h-6 bg-gray-700 rounded-r relative overflow-hidden">
        <div 
          class="h-full bg-emerald-500 rounded-r absolute left-0 top-0"
          :style="{ width: `${(app.click_count / maxCount) * 100}%` }"
        ></div>
        <span class="text-xs text-white absolute left-2 top-1/2 transform -translate-y-1/2 whitespace-nowrap">
          {{ app.app_name }} ({{ app.click_count }})
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { computed, onMounted, ref } from 'vue';

interface AppCount {
  app_name: string;
  click_count: number;
}

const apps = ref<AppCount[]>([]);

const sorted_apps = computed(() => 
  [...apps.value].sort((a, b) => b.click_count - a.click_count)
);

const fetchAppCounts = async () => {
  try {
    const result = await invoke<AppCount[]>('get_app_counts');
    apps.value = result;
  } catch (error) {
    console.error('Error fetching app counts:', error);
  }
};

onMounted(fetchAppCounts);

// Calculate the maximum count for scaling
const maxCount = computed(() => Math.max(...apps.value.map(app => app.click_count)));
</script>