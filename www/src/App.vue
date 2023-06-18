<script setup>
import Board from '@/components/Board.vue'
import { useWasmStore } from '@/stores/store.js'
import { onMounted, onDeactivated } from 'vue'
//import * as wasm from 'wasm'

onMounted(() => {
  console.log("mounting module")
  //console.log(wasm.healthcheck())
  //console.log("mounted was module")
});

onDeactivated(() => {
  //wasm.free()
})
</script>

<template>
<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 h-screen">
  <div class="mx-auto max-w-3xl h-full">
    Hello
    <Board />
  </div>
</div>
</template>
