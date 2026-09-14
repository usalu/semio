// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizFormat, type VizLanguage } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🔢️format-number/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 🧫️ The specifier list the case runs, in the order `format-number` emits them. */
const SPECIFIERS: readonly (readonly [string, number | string])[] = [
  [".2f", 3.14159],
  [".0f", 2.5],
  ["d", 42.6],
  [",", 1234567.891],
  [",.2f", 1234567.891],
  [".3e", 42],
  [".3s", 1234567],
  [".2s", 0.00042],
  [".1%", 0.234],
  [".2p", 0.1234],
  [".3r", 123456],
  [".4g", 0.00012367],
  [".3~f", 1.5],
  ["", 1234.5678],
  ["+.2f", 3.14159],
  ["08.2f", -3.14159],
  [">10.2f", 3.1],
  ["<10.2f", 3.1],
  ["^10.2f", 3.1],
  ["=10.2f", -3.1],
  ["012,.2f", 1234.5],
  ["(.2f", -3.5],
  [".2f", -0.001],
  ["x", 255],
  ["X", 255],
  ["b", 10],
  ["o", 64],
  ["c", "unit"],
];

/** 🔮️ The oracle of `format-number`, reused so both subjects meet the same reference strings. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`format-number declares no oracle for ${id}`);
  return handler;
}

/** 🎯️ Renders the specifier list through the twin in the transport encoding the probe uses. */
function rendered(language: VizLanguage): { projection: Record<string, string[]> } {
  return { projection: Object.fromEntries(SPECIFIERS.map(([specifier, value], index) => [`format/${index}`, [`|${vizFormat(specifier, language)(value as number).replaceAll(" ", "_")}`]])) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "en-locale": {
      oracle: oracle("en-locale"),
      /** 🎯️ The twin's formatter with the English document language. */
      subject: () => rendered("en"),
    },
    "de-locale": {
      oracle: oracle("de-locale"),
      /** 🎯️ The twin's formatter with the German document language. */
      subject: () => rendered("de"),
    },
  },
});
// #endregion 🧭️Adapter
