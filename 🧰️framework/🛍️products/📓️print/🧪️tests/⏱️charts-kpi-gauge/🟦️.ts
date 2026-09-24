// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { arc as d3Arc } from "d3-shape";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, roundProbeNumbers, type ProbeRecord } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-kpi-gauge";
const DECIMALS = 4;

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🧭 The family's degrees counter-clockwise from +x, as d3's radians clockwise from twelve. */
const toD3Angle = (degrees: number): number => ((90 - degrees) * Math.PI) / 180;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🎯️ The compiled probe of one fixture, kept whole so the arc and its centroid stay paired. */
async function probe(ctx: AdapterContext, fixture: string): Promise<ProbeRecord[]> {
  const parsed = await compileVizProbe(ctx.fixture(`shared://⏱️charts-kpi-gauge/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return roundProbeNumbers(parsed, DECIMALS);
}

/** 🎛️ The last arc a gauge drew is its value arc; the track is drawn first. */
const valueArc = (records: readonly ProbeRecord[]): number[] =>
  records.filter((record) => record.key === "geometry/arc").map((record) => record.values.map(Number)).at(-1) ?? [];

const valueCentroid = (records: readonly ProbeRecord[]): number[] =>
  records.filter((record) => record.key === "geometry/arc-centroid").map((record) => record.values.map(Number)).at(-1) ?? [];
// #endregion 🧫️Vectors

// #region 🧭️Adapter
/** 🖼️ The gauge disc of an 80x40 canvas with the family's padding: centre and radius in mm. */
const DISC = { cx: 43, cy: 22, radius: 12 } as const;

/** 🔮️ d3-shape's arc centroid for the sweep the feature's gauge row describes. */
function centroidOf(ctx: AdapterContext): number[] {
  const row = rows(ctx)[0]!;
  const start = Number(row.startAngle);
  const fraction = (Number(row.value) - Number(row.min)) / (Number(row.max) - Number(row.min));
  const end = start + fraction * (Number(row.endAngle) - start);
  const [x, y] = d3Arc()
    .innerRadius(DISC.radius)
    .outerRadius(DISC.radius)
    .startAngle(toD3Angle(start))
    .endAngle(toD3Angle(end))
    .centroid({} as never);
  return [grid(DISC.cx + x), grid(DISC.cy - y)];
}

/** 🎛️ Both scenarios ask the same question of a different sweep, so they share one shape. */
const gaugeScenario = (fixture: string) => ({
  oracle: (ctx: AdapterContext) => ({ projection: { "gauge/arc-centroid": centroidOf(ctx) } }),
  subject: async (ctx: AdapterContext) => {
    const records = await probe(ctx, fixture);
    if (valueArc(records).length < 5) throw new Error(`fixture ${fixture} drew no value arc`);
    return { projection: { "gauge/arc-centroid": valueCentroid(records) } };
  },
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    /** 🔮️/🎯️ The half-turn sweep of a radial gauge. */
    "radial-gauge": gaugeScenario("radial-gauge.tex"),
    /** 🔮️/🎯️ The full-turn sweep of a progress ring. */
    "progress-ring": gaugeScenario("progress-ring.tex"),
  },
});
// #endregion 🧭️Adapter
