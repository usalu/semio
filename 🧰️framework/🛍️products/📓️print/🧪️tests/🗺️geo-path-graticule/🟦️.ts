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
const CASE = "geo-path-graticule";
const DECIMALS = 9;

/** 🌍️ The demo landmass `demo-geo-regions` of semio-viz-geo, as the GeoJSON d3-geo streams. */
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

const KINDS: Record<string, () => geo.GeoProjection> = {
  equirectangular: geo.geoEquirectangular,
  mercator: geo.geoMercator,
  "equal-earth": geo.geoEqualEarth,
  "conic-equal-area": geo.geoConicEqualArea,
  "natural-earth1": geo.geoNaturalEarth1,
};

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function grid(value: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** DECIMALS) / 10 ** DECIMALS;
}

/** 🛤️ Streams the landmass through a projection and returns the emitted subpaths, flattened. */
function projectedVertices(projection: geo.GeoProjection): number[] {
  const out: number[] = [];
  let current: number[] = [];
  const sink = {
    point(x: number, y: number) { current.push(grid(x), grid(y)); },
    lineStart() { current = []; },
    lineEnd() { out.push(...current); },
    polygonStart() { /* rings are streamed as lines */ },
    polygonEnd() { /* rings are streamed as lines */ },
    sphere() { /* the landmass carries no sphere */ },
  };
  geo.geoStream(REGIONS as never, projection.stream(sink as never));
  return out;
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
    "path-without-resampling": {
      /** 🔮️ d3-geo's own stream at `precision(0)`, which is its `resampleNone`. */
      oracle: (ctx: AdapterContext) => ({
        projection: { "geo/path": rows(ctx).flatMap((row) => projectedVertices(KINDS[row.kind!]!().precision(Number(row.precision)))) },
      }),
      /** 🎯️ `\semio_viz_geo_path:nn` with `precision = 0` on the same three projections. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "local://path-none.tex")),
    },
    "path-with-resampling": {
      /** 🔮️ d3-geo's own stream at its default precision, i.e. the adaptive resampler. */
      oracle: (ctx: AdapterContext) => ({
        projection: { "geo/path": rows(ctx).flatMap((row) => projectedVertices(KINDS[row.kind!]!())) },
      }),
      /** 🎯️ `\semio_viz_geo_path:nn` at the same default precision. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "local://path-resampled.tex")),
    },
    "graticule-lines": {
      /** 🔮️ `d3.geoGraticule().lines()` at the same minor step, before any projection. */
      oracle: (ctx: AdapterContext) => ({
        projection: {
          "geo/graticule": rows(ctx).flatMap((row) =>
            geo
              .geoGraticule()
              .stepMinor([Number(row.stepMinorX), Number(row.stepMinorY)])
              .precision(Number(row.precision))
              .lines()
              .flatMap((line) => (line.coordinates as [number, number][]).flatMap(([lon, lat]) => [grid(lon), grid(lat)])),
          ),
        },
      }),
      /** 🎯️ `\semio_viz_geo_graticule_lines:n` at the same minor step. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "local://graticule.tex")),
    },
  },
});
// #endregion 🧭️Adapter
