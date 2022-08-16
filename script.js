import init, { add } from "./pkg/hello_wasm.js";

async function main() {
  await init();

  console.log(`The answer is ${add(40, 2)}`);
}

main();
