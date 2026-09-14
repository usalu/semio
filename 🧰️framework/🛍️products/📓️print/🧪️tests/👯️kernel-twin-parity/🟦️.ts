// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import { scaleLinear, scaleLog, vizConvexHull, vizGeoProjection } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import chordLatex from "../🪢️network-chord/🟦️.ts";
import chordTwin from "../🎗️network-chord-twin/🟦️.ts";
import circularLatex from "../⭕️network-circular-arc/🟦️.ts";
import circularTwin from "../🌐️network-circular-arc-twin/🟦️.ts";
import sankeyLatex from "../🚰️flow-sankey/🟦️.ts";
import sankeyTwin from "../🚿️flow-sankey-twin/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "kernel-twin-parity";
const DECIMALS = 6;

/** 🔢️ Rounds the twin onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🎯️ Compiles a probe document built here and projects its records. */
async function probe(ctx: AdapterContext, packages: readonly string[], body: readonly VizProbeStatement[], preamble?: readonly string[]): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: [...packages], preamble: preamble === undefined ? undefined : [...preamble], body: [...body] },
    { workDir: ctx.workDir },
  );
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
}

/** 👯️ One scenario the two kernels already answer through the case they double: the LaTeX probe of
 *  the base case as the subject, the twin call of that case's `-twin` as the reference. */
function doubled(
  latex: { scenarios: Record<string, { subject?: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> }> },
  twin: { scenarios: Record<string, { subject?: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> }> },
  id: string,
): { oracle: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome>; subject: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> } {
  const subject = latex.scenarios[id]?.subject;
  const oracle = twin.scenarios[id]?.subject;
  if (subject === undefined || oracle === undefined) throw new Error(`no doubled pair for ${id}`);
  return { oracle, subject };
}
// #endregion 🧫️Vectors

// #region 📐️Scale
const SCALE_PREAMBLE = [
  "\\ExplSyntaxOn",
  "\\fp_new:N \\l_ktp_value_fp",
  "\\seq_new:N \\l_ktp_seq",
  "\\cs_new_protected:Npn \\ktpmap #1#2#3 {",
  "  \\seq_clear:N \\l_ktp_seq",
  "  \\clist_map_inline:nn {#3}",
  "    {",
  "      \\semio_viz_scale_map:nnN {#2} {##1} \\l_ktp_value_fp",
  "      \\seq_put_right:Nx \\l_ktp_seq { \\fp_to_decimal:n { round( \\l_ktp_value_fp , 6 ) } }",
  "    }",
  "  \\semio_viz_probe_values:nx {#1} { \\seq_use:Nn \\l_ktp_seq { , } }",
  "}",
  "\\ExplSyntaxOff",
];

const LINEAR_INPUTS = [0, 12.5, 25, 42, 100];
const LOG_INPUTS = [1, 10, 42, 100, 1000];
// #endregion 📐️Scale

// #region 🌍️Geo
function geoPoints(ctx: AdapterContext): [number, number][] {
  return rows(ctx).map((row) => [Number(row.longitude), Number(row.latitude)] as [number, number]);
}
// #endregion 🌍️Geo

// #region 📍️Spatial
function hullPoints(ctx: AdapterContext): [number, number][] {
  return rows(ctx).map((row) => [Number(row.x), Number(row.y)] as [number, number]);
}

/** 🔶️ One-based hull indices rotated so the cycle starts at the lexicographically smallest point. */
function rotated(indices: readonly number[], points: readonly [number, number][]): number[] {
  let start = 0;
  for (let i = 1; i < indices.length; i += 1) {
    const a = points[indices[i]! - 1]!;
    const b = points[indices[start]! - 1]!;
    if (a[0] < b[0] || (a[0] === b[0] && a[1] < b[1])) start = i;
  }
  return indices.slice(start).concat(indices.slice(0, start));
}
// #endregion 📍️Spatial

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "plain-matrix": doubled(chordLatex, chordTwin, "plain-matrix"),
    "circular-equal-spacing": doubled(circularLatex, circularTwin, "circular-equal-spacing"),
    "align-justify": doubled(sankeyLatex, sankeyTwin, "align-justify"),
    "scale-linear-and-log": {
      /** 🔮️ The twin's linear and logarithmic scales over the same domains and ranges. */
      oracle: () => {
        const lin = scaleLinear([0, 100], [0, 180]);
        const lg = scaleLog([1, 1000], [0, 300]);
        return {
          projection: {
            "scale/linear": grid(LINEAR_INPUTS.map((value) => lin(value))),
            "scale/log": grid(LOG_INPUTS.map((value) => lg(value))),
          },
        };
      },
      /** 🎯️ `\SemioVizScale` in a compiled document, read back through `semio_viz_scale_map:nnN`. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, ["semio-viz-scale"], [
          { precision: DECIMALS },
          { raw: "\\SemioVizScale{lin}{linear}{0,100}{0,180}" },
          { raw: "\\SemioVizScale{lg}{log}{1,1000}{0,300}" },
          { raw: `\\ktpmap{scale/linear}{lin}{${LINEAR_INPUTS.join(",")}}` },
          { raw: `\\ktpmap{scale/log}{lg}{${LOG_INPUTS.join(",")}}` },
        ], SCALE_PREAMBLE)),
    },
    "geo-mercator": {
      /** 🔮️ The twin's Mercator projection at the library's default scale and translation. */
      oracle: (ctx: AdapterContext) => {
        const projection = vizGeoProjection("mercator");
        return { projection: { "geo/mercator": grid(geoPoints(ctx).flatMap((point) => projection(point))) } };
      },
      /** 🎯️ `\SemioVizProjection{mercator}` and `\SemioVizGeoProject` in a compiled document. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, ["semio-viz-geo"], [
          { precision: DECIMALS },
          { raw: "\\ExplSyntaxOn" },
          { raw: "\\SemioVizProjection { probe } [ kind = mercator ]" },
          ...geoPoints(ctx).map(([lon, lat]) => ({
            raw: `\\SemioVizGeoProject { probe } { ${lon} } { ${lat} } \\SemioVizProbeEval { geo/mercator } { \\l_semio_viz_geo_x_fp , \\l_semio_viz_geo_y_fp }`,
          })),
          { raw: "\\ExplSyntaxOff" },
        ])),
    },
    "spatial-hull": {
      /** 🔮️ The twin's monotone chain over the same points, one-based and rotated to the same start. */
      oracle: (ctx: AdapterContext) => {
        const points = hullPoints(ctx);
        return { projection: { "spatial/hull": rotated(vizConvexHull(points).map((index) => index + 1), points) } };
      },
      /** 🎯️ `\SemioVizHull` over `demo-points`, the same twelve points the table writes out. */
      subject: async (ctx: AdapterContext) => {
        const points = hullPoints(ctx);
        const raw = (await probe(ctx, ["semio-viz-spatial"], [
          { precision: DECIMALS },
          { raw: "\\ExplSyntaxOn" },
          { raw: "\\SemioVizHull { demo-points }" },
          { raw: "\\use:x { \\exp_not:N \\SemioVizProbeEval { spatial/hull } { \\seq_use:Nn \\g_semio_viz_spatial_hull_seq { , } } }" },
          { raw: "\\ExplSyntaxOff" },
        ])).projection["spatial/hull"] ?? [];
        return { projection: { "spatial/hull": rotated(raw.map(Number), points) } };
      },
    },
  },
});
// #endregion 🧭️Adapter
