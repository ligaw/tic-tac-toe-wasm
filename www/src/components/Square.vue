<script setup>
  import { useWasmStore } from '@/stores/store.js'

  const store = useWasmStore()
  const props = defineProps({
    cell: null,
    cellValue: null
  })
</script>

<template>
  <div @click="store.makeMove(cell)">{{ cellValue == 'Empty' ? '' : cellValue }}</div>
</template>

<style scoped>
div {
  background-color: #fff;
  display: flex;
  justify-content: center;
  place-items: center;
}
div:hover {
	cursor: pointer;
}
</style>
