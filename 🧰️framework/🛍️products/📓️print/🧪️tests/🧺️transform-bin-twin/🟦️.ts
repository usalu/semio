// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { bin } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🪣️transform-bin/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🧫️ The `value` column of the shipped `demo-distribution` table. */
const SAMPLE: readonly number[] = [
  4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9, 8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4, 3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8,
];

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🪣️ One twin histogram, split into the three columns the transform writes. */
function histogram(prefix: string, thresholds?: number | readonly number[]): Record<string, (number | string)[]> {
  const bins = bin(SAMPLE, thresholds === undefined ? {} : { thresholds });
  return {
    [`bin/${prefix}/x0`]: grid(bins.map((entry) => entry.x0)),
    [`bin/${prefix}/x1`]: grid(bins.map((entry) => entry.x1)),
    [`bin/${prefix}/count`]: bins.map((entry) => entry.values.length),
  };
}

/** 🔮️ The oracle of `transform-bin`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`transform-bin declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    sturges: {
      oracle: oracle("sturges"),
      /** 🎯️ The twin's bin with its default Sturges threshold count. */
      subject: () => ({ projection: histogram("sturges") }),
    },
    "count-five": {
      oracle: oracle("count-five"),
      /** 🎯️ The twin's bin with a requested count of five. */
      subject: () => ({ projection: histogram("five", 5) }),
    },
    "count-twenty": {
      oracle: oracle("count-twenty"),
      /** 🎯️ The twin's bin with a requested count of twenty. */
      subject: () => ({ projection: histogram("twenty", 20) }),
    },
    "explicit-thresholds": {
      oracle: oracle("explicit-thresholds"),
      /** 🎯️ The twin's bin with an explicit threshold list. */
      subject: () => ({ projection: histogram("explicit", [4, 6, 8]) }),
    },
  },
});
// #endregion 🧭️Adapter
