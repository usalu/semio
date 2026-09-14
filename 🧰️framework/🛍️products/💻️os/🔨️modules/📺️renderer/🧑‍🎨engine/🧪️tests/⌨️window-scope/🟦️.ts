/** ⌨️ The two window-scope decisions as laws over ONE language-neutral fixture
 * (`🏛️ShellHost/🧫️fixtures/⌨️window-scope/🔣️.json`), shared verbatim with the wgpu shell's Rust twin
 * (`🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`).
 *
 * 🏁️ What these encode: at boot and after every mode switch the React shell left `activeWindowId` at
 * `null`, and its keybinding loop resolved a chord against the FOCUSED window kind alone. Measured on
 * `http://127.0.0.1:6018/?plugin=generation3d`: `mod+shift+g` (Add Generation) invoked NOTHING with the
 * app untouched (no active window at all), and still nothing once `procedural-main` was active (the
 * flow window does not own the verb) — while `addGeneration` is declared on the Generations window and
 * the chord is app-wide (`✏️editor/🦀️.rs`). Two layers, two laws:
 *   1. a seeded mode layout ALWAYS opens with one window active ({@link dockSeedActiveWindowIdV1});
 *   2. an app-wide chord addresses the window of the ACTIVE MODE that owns the verb, focused or not,
 *      and is a hinted no-op — never a dead key — when no mounted window owns it
 *      ({@link resolveKeybindingTargetWindowV1}).
 *
 * ⚖️ Every row runs at least twice: through the shipped module and through an independent in-file
 * oracle, with Ajv (third party) validating the fixture and rejecting hostile mutations of it. */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "vitest";
import Ajv from "ajv";
import { SHELL_KEYBINDINGS } from "@semio-tech/ui-react";
import { TOOL_RUN_STEP_CHORD } from "../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
import {
  KEYBINDING_UNOWNED_LABEL,
  chordCarriesAcceleratorV1,
  dockSeedActiveWindowIdV1,
  keybindingUnownedTextV1,
  modeLayoutStacksV1,
  reservedShellChordsV1,
  resolveKeybindingTargetWindowV1,
  type WindowScopeInstanceV1,
  type WindowScopeKindV1,
  type WindowScopeLayoutNodeV1,
  type WindowScopeStackV1,
} from "../../🧱️elements/🏛️ShellHost/⌨️window-scope/🟦️.ts";

type FixtureApp = { readonly kinds: readonly WindowScopeKindV1[]; readonly modes: Readonly<Record<string, WindowScopeLayoutNodeV1>> };
type Fixture = {
  readonly note: string;
  readonly apps: Readonly<Record<string, FixtureApp>>;
  readonly stacks: readonly { readonly id: string; readonly app: string; readonly mode: string; readonly expected: readonly WindowScopeStackV1[] }[];
  readonly dockSeed: readonly { readonly id: string; readonly app: string; readonly mode: string; readonly activeWindowId: string | null; readonly expected: string | null }[];
  readonly chords: readonly { readonly id: string; readonly app: string; readonly mode: string; readonly focusedWindowId: string | null; readonly actionId: string; readonly expectedKind: string; readonly expectedWindowId: string | null }[];
  readonly unownedHint: readonly { readonly locale: string; readonly chord: string; readonly label: string; readonly text: string }[];
  readonly reservedChords: {
    readonly note: string;
    readonly collision: { readonly shellControlId: string; readonly chord: string; readonly alsoMintedFor: string };
    readonly shellTable: Readonly<Record<string, string>>;
    readonly overrides: Readonly<Record<string, string>>;
    readonly expected: readonly string[];
    readonly cases: readonly { readonly chord: string; readonly accelerator: boolean; readonly reserved: boolean; readonly why?: string }[];
  };
};

