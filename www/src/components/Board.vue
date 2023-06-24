<script setup>
  import { useWasmStore } from '@/stores/store'
  import Square from "@/components/Square.vue"
  import { storeToRefs } from 'pinia'

  const store = useWasmStore()
  const { boardState } = storeToRefs(store)

</script>

<template>
  <div id="board" class="grid grid-cols-3 gap-3 h-full bg-amber-700">
  <template v-for="row in [0,3,6]">
    <template v-for="column in [0,1,2]" :key="row + column">
      <Square :cell="row + column" :cell-value="boardState[row + column]" />
    </template>
  </template>
  </div>
</template>
