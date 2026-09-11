/** 🪟 Leftover Viewport dirty defaults the guest surface to `window`; host must bind that id. */
import assert from "node:assert/strict";
import { describe, it } from "vitest";
import Ajv from "ajv";
import { leftoverInspectionPanelHash, leftoverInspectionRefreshScope, windowHostContextBindings } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import fixture from "../../🧫️fixtures/🔬️window-host-context/🔣️.json";

type Binding = { readonly surface: { readonly instance: number; readonly surface: string }; readonly bodyKey: string; readonly windowKey: string };

function oracleWindowHostContextBindings(instanceId: number, windows: readonly { readonly key: string; readonly bodyKey?: string }[], view: { readonly windowInstances?: ReadonlyArray<{ readonly id: string }> }): Binding[] {
  const known = new Set((view.windowInstances ?? []).map((window) => window.id));
  const bindings: Binding[] = [];
  let alias: Binding | undefined;
  for (const target of windows) {
    if (!target.bodyKey || !known.has(target.key)) continue;
    bindings.push({ surface: { instance: instanceId, surface: target.key }, bodyKey: target.bodyKey, windowKey: target.key });
    alias = { surface: { instance: instanceId, surface: "window" }, bodyKey: target.bodyKey, windowKey: target.key };
  }
  if (alias) bindings.push(alias);
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
  it("aliases leftover window and refreshes Inspection when leftover selects", () => {
    testWindowHostContext();
  });
});
