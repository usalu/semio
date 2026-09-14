/**
 * 🖱️ The TypeScript twin of `🧊️renderer/🦀️.rs`'s `AppWheel` law. Both read the SAME neutral oracle
 * (`🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json`): the Rust law drives the production
 * accumulator, this one re-derives the rule from the fixture's own statement, so the two
 * implementations hold each other rather than one restating the other.
 *
 * The two defects both sides pin: `dispatch_normalized_event`'s `Scroll { delta_y, .. }` arm dropped
 * the position that `🎮️wgpu-browser-input-wire/🔣️.json` had already proven survives both hops from
 * the DOM, and the frame applied the coalesced delta at `last_pointer_x/y` instead; and the
 * accumulator that fixed THAT still merged notches across points, so a wheel stream that travels
 * collapsed into one application at its newest point. The wgpu tick is input-driven, so the drain
 * normally happens on a LATER event — measured on 6118 as `wheel gate x=5 y=5 delta=-480 …
 * worlds=[("procedural-preview", false)]` and, after the first fix, as five notches
 * `{x:1208,y:461} … {x:6,y:6}` that still left `wheel=0` on all 337 world3d intents
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.1,
 * `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §2).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(suiteRoot, "../../🧫️fixtures/🖱️wheel-application-point/🔣️.json");

type Event = { readonly kind: "scroll" | "pointer-move"; readonly x: number; readonly y: number; readonly deltaY?: number };
type Application = { readonly x: number; readonly y: number; readonly delta: number };
type Case = { readonly name: string; readonly events: readonly Event[]; readonly applications: readonly Application[]; readonly baselineApplications?: readonly Application[] };
type Fixture = { readonly rule: { readonly statement: string; readonly capacity: number }; readonly cases: readonly Case[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

/** 🖱️ The pending wheel: notches coalesce only while their point does not move, a new point opens a
 * new application, the frame drains them oldest first, a zero delta is not a wheel, and a stream
 * longer than the declared credits merges into its newest application. */
class PendingWheel {
  private readonly notches: { x: number; y: number; delta: number }[] = [];

  constructor(private readonly capacity: number) {}

  accumulate(x: number, y: number, deltaY: number): void {
    const newest = this.notches.at(-1);
    if (newest && (this.notches.length >= this.capacity || (newest.x === x && newest.y === y))) {
      newest.delta += deltaY;
      newest.x = x;
      newest.y = y;
      return;
    }
    this.notches.push({ x, y, delta: deltaY });
  }

  take(): Application | null {
    while (this.notches.length > 0) {
      const oldest = this.notches.shift();
      if (oldest && oldest.delta !== 0) return { x: oldest.x, y: oldest.y, delta: oldest.delta };
    }
    return null;
  }

  pending(): boolean {
    return this.notches.some((notch) => notch.delta !== 0);
  }
}

/** 🖱️ One frame's worth of input, in order: every application the frame makes, oldest first, plus
 * where the pointer ended up. */
const drive = (events: readonly Event[], capacity: number): { readonly applications: Application[]; readonly pointer: readonly [number, number] } => {
  const wheel = new PendingWheel(capacity);
  let pointer: [number, number] = [0, 0];
  for (const event of events) {
    if (event.kind === "scroll") wheel.accumulate(event.x, event.y, event.deltaY ?? 0);
    pointer = [event.x, event.y];
  }
  const applications: Application[] = [];
  for (let application = wheel.take(); application !== null; application = wheel.take()) applications.push(application);
  expect(wheel.pending()).toBe(false);
  return { applications, pointer };
};

describe("🖱️ wheel application point", () => {
  it("declares the rule its cases are read against", () => {
    expect(fixture.rule.statement).toContain("applied at its own point");
    expect(fixture.rule.capacity).toBeGreaterThan(1);
    expect(fixture.cases.length).toBeGreaterThan(0);
  });

  for (const testCase of fixture.cases) {
    it(`applies every notch where it was scrolled — ${testCase.name}`, () => {
      const { applications, pointer } = drive(testCase.events, fixture.rule.capacity);
      expect(applications).toEqual(testCase.applications.map((application) => ({ ...application })));
      expect(applications.length).toBeLessThanOrEqual(fixture.rule.capacity);

      // 🔍️ Both pre-fix shapes in one derivation: the whole delta as ONE application, at wherever
      // the pointer ended up.
      if (testCase.baselineApplications) {
        const total = applications.reduce((sum, application) => sum + application.delta, 0);
        const preFix = [{ x: pointer[0], y: pointer[1], delta: total }];
        expect(preFix).toEqual(testCase.baselineApplications.map((application) => ({ ...application })));
        expect(preFix).not.toEqual(applications);
      }
    });
  }
});
