// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { rawConicEqualArea, rawEquirectangular, rawMercator, vizGeoBounds, vizGeoFitExtent, vizGeoFitSize, vizGeoProjection, vizProjection, type VizGeoGeometry, type VizProjectionKind, type VizProjectionSettings, type VizRawProjection } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🌍️geo-projections/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 9;
const RADIANS = Math.PI / 180;

/** 🌍️ The demo landmass `demo-geo-regions` of semio-viz-geo, as the GeoJSON the twin fits. */
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

/** 🗺️ The kind names of the feature's tables, mapped onto the twin's own registered kinds. */
const KINDS: Readonly<Record<string, VizProjectionKind>> = {
  equirectangular: "equirectangular",
  mercator: "mercator",
  "transverse-mercator": "transverse-mercator",
  "equal-earth": "equal-earth",
  "natural-earth1": "natural-earth",
  orthographic: "orthographic",
  stereographic: "stereographic",
  gnomonic: "gnomonic",
  "azimuthal-equal-area": "azimuthal-equal-area",
  "azimuthal-equidistant": "azimuthal-equidistant",
  "conic-conformal": "conic-conformal",
  "conic-equal-area": "conic-equal-area",
  "conic-equidistant": "conic-equidistant",
  albers: "albers",
};

/** 🗺️ The raw projection and the settings a fit has to carry through, per fitted kind. */
const FITTED: Readonly<Record<string, { raw: VizRawProjection; settings: VizProjectionSettings }>> = {
  equirectangular: { raw: rawEquirectangular, settings: {} },
  mercator: { raw: rawMercator, settings: {} },
  "conic-equal-area": { raw: rawConicEqualArea(0 * RADIANS, 60 * RADIANS), settings: { center: [0, 33.6442] } },
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

function fittedOf(kind: string): { raw: VizRawProjection; settings: VizProjectionSettings } {
  const found = FITTED[kind];
  if (found === undefined) throw new Error(`the twin registers no raw projection for the fitted kind ${JSON.stringify(kind)}`);
  return found;
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

/** 🔮️ The oracle of `geo-projections`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`geo-projections declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🎯️Subject
/** 🗺️ Fits a geometry to a width alone: d3's `fitWidth` is `fitExtent` on the height the width buys. */
function fitWidth(width: number, kind: string): { scale: number; translate: readonly [number, number] } {
  const { raw, settings } = fittedOf(kind);
  const bounds = vizGeoBounds(REGIONS, vizProjection(raw, { ...settings, scale: 150, translate: [0, 0] }));
  const k = width / (bounds.x1 - bounds.x0);
  const fitted = vizGeoFitExtent(vizGeoProjection(kindOf(kind)), [[0, 0], [width, k * (bounds.y1 - bounds.y0)]], REGIONS, raw, settings);
  return { scale: fitted.scale(), translate: fitted.translate() };
}
// #endregion 🎯️Subject

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "forward-projection": {
      oracle: oracle("forward-projection"),
      /** 🎯️ `vizGeoProjection` on the same fourteen kinds, each with its own untouched defaults. */
      subject: (ctx: AdapterContext) => ({
        projection: { "geo/forward": grid(rows(ctx).flatMap((row) => vizGeoProjection(kindOf(row.kind!))([Number(row.lon), Number(row.lat)]))) },
      }),
    },
    "fit-extent": {
      oracle: oracle("fit-extent"),
      /** 🎯️ `vizGeoFitSize`, `vizGeoFitExtent` and the width fit over `demo-geo-regions`. */
      subject: (ctx: AdapterContext) => {
        const size: number[] = [];
        const extent: number[] = [];
        const width: number[] = [];
        for (const row of rows(ctx)) {
          const w = Number(row.width);
          const h = Number(row.height);
          const { raw, settings } = fittedOf(row.kind!);
          const a = vizGeoFitSize(vizGeoProjection(kindOf(row.kind!)), [w, h], REGIONS, raw, settings);
          size.push(a.scale(), ...a.translate());
          const b = vizGeoFitExtent(vizGeoProjection(kindOf(row.kind!)), [[10, 5], [110, 65]], REGIONS, raw, settings);
          extent.push(b.scale(), ...b.translate());
          const c = fitWidth(w, row.kind!);
          width.push(c.scale, ...c.translate);
        }
        return { projection: { "geo/fit-size": grid(size), "geo/fit-extent": grid(extent), "geo/fit-width": grid(width) } };
      },
    },
  },
});
// #endregion 🧭️Adapter
