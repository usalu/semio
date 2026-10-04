/** 👆️ Unit suite of the gesture module: the press machine edge by edge, the heat of attention against its closed form, circling, stroking and shaking against drawn shapes and their mirrors, the committed vectors the Python oracle admitted, and ordinary travel that must set nothing off.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/👆️gesture-recognition/🔣️.json — the committed vectors (case 👆️gesture-recognition: numpy's winding, scipy's peaks)
 * @see ../../../../🧪️tests/👆️gesture-recognition/🟦️.ts — the replay of the vectors this suite shares with the case
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { type Point, type Press, type Rect, TIERS, type Warmth } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { CONSTANTS, type Sighted, type Vectors, boxVariant, held, hovered, pressed, trailOf, travelled, variantOf, warmed } from "../../../../🧪️tests/👆️gesture-recognition/🟦️.ts";
import {
  CIRCLE_FAST,
  CIRCLE_MARGIN,
  CIRCLE_REACH,
  CIRCLE_REST,
  COLD,
  ENOUGH_TICKS,
  type Guards,
  HEAT_ENOUGH,
  HEAT_FORGIVEN,
  HEAT_HOLD,
  HOLD_TICKS,
  IDLE,
  type PressInput,
  type PressSignal,
  SCROLL_TICKS,
  SHAKE_REST,
  SLOP_COARSE,
  SLOP_FINE,
  STROKE_PAUSE,
  UNGUARDED,
  circleStep,
  heatAfter,
  heatAt,
  hoverBusy,
  hoverStep,
  noCircling,
  noHover,
  noShaking,
  noStroking,
  pressDue,
  pressStep,
  shakeStep,
  slopOf,
  strokeStep,
  tierOf,
  warmthAfter,
} from "../../🟦️.ts";

const VECTORS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/👆️gesture-recognition/🔣️.json", import.meta.url), "utf8")) as Vectors;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const BODY: Rect = { x: 380, y: 276, width: 40, height: 48 };
const CENTRE: Point = { x: 400, y: 300 };

/** 🎲️ How many random shapes each law of the path gestures is checked on at the level of the run. */
const SHAPES = sampled(12, 120, 1200);

/** 🧭️ In how many of their sixteen variants the committed stretches of ordinary travel pass every body at the level of the run: the sample in one at the fundamental level, everything in four, then in all sixteen. */
const TRAVEL = sampled({ sample: true, variants: 1 }, { sample: false, variants: 4 }, { sample: false, variants: 16 });

/** 🎰️ A deterministic stream of numbers in `[0, 1)` (a 32-bit linear congruential generator). */
function stream(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state / 4294967296;
  };
}

/** 🔒️ The guards with some of them up. */
function up(...names: (keyof Guards)[]): Guards {
  return names.reduce<Guards>((guards, name) => ({ ...guards, [name]: true }), UNGUARDED);
}

/** 🕹️ The signals of a sequence of press inputs, one per tick from tick 1, each tick passing first. */
function signalsOf(inputs: readonly (PressInput | null)[], guards: readonly Guards[] = []): [number, PressSignal][] {
  const signals: [number, PressSignal][] = [];
  let press: Press = IDLE;
  inputs.forEach((input, index) => {
    const tick = index + 1;
    const passed = pressStep(press, { kind: "ticked" }, tick, UNGUARDED);
    press = passed.state;
    if (passed.signal !== null) signals.push([tick, passed.signal]);
    if (input === null) return;
    const step = pressStep(press, input, tick, guards[index] ?? UNGUARDED);
    press = step.state;
    if (step.signal !== null) signals.push([tick, step.signal]);
  });
  return signals;
}

/** ⏱️ A list of press inputs with the given ones at their ticks and nothing at the others. */
function at(ticks: number, events: Record<number, PressInput>): (PressInput | null)[] {
  return Array.from({ length: ticks }, (_, index) => events[index + 1] ?? null);
}

