//! ⏱️ Exercises the consumed Flow module's installed real browser clock.

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { flowWasmContract } from "../../🔮️oracles/🛂️contract-admission/🟦️.ts";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

//#region ⏱️ConsumedClock
export async function testFlowBrowserClock(packageRoot) {
  const fixture = JSON.parse(await readFile(new URL("../🧫️fixtures/🚀️browser-startup/🔣️.json", import.meta.url), "utf8"));
  assert.equal(flowWasmContract("FlowBrowserStartupV1")(fixture), true);
  const descriptor = Object.getOwnPropertyDescriptor(globalThis, "performance");
  const receiver = globalThis.performance;
  const dag = JSON.parse(await readFile(new URL("../🧫️fixtures/🧵️dag-retained/🔣️.json", import.meta.url), "utf8"));
  let samples = 0;
  let delayedSample = Infinity;
  let elapsedPause = 0;
  let first;
  let last;
  Object.defineProperty(globalThis, "performance", { configurable: true, value: { now() {
    if (samples === delayedSample) elapsedPause = 20;
    const value = receiver.now() + elapsedPause;
    first ??= value;
    last = value;
    samples += 1;
    return value;
  } } });
  try {
    const published = await import(pathToFileURL(join(packageRoot, "🌐️browser", "🟨️.js")).href);
    const runtime = await published.createFlowBrowserRuntime({ source: await readFile(join(packageRoot, "flow_core_bg.wasm")) });
    const session = runtime.openSession();
    const started = samples;
    await session.catalogueJson().result;
    delayedSample = samples + 3;
    await session.setSelection(JSON.stringify(dag.selection)).result;
    assert.equal(elapsedPause, 20, "a wall-clock pause occurred during scene synchronization");
    await runtime.close();
    assert.ok(started >= 1, "the consumed module must install its real clock before its first Flow request");
    assert.ok(samples > started && samples >= fixture.clock.minimumSamples, "Flow turns must resample the installed clock");
    assert.ok(last > first, "the consumed clock must advance rather than freeze its startup sample");
  } finally {
    if (descriptor) Object.defineProperty(globalThis, "performance", descriptor);
    else Reflect.deleteProperty(globalThis, "performance");
  }
}
//#endregion ⏱️ConsumedClock
