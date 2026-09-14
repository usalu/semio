// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "facet-layout";
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

/** 🎯️ Compiles the committed fixture of this case and projects its records. */
async function subject(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

/** 🧱️ Splits a flat record stream into one entry per panel, keyed `panel<index>`. */
function panels(values: readonly (number | string)[], width: number, prefix: string): Record<string, number[]> {
  const out: Record<string, number[]> = {};
  for (let index = 0; index + width <= values.length; index += width) out[`${prefix}${index / width}`] = values.slice(index, index + width).map(Number);
  return out;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "wrap-grid": {
      /** 📐️ The specified panel rectangles: index, x, y, width, height. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The rectangles the facet emitted while laying its panels out. */
      subject: async (ctx: AdapterContext) => ({ projection: panels((await subject(ctx, "wrap.tex")).projection["geometry/facet-panel"] ?? [], 5, "panel") }),
    },
    "shared-axes": {
      /** 📐️ Four axes for a four-panel two-column grid: x on the bottom row, y on the left column. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The number of axis domain lines the panels actually drew. */
      subject: async (ctx: AdapterContext) => {
        const domains = (await subject(ctx, "wrap.tex")).projection["geometry/axis-domain"] ?? [];
        return { projection: { axisCount: [domains.length / 4] } };
      },
    },
  },
});
// #endregion 🧭️Adapter
