// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { pie as d3pie } from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-pie-donut";
const DECIMALS = 9;
const TAU = 2 * Math.PI;

/** 🧪️ The `share` column of `demo-parts` of `semio-viz-charts-distribution`. */
const SHARES = [34, 26, 18, 12, 7, 3] as const;

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`shared://🥧️charts-pie-donut/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ d3-shape's pie, split into the two angle sequences the probe emits. */
function angles(start: number, end: number, pad: number, sorted: boolean): { start: number[]; end: number[] } {
  const layout = d3pie<number>().value((d) => d).startAngle(start).endAngle(end).padAngle(pad);
  if (sorted) layout.sort((a, b) => b - a);
  else layout.sort(null);
  const arcs = layout([...SHARES]);
  return { start: arcs.map((a) => grid(a.startAngle)), end: arcs.map((a) => grid(a.endAngle)) };
}

/** 🔮️ The annular sectors a donut of the given frame draws for those angles. */
function annulus(width: number, height: number, pad: number, inner: number): number[] {
  const cx = width / 2;
  const cy = height / 2;
  const radius = Math.min(width - 2 * pad, height - 2 * pad) / 2;
  const { start, end } = angles(0, TAU, 0, true);
  return start.flatMap((a, i) => [cx, cy, grid(radius * inner), grid(radius), a, end[i]!].map(Number));
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "pie": {
      /** 🔮️ d3-shape's pie over a full turn, sorted descending by value. */
      oracle: () => {
        const a = angles(0, TAU, 0, true);
        return { projection: { "start-angle": a.start, "end-angle": a.end } };
      },
      /** 🎯️ The angles `\semio_viz_stat_pie:Nnnnn` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "pie.tex")),
    },
    "pie-unsorted": {
      /** 🔮️ The same generator with sorting disabled, so table order is arc order. */
      oracle: () => {
        const a = angles(0, TAU, 0, false);
        return { projection: { "start-angle": a.start, "end-angle": a.end } };
      },
      /** 🎯️ The angles the unsorted transform produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "pie-unsorted.tex")),
    },
    "pie-padded": {
      /** 🔮️ A partial circle with a pad angle, where d3 clamps both the sweep and the pad. */
      oracle: () => {
        const a = angles(0.5, 3.641592653589793, 0.02, true);
        return { projection: { "start-angle": a.start, "end-angle": a.end } };
      },
      /** 🎯️ The angles the padded partial circle produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "pie-padded.tex")),
    },
    "donut": {
      /** 🔮️ Those angles rendered into the annulus the frame row describes. */
      oracle: () => ({ projection: { "geometry/arc": annulus(60, 34, 3, 0.55) } }),
      /** 🎯️ The `geometry/arc` records the donut drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/arc": (await probe(ctx, "donut.tex")).projection["geometry/arc"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
