// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "coordinate-polar-ternary";
const FIXTURE = "local://coordinate-polar-ternary.tex";

const DECIMALS = 6;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 📐️ The cartesian formula the package documents: origin plus the unit share of the frame. */
function cartesian(u: number, v: number, width: number, height: number, ox = 0, oy = 0, flip = false): number[] {
  return grid([ox + u * width, oy + (flip ? 1 - v : v) * height]);
}

/** 🧭️ The polar formula: angle clockwise from twelve o'clock, radius between the two radii. */
function polar(u: number, v: number, inner: number, outer: number, clockwise = true, base?: number): number[] {
  const angle = (clockwise ? 1 : -1) * u * 2 * Math.PI;
  const share = base === undefined ? v : Math.log(1 + v * (base - 1)) / Math.log(base);
  const radius = inner + share * (outer - inner);
  return grid([radius * Math.sin(angle), radius * Math.cos(angle)]);
}

/** 🔺️ The ternary formula: b along the base, the remaining share up the height. */
function ternary(a: number, b: number, size: number): number[] {
  const c = 1 - a - b;
  return grid([size * (b + c / 2), (size * Math.sqrt(3) / 2) * c]);
}

/** ∥ The parallel formula: the axis index spaced evenly, the value up the frame. */
function parallel(axis: number, value: number, axes: number, width: number, height: number): number[] {
  return grid([(width * (axis - 1)) / Math.max(1, axes - 1), value * height]);
}

/** 🌍️ The equirectangular fallback of the geographic hook. */
function geographic(lon: number, lat: number, width: number, height: number): number[] {
  return grid([(width * (lon + 180)) / 360, (height * (lat + 90)) / 180]);
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
    cartesian: {
      /** 📏️ The specification vectors of the cartesian system. */
      oracle: () => ({
        projection: {
          "coordinate/cartesian-origin": cartesian(0, 0, 80, 40),
          "coordinate/cartesian-middle": cartesian(0.5, 0.25, 80, 40),
          "coordinate/cartesian-corner": cartesian(1, 1, 80, 40),
          "coordinate/cartesian-flipped": cartesian(0.5, 0.25, 80, 40, 10, 5, true),
          "coordinate/cartesian-domain": cartesian(0.25, 0.5, 80, 40),
        },
      }),
      /** 🎯️ \SemioVizProject inside a compiled document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    polar: {
      /** 📏️ The specification vectors of the polar and log-polar systems. */
      oracle: () => ({
        projection: {
          "coordinate/polar-quarter": polar(0.25, 1, 0, 20),
          "coordinate/polar-noon": polar(0, 1, 0, 20),
          "coordinate/polar-half": polar(0.5, 0.5, 0, 20),
          "coordinate/polar-counter": polar(0.25, 0, 8, 20, false),
          "coordinate/logpolar-half": polar(0, 0.5, 0, 20, true, 10),
          "coordinate/logpolar-full": polar(0, 1, 0, 20, true, 10),
        },
      }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    ternary: {
      /** 📏️ The specification vectors of the ternary triangle. */
      oracle: () => ({
        projection: {
          "coordinate/ternary-a": ternary(1, 0, 60),
          "coordinate/ternary-b": ternary(0, 1, 60),
          "coordinate/ternary-c": ternary(0, 0, 60),
          "coordinate/ternary-mid": ternary(0.5, 0.25, 60),
        },
      }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    parallel: {
      /** 📏️ The specification vectors of the parallel axes. */
      oracle: () => ({
        projection: {
          "coordinate/parallel-first": parallel(1, 0, 4, 90, 50),
          "coordinate/parallel-third": parallel(3, 0.5, 4, 90, 50),
          "coordinate/parallel-last": parallel(4, 1, 4, 90, 50),
        },
      }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    geographic: {
      /** 📏️ The equirectangular fallback the hook ships with. */
      oracle: () => ({
        projection: {
          "coordinate/geographic-null": geographic(0, 0, 360, 180),
          "coordinate/geographic-hannover": geographic(9.72, 52.37, 360, 180),
          "coordinate/geographic-antimeridian": geographic(180, -90, 360, 180),
        },
      }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
