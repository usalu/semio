// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as geo from "d3-geo";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "geo-projections";
const DECIMALS = 9;

/** 🌍️ The demo landmass `demo-geo-regions` of semio-viz-geo, as the GeoJSON d3-geo fits and paths. */
const REGIONS = {
  type: "FeatureCollection",
  features: (
    [
      [[-10, 50], [6, 52], [10, 46], [-4, 44]],
      [[10, 46], [24, 48], [28, 38], [12, 36]],
      [[-4, 44], [10, 46], [12, 36], [-2, 34]],
      [[-24, 46], [-10, 50], [-4, 44], [-18, 40]],
    ] as [number, number][][]
  ).map((ring) => ({ type: "Feature", properties: {}, geometry: { type: "Polygon", coordinates: [[...ring, ring[0]!]] } })),
} as const;

/** 🗺️ Every kind semio-viz-geo implements, with the d3 constructor it reimplements. */
const KINDS: Record<string, () => geo.GeoProjection> = {
  equirectangular: geo.geoEquirectangular,
  mercator: geo.geoMercator,
  "transverse-mercator": geo.geoTransverseMercator,
  "equal-earth": geo.geoEqualEarth,
  "natural-earth1": geo.geoNaturalEarth1,
  orthographic: geo.geoOrthographic,
  stereographic: geo.geoStereographic,
  gnomonic: geo.geoGnomonic,
  "azimuthal-equal-area": geo.geoAzimuthalEqualArea,
  "azimuthal-equidistant": geo.geoAzimuthalEquidistant,
  "conic-conformal": geo.geoConicConformal,
  "conic-equal-area": geo.geoConicEqualArea,
  "conic-equidistant": geo.geoConicEquidistant,
  albers: geo.geoAlbers,
};

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🗺️ The d3 projection of a table row's `kind` column. */
function projectionOf(kind: string): geo.GeoProjection {
  const make = KINDS[kind];
  if (make === undefined) throw new Error(`no d3-geo constructor is registered for the kind ${JSON.stringify(kind)}`);
  return make();
}

/** 🔢️ Rounds an oracle's own numbers onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

async function subject(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(fixture), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "forward-projection": {
      /** 🔮️ d3-geo's own constructors, each with its own untouched defaults. */
      oracle: (ctx: AdapterContext) => ({
        projection: {
          "geo/forward": grid(rows(ctx).flatMap((row) => projectionOf(row.kind!)([Number(row.lon), Number(row.lat)]) as [number, number])),
        },
      }),
      /** 🎯️ `\SemioVizGeoProject` on the same fourteen kinds, compiled with the repository tectonic. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "shared://🌍️geo-projections/forward.tex")),
    },
    "inverse-projection": {
      /** 🔮️ d3-geo's `invert` on the same plane point. */
      oracle: (ctx: AdapterContext) => ({
        projection: {
          "geo/inverse": grid(rows(ctx).flatMap((row) => projectionOf(row.kind!).invert!([Number(row.x), Number(row.y)]) as [number, number])),
        },
      }),
      /** 🎯️ `\SemioVizGeoInvert` on the same fourteen kinds. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "shared://🌍️geo-projections/inverse.tex")),
    },
    "fit-extent": {
      /** 🔮️ d3-geo's `fitSize`, `fitExtent` and `fitWidth` over the same landmass. */
      oracle: (ctx: AdapterContext) => {
        const size: number[] = [];
        const extent: number[] = [];
        const width: number[] = [];
        for (const row of rows(ctx)) {
          const w = Number(row.width);
          const h = Number(row.height);
          const a = projectionOf(row.kind!).fitSize([w, h], REGIONS as never);
          size.push(a.scale(), ...a.translate());
          const b = projectionOf(row.kind!).fitExtent([[10, 5], [110, 65]], REGIONS as never);
          extent.push(b.scale(), ...b.translate());
          const c = projectionOf(row.kind!).fitWidth(w, REGIONS as never);
          width.push(c.scale(), ...c.translate());
        }
        return { projection: { "geo/fit-size": grid(size), "geo/fit-extent": grid(extent), "geo/fit-width": grid(width) } };
      },
      /** 🎯️ `\SemioVizGeoFitSize`, `\SemioVizGeoFit` and `\SemioVizGeoFitWidth` over `demo-geo-regions`. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "shared://🌍️geo-projections/fit.tex")),
    },
  },
});
// #endregion 🧭️Adapter