const fixturePath = resolve(dirname(fileURLToPath(import.meta.url)), "../../🧱️elements/🏛️ShellHost/🧫️fixtures/⌨️window-scope/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

const FIXTURE_SCHEMA: Record<string, unknown> = {
  type: "object",
  additionalProperties: false,
  required: ["note", "apps", "stacks", "dockSeed", "chords", "unownedHint", "reservedChords"],
  properties: {
    note: { type: "string", minLength: 1 },
    apps: {
      type: "object",
      minProperties: 2,
      additionalProperties: {
        type: "object",
        additionalProperties: false,
        required: ["kinds", "modes"],
        properties: {
          kinds: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["id", "actionIds"], properties: { id: { type: "string", minLength: 1 }, actionIds: { type: "array", items: { type: "string", minLength: 1 } } } } },
          modes: { type: "object", minProperties: 1, additionalProperties: { $ref: "#/$defs/node" } },
        },
      },
    },
    stacks: { type: "array", minItems: 3, items: { type: "object", additionalProperties: false, required: ["id", "app", "mode", "expected"], properties: { id: { type: "string" }, app: { type: "string" }, mode: { type: "string" }, expected: { type: "array", items: { $ref: "#/$defs/stack" } } } } },
    dockSeed: { type: "array", minItems: 6, items: { type: "object", additionalProperties: false, required: ["id", "app", "mode", "activeWindowId", "expected"], properties: { id: { type: "string" }, app: { type: "string" }, mode: { type: "string" }, activeWindowId: { type: ["string", "null"] }, expected: { type: ["string", "null"] } } } },
    chords: {
      type: "array",
      minItems: 8,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "app", "mode", "focusedWindowId", "actionId", "expectedKind", "expectedWindowId"],
        properties: { id: { type: "string" }, app: { type: "string" }, mode: { type: "string" }, focusedWindowId: { type: ["string", "null"] }, actionId: { type: "string", minLength: 1 }, expectedKind: { enum: ["focused", "owner", "unowned"] }, expectedWindowId: { type: ["string", "null"] } },
      },
    },
    reservedChords: {
      type: "object",
      additionalProperties: false,
      required: ["note", "collision", "shellTable", "overrides", "expected", "cases"],
      properties: {
        note: { type: "string", minLength: 1 },
        collision: { type: "object", additionalProperties: false, required: ["shellControlId", "chord", "alsoMintedFor"], properties: { shellControlId: { type: "string" }, chord: { type: "string" }, alsoMintedFor: { type: "string" } } },
        shellTable: { type: "object", minProperties: 5, additionalProperties: { type: "string", minLength: 1 } },
        overrides: { type: "object", additionalProperties: { type: "string", minLength: 1 } },
        expected: { type: "array", minItems: 4, items: { type: "string", minLength: 1 } },
        cases: { type: "array", minItems: 8, items: { type: "object", additionalProperties: false, required: ["chord", "accelerator", "reserved"], properties: { chord: { type: "string", minLength: 1 }, accelerator: { type: "boolean" }, reserved: { type: "boolean" }, why: { type: "string" } } } },
      },
    },
    unownedHint: { type: "array", minItems: 3, items: { type: "object", additionalProperties: false, required: ["locale", "chord", "label", "text"], properties: { locale: { type: "string", minLength: 1 }, chord: { type: "string", minLength: 1 }, label: { type: "string", minLength: 1 }, text: { type: "string", minLength: 1 } } } },
  },
  $defs: {
    stack: { type: "object", additionalProperties: false, required: ["windowIds", "activeWindowId"], properties: { windowIds: { type: "array", items: { type: "string" } }, activeWindowId: { type: ["string", "null"] } } },
    node: {
      type: "object",
      required: ["kind"],
      oneOf: [
        { additionalProperties: false, required: ["kind", "id"], properties: { kind: { const: "window" }, id: { type: "string", minLength: 1 } } },
        { additionalProperties: false, required: ["kind", "children"], properties: { kind: { const: "stack" }, activeId: { type: "string" }, children: { type: "array", items: { type: "object", additionalProperties: false, required: ["kind", "id"], properties: { kind: { const: "window" }, id: { type: "string", minLength: 1 } } } } } },
        { additionalProperties: false, required: ["kind", "children"], properties: { kind: { enum: ["row", "column"] }, children: { type: "array", minItems: 1, items: { $ref: "#/$defs/node" } } } },
      ],
    },
  },
};

const appOf = (id: string): FixtureApp => fixture.apps[id]!;
const layoutOf = (app: string, mode: string): WindowScopeLayoutNodeV1 => appOf(app).modes[mode]!;
/** 🪟️ The active mode's live instances in layout order — instance id == kind id for every fixture app. */
const mountedOf = (app: string, mode: string): readonly WindowScopeInstanceV1[] =>
  modeLayoutStacksV1(layoutOf(app, mode)).flatMap((stack) => stack.windowIds.map((id) => ({ id, windowKindId: id })));

