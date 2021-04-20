import init , { source, compute, say, echo, test_log } from "./wasm.js";

await init(source);

const testCompute = compute(42);

const testSay = say()
console.log(testSay)

const testEcho = echo("Hola Rust!!!");
console.log(testEcho);

test_log();