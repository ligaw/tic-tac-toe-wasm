<script setup>
import Board from '@/components/Board.vue'
import { useWasmStore } from '@/stores/store.js'
import { onMounted, onDeactivated } from 'vue'

onMounted(() => {
  console.log("mounting module")
  const store = useWasmStore()
  store.initializeWasm()
  console.log("mounted was module")
});

onDeactivated(() => {
  //wasm.free()
})
</script>

<template>
<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 h-screen">
  <div class="mx-auto max-w-3xl h-full">
    <Board />
  </div>
</div>
</template>