/** ⚖️ An independent walk of the same layout: collects window ids depth-first with a plain recursion
 * and picks the first stack's shown tab, written without reusing the shipped helpers. */
function oracleStacks(node: WindowScopeLayoutNodeV1): { windowIds: string[]; activeWindowId: string | null }[] {
  if (node.kind === "window") return [{ windowIds: [node.id], activeWindowId: node.id }];
  if (node.kind === "stack") {
    const windowIds = node.children.map((child) => child.id);
    return [{ windowIds, activeWindowId: node.activeId ?? windowIds[0] ?? null }];
  }
  const out: { windowIds: string[]; activeWindowId: string | null }[] = [];
  for (const child of node.children) out.push(...oracleStacks(child));
  return out;
}

function oracleSeed(node: WindowScopeLayoutNodeV1, activeWindowId: string | null): string | null {
  const stacks = oracleStacks(node);
  const all = stacks.flatMap((stack) => stack.windowIds);
  if (all.length === 0) return null;
  if (activeWindowId !== null && all.includes(activeWindowId)) return null;
  for (const stack of stacks) if (stack.activeWindowId !== null && stack.windowIds.includes(stack.activeWindowId)) return stack.activeWindowId;
  return all[0] ?? null;
}

function oracleTarget(app: string, mode: string, focusedWindowId: string | null, actionId: string): { kind: string; windowId: string | null } {
  const owners = appOf(app).kinds.filter((kind) => kind.actionIds.includes(actionId)).map((kind) => kind.id);
  const mounted = mountedOf(app, mode).map((instance) => instance.id);
  if (focusedWindowId !== null && mounted.includes(focusedWindowId) && owners.includes(focusedWindowId)) return { kind: "focused", windowId: focusedWindowId };
  for (const id of mounted) if (owners.includes(id)) return { kind: "owner", windowId: id };
  return { kind: "unowned", windowId: null };
}

