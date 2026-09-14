// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizConvexHull } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🥚️spatial-hull/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 🔮️ The oracle of `🥚️spatial-hull`, reused so both subjects meet the same hull. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`spatial-hull declares no oracle for ${id}`);
  return handler;
}

/** 🧫️ The point set of the scenario's data table, in the order the base case loads it. */
function points(ctx: AdapterContext): [number, number][] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => {
    const record = Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""]));
    return [Number(record.x), Number(record.y)] as [number, number];
  });
}

/** 🔶️ One-based hull indices rotated so the cycle starts at the lexicographically smallest point. */
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
      oracle: oracle("convex-hull"),
      /** 🎯️ The twin's monotone chain, one-based and rotated onto the same starting vertex. */
      subject: (ctx: AdapterContext) => {
        const pts = points(ctx);
        const hull = vizConvexHull(pts).map((index) => index + 1);
        return { projection: { "spatial/hull": rotated(hull, pts) } };
      },
    },
  },
});
// #endregion 🧭️Adapter
