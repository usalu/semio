// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleOrdinal } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "guide-legend";
const DECIMALS = 6;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🎯️ Compiles one committed fixture of this case and projects its records. */
async function subject(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

/** 🔢️ Every n-th triple of a legend-item record stream, as one flat list. */
function itemColumns(values: readonly (number | string)[], width: number, offsets: readonly number[]): number[] {
  const out: number[] = [];
  for (let index = 0; index + width <= values.length; index += width) for (const offset of offsets) out.push(Number(values[index + offset]));
  return out;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "categorical-entries": {
      /** 🔮️ d3-scale's ordinal scale: its domain is exactly the entry set a swatch legend shows. */
      oracle: (ctx: AdapterContext) => ({ projection: { "legend/entries": [...scaleOrdinal<string, string>().domain(rows(ctx)[0]!.domain!.split(",")).domain()] } }),
      /** 🎯️ The entries the legend enumerated from the same scale. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, "categorical.tex")),
    },
    "vertical-layout": {
      /** 📐️ The specified entry origins of a vertical legend. */
      oracle: (ctx: AdapterContext) => ({ projection: Object.fromEntries(rows(ctx).map((row) => [row.key!, row.values!.split(",").map(Number)])) }),
      /** 🎯️ The origins the legend emitted while drawing. */
      subject: async (ctx: AdapterContext) => {
        const projection = (await subject(ctx, "categorical.tex")).projection;
        return { projection: { "legend-origins": itemColumns(projection["geometry/legend-item"] ?? [], 3, [1, 2]) } };
      },
    },
    "size-entries": {
      /** 📐️ The specified circle radii: the scale's mapping of its own ticks. */
      oracle: (ctx: AdapterContext) => ({ projection: Object.fromEntries(rows(ctx).map((row) => [row.key!, row.values!.split(",").map(Number)])) }),
      /** 🎯️ The radii the size legend emitted while drawing. */
      subject: async (ctx: AdapterContext) => {
        const projection = (await subject(ctx, "size.tex")).projection;
        return { projection: { "legend-radii": itemColumns(projection["geometry/legend-item"] ?? [], 3, [2]) } };
      },
    },
  },
});
// #endregion 🧭️Adapter
