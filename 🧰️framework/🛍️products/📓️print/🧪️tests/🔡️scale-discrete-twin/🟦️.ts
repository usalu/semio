// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { scaleBand, scaleOrdinal, scalePoint, scaleQuantile, scaleQuantize, scaleThreshold, type VizScale } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🔠️scale-discrete/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔮️ The oracle of `scale-discrete`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`scale-discrete declares no oracle for ${id}`);
  return handler;
}

/** 📦️ Starts, bandwidth and step of one twin band scale, in the probe's key layout. */
function bandProjection(name: string, scale: VizScale<string, number>, domain: readonly string[]): Record<string, number[]> {
  return {
    [`band/${name}`]: grid(domain.map((entry) => scale(entry))),
    [`metrics/${name}`]: grid([scale.bandwidth!(), scale.step!()]),
  };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    band: {
      oracle: oracle("band"),
      /** 🎯️ The twin's band scale under four padding, alignment and direction settings. */
      subject: () => ({
        projection: {
          ...bandProjection("plain", scaleBand(["A", "B", "C", "D", "E"], [0, 100]), ["A", "B", "C", "D", "E"]),
          ...bandProjection("padded", scaleBand(["A", "B", "C", "D"], [0, 120], { paddingInner: 0.1, paddingOuter: 0.2 }), ["A", "B", "C", "D"]),
          ...bandProjection("aligned", scaleBand(["A", "B", "C"], [0, 90], { paddingInner: 0.5, paddingOuter: 0.5, align: 0 }), ["A", "B", "C"]),
          ...bandProjection("reversed", scaleBand(["A", "B", "C", "D"], [120, 0], { paddingInner: 0.2 }), ["A", "B", "C", "D"]),
        },
      }),
    },
    point: {
      oracle: oracle("point"),
      /** 🎯️ The twin's point scale with and without outer padding. */
      subject: () => {
        const plain = scalePoint(["A", "B", "C", "D"], [0, 120]);
        const padded = scalePoint(["A", "B", "C", "D"], [0, 120], { padding: 0.5 });
        return {
          projection: {
            "point/pt": grid(["A", "B", "C", "D"].map((entry) => plain(entry))),
            "metrics/pt": grid([plain.bandwidth!(), plain.step!()]),
            "point/ptpad": grid(["A", "B", "C", "D"].map((entry) => padded(entry))),
            "metrics/ptpad": grid([padded.bandwidth!(), padded.step!()]),
          },
        };
      },
    },
    ordinal: {
      oracle: oracle("ordinal"),
      /** 🎯️ The twin's ordinal scale cycling a shorter range. */
      subject: () => {
        const scale = scaleOrdinal(["A", "B", "C", "D", "E"], [10, 20, 30]);
        return { projection: { "ordinal/ord": ["A", "B", "C", "D", "E"].map((entry) => scale(entry)) } };
      },
    },
    quantize: {
      oracle: oracle("quantize"),
      /** 🎯️ The twin's quantize scale and its derived thresholds. */
      subject: () => {
        const plain = scaleQuantize([0, 1], [1, 2, 3, 4]);
        const wide = scaleQuantize([-10, 10], ["a", "b", "c"]);
        return {
          projection: {
            "quantize/qz": [-1, 0, 0.1, 0.25, 0.5, 0.6, 0.9, 1, 2].map((value) => plain(value)),
            "thresholds/qz": grid(plain.thresholds()),
            "quantize/qzwide": [-10, -4, 0, 4, 10].map((value) => wide(value)),
            "thresholds/qzwide": grid(wide.thresholds()),
          },
        };
      },
    },
    quantile: {
      oracle: oracle("quantile"),
      /** 🎯️ The twin's quantile scale and its R-7 quantiles. */
      subject: () => {
        const scale = scaleQuantile([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], [10, 20, 30, 40]);
        return {
          projection: {
            "quantile/ql": [1, 3, 5, 7, 9, 10].map((value) => scale(value)),
            "thresholds/ql": grid(scale.thresholds()),
          },
        };
      },
    },
    threshold: {
      oracle: oracle("threshold"),
      /** 🎯️ The twin's threshold scale over its explicit cuts. */
      subject: () => {
        const scale = scaleThreshold([0, 1], ["low", "mid", "high"]);
        return {
          projection: {
            "threshold/th": [-1, 0, 0.5, 1, 2].map((value) => scale(value)),
            "thresholds/th": grid(scale.domain()),
          },
        };
      },
    },
  },
});
// #endregion 🧭️Adapter
