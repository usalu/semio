// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "composition-concat-inset";
const DECIMALS = 6;

/** 🧫️ The scenario's data table as records — the feature owns every rectangle this case fixes. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 📐️ The `key | values` table of a conformance scenario. */
function specified(ctx: AdapterContext): Record<string, number[]> {
  return Object.fromEntries(rows(ctx).map((row) => [row.key!, row.values!.split(",").map(Number)]));
}

/** 🎯️ Compiles one committed fixture of this case and projects its records. */
async function subject(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

/** 🧱️ Splits a flat record stream into one entry per rectangle, keyed `<prefix><index>`. */
function chunks(values: readonly (number | string)[], width: number, prefix: string): Record<string, number[]> {
  const out: Record<string, number[]> = {};
  for (let index = 0; index + width <= values.length; index += width) out[`${prefix}${index / width}`] = values.slice(index, index + width).map(Number);
  return out;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "concat-horizontal": {
      /** 📐️ The specified item rectangles: index, x, y, width, height. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The rectangles the concatenation handed to its items. */
      subject: async (ctx: AdapterContext) => ({ projection: chunks((await subject(ctx, "concat.tex")).projection["geometry/concat-item"] ?? [], 5, "item") }),
    },
    "inset-rectangle": {
      /** 📐️ The specified inset rectangle, unchanged from its keys. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The rectangle the inset handed to its body. */
      subject: async (ctx: AdapterContext) => ({ projection: { inset: ((await subject(ctx, "inset.tex")).projection["geometry/inset"] ?? []).map(Number) } }),
    },
    "dashboard-span": {
      /** 📐️ The specified cell rectangles, including the slot skipped by the spanning cell. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The rectangles the dashboard handed to its cells. */
      subject: async (ctx: AdapterContext) => ({ projection: chunks((await subject(ctx, "dashboard.tex")).projection["geometry/dashboard-cell"] ?? [], 5, "cell") }),
    },
  },
});
// #endregion 🧭️Adapter
