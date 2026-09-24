// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { arc, pie } from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "shape-arc-pie";
const FIXTURE = "shared://🌗️shape-arc-pie/shape-arc-pie.tex";

const DECIMALS = 6;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔤️ Splits an SVG path into the token stream the probe records: command letters as
 *  strings, coordinates as numbers, in the order d3 wrote them. */
function tokens(path: string | null): (number | string)[] {
  const out: (number | string)[] = [];
  const pattern = /[MLCQAZhv]|-?\d*\.?\d+(?:e[-+]?\d+)?/gi;
  let match: RegExpExecArray | null;
  while ((match = pattern.exec(path ?? "")) !== null) {
    const raw = match[0];
    const value = Number(raw);
    out.push(Number.isNaN(value) ? raw : grid([value])[0]!);
  }
  return out;
}

type ArcSpec = Readonly<{ innerRadius: number; outerRadius: number; startAngle: number; endAngle: number; padAngle?: number; padRadius?: number; cornerRadius?: number }>;

/** 🔮️ The generator d3 builds for one arc specification. */
function generator(spec: ArcSpec) {
  let built = arc<unknown>()
    .innerRadius(spec.innerRadius)
    .outerRadius(spec.outerRadius)
    .startAngle(spec.startAngle)
    .endAngle(spec.endAngle)
    .padAngle(spec.padAngle ?? 0)
    .cornerRadius(spec.cornerRadius ?? 0)
    .digits(15);
  if (spec.padRadius !== undefined) built = built.padRadius(spec.padRadius);
  return built;
}

const ARCS: Record<string, ArcSpec> = {
  plain: { innerRadius: 0, outerRadius: 100, startAngle: 0, endAngle: 1.2 },
  annular: { innerRadius: 40, outerRadius: 100, startAngle: 0.3, endAngle: 2.5 },
  pad: { innerRadius: 40, outerRadius: 100, startAngle: 0.3, endAngle: 2.5, padAngle: 0.05 },
  "pad-radius": { innerRadius: 40, outerRadius: 100, startAngle: 0.3, endAngle: 2.5, padAngle: 0.05, padRadius: 120 },
  corner: { innerRadius: 40, outerRadius: 100, startAngle: 0.3, endAngle: 2.5, cornerRadius: 12 },
  "pad-corner": { innerRadius: 40, outerRadius: 100, startAngle: 0.3, endAngle: 2.5, padAngle: 0.05, cornerRadius: 12 },
  "corner-solid": { innerRadius: 0, outerRadius: 100, startAngle: 0.3, endAngle: 2.5, cornerRadius: 12 },
  full: { innerRadius: 40, outerRadius: 100, startAngle: 0, endAngle: 2 * Math.PI },
  "full-disc": { innerRadius: 0, outerRadius: 100, startAngle: 0, endAngle: 2 * Math.PI },
  counter: { innerRadius: 40, outerRadius: 100, startAngle: 2.5, endAngle: 0.3 },
  collapsed: { innerRadius: 40, outerRadius: 100, startAngle: 0, endAngle: 0.2, padAngle: 0.5 },
};

const CENTROIDS: Record<string, ArcSpec> = {
  annular: { innerRadius: 40, outerRadius: 100, startAngle: 0.3, endAngle: 2.5 },
  wedge: { innerRadius: 0, outerRadius: 60, startAngle: 1, endAngle: 1.9 },
  counter: { innerRadius: 10, outerRadius: 30, startAngle: 2.5, endAngle: 0.3 },
};

type PieSpec = Readonly<{ value: number[]; startAngle: number; endAngle: number; padAngle?: number; sort?: string }>;
const PIES: Record<string, PieSpec> = {
  descending: { value: [1, 2, 3, 4], startAngle: 0, endAngle: 2 * Math.PI },
  padded: { value: [1, 2, 3, 4], startAngle: 0, endAngle: 2 * Math.PI, padAngle: 0.05 },
  unsorted: { value: [5, 1, 4, 2], startAngle: 0.5, endAngle: 4, sort: "none" },
  ascending: { value: [5, 1, 4, 2], startAngle: 0, endAngle: 2 * Math.PI, sort: "ascending" },
  "all-zero": { value: [0, 0, 0], startAngle: 0, endAngle: 2 * Math.PI },
};

/** 🔮️ Start angles first, then end angles, in the data's own order. */
function angles(spec: PieSpec): number[] {
  let layout = pie<number>().startAngle(spec.startAngle).endAngle(spec.endAngle).padAngle(spec.padAngle ?? 0);
  if (spec.sort === "none") layout = layout.sortValues(null);
  else if (spec.sort === "ascending") layout = layout.sortValues((a, b) => a - b);
  const arcs = layout(spec.value);
  return grid([...arcs.map((a) => a.startAngle), ...arcs.map((a) => a.endAngle)]);
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "arc-geometry": {
      /** 🔮️ d3-arc walks every padding and corner branch these specifications reach. */
      oracle: () => ({
        projection: Object.fromEntries(Object.entries(ARCS).map(([tag, spec]) => [`arc/${tag}`, tokens(generator(spec)(undefined))])),
      }),
      /** 🎯️ The same specifications through \SemioVizArc, drawing nothing. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "arc-centroid": {
      /** 🔮️ d3's own centroid, which ignores padding and corners entirely. */
      oracle: () => ({
        projection: Object.fromEntries(Object.entries(CENTROIDS).map(([tag, spec]) => [`centroid/${tag}`, grid(generator(spec).centroid(undefined))])),
      }),
      /** 🎯️ \SemioVizArcCentroid after the same arcs. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "pie-angles": {
      /** 🔮️ d3-pie's angles, in the data's order whatever the sort was. */
      oracle: () => ({
        projection: Object.fromEntries(Object.entries(PIES).map(([tag, spec]) => [`pie/${tag}`, angles(spec)])),
      }),
      /** 🎯️ \SemioVizPie over the same values. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
