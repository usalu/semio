// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Delaunay } from "d3-delaunay";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "spatial-hull";

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 📍 The point set of the scenario's data table, in the order the probe loads it. */
function points(ctx: AdapterContext): [number, number][] {
  return rows(ctx).map((row) => [Number(row.x), Number(row.y)] as [number, number]);
}

/** 🔶 One-based hull indices rotated so the cycle starts at the lexicographically smallest point. */
function rotated(indices: readonly number[], pts: readonly [number, number][]): number[] {
  let start = 0;
  for (let i = 1; i < indices.length; i += 1) {
    const a = pts[indices[i]! - 1]!;
    const b = pts[indices[start]! - 1]!;
    if (a[0] < b[0] || (a[0] === b[0] && a[1] < b[1])) start = i;
  }
  return indices.slice(start).concat(indices.slice(0, start));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "convex-hull": {
      /** 🔮️ `d3-delaunay`'s own hull, reversed for the y-up frame and rotated to the same start. */
      oracle: (ctx: AdapterContext) => {
        const pts = points(ctx);
        const hull = Array.from(Delaunay.from(pts).hull, (index) => index + 1).reverse();
        return { projection: { "spatial/hull": rotated(hull, pts) } };
      },
      /** 🎯️ `\SemioVizHull`'s monotone chain over the same point set. */
      subject: async (ctx: AdapterContext): Promise<{ projection: ProbeProjection }> => {
        const records = await compileVizProbe(ctx.fixture("local://hull.tex"), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
        return { projection: probeProjection(records) };
      },
    },
  },
});
// #endregion 🧭️Adapter
