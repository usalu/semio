import { expect, test } from "bun:test";
import { MockFlowBridge } from "../🎭️mock-flow-bridge/🟦️.ts";
import { createFlowBrowserRuntime } from "../../🌐️browser/🏃️runtime/🟨️.js";

const fonts = () => ({ math: new Uint8Array([0]), serif: new Uint8Array([1]), serifItalic: new Uint8Array([2]), mono: new Uint8Array([3]), emoji: new Uint8Array([4]) });

test("the Flow browser runtime installs the host's compiler fonts once, in role order, before any session opens", async () => {
  const installed: Uint8Array[][] = [];
  const offered = fonts();
  const runtime = await createFlowBrowserRuntime({ source: new MockFlowBridge(new WebAssembly.Memory({ initial: 2 })).exports, bindings: { flowSetCompilerFonts: (...bytes: Uint8Array[]) => installed.push(bytes) }, compilerFonts: offered });
  expect(installed).toHaveLength(1);
  expect(installed[0]).toEqual([offered.math, offered.serif, offered.serifItalic, offered.mono, offered.emoji]);
  await runtime.close();
});

test("compiler fonts without a Flow font door are refused instead of silently dropped", async () => {
  await expect(createFlowBrowserRuntime({ source: new MockFlowBridge(new WebAssembly.Memory({ initial: 2 })).exports, bindings: {}, compilerFonts: fonts() })).rejects.toThrow("compiler fonts need Flow bindings with flowSetCompilerFonts");
});

test("a runtime opened without compiler fonts never touches the font door", async () => {
  let calls = 0;
  const runtime = await createFlowBrowserRuntime({ source: new MockFlowBridge(new WebAssembly.Memory({ initial: 2 })).exports, bindings: { flowSetCompilerFonts: () => void calls++ } });
  expect(calls).toBe(0);
  await runtime.close();
});