/** ⭕️ The pointer of every tick on a circle round `centre`: `turns` per second, clockwise on screen for `way` = 1, starting at the angle `start` (in turns), for `ticks` ticks; `radius` may change along the way. */
function circling(centre: Point, radius: (tick: number) => number, turns: number, way: number, start: number, ticks: number): Point[] {
  return Array.from({ length: ticks }, (_, tick) => {
    const angle = 2 * Math.PI * (start + (way * turns * tick) / 64);
    return { x: centre.x + radius(tick) * Math.cos(angle), y: centre.y + radius(tick) * Math.sin(angle) };
  });
}

/** 🐈️ The pointer of every tick going left and right over `centre`: `swing` pixels to either side, `hertz` times a second, `rise` pixels up and down with each stroke. */
function petting(centre: Point, swing: number, hertz: number, ticks: number, rise = 0, phase = 0): Point[] {
  return Array.from({ length: ticks }, (_, tick) => {
    const wave = Math.sin(2 * Math.PI * (phase + (hertz * tick) / 64));
    return { x: centre.x + swing * wave, y: centre.y + rise * wave };
  });
}

/** 🎬️ Every cue of the hover gestures along points past a body. */
function hoverCues(points: readonly Point[], body: Rect = BODY, guards: (tick: number) => Guards = () => UNGUARDED): Sighted[] {
  const cues: Sighted[] = [];
  let hover = noHover(0);
  points.forEach((point, tick) => {
    const step = hoverStep(hover, point, body, tick, guards(tick));
    hover = step.state;
    if (step.cue !== null) cues.push([tick, step.cue]);
  });
  return cues;
}

/** 🔁️ Every cue of circling alone along points round a body. */
function circleCues(points: readonly Point[], body: Rect = BODY, guards: (tick: number) => Guards = () => UNGUARDED): Sighted[] {
  const cues: Sighted[] = [];
  let state = noCircling(0);
  points.forEach((point, tick) => {
    const step = circleStep(state, point, body, tick, guards(tick));
    state = step.state;
    if (step.cue !== null) cues.push([tick, step.cue]);
  });
  return cues;
}

/** 🥤️ Every cue of the shake along the points of a grip. */
function shakeCues(points: readonly Point[], height = 48, guards: Guards = UNGUARDED): Sighted[] {
  const cues: Sighted[] = [];
  let shaking = noShaking(0);
  points.forEach((point, tick) => {
    const step = shakeStep(shaking, point, height, tick, guards);
    shaking = step.state;
    if (step.cue !== null) cues.push([tick, step.cue]);
  });
  return cues;
}

/** 🪞️ Points mirrored left and right about `x`. */
function mirrored(points: readonly Point[], x: number): Point[] {
  return points.map((point) => ({ x: 2 * x - point.x, y: point.y }));
}

/** 🧊️ A value frozen all the way down, so that a step that changed its input would throw. */
function frozen<Value>(value: Value): Value {
  if (typeof value === "object" && value !== null) for (const inner of Object.values(value)) frozen(inner);
  return Object.freeze(value);
}

