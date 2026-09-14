// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizHexbin } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🐝️spatial-hexbin/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 📍️ `demo-points-dense` of semio-viz-spatial, the sample the base case bins. */
const POINTS: [number, number][] = [
  [6, 8], [14, 11], [22, 7], [30, 14], [38, 9], [46, 16], [54, 10], [62, 18],
  [9, 22], [17, 28], [25, 24], [33, 31], [41, 26], [49, 33], [57, 27], [65, 35],
  [11, 38], [19, 44], [27, 40], [35, 47], [43, 42], [51, 49], [59, 43], [67, 51],
];

/** 🔮️ The oracle of `🐝️spatial-hexbin`, reused so both subjects meet the same lattice. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`spatial-hexbin declares no oracle for ${id}`);
  return handler;
}

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function grid(value: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** DECIMALS) / 10 ** DECIMALS;
}

/** 🍯️ One flattened `centreX, centreY, count` triple per bin, sorted by centre. */
function bins(radius: number): number[] {
  const binned = vizHexbin(POINTS, { radius, x: (point) => point[0], y: (point) => point[1] }).map((bin) => [grid(bin.x), grid(bin.y), bin.values.length] as [number, number, number]);
  binned.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return binned.flat();
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "hexagonal-binning": {
      oracle: oracle("hexagonal-binning"),
      /** 🎯️ The twin's hexagonal lattice at the same radii, sorted the same way. */
      subject: (ctx: AdapterContext) => ({ projection: { "spatial/hexbin": rows(ctx).flatMap((row) => bins(Number(row.radius))) } }),
    },
  },
});
// #endregion 🧭️Adapter
