<template>
  <div class="settings-view">
    <div class="flex flex-col h-fit w-fit px-8 py-10 bg-gray-800 rounded-md gap-2">
      <h2 class="text-xl font-bold mb-4 text-white text-center">Settings</h2>
      <div class="setting-item flex items-center">
        <label for="autostart" class="text-white mr-4 flex-grow">Enable Autostart</label>
        <div class="custom-checkbox">
          <input type="checkbox" id="autostart" v-model="autostart" @change="toggleAutostart" class="hidden">
          <label for="autostart" class="checkbox-label"></label>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue';

const autostart = ref(false);

onMounted(async () => {
  autostart.value = await invoke('is_autostart_enabled');
});

const toggleAutostart = async () => {
  try {
    await invoke('toggle_autostart', { enable: autostart.value });
    console.log(`Autostart ${autostart.value ? 'enabled' : 'disabled'}`);
  } catch (error) {
    console.error('Error toggling autostart:', error);
    autostart.value = !autostart.value; // Revert the change if there's an error
  }
};
</script>

<style scoped>
.settings-view {
  @apply w-full h-full flex flex-col items-center justify-center;
}

.setting-item {
  @apply mb-4 w-full;
}

.custom-checkbox {
  @apply relative inline-block w-12 h-6 select-none;
}

.checkbox-label {
  @apply block overflow-hidden h-6 rounded-full bg-gray-600 cursor-pointer;
}

.checkbox-label::after {
  content: "";
  @apply absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-300 ease-in-out;
}

input:checked + .checkbox-label {
  @apply bg-emerald-500;
}

input:checked + .checkbox-label::after {
  @apply transform translate-x-6;
}

input:focus + .checkbox-label {
  @apply ring-2 ring-emerald-500;
}
</style>