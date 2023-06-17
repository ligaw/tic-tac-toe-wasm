<script setup lang="ts">
</script>

<template>
  <div id="board" class="grid grid-cols-3 gap-3 h-full bg-amber-700">
    <div>0,0</div>
    <div>0,1</div>
    <div>0,2</div>
    <div>1,0</div>
    <div>1,1</div>
    <div>1,2</div>
    <div>2,0</div>
    <div>2,1</div>
    <div>2,2</div>
  </div>
</template>

<style scoped>
#board div {
  background-color: #fff;
  display: flex;
  justify-content: center;
  place-items: center;
}
</style>
