/**
 * 🪟️ The TypeScript twin of `🐚️Shell/🧪️tests/🪟️action-window-scope/🦀️.rs`. Both read the SAME neutral
 * fixture (`🧑‍🎨engine/🧫️fixtures/🪟️action-window-scope/🔣️.json`): the Rust law drives the production
 * `scope_action_to_window`, this one re-derives the rewrite AND the shell's address resolution from
 * the fixture's own declared `resolutionOrder`.
 *
 * The defect both sides pin: a retained window body's row action carried no `windowId`, so
 * `ActionAddress::window_instance_id` fell through to the focused window and the guest refused it —
 * `handle_action promise failed: window kind procedural-main does not own action addGeneration`
 * for a press on the Generations window's own `Add Generation` row
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(suiteRoot, "../../🧫️fixtures/🪟️action-window-scope/🔣️.json");

type Pair = readonly [string, string];
type Case = {
  readonly name: string;
  readonly windowId: string;
  readonly args: readonly Pair[] | null;
  readonly expectedArgs: readonly Pair[];
  readonly resolvesTo: string;
  readonly baselineResolvesTo: string;
};
type Fixture = { readonly rule: { readonly resolutionOrder: readonly string[] }; readonly cases: readonly Case[]; readonly focusedWindowId: string };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

/** 🪟️ The rule: replace any existing `windowId`, preserve the rest in order, create the object when
 * the action carried none. */
const scope = (args: readonly Pair[] | null, windowId: string): Pair[] => [...(args ?? []).filter(([key]) => key !== "windowId"), ["windowId", windowId]];

/** 🪟️ The shell's own `window_instance_id` resolution, first entry wins. */
const resolveWindow = (args: readonly Pair[] | null, focused: string): string => args?.find(([key]) => key === "windowId")?.[1] ?? focused;

describe("🪟️ action window scope", () => {
  it("declares the address resolution order the rewrite depends on", () => {
    expect(fixture.rule.resolutionOrder[0]).toBe("args.windowId");
    expect(fixture.cases.length).toBeGreaterThan(0);
  });

  for (const testCase of fixture.cases) {
    it(`binds the action to its own window — ${testCase.name}`, () => {
      const scoped = scope(testCase.args, testCase.windowId);
      expect(scoped).toEqual(testCase.expectedArgs.map((pair) => [...pair]));
      expect(scoped.filter(([key]) => key === "windowId")).toHaveLength(1);
      expect(resolveWindow(scoped, fixture.focusedWindowId)).toBe(testCase.resolvesTo);
    });

    it(`records what the unscoped action resolved to — ${testCase.name}`, () => {
      expect(resolveWindow(testCase.args, fixture.focusedWindowId)).toBe(testCase.baselineResolvesTo);
    });

    it(`is idempotent — ${testCase.name}`, () => {
      const once = scope(testCase.args, testCase.windowId);
      expect(scope(once, testCase.windowId)).toEqual(once);
    });
  }

  it("carries cases the unscoped path addressed to the wrong window", () => {
    expect(fixture.cases.filter((entry) => entry.resolvesTo !== entry.baselineResolvesTo).length).toBeGreaterThanOrEqual(3);
  });
});
