/** 🌲️ Independent source-slot transfer and schema oracle for retained runtime tree closure. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { testBuiltTreeRetirementFixture } from "../../../🧬️contract/♻️retirement/🌲️built/📜️script.ts";

export function testRuntimeTreeRetirement(): void {
  const read = (path: string) => readFileSync(new URL(path, import.meta.url), "utf8");
  const fixture = JSON.parse(read("./🧫️fixture/🔣️.json"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(read("./🧬️schema.json")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  let active: Uint8Array | undefined;
  let handback: Uint8Array | undefined;
  let source: Uint8Array | undefined = Buffer.from("a");
  const take = () => { if (active || !source) return false; active = source; source = undefined; return true; };
  const trace: object[] = [];
  const observe = (event: string) => { const row: Record<string, string | null> = { event, active: active ? Buffer.from(active).toString() : null, source: source ? Buffer.from(source).toString() : null }; if (event.includes("handback")) row.handback = handback ? Buffer.from(handback).toString() : null; trace.push(row); };
  assert(take()); observe("begin-a");
  const zeroActive = active; const close = (grant: number) => { if (grant === 0 || !active) return false; active = undefined; return true; };
  assert.equal(close(0), fixture.ownership.zeroGrantAdvances); assert.equal(active, zeroActive); observe("zero-grant");
  source = Buffer.from("b"); const exactSource = source; const exactActive = active;
  assert.equal(take(), fixture.ownership.occupiedTransfer);
  assert.equal(source, exactSource); assert.equal(active, exactActive); observe("begin-b-occupied");
  handback = active; active = undefined; assert.equal(handback, exactActive); observe("handback-a");
  handback = undefined; observe("finish-handback-a"); assert(take()); assert.equal(active, exactSource); observe("begin-b");
  active = undefined; observe("finish-b"); assert.deepEqual(trace, fixture.trace);
  assert.deepEqual(JSON.parse(Buffer.from(JSON.stringify(fixture.foreign)).toString()), fixture.foreign);
  let hostile = 0;
  for (const [key, value] of Object.entries(fixture.ownership)) { assert(!validate({ ...fixture, ownership: { ...fixture.ownership, [key]: !value } })); hostile++; }
  testBuiltTreeRetirementFixture();
  const runtime = read("../../📦️packages/🦀️rust/♻️reconcile.rs");
  assert(runtime.includes('mod tree_retirement;'), "runtime must mount the exact retained tree owner");
  const owner = read("./🦀️.rs");
  assert(owner.includes("Option<ui_contract::BuiltTreeRetirement>"));
  assert(!owner.includes("close_built_node_page_one") && !owner.includes("close_ui_value_page_one"));
  assert(!runtime.includes("SURFACE_RECONCILE_TREE_RETIRE_DEPTH"));
  assert(runtime.includes("fn try_reserve_surface_reconcile_handback"));
  assert(runtime.includes("fn close_admitted_step"));
  assert.equal((runtime.match(/state\.close_admitted_step\(\)/g) ?? []).length, 2);
  const inventory = read("../../📏️ownership/🧪️tests/🦀️.rs");
  assert(inventory.includes("UI_BUILT_CHILD_RETIRE_SLOTS") && !inventory.includes("SURFACE_RECONCILE_TREE_RETIRE_DEPTH"));
  const cursor = runtime.slice(runtime.indexOf("    fn retire_one(&mut self) -> bool {", runtime.indexOf("impl SurfaceReconcileCursor")));
  assert(cursor.indexOf("self.retire_tree.step()") >= 0 && cursor.indexOf("self.retire_tree.step()") < cursor.indexOf("self.retire_tree.try_begin"));
  console.log(`[DEBUG] runtime-tree source:7 exact source/handback transitions,${hostile} hostile contracts,384 pages,9 typed fields; native runtime closure unverified`);
}