export function testWindowScope(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(FIXTURE_SCHEMA);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  for (const hostile of [
    { ...fixture, extra: true },
    { ...fixture, chords: fixture.chords.map((row, index) => (index === 0 ? { ...row, expectedKind: "elsewhere" } : row)) },
    { ...fixture, dockSeed: fixture.dockSeed.slice(0, 2) },
    { ...fixture, apps: { ...fixture.apps, generation3d: { ...fixture.apps.generation3d!, modes: { edit: { kind: "row", children: [] } } } } },
  ]) {
    assert.equal(validate(hostile), false, "🧨️ a hostile fixture must be refused");
  }

  for (const row of fixture.stacks) {
    const stacks = modeLayoutStacksV1(layoutOf(row.app, row.mode));
    assert.deepEqual(stacks.map((stack) => ({ windowIds: [...stack.windowIds], activeWindowId: stack.activeWindowId ?? null })), row.expected.map((stack) => ({ windowIds: [...stack.windowIds], activeWindowId: stack.activeWindowId ?? null })), `${row.id}: stacks`);
    assert.deepEqual(oracleStacks(layoutOf(row.app, row.mode)), row.expected.map((stack) => ({ windowIds: [...stack.windowIds], activeWindowId: stack.activeWindowId ?? null })), `${row.id}: oracle`);
  }

  for (const row of fixture.dockSeed) {
    const layout = layoutOf(row.app, row.mode);
    assert.equal(dockSeedActiveWindowIdV1(modeLayoutStacksV1(layout), row.activeWindowId), row.expected, `${row.id}: seed`);
    assert.equal(oracleSeed(layout, row.activeWindowId), row.expected, `${row.id}: oracle`);
    if (row.expected !== null) {
      // 🔁️ Idempotent: re-seeding over the answer it just gave must leave the layout alone.
      assert.equal(dockSeedActiveWindowIdV1(modeLayoutStacksV1(layout), row.expected), null, `${row.id}: idempotent`);
    }
  }

  for (const row of fixture.chords) {
    const target = resolveKeybindingTargetWindowV1({ kinds: appOf(row.app).kinds, mounted: mountedOf(row.app, row.mode), focusedWindowId: row.focusedWindowId, actionId: row.actionId });
    assert.equal(target.kind, row.expectedKind, `${row.id}: kind`);
    assert.equal(target.windowId, row.expectedWindowId, `${row.id}: window`);
    assert.deepEqual(oracleTarget(row.app, row.mode, row.focusedWindowId, row.actionId), { kind: row.expectedKind, windowId: row.expectedWindowId }, `${row.id}: oracle`);
    // 🎯️ A resolved target always names a window the active mode actually mounts, and that window's
    // kind always declares the verb — otherwise the dispatch dies on the undeclared-action gate.
    if (target.windowId !== null) {
      assert.ok(mountedOf(row.app, row.mode).some((instance) => instance.id === target.windowId), `${row.id}: target is mounted`);
      assert.ok(appOf(row.app).kinds.find((kind) => kind.id === target.windowId)?.actionIds.includes(row.actionId), `${row.id}: target declares the verb`);
    }
  }

  for (const row of fixture.unownedHint) {
    assert.equal(keybindingUnownedTextV1(row.locale, row.chord, row.label), row.text, `${row.locale}: hint`);
  }
  assert.notEqual(KEYBINDING_UNOWNED_LABEL.en, KEYBINDING_UNOWNED_LABEL.de, "🇩🇪️ the two languages are authored, not copied");
  assert.equal(fixture.unownedHint.filter((row) => row.locale === "de").length, 1, "🇩🇪️ German is pinned by its own row");

  const reserved = reservedShellChordsV1(fixture.reservedChords.shellTable, fixture.reservedChords.overrides);
  assert.deepEqual([...reserved].sort(), [...fixture.reservedChords.expected].sort(), "🛡️ the reserved set");
  for (const row of fixture.reservedChords.cases) {
    assert.equal(chordCarriesAcceleratorV1(row.chord), row.accelerator, `${row.chord}: accelerator`);
    assert.equal(reserved.has(row.chord), row.reserved, `${row.chord}: reserved`);
    // ⚖️ Oracle: a chord is reserved exactly when the shell table (after overrides) names it AND it
    // carries an accelerator — restated here without the shipped helper.
    const named = new Set(Object.keys(fixture.reservedChords.shellTable).flatMap((controlId) => (fixture.reservedChords.overrides[controlId] ?? fixture.reservedChords.shellTable[controlId]!).split(",").map((key) => key.trim())));
    assert.equal(named.has(row.chord) && row.chord.split("+").slice(0, -1).some((segment) => ["mod", "ctrl", "meta", "cmd"].includes(segment)), row.reserved, `${row.chord}: oracle`);
  }
  // 🧨️ The collision is REAL, not hypothetical: the shell's own mode-step chord is the chord
  // `build_definition` mints for `toolRunStep` on every app that declares a tool run.
  assert.equal(SHELL_KEYBINDINGS[fixture.reservedChords.collision.shellControlId], fixture.reservedChords.collision.chord, "the shell table still owns the chord the fixture names");
  assert.equal(TOOL_RUN_STEP_CHORD, fixture.reservedChords.collision.chord, "the tool-run step chord still collides with it");
  assert.ok(reservedShellChordsV1(SHELL_KEYBINDINGS).has(TOOL_RUN_STEP_CHORD), "the live shell table reserves the colliding chord from the app loop");
  assert.ok(!reservedShellChordsV1(SHELL_KEYBINDINGS).has("arrowright"), "a bare arrow is never reserved — the node graph owns it");
  assert.ok(!reservedShellChordsV1(SHELL_KEYBINDINGS).has("mod+shift+g"), "an app chord the shell never names stays the app's");

  const generateOwner = fixture.chords.filter((row) => row.mode === "generate" && row.actionId === "addGeneration");
  console.log(`[DEBUG] window-scope: ${fixture.stacks.length} stack projections, ${fixture.dockSeed.length} dock seeds, ${fixture.chords.length} chord targets (${generateOwner.length} addGeneration), ${fixture.unownedHint.length} hint locales, ${fixture.reservedChords.cases.length} reserved-chord rows (${reserved.size} reserved)`);
}

describe("⌨️ window scope", () => {
  it("seeds one active window per mode layout and routes an app-wide chord to the window that owns its verb", () => {
    testWindowScope();
  });
});
