// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizGeoPath, vizGeoProjection, type VizGeoGeometry, type VizProjectionKind } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🗺️geo-path-graticule/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 9;

/** 🌍️ The demo landmass `demo-geo-regions` of semio-viz-geo, as the GeoJSON the twin streams. */
const REGIONS: VizGeoGeometry = {
  type: "FeatureCollection",
  features: (
    [
      [[-10, 50], [6, 52], [10, 46], [-4, 44]],
      [[10, 46], [24, 48], [28, 38], [12, 36]],
      [[-4, 44], [10, 46], [12, 36], [-2, 34]],
      [[-24, 46], [-10, 50], [-4, 44], [-18, 40]],
    ] as [number, number][][]
  ).map((ring) => ({ geometry: { type: "Polygon", coordinates: [[...ring, ring[0]!]] } })),
} as VizGeoGeometry;

const KINDS: Readonly<Record<string, VizProjectionKind>> = {
  equirectangular: "equirectangular",
  mercator: "mercator",
  "equal-earth": "equal-earth",
  "conic-equal-area": "conic-equal-area",
  "natural-earth1": "natural-earth",
};

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function kindOf(kind: string): VizProjectionKind {
  const found = KINDS[kind];
  if (found === undefined) throw new Error(`the twin registers no projection for the kind ${JSON.stringify(kind)}`);
  return found;
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(value: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** DECIMALS) / 10 ** DECIMALS;
}

/** 🔮️ The oracle of `geo-path-graticule`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`geo-path-graticule declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🎯️Subject
/** 🛤️ Streams the landmass through a projection and returns the emitted vertices, flattened. */
function projectedVertices(kind: string): number[] {
  const out: number[] = [];
  for (const command of vizGeoPath(REGIONS, vizGeoProjection(kindOf(kind)))) {
    if (command.op === "moveTo" || command.op === "lineTo") out.push(grid(command.args[0]), grid(command.args[1]));
  }
  return out;
}
// #endregion 🎯️Subject

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "path-without-resampling": {
      oracle: oracle("path-without-resampling"),
      /** 🎯️ `vizGeoPath` without a resampler on the same three projections. */
      subject: (ctx: AdapterContext) => ({ projection: { "geo/path": rows(ctx).flatMap((row) => projectedVertices(row.kind!)) } }),
    },
  },
});
// #endregion 🧭️Adapter
