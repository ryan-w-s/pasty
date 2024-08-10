<template>
  <div>
    <h2 class="text-2xl font-bold mb-4">Create New Paste</h2>
    <form @submit.prevent="createPaste" class="space-y-4">
      <div>
        <textarea
          v-model="content"
          placeholder="Enter your paste content here"
          class="w-full h-32 p-2 border rounded"
          required
        ></textarea>
      </div>
      <button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600">
        Create Paste
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'

const content = ref('')
const BASE_URL = 'http://localhost:8080'

const emit = defineEmits(['navigate'])

const createPaste = async () => {
  try {
    const response = await axios.post(`${BASE_URL}/pastes`, { content: content.value })
    console.log('Paste created:', response.data)
    content.value = '' // Clear the textarea after successful creation
    emit('navigate', 'home') // Navigate back home
  } catch (error) {
    console.error('Error creating paste:', error)
    // Display an error message to the user
    alert('An error occurred while creating the paste. Please try again.')
  }
}
</script>
