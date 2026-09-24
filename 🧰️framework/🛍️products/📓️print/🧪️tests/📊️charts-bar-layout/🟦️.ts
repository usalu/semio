// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleBand, scaleLinear } from "d3-scale";
import { stack, stackOffsetExpand } from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-bar-layout";
const DECIMALS = 4;

/** 🧪️ `demo-cartesian` of `semio-viz-charts-bar`, in the row order the table declares. */
const CATEGORIES = ["A", "B", "C", "D", "E"] as const;
const SERIES = ["North", "South", "East"] as const;
const VALUES: Record<string, Record<string, number>> = {
  A: { North: 4, South: 3, East: 2 },
  B: { North: 7, South: 2, East: 5 },
  C: { North: 3, South: 6, East: 1 },
  D: { North: 8, South: 4, East: 3 },
  E: { North: 5, South: 7, East: 4 },
};

type Frame = { x0: number; x1: number; y0: number; y1: number; padding: number };

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🖼️ The plot rectangle the frame row describes, in the millimetres the family draws in. */
function frame(ctx: AdapterContext): Frame {
  const row = rows(ctx)[0]!;
  return {
    x0: Number(row.padLeft),
    x1: Number(row.width) - Number(row.padRight),
    y0: Number(row.padBottom),
    y1: Number(row.height) - Number(row.padTop),
    padding: Number(row.padding),
  };
}

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Compiles one committed fixture of this case and projects its rectangles. */
async function rectangles(ctx: AdapterContext, fixture: string): Promise<number[]> {
  const records = await compileVizProbe(ctx.fixture(`shared://📊️charts-bar-layout/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  const projection: ProbeProjection = probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id);
  return (projection["geometry/rect"] ?? []).map(Number);
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ Two nested d3 band scales plus a linear value scale — a grouped bar chart's whole geometry. */
function groupedRects(f: Frame, horizontal: boolean): number[] {
  const band = horizontal
    ? scaleBand<string>().domain([...CATEGORIES]).range([f.y1, f.y0])
    : scaleBand<string>().domain([...CATEGORIES]).range([f.x0, f.x1]);
  const value = scaleLinear().domain([0, 8]).range(horizontal ? [f.x0, f.x1] : [f.y0, f.y1]);
  const inner = Math.abs(band.bandwidth()) * (1 - f.padding);
  const slot = inner / SERIES.length;
  const out: number[] = [];
  for (const category of CATEGORIES) {
    const centre = band(category)! + band.bandwidth() / 2;
    SERIES.forEach((series, index) => {
      const offset = centre - inner / 2 + index * slot;
      const length = value(VALUES[category]![series]!) - value(0);
      out.push(...(horizontal ? [value(0), offset, length, slot] : [offset, value(0), slot, length]).map(grid));
    });
  }
  return out;
}

/** 🔮️ d3-shape's stack, with or without the expand offset, mapped through the value scale. */
function stackedRects(f: Frame, expand: boolean): number[] {
  const table = CATEGORIES.map((category) => VALUES[category]!);
  const layout = stack<Record<string, number>>().keys([...SERIES]);
  if (expand) layout.offset(stackOffsetExpand);
  const series = layout(table);
  const total = Math.max(...table.map((row) => SERIES.reduce((sum, key) => sum + row[key]!, 0)));
  const band = scaleBand<string>().domain([...CATEGORIES]).range([f.x0, f.x1]);
  const value = scaleLinear().domain(expand ? [0, 1] : [0, total]).range([f.y0, f.y1]);
  const inner = band.bandwidth() * (1 - f.padding);
  const out: number[] = [];
  CATEGORIES.forEach((category, row) => {
    const left = band(category)! + band.bandwidth() / 2 - inner / 2;
    SERIES.forEach((_, key) => {
      const [lower, upper] = series[key]![row]!;
      out.push(...[left, value(lower!), inner, value(upper!) - value(lower!)].map(grid));
    });
  });
  return out;
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "grouped": {
      /** 🔮️ d3-scale: the outer category band, the inner series band, the linear value scale. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/rect": groupedRects(frame(ctx), false) } }),
      /** 🎯️ The rectangles the bar family actually drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": await rectangles(ctx, "grouped.tex") } }),
    },
    "horizontal": {
      /** 🔮️ The same construction with the band running down the y range instead. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/rect": groupedRects(frame(ctx), true) } }),
      /** 🎯️ The rectangles the horizontally oriented bar family drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": await rectangles(ctx, "horizontal.tex") } }),
    },
    "stacked": {
      /** 🔮️ d3-shape's stack with the default offset. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/rect": stackedRects(frame(ctx), false) } }),
      /** 🎯️ The stacked rectangles the bar family drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": await rectangles(ctx, "stacked.tex") } }),
    },
    "percent": {
      /** 🔮️ d3-shape's stack under stackOffsetExpand. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/rect": stackedRects(frame(ctx), true) } }),
      /** 🎯️ The percent-stacked rectangles the bar family drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": await rectangles(ctx, "percent.tex") } }),
    },
    "diverging": {
      /** 📐️ The specification vectors — d3 splits a diverging stack by sign, the family by position. */
      oracle: (ctx: AdapterContext) => ({
        projection: {
          "geometry/rect": rows(ctx).flatMap((row) => [Number(row.x), Number(row.y), Number(row.w), Number(row.h)].map(grid)),
        },
      }),
      /** 🎯️ The rectangles the diverging mode drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": await rectangles(ctx, "diverging.tex") } }),
    },
  },
});
// #endregion 🧭️Adapter
