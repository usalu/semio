// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleLinear } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "annotation-placement";
const DECIMALS = 6;
const X = scaleLinear().domain([0, 10]).range([8, 118]);
const Y = scaleLinear().domain([0, 100]).range([8, 58]);

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
  const records = await compileVizProbe(ctx.fixture(`shared://🏷️annotation-placement/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

/** 📐️ The `key | values` table of a conformance scenario. */
function specified(ctx: AdapterContext): Record<string, number[]> {
  return Object.fromEntries(rows(ctx).map((row) => [row.key!, row.values!.split(",").map(Number)]));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "data-space-anchors": {
      /** 🔮️ d3-scale mapping the same data coordinates onto the same millimetre ranges. */
      oracle: (ctx: AdapterContext) => ({
        projection: { "annotation-anchors": grid(rows(ctx).flatMap((row) => [X(Number(row.x)), Y(Number(row.y))])) },
      }),
      /** 🎯️ The positions the annotations resolved to while drawing. */
      subject: async (ctx: AdapterContext) => {
        const values = (await subject(ctx, "data-space.tex")).projection["geometry/annotation-reference-point"] ?? [];
        const anchors: number[] = [];
        for (let index = 0; index + 3 <= values.length; index += 3) anchors.push(Number(values[index]), Number(values[index + 1]));
        return { projection: { "annotation-anchors": anchors } };
      },
    },
    "reference-extent": {
      /** 📐️ The specified extents of a reference line, band and event marker. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The extents the annotations emitted while drawing. */
      subject: async (ctx: AdapterContext) => {
        const projection = (await subject(ctx, "data-space.tex")).projection;
        return {
          projection: {
            "reference-line": projection["geometry/annotation-reference-line"] ?? [],
            "reference-band": projection["geometry/annotation-reference-band"] ?? [],
            "event-marker": projection["geometry/annotation-event-marker"] ?? [],
          },
        };
      },
    },
    "bracket-normal": {
      /** 📐️ The specified bracket segment and arm depth. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The bracket the annotation emitted while drawing. */
      subject: async (ctx: AdapterContext) => ({ projection: { bracket: (await subject(ctx, "bracket.tex")).projection["geometry/annotation-bracket"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
