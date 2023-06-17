<script setup lang="ts">
	defineProps<{
	  column: number
	  row: number
	}>()
</script>

<template>
  <div @click="$emit('make-move', row, column)"></div>
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
