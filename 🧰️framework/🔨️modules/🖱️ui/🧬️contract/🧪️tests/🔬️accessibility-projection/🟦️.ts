/** 🔬️ Canonical accessibilityProjectionSelfTests: the TypeScript half of the shared accessibility
 * projection laws, answering the very same `🧫️fixtures/♿️accessibility-projection.json` rows the Rust
 * `🔬️accessibility-unit` laws answer through `accessibility_role`/`accessibility_projection_node`.
 * Two independent implementations, one fixture — which is what makes "the wgpu renderer announces the
 * same tree React does" a law rather than a claim. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import type { Component, UiNodeRecord } from "../../../../🛂️manifest/🟦️.ts";
import { uiAccessibilityAnnouncedV1, uiAccessibilityIsFocusableV1, uiAccessibilityProjectionNodeV1, uiAccessibilityRoleV1, uiAccessibilityValueV1, uiProgressFractionV1 } from "../../♿️accessibility/🟦️.ts";

type RoleRow = { readonly id: string; readonly component: Component; readonly activatable: boolean; readonly role: string; readonly focusable: boolean };
type ExpectedRow = {
  readonly nodeId: number;
  readonly key: string;
  readonly depth: number;
  readonly role: string;
  readonly label: string | null;
  readonly description: string | null;
  readonly live: string;
  readonly shortcut: string | null;
  readonly hidden: boolean;
  readonly disabled: boolean;
  readonly focusable: boolean;
  readonly actionable: boolean;
  readonly valueMin?: number | null;
  readonly valueMax?: number | null;
  readonly valueNow?: number | null;
  readonly valueText?: string | null;
  readonly busy?: boolean;
};
type ProgressCase = { readonly id: string; readonly component: Component; readonly fraction: number | null; readonly accessibility: { readonly role: string; readonly focusable: boolean; readonly valueMin: number | null; readonly valueMax: number | null; readonly valueNow: number | null; readonly valueText: string | null; readonly busy: boolean } };
type Fixture = { readonly roles: readonly RoleRow[]; readonly document: { readonly nodes: readonly UiNodeRecord[] }; readonly expected: readonly ExpectedRow[]; readonly announced: readonly number[] };
type ToggleCase = { readonly id: string; readonly component: Component; readonly accessibleLabel: string; readonly expected: { readonly role: string; readonly stateAttribute: "aria-pressed" | "aria-checked"; readonly stateValue: boolean } };

/** ♿️ Answers every shared fixture row, returning how many assertions the corpus carried. */
export function accessibilityProjectionSelfTests(): number {
  const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("../../🧫️fixtures/♿️accessibility-projection.json", import.meta.url)), "utf8")) as Fixture;
  let checks = 0;
  const components = new Set<string>();
  for (const row of fixture.roles) {
    assert.equal(uiAccessibilityRoleV1(row.component, row.activatable), row.role, `${row.id}: implied role`);
    assert.equal(uiAccessibilityIsFocusableV1(row.component, row.activatable), row.focusable, `${row.id}: focusability`);
    components.add(row.component.type);
    checks += 2;
  }
  assert.equal(components.size, 19, "the fixture covers every one of the contract's 19 components");

  const toggles = JSON.parse(readFileSync(fileURLToPath(new URL("../../🧫️fixtures/♿️retained-toggle-semantics/🔣️.json", import.meta.url)), "utf8")) as { readonly cases: readonly ToggleCase[] };
  for (const [index, row] of toggles.cases.entries()) {
    const record = {
      id: index,
      key: `#toggle-${index}`,
      component: row.component,
      layout: { kind: "leaf", width: "hug", height: "hug" },
      style: {},
      activity: "idle",
      accessibility: { label: row.accessibleLabel },
    } as UiNodeRecord;
    const node = uiAccessibilityProjectionNodeV1(record, 0);
    assert.equal(node.role, row.expected.role, `${row.id}: role`);
    assert.equal(node.checked, row.expected.stateAttribute === "aria-checked" ? row.expected.stateValue : null, `${row.id}: checked`);
    assert.equal(node.pressed, row.expected.stateAttribute === "aria-pressed" ? row.expected.stateValue : null, `${row.id}: pressed`);
    checks += 3;
  }

  const projection = fixture.expected.map((row) => {
    const record = fixture.document.nodes.find((node) => node.id === row.nodeId);
    assert(record, `the fixture document declares node ${row.nodeId}`);
    return uiAccessibilityProjectionNodeV1(record, row.depth);
  });
  for (const [index, node] of projection.entries()) {
    const row = fixture.expected[index];
    assert.equal(node.key, row.key, `${row.key}: key`);
    assert.equal(node.role, row.role, `${row.key}: role`);
    assert.equal(node.label, row.label, `${row.key}: label`);
    assert.equal(node.description, row.description, `${row.key}: description`);
    assert.equal(node.live, row.live, `${row.key}: live`);
    assert.equal(node.shortcut, row.shortcut, `${row.key}: shortcut`);
    assert.equal(node.hidden, row.hidden, `${row.key}: hidden`);
    assert.equal(node.disabled, row.disabled, `${row.key}: disabled`);
    assert.equal(node.focusable, row.focusable, `${row.key}: focusable`);
    assert.equal(node.actionable, row.actionable, `${row.key}: actionable`);
    assert.equal(node.focused, false, "the pure projection stamps no live focus — only a renderer's own walk does");
    assert.equal(node.valueMin, row.valueMin ?? null, `${row.key}: valueMin`);
    assert.equal(node.valueMax, row.valueMax ?? null, `${row.key}: valueMax`);
    assert.equal(node.valueNow, row.valueNow ?? null, `${row.key}: valueNow`);
    assert.equal(node.valueText, row.valueText ?? null, `${row.key}: valueText`);
    assert.equal(node.busy, row.busy ?? false, `${row.key}: busy`);
    checks += 16;
  }
  assert.deepEqual(
    uiAccessibilityAnnouncedV1(projection).map((node) => node.nodeId),
    [...fixture.announced],
    "exactly the reachable, named nodes",
  );
  const decorative = projection.find((node) => node.hidden);
  assert(decorative, "the fixture declares a decorative node");
  assert.equal(decorative.role, "img", "a hidden node still carries the role it would have had, rather than vanishing");
  checks += 2;
  const progress = JSON.parse(readFileSync(fileURLToPath(new URL("../../🧫️fixtures/📶️progress.json", import.meta.url)), "utf8")) as { readonly cases: readonly ProgressCase[] };
  for (const row of progress.cases) {
    assert.equal(uiAccessibilityRoleV1(row.component, false), row.accessibility.role, `${row.id}: role`);
    assert.equal(uiAccessibilityIsFocusableV1(row.component, false), row.accessibility.focusable, `${row.id}: focusable`);
    const { role: _role, focusable: _focusable, ...value } = row.accessibility;
    assert.deepEqual(uiAccessibilityValueV1(row.component), value, `${row.id}: announced value`);
    assert(row.component.type === "progress", `${row.id}: a progress component`);
    assert.equal(uiProgressFractionV1(row.component.completed, row.component.total), row.fraction, `${row.id}: filled fraction`);
    checks += 5;
  }
  return checks;
}

if (import.meta.main) console.log(`[DEBUG] accessibility-projection-twin checks=${accessibilityProjectionSelfTests()}`);
