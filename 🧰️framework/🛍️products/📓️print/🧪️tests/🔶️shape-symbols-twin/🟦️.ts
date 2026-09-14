// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { pathRound } from "d3-path";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { drawVizSymbol, type VizSymbolKind } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🔷️shape-symbols/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;
const SIZES = ["64", "17.5", "200"] as const;

/** 🔣️ The thirteen symbol types, keyed the way the LaTeX vocabulary names them. */
const TYPES: readonly VizSymbolKind[] = ["circle", "cross", "diamond", "diamond2", "plus", "square", "square2", "star", "times", "triangle", "triangle2", "wye", "asterisk"];

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

/** 🎯️ The SVG path the twin writes for one glyph of one area; the twin computes the geometry and
 *  d3-path only formats it, so the two subjects differ in nothing but the geometry. */
function glyph(kind: VizSymbolKind, size: number): (number | string)[] {
  const context = pathRound(15);
  drawVizSymbol(kind, context, size);
  return tokens(context.toString());
}

/** 🔮️ The oracle of `shape-symbols`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`shape-symbols declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "symbol-paths": {
      oracle: oracle("symbol-paths"),
      /** 🎯️ The twin draws each type from the area it was given. */
      subject: () => {
        const projection: Record<string, (number | string)[]> = {};
        for (const size of SIZES) for (const name of TYPES) projection[`symbol/${name}/${size}`] = glyph(name, Number(size));
        return { projection };
      },
    },
  },
});
// #endregion 🧭️Adapter
