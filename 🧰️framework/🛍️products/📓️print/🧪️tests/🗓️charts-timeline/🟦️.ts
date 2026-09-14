// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-timeline";
const DECIMALS = 4;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ The bars one compiled fixture drew, flattened in row order. */
async function bars(ctx: AdapterContext, fixture: string): Promise<number[]> {
  const parsed = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return roundProbeNumbers(parsed, DECIMALS)
    .filter((record) => record.key === "geometry/rect")
    .flatMap((record) => record.values.map(Number));
}

/** 📐️ The specification vectors of the scenario, flattened the same way. */
const specified = (ctx: AdapterContext): number[] =>
  rows(ctx).flatMap((row) => [Number(row.x), Number(row.y), Number(row.w), Number(row.h)].map(grid));
// #endregion 🧫️Vectors

// #region 🧭️Adapter
const laneScenario = (fixture: string) => ({
  oracle: (ctx: AdapterContext) => ({ projection: { "timeline/bars": specified(ctx) } }),
  subject: async (ctx: AdapterContext) => ({ projection: { "timeline/bars": await bars(ctx, fixture) } }),
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    /** 📐️/🎯️ Interval bars against the specified lane geometry. */
    "interval": laneScenario("interval.tex"),
    /** 📐️/🎯️ Swimlane bars against the same spans in a thinner lane fill. */
    "swimlane": laneScenario("swimlane.tex"),
  },
});
// #endregion 🧭️Adapter
