// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { hexbin as makeHexbin } from "d3-hexbin";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "spatial-hexbin";
const DECIMALS = 6;

/** 📍 `demo-points-dense` of semio-viz-spatial, in declaration order. */
const POINTS: [number, number][] = [
  [6, 8], [14, 11], [22, 7], [30, 14], [38, 9], [46, 16], [54, 10], [62, 18],
  [9, 22], [17, 28], [25, 24], [33, 31], [41, 26], [49, 33], [57, 27], [65, 35],
  [11, 38], [19, 44], [27, 40], [35, 47], [43, 42], [51, 49], [59, 43], [67, 51],
];

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function grid(value: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** DECIMALS) / 10 ** DECIMALS;
}

/** 🍯 One flattened `centreX, centreY, count` triple per bin, sorted by centre. */
function bins(radius: number): number[] {
  const binned = makeHexbin().radius(radius)(POINTS).map((bin) => [grid(bin.x), grid(bin.y), bin.length] as [number, number, number]);
  binned.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return binned.flat();
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "hexagonal-binning": {
      /** 🔮️ `d3-hexbin` at the same radii, sorted the same way as the probe. */
      oracle: (ctx: AdapterContext) => ({ projection: { "spatial/hexbin": rows(ctx).flatMap((row) => bins(Number(row.radius))) } }),
      /** 🎯️ `\SemioVizHexbin` over the same points and radii. */
      subject: async (ctx: AdapterContext): Promise<{ projection: ProbeProjection }> => {
        const records = await compileVizProbe(ctx.fixture("local://hexbin.tex"), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
        return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
      },
    },
  },
});
// #endregion 🧭️Adapter
