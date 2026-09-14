// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { pathRound } from "d3-path";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizCurve, vizLine, type VizCurveFactory, type VizCurveKind } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../➰️shape-curves/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;
const SERIES: [number, number][] = [[0, 0], [1, 3], [2, 1], [3, 4], [4, 2], [5, 5]];

/** 〰️ The interpolators, keyed the way the LaTeX vocabulary names them. */
const CURVES: readonly VizCurveKind[] = [
  "linear",
  "linear-closed",
  "step",
  "step-before",
  "step-after",
  "basis",
  "basis-open",
  "basis-closed",
  "bundle",
  "cardinal",
  "cardinal-open",
  "cardinal-closed",
  "catmull-rom",
  "catmull-rom-open",
  "catmull-rom-closed",
  "monotone-x",
  "monotone-y",
  "natural",
];

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔤️ Splits an SVG path into the token stream the probe records: command letters as
 *  strings, coordinates as numbers, in the order the generator wrote them. */
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

/** 🎯️ The SVG path the twin writes for one curve over one series; the twin computes the geometry
 *  and d3-path only formats it, so the two subjects differ in nothing but the geometry. */
function path(curve: VizCurveFactory, points: readonly (readonly [number, number])[]): (number | string)[] {
  const context = pathRound(15);
  vizLine(points, { curve }, context);
  return tokens(context.toString());
}

/** 🔮️ The oracle of `shape-curves`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`shape-curves declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    interpolators: {
      oracle: oracle("interpolators"),
      /** 🎯️ The twin draws the same six points through each of its eighteen interpolators. */
      subject: () => ({
        projection: Object.fromEntries(CURVES.map((name) => [`curve/${name}`, path(vizCurve(name), SERIES)])),
      }),
    },
    parameters: {
      oracle: oracle("parameters"),
      /** 🎯️ The parameterised factories, with the parameter the feature table names. */
      subject: () => ({
        projection: {
          "curve/cardinal-tension": path(vizCurve("cardinal", { tension: 0.5 }), SERIES),
          "curve/catmull-rom-alpha-zero": path(vizCurve("catmull-rom", { alpha: 0 }), SERIES),
          "curve/catmull-rom-alpha-one": path(vizCurve("catmull-rom", { alpha: 1 }), SERIES),
          "curve/bundle-beta": path(vizCurve("bundle", { beta: 0.5 }), SERIES),
        },
      }),
    },
    degenerate: {
      oracle: oracle("degenerate"),
      /** 🎯️ The twin closes a one-point line and straightens a two-point one. */
      subject: () => ({
        projection: {
          "curve/one-point": path(vizCurve("linear"), [[2, 7]]),
          "curve/two-points": path(vizCurve("basis"), [[2, 7], [4, 9]]),
          "curve/two-points-natural": path(vizCurve("natural"), [[2, 7], [4, 9]]),
          "curve/three-points": path(vizCurve("catmull-rom"), [[2, 7], [4, 9], [6, 3]]),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
