// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizDelaunay, vizVoronoi } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🔺️spatial-delaunay-voronoi/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS_PLANE = 6;
const BOUNDS: [number, number, number, number] = [0, 0, 70, 50];

/** 🔮️ The oracle of `🔺️spatial-delaunay-voronoi`, reused so both subjects meet the same numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`spatial-delaunay-voronoi declares no oracle for ${id}`);
  return handler;
}

/** 🧫️ The point set of the scenario's last data table, in the order the base case loads it. */
function points(ctx: AdapterContext): [number, number][] {
  const tables = ctx.scenario.steps.filter((step) => step.dataTable !== undefined).map((step) => step.dataTable!);
  const table = tables[tables.length - 1];
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => {
    const record = Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""]));
    return [Number(record.x), Number(record.y)] as [number, number];
  });
}

function grid(value: number, decimals: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** decimals) / 10 ** decimals;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "delaunay-triangles": {
      oracle: oracle("delaunay-triangles"),
      /** 🎯️ The twin's Bowyer–Watson triangulation, canonicalised exactly as the probe canonicalises. */
      subject: (ctx: AdapterContext) => {
        const codes = vizDelaunay(points(ctx)).triangles.map((triangle) => {
          const [a, b, c] = [triangle[0] + 1, triangle[1] + 1, triangle[2] + 1].sort((p, q) => p - q);
          return a! * 10000 + b! * 100 + c!;
        });
        return { projection: { "spatial/delaunay": codes.sort((p, q) => p - q) } };
      },
    },
    "voronoi-cells": {
      oracle: oracle("voronoi-cells"),
      /** 🎯️ The twin's half-plane clipped cells, one lexicographically sorted vertex list per site. */
      subject: (ctx: AdapterContext) => {
        const cells = vizVoronoi(points(ctx), BOUNDS).cells.map((cell) => {
          const vertices = cell.map(([x, y]) => [grid(x, DECIMALS_PLANE), grid(y, DECIMALS_PLANE)] as [number, number]);
          vertices.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
          return vertices.flat();
        });
        return { projection: { "spatial/voronoi": cells.flat() } };
      },
    },
  },
});
// #endregion 🧭️Adapter