describe("the press", () => {
  it("only arms: a press alone changes nothing, however long the stage waits for less than a hold", () => {
    const armed = pressStep(IDLE, { kind: "pressed", x: 10, y: 20, pointer: "mouse" }, 5, UNGUARDED);
    expect(armed).toEqual({ state: { phase: "armed", x: 10, y: 20, since: 5, slop: SLOP_FINE }, signal: null });
    expect(signalsOf(at(HOLD_TICKS, { 1: { kind: "pressed", x: 0, y: 0, pointer: "mouse" } }))).toEqual([]);
    expect(pressDue(armed.state)).toBe(5 + HOLD_TICKS);
    expect(pressDue(IDLE)).toBeNull();
  });

  it("is a click when released before the hold without leaving the slop, and a hold from the tick of the hold on", () => {
    const press: PressInput = { kind: "pressed", x: 100, y: 100, pointer: "mouse" };
    expect(signalsOf(at(40, { 2: press, [1 + HOLD_TICKS]: { kind: "released", x: 103, y: 104 } }))).toEqual([[1 + HOLD_TICKS, "click"]]);
    expect(signalsOf(at(40, { 2: press, [2 + HOLD_TICKS]: { kind: "released", x: 100, y: 100 } }))).toEqual([
      [2 + HOLD_TICKS, "hold"],
      [2 + HOLD_TICKS, "unhold"],
    ]);
    expect(signalsOf(at(90, { 2: press, 80: { kind: "released", x: 100, y: 100 } }))).toEqual([
      [2 + HOLD_TICKS, "hold"],
      [80, "unhold"],
    ]);
  });

  it("is a pick-up at the slop and beyond, for a mouse and a pen at six pixels and a finger at ten", () => {
    expect(slopOf("mouse")).toBe(SLOP_FINE);
    expect(slopOf("pen")).toBe(SLOP_FINE);
    expect(slopOf("touch")).toBe(SLOP_COARSE);
    for (const pointer of ["mouse", "pen", "touch"] as const) {
      const slop = slopOf(pointer);
      const press: PressInput = { kind: "pressed", x: 0, y: 0, pointer };
      expect(signalsOf(at(20, { 1: press, 3: { kind: "dragged", x: slop - 0.25, y: 0 }, 9: { kind: "released", x: 0, y: 0 } }))).toEqual([[9, "click"]]);
      expect(signalsOf(at(20, { 1: press, 3: { kind: "dragged", x: 0, y: -slop }, 4: { kind: "dragged", x: 50, y: 50 }, 9: { kind: "released", x: 60, y: 60 } }))).toEqual([
        [3, "lift"],
        [9, "drop"],
      ]);
    }
  });

  it("keeps the grip where it was pressed while the pet is lifted, and a hold that leaves the slop becomes a pick-up", () => {
    const lifted = pressStep(pressStep(IDLE, { kind: "pressed", x: 7, y: 9, pointer: "pen" }, 3, UNGUARDED).state, { kind: "dragged", x: 40, y: 9 }, 4, UNGUARDED);
    expect(lifted).toEqual({ state: { phase: "lifted", x: 7, y: 9, since: 3, slop: SLOP_FINE }, signal: "lift" });
    expect(pressStep(lifted.state, { kind: "dragged", x: 400, y: 900 }, 5, UNGUARDED)).toEqual({ state: lifted.state, signal: null });
    expect(signalsOf(at(80, { 1: { kind: "pressed", x: 0, y: 0, pointer: "mouse" }, 50: { kind: "dragged", x: 30, y: 0 }, 70: { kind: "released", x: 30, y: 0 } }))).toEqual([
      [1 + HOLD_TICKS, "hold"],
      [50, "lift"],
      [70, "drop"],
    ]);
  });

  it("is aborted by a cancellation in every open phase, and a cancellation without a press does nothing", () => {
    const press: PressInput = { kind: "pressed", x: 0, y: 0, pointer: "mouse" };
    expect(signalsOf(at(20, { 1: press, 5: { kind: "cancelled" }, 8: { kind: "released", x: 0, y: 0 } }))).toEqual([[5, "abort"]]);
    expect(signalsOf(at(60, { 1: press, 50: { kind: "cancelled" } }))).toEqual([
      [1 + HOLD_TICKS, "hold"],
      [50, "abort"],
    ]);
    expect(signalsOf(at(20, { 1: press, 2: { kind: "dragged", x: 20, y: 0 }, 9: { kind: "cancelled" } }))).toEqual([
      [2, "lift"],
      [9, "abort"],
    ]);
    expect(signalsOf(at(10, { 3: { kind: "cancelled" }, 4: { kind: "released", x: 0, y: 0 }, 5: { kind: "dragged", x: 9, y: 9 } }))).toEqual([]);
  });

  it("is not taken over a control, is silenced and aborted by a still stage, aborted by a scroll before a pick-up, and works as ever in a quiet stage", () => {
    const press: PressInput = { kind: "pressed", x: 0, y: 0, pointer: "mouse" };
    const release: PressInput = { kind: "released", x: 0, y: 0 };
    expect(signalsOf(at(20, { 1: press, 5: release }), [up("control")])).toEqual([]);
    expect(signalsOf(at(20, { 1: press, 5: release }), [up("still")])).toEqual([]);
    expect(signalsOf(at(20, { 1: press, 3: { kind: "dragged", x: 1, y: 0 }, 5: release }), [UNGUARDED, UNGUARDED, up("still")])).toEqual([[3, "abort"]]);
    expect(signalsOf(at(20, { 1: press, 3: { kind: "dragged", x: 1, y: 0 }, 5: release }), [UNGUARDED, UNGUARDED, up("scrolled")])).toEqual([[3, "abort"]]);
    expect(signalsOf(at(20, { 1: press, 2: { kind: "dragged", x: 30, y: 0 }, 4: { kind: "dragged", x: 30, y: 40 }, 6: release }), [UNGUARDED, UNGUARDED, UNGUARDED, up("scrolled"), UNGUARDED, up("scrolled")])).toEqual([
      [2, "lift"],
      [6, "drop"],
    ]);
    expect(signalsOf(at(20, { 1: press, 5: release }), [up("quiet"), UNGUARDED, UNGUARDED, UNGUARDED, up("quiet")])).toEqual([[5, "click"]]);
  });

  it("aborts an open press when the next one comes, and arms the new one", () => {
    expect(signalsOf(at(20, { 1: { kind: "pressed", x: 0, y: 0, pointer: "mouse" }, 6: { kind: "pressed", x: 50, y: 0, pointer: "touch" }, 9: { kind: "dragged", x: 59, y: 0 }, 10: { kind: "released", x: 59, y: 0 } }))).toEqual([
      [6, "abort"],
      [10, "click"],
    ]);
  });
});

