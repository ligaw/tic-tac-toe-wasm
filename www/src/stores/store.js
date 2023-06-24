import * as wasm from 'wasm'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useWasmStore = defineStore('wasm', () => {
  const boardState = ref([null, null, null, null, null, null, null, null, null])
  let wasmModule

  function initializeWasm() {
    wasmModule = new wasm.GameWrapper()
    boardState.value = wasmModule.board();
  }

  function makeMove(cell) {
    wasmModule.make_move(cell)
    boardState.value = wasmModule.board()
  }

  return { boardState, initializeWasm, makeMove }
})
