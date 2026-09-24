// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { bin, deviation, quantileSorted, ticks as d3ticks } from "d3-array";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-histogram-density";
const DECIMALS = 9;

/** 🧪️ `demo-distribution` of `semio-viz-data`, in the row order the table declares. */
const SAMPLE = [
  4.2, 5.1, 4.8, 6.3, 5.6, 5.9, 6.8, 4.5, 7.1, 5.4, 6.0, 5.2, 6.6, 4.9, 5.8, 7.4, 6.1, 5.5, 4.4, 6.9,
  8.3, 7.6, 9.1, 8.8, 7.9, 8.1, 9.6, 7.2, 8.5, 9.3, 8.0, 7.7, 8.9, 9.8, 8.2, 7.4, 8.6, 9.0, 7.8, 8.4,
  3.1, 2.6, 3.8, 2.9, 3.4, 2.2, 3.6, 2.8,
] as const;

const SORTED = [...SAMPLE].sort((a, b) => a - b);

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`shared://📶️charts-histogram-density/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ d3-array's bin with a threshold count, split into the three projected sequences. */
function bins(count: number): { projection: ProbeProjection } {
  const layout = bin().thresholds(count)([...SAMPLE]);
  return {
    "bin-lower": layout.map((b) => grid(b.x0!)),
    "bin-upper": layout.map((b) => grid(b.x1!)),
    "bin-count": layout.map((b) => b.length),
  };
}

/** 🔮️ Silverman's rule of thumb on d3-array's deviation and quartiles. */
function bandwidth(): number {
  const iqr = (quantileSorted(SORTED, 0.75)! - quantileSorted(SORTED, 0.25)!) / 1.349;
  return 1.06 * Math.min(deviation(SAMPLE)!, iqr) * SAMPLE.length ** -0.2;
}

/** 🔮️ A gaussian kernel density estimate of the sample on a regular grid. */
function density(h: number, samples: number, lo: number, hi: number): number[] {
  return Array.from({ length: samples }, (_, i) => lo + ((hi - lo) * i) / (samples - 1)).map((x) =>
    grid(SAMPLE.reduce((sum, xi) => sum + Math.exp(-0.5 * ((x - xi) / h) ** 2), 0) / (SAMPLE.length * h * Math.sqrt(2 * Math.PI))),
  );
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "bins": {
      /** 🔮️ d3-array: nice, ticks, the threshold rescue and the bisect-right placement. */
      oracle: () => ({ projection: bins(10) }),
      /** 🎯️ The bins `\semio_viz_stat_bin:Nn` produced for the same sample. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "bins.tex")),
    },
    "bins-coarse": {
      /** 🔮️ The same construction at four thresholds, where the last threshold is popped. */
      oracle: () => ({ projection: bins(4) }),
      /** 🎯️ The four-threshold bins the kernel produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "bins-coarse.tex")),
    },
    "ticks": {
      /** 🔮️ d3-array's ticks on an integer, a fractional and a sub-unit step. */
      oracle: () => ({
        projection: {
          "ticks-a": d3ticks(0, 12, 10).map(grid),
          "ticks-b": d3ticks(2.4, 11.3, 7).map(grid),
          "ticks-c": d3ticks(0, 1, 5).map(grid),
        },
      }),
      /** 🎯️ The ticks `\semio_viz_stat_ticks:nnn` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "ticks.tex")),
    },
    "density": {
      /** 🔮️ Silverman's bandwidth on d3-array's spread estimates plus the gaussian sum. */
      oracle: () => {
        const h = bandwidth();
        return {
          projection: {
            "bandwidth": [grid(h)],
            "grid": Array.from({ length: 9 }, (_, i) => grid(2 + i)),
            "density": density(h, 9, 2, 10),
            "fixed-density": density(0.75, 9, 2, 10),
          },
        };
      },
      /** 🎯️ The bandwidth and curve `\semio_viz_stat_kde:Nnnnn` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "density.tex")),
    },
    "ecdf": {
      /** 🔮️ The sorted sample and its i/n plotting positions. */
      oracle: () => ({
        projection: {
          "ecdf-x": SORTED.map(grid),
          "ecdf-y": SORTED.map((_, i) => grid((i + 1) / SORTED.length)),
        },
      }),
      /** 🎯️ The steps `\semio_viz_stat_ecdf:N` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "ecdf.tex")),
    },
  },
});
// #endregion 🧭️Adapter
