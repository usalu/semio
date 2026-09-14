// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { bin } from "d3-array";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "transform-bin";
const FIXTURE = "local://transform-bin.tex";
const DECIMALS = 6;

/** 🧫️ The `value` column of the shipped `demo-distribution` table. */
const SAMPLE: readonly number[] = [
  4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9, 8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4, 3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8,
];

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🪣️ One d3 histogram, split into the three columns the transform writes. */
function histogram(prefix: string, thresholds?: number | readonly number[]): Record<string, (number | string)[]> {
  const generator = bin<number, number>();
  if (typeof thresholds === "number") generator.thresholds(thresholds);
  else if (thresholds !== undefined) generator.thresholds([...thresholds]);
  const bins = generator([...SAMPLE]);
  return {
    [`bin/${prefix}/x0`]: grid(bins.map((entry) => entry.x0 ?? 0)),
    [`bin/${prefix}/x1`]: grid(bins.map((entry) => entry.x1 ?? 0)),
    [`bin/${prefix}/count`]: bins.map((entry) => entry.length),
  };
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
    sturges: {
      /** 🔮️ d3-array's bin with its default Sturges threshold count. */
      oracle: () => ({ projection: histogram("sturges") }),
      /** 🎯️ The same binning from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "count-five": {
      /** 🔮️ d3-array's bin with a requested count of five. */
      oracle: () => ({ projection: histogram("five", 5) }),
      /** 🎯️ The same binning from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "count-twenty": {
      /** 🔮️ d3-array's bin with a requested count of twenty. */
      oracle: () => ({ projection: histogram("twenty", 20) }),
      /** 🎯️ The same binning from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "explicit-thresholds": {
      /** 🔮️ d3-array's bin with an explicit threshold list. */
      oracle: () => ({ projection: histogram("explicit", [4, 6, 8]) }),
      /** 🎯️ The same binning from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
