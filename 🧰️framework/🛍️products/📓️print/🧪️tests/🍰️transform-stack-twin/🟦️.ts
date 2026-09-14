// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizStack, type VizStackOffset, type VizStackOrder, type VizStackSeries } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🥞️transform-stack/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

type Row = Record<string, number>;

/** 🧫️ The shipped `demo-series` table in wide form, which is what the twin's stack consumes. */
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

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🥞️ One twin stack over the demo series with the given order and offset. */
function stacked(order: VizStackOrder, offset?: VizStackOffset): VizStackSeries[] {
  return vizStack(ROWS, [...KEYS], offset === undefined ? { order } : { order, offset });
}

/** 🥞️ The baselines of one series of a stack, in the probe's key layout. */
function baselines(series: readonly VizStackSeries[], key: string, prefix: string, side: 0 | 1): Record<string, (number | string)[]> {
  const found = series.find((entry) => entry.key === key)!;
  return { [`${prefix}/${key}/y${side}`]: grid(found.points.map((point) => point[side])) };
}

/** 🥞️ The sequence the series were laid on top of each other in, read off their stacking positions. */
function sequence(series: readonly VizStackSeries[]): string[] {
  const out = new Array<string>(series.length);
  for (const entry of series) out[entry.index] = entry.key;
  return out;
}

/** 🔮️ The oracle of `transform-stack`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`transform-stack declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "order-none": {
      oracle: oracle("order-none"),
      /** 🎯️ The twin's stack with the identity order and the zero baseline. */
      subject: () => {
        const series = stacked("none");
        return {
          projection: {
            ...baselines(series, "alpha", "stack/none", 0),
            ...baselines(series, "alpha", "stack/none", 1),
            ...baselines(series, "beta", "stack/none", 0),
            ...baselines(series, "beta", "stack/none", 1),
            ...baselines(series, "gamma", "stack/none", 0),
            ...baselines(series, "gamma", "stack/none", 1),
            "stack/none/order": sequence(series),
          },
        };
      },
    },
    "order-ascending": {
      oracle: oracle("order-ascending"),
      /** 🎯️ The twin's ascending order, with the stacking sequence read off separately. */
      subject: () => {
        const series = stacked("ascending");
        return {
          projection: {
            "stack/ascending/order": sequence(series),
            ...baselines(series, "alpha", "stack/ascending", 0),
            ...baselines(series, "gamma", "stack/ascending", 1),
          },
        };
      },
    },
    "order-reverse": {
      oracle: oracle("order-reverse"),
      /** 🎯️ The twin's reverse order. */
      subject: () => {
        const series = stacked("reverse");
        return {
          projection: {
            "stack/reverse/order": sequence(series),
            ...baselines(series, "gamma", "stack/reverse", 1),
            ...baselines(series, "alpha", "stack/reverse", 0),
          },
        };
      },
    },
    "offset-expand": {
      oracle: oracle("offset-expand"),
      /** 🎯️ The twin's expand offset. */
      subject: () => {
        const series = stacked("none", "expand");
        return {
          projection: {
            ...baselines(series, "alpha", "stack/expand", 1),
            ...baselines(series, "beta", "stack/expand", 0),
            ...baselines(series, "gamma", "stack/expand", 1),
          },
        };
      },
    },
    "offset-silhouette": {
      oracle: oracle("offset-silhouette"),
      /** 🎯️ The twin's silhouette offset. */
      subject: () => {
        const series = stacked("none", "silhouette");
        return {
          projection: {
            ...baselines(series, "alpha", "stack/silhouette", 0),
            ...baselines(series, "gamma", "stack/silhouette", 1),
          },
        };
      },
    },
    "offset-diverging": {
      oracle: oracle("offset-diverging"),
      /** 🎯️ The twin's diverging offset over a series that changes sign. */
      subject: () => {
        const series = vizStack(SIGNED, [...SIGNED_KEYS], { offset: "diverging" });
        const projection: Record<string, (number | string)[]> = {};
        for (const key of SIGNED_KEYS) {
          Object.assign(projection, baselines(series, key, "stack/diverging", 0));
          Object.assign(projection, baselines(series, key, "stack/diverging", 1));
        }
        return { projection };
      },
    },
  },
});
// #endregion 🧭️Adapter
