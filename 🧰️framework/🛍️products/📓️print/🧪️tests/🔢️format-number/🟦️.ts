// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { formatLocale, type FormatLocaleDefinition } from "d3-format";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "format-number";
const FIXTURE = "shared://🔢️format-number/format-number.tex";

/** 🌍️ d3-format's own locale definitions; `−` (U+2212) is d3's default minus and print's correct one. */
const LOCALES: Readonly<Record<string, FormatLocaleDefinition>> = {
  en: { decimal: ".", thousands: ",", grouping: [3], currency: ["$", ""], minus: "−" },
  de: { decimal: ",", thousands: ".", grouping: [3], currency: ["", " €"], minus: "−" },
};

/** 🧫️ The specifier list the fixture runs, in the order it emits them. */
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

/** 🔮️ Renders the specifier list through d3-format in the transport encoding the probe uses. */
function reference(locale: string): { projection: ProbeProjection } {
  const format = formatLocale(LOCALES[locale]!);
  return Object.fromEntries(SPECIFIERS.map(([specifier, value], index) => [`format/${index}`, [`|${format.format(specifier)(value as number).replaceAll(" ", "_")}`]]));
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(records, ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "en-locale": {
      /** 🔮️ d3-format with its English locale definition. */
      oracle: () => ({ projection: reference("en") }),
      /** 🎯️ The same specifiers rendered by semio-viz-format inside a compiled document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "de-locale": {
      /** 🔮️ d3-format with its German locale definition. */
      oracle: () => ({ projection: reference("de") }),
      /** 🎯️ The same specifiers rendered with the German locale selected. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
