/** 📐️ Unit suite of the pets trigonometry: exact landmarks, symmetries and periods, accuracy against numpy's committed answers and against gl-matrix rotations, the scalar blends, and the ban on platform transcendentals.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/📐️turn-trigonometry/🔣️.json — numpy's answers (case 📐️turn-trigonometry)
 * @see https://glmatrix.net/docs/module-mat2d.html — `mat2d.fromRotation`, the JavaScript oracle
 */
import { mat2d } from "gl-matrix";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { clamp, cosTurns, lerp, sinTurns, smoothstep } from "../../🟦️.ts";

type Vectors = {
  readonly angles: readonly { readonly id: string; readonly numerator: number; readonly denominator: number; readonly expected: { readonly sine: number; readonly cosine: number }; readonly bits: { readonly sine: string; readonly cosine: string } }[];
  readonly sweeps: readonly { readonly id: string; readonly denominator: number; readonly first: number; readonly last: number; readonly expected: { readonly sines: readonly number[]; readonly cosines: readonly number[] } }[];
  readonly clamps: readonly { readonly id: string; readonly value: number; readonly low: number; readonly high: number; readonly expected: number }[];
  readonly lerps: readonly { readonly id: string; readonly from: number; readonly to: number; readonly amount: number; readonly expected: number }[];
  readonly smoothsteps: readonly { readonly id: string; readonly amount: number; readonly expected: number }[];
};

const VECTORS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/📐️turn-trigonometry/🔣️.json", import.meta.url), "utf8")) as Vectors;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const BOUND = 1e-12;
const ULP = 2 ** -52;
const VIEW = new DataView(new ArrayBuffer(8));

/** 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🔪️ Into how many dyadic angles a turn is cut at the level of the run: a power of two, so every angle and every sum with whole turns stays exact. */
const CUTS = sampled(64, 1024, 4096);

/** 🧮️ Dyadic angles `k ÷ CUTS` over ±`turns` turns: every one is exact, and so is adding whole turns to it. */
function dyadic(turns: number): number[] {
  return Array.from({ length: 2 * turns * CUTS + 1 }, (_, index) => (index - turns * CUTS) / CUTS);
}

describe("sinTurns and cosTurns", () => {
  it("give exactly 0, 1 and −1 at quarter turns, never a negative zero", () => {
    for (let whole = -4; whole <= 4; whole++) {
      expect(Object.is(sinTurns(whole), 0)).toBe(true);
      expect(Object.is(cosTurns(whole), 1)).toBe(true);
      expect(Object.is(sinTurns(whole + 0.5), 0)).toBe(true);
      expect(Object.is(cosTurns(whole + 0.5), -1)).toBe(true);
      expect(Object.is(cosTurns(whole + 0.25), 0)).toBe(true);
      expect(Object.is(cosTurns(whole + 0.75), 0)).toBe(true);
    }
    expect(sinTurns(0.25)).toBe(1);
    expect(sinTurns(0.75)).toBe(-1);
    expect(sinTurns(-0.25)).toBe(-1);
    expect(sinTurns(-0.75)).toBe(1);
    expect(Object.is(sinTurns(-0), 0)).toBe(true);
  });

  it("agree at an eighth of a turn within one unit in the last place of √½", () => {
    for (const eighth of [1, 3, 5, 7, -1, -3, -5, -7]) {
      expect(Math.abs(Math.abs(sinTurns(eighth / 8)) - Math.SQRT1_2)).toBeLessThanOrEqual(ULP);
      expect(Math.abs(Math.abs(cosTurns(eighth / 8)) - Math.SQRT1_2)).toBeLessThanOrEqual(ULP);
    }
  });

  it("are odd and even exactly", () => {
    for (const turns of [...dyadic(2), 0.1, 0.3, 1 / 3, 1 / 7, 20 / 360, 0.999, 5e-324, 1e-20, 123456.789, 2 ** 52 + 1]) {
      expect(sinTurns(-turns) + sinTurns(turns)).toBe(0);
      expect(cosTurns(-turns)).toBe(cosTurns(turns));
    }
  });

  it("repeat exactly after whole turns of a non-negative angle", () => {
    for (const turns of dyadic(1).filter((angle) => angle >= 0)) {
      for (const whole of [1, 2, 7, 1024, 2 ** 30]) {
        expect(sinTurns(turns + whole)).toBe(sinTurns(turns));
        expect(cosTurns(turns + whole)).toBe(cosTurns(turns));
      }
    }
  });

  it("repeat within two units in the last place across the sign change", () => {
    for (const turns of dyadic(1)) {
      for (const whole of [-3, -1, 1, 3]) {
        expect(Math.abs(sinTurns(turns + whole) - sinTurns(turns))).toBeLessThanOrEqual(2 * ULP);
        expect(Math.abs(cosTurns(turns + whole) - cosTurns(turns))).toBeLessThanOrEqual(2 * ULP);
      }
    }
  });

  it("stay on the unit circle", () => {
    let worst = 0;
    for (const turns of [...dyadic(2), 0.1, 1 / 3, 1 / 7, 0.123456789, 7.654321]) worst = Math.max(worst, Math.abs(sinTurns(turns) * sinTurns(turns) + cosTurns(turns) * cosTurns(turns) - 1));
    expect(worst).toBeLessThanOrEqual(3 * ULP);
  });

  it("stay within [−1, 1] and finite for every finite angle", () => {
    for (const turns of [5e-324, 1e-300, 2 ** -27, 1e15 + 0.125, 2 ** 52 + 0.5, 2 ** 53, 1e300, Number.MAX_VALUE]) {
      for (const angle of [turns, -turns]) {
        expect(Math.abs(sinTurns(angle))).toBeLessThanOrEqual(1);
        expect(Math.abs(cosTurns(angle))).toBeLessThanOrEqual(1);
      }
    }
    expect(sinTurns(5e-324)).toBeGreaterThan(0);
    expect(cosTurns(5e-324)).toBe(1);
  });

  it("match numpy's sine and cosine of every committed angle", () => {
    expect(VECTORS.angles.length).toBeGreaterThan(100);
    for (const vector of VECTORS.angles) {
      const turns = vector.numerator / vector.denominator;
      const slack = Math.abs(turns) <= 8 ? 1e-14 : 1e-9;
      expect(Math.abs(sinTurns(turns) - vector.expected.sine), vector.id).toBeLessThanOrEqual(slack);
      expect(Math.abs(cosTurns(turns) - vector.expected.cosine), vector.id).toBeLessThanOrEqual(slack);
    }
  });

  it("reproduce the committed 64-bit pattern of every angle", () => {
    for (const vector of VECTORS.angles) {
      expect(bits(sinTurns(vector.numerator / vector.denominator)), vector.id).toBe(vector.bits.sine);
      expect(bits(cosTurns(vector.numerator / vector.denominator)), vector.id).toBe(vector.bits.cosine);
    }
  });

  it("match numpy along every committed sweep within the error bound", () => {
    let worst = 0;
    for (const vector of VECTORS.sweeps) {
      expect(vector.expected.sines.length).toBe(vector.last - vector.first + 1);
      for (let step = vector.first; step <= vector.last; step++) {
        worst = Math.max(worst, Math.abs(sinTurns(step / vector.denominator) - vector.expected.sines[step - vector.first]!));
        worst = Math.max(worst, Math.abs(cosTurns(step / vector.denominator) - vector.expected.cosines[step - vector.first]!));
      }
    }
    expect(worst).toBeLessThanOrEqual(BOUND);
  });

  it("match the rotation gl-matrix builds for 65 537 angles over ±8 turns within the error bound", () => {
    const rotation: [number, number, number, number, number, number] = [1, 0, 0, 1, 0, 0];
    let worst = 0;
    for (let step = -32768; step <= 32768; step++) {
      const turns = step / 4096 + step / 1048576;
      mat2d.fromRotation(rotation, 2 * Math.PI * turns);
      worst = Math.max(worst, Math.abs(cosTurns(turns) - rotation[0]), Math.abs(sinTurns(turns) - rotation[1]));
    }
    expect(worst).toBeLessThanOrEqual(BOUND);
  });
});

