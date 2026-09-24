// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Delaunay } from "d3-delaunay";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "spatial-delaunay-voronoi";
const DECIMALS_INDEX = 9;
const DECIMALS_PLANE = 6;

function rows(ctx: AdapterContext): Record<string, string>[] {
  const tables = ctx.scenario.steps.filter((step) => step.dataTable !== undefined).map((step) => step.dataTable!);
  const table = tables[tables.length - 1];
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 📍 The point set of the scenario's last data table, in the order the probe loads it. */
function points(ctx: AdapterContext): [number, number][] {
  return rows(ctx).map((row) => [Number(row.x), Number(row.y)] as [number, number]);
}

function grid(value: number, decimals: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** decimals) / 10 ** decimals;
}

/** 🔺 The canonical triangle code: indices sorted ascending, one-based, packed as i*10000 + j*100 + k. */
function triangleCodes(pts: readonly [number, number][]): number[] {
  const delaunay = Delaunay.from(pts as [number, number][]);
  const codes: number[] = [];
  for (let i = 0; i < delaunay.triangles.length; i += 3) {
    const [a, b, c] = [delaunay.triangles[i]! + 1, delaunay.triangles[i + 1]! + 1, delaunay.triangles[i + 2]! + 1].sort((p, q) => p - q);
    codes.push(a! * 10000 + b! * 100 + c!);
  }
  return codes.sort((p, q) => p - q);
}

/** 🔷 Every clipped cell as a lexicographically sorted vertex list, closing vertex removed. */
function cellVertices(pts: readonly [number, number][], bounds: [number, number, number, number]): number[][] {
  const voronoi = Delaunay.from(pts as [number, number][]).voronoi(bounds);
  const cells: number[][] = [];
  for (let i = 0; i < pts.length; i += 1) {
    const polygon = voronoi.cellPolygon(i) as [number, number][] | null;
    if (polygon === null) { cells.push([]); continue; }
    const vertices = polygon.slice(0, -1).map(([x, y]) => [grid(x, DECIMALS_PLANE), grid(y, DECIMALS_PLANE)] as [number, number]);
    vertices.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    cells.push(vertices.flat());
  }
  return cells;
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
    "delaunay-triangles": {
      /** 🔮️ Delaunator's triangulation through d3-delaunay, canonicalised the same way as the probe. */
      oracle: (ctx: AdapterContext) => ({ projection: { "spatial/delaunay": triangleCodes(points(ctx)) } }),
      /** 🎯️ `\SemioVizDelaunay`'s Bowyer–Watson triangulation of the same point set. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "shared://🔺️spatial-delaunay-voronoi/delaunay.tex", DECIMALS_INDEX)),
    },
    "voronoi-cells": {
      /** 🔮️ `d3-delaunay`'s `voronoi(bounds).cellPolygon`, one sorted vertex list per site. */
      oracle: (ctx: AdapterContext) => ({ projection: { "spatial/voronoi": cellVertices(points(ctx), [0, 0, 70, 50]).flat() } }),
      /** 🎯️ `\SemioVizVoronoi` over the same sites and the same clip rectangle. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "shared://🔺️spatial-delaunay-voronoi/voronoi.tex", DECIMALS_PLANE)),
    },
  },
});
// #endregion 🧭️Adapter
