<template>
  <nav class="bg-gray-800 p-4">
    <div class="w-full flex justify-between items-center">
      <a @click="$emit('navigate', 'home')" class="text-white text-2xl font-bold cursor-pointer"
        >Pasty</a
      >
      <button
        @click="$emit('navigate', 'new-paste')"
        class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
      >
        New Paste
      </button>
    </div>
  </nav>
</template>

<script setup lang="ts">
defineEmits(['navigate'])
</script>
