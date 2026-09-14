// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleLinear, scaleLog, scalePow, scaleSqrt, scaleSymlog } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "scale-continuous";
const FIXTURE = "local://scale-continuous.tex";
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
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    linear: {
      /** 🔮️ d3-scale's linear scale, with and without clamping, forwards and inverted. */
      oracle: () => {
        const lin = scaleLinear().domain([0, 100]).range([0, 180]);
        const linrev = scaleLinear().domain([-5, 5]).range([100, 0]);
        const clamped = scaleLinear().domain([0, 10]).range([0, 1]).clamp(true);
        return {
          projection: {
            "map/lin": grid([0, 25, 42, 100, 150].map((value) => lin(value))),
            "map/linrev": grid([-5, 0, 2.5, 5].map((value) => linrev(value))),
            "map/linclamp": grid([-3, 0, 5, 10, 17].map((value) => clamped(value))),
            "invert/lin": grid([0, 45, 75.6, 180].map((value) => lin.invert(value))),
            "ticks/lin": grid(lin.ticks(10)),
            "ticks/linrev": grid(linrev.ticks(5)),
          },
        };
      },
      /** 🎯️ The same scales declared with \SemioVizScale inside a compiled document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    log: {
      /** 🔮️ d3-scale's log scale and its decade ticks. */
      oracle: () => {
        const lg = scaleLog().domain([1, 1000]).range([0, 300]);
        const small = scaleLog().domain([0.001, 1]).range([0, 100]);
        return {
          projection: {
            "map/lg": grid([1, 10, 100, 1000, 42].map((value) => lg(value))),
            "invert/lg": grid([0, 100, 200, 300].map((value) => lg.invert(value))),
            "ticks/lg": grid(lg.ticks(10)),
            "ticks/lgsmall": grid(small.ticks(10)),
          },
        };
      },
      /** 🎯️ The same logarithmic scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    pow: {
      /** 🔮️ d3-scale's power and square-root scales. */
      oracle: () => {
        const pw = scalePow().exponent(2).domain([0, 10]).range([0, 100]);
        const half = scalePow().exponent(0.5).domain([0, 16]).range([0, 64]);
        const sq = scaleSqrt().domain([0, 100]).range([0, 10]);
        return {
          projection: {
            "map/pw": grid([0, 2.5, 5, 7.5, 10].map((value) => pw(value))),
            "map/pwhalf": grid([0, 1, 4, 9, 16].map((value) => half(value))),
            "map/sq": grid([0, 25, 50, 75, 100].map((value) => sq(value))),
            "invert/pw": grid([0, 25, 64, 100].map((value) => pw.invert(value))),
            "invert/sq": grid([0, 5, 7.071068, 10].map((value) => sq.invert(value))),
          },
        };
      },
      /** 🎯️ The same power scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    symlog: {
      /** 🔮️ d3-scale's symmetric-logarithmic scale at two linear-region constants. */
      oracle: () => {
        const sl = scaleSymlog().domain([-100, 100]).range([0, 200]);
        const constant = scaleSymlog().constant(10).domain([-100, 100]).range([0, 200]);
        return {
          projection: {
            "map/sl": grid([-100, -10, -1, 0, 1, 10, 100].map((value) => sl(value))),
            "map/slc": grid([-100, -10, 0, 10, 100].map((value) => constant(value))),
            "invert/sl": grid([0, 48.042629, 100, 151.957371, 200].map((value) => sl.invert(value))),
            "ticks/sl": grid(sl.ticks(5)),
          },
        };
      },
      /** 🎯️ The same symlog scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    nice: {
      /** 🔮️ d3-scale's nice() over four domains and tick counts. */
      oracle: () => ({
        projection: {
          "nice/n1": grid(scaleLinear().domain([0.1, 0.9]).nice(10).domain()),
          "nice/n2": grid(scaleLinear().domain([1.1, 10.9]).nice(10).domain()),
          "nice/n3": grid(scaleLinear().domain([-0.5, 17.3]).nice(5).domain()),
          "nice/n4": grid(scaleLinear().domain([12, 87]).nice(4).domain()),
        },
      }),
      /** 🎯️ The domains the probe's scales carry after their `nice` option ran. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
