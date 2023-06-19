import { defineStore } from 'pinia'
import * as wasm from 'wasm'

export const useWasmStore = defineStore('wasm', () => {
  function initializeWasm() {
    console.log("initializing wasm")
    const blah = wasm.healthcheck()    
    console.log(blah)
  }

  function makeMove(column, row) {
    const result = wasm.make_move(column, row)
    console.log(result)
    console.log("Made Move")
  }


  return { initializeWasm, makeMove }
})
