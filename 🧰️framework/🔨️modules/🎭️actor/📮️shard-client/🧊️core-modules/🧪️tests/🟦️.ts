import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { ShardCoreModules } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

/** 🧪️ Laws of {@link ShardCoreModules}, driven by the language-agnostic fixture: one compilation per served URL for any number of activations, a seed adopted without compiling, and every entry dropped with its last holder. V8's own `WebAssembly` compiles the fixture module, and its instances prove that one compiled module serves independent memories. */
const bytes = Uint8Array.from(fixture.module.hex.match(/../gu)!.map((pair) => Number.parseInt(pair, 16)));

describe("ShardCoreModules", () => {
  it("validates its fixture with an independent JSON schema validator", () => {
    const ajv = new Ajv({ strict: true, allErrors: true });
    const valid = ajv.validate(schema, fixture);
    expect(ajv.errors ?? []).toEqual([]);
    expect(valid).toBe(true);
  });

  for (const testCase of fixture.cases) {
    it(testCase.id, async () => {
      const seed = await WebAssembly.compile(bytes);
      const modules = new ShardCoreModules((_url) => WebAssembly.compile(bytes));
      for (const step of testCase.steps as readonly ({ acquire: string; holder: string; seeded?: boolean } | { release: string; dropped: string[] })[]) {
        if ("acquire" in step) expect(await modules.acquire(step.holder, step.acquire, step.seeded ? seed : undefined)).toBeInstanceOf(WebAssembly.Module);
        else expect(modules.release(step.release).sort()).toEqual([...step.dropped].sort());
      }
      expect(modules.compilations).toBe(testCase.expected.compilations);
      expect(Object.fromEntries([...modules.entries.keys()].map((url) => [url, modules.holders(url)]))).toEqual(testCase.expected.held);
      expect(modules.seeds()).toHaveLength(testCase.expected.seeds);
    });
  }

  it("compiles once for N instances whose memories stay independent, and closing them releases the entry", async () => {
    const modules = new ShardCoreModules((_url) => WebAssembly.compile(bytes));
    const holders = [...new Set(fixture.module.calls.map((call) => call.holder))];
    const instances = new Map<string, WebAssembly.Instance>();
    for (const holder of holders) instances.set(holder, new WebAssembly.Instance(await modules.acquire(holder, "core.wasm")));
    const advance = (holder: string, delta: number) => (instances.get(holder)!.exports[fixture.module.export] as (delta: number) => number)(delta);
    expect(fixture.module.calls.map((call) => advance(call.holder, call.delta))).toEqual(fixture.module.results);
    expect(modules.compilations).toBe(1);
    expect(new Set([...instances.values()].map((instance) => instance.exports.memory)).size).toBe(holders.length);
    expect(holders.flatMap((holder) => modules.release(holder))).toEqual(["core.wasm"]);
    expect(modules.entries.size).toBe(0);
    expect(modules.seeds()).toEqual([]);
  });

  it("leaves no entry behind for a failed compilation and compiles again for the next holder", async () => {
    let attempts = 0;
    const modules = new ShardCoreModules((_url) => (attempts += 1) === 1 ? Promise.reject(new Error("core-module.fetch-failed")) : WebAssembly.compile(bytes));
    await expect(modules.acquire("a#1", "core.wasm")).rejects.toThrow("core-module.fetch-failed");
    expect(modules.entries.size).toBe(0);
    expect(await modules.acquire("b#2", "core.wasm")).toBeInstanceOf(WebAssembly.Module);
    expect(modules.compilations).toBe(2);
  });

  it("is self-contained, so the generated shard worker can embed its source text", async () => {
    const embedded = new Function(`${ShardCoreModules.toString()}; return ShardCoreModules;`)() as typeof ShardCoreModules;
    const modules = new embedded((_url: string) => WebAssembly.compile(bytes));
    await Promise.all(["a#1", "b#2", "c#3"].map((holder) => modules.acquire(holder, "core.wasm")));
    expect(modules.compilations).toBe(1);
    expect(modules.release("a#1")).toEqual([]);
  });
});
