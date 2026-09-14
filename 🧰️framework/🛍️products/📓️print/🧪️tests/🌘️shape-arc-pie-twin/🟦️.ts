// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { pathRound } from "d3-path";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizArc, vizArcCentroid, vizPie, type VizArcOptions } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🌗️shape-arc-pie/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔤️ Splits an SVG path into the token stream the probe records: command letters as
 *  strings, coordinates as numbers, in the order the generator wrote them. */
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

/** 🖊️ The twin writes its geometry into the same serialiser d3 writes into, so only the geometry
 *  and never the number formatting distinguishes the two subjects. */
function serialise(draw: (context: ReturnType<typeof pathRound>) => void): (number | string)[] {
  const context = pathRound(15);
  draw(context);
  return tokens(context.toString());
}

const ARCS: Record<string, VizArcOptions> = {
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

const CENTROIDS: Record<string, VizArcOptions> = {
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

/** 🥧️ Start angles first, then end angles, in the data's own order. */
function angles(spec: PieSpec): number[] {
  const sortValues = spec.sort === "none" ? null : spec.sort === "ascending" ? (a: number, b: number) => a - b : undefined;
  const slices = vizPie(spec.value, { startAngle: spec.startAngle, endAngle: spec.endAngle, padAngle: spec.padAngle ?? 0, ...(sortValues === undefined ? {} : { sortValues }) });
  return grid([...slices.map((slice) => slice.startAngle), ...slices.map((slice) => slice.endAngle)]);
}

/** 🔮️ The oracle of `shape-arc-pie`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`shape-arc-pie declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "arc-geometry": {
      oracle: oracle("arc-geometry"),
      /** 🎯️ The twin's `vizArc` walks every padding and corner branch these specifications reach. */
      subject: () => ({
        projection: Object.fromEntries(Object.entries(ARCS).map(([tag, spec]) => [`arc/${tag}`, serialise((context) => void vizArc(spec, context))])),
      }),
    },
    "arc-centroid": {
      oracle: oracle("arc-centroid"),
      /** 🎯️ The twin's centroid, which ignores padding and corners entirely. */
      subject: () => ({
        projection: Object.fromEntries(Object.entries(CENTROIDS).map(([tag, spec]) => [`centroid/${tag}`, grid(vizArcCentroid(spec))])),
      }),
    },
    "pie-angles": {
      oracle: oracle("pie-angles"),
      /** 🎯️ The twin's `vizPie`, in the data's order whatever the sort was. */
      subject: () => ({
        projection: Object.fromEntries(Object.entries(PIES).map(([tag, spec]) => [`pie/${tag}`, angles(spec)])),
      }),
    },
  },
});
// #endregion 🧭️Adapter
