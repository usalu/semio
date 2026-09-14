/**
 * 🖱️ The TypeScript twin of `🧊️renderer/🦀️.rs`'s `AppWheel` law. Both read the SAME neutral oracle
 * (`🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json`): the Rust law drives the production
 * accumulator, this one re-derives the rule from the fixture's own statement, so the two
 * implementations hold each other rather than one restating the other.
 *
 * The defect both sides pin: `dispatch_normalized_event`'s `Scroll { delta_y, .. }` arm dropped the
 * position that `🎮️wgpu-browser-input-wire/🔣️.json` had already proven survives both hops from the
 * DOM, and the frame applied the coalesced delta at `last_pointer_x/y` instead. The wgpu tick is
 * input-driven, so the drain normally happens on a LATER pointer event — measured on 6118 as
 * `wheel gate x=5 y=5 delta=-480 … worlds=[("procedural-preview", false)]` for four notches scrolled
 * over the preview centre (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.1).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(suiteRoot, "../../🧫️fixtures/🖱️wheel-application-point/🔣️.json");

type Event = { readonly kind: "scroll" | "pointer-move"; readonly x: number; readonly y: number; readonly deltaY?: number };
type Applied = { readonly x: number; readonly y: number; readonly delta: number } | null;
type Case = { readonly name: string; readonly events: readonly Event[]; readonly applied: Applied; readonly baselineApplied?: Applied; readonly appliedAgain?: Applied };
type Fixture = { readonly rule: { readonly statement: string }; readonly cases: readonly Case[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

/** 🖱️ The pending wheel: scrolls coalesce their delta and carry the newest scroll's own point;
 * taking it leaves none, and a zero delta is not a wheel. */
class PendingWheel {
  private delta = 0;
  private x = 0;
  private y = 0;

  accumulate(x: number, y: number, deltaY: number): void {
    this.delta += deltaY;
    this.x = x;
    this.y = y;
  }

  take(): Applied {
    const pending = { x: this.x, y: this.y, delta: this.delta };
    this.delta = 0;
    this.x = 0;
    this.y = 0;
    return pending.delta === 0 ? null : pending;
  }
}

/** 🖱️ One frame's worth of input, in order: the pending wheel plus where the pointer ended up. */
const drive = (events: readonly Event[]): { readonly wheel: PendingWheel; readonly pointer: readonly [number, number] } => {
  const wheel = new PendingWheel();
  let pointer: [number, number] = [0, 0];
  for (const event of events) {
    if (event.kind === "scroll") wheel.accumulate(event.x, event.y, event.deltaY ?? 0);
    pointer = [event.x, event.y];
  }
  return { wheel, pointer };
};

describe("🖱️ wheel application point", () => {
  it("declares the rule its cases are read against", () => {
    expect(fixture.rule.statement).toContain("newest scroll's own point");
    expect(fixture.cases.length).toBeGreaterThan(0);
  });

  for (const testCase of fixture.cases) {
    it(`applies the wheel where it was scrolled — ${testCase.name}`, () => {
      const { wheel, pointer } = drive(testCase.events);
      const applied = wheel.take();
      expect(applied).toEqual(testCase.applied);

      // 🔍️ The pre-fix shape: the same delta at the LAST POINTER position.
      if (testCase.baselineApplied) {
        const preFix = applied === null ? null : { x: pointer[0], y: pointer[1], delta: applied.delta };
        expect(preFix).toEqual(testCase.baselineApplied);
        expect(preFix).not.toEqual(testCase.applied);
      }

      expect(wheel.take()).toEqual(testCase.appliedAgain ?? null);
    });
  }
});
