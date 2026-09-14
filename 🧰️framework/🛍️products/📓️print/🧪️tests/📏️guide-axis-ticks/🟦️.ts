// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ticks } from "d3-array";
import { formatLocale, type FormatLocaleDefinition } from "d3-format";
import { scaleBand, scaleLog } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "guide-axis-ticks";
const DECIMALS = 6;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

/** 🎯️ Compiles one committed fixture of this case and projects its records. */
async function subject(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Locales
/** 🌍️ The two locales the library ships; d3-format's own definitions, so the comparison is real. */
const LOCALES: Readonly<Record<string, FormatLocaleDefinition>> = {
  en: { decimal: ".", thousands: ",", grouping: [3], currency: ["$", ""], minus: "−" },
  de: { decimal: ",", thousands: ".", grouping: [3], currency: ["", " €"], minus: "−" },
};
// #endregion 🔮️Locales

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "linear-ticks": {
      /** 🔮️ d3-array's ticks(start, stop, count), the published tick algorithm an axis follows. */
      oracle: (ctx: AdapterContext) => ({
        projection: Object.fromEntries(rows(ctx).map((row, index) => [`ticks/${index}`, grid(ticks(Number(row.start), Number(row.stop), Number(row.count)))])),
      }),
      /** 🎯️ The compiled probe of \semio_viz_guide_ticks_linear:nnnN on the same domains. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "linear-ticks.tex")),
    },
    "log-ticks": {
      /** 🔮️ d3-scale's log scale ticks over the same domains. */
      oracle: (ctx: AdapterContext) => ({
        projection: Object.fromEntries(rows(ctx).map((row, index) => [`ticks/${index}`, grid(scaleLog().domain([Number(row.start), Number(row.stop)]).ticks(Number(row.count)))])),
      }),
      /** 🎯️ The compiled probe of \semio_viz_guide_ticks_log:nnnnN. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "log-ticks.tex")),
    },
    "band-ticks": {
      /** 🔮️ d3-scale's band scale: one tick per domain entry, at the band's centre. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const domain = row.domain!.split(",");
        const scale = scaleBand<string>().domain(domain).range([Number(row.rangeMin), Number(row.rangeMax)]);
        return {
          projection: {
            "ticks/positions": grid(domain.map((entry) => scale(entry)! + scale.bandwidth() / 2)),
            "ticks/values": [...domain],
          },
        };
      },
      /** 🎯️ The compiled probe of the guide's own band tick positions. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "band-ticks.tex")),
    },
    "axis-geometry": {
      /** 📐️ The specified geometry of a bottom axis on an 80x40 frame with pad 8. */
      oracle: (ctx: AdapterContext) => ({
        projection: Object.fromEntries(rows(ctx).map((row) => [row.key!, row.values!.split(",").map(Number)])),
      }),
      /** 🎯️ The geometry the renderer emitted while drawing that axis. */
      subject: async (ctx: AdapterContext) => {
        const projection = (await subject(ctx, "axis-geometry.tex")).projection;
        const gridLines = projection["geometry/grid-line"] ?? [];
        return {
          projection: {
            "axis-domain": projection["geometry/axis-domain"] ?? [],
            "axis-tick-positions": projection["geometry/axis-tick-positions"] ?? [],
            "grid-line-first": gridLines.slice(0, 4),
          },
        };
      },
    },
    "tick-format-labels": {
      /** 🔮️ d3-format's own locale definitions rendering the same tick values. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const values = ticks(Number(row.start), Number(row.stop), Number(row.count));
        const projection: Record<string, (number | string)[]> = {};
        for (const locale of row.locales!.split(",")) {
          const render = formatLocale(LOCALES[locale]!).format(row.specifier!);
          values.forEach((value, index) => {
            const text = render(value);
            projection[`label/${locale}/${index}`] = [locale === "en" ? Number(text) : text];
          });
        }
        return { projection };
      },
      /** 🎯️ The labels the guide wrote through semio-viz-format for each locale. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "tick-format-labels.tex")),
    },
  },
});
// #endregion 🧭️Adapter
