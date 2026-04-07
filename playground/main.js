const wasm = await WebAssembly.instantiate(
  await Bun.file("./target/wasm32-unknown-unknown/release/cheatscan.wasm").arrayBuffer(),
  {}
)

const e = wasm.instance.exports
const memory = e.memory
let mem = new Uint8Array(memory.buffer)
let dv = new DataView(memory.buffer)

function refreshViews() {
  mem = new Uint8Array(memory.buffer)
  dv = new DataView(memory.buffer)
}

let heapPtr = Number(e.__heap_base.value)

function alloc(size, align = 1) {
  heapPtr = (heapPtr + align - 1) & ~(align - 1)
  const ptr = heapPtr
  heapPtr += size
  return ptr
}

const initial = Uint8Array.from([10, 20, 30, 40])
const next = Uint8Array.from([10, 25, 30, 35])

const initialPtr = alloc(initial.length, 1)
mem.set(initial, initialPtr)

const nextPtr = alloc(next.length, 1)
mem.set(next, nextPtr)

const errorPtr = alloc(1, 1)

// `Configuration` est passé par valeur via l'ABI C WASM.
// Avec `#[repr(C)] struct Configuration { alignment: u8, base_address: u32, endianness: u8, value_type: u8 }`,
// le lowering WASM le découpe ici en 3 mots i32 :
//   word0 = alignment + padding
//   word1 = base_address
//   word2 = endianness | (value_type << 8)
const configWord0 = 0 // value_type = U8
const configWord1 = 0 // endianness = Little
const configWord2 = 0 // anigment = Unaligned
const baseAddress = 0

const scanner = e.cheatscan_new_from_unknown(
  configWord0,
  configWord1,
  configWord2,
  baseAddress,
  initialPtr,
  initial.length,
  errorPtr
)

refreshViews()

if (scanner === 0) {
  console.log("error:", mem[errorPtr])
  process.exit(1)
}

e.cheatscan_scan_previous(
  scanner,
  nextPtr,
  next.length,
  4 // Gt
)

refreshViews()

const count = e.cheatscan_count(scanner)
console.log("count =", count)

const resultsPtr = alloc(16 * 4, 4)

const written = e.cheatscan_write_results(
  scanner,
  resultsPtr,
  16,
  0
)

refreshViews()

for (let i = 0; i < written; i++) {
  console.log("result", i, "=", dv.getUint32(resultsPtr + i * 4, true))
}

e.cheatscan_free(scanner)