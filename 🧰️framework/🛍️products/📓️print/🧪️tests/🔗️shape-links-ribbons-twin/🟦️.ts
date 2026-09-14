// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { pathRound } from "d3-path";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizLink, type VizLinkKind, type VizPoint } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🎀️shape-links-ribbons/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

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

/** 🔗️ One twin link of the named flavour between two points; the twin computes the geometry and
 *  d3-path only formats it, so the two subjects differ in nothing but the geometry. */
function link(kind: VizLinkKind, source: VizPoint, target: VizPoint): (number | string)[] {
  const context = pathRound(15);
  vizLink(kind, source, target, context);
  return tokens(context.toString());
}

/** 🔮️ The oracle of `shape-links-ribbons`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`shape-links-ribbons declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    links: {
      oracle: oracle("links"),
      /** 🎯️ The twin's three link orientations, including the coincident-endpoint case. */
      subject: () => ({
        projection: {
          "link/horizontal": link("horizontal", [10, 20], [80, 60]),
          "link/vertical": link("vertical", [10, 20], [80, 60]),
          "link/radial": link("radial", [0.5, 30], [2.5, 90]),
          "link/flat": link("horizontal", [10, 20], [10, 20]),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
