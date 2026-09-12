/** 🪟️ The React half of the shared mode-layout law — same fixture as the wgpu dock's own
 * (`🧫️fixtures/🪟️app-mode-layouts/🔣️.json`, Rust twin
 * `🧱️elements/🛰️Dock/🧪️tests/🪟️app-mode-layouts/🦀️.rs`).
 *
 * 🏁️ What these encode: an app's authored mode layout is ONE contract and both renderer targets have
 * to read it the same way — which window instances a mode places, in what tree, at what fractions,
 * and which one is focused. React has read it since `resolveLayoutForMode`/`resolveFrameworkLayoutSeed`
 * existed; the wgpu shell did not read it at all until ticket 26/09/09/PROCEDURAL-3D-END-TO-END, so
 * edit mode's Preview window never got a rect, an arena subtree or a World3d engine surface.
 *
 * ⚖️ Every row runs twice: through the shipped modules and through an independent in-file oracle
 * that re-derives the same answer from the fixture's own declarations. */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "vitest";
import { EMPTY_APP_LABELS_OVERLAY, resolveLayoutForMode } from "@semio-tech/framework";
// 🔗️ `🐚️Shell` and `🛠️ShellHelpers` are mutually recursive modules (Shell calls `shellLabel` at its own
// module scope), so entering the pair through ShellHelpers leaves Shell reading a half-initialised
// binding. Every production entry reaches them through `🏛️ShellHost` → `🐚️Shell`, and so does this
// suite — the side-effect import is the evaluation order, not a dependency.
import "../../🧱️elements/🐚️Shell/🟦️.tsx";
import { resolveFrameworkLayoutSeed } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";

//#region 🧫️Fixture
type FixtureRect = { readonly x: number; readonly y: number; readonly w: number; readonly h: number };
type FixtureWindowNode = { readonly kind: "window"; readonly windowKindId: string; readonly instanceId?: string; readonly title?: string };
type FixtureStackNode = { readonly kind: "stack"; readonly size?: number; readonly activeWindowKindId?: string; readonly children: readonly FixtureWindowNode[] };
type FixtureAxisNode = { readonly kind: "row" | "column"; readonly size?: number; readonly children: readonly (FixtureAxisNode | FixtureStackNode)[] };
type FixtureLayout = { readonly root: FixtureAxisNode | FixtureStackNode };
type FixtureCase = {
  readonly id: string;
  readonly activeModeId: string;
  readonly app: {
    readonly defaultModeId: string;
    readonly modes: readonly { readonly id: string; readonly layoutId?: string }[];
    readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
    readonly defaultLayout?: FixtureLayout;
    readonly namedLayouts: readonly { readonly id: string; readonly layout: FixtureLayout }[];
  };
  readonly expected: {
    readonly resolvedLayoutId: string | null;
    readonly axis: string;
    readonly windowInstances: readonly { readonly id: string; readonly windowKindId: string }[];
    readonly activeWindowId: string;
    readonly extraInstances?: readonly { readonly id: string; readonly windowKindId: string }[];
    readonly stacks: readonly { readonly path: string; readonly windows: readonly string[]; readonly active: string; readonly fraction: number; readonly rect: readonly [number, number, number, number] }[];
  };
};
type Fixture = { readonly note: string; readonly provenance: Record<string, string>; readonly canvas: FixtureRect; readonly cases: readonly FixtureCase[] };

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../🧫️fixtures/🪟️app-mode-layouts/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

const windowKindsForSeed = (kase: FixtureCase) => kase.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label }));
const EMPTY_OVERLAY = EMPTY_APP_LABELS_OVERLAY;
//#endregion 🧫️Fixture

//#region 🔮️Oracle
/** 🔮️ An independent re-derivation of the same answers straight off the fixture's declarations —
 * no shell module involved, so a shipped regression cannot move both sides together. */
