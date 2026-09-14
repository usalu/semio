// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { contours } from "d3-contour";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "spatial-contours-density";
const DECIMALS = 6;
const DENSITY_DECIMALS = 9;

/** 🔢️ `demo-grid` of semio-viz-spatial: six columns, five rows, row-major. */
const DEMO_GRID = [0, 1, 2, 2, 1, 0, 1, 3, 5, 5, 3, 1, 2, 5, 9, 8, 5, 2, 1, 4, 6, 6, 4, 1, 0, 1, 2, 2, 1, 0];

/** 📍 `demo-points` of semio-viz-spatial, the sample the density grid is built from. */
const POINTS: [number, number][] = [[12, 14], [28, 9], [41, 26], [19, 33], [55, 17], [63, 31], [34, 41], [8, 27], [47, 7], [58, 44], [25, 20], [39, 13]];

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function grid(value: number, decimals: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** decimals) / 10 ** decimals;
}

/** 〰️ d3-contour's rings, shifted onto the node-centred grid, with the consecutive duplicates it emits where two
 *  segments meet exactly on a grid node removed. The closing vertex is kept: both sides report a closed ring. */
function isolines(values: readonly number[], width: number, height: number, thresholds: readonly number[], decimals: number): number[] {
  const out: number[] = [];
  for (const threshold of thresholds) {
    const [contour] = contours().size([width, height]).thresholds([threshold])(values as number[]);
    if (contour === undefined) throw new Error(`d3-contour produced no contour at the threshold ${threshold}`);
    for (const polygon of contour.coordinates) {
      for (const ring of polygon) {
        const vertices: [number, number][] = [];
        for (const [x, y] of ring as [number, number][]) {
          const point: [number, number] = [grid(x - 0.5, decimals), grid(y - 0.5, decimals)];
          const previous = vertices[vertices.length - 1];
          if (previous === undefined || previous[0] !== point[0] || previous[1] !== point[1]) vertices.push(point);
        }
        out.push(...vertices.flat());
      }
    }
  }
  return out;
}

/** 🌫️ The exact Gaussian kernel-density sum this library specifies, evaluated on the same lattice. */
function densityGrid(width: number, height: number, originX: number, originY: number, cell: number, bandwidth: number): number[] {
  const values: number[] = [];
  for (let j = 0; j < height; j += 1) {
    for (let i = 0; i < width; i += 1) {
      const x = originX + i * cell;
      const y = originY + j * cell;
      let sum = 0;
      for (const [px, py] of POINTS) sum += Math.exp(-((x - px) ** 2 + (y - py) ** 2) / (2 * bandwidth ** 2)) / (2 * Math.PI * bandwidth ** 2);
      values.push(sum);
    }
  }
  return values;
}

async function subject(ctx: AdapterContext, fixture: string, decimals: number): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(fixture), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, decimals)) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "marching-squares": {
      /** 🔮️ `d3-contour`'s isolines of the same grid, at the same interior thresholds. */
      oracle: (ctx: AdapterContext) => {
        const [row] = rows(ctx);
        if (row === undefined) throw new Error("the marching-squares scenario carries no parameter row");
        const thresholds = row.thresholds!.split(",").map((value) => Number(value.trim()));
        return { projection: { "spatial/contour": isolines(DEMO_GRID, Number(row.width), Number(row.height), thresholds, DECIMALS) } };
      },
      /** 🎯️ `\SemioVizContour` over `demo-grid` at the same thresholds. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "local://contours.tex", DECIMALS)),
    },
    "kernel-density": {
      /** 🔮️ The exact Gaussian sum (independent implementation of the specification) and `d3-contour` on it. */
      oracle: (ctx: AdapterContext) => {
        const [row] = rows(ctx);
        if (row === undefined) throw new Error("the kernel-density scenario carries no parameter row");
        const width = Number(row.width);
        const height = Number(row.height);
        const values = densityGrid(width, height, Number(row.originX), Number(row.originY), Number(row.cell), Number(row.bandwidth));
        return {
          projection: {
            "spatial/density": values.map((value) => grid(value, DENSITY_DECIMALS)),
            "spatial/density-contour": isolines(values, width, height, [Number(row.threshold)], DENSITY_DECIMALS),
          },
        };
      },
      /** 🎯️ `\SemioVizDensity` and `\SemioVizContour` on the grid it produced. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "local://density.tex", DENSITY_DECIMALS)),
    },
  },
});
// #endregion 🧭️Adapter