describe("the heat", () => {
  it("leaks half a unit per second down to zero and never grows before the tick it was measured at", () => {
    expect(heatAt(3, 100, 100)).toBe(3);
    expect(heatAt(3, 100, 164)).toBe(2.5);
    expect(heatAt(3, 100, 100 + 64 * 6)).toBe(0);
    expect(heatAt(2, 600, 100)).toBe(2);
    expect(heatAfter(0, 0, 50, "click")).toBe(1);
    expect(heatAfter(1, 50, 50, "hold")).toBe(1 + HEAT_HOLD);
  });

  it("is answered by tiers whose edges belong to the lower tier, except enough", () => {
    expect(TIERS).toEqual(["hello", "trick", "purr", "enough"]);
    expect([0, 1, 1.0078125, 3, 3.0078125, 6.9921875, 7, 40].map(tierOf)).toEqual(["hello", "hello", "trick", "trick", "purr", "purr", "enough", "enough"]);
  });

  it("greets, plays tricks, purrs and has enough when clicked twice a second, then ignores clicks and forgives", () => {
    let warmth: Warmth = COLD;
    const answers: string[] = [];
    for (let click = 0; click < 12; click++) {
      warmth = warmthAfter(warmth, 32 * click, "click");
      answers.push(`${warmth.tier}${warmth.run}`);
    }
    expect(answers).toEqual(["hello1", "trick1", "trick2", "purr1", "purr2", "purr3", "purr4", "purr5", "enough1", "enough2", "enough3", "enough4"]);
    expect(warmth.tricks).toBe(2);
    expect(warmth.until).toBe(32 * 8 + ENOUGH_TICKS);
    const forgiven = warmthAfter(warmth, warmth.until, "click");
    expect(forgiven).toEqual({ heat: HEAT_FORGIVEN + 1, since: warmth.until, until: warmth.until, tier: "trick", run: 1, tricks: 3 });
  });

  it("purrs while held until the heat has had enough, about five seconds from cold", () => {
    let warmth: Warmth = COLD;
    let tick = 0;
    while (warmth.tier !== "enough") {
      warmth = warmthAfter(warmth, tick, "hold");
      if (warmth.tier !== "enough") expect(warmth.tier).toBe("purr");
      tick += 1;
    }
    expect(tick / 64).toBeGreaterThan(4.5);
    expect(tick / 64).toBeLessThan(5);
    expect(warmth).toMatchObject({ heat: HEAT_FORGIVEN, run: 1, tricks: 0 });
  });

  it("answers every committed history as numpy's closed form does", () => {
    for (const vector of VECTORS.warmths) expect(warmed(vector.caresses), vector.id).toEqual(vector.expected);
  });

  it("matches a running leaky bucket over random histories", () => {
    const draw = stream(7);
    for (let history = 0; history < SHAPES; history++) {
      let warmth: Warmth = COLD;
      let bucket = 0;
      let last = 0;
      let tick = 0;
      for (let caress = 0; caress < 40; caress++) {
        tick += Math.floor(draw() * 200);
        const kind = draw() < 0.3 ? "hold" : "click";
        const before = warmth;
        warmth = warmthAfter(warmth, tick, kind);
        if (tick < before.until) {
          expect(warmth.tier).toBe("enough");
          continue;
        }
        bucket = Math.max(bucket - ((tick - last) * 0.5) / 64, 0) + (kind === "hold" ? HEAT_HOLD : 1);
        last = tick;
        if (bucket >= HEAT_ENOUGH) {
          expect(warmth.tier).toBe("enough");
          bucket = HEAT_FORGIVEN;
          last = tick + ENOUGH_TICKS;
        } else expect(warmth.heat).toBe(bucket);
      }
    }
  });
});

