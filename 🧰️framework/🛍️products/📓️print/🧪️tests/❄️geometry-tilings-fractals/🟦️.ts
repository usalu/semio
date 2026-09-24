// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "geometry-tilings-fractals";
const FIXTURE = "shared://❄️geometry-tilings-fractals/geometry-tilings-fractals.tex";
const DECIMALS = 3;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the comparison uses. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** ❄️ Rewrites an axiom with its productions, exactly as the turtle reads it. */
function rewrite(axiom: string, rules: ReadonlyMap<string, string>, depth: number): string {
  let word = axiom;
  for (let pass = 0; pass < depth; pass += 1) {
    word = [...word].map((symbol) => rules.get(symbol) ?? symbol).join("");
  }
  return word;
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
const KOCH = new Map([["F", "F+F--F+F"]]);
const SIERPINSKI = new Map([["F", "F-G+F+G-F"], ["G", "GG"]]);
const DRAGON = new Map([["F", "F+G"], ["G", "F-G"]]);
const SIZE = 6;
const SPACING = SIZE * 1.732;

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "penrose-substitution": {
      /** 🔮️ The P3 substitution replaces every rhomb with two, starting from a ten-rhomb wheel. */
      oracle: () => ({
        projection: {
          "geometry/tiling/penrose": [0, 1, 2, 3].flatMap((depth) => [depth, 10 * 2 ** depth]),
        },
      }),
      subject,
    },
    "l-system-words": {
      /** 🔮️ The rewritten words, counted symbol by symbol. */
      oracle: () => ({
        projection: {
          "geometry/fractal/word": [
            1, rewrite("F--F--F", KOCH, 1).length,
            2, rewrite("F--F--F", KOCH, 2).length,
            3, rewrite("F--F--F", KOCH, 3).length,
            3, rewrite("F-G-G", SIERPINSKI, 3).length,
            6, rewrite("F", DRAGON, 6).length,
          ],
        },
      }),
      subject,
    },
    "regular-lattice": {
      /** 🔮️ A square tiling of side `size` packs its centres a `size·sqrt(3)` grid apart. */
      oracle: () => {
        const cells: number[] = [];
        for (let column = 0; column < 3; column += 1) {
          for (let row = 0; row < 3; row += 1) cells.push(column * SPACING, row * SPACING);
        }
        return { projection: { "geometry/tiling/cell": grid(cells) } };
      },
      subject,
    },
  },
});
// #endregion 🧭️Adapter
