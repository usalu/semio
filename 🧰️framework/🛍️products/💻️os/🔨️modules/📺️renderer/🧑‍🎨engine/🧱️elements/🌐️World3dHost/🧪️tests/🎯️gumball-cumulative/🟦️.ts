// #region 🧲️Header
/** 🎯️ The cumulative World3d gumball protocol (`🧫️fixtures/🛠️gumball-cumulative-protocol.json`): every case replays through the pure
 * `worldGumballStep` with a selection record that asks for `gumballCumulative`, each step sending exactly the one dispatch
 * the corpus names. Third-party checks: `three` re-derives the net rotation from the dispatched axis and angle, `gl-matrix`
 * the net offset from the grab pose, so a dropped stream tick provably costs the committed motion nothing. The fem 3d guest
 * replays the same corpus natively (`🎮️commands/🧭️gumball`). */
// #endregion 🧲️Header

import { vec3 } from "gl-matrix";
import { Quaternion } from "three";
import { describe, expect, it } from "vitest";
import corpus from "../🧫️fixtures/🛠️gumball-cumulative-protocol.json" with { type: "json" };
import { type WorldGumballDispatch, type WorldGumballEvent, WORLD_GUMBALL_IDLE, worldGumballStep } from "../../🧮️gumball-math/🟦️.ts";

type Pose = { readonly position: readonly [number, number, number]; readonly quaternion: readonly [number, number, number, number]; readonly scale: readonly [number, number, number] };
type Step = { readonly event: Record<string, unknown>; readonly dispatch: WorldGumballDispatch | null };
type Case = { readonly name: string; readonly live: boolean; readonly cumulative: boolean; readonly transformMode: string; readonly targets: { readonly mode: string; readonly ids: readonly string[] }; readonly steps: readonly Step[] };
const cases = corpus.cases as unknown as readonly Case[];
const TOLERANCE = 1e-9;

function close(actual: unknown, expected: unknown, path: string): void {
  if (typeof expected === "number") return expect(actual as number, path).toBeCloseTo(expected, 9);
  if (expected === null || typeof expected !== "object") return expect(actual, path).toEqual(expected);
  for (const key of Object.keys(expected)) close((actual as Record<string, unknown>)[key], (expected as Record<string, unknown>)[key], `${path}.${key}`);
  expect(Object.keys(actual as object).sort(), path).toEqual(Object.keys(expected).sort());
}

function replay(entry: Case): readonly WorldGumballDispatch[] {
  let gesture = WORLD_GUMBALL_IDLE;
  const sent: WorldGumballDispatch[] = [];
  entry.steps.forEach((step, index) => {
    const event = (step.event.kind === "start" ? { ...step.event, live: entry.live, cumulative: entry.cumulative, transformMode: entry.transformMode, targets: entry.targets } : step.event.kind === "release" ? { ...step.event, targets: entry.targets } : step.event) as unknown as WorldGumballEvent;
    const result = worldGumballStep(gesture, event);
    gesture = result.gesture;
    if (step.dispatch === null) expect(result.dispatch, `${entry.name} step ${index}`).toBeNull();
    else {
      expect(result.dispatch, `${entry.name} step ${index}`).not.toBeNull();
      close(result.dispatch, step.dispatch, `${entry.name} step ${index}`);
      sent.push(result.dispatch as WorldGumballDispatch);
    }
  });
  return sent;
}

describe("cumulative World3d gumball protocol", () => {
  for (const entry of cases) it(entry.name, () => void replay(entry));

  it("the commit of every cumulative case is the whole motion from the grab, whichever stream ticks went out", () => {
    for (const entry of cases) {
      const start = entry.steps.find((step) => step.event.kind === "start")?.event.pose as Pose;
      const release = entry.steps.find((step) => step.event.kind === "release")?.event as { readonly after: Pose } | undefined;
      const commit = replay(entry).find((dispatch) => dispatch.args.phase === "commit");
      if (!release || !commit) continue;
      if (commit.action === "translateSelection") {
        const net = vec3.sub(vec3.create(), release.after.position as [number, number, number], start.position as [number, number, number]);
        const sent = [commit.args.dx as number, commit.args.dy as number, commit.args.dz as number];
        sent.forEach((value, axis) => expect(Math.abs(value - net[axis]!), `${entry.name} axis ${axis}`).toBeLessThan(TOLERANCE));
      } else if (commit.action === "rotateSelection") {
        const turn = new Quaternion().setFromAxisAngle({ x: commit.args.ax as number, y: commit.args.ay as number, z: commit.args.az as number } as never, commit.args.angle as number);
        const after = new Quaternion(...release.after.quaternion);
        expect(Math.abs(turn.dot(after)), entry.name).toBeGreaterThan(1 - TOLERANCE);
      } else {
        release.after.scale.forEach((value, axis) => expect(Math.abs((commit.args[["sx", "sy", "sz"][axis]!] as number) - value / start.scale[axis]!), `${entry.name} axis ${axis}`).toBeLessThan(TOLERANCE));
      }
    }
  });
});
