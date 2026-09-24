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
const CASE = "shape-symbols";
const FIXTURE = "shared://🔷️shape-symbols/shape-symbols.tex";
const SIZES = ["64", "17.5", "200"] as const;

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

/** 🔣️ The thirteen symbol types, keyed the way the LaTeX vocabulary names them. */
const TYPES: Record<string, shape.SymbolType> = {
  circle: shape.symbolCircle,
  cross: shape.symbolCross,
  diamond: shape.symbolDiamond,
  "diamond2": shape.symbolDiamond2,
  plus: shape.symbolPlus,
  square: shape.symbolSquare,
  "square2": shape.symbolSquare2,
  star: shape.symbolStar,
  times: shape.symbolTimes,
  triangle: shape.symbolTriangle,
  "triangle2": shape.symbolTriangle2,
  wye: shape.symbolWye,
  asterisk: shape.symbolAsterisk,
};

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
    "symbol-paths": {
      /** 🔮️ d3-symbol draws each type from the area it was given. */
      oracle: () => {
        const projection: Record<string, (number | string)[]> = {};
        for (const size of SIZES) {
          for (const [name, type] of Object.entries(TYPES)) {
            projection[`symbol/${name}/${size}`] = tokens(shape.symbol().type(type).size(Number(size)).digits(15)());
          }
        }
        return { projection };
      },
      /** 🎯️ The same thirteen routines, transcribed into expl3. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
