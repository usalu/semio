// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { quantileSorted } from "d3-array";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-box-violin";
const DECIMALS = 9;

/** 🧪️ The three group levels of `demo-distribution`, in the row order the table declares. */
const GROUPS = [
  [4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9],
  [8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4],
  [3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8],
] as const;

const ALL = GROUPS.flatMap((group) => [...group]);

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ Tukey's five-number summary of one group on d3-array's R-7 quantiles. */
function summary(group: readonly number[]): number[] {
  const sorted = [...group].sort((a, b) => a - b);
  const q1 = quantileSorted(sorted, 0.25)!;
  const q2 = quantileSorted(sorted, 0.5)!;
  const q3 = quantileSorted(sorted, 0.75)!;
  const iqr = q3 - q1;
  const lower = Math.min(...sorted.filter((v) => v >= q1 - 1.5 * iqr));
  const upper = Math.max(...sorted.filter((v) => v <= q3 + 1.5 * iqr));
  return [lower, q1, q2, q3, upper].map(grid);
}

/** 🔮️ The letter-value pairs of one group at the depths a boxen plot draws. */
function letters(group: readonly number[], depth: number): number[] {
  const sorted = [...group].sort((a, b) => a - b);
  return Array.from({ length: depth }, (_, k) => {
    const p = 0.5 ** (k + 2);
    return [k + 1, grid(quantileSorted(sorted, p)!), grid(quantileSorted(sorted, 1 - p)!)];
  }).flat();
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "quartiles": {
      /** 🔮️ d3-array's quantileSorted at the seven probability levels the fixture asks for. */
      oracle: () => {
        const sorted = [...ALL].sort((a, b) => a - b);
        return { projection: { quantile: [0, 0.05, 0.25, 0.5, 0.75, 0.95, 1].map((p) => grid(quantileSorted(sorted, p)!)) } };
      },
      /** 🎯️ The quantiles `\semio_viz_stat_quantile:NnN` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "quartiles.tex")),
    },
    "whiskers": {
      /** 🔮️ One Tukey summary per group level, in the level order the table declares. */
      oracle: () => ({ projection: { "geometry/box": GROUPS.flatMap((group) => summary(group)) } }),
      /** 🎯️ The `geometry/box` records the grouped box family emitted. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/box": (await probe(ctx, "whiskers.tex")).projection["geometry/box"] ?? [] } }),
    },
    "letter": {
      /** 🔮️ Three letter depths per group, each a quantile pair around the median. */
      oracle: () => ({ projection: { "geometry/letter": GROUPS.flatMap((group) => letters(group, 3)) } }),
      /** 🎯️ The `geometry/letter` records the boxen plot emitted. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/letter": (await probe(ctx, "letter.tex")).projection["geometry/letter"] ?? [] } }),
    },
    "violin": {
      /** 🔮️ The same summaries the box family reports, because a boxed violin composes it. */
      oracle: () => ({ projection: { "geometry/box": GROUPS.flatMap((group) => summary(group)) } }),
      /** 🎯️ The `geometry/box` records the boxed violin emitted. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/box": (await probe(ctx, "violin.tex")).projection["geometry/box"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
