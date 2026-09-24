// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleLinear } from "d3-scale";
import { curveLinear, curveStepAfter, curveStepBefore, line as d3Line, stack, type CurveFactory } from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-line-area";
const DECIMALS = 4;

/** 🧪️ `demo-multiseries` of `semio-viz-charts-bar`, in the row order the table declares. */
const TIME = [1, 2, 3, 4, 5, 6];
const ALPHA = [3, 5, 4, 8, 7, 10];
const BETA = [6, 4, 7, 5, 9, 6];

type Frame = { x0: number; x1: number; y0: number; y1: number };

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function frame(ctx: AdapterContext): Frame {
  const row = rows(ctx)[0]!;
  return {
    x0: Number(row.padLeft),
    x1: Number(row.width) - Number(row.padRight),
    y0: Number(row.padBottom),
    y1: Number(row.height) - Number(row.padTop),
  };
}

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Every `geometry/path` record of one compiled fixture, in emission order. */
async function paths(ctx: AdapterContext, fixture: string): Promise<number[][]> {
  const records = await compileVizProbe(ctx.fixture(`shared://📈️charts-line-area/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return roundProbeNumbers(records, DECIMALS)
    .filter((record) => record.key === "geometry/path")
    .map((record) => record.values.map(Number));
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ The vertex list d3-shape's own line generator writes for one curve. */
function vertices(values: readonly number[], curve: CurveFactory, f: Frame): number[] {
  const x = scaleLinear().domain([TIME[0]!, TIME[TIME.length - 1]!]).range([f.x0, f.x1]);
  const y = scaleLinear().domain([0, 10]).range([f.y0, f.y1]);
  const path = d3Line<number>().x((_, index) => x(TIME[index]!)).y((value) => y(value)).curve(curve)([...values]);
  if (path === null) throw new Error("d3-shape produced no path");
  return path
    .replace(/^M/, "")
    .split("L")
    .flatMap((point) => point.split(",").map(Number))
    .map(grid);
}

/** 🔮️ The layer boundaries d3-shape's stack produces, mapped through the value scale. */
function layerBoundaries(f: Frame): { upper: number[][]; lower: number[][] } {
  const table = TIME.map((_, index) => ({ Alpha: ALPHA[index]!, Beta: BETA[index]! }));
  const series = stack<Record<string, number>>().keys(["Alpha", "Beta"])(table);
  const total = Math.max(...table.map((row) => row.Alpha + row.Beta));
  const x = scaleLinear().domain([TIME[0]!, TIME[TIME.length - 1]!]).range([f.x0, f.x1]);
  const y = scaleLinear().domain([0, total]).range([f.y0, f.y1]);
  const boundary = (key: number, edge: 0 | 1): number[] =>
    series[key]!.flatMap((point, index) => [grid(x(TIME[index]!)), grid(y(point[edge]!))]);
  return { upper: [boundary(0, 1), boundary(1, 1)], lower: [boundary(0, 0), boundary(1, 0)] };
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
/** 🧵 One projection key per series, so a wrong series is a named failure. */
const seriesProjection = (lists: number[][]): Record<string, number[]> => ({
  "vertices/alpha": lists[0] ?? [],
  "vertices/beta": lists[1] ?? [],
});

const curveScenario = (fixture: string, curve: CurveFactory) => ({
  oracle: (ctx: AdapterContext) => ({
    projection: seriesProjection([vertices(ALPHA, curve, frame(ctx)), vertices(BETA, curve, frame(ctx))]),
  }),
  subject: async (ctx: AdapterContext) => ({ projection: seriesProjection(await paths(ctx, fixture)) }),
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    /** 🔮️/🎯️ d3-shape's curveLinear against the vertices the line family emitted. */
    "linear": curveScenario("linear.tex", curveLinear),
    /** 🔮️/🎯️ d3-shape's curveStepAfter against the vertices the line family emitted. */
    "step-after": curveScenario("step-after.tex", curveStepAfter),
    /** 🔮️/🎯️ d3-shape's curveStepBefore against the vertices the line family emitted. */
    "step-before": curveScenario("step-before.tex", curveStepBefore),
    "area-stack": {
      /** 🔮️ d3-shape's stack boundaries for both layers. */
      oracle: (ctx: AdapterContext) => {
        const { upper, lower } = layerBoundaries(frame(ctx));
        return {
          projection: {
            "layer/alpha/upper": upper[0]!,
            "layer/alpha/lower": lower[0]!,
            "layer/beta/upper": upper[1]!,
          },
        };
      },
      /** 🎯️ The band boundaries the area family drew: upper, lower, upper again, per layer. */
      subject: async (ctx: AdapterContext) => {
        const lists = await paths(ctx, "area-stack.tex");
        return {
          projection: {
            "layer/alpha/upper": lists[0] ?? [],
            "layer/alpha/lower": lists[1] ?? [],
            "layer/beta/upper": lists[3] ?? [],
          },
        };
      },
    },
  },
});
// #endregion 🧭️Adapter
