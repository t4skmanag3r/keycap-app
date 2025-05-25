<template>
  <main class="p-4 flex min-h-screen flex h-full items-center justify-center">
    <KeyboardHeatmap ref="heatmapRef" />
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import KeyboardHeatmap from './components/KeyboardHeatmap.vue';

const heatmapRef = ref<InstanceType<typeof KeyboardHeatmap> | null>(null);

</script>

<style scoped>
</style>
<style>
body {
  @apply bg-gray-900
}

h1, h2, h3, h4, h5, h6 {
  font-family: monospace
}
p {
  font-family: sans-serif
}
</style>