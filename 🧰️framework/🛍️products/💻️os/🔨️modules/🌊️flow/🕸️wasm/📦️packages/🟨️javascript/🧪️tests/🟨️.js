//! ⏱️ Exercises the consumed Flow module's installed real browser clock.

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import Ajv from "ajv";
import { createFlowBrowserFeatures } from "../🌐️flow-browser.js";

//#region ⏱️ConsumedClock
export async function testFlowBrowserClock() {
  const fixture = JSON.parse(await readFile(new URL("../../../🧪️fixtures/🚀️browser-startup/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(await readFile(new URL("../../../🧪️fixtures/🚀️browser-startup/🧬️.schema.json", import.meta.url), "utf8"));
  assert.equal(new Ajv({ strict: true }).compile(schema)(fixture), true);
  const descriptor = Object.getOwnPropertyDescriptor(globalThis, "performance");
  const receiver = globalThis.performance;
  let samples = 0;
  let first;
  let last;
  Object.defineProperty(globalThis, "performance", { configurable: true, value: { now() {
    const value = receiver.now();
    first ??= value;
    last = value;
    samples += 1;
    return value;
  } } });
  try {
    const { features } = await createFlowBrowserFeatures({ source: await readFile(new URL("../../../../🫀️core/🕸️bindings/flow_core_bg.wasm", import.meta.url)) });
    const started = samples;
    await features.document.catalogueJson({}).result;
    await features.lifetime.close();
    assert.ok(started >= 1, "the consumed module must install its real clock before its first Flow request");
    assert.ok(samples > started && samples >= fixture.clock.minimumSamples, "Flow turns must resample the installed clock");
    assert.ok(last > first, "the consumed clock must advance rather than freeze its startup sample");
    console.log(`[DEBUG] consumed Flow Wasm sampled real ${fixture.clock.source} ${samples} times from ${first}ms to ${last}ms, then closed terminal-empty`);
  } finally {
    if (descriptor) Object.defineProperty(globalThis, "performance", descriptor);
    else Reflect.deleteProperty(globalThis, "performance");
  }
}
//#endregion ⏱️ConsumedClock
