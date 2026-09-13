/** 🪟 Every host window binding names a REAL window instance — no synthetic `window` alias.
 *
 * 🦾 WAVE B56 LAW: for an app whose host view carries window instances the binding table contains
 * exactly one entry per carried instance and NO entry named `window`. The alias this law used to pin
 * (wave W-G3 §8.29) mounted a fourteenth surface no pane reads, and its reconcile reservation refusal
 * starved every real surface (`reserve_refusal=1:window:registry-reservation-unavailable`, wave B54
 * §6.4). Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import assert from "node:assert/strict";
import { describe, it } from "vitest";
import Ajv from "ajv";
import { leftoverInspectionPanelHash, leftoverInspectionRefreshScope, windowHostContextBindings } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import fixture from "../../🧫️fixtures/🔬️window-host-context/🔣️.json";

type Binding = { readonly surface: { readonly instance: number; readonly surface: string }; readonly bodyKey: string; readonly windowKey: string };

function oracleWindowHostContextBindings(instanceId: number, windows: readonly { readonly key: string; readonly bodyKey?: string }[], view: { readonly windowInstances?: ReadonlyArray<{ readonly id: string }> }): Binding[] {
  const known = new Set((view.windowInstances ?? []).map((window) => window.id));
  const bindings: Binding[] = [];
  for (const target of windows) {
    if (!target.bodyKey || !known.has(target.key)) continue;
    bindings.push({ surface: { instance: instanceId, surface: target.key }, bodyKey: target.bodyKey, windowKey: target.key });
  }
  return bindings;
}

export function testWindowHostContext(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile({
    type: "object",
    additionalProperties: false,
    required: ["instanceId", "view", "windows", "expected", "inspectionRefresh"],
    properties: {
      instanceId: { type: "integer" },
      view: { type: "object" },
      windows: { type: "array" },
      expected: { type: "array" },
      inspectionRefresh: { type: "array" },
    },
  });
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const actual = windowHostContextBindings(fixture.instanceId, fixture.windows, fixture.view);
  const oracle = oracleWindowHostContextBindings(fixture.instanceId, fixture.windows, fixture.view);
  assert.deepEqual(actual, fixture.expected);
  assert.deepEqual(oracle, fixture.expected);
  assert.equal(fixture.view.windowInstances.length > 0, true);
  assert.deepEqual(actual.filter((binding) => binding.surface.surface === "window"), [], "no synthetic `window` surface may be bound for an app with window instances");
  assert.deepEqual([...new Set(actual.map((binding) => binding.surface.surface))], actual.map((binding) => binding.surface.surface));
  for (const row of fixture.inspectionRefresh) {
    const scope = leftoverInspectionRefreshScope(row.selectedIds);
    assert.equal(scope?.kind ?? null, row.kind);
    const hash = leftoverInspectionPanelHash(row.selectedIds, "abc");
    assert.equal(hash === undefined, row.omitHash);
    assert.equal(hash === undefined, Boolean(row.selectedIds.length));
  }
  console.log(`[DEBUG] window-host-context bindings=${actual.length} inspection=${fixture.inspectionRefresh.length}`);
}

describe("window host context", () => {
  it("binds only real window instances and refreshes Inspection when leftover selects", () => {
    testWindowHostContext();
  });
});
