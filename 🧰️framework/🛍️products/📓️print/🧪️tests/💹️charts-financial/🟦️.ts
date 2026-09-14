// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { mean } from "d3-array";
import { scaleLinear } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-financial";
const DECIMALS = 4;

/** 🖼️ The canvas and padding the fixtures use; the family's own defaults. */
const FRAME = { x0: 10, x1: 76, y0: 8, y1: 36 } as const;
/** 📏 The share of one period step a candle body occupies. */
const BODY_SHARE = 0.62;
/** 📏 The minimum body height, so that a doji is drawn rather than lost. */
const MIN_BODY = 0.4;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Every record of one compiled fixture that carries the given key. */
async function records(ctx: AdapterContext, fixture: string, key: string): Promise<number[][]> {
  const parsed = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return roundProbeNumbers(parsed, DECIMALS)
    .filter((record) => record.key === key)
    .map((record) => record.values.map(Number));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "candlestick": {
      /** 🔮️ d3-scale over the period and price extents, then the body rectangle of each candle. */
      oracle: (ctx: AdapterContext) => {
        const periods = rows(ctx);
        const times = periods.map((row) => Number(row.t));
        const lows = periods.map((row) => Number(row.low));
        const highs = periods.map((row) => Number(row.high));
        const x = scaleLinear().domain([Math.min(...times), Math.max(...times)]).range([FRAME.x0, FRAME.x1]);
        const y = scaleLinear().domain([Math.min(...lows), Math.max(...highs)]).range([FRAME.y0, FRAME.y1]);
        const width = ((FRAME.x1 - FRAME.x0) / periods.length) * BODY_SHARE;
        const projection = periods.flatMap((row) => {
          const open = y(Number(row.open));
          const close = y(Number(row.close));
          return [x(Number(row.t)) - width / 2, Math.min(open, close), width, Math.max(MIN_BODY, Math.abs(close - open))].map(grid);
        });
        return { projection: { "candle/body": projection } };
      },
      /** 🎯️ The candle bodies the financial family drew. */
      subject: async (ctx: AdapterContext) => ({
        projection: { "candle/body": (await records(ctx, "candlestick.tex", "geometry/rect")).flat() },
      }),
    },
    "moving-average": {
      /** 🔮️ d3-array's mean over the same trailing windows. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const closes = row.closes!.split(",").map(Number);
        const window = Number(row.window);
        const means = closes.map((_, index) => grid(mean(closes.slice(Math.max(0, index - window + 1), index + 1)) ?? 0));
        return { projection: { "overlay/window-mean": means } };
      },
      /** 🎯️ The window means the overlay computed. */
      subject: async (ctx: AdapterContext) => ({
        projection: { "overlay/window-mean": (await records(ctx, "moving-average.tex", "geometry/window-mean"))[0] ?? [] },
      }),
    },
  },
});
// #endregion 🧭️Adapter
