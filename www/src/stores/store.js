import * as wasm from 'wasm'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useWasmStore = defineStore('wasm', () => {
  const boardState = ref([null, null, null, null, null, null, null, null, null])
  function initializeWasm() {
    console.log(wasm.health_check())
    const module = new wasm.GameWrapper()
    boardState.value = module.board();
  }

  function makeMove(cell) {
    const module = new wasm.GameWrapper()
    module.make_move(cell)
    boardState.value = module.board()
    console.log("Made Move")
  }

  return { boardState, initializeWasm, makeMove }
})
