<template>
  <div>
    <Navbar @navigate="handleNavigation" />
    <div class="container mx-auto px-4 py-8">
      <component :is="currentView"></component>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Navbar from './components/Navbar.vue'
import PasteList from './components/PasteList.vue'
import NewPaste from './components/NewPaste.vue'

const currentView = ref(PasteList)

const handleNavigation = (view: string) => {
  if (view === 'home') {
    currentView.value = PasteList
  } else if (view === 'new-paste') {
    currentView.value = NewPaste
  }
}
</script>
