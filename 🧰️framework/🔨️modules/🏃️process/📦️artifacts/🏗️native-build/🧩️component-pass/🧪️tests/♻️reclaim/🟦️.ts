import { registerIsolatedBuildTestV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect } from "bun:test";
import { mkdtempSync } from "node:fs";
import { join } from "node:path";
import { lawStoragePolicyV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { ConcurrentEditErrorV1 } from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { artifacts, stubCargo, stubWorkspace, runStub } from "../🎭️stub/🟦️.ts";

for (const count of [1, 2]) registerIsolatedBuildTestV1(import.meta.path, `reclaim preserves Cargo's hard failure after ${count} pass(es)`, { bodyMilliseconds: 60000, scope: { kind: "body" } }, async checkpoint => {
  const root = mkdtempSync(join(artifacts, "reclaim-")), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const passes = count === 2 ? [stubWorkspace(root, "earlier", ["comp-earlier"], "release"), pass] : [pass];
  const logs: string[] = [];
  const failure = await runStub(root, { version: 1, passes }, stubCargo(root), {
    generation: "fresh", storageBytes: 2 * lawStoragePolicyV1.realImageBytes, log: line => logs.push(line),
  }, "reclaim-error").then(() => undefined, (error: unknown) => error);
  expect(failure).not.toBeInstanceOf(ConcurrentEditErrorV1);
  expect(String((failure as Error).message)).toMatch(/Native build failed: comp-b/u);
  for (const { id } of passes) expect(logs.some(line => line.includes(`${id}: reclaimed 0 file(s)`)), logs.join("\n")).toBe(true);
  checkpoint();
}, /Native build failed: comp-b/u);
