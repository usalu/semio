import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { z } from "zod";
import union from "lodash/union.js";
import deepEqual from "fast-deep-equal";
import { mergeUiDirtyScopes, resolveUiDirtyScope, uiDirtyScopeAsksForNothing, uiDirtyScopeWantsCatalogue, uiDirtyScopeWantsPanelBody, uiDirtyScopeWantsSection, uiDirtyScopeWantsWindowBody, type UiDirtyScope, type UiDirtySection } from "../../🟦️.ts";

/** 🐢️ TypeScript twin of `🧪️tests/🐢️ui-dirty-scope/🦀️.rs`, driven from the SAME fixture
 * (`🧫️fixtures/🐢️ui-dirty-scope/🔣️.json`): a settle re-renders EXACTLY the surfaces its
 * `InvocationResult.uiScope` names, a scope-less settle re-renders all of them, a `none` scope opens
 * no pass at all, and a coalesced pass owes the UNION of everything asked for while it ran.
 *
 * 🐛️ The defect this pins: a shell may read the field and a shell may throw it away, and both boot,
 * paint and pass every other gate. The wgpu shell's `refresh_ui` walked every window and every panel
 * leaf unconditionally — 116 of 137 renders per converging edit answered `patched=0`, and the flow
 * window was re-minted eight times at ~525 000 intake phases each, ≈1.7 s of a 5.4 s
 * `flush-deferred` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).
 *
 * 🔍️ Independent where it counts: the wire shape of every scope in the fixture is parsed by `zod`'s
 * own discriminated union — a third-party validator that shares no line with our type — and every
 * body-key union is recomputed by `lodash`'s `union` and compared with `fast-deep-equal`, so our
 * merge rule is checked against a third-party implementation of the same set operation rather than
 * against itself.
 */

const SECTIONS: readonly UiDirtySection[] = ["utilities", "tools", "engagements", "measures", "labels"];

const scopeSchema = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("full") }).strict(),
  z.object({ kind: z.literal("none") }).strict(),
  z
    .object({
      kind: z.literal("partial"),
      windowBodies: z.array(z.string()).optional(),
      panelBodies: z.array(z.string()).optional(),
      utilities: z.boolean().optional(),
      tools: z.boolean().optional(),
      engagements: z.boolean().optional(),
      measures: z.boolean().optional(),
      labels: z.boolean().optional(),
    })
    .strict(),
]);

interface FlagsFixture {
  readonly utilities: boolean;
  readonly tools: boolean;
  readonly engagements: boolean;
  readonly measures: boolean;
  readonly labels: boolean;
}

interface SelectionFixture {
  readonly id: string;
  readonly scope: UiDirtyScope;
  readonly windowBodies: readonly string[];
  readonly panelBodies: readonly string[];
  readonly flags: FlagsFixture;
  readonly catalogue: boolean;
  readonly asksForNothing: boolean;
}

interface UnionFixture {
  readonly id: string;
  readonly first: UiDirtyScope;
  readonly second: UiDirtyScope;
  readonly union: UiDirtyScope;
}

interface ScopeFixture {
  readonly schema: string;
  readonly surfaces: { readonly windowBodies: readonly string[]; readonly panelBodies: readonly string[] };
  readonly selections: readonly SelectionFixture[];
  readonly unions: readonly UnionFixture[];
  readonly laws: readonly string[];
}

function loadFixture(): ScopeFixture {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🐢️ui-dirty-scope/🔣️.json"), "utf8")) as ScopeFixture;
}

/** 🔍️ `lodash.union` recomputes first-seen-order-no-duplicates; anything else is our bug, not the fixture's. */
function oracleUnion(first: UiDirtyScope, second: UiDirtyScope): UiDirtyScope {
  if (first.kind === "full" || second.kind === "full") return { kind: "full" };
  if (first.kind === "none") return second;
  if (second.kind === "none") return first;
  const flags = Object.fromEntries(SECTIONS.map((section) => [section, Boolean(first[section] || second[section])]));
  return { kind: "partial", windowBodies: union([...(first.windowBodies ?? [])], [...(second.windowBodies ?? [])]), panelBodies: union([...(first.panelBodies ?? [])], [...(second.panelBodies ?? [])]), ...flags } as UiDirtyScope;
}

export function testUiDirtyScopeContract(): void {
  const fixture = loadFixture();
  assert.equal(fixture.schema, "semio.framework.kernel.ui-dirty-scope.v1");
  assert.equal(fixture.laws.length, 5);

  for (const selection of fixture.selections) {
    assert.deepEqual(scopeSchema.parse(selection.scope), selection.scope, `zod refused the wire shape of ${selection.id}`);
    const windows = fixture.surfaces.windowBodies.filter((body) => uiDirtyScopeWantsWindowBody(selection.scope, body));
    assert.deepEqual(windows, [...selection.windowBodies], `window bodies for ${selection.id}`);
    const panels = fixture.surfaces.panelBodies.filter((body) => uiDirtyScopeWantsPanelBody(selection.scope, body));
    assert.deepEqual(panels, [...selection.panelBodies], `panel bodies for ${selection.id}`);
    for (const section of SECTIONS) assert.equal(uiDirtyScopeWantsSection(selection.scope, section), selection.flags[section], `${section} for ${selection.id}`);
    assert.equal(uiDirtyScopeWantsCatalogue(selection.scope), selection.catalogue, `catalogue for ${selection.id}`);
    assert.equal(uiDirtyScopeAsksForNothing(selection.scope), selection.asksForNothing, `asks-for-nothing for ${selection.id}`);
  }

  for (const merge of fixture.unions) {
    const merged = mergeUiDirtyScopes(merge.first, merge.second);
    assert.deepEqual(scopeSchema.parse(merged).kind, merge.union.kind, `union kind for ${merge.id}`);
    if (merged.kind === "partial" && merge.union.kind === "partial") {
      assert.deepEqual([...(merged.windowBodies ?? [])], [...(merge.union.windowBodies ?? [])], `union window bodies for ${merge.id}`);
      assert.deepEqual([...(merged.panelBodies ?? [])], [...(merge.union.panelBodies ?? [])], `union panel bodies for ${merge.id}`);
      for (const section of SECTIONS) assert.equal(Boolean(merged[section]), Boolean(merge.union[section]), `union ${section} for ${merge.id}`);
    } else {
      assert.deepEqual(merged, merge.union, `union for ${merge.id}`);
    }
    const oracle = oracleUnion(merge.first, merge.second);
    assert.ok(deepEqual(normalize(merged), normalize(oracle)), `lodash disagreed with our union for ${merge.id}: ${JSON.stringify(merged)} vs ${JSON.stringify(oracle)}`);
  }

  assert.deepEqual(resolveUiDirtyScope(undefined), { kind: "full" });
  assert.equal(uiDirtyScopeAsksForNothing(resolveUiDirtyScope(undefined)), false);
  console.log(`[DEBUG] ui-dirty-scope contract: ${fixture.selections.length} selections, ${fixture.unions.length} unions, ${fixture.laws.length} laws`);
}

/** 🧮️ One canonical shape for comparison — an omitted optional and an explicit `false`/`[]` are the
 * same scope, and the Rust twin's `skip_serializing_if` emits the omitted form. */
function normalize(scope: UiDirtyScope): Record<string, unknown> {
  if (scope.kind !== "partial") return { kind: scope.kind };
  return { kind: "partial", windowBodies: [...(scope.windowBodies ?? [])], panelBodies: [...(scope.panelBodies ?? [])], ...Object.fromEntries(SECTIONS.map((section) => [section, Boolean(scope[section])])) };
}
