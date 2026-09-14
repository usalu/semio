// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizContours } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🏔️spatial-contours-density/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🔢️ `demo-grid` of semio-viz-spatial: six columns, five rows, row-major. */
const DEMO_GRID = [0, 1, 2, 2, 1, 0, 1, 3, 5, 5, 3, 1, 2, 5, 9, 8, 5, 2, 1, 4, 6, 6, 4, 1, 0, 1, 2, 2, 1, 0];

/** 🔮️ The oracle of `🏔️spatial-contours-density`, reused so both subjects meet the same rings. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`spatial-contours-density declares no oracle for ${id}`);
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

/** 〰️ The twin's rings, shifted onto the node-centred grid and de-duplicated exactly as the oracle is. */
function isolines(values: readonly number[], width: number, height: number, thresholds: readonly number[]): number[] {
  const out: number[] = [];
  for (const threshold of thresholds) {
    const [contour] = vizContours(values, [width, height], [threshold]);
    if (contour === undefined) throw new Error(`the twin produced no contour at the threshold ${threshold}`);
    for (const polygon of contour.coordinates) {
      for (const ring of polygon) {
        const vertices: [number, number][] = [];
        for (const [x, y] of ring) {
          const point: [number, number] = [grid(x - 0.5), grid(y - 0.5)];
          const previous = vertices[vertices.length - 1];
          if (previous === undefined || previous[0] !== point[0] || previous[1] !== point[1]) vertices.push(point);
        }
        out.push(...vertices.flat());
      }
    }
  }
  return out;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "marching-squares": {
      oracle: oracle("marching-squares"),
      /** 🎯️ The twin's marching squares over `demo-grid` at the same thresholds. */
      subject: (ctx: AdapterContext) => {
        const [row] = rows(ctx);
        if (row === undefined) throw new Error("the marching-squares scenario carries no parameter row");
        const thresholds = row.thresholds!.split(",").map((value) => Number(value.trim()));
        return { projection: { "spatial/contour": isolines(DEMO_GRID, Number(row.width), Number(row.height), thresholds) } };
      },
    },
  },
});
// #endregion 🧭️Adapter
