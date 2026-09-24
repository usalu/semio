// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { max, mean, median, min, quantile, sum } from "d3-array";
import { regressionExp, regressionLinear, regressionLog, regressionPoly, regressionPow } from "d3-regression";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "transform-statistics";
const FIXTURE = "shared://📋️transform-statistics/transform-statistics.tex";
const DECIMALS = 6;

/** 🧫️ The three groups of the shipped `demo-distribution` table, in table order. */
const GROUPS: readonly (readonly [string, readonly number[]])[] = [
  ["x", [4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9]],
  ["y", [8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4]],
  ["z", [3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8]],
];
const SAMPLE: readonly number[] = GROUPS.flatMap(([, values]) => values);

/** 🧫️ The point set the regression scenario fits. */
const POINTS: readonly (readonly [number, number])[] = [
  [1, 2.1],
  [2, 3.9],
  [3, 6.2],
  [4, 7.8],
  [5, 10.3],
  [6, 11.9],
];

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔔️ An independent kernel density estimate over an evenly spaced grid across the data extent. */
function density(values: readonly number[], bandwidth: number, samples: number, kernel: (u: number) => number): number[] {
  const low = Math.min(...values);
  const high = Math.max(...values);
  return grid(
    Array.from({ length: samples }, (_, index) => {
      const position = low + (index / (samples - 1)) * (high - low);
      return values.reduce((total, value) => total + kernel((position - value) / bandwidth), 0) / (values.length * bandwidth);
    }),
  );
}

const GAUSSIAN = (u: number) => Math.exp(-0.5 * u * u) / Math.sqrt(2 * Math.PI);
const EPANECHNIKOV = (u: number) => (Math.abs(u) < 1 ? 0.75 * (1 - u * u) : 0);

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

const abscissa = (point: readonly [number, number]) => point[0];
const ordinate = (point: readonly [number, number]) => point[1];
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    aggregates: {
      /** 🔮️ d3-array's reductions over the three groups of the demo distribution. */
      oracle: () => ({
        projection: {
          "aggregate/groups": GROUPS.map(([name]) => name),
          "aggregate/mean": grid(GROUPS.map(([, values]) => mean(values)!)),
          "aggregate/median": grid(GROUPS.map(([, values]) => median(values)!)),
          "aggregate/sum": grid(GROUPS.map(([, values]) => sum(values))),
          "aggregate/min": grid(GROUPS.map(([, values]) => min(values)!)),
          "aggregate/max": grid(GROUPS.map(([, values]) => max(values)!)),
          "aggregate/count": GROUPS.map(([, values]) => values.length),
        },
      }),
      /** 🎯️ The same rollups from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    quantiles: {
      /** 🔮️ d3-array's R-7 quantile at the default and at four custom probabilities. */
      oracle: () => {
        const defaults = [0, 0.25, 0.5, 0.75, 1];
        const custom = [0.1, 0.33, 0.66, 0.9];
        return {
          projection: {
            "quantile/default/p": defaults,
            "quantile/default/value": grid(defaults.map((probability) => quantile(SAMPLE, probability)!)),
            "quantile/custom/p": custom,
            "quantile/custom/value": grid(custom.map((probability) => quantile(SAMPLE, probability)!)),
          },
        };
      },
      /** 🎯️ The same quantiles from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    kde: {
      /** 🔮️ An independent implementation of the same density formula; d3 has no estimator. */
      oracle: () => {
        const low = Math.min(...SAMPLE);
        const high = Math.max(...SAMPLE);
        return {
          projection: {
            "kde/gaussian/x": grid(Array.from({ length: 9 }, (_, index) => low + (index / 8) * (high - low))),
            "kde/gaussian/density": density(SAMPLE, 1, 9, GAUSSIAN),
            "kde/epanechnikov/density": density(SAMPLE, 1.5, 9, EPANECHNIKOV),
          },
        };
      },
      /** 🎯️ The same estimates from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    regression: {
      /** 🔮️ d3-regression's five fits, renamed onto the library's constant-term-first convention. */
      oracle: () => {
        const points = POINTS.map((point) => [...point] as [number, number]);
        const linear = regressionLinear().x(abscissa).y(ordinate)(points);
        const polynomial = regressionPoly().x(abscissa).y(ordinate).order(2)(points);
        const logarithmic = regressionLog().x(abscissa).y(ordinate)(points);
        const power = regressionPow().x(abscissa).y(ordinate)(points);
        const exponential = regressionExp().x(abscissa).y(ordinate)(points);
        return {
          projection: {
            "regression/linear": grid([linear.b, linear.a]),
            "regression/poly": grid(polynomial.coefficients),
            "regression/log": grid([logarithmic.b, logarithmic.a]),
            "regression/pow": grid([power.a, power.b]),
            "regression/exp": grid([exponential.a, exponential.b]),
          },
        };
      },
      /** 🎯️ The same fits from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
