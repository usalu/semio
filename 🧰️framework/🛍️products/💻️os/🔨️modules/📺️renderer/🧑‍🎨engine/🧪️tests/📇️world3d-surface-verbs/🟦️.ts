/**
 * 📇️ The TypeScript twin of `♾️infinite/🌍️world/🧪️tests/📇️surface-verbs/🦀️.rs`. Both read the SAME
 * neutral oracle (`🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json`): the Rust law drives the
 * production predicate and the production admission gate, this one re-derives the rule from the
 * fixture's own `handleVerbs` mapping.
 *
 * The rule: a World3d surface may only mint a verb its WINDOW KIND declares, and the declaration has
 * exactly one source — the app manifest's `window_kind_action_refs`, resolved into
 * `WindowKindDefinition.actions` and republished onto the surface by
 * `ShellState::sync_world3d_declared_actions`. The gumball is an affordance of a window kind, not of
 * World3d.
 *
 * The defect both sides pin: the procedural VIEWER offered a translate gizmo and published
 * `translateSelection` into `procedural-view-preview`, which declares only `exportDocument` and the
 * camera verbs — `handle_action promise failed: window kind procedural-view-preview does not own
 * action translateSelection` on 6118 (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.2).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(suiteRoot, "../../🧱️elements/🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json");

type HandleFamily = "translate" | "rotate" | "scale";
type Case = {
  readonly name: string;
  readonly windowKindId: string;
  readonly declaredActionIds: readonly string[];
  readonly offersGumball: boolean;
  readonly offeredHandleVerbs: readonly string[];
};
type Fixture = { readonly rule: { readonly handleVerbs: Readonly<Record<HandleFamily, string>> }; readonly cases: readonly Case[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
const families: readonly HandleFamily[] = ["translate", "rotate", "scale"];

/** 📇️ Whether this surface's window kind declares a verb, i.e. whether the surface may mint it. */
const declares = (declaredActionIds: readonly string[], actionId: string): boolean => declaredActionIds.includes(actionId);

/** 🎚️ The handle verbs a surface offers, in the fixture's own handle order. */
const offeredHandleVerbs = (declaredActionIds: readonly string[]): string[] => families.map((family) => fixture.rule.handleVerbs[family]).filter((verb) => declares(declaredActionIds, verb));

/** 🎚️ Whether a transform gumball may be offered at all. */
const offersGumball = (declaredActionIds: readonly string[]): boolean => offeredHandleVerbs(declaredActionIds).length > 0;

describe("📇️ world3d surface verbs", () => {
  it("declares the handle→verb mapping both sides read", () => {
    expect(families.map((family) => fixture.rule.handleVerbs[family])).toEqual(["translateSelection", "rotateSelection", "scaleSelection"]);
    expect(fixture.cases.length).toBeGreaterThan(0);
  });

  for (const testCase of fixture.cases) {
    it(`offers only the verbs the window kind declares — ${testCase.name}`, () => {
      expect(offersGumball(testCase.declaredActionIds)).toBe(testCase.offersGumball);
      expect(offeredHandleVerbs(testCase.declaredActionIds)).toEqual([...testCase.offeredHandleVerbs]);
    });
  }

  it("is discriminating: the pre-fix shape offered every verb on every surface", () => {
    const wrongBefore = fixture.cases.filter((testCase) => testCase.offeredHandleVerbs.length < families.length);
    expect(wrongBefore.length).toBeGreaterThanOrEqual(2);
  });
});