describe("circling", () => {
  it("cues a circle for clockwise on screen — right, below, left, above — and a countercircle the other way", () => {
    const clockwise = circling(CENTRE, () => 80, 1, 1, 0, 160);
    expect(clockwise[16]!.y).toBeGreaterThan(CENTRE.y + 70);
    expect(hoverCues(clockwise).map(([, cue]) => cue)).toEqual(["circle"]);
    expect(hoverCues(circling(CENTRE, () => 80, 1, -1, 0, 160)).map(([, cue]) => cue)).toEqual(["countercircle"]);
  });

  it("waits until the pointer has crossed again the axis where the lap began, and then rests", () => {
    const cues = hoverCues(circling(CENTRE, () => 80, 1, 1, 0.1, 64 * 6));
    expect(cues[0]![0]).toBeGreaterThan(64 * 1.15);
    expect(cues[0]![0]).toBeLessThan(64 * 1.2);
    for (let index = 1; index < cues.length; index++) expect(cues[index]![0] - cues[index - 1]![0]).toBeGreaterThanOrEqual(CIRCLE_REST);
    expect(cues.length).toBeGreaterThanOrEqual(2);
  });

  it("is cued at the same tick and the other way round in the mirror of any circle", () => {
    const draw = stream(11);
    let cued = 0;
    for (let shape = 0; shape < SHAPES; shape++) {
      const radius = 50 + draw() * 80;
      const points = circling({ x: CENTRE.x + (draw() - 0.5) * 12, y: CENTRE.y + (draw() - 0.5) * 12 }, (tick) => radius * (1 + 0.1 * Math.sin(tick / 9)), 0.5 + draw() * 2.5, draw() < 0.5 ? 1 : -1, draw(), 64 * 3);
      const cues = circleCues(points);
      const mirror = circleCues(mirrored(points, CENTRE.x));
      expect(mirror.map(([tick]) => tick)).toEqual(cues.map(([tick]) => tick));
      expect(mirror.map(([, cue]) => cue)).toEqual(cues.map(([, cue]) => (cue === "circle" ? "countercircle" : "circle")));
      cued += cues.length > 0 ? 1 : 0;
    }
    expect(cued).toBe(SHAPES);
  });

  it("hears nothing inside the body's band or beyond it, too fast, too slow, or going back and forth", () => {
    const reach = Math.max(BODY.width, BODY.height);
    expect(circleCues(circling(CENTRE, () => reach / 2 + CIRCLE_MARGIN - 2, 1, 1, 0, 300))).toEqual([]);
    expect(circleCues(circling(CENTRE, () => reach * CIRCLE_REACH + 2, 1, 1, 0, 300))).toEqual([]);
    expect(circleCues(circling(CENTRE, () => 80, 64 / (4 * CIRCLE_FAST) + 0.5, 1, 0, 300))).toEqual([]);
    expect(circleCues(circling(CENTRE, () => 80, 0.3, 1, 0.05, 64 * 8))).toEqual([]);
    const sway = Array.from({ length: 64 * 10 }, (_, tick) => {
      const angle = 2 * Math.PI * 0.4 * Math.sin((2 * Math.PI * tick) / 70);
      return { x: CENTRE.x + 80 * Math.cos(angle), y: CENTRE.y + 80 * Math.sin(angle) };
    });
    expect(circleCues(sway)).toEqual([]);
  });

  it("hears nothing of a lap that is not round or does not close", () => {
    expect(circleCues(circling(CENTRE, (tick) => (tick % 32 < 16 ? 36 : 130), 1, 1, 0, 64 * 4))).toEqual([]);
    expect(circleCues(circling(CENTRE, (tick) => 50 * 1.5 ** (tick / 64), 1, 1, 0, 160))).toEqual([]);
    expect(circleCues(circling(CENTRE, (tick) => 50 * 1.3 ** (tick / 64), 1, 1, 0, 160)).length).toBeGreaterThan(0);
  });

  it("hears nothing over a control, in a quiet or still stage, and nothing for a while after a scroll", () => {
    const points = circling(CENTRE, () => 80, 1, 1, 0, 64 * 3);
    for (const guard of ["control", "quiet", "still"] as const) expect(circleCues(points, BODY, () => up(guard))).toEqual([]);
    const plain = circleCues(points)[0]![0];
    const scrolled = circleCues(points, BODY, (tick) => (tick === plain - 2 ? up("scrolled") : UNGUARDED));
    expect(scrolled[0]![0]).toBeGreaterThan(plain - 2 + SCROLL_TICKS);
    expect(circleStep(noCircling(0), CENTRE, BODY, 10, up("scrolled")).state.rest).toBe(10 + SCROLL_TICKS);
  });

  it("is heard on a device that reports only every second or fourth tick", () => {
    for (const every of [2, 4]) {
      const points = circling(CENTRE, () => 80, 1, 1, 0, 64 * 3).map((_, tick, all) => all[tick - (tick % every)]!);
      expect(circleCues(points).map(([, cue]) => cue)).toEqual(["circle"]);
    }
  });

  it("never hears a straight pass past a pet, however near, fast or slow", () => {
    const draw = stream(13);
    for (let pass = 0; pass < SHAPES * 4; pass++) {
      const angle = 2 * Math.PI * draw();
      const aside = (draw() - 0.5) * 300;
      const speed = 30 + draw() * 3000;
      const points = Array.from({ length: Math.ceil((1200 / speed) * 64) }, (_, tick) => {
        const along = -600 + (speed * tick) / 64;
        return { x: CENTRE.x + along * Math.cos(angle) - aside * Math.sin(angle), y: CENTRE.y + along * Math.sin(angle) + aside * Math.cos(angle) };
      });
      expect(hoverCues(points)).toEqual([]);
    }
  });
});

