/** 🎯️ The portable elementary functions (`../../🧮️math/🎯️portable/🟦️.ts`) against the language-agnostic vectors in `../../🧫️fixtures/🎯️portable-math/🔣️.json`: the expected doubles are rounded from 90-digit decimal series (Python `decimal`) and every one was cross-checked against the platform libm within two ulps when the vectors were written (`repo-run-portable-vectors.py` of the ticket). The runtime's own `Math` is the second, independent reference: it must agree within one ulp on a random sweep, and the portable results must not depend on it.
 * @see ../../🧮️math/🎯️portable/🟦️.ts
 */
import { describe, expect, it } from "vitest";

import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

type Case = { fn: keyof typeof portable; args: number[]; expected: number };
const cases = fixture<{ schema: string; cases: Case[] }>("🎯️portable-math").cases;
const functions = portable as unknown as Record<string, (...args: number[]) => number>;
const buffer = new DataView(new ArrayBuffer(8));
const bits = (x: number): bigint => (buffer.setFloat64(0, x), buffer.getBigInt64(0));
const ulps = (a: number, b: number): number => Number(bits(a) - bits(b));

describe("🎯️ portable elementary functions", () => {
  it("carries the closed vector corpus", () => {
    expect(new Set(cases.map((c) => c.fn))).toEqual(new Set(["sin", "cos", "tan", "atan2", "asin", "acos", "exp", "pow", "hypot", "log", "log2", "log10", "cbrt"]));
    expect(cases.length).toBeGreaterThan(1000);
  });

  it.each(["sin", "cos", "tan", "atan2", "asin", "acos", "exp", "pow", "hypot", "log", "log2", "log10", "cbrt"])("%s returns the correctly rounded double of every vector", (fn) => {
    for (const c of cases.filter((candidate) => candidate.fn === fn)) expect(Object.is(functions[fn]!(...c.args), c.expected), `${fn}(${c.args.join(", ")}) = ${functions[fn]!(...c.args)}, expected ${c.expected}`).toBe(true);
  });

  it("agrees with the runtime within one ulp (two for hypot, which V8 computes with a larger error) on a deterministic sweep", () => {
    let state = 12345;
    const next = (): number => (state = (state * 1103515245 + 12345) % 2147483648) / 2147483648;
    for (let i = 0; i < 4000; i += 1) {
      const a = next() * 40 - 20, b = next() * 40 - 20, u = next() * 2 - 1;
      expect(Math.abs(ulps(portable.sin(a), Math.sin(a)))).toBeLessThanOrEqual(1);
      expect(Math.abs(ulps(portable.cos(a), Math.cos(a)))).toBeLessThanOrEqual(1);
      expect(Math.abs(ulps(portable.atan2(a, b), Math.atan2(a, b)))).toBeLessThanOrEqual(1);
      expect(Math.abs(ulps(portable.asin(u), Math.asin(u)))).toBeLessThanOrEqual(1);
      expect(Math.abs(ulps(portable.acos(u), Math.acos(u)))).toBeLessThanOrEqual(1);
      expect(Math.abs(ulps(portable.exp(a), Math.exp(a)))).toBeLessThanOrEqual(1);
      expect(Math.abs(ulps(portable.hypot(a, b, u), Math.hypot(a, b, u)))).toBeLessThanOrEqual(2);
    }
  });

  it("offers a fast tier of plain doubles that stays within a few ulp of the correctly rounded results", () => {
    let state = 99;
    const next = (): number => (state = (state * 1103515245 + 12345) % 2147483648) / 2147483648;
    for (let i = 0; i < 4000; i += 1) {
      const x = next() * 40 - 20, a = next() * 1000 + 0.001, base = next() * 50 + 0.01, power = next() * 6 - 3;
      expect(Math.abs(ulps(portable.fastExp(x), portable.exp(x)))).toBeLessThanOrEqual(2);
      expect(Math.abs(ulps(portable.fastLog(a), portable.log(a)))).toBeLessThanOrEqual(2);
      expect(Math.abs(ulps(portable.fastPow(base, power), portable.pow(base, power)))).toBeLessThanOrEqual(32);
    }
    expect(portable.fastPow(2, 10)).toBe(1024);
    expect(portable.fastPow(9, 0.5)).toBe(3);
    expect(portable.fastPow(-2, 3)).toBe(-8);
  });

  it("keeps the exact and special values", () => {
    expect(Object.is(portable.sin(-0), -0)).toBe(true);
    expect(portable.sin(0)).toBe(0);
    expect(portable.cos(0)).toBe(1);
    expect(portable.exp(0)).toBe(1);
    expect(portable.exp(-Infinity)).toBe(0);
    expect(portable.exp(Infinity)).toBe(Infinity);
    expect(portable.pow(2, 10)).toBe(1024);
    expect(portable.pow(-2, 3)).toBe(-8);
    expect(portable.pow(-2, 0.5)).toBeNaN();
    expect(portable.pow(5, 0)).toBe(1);
    expect(portable.asin(1)).toBe(Math.PI / 2);
    expect(portable.asin(2)).toBeNaN();
    expect(portable.acos(1)).toBe(0);
    expect(portable.acos(-1)).toBe(Math.PI);
    expect(portable.atan2(0, -1)).toBe(Math.PI);
    expect(portable.atan2(1, 0)).toBe(Math.PI / 2);
    expect(portable.atan(1)).toBe(Math.PI / 4);
    expect(portable.log10(1000)).toBe(3);
    expect(portable.log2(1024)).toBe(10);
    expect(portable.log(1)).toBe(0);
    expect(portable.cbrt(27)).toBe(3);
    expect(portable.cbrt(-8)).toBe(-2);
    expect(portable.hypot(3, 4)).toBe(5);
    expect(portable.hypot()).toBe(0);
    expect(portable.hypot(NaN, Infinity)).toBe(Infinity);
    expect(portable.hypot(1e200, 1e200)).toBe(1.4142135623730951e200);
    expect(portable.sin(NaN)).toBeNaN();
    expect(portable.sin(Infinity)).toBeNaN();
    expect(portable.tan(Math.PI / 4)).toBe(0.9999999999999999);
  });
});