function oracleStacks(node: FixtureAxisNode | FixtureStackNode, path: readonly number[], rect: FixtureRect): { path: string; windows: string[]; active: string; fraction: number; rect: [number, number, number, number] }[] {
  if (node.kind === "stack") {
    const windows = node.children.map((child) => child.instanceId ?? child.windowKindId);
    const active = windows.find((id) => id === node.activeWindowKindId) ?? node.children.find((child) => child.windowKindId === node.activeWindowKindId)?.windowKindId ?? windows[0] ?? "";
    return [{ path: path.join(","), windows, active, fraction: 1, rect: [rect.x, rect.y, rect.w, rect.h] }];
  }
  const sizes = node.children.map((child) => child.size ?? 1);
  const total = sizes.reduce((sum, size) => sum + size, 0) || 1;
  const rows: { path: string; windows: string[]; active: string; fraction: number; rect: [number, number, number, number] }[] = [];
  let cursor = node.kind === "row" ? rect.x : rect.y;
  node.children.forEach((child, index) => {
    const fraction = sizes[index]! / total;
    const childRect = node.kind === "row" ? { x: cursor, y: rect.y, w: rect.w * fraction, h: rect.h } : { x: rect.x, y: cursor, w: rect.w, h: rect.h * fraction };
    cursor += node.kind === "row" ? childRect.w : childRect.h;
    const nested = oracleStacks(child, [...path, index], childRect);
    if (child.kind === "stack") nested[0] = { ...nested[0]!, fraction };
    rows.push(...nested);
  });
  return rows;
}

