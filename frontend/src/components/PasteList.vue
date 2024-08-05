<template>
  <div class="container mx-auto px-4">
    <div class="grid grid-cols-1 gap-6 mt-4">
      <div
        v-for="paste in pastes"
        :key="paste.id"
        class="bg-white shadow-md rounded-lg p-4 border-2 border-gray-200"
      >
        <div class="text-sm font-semibold text-gray-600 mb-2">ID: {{ paste.id }}</div>
        <div class="text-gray-800 whitespace-pre-wrap">{{ paste.content }}</div>
      </div>
    </div>
    <div class="mt-6 flex justify-center">
      <button
        @click="prevPage"
        :disabled="currentPage === 1"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mr-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Previous
      </button>
      <span class="py-2 px-4">Page {{ currentPage }} of {{ totalPages }}</span>
      <button
        @click="nextPage"
        :disabled="currentPage === totalPages"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ml-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Next
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'

interface Paste {
  id: number
  content: string
}

const pastes = ref<Paste[]>([])
const currentPage = ref(1)
const perPage = ref(20)
const totalPages = ref(1)
const total = ref(0)

async function fetchPastes() {
  try {
    const response = await axios.get<{ pastes: Paste[]; total: number; total_pages: number }>(
      `/pastes?page=${currentPage.value}&per_page=${perPage.value}`
    )
    pastes.value = response.data.pastes
    total.value = response.data.total
    totalPages.value = response.data.total_pages
  } catch (error) {
    console.error('Error fetching pastes:', error)
  }
}

function prevPage() {
  if (currentPage.value > 1) {
    currentPage.value--
    fetchPastes()
  }
}

function nextPage() {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
    fetchPastes()
  }
}

onMounted(fetchPastes)
</script>
