// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleBand, scaleOrdinal, scalePoint, scaleQuantile, scaleQuantize, scaleThreshold } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "scale-discrete";
const FIXTURE = "local://scale-discrete.tex";
const DECIMALS = 6;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

/** 📦️ Starts, bandwidth and step of one d3 band scale, in the probe's key layout. */
function bandProjection(name: string, scale: { (value: string): number | undefined; bandwidth(): number; step(): number }, domain: readonly string[]): Record<string, (number | string)[]> {
  return {
    [`band/${name}`]: grid(domain.map((entry) => scale(entry) ?? 0)),
    [`metrics/${name}`]: grid([scale.bandwidth(), scale.step()]),
  };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    band: {
      /** 🔮️ d3-scale's band scale under four padding, alignment and direction settings. */
      oracle: () => ({
        projection: {
          ...bandProjection("plain", scaleBand<string>().domain(["A", "B", "C", "D", "E"]).range([0, 100]), ["A", "B", "C", "D", "E"]),
          ...bandProjection("padded", scaleBand<string>().domain(["A", "B", "C", "D"]).range([0, 120]).paddingInner(0.1).paddingOuter(0.2), ["A", "B", "C", "D"]),
          ...bandProjection("aligned", scaleBand<string>().domain(["A", "B", "C"]).range([0, 90]).paddingInner(0.5).paddingOuter(0.5).align(0), ["A", "B", "C"]),
          ...bandProjection("reversed", scaleBand<string>().domain(["A", "B", "C", "D"]).range([120, 0]).paddingInner(0.2), ["A", "B", "C", "D"]),
        },
      }),
      /** 🎯️ The same band scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    point: {
      /** 🔮️ d3-scale's point scale with and without outer padding. */
      oracle: () => {
        const plain = scalePoint<string>().domain(["A", "B", "C", "D"]).range([0, 120]);
        const padded = scalePoint<string>().domain(["A", "B", "C", "D"]).range([0, 120]).padding(0.5);
        return {
          projection: {
            "point/pt": grid(["A", "B", "C", "D"].map((entry) => plain(entry) ?? 0)),
            "metrics/pt": grid([plain.bandwidth(), plain.step()]),
            "point/ptpad": grid(["A", "B", "C", "D"].map((entry) => padded(entry) ?? 0)),
            "metrics/ptpad": grid([padded.bandwidth(), padded.step()]),
          },
        };
      },
      /** 🎯️ The same point scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    ordinal: {
      /** 🔮️ d3-scale's ordinal scale cycling a shorter range. */
      oracle: () => {
        const scale = scaleOrdinal<string, number>().domain(["A", "B", "C", "D", "E"]).range([10, 20, 30]);
        return { projection: { "ordinal/ord": ["A", "B", "C", "D", "E"].map((entry) => scale(entry)) } };
      },
      /** 🎯️ The same ordinal scale from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    quantize: {
      /** 🔮️ d3-scale's quantize scale and its derived thresholds. */
      oracle: () => {
        const plain = scaleQuantize<number>().domain([0, 1]).range([1, 2, 3, 4]);
        const wide = scaleQuantize<string>().domain([-10, 10]).range(["a", "b", "c"]);
        return {
          projection: {
            "quantize/qz": [-1, 0, 0.1, 0.25, 0.5, 0.6, 0.9, 1, 2].map((value) => plain(value)),
            "thresholds/qz": grid(plain.thresholds()),
            "quantize/qzwide": [-10, -4, 0, 4, 10].map((value) => wide(value)),
            "thresholds/qzwide": grid(wide.thresholds()),
          },
        };
      },
      /** 🎯️ The same quantize scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    quantile: {
      /** 🔮️ d3-scale's quantile scale and its R-7 quantiles. */
      oracle: () => {
        const scale = scaleQuantile<number>().domain([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).range([10, 20, 30, 40]);
        return {
          projection: {
            "quantile/ql": [1, 3, 5, 7, 9, 10].map((value) => scale(value)),
            "thresholds/ql": grid(scale.quantiles()),
          },
        };
      },
      /** 🎯️ The same quantile scale from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    threshold: {
      /** 🔮️ d3-scale's threshold scale over its explicit cuts. */
      oracle: () => {
        const scale = scaleThreshold<number, string>().domain([0, 1]).range(["low", "mid", "high"]);
        return {
          projection: {
            "threshold/th": [-1, 0, 0.5, 1, 2].map((value) => scale(value)),
            "thresholds/th": [0, 1],
          },
        };
      },
      /** 🎯️ The same threshold scale from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
