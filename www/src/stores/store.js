import { defineStore } from 'pinia'
//import * as wasm from 'wasm'

export const useWasmStore = defineStore('wasm', () => {
  //const count = ref(0)
  //const name = ref('Eduardo')
  //const doubleCount = computed(() => count.value * 2)

  function makeMove(column, row) {
    //const result = wasm.make_move(column, row)
    console.log("Made Move")
  }
/*
  function initializeWasm() {
    console.log("initializing wasm")
    const blah = wasm.healtchech()    
    console.log(blah)
  }
*/
  return { makeMove }
})
