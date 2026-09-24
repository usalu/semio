// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { compile } from "mathjs";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "math-functions-sampling";
const FIXTURE = "shared://➗️math-functions-sampling/math-functions-sampling.tex";
const DECIMALS = 6;
const PI = Math.PI;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔮️ The oracle's evaluator: `mathjs` parses and compiles the very expression string the fixture carries. */
function fn(expression: string): (x: number) => number {
  const code = compile(expression);
  return (x: number) => Number(code.evaluate({ x }));
}

/** 📈️ Uniform sampling: n+1 points, the same partition the probe walks. */
function sample(f: (x: number) => number, from: number, to: number, n: number): number[] {
  const out: number[] = [];
  for (let index = 0; index <= n; index += 1) {
    const x = from + ((to - from) * index) / n;
    out.push(x, f(x));
  }
  return out;
}

/** ∫️ Riemann sums by rule, recomputed independently of the probe's accumulation order. */
function riemann(f: (x: number) => number, from: number, to: number, bins: number, rule: string): number {
  const h = (to - from) / bins;
  let total = 0;
  for (let index = 0; index < bins; index += 1) {
    const left = from + index * h;
    if (rule === "left") total += h * f(left);
    else if (rule === "right") total += h * f(left + h);
    else if (rule === "mid") total += h * f(left + h / 2);
    else total += (h * (f(left) + f(left + h))) / 2;
  }
  return total;
}

/** 📈️ One refinement pass: a midpoint wherever the chord sagitta exceeds `tolerance` of the y extent. */
function refine(points: readonly number[], f: (x: number) => number, tolerance: number): number[] {
  const pairs: Array<[number, number]> = [];
  for (let index = 0; index < points.length; index += 2) pairs.push([points[index]!, points[index + 1]!]);
  const ys = pairs.map(([, y]) => y);
  const extent = Math.max(1e-9, Math.max(...ys) - Math.min(...ys));
  const limit = tolerance * extent;
  const out: Array<[number, number]> = [];
  for (let index = 0; index < pairs.length - 1; index += 1) {
    const [xa, ya] = pairs[index]!;
    const [xb, yb] = pairs[index + 1]!;
    out.push([xa, ya]);
    const xm = (xa + xb) / 2;
    const ym = f(xm);
    if (Math.abs(ym - (ya + yb) / 2) > limit) out.push([xm, ym]);
  }
  out.push(pairs[pairs.length - 1]!);
  return out.flat();
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "uniform-sampling": {
      /** 🔮️ The same partition, evaluated by `mathjs` from the fixture's own expression strings. */
      oracle: () => ({
        projection: {
          "sample/sine": grid(sample(fn("sin(x)"), 0, PI, 8)),
          "sample/damped": grid(sample(fn("exp(-x)*cos(4x)"), 0, 2, 10)),
          "sample/cubic": grid(sample(fn("x^3-3x"), -2, 2, 8)),
        },
      }),
      subject,
    },
    "riemann-rules": {
      /** 🔮️ The four rules over integrand values `mathjs` supplies; the exact integrals are in the feature text. */
      oracle: () => ({
        projection: {
          "riemann/square": grid(["left", "right", "mid", "trapezoid"].map((rule) => riemann(fn("x^2"), 0, 1, 8, rule))),
          "riemann/sine": grid(["left", "right", "mid", "trapezoid"].map((rule) => riemann(fn("sin(x)"), 0, PI, 12, rule))),
          "riemann/exponential": grid(["mid", "trapezoid"].map((rule) => riemann(fn("exp(x)"), 0, 2, 16, rule))),
        },
      }),
      subject,
    },
    "adaptive-refinement": {
      /** 🔮️ The documented refinement rule, over the same starting sample `mathjs` evaluates. */
      oracle: () => {
        const f = fn("sin(4x)");
        const start = sample(f, 0, PI, 4);
        const one = refine(start, f, 0.05);
        const two = refine(one, f, 0.05);
        return {
          projection: {
            "refine/before": [start.length / 2],
            "refine/pass-one": grid(one),
            "refine/pass-two-count": [two.length / 2],
          },
        };
      },
      subject,
    },
  },
});
// #endregion 🧭️Adapter
