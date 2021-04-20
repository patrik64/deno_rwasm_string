import init , { source, compute, say, echo } from "./wasm.js";

await init(source);

const testCompute = compute(42);
console.log(testCompute);

const testSay = say();
console.log(testSay);

const testEcho = echo("Hola Rust!!!");
console.log(testEcho);