describe("stroking", () => {
  it("cues a petting once three strokes that begin at a reversal are done, and again with every three more", () => {
    const cues = hoverCues(petting(CENTRE, 14, 2, 64 * 4));
    expect(cues.length).toBe(5);
    expect(cues.every(([, cue]) => cue === "stroke")).toBe(true);
    expect(cues[0]![0]).toBeGreaterThan(48);
    expect(cues[0]![0]).toBeLessThan(64);
    for (let index = 1; index < cues.length; index++) expect(cues[index]![0] - cues[index - 1]![0]).toBe(48);
  });

  it("counts no stroke that is too short, wanders up and down, lies beside the body, or is too slow or too fast", () => {
    expect(hoverCues(petting(CENTRE, 8, 2, 64 * 4))).toEqual([]);
    expect(hoverCues(petting(CENTRE, 14, 2, 64 * 4, 16))).toEqual([]);
    expect(hoverCues(petting({ x: CENTRE.x + 21, y: CENTRE.y }, 9, 2, 64 * 4))).toEqual([]);
    expect(hoverCues(petting(CENTRE, 14, 0.25, 64 * 8))).toEqual([]);
    const wide = { x: 372, y: 272, width: 56, height: 56 };
    expect(hoverCues(petting(CENTRE, 26, 2, 64 * 4), wide).length).toBeGreaterThan(0);
    expect(hoverCues(petting(CENTRE, 26, 12, 64 * 4), wide)).toEqual([]);
  });

  it("forgets everything when the pointer leaves the zone or rests, and hears nothing under a guard", () => {
    const strokes = petting(CENTRE, 14, 2, 64 * 4);
    const left = strokes.map((point, tick) => (tick % 60 === 59 ? { x: CENTRE.x, y: CENTRE.y + 60 } : point));
    expect(hoverCues(left)).toEqual([]);
    const rests = strokes.flatMap((point, tick) => (tick % 24 === 0 ? Array.from({ length: STROKE_PAUSE + 2 }, () => point) : [point]));
    expect(hoverCues(rests)).toEqual([]);
    for (const guard of ["control", "quiet", "still"] as const) expect(hoverCues(strokes, BODY, () => up(guard))).toEqual([]);
    expect(strokeStep(noStroking(0), CENTRE, BODY, 4, up("scrolled")).state.rest).toBe(4 + SCROLL_TICKS);
  });

  it("is cued at the same ticks in the mirror of any petting", () => {
    const draw = stream(17);
    for (let shape = 0; shape < SHAPES; shape++) {
      const points = petting({ x: CENTRE.x + (draw() - 0.5) * 8, y: CENTRE.y + (draw() - 0.5) * 30 }, 12 + draw() * 12, 1.5 + draw() * 2.5, 64 * 3, (draw() - 0.5) * 6, draw());
      const cues = hoverCues(points);
      expect(cues.length).toBeGreaterThan(0);
      expect(hoverCues(mirrored(points, CENTRE.x))).toEqual(cues);
    }
  });
});