describe("clamp, lerp and smoothstep", () => {
  it("clamp raises to the low bound, then lowers to the high bound", () => {
    expect(clamp(0.5, 0, 1)).toBe(0.5);
    expect(clamp(-2, 0, 1)).toBe(0);
    expect(clamp(2, 0, 1)).toBe(1);
    expect(clamp(0, 2, -2)).toBe(-2);
    expect(clamp(-5, 2, -2)).toBe(-2);
    for (const vector of VECTORS.clamps) expect(clamp(vector.value, vector.low, vector.high), vector.id).toBe(vector.expected);
  });

  it("lerp starts at the start, moves in proportion and extrapolates", () => {
    expect(lerp(2, 10, 0)).toBe(2);
    expect(lerp(2, 10, 1)).toBe(10);
    expect(lerp(2, 10, 0.5)).toBe(6);
    expect(lerp(2, 10, -0.5)).toBe(-2);
    expect(lerp(2, 10, 1.5)).toBe(14);
    for (const vector of VECTORS.lerps) expect(Math.abs(lerp(vector.from, vector.to, vector.amount) - vector.expected), vector.id).toBeLessThanOrEqual(BOUND);
  });

  it("smoothstep is flat at both ends, symmetric about one half and rising", () => {
    expect(smoothstep(-3)).toBe(0);
    expect(smoothstep(0)).toBe(0);
    expect(smoothstep(0.5)).toBe(0.5);
    expect(smoothstep(1)).toBe(1);
    expect(smoothstep(3)).toBe(1);
    for (let step = 0; step < 256; step++) {
      expect(smoothstep(step / 256) + smoothstep(1 - step / 256)).toBe(1);
      expect(smoothstep((step + 1) / 256)).toBeGreaterThan(smoothstep(step / 256));
    }
    for (const vector of VECTORS.smoothsteps) expect(Math.abs(smoothstep(vector.amount) - vector.expected), vector.id).toBeLessThanOrEqual(BOUND);
  });
});

describe("determinism", () => {
  it("the module calls no platform transcendental, random source or clock", () => {
    const banned = ["sin", "cos", "tan", "atan2", "exp", "pow", "hypot", "log", "random"].map((name) => `Math.${name}(`).concat(["Date", "performance"]);
    for (const call of banned) expect(SOURCE.includes(call), call).toBe(false);
  });
});