function oracleLayout(kase: FixtureCase): FixtureLayout | undefined {
  const mode = kase.app.modes.find((entry) => entry.id === kase.activeModeId);
  if (mode?.layoutId) {
    const named = kase.app.namedLayouts.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return kase.app.defaultLayout;
}

const close = (actual: number, expected: number, what: string) => assert.ok(Math.abs(actual - expected) < 0.01, `${what}: ${actual} vs declared ${expected}`);
//#endregion 🔮️Oracle

//#region 🧪️Laws
describe("🪟️ app mode layouts — one contract, both renderer targets", () => {
  it("resolves the layout each mode names, named layout ahead of the default", () => {
    for (const kase of fixture.cases) {
      const resolved = resolveLayoutForMode(kase.app as never, kase.activeModeId) as FixtureLayout | undefined;
      assert.ok(resolved, `${kase.id}: a mode always resolves some layout`);
      const namedMatch = kase.app.namedLayouts.find((named) => JSON.stringify(named.layout) === JSON.stringify(resolved))?.id ?? null;
      assert.equal(namedMatch, kase.expected.resolvedLayoutId, `${kase.id}: resolved layout identity`);
      assert.equal(JSON.stringify(resolved), JSON.stringify(oracleLayout(kase)), `${kase.id}: the oracle resolves the same layout`);
      assert.equal(resolved!.root.kind, kase.expected.axis, `${kase.id}: root axis`);
    }
  });

  it("seeds every window instance the active mode's layout names, in layout order", () => {
    for (const kase of fixture.cases) {
      const layout = resolveLayoutForMode(kase.app as never, kase.activeModeId);
      const seed = resolveFrameworkLayoutSeed(layout as never, windowKindsForSeed(kase), EMPTY_OVERLAY, "default", "en");
      const seen: { id: string; windowKindId: string }[] = [];
      const walk = (node: { kind: string; children?: readonly unknown[]; id?: string }) => {
        if (node.kind === "window") {
          const id = node.id!;
          const extra = seed.extraInstances.find((entry) => entry.id === id);
          seen.push({ id, windowKindId: extra?.windowKindId ?? id });
          return;
        }
        for (const child of node.children ?? []) walk(child as { kind: string; children?: readonly unknown[]; id?: string });
      };
      walk(seed.modeLayout as never);
      assert.deepEqual(seen, kase.expected.windowInstances.map((row) => ({ ...row })), `${kase.id}: window instances, in layout order`);
      const focus = kase.expected.stacks[0]!.active;
      assert.equal(focus, kase.expected.activeWindowId, `${kase.id}: focus is the first stack's active window — the dock's own \`from_app\` rule`);
      assert.ok(seen.some((row) => row.id === kase.expected.activeWindowId), `${kase.id}: the focused window is one of the placed ones`);
      assert.deepEqual(
        seed.extraInstances.map((entry) => ({ id: entry.id, windowKindId: entry.windowKindId })),
        (kase.expected.extraInstances ?? []).map((row) => ({ ...row })),
        `${kase.id}: only a window whose instance id differs from its kind is an extra instance`,
      );
    }
  });

  it("carries each authored fraction into the seeded tree", () => {
    for (const kase of fixture.cases) {
      const layout = resolveLayoutForMode(kase.app as never, kase.activeModeId);
      const seed = resolveFrameworkLayoutSeed(layout as never, windowKindsForSeed(kase), EMPTY_OVERLAY, "default", "en");
      const root = seed.modeLayout as { kind: string; children?: readonly { size?: number }[] };
      if (root.kind === "stack") {
        assert.equal(kase.expected.stacks.length, 1, `${kase.id}: a root stack is one pane`);
        close(1, kase.expected.stacks[0]!.fraction, `${kase.id}: root stack fraction`);
        continue;
      }
      const sizes = (root.children ?? []).map((child) => child.size ?? 0);
      const total = sizes.reduce((sum, size) => sum + size, 0) || 1;
      assert.equal(sizes.length, kase.expected.stacks.length, `${kase.id}: one child per declared stack`);
      sizes.forEach((size, index) => close(size / total, kase.expected.stacks[index]!.fraction, `${kase.id}: fraction of stack ${index}`));
    }
  });

  it("solves the declared rects from the declared fractions — the geometry the wgpu dock paints", () => {
    for (const kase of fixture.cases) {
      const layout = oracleLayout(kase)!;
      const solved = oracleStacks(layout.root, [], fixture.canvas);
      assert.equal(solved.length, kase.expected.stacks.length, `${kase.id}: one solved stack per declared stack`);
      solved.forEach((stack, index) => {
        const expected = kase.expected.stacks[index]!;
        assert.equal(stack.path, expected.path, `${kase.id}: stack path`);
        assert.deepEqual(stack.windows, [...expected.windows], `${kase.id}: windows of stack ${expected.path}`);
        assert.equal(stack.active, expected.active, `${kase.id}: active window of stack ${expected.path}`);
        close(stack.fraction, expected.fraction, `${kase.id}: fraction of stack ${expected.path}`);
        stack.rect.forEach((value, axis) => close(value, expected.rect[axis]!, `${kase.id}: rect[${axis}] of stack ${expected.path}`));
      });
    }
  });

  it("keeps the fixture itself closed: fractions sum to one, rects tile the canvas, instances name declared kinds", () => {
    assert.ok(Object.keys(fixture.provenance).length > 0, "the fixture names where each authored layout comes from");
    for (const kase of fixture.cases) {
      const sum = kase.expected.stacks.reduce((total, stack) => total + stack.fraction, 0);
      close(sum, 1, `${kase.id}: declared fractions sum`);
      const area = kase.expected.stacks.reduce((total, stack) => total + stack.rect[2] * stack.rect[3], 0);
      close(area, fixture.canvas.w * fixture.canvas.h, `${kase.id}: declared rects tile the canvas`);
      const kinds = new Set(kase.app.windowKinds.map((kind) => kind.id));
      for (const instance of kase.expected.windowInstances) assert.ok(kinds.has(instance.windowKindId), `${kase.id}: instance ${instance.id} names an undeclared window kind`);
      assert.ok(kase.expected.stacks.some((stack) => stack.windows.includes(kase.expected.activeWindowId)), `${kase.id}: the focused window is in one of the declared stacks`);
    }
  });

  it("re-seeds on a mode change: generate replaces edit's windows and edit restores them", () => {
    const edit = fixture.cases.find((kase) => kase.id === "generation3d-edit-is-a-row-split")!;
    const generate = fixture.cases.find((kase) => kase.id === "generation3d-generate-is-the-named-three-pane-row")!;
    const seedFor = (modeId: string) => {
      const layout = resolveLayoutForMode(edit.app as never, modeId);
      const seed = resolveFrameworkLayoutSeed(layout as never, windowKindsForSeed(edit), EMPTY_OVERLAY, "default", "en");
      const ids: string[] = [];
      const walk = (node: { kind: string; children?: readonly unknown[]; id?: string }) => {
        if (node.kind === "window") return void ids.push(node.id!);
        for (const child of node.children ?? []) walk(child as { kind: string; children?: readonly unknown[]; id?: string });
      };
      walk(seed.modeLayout as never);
      return ids;
    };
    assert.deepEqual(seedFor("edit"), edit.expected.windowInstances.map((row) => row.id));
    assert.deepEqual(seedFor("generate"), generate.expected.windowInstances.map((row) => row.id));
    assert.deepEqual(seedFor("edit"), edit.expected.windowInstances.map((row) => row.id), "switching back restores edit's own windows");
  });
});
//#endregion 🧪️Laws