describe("shaking", () => {
  it("cues a shake after four counted reversals within a second, then rests", () => {
    const shake = petting(CENTRE, 30, 3, 64 * 4);
    const cues = shakeCues(shake);
    expect(cues.map(([, cue]) => cue)).toEqual(["shake", "shake"]);
    expect(cues[0]![0]).toBeLessThan(64);
    expect(cues[1]![0] - cues[0]![0]).toBeGreaterThanOrEqual(SHAKE_REST);
  });

  it("hears no swing that is too short or too slow, no sway, no carry, and nothing on a still stage", () => {
    expect(shakeCues(petting(CENTRE, 12, 4, 64 * 3))).toEqual([]);
    expect(shakeCues(petting(CENTRE, 30, 1.2, 64 * 4))).toEqual([]);
    expect(shakeCues(Array.from({ length: 64 * 3 }, (_, tick) => ({ x: CENTRE.x + tick * 4, y: CENTRE.y - tick })))).toEqual([]);
    expect(shakeCues(petting(CENTRE, 30, 3, 64 * 4), 48, up("still"))).toEqual([]);
    for (const guard of ["control", "quiet", "scrolled"] as const) expect(shakeCues(petting(CENTRE, 30, 3, 64 * 4), 48, up(guard))).toEqual(shakeCues(petting(CENTRE, 30, 3, 64 * 4)));
  });

  it("is cued at the same ticks in any direction and in the mirror", () => {
    const draw = stream(19);
    for (let shape = 0; shape < SHAPES; shape++) {
      const angle = 2 * Math.PI * draw();
      const swing = 25 + draw() * 40;
      const hertz = 2.5 + draw() * 2;
      const points = Array.from({ length: 64 * 2 }, (_, tick) => {
        const wave = swing * Math.sin((2 * Math.PI * hertz * tick) / 64);
        return { x: CENTRE.x + wave * Math.cos(angle), y: CENTRE.y + wave * Math.sin(angle) };
      });
      const cues = shakeCues(points);
      expect(cues.length).toBeGreaterThan(0);
      expect(shakeCues(mirrored(points, CENTRE.x))).toEqual(cues);
    }
  });
});

