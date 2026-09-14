// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { timeFormatLocale, type TimeLocaleDefinition } from "d3-time-format";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "format-time";
const FIXTURE = "local://format-time.tex";

/** 🌍️ The two calendar name lists the library ships, as d3-time-format locale definitions. */
const LOCALES: Readonly<Record<string, TimeLocaleDefinition>> = {
  en: {
    dateTime: "",
    date: "",
    time: "",
    periods: ["AM", "PM"],
    days: ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
    shortDays: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
    shortMonths: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
  },
  de: {
    dateTime: "",
    date: "",
    time: "",
    periods: ["AM", "PM"],
    days: ["Sonntag", "Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag"],
    shortDays: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"],
    months: ["Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober", "November", "Dezember"],
    shortMonths: ["Jan", "Feb", "Mrz", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"],
  },
};

/** 🧫️ The directive list the fixture runs, in the order it emits them. */
const DIRECTIVES: readonly (readonly [string, string])[] = [
  ["%Y-%m-%d", "2026-09-05"],
  ["%A %B %e, %Y", "2026-09-05"],
  ["%a %b %d", "2024-02-29T13:07:09"],
  ["%H:%M:%S %p", "2024-02-29T13:07:09"],
  ["%j %U %W %Z", "2024-02-29T13:07:09"],
  ["%y/%I %p", "1999-01-03T00:30"],
  ["%A", "2000-01-01"],
  ["%B %Y", "2026-03-01"],
  ["%d.%m.%Y", "2026-12-31"],
  ["%U %W", "2021-01-01"],
  ["%j", "2020-12-31"],
  ["100%% %b", "2026-05-04"],
];

/** 🔮️ Renders the directive list through d3-time-format in the transport encoding the probe uses. */
function reference(locale: string): { projection: ProbeProjection } {
  const format = timeFormatLocale(LOCALES[locale]!);
  return Object.fromEntries(DIRECTIVES.map(([specifier, timestamp], index) => [`time/${index}`, [`|${format.utcFormat(specifier)(new Date(`${timestamp}Z`)).replaceAll(" ", "_")}`]]));
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
      /** 🔮️ d3-time-format's utcFormat with the English calendar names. */
      oracle: () => ({ projection: reference("en") }),
      /** 🎯️ The same directives rendered by semio-viz-format inside a compiled document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "de-locale": {
      /** 🔮️ d3-time-format's utcFormat with the German calendar names. */
      oracle: () => ({ projection: reference("de") }),
      /** 🎯️ The same directives rendered with the German locale selected. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
