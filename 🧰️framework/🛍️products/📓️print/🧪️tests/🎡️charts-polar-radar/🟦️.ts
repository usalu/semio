// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleLinear } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-polar-radar";
const DECIMALS = 9;
const TAU = 2 * Math.PI;

/** 🧪️ `demo-direction` of `semio-viz-charts-polar`: the wind sectors with speed and count. */
const SPEED = [4.2, 5.6, 6.8, 3.9, 5.1, 7.4, 6.2, 4.7] as const;
const COUNT = [12, 9, 17, 7, 14, 21, 16, 10] as const;

/** 🧪️ The first series column of `demo-profile`. */
const PROFILE = [8, 4, 6, 9, 3, 5] as const;

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

type Frame = { cx: number; cy: number; radius: number };

/** 🖼️ The inscribed circle of the frame the fixture draws in. */
function frame(width: number, height: number, pad: number): Frame {
  return { cx: width / 2, cy: height / 2, radius: Math.min(width - 2 * pad, height - 2 * pad) / 2 };
}

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ d3-scale's linear radius scale from the value domain into the inner/outer radius band. */
function radial(values: readonly number[], f: Frame, inner: number) {
  return scaleLinear().domain([0, Math.max(...values)]).range([f.radius * inner, f.radius]);
}

/** 🔮️ The polar projection: zero at twelve o'clock, growing clockwise. */
const px = (f: Frame, angle: number, r: number): number => grid(f.cx + r * Math.sin(angle));
const py = (f: Frame, angle: number, r: number): number => grid(f.cy + r * Math.cos(angle));
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "polar-scatter": {
      /** 🔮️ One point per sector at angle i/n of the turn and the scaled radius. */
      oracle: () => {
        const f = frame(60, 40, 4);
        const r = radial(SPEED, f, 0.15);
        return {
          projection: {
            "geometry/circle": SPEED.flatMap((value, i) => {
              const angle = (TAU * i) / SPEED.length;
              return [px(f, angle, r(value)), py(f, angle, r(value)), 0.9];
            }),
          },
        };
      },
      /** 🎯️ The `geometry/circle` records the polar scatter drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/circle": (await probe(ctx, "polar-scatter.tex")).projection["geometry/circle"] ?? [] } }),
    },
    "polar-bars": {
      /** 🔮️ One annular sector per slot, inset by the family's 0.03 rad hairline gap. */
      oracle: () => {
        const f = frame(60, 40, 4);
        const r = radial(SPEED, f, 0.2);
        return {
          projection: {
            "geometry/arc": SPEED.flatMap((value, i) => [
              f.cx,
              f.cy,
              grid(f.radius * 0.2),
              grid(r(value)),
              grid((TAU * i) / SPEED.length + 0.03),
              grid((TAU * (i + 1)) / SPEED.length - 0.03),
            ]),
          },
        };
      },
      /** 🎯️ The `geometry/arc` records the radial bars drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/arc": (await probe(ctx, "polar-bars.tex")).projection["geometry/arc"] ?? [] } }),
    },
    "radar": {
      /** 🔮️ The single grid ring, then a closed profile whose radius is the value over the
       * maximum of every series column — the two polygons a one-level radar emits, in that order. */
      oracle: () => {
        const f = frame(60, 40, 4);
        const max = Math.max(...PROFILE);
        const ring = PROFILE.flatMap((_, i) => {
          const angle = (TAU * i) / PROFILE.length;
          return [px(f, angle, f.radius), py(f, angle, f.radius)];
        });
        const profile = PROFILE.flatMap((value, i) => {
          const angle = (TAU * i) / PROFILE.length;
          const r = (f.radius * value) / max;
          return [px(f, angle, r), py(f, angle, r)];
        });
        return { projection: { "geometry/polygon": [...ring, ...profile] } };
      },
      /** 🎯️ The `geometry/polygon` record the radar profile drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/polygon": (await probe(ctx, "radar.tex")).projection["geometry/polygon"] ?? [] } }),
    },
    "coxcomb": {
      /** 🔮️ Equal-angle sectors whose radius is the square root of the normalised count. */
      oracle: () => {
        const f = frame(60, 40, 4);
        const max = Math.max(...COUNT);
        return {
          projection: {
            "geometry/arc": COUNT.flatMap((value, i) => [
              f.cx,
              f.cy,
              0,
              grid(f.radius * Math.sqrt(value / max)),
              grid((TAU * i) / COUNT.length),
              grid((TAU * (i + 1)) / COUNT.length),
            ]),
          },
        };
      },
      /** 🎯️ The `geometry/arc` records the coxcomb drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/arc": (await probe(ctx, "coxcomb.tex")).projection["geometry/arc"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
