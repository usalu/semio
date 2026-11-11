/** 🖼️ Keep a native paint opportunity separate from the controlled animation clock. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {captureNativePaint,paintBarrier} from "../🧰️tools.mjs";
const sample=JSON.parse(await readFile(new URL("./paint-barrier.cases.json",import.meta.url),"utf8"));
let paints=0;
globalThis.requestAnimationFrame=callback=>{paints++;callback(0);return 1};
const page={evaluate:async fn=>fn()};
await captureNativePaint(page);
globalThis.requestAnimationFrame=()=>{throw Error("fake clock must not drive native paint")};
await paintBarrier(page);
assert.equal(paints,sample.expected,sample.id);
console.log("PASS "+sample.id);
