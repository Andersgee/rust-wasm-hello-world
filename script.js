import init, { add, Example } from "./pkg/rust_wasm_hello_world.js";
//import { memory } from "./pkg/rust_wasm_hello_world_bg.wasm.d.ts";

/**
 * view into the linear webassembly memory.
 *
 * note: be reasonable with this.
 * for example: calling .free() on a struct that gave the ptr will make this array reference garbage.
 * todo: find out under which conditions this becomes garbage. probably any and all allocation in the specific struct?
 * also: there nothing preventing us from writing outside the array bounds... corrupting whatever comes after.
 */
function f32arrayFromMemory(buffer, ptr, len) {
  return new Float32Array(buffer, ptr, len);
}

async function main() {
  const wasm = await init();

  console.log(`The answer is ${add(40, 2)}`);
  const buf = wasm.memory.buffer;
  const example = new Example();
  console.log(example);

  console.log(example.my_number);
  example.my_number = 99;

  console.log(example.my_number);
  console.log(example.my_vec);

  const kek = example.my_vec;
  console.log(example.my_vec);

  kek[1] = 919221;
  console.log(example.my_vec);

  console.log(kek.length);

  example.my_vec[2] = 11111;
  console.log(example.my_vec);

  example.my_vec = new Float32Array([91, 83, 2]);
  console.log(example.my_vec);

  console.log(example.sum_my_vec());

  example.my_vec = [1, 3, 5, 99];
  console.log(example.sum_my_vec());
}

main();
