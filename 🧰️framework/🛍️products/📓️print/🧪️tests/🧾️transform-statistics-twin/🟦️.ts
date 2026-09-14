// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { extent, kernelDensity1d, mean, median, quantile, sum } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../📋️transform-statistics/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🧫️ The three groups of the shipped `demo-distribution` table, in table order. */
const GROUPS: readonly (readonly [string, readonly number[]])[] = [
  ["x", [4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9]],
  ["y", [8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4]],
  ["z", [3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8]],
];
const SAMPLE: readonly number[] = GROUPS.flatMap(([, values]) => values);

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔔️ The evenly spaced grid across the data extent the density is evaluated on. */
function positions(values: readonly number[], samples: number): number[] {
  const [low, high] = extent(values) as [number, number];
  return Array.from({ length: samples }, (_, index) => low + (index / (samples - 1)) * (high - low));
}

/** 🔮️ The oracle of `transform-statistics`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`transform-statistics declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    aggregates: {
      oracle: oracle("aggregates"),
      /** 🎯️ The twin's reductions over the three groups of the demo distribution. */
      subject: () => ({
        projection: {
          "aggregate/groups": GROUPS.map(([name]) => name),
          "aggregate/mean": grid(GROUPS.map(([, values]) => mean(values)!)),
          "aggregate/median": grid(GROUPS.map(([, values]) => median(values))),
          "aggregate/sum": grid(GROUPS.map(([, values]) => sum(values))),
          "aggregate/min": grid(GROUPS.map(([, values]) => extent(values)[0]!)),
          "aggregate/max": grid(GROUPS.map(([, values]) => extent(values)[1]!)),
          "aggregate/count": GROUPS.map(([, values]) => values.length),
        },
      }),
    },
    quantiles: {
      oracle: oracle("quantiles"),
      /** 🎯️ The twin's R-7 quantile at the default and at four custom probabilities. */
      subject: () => {
        const defaults = [0, 0.25, 0.5, 0.75, 1];
        const custom = [0.1, 0.33, 0.66, 0.9];
        return {
          projection: {
            "quantile/default/p": defaults,
            "quantile/default/value": grid(defaults.map((probability) => quantile(SAMPLE, probability))),
            "quantile/custom/p": custom,
            "quantile/custom/value": grid(custom.map((probability) => quantile(SAMPLE, probability))),
          },
        };
      },
    },
    kde: {
      oracle: oracle("kde"),
      /** 🎯️ The twin's Gaussian and Epanechnikov estimates on the same nine-point grid. */
      subject: () => {
        const points = positions(SAMPLE, 9);
        return {
          projection: {
            "kde/gaussian/x": grid(points),
            "kde/gaussian/density": grid(kernelDensity1d(SAMPLE, points, { bandwidth: 1, kernel: "gaussian" })),
            "kde/epanechnikov/density": grid(kernelDensity1d(SAMPLE, points, { bandwidth: 1.5, kernel: "epanechnikov" })),
          },
        };
      },
    },
  },
});
// #endregion 🧭️Adapter
