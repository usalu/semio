// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { rgb } from "d3-color";
import { interpolateHcl, interpolateLab, interpolateRgb, piecewise } from "d3-interpolate";
import { scaleDiverging, scaleSequential } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "scale-color";
const FIXTURE = "shared://🎨️scale-color/scale-color.tex";
const DECIMALS = 6;
const IDENTITY = (t: number) => t;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🌈️ One interpolated colour in the transport encoding the probe uses. */
function mix(interpolator: (a: string, b: string) => (t: number) => string, from: string, to: string, position: number): string[] {
  return [`|${rgb(interpolator(`#${from}`, `#${to}`)(position)).formatHex()}`];
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

/** 🌈️ Five samples of one two-stop ramp plus a sixth pair, in the probe's key layout. */
function mixProjection(space: string, interpolator: (a: string, b: string) => (t: number) => string, extra: readonly [string, string, number]): Record<string, (number | string)[]> {
  const projection: Record<string, (number | string)[]> = {};
  [0, 0.25, 0.5, 0.75, 1].forEach((position, index) => {
    projection[`mix/${space}/${index}`] = mix(interpolator, "ff344f", "34d1bf", position);
  });
  projection[`mix/${space}/5`] = mix(interpolator, extra[0], extra[1], extra[2]);
  return projection;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "sequential-position": {
      /** 🔮️ d3-scale's sequential scale with the identity interpolator isolates the domain arithmetic. */
      oracle: () => {
        const plain = scaleSequential<number>().domain([0, 100]).interpolator(IDENTITY);
        const clamped = scaleSequential<number>().domain([0, 100]).interpolator(IDENTITY).clamp(true);
        const inputs = [-20, 0, 25, 50, 75, 100, 140];
        return { projection: { "position/seq": grid(inputs.map((value) => plain(value))), "position/seqclamp": grid(inputs.map((value) => clamped(value))) } };
      },
      /** 🎯️ The same sequential scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "diverging-position": {
      /** 🔮️ d3-scale's diverging scale with the identity interpolator. */
      oracle: () => {
        const centred = scaleDiverging<number>().domain([-10, 0, 30]).interpolator(IDENTITY);
        const offset = scaleDiverging<number>().domain([0, 2, 10]).interpolator(IDENTITY);
        return { projection: { "position/div": grid([-10, -5, 0, 15, 30].map((value) => centred(value))), "position/divoff": grid([0, 1, 2, 6, 10].map((value) => offset(value))) } };
      },
      /** 🎯️ The same diverging scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "interpolate-rgb": {
      /** 🔮️ d3-interpolate's sRGB interpolator. */
      oracle: () => ({ projection: mixProjection("rgb", interpolateRgb, ["000000", "ffffff", 0.5]) }),
      /** 🎯️ The same mixes from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "interpolate-lab": {
      /** 🔮️ d3-interpolate's CIE Lab interpolator on the D50 white point. */
      oracle: () => ({ projection: mixProjection("lab", interpolateLab, ["fa9500", "001117", 0.5]) }),
      /** 🎯️ The same mixes from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "interpolate-hcl": {
      /** 🔮️ d3-interpolate's HCL interpolator with shortest-path hue. */
      oracle: () => ({ projection: mixProjection("hcl", interpolateHcl, ["7eb77f", "a60009", 0.5]) }),
      /** 🎯️ The same mixes from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "ramp-stops": {
      /** 🔮️ d3-interpolate's piecewise over three stops, in sRGB and in CIE Lab. */
      oracle: () => {
        const stops = ["#ff344f", "#ffffff", "#34d1bf"];
        const srgb = piecewise(interpolateRgb, stops);
        const lab = piecewise(interpolateLab, stops);
        const projection: Record<string, (number | string)[]> = {};
        [0, 0.25, 0.5, 0.75, 1].forEach((position, index) => {
          projection[`ramp/${index}`] = [`|${rgb(srgb(position)).formatHex()}`];
        });
        projection["ramp/5"] = [`|${rgb(lab(0.125)).formatHex()}`];
        return { projection };
      },
      /** 🎯️ The same ramp samples from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
