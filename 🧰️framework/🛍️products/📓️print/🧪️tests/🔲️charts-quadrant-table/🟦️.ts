// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleBand } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-quadrant-table";
const DECIMALS = 6;

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ An unpadded d3-scale band scale per axis, walked column-major inside each row. */
function quadrants(width: number, height: number, cells: number): number[] {
  const domain = Array.from({ length: cells }, (_, i) => String(i));
  const columns = scaleBand<string>().domain(domain).range([0, width]);
  const bandRows = scaleBand<string>().domain(domain).range([0, height]);
  const out: number[] = [];
  for (const column of domain) {
    for (const row of domain) {
      out.push(grid(columns(column)!), grid(bandRows(row)!), grid(columns.bandwidth()), grid(bandRows.bandwidth()));
    }
  }
  return out;
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "quadrant": {
      /** 🔮️ d3-scale's band scale twice over a 40 mm square, two cells per axis. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        return { projection: { "geometry/rect": quadrants(Number(row.width), Number(row.height), Number(row.cells)) } };
      },
      /** 🎯️ The `geometry/rect` records the quadrant matrix drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": (await probe(ctx, "quadrant.tex")).projection["geometry/rect"] ?? [] } }),
    },
    "risk": {
      /** 🔮️ The same construction with three cells per axis. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        return { projection: { "geometry/rect": quadrants(Number(row.width), Number(row.height), Number(row.cells)) } };
      },
      /** 🎯️ The `geometry/rect` records the nine-box risk grid drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": (await probe(ctx, "risk.tex")).projection["geometry/rect"] ?? [] } }),
    },
    "table": {
      /** 📐️ The specification anchors: header on the top edge, rule at 0.6 rows, body every row. */
      oracle: (ctx: AdapterContext) => {
        const spec = rows(ctx);
        const anchors = spec.filter((row) => !row.key!.startsWith("rule"));
        const rule = spec.filter((row) => row.key!.startsWith("rule"));
        return {
          projection: {
            anchors: anchors.flatMap((row) => [grid(Number(row.x)), grid(Number(row.y))]),
            rule: rule.flatMap((row) => [grid(Number(row.x)), grid(Number(row.y))]),
          },
        };
      },
      /** 🎯️ The header, rule and body anchors the plain table drew, sampled at the named rows. */
      subject: async (ctx: AdapterContext) => {
        const projection = (await probe(ctx, "table.tex")).projection;
        const text = (projection["geometry/text"] ?? []).map(Number);
        const line = (projection["geometry/line"] ?? []).map(Number);
        const at = (index: number): number[] => [text[index * 2]!, text[index * 2 + 1]!];
        return {
          projection: {
            anchors: [...at(0), ...at(1), ...at(2), ...at(3), ...at(4), ...at(5), ...at(8), ...at(24), ...at(27)],
            rule: [line[0]!, line[1]!, line[2]!, line[3]!],
          },
        };
      },
    },
    "table-bars": {
      /** 📐️ The specification bars: one per body row, as wide as the value's column share. */
      oracle: (ctx: AdapterContext) => ({
        projection: { "geometry/rect": rows(ctx).flatMap((row) => [Number(row.x), Number(row.y), Number(row.w), Number(row.h)].map(grid)) },
      }),
      /** 🎯️ The `geometry/rect` records the bar-in-cell table drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": (await probe(ctx, "table-bars.tex")).projection["geometry/rect"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
