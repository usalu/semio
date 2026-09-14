// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { scaleLinear, scaleLog, scalePow, scaleSqrt, scaleSymlog, niceDomain } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../📐️scale-continuous/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔮️ The oracle of `scale-continuous`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`scale-continuous declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    linear: {
      oracle: oracle("linear"),
      /** 🎯️ The twin's linear scale, with and without clamping, forwards and inverted. */
      subject: () => {
        const lin = scaleLinear([0, 100], [0, 180]);
        const linrev = scaleLinear([-5, 5], [100, 0]);
        const clamped = scaleLinear([0, 10], [0, 1], { clamp: true });
        return {
          projection: {
            "map/lin": grid([0, 25, 42, 100, 150].map((value) => lin(value))),
            "map/linrev": grid([-5, 0, 2.5, 5].map((value) => linrev(value))),
            "map/linclamp": grid([-3, 0, 5, 10, 17].map((value) => clamped(value))),
            "invert/lin": grid([0, 45, 75.6, 180].map((value) => lin.invert!(value))),
            "ticks/lin": grid(lin.ticks!(10)),
            "ticks/linrev": grid(linrev.ticks!(5)),
          },
        };
      },
    },
    log: {
      oracle: oracle("log"),
      /** 🎯️ The twin's logarithmic scale and its decade ticks. */
      subject: () => {
        const lg = scaleLog([1, 1000], [0, 300]);
        const small = scaleLog([0.001, 1], [0, 100]);
        return {
          projection: {
            "map/lg": grid([1, 10, 100, 1000, 42].map((value) => lg(value))),
            "invert/lg": grid([0, 100, 200, 300].map((value) => lg.invert!(value))),
            "ticks/lg": grid(lg.ticks!(10)),
            "ticks/lgsmall": grid(small.ticks!(10)),
          },
        };
      },
    },
    pow: {
      oracle: oracle("pow"),
      /** 🎯️ The twin's power and square-root scales. */
      subject: () => {
        const pw = scalePow([0, 10], [0, 100], { exponent: 2 });
        const half = scalePow([0, 16], [0, 64], { exponent: 0.5 });
        const sq = scaleSqrt([0, 100], [0, 10]);
        return {
          projection: {
            "map/pw": grid([0, 2.5, 5, 7.5, 10].map((value) => pw(value))),
            "map/pwhalf": grid([0, 1, 4, 9, 16].map((value) => half(value))),
            "map/sq": grid([0, 25, 50, 75, 100].map((value) => sq(value))),
            "invert/pw": grid([0, 25, 64, 100].map((value) => pw.invert!(value))),
            "invert/sq": grid([0, 5, 7.071068, 10].map((value) => sq.invert!(value))),
          },
        };
      },
    },
    symlog: {
      oracle: oracle("symlog"),
      /** 🎯️ The twin's symmetric-logarithmic scale at two linear-region constants. */
      subject: () => {
        const sl = scaleSymlog([-100, 100], [0, 200]);
        const constant = scaleSymlog([-100, 100], [0, 200], { constant: 10 });
        return {
          projection: {
            "map/sl": grid([-100, -10, -1, 0, 1, 10, 100].map((value) => sl(value))),
            "map/slc": grid([-100, -10, 0, 10, 100].map((value) => constant(value))),
            "invert/sl": grid([0, 48.042629, 100, 151.957371, 200].map((value) => sl.invert!(value))),
            "ticks/sl": grid(sl.ticks!(5)),
          },
        };
      },
    },
    nice: {
      oracle: oracle("nice"),
      /** 🎯️ The domains the twin's `niceDomain` rounds four raw domains onto. */
      subject: () => ({
        projection: {
          "nice/n1": grid(niceDomain([0.1, 0.9], 10)),
          "nice/n2": grid(niceDomain([1.1, 10.9], 10)),
          "nice/n3": grid(niceDomain([-0.5, 17.3], 5)),
          "nice/n4": grid(niceDomain([12, 87], 4)),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
