// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { stack, stackOffsetDiverging, stackOffsetExpand, stackOffsetSilhouette, stackOrderAscending, stackOrderNone, stackOrderReverse, type Series } from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "transform-stack";
const FIXTURE = "local://transform-stack.tex";
const DECIMALS = 6;

type Row = Record<string, number>;

/** 🧫️ The shipped `demo-series` table in wide form, which is what d3's stack consumes. */
const KEYS = ["alpha", "beta", "gamma"] as const;
const SERIES: Readonly<Record<string, readonly number[]>> = {
  alpha: [4, 7, 6, 9, 8, 12, 11, 14],
  beta: [2, 3, 5, 4, 7, 6, 9, 8],
  gamma: [6, 5, 3, 5, 2, 4, 3, 5],
};
const ROWS: Row[] = SERIES.alpha!.map((_, index) => Object.fromEntries([["t", index + 1], ...KEYS.map((key) => [key, SERIES[key]![index]!])])) as Row[];

const SIGNED_KEYS = ["up", "down", "mixed"] as const;
const SIGNED: Row[] = [
  { t: 1, up: 3, down: -2, mixed: 1 },
  { t: 2, up: 4, down: -5, mixed: -3 },
  { t: 3, up: 2, down: -1, mixed: 4 },
];

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🥞️ One d3 stack over the demo series with the given order and offset. */
function stacked(order: (series: Series<Row, string>) => number[], offset?: (series: Series<Row, string>, order: readonly number[]) => void): Series<Row, string> {
  const generator = stack<Row>().keys([...KEYS]).order(order);
  if (offset !== undefined) generator.offset(offset);
  return generator(ROWS);
}

/** 🥞️ The baselines of one series of a stack, in the probe's key layout. */
function baselines(series: Series<Row, string>, key: string, prefix: string, side: 0 | 1): Record<string, (number | string)[]> {
  const found = series.find((entry) => entry.key === key)!;
  return { [`${prefix}/${key}/y${side}`]: grid(found.map((point) => point[side])) };
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
    "order-none": {
      /** 🔮️ d3-shape's stack with the identity order and the zero baseline. */
      oracle: () => {
        const series = stacked(stackOrderNone);
        return {
          projection: {
            ...baselines(series, "alpha", "stack/none", 0),
            ...baselines(series, "alpha", "stack/none", 1),
            ...baselines(series, "beta", "stack/none", 0),
            ...baselines(series, "beta", "stack/none", 1),
            ...baselines(series, "gamma", "stack/none", 0),
            ...baselines(series, "gamma", "stack/none", 1),
            "stack/none/order": stackOrderNone(series).map((index) => KEYS[index]!),
          },
        };
      },
      /** 🎯️ The same stack from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "order-ascending": {
      /** 🔮️ d3-shape's stackOrderAscending, with the stacking sequence read off separately. */
      oracle: () => {
        const series = stacked(stackOrderAscending);
        return {
          projection: {
            "stack/ascending/order": stackOrderAscending(series).map((index) => KEYS[index]!),
            ...baselines(series, "alpha", "stack/ascending", 0),
            ...baselines(series, "gamma", "stack/ascending", 1),
          },
        };
      },
      /** 🎯️ The same stack from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "order-reverse": {
      /** 🔮️ d3-shape's stackOrderReverse. */
      oracle: () => {
        const series = stacked(stackOrderReverse);
        return {
          projection: {
            "stack/reverse/order": stackOrderReverse(series).map((index) => KEYS[index]!),
            ...baselines(series, "gamma", "stack/reverse", 1),
            ...baselines(series, "alpha", "stack/reverse", 0),
          },
        };
      },
      /** 🎯️ The same stack from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "offset-expand": {
      /** 🔮️ d3-shape's stackOffsetExpand. */
      oracle: () => {
        const series = stacked(stackOrderNone, stackOffsetExpand);
        return {
          projection: {
            ...baselines(series, "alpha", "stack/expand", 1),
            ...baselines(series, "beta", "stack/expand", 0),
            ...baselines(series, "gamma", "stack/expand", 1),
          },
        };
      },
      /** 🎯️ The same stack from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "offset-silhouette": {
      /** 🔮️ d3-shape's stackOffsetSilhouette. */
      oracle: () => {
        const series = stacked(stackOrderNone, stackOffsetSilhouette);
        return {
          projection: {
            ...baselines(series, "alpha", "stack/silhouette", 0),
            ...baselines(series, "gamma", "stack/silhouette", 1),
          },
        };
      },
      /** 🎯️ The same stack from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "offset-diverging": {
      /** 🔮️ d3-shape's stackOffsetDiverging over a series that changes sign. */
      oracle: () => {
        const series = stack<Row>().keys([...SIGNED_KEYS]).offset(stackOffsetDiverging)(SIGNED);
        const projection: Record<string, (number | string)[]> = {};
        for (const key of SIGNED_KEYS) {
          Object.assign(projection, baselines(series, key, "stack/diverging", 0));
          Object.assign(projection, baselines(series, key, "stack/diverging", 1));
        }
        return { projection };
      },
      /** 🎯️ The same stack from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