describe("one gesture at a time", () => {
  it("lets a circle silence petting for its rest, and says when a gesture is under way", () => {
    let hover = noHover(0);
    const points = circling(CENTRE, () => 80, 1, 1, 0, 160);
    let busy = false;
    let circled = -1;
    points.forEach((point, tick) => {
      const step = hoverStep(hover, point, BODY, tick, UNGUARDED);
      hover = step.state;
      busy ||= hoverBusy(hover);
      if (step.cue !== null) circled = tick;
    });
    expect(busy).toBe(true);
    expect(circled).toBeGreaterThan(0);
    expect(hover.stroking.rest).toBe(circled + CIRCLE_REST);
    expect(hoverBusy(noHover(0))).toBe(false);
  });

  it("changes none of its inputs", () => {
    const body = frozen({ ...BODY });
    let hover = frozen(noHover(0));
    let shaking = frozen(noShaking(0));
    let press = frozen(IDLE);
    let warmth = frozen(COLD);
    circling(CENTRE, () => 80, 1, 1, 0, 160).forEach((point, tick) => {
      hover = frozen(hoverStep(hover, frozen(point), body, tick, frozen({ ...UNGUARDED })).state);
      shaking = frozen(shakeStep(shaking, frozen(point), 48, tick, UNGUARDED).state);
      press = frozen(pressStep(press, frozen({ kind: "dragged", x: point.x, y: point.y }), tick, UNGUARDED).state);
      warmth = frozen(warmthAfter(warmth, tick, "hold"));
    });
    expect(hover.circling.rest).toBeGreaterThan(0);
  });
});

describe("the committed vectors", () => {
  it("states the constants the vectors were generated with", () => {
    expect(VECTORS.constants).toEqual(CONSTANTS);
  });

  it("answers every committed press, circle, petting and held path as committed", () => {
    for (const vector of VECTORS.presses) expect(pressed(vector, VECTORS.quantum), vector.id).toEqual(vector.expected);
    for (const vector of [...VECTORS.circles, ...VECTORS.strokes]) {
      const guards = vector.guard === undefined ? UNGUARDED : up(vector.guard);
      expect(hovered(trailOf(vector.path, VECTORS.hold), VECTORS.quantum, vector.body, guards), vector.id).toEqual(vector.expected);
    }
    for (const vector of VECTORS.shakes) expect(held(trailOf(vector.path, VECTORS.hold), VECTORS.quantum, vector.height, vector.guard === undefined ? UNGUARDED : up(vector.guard)), vector.id).toEqual(vector.expected);
  });

  it("recognises every clean gesture and the deliberate ones at their floors", () => {
    for (const kind of ["circles", "strokes", "shakes"] as const) {
      const deliberate = VECTORS[kind].filter((vector) => vector.guard === undefined && vector.label !== "carry");
      expect(deliberate.filter((vector) => vector.robust).every((vector) => vector.expected.length > 0), kind).toBe(true);
      expect(deliberate.filter((vector) => vector.expected.length > 0).length / deliberate.length, kind).toBeGreaterThanOrEqual(VECTORS.floors[kind]);
    }
    expect(VECTORS.shakes.filter((vector) => vector.label === "carry").every((vector) => vector.expected.length === 0)).toBe(true);
  });

  it("sets nothing off with ordinary travel past forty pets", () => {
    const watched = travelled({ ...VECTORS, variants: TRAVEL.variants }, TRAVEL.sample);
    expect(watched.cues).toEqual([]);
    expect(watched.pointerTicks).toBeGreaterThan(64 * 60 * 3);
  });

  it("mirrors, transposes and reverses a trail exactly, twice the same variant giving the trail back", () => {
    const trail = trailOf(VECTORS.travels[0]!.path, VECTORS.hold);
    for (const variant of [1, 2, 3, 8, 9]) {
      const twice = variantOf(variantOf(trail, VECTORS.page, variant), VECTORS.page, variant);
      expect(twice).toEqual(trail);
      const box = VECTORS.bodies[0]!;
      expect(boxVariant(boxVariant(box, VECTORS.page, variant), VECTORS.page, variant)).toEqual(box);
    }
  });
});

describe("determinism", () => {
  it("uses nothing but exact operations and has no side effect", () => {
    const used = [...SOURCE.matchAll(/Math\.(\w+)/g)].map((match) => match[1]!);
    expect([...new Set(used)].sort()).toEqual(["abs", "max", "min"]);
    expect(SOURCE).not.toMatch(/\b(Date|performance|console|globalThis|process)\b/);
    expect(SOURCE).not.toMatch(/\S \*\* \S/);
  });
});
