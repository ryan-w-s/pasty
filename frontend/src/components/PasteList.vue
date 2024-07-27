<template>
  <div>
    <ul>
      <li v-for="paste in pastes" :key="paste.id">
        {{ paste.content }}
      </li>
    </ul>
    <div class="pagination">
      <button @click="prevPage" :disabled="currentPage === 1">Previous</button>
      <span>Page {{ currentPage }} of {{ totalPages }}</span>
      <button @click="nextPage" :disabled="currentPage === totalPages">Next</button>
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
