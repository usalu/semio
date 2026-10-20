/** 🧪️ Standard Set and mathjs independently verify authored membership and backing policy. */
import { expect, test } from "bun:test";
import { create, all } from "mathjs";
import fixture from "./🧫️fixtures/🔣️.json";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
const math = create(all, {});
test("Puzzle3D borrowed focus preserves independent Set membership", async () => {
  for (const row of fixture.selectionCases) {
    const owned = new Set(row.selected);
    expect(owned.size === 0 || owned.has(row.target)).toBe(row.frames);
  }
  expect(math.sum(fixture.buffers.map(row => Number("vortices" in row ? row.vortices : 0)))).toBe(192);
  expect(math.sum(fixture.buffers.map(row => Number("mutations" in row ? row.mutations : 0)))).toBe(8196);
  const action = readModuleTreeSource(new URL("../🦀️.rs", import.meta.url));
  const document = readModuleTreeSource(new URL("../../🗂️document/🦀️.rs", import.meta.url));
  expect(action).not.toContain("HashSet::with_capacity");
  expect(document).not.toContain("HashSet::with_capacity");
  expect(document).not.toContain("self.result");
  for (const source of [action, document]) {
    expect(source).toContain("scope.admit_owner(Self::original_birth_demands()?)?");
    expect(source).toContain("retained_capacity_bytes: capacity");
  }
  console.log("[DEBUG] Puzzle3D original construction buffers=8 borrowed selection cases=5 independent Set/mathjs agree");
});

test("Puzzle original fixture keeps one finite stock through work and close", async () => {
  const fixture = await Bun.file(new URL("../../../../../../../../../../🎮️commands/📤️jobs/🧪️tests/🧫️fixtures/🎟️source/🔣️.json", import.meta.url)).json();
  const totals = math.sum(fixture.receipts, 0);
  expect(math.subtract([fixture.stock.items, fixture.stock.copyBytes, fixture.stock.capacityBytes, fixture.stock.releaseBytes], totals)).toEqual(fixture.remaining);
  const source = readModuleTreeSource(new URL("../../../../../../../../../../🎮️commands/📤️jobs/🦀️.rs", import.meta.url));
  expect(source).not.toContain("Box::leak");
  expect(source).not.toContain("StepContext::new");
  expect(source).not.toContain("test_payload_authority");
  expect(source).toContain(".turn_with_decoder(grant, &mut self.decoder, advance)");
  expect(source).toContain(".retire(fixture_stock())");
  expect(source).toContain("cx.with_original_work_loan(budget.fuel, budget.deadline_us");
  expect(source).not.toContain("narrow_original_work_budget");
  expect(source).toContain("source: RetainedCloneGrant, clock: fn() -> Option<u64>, deadline: u64, fuel: u64");
  console.log("[DEBUG] Puzzle original fixture stock is finite and cumulative; independent mathjs receipts preserve separate currencies");
});


test("Puzzle native app and brush fixtures preserve original close custody", async () => {
  const app = readModuleTreeSource(new URL("../../../🧪️tests/🧰️context/🦀️.rs", import.meta.url));
  const brushFiles = ["🦀️.rs", "🍓️placement-compatibility-and-fill-laws/🦀️.rs", "🥦️suggestions-run-laws/🦀️.rs"];
  const brush = (await Promise.all(brushFiles.map((file) => Promise.resolve(readModuleTreeSource(new URL(`../../../⏳️precompute/🖌️brush/🧪️tests/🔬️unit/${file}`, import.meta.url)))))).join("\n");
  expect(app).toContain("scope: semio_framework_plugin::artifact_app_laws::OriginalFixtureScope");
  expect(app).toContain("pub fn close_witness(app: &mut Puzzle3dApp)");
  expect(app).not.toContain("std::mem::forget(raw)");
  expect(app).toContain("PluginApp::maintenance_step(raw, cx, loan)");
  expect(app).toContain("artifact_app_laws::close_registered_fixture_app(");
  expect(brush).toContain("cx.with_original_work_loan(fuel, deadline");
  expect(brush).not.toContain("StepContext::new");
  expect(brush).not.toContain("narrow_original_work_budget");
  console.log("[DEBUG] Puzzle native app retains refused close custody; brush source borrows work budget without replacing original currencies");
});
