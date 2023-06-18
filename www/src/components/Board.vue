<script setup>
  import Square from "@/components/Square.vue"
</script>

<template>
  <div id="board" class="grid grid-cols-3 gap-3 h-full bg-amber-700">
  <template v-for="column in [1,2,3]">
    <template v-for="row in [1,2,3]">
      <Square :row="row" :column="column" />
    </template>
  </template>
  </div>
</template>
