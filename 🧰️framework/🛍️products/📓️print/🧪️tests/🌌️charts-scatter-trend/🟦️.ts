// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleSqrt } from "d3-scale";
import { regressionLinear } from "d3-regression";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-scatter-trend";
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

/** 🎯️ Every record of one compiled fixture that carries the given key. */
async function records(ctx: AdapterContext, fixture: string, key: string): Promise<number[][]> {
  const parsed = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return roundProbeNumbers(parsed, DECIMALS)
    .filter((record) => record.key === key)
    .map((record) => record.values.map(Number));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "linear-trend": {
      /** 🔮️ d3-regression's own least-squares fit over the feature's points. */
      oracle: (ctx: AdapterContext) => {
        const points = rows(ctx).map((row) => [Number(row.x), Number(row.y)] as [number, number]);
        const fit = regressionLinear<[number, number]>().x((point) => point[0]).y((point) => point[1])(points);
        return { projection: { "trend/linear": [grid(fit.a), grid(fit.b)] } };
      },
      /** 🎯️ The slope and intercept the scatter family computed before mapping. */
      subject: async (ctx: AdapterContext) => ({
        projection: { "trend/linear": (await records(ctx, "linear-trend.tex", "geometry/regression"))[0] ?? [] },
      }),
    },
    "bubble-size": {
      /** 🔮️ d3-scale's square-root scale from the size extent onto the radius range. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const values = row.values!.split(",").map(Number);
        const scale = scaleSqrt().domain([Math.min(...values), Math.max(...values)]).range([Number(row.rmin), Number(row.rmax)]);
        return { projection: { "size/radius": values.map((value) => grid(scale(value))) } };
      },
      /** 🎯️ The radii the scatter family drew, in row order. */
      subject: async (ctx: AdapterContext) => ({
        projection: { "size/radius": (await records(ctx, "bubble-size.tex", "geometry/point")).map((values) => values[2] ?? 0) },
      }),
    },
  },
});
// #endregion 🧭️Adapter
