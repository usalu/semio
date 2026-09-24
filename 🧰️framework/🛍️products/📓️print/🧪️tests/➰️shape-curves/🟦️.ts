// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as shape from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "shape-curves";
const FIXTURE = "shared://➰️shape-curves/shape-curves.tex";
const SERIES: [number, number][] = [[0, 0], [1, 3], [2, 1], [3, 4], [4, 2], [5, 5]];

const DECIMALS = 6;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔤️ Splits an SVG path into the token stream the probe records: command letters as
 *  strings, coordinates as numbers, in the order d3 wrote them. */
function tokens(path: string | null): (number | string)[] {
  const out: (number | string)[] = [];
  const pattern = /[MLCQAZhv]|-?\d*\.?\d+(?:e[-+]?\d+)?/gi;
  let match: RegExpExecArray | null;
  while ((match = pattern.exec(path ?? "")) !== null) {
    const raw = match[0];
    const value = Number(raw);
    out.push(Number.isNaN(value) ? raw : grid([value])[0]!);
  }
  return out;
}

/** 〰️ The interpolators, keyed the way the LaTeX vocabulary names them. */
const CURVES: Record<string, unknown> = {
  linear: shape.curveLinear,
  "linear-closed": shape.curveLinearClosed,
  step: shape.curveStep,
  "step-before": shape.curveStepBefore,
  "step-after": shape.curveStepAfter,
  basis: shape.curveBasis,
  "basis-open": shape.curveBasisOpen,
  "basis-closed": shape.curveBasisClosed,
  bundle: shape.curveBundle,
  cardinal: shape.curveCardinal,
  "cardinal-open": shape.curveCardinalOpen,
  "cardinal-closed": shape.curveCardinalClosed,
  "catmull-rom": shape.curveCatmullRom,
  "catmull-rom-open": shape.curveCatmullRomOpen,
  "catmull-rom-closed": shape.curveCatmullRomClosed,
  "monotone-x": shape.curveMonotoneX,
  "monotone-y": shape.curveMonotoneY,
  natural: shape.curveNatural,
};

/** 🔮️ The SVG path d3 writes for one curve over one series. */
function path(curve: unknown, points: readonly (readonly [number, number])[]): (number | string)[] {
  const line = shape.line().curve(curve as shape.CurveFactory).digits(15);
  return tokens(line(points as [number, number][]));
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
    interpolators: {
      /** 🔮️ d3-shape draws the same six points through each of its eighteen interpolators. */
      oracle: () => ({
        projection: Object.fromEntries(Object.entries(CURVES).map(([name, curve]) => [`curve/${name}`, path(curve, SERIES)])),
      }),
      /** 🎯️ The same interpolators, transcribed into expl3 and run inside a compiled document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    parameters: {
      /** 🔮️ The parameterised factories, with the parameter the feature table names. */
      oracle: () => ({
        projection: {
          "curve/cardinal-tension": path(shape.curveCardinal.tension(0.5), SERIES),
          "curve/catmull-rom-alpha-zero": path(shape.curveCatmullRom.alpha(0), SERIES),
          "curve/catmull-rom-alpha-one": path(shape.curveCatmullRom.alpha(1), SERIES),
          "curve/bundle-beta": path(shape.curveBundle.beta(0.5), SERIES),
        },
      }),
      /** 🎯️ The same parameters, given as curve options in the probe document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    degenerate: {
      /** 🔮️ d3 closes a one-point line and straightens a two-point one. */
      oracle: () => ({
        projection: {
          "curve/one-point": path(shape.curveLinear, [[2, 7]]),
          "curve/two-points": path(shape.curveBasis, [[2, 7], [4, 9]]),
          "curve/two-points-natural": path(shape.curveNatural, [[2, 7], [4, 9]]),
          "curve/three-points": path(shape.curveCatmullRom, [[2, 7], [4, 9], [6, 3]]),
        },
      }),
      /** 🎯️ The same short series through the transcribed state machines. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
