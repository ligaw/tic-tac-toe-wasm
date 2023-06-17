<script setup lang="ts">
  import Square from "./Square.vue"
  
  function make_move(column, row) {
    console.log(column, row);
  }
</script>

<template>
  <div id="board" class="grid grid-cols-3 gap-3 h-full bg-amber-700">
  <template v-for="column in [1,2,3]">
    <template v-for="row in [1,2,3]">
      <Square @make-move="make_move" :column="column" :row="row" />    
    </template>
  </template>
  </div>
</template>
