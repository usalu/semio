/** 🧪️ The Flow browser runtime hands the host's runtime assets to the wasm module's `semioSetRuntimeAssets` door once, in catalog order. */
import { expect, test } from "bun:test";
import { MockFlowBridge } from "../🎭️mock-flow-bridge/🟦️.ts";
import { createFlowBrowserRuntime } from "../../🌐️browser/🏃️runtime/🟨️.js";

const assets = () => ({ math: new Uint8Array([0]), serif: new Uint8Array([1]), serifItalic: new Uint8Array([2]), mono: new Uint8Array([3]), emoji: new Uint8Array([4]), labelFont: new Uint8Array([5]), iconPack: new Uint8Array([6]) });

test("the Flow browser runtime installs the host's runtime assets once, in catalog order, before any session opens", async () => {
  const installed: Uint8Array[][] = [];
  const offered = assets();
  const runtime = await createFlowBrowserRuntime({ source: new MockFlowBridge(new WebAssembly.Memory({ initial: 2 })).exports, bindings: { semioSetRuntimeAssets: (...bytes: Uint8Array[]) => installed.push(bytes) }, runtimeAssets: offered });
  expect(installed).toHaveLength(1);
  expect(installed[0]).toEqual([offered.math, offered.serif, offered.serifItalic, offered.mono, offered.emoji, offered.labelFont, offered.iconPack]);
  await runtime.close();
});

test("runtime assets without a Flow asset door are refused instead of silently dropped", async () => {
  await expect(createFlowBrowserRuntime({ source: new MockFlowBridge(new WebAssembly.Memory({ initial: 2 })).exports, bindings: {}, runtimeAssets: assets() })).rejects.toThrow("runtime assets need Flow bindings with semioSetRuntimeAssets");
});

test("a runtime opened without runtime assets never touches the asset door", async () => {
  let calls = 0;
  const runtime = await createFlowBrowserRuntime({ source: new MockFlowBridge(new WebAssembly.Memory({ initial: 2 })).exports, bindings: { semioSetRuntimeAssets: () => void calls++ } });
  expect(calls).toBe(0);
  await runtime.close();
});
