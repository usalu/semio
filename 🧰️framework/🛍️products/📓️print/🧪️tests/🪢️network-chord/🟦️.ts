// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ascending, descending } from "d3-array";
import { chord as d3Chord } from "d3-chord";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "network-chord";
const DECIMALS = 6;

type Options = Readonly<{ padAngle: number; sortGroups: string; sortSubgroups: string }>;

const OPTIONS: Readonly<Record<string, Options>> = {
  "plain-matrix": { padAngle: 0, sortGroups: "none", sortSubgroups: "none" },
  "padded-and-sorted": { padAngle: 0.05, sortGroups: "descending", sortSubgroups: "ascending" },
};

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🔵 Node ids in first-appearance order, source before target — the order the kernel derives too. */
function nodeIds(ctx: AdapterContext): string[] {
  const seen: string[] = [];
  for (const row of rows(ctx)) for (const id of [row.source!, row.target!]) if (!seen.includes(id)) seen.push(id);
  return seen;
}

/** 🔲 The dense matrix the chord layout reads: cell (i, j) is the summed weight of the edges i → j. */
function matrix(ctx: AdapterContext): number[][] {
  const ids = nodeIds(ctx);
  const cells = ids.map(() => ids.map(() => 0));
  for (const row of rows(ctx)) cells[ids.indexOf(row.source!)]![ids.indexOf(row.target!)] += Number(row.weight);
  return cells;
}

function options(ctx: AdapterContext): Options {
  const found = OPTIONS[ctx.scenario.id];
  if (found === undefined) throw new Error(`scenario ${ctx.scenario.id} has no chord options`);
  return found;
}

/** 🔢️ Rounds an oracle's own numbers onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
function comparator(name: string): ((a: number, b: number) => number) | null {
  if (name === "ascending") return ascending;
  if (name === "descending") return descending;
  return null;
}

/** 🔮️ d3-chord itself, over the same matrix and the same padding and sorting. */
function d3Projection(ctx: AdapterContext): { projection: ProbeProjection } {
  const config = options(ctx);
  let layout = d3Chord().padAngle(config.padAngle);
  const groups = comparator(config.sortGroups);
  const subgroups = comparator(config.sortSubgroups);
  if (groups !== null) layout = layout.sortGroups(groups);
  if (subgroups !== null) layout = layout.sortSubgroups(subgroups);
  const chords = layout(matrix(ctx));
  const groupValues: number[] = [];
  for (const group of chords.groups) groupValues.push(group.index + 1, group.startAngle, group.endAngle, group.value);
  const ribbonValues: number[] = [];
  let ribbon = 0;
  for (const chord of chords) {
    ribbon += 1;
    ribbonValues.push(ribbon, chord.source.index + 1, chord.target.index + 1,
      chord.source.startAngle, chord.source.endAngle, chord.target.startAngle, chord.target.endAngle);
  }
  return { "geometry/layout/chord-group": grid(groupValues), "geometry/layout/chord-ribbon": grid(ribbonValues) };
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
/** 🧪️ The probe document: the same edge list through the kernel's own `chord` layout. */
function statements(ctx: AdapterContext): VizProbeStatement[] {
  const config = options(ctx);
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizTable{edges}{source,target,weight}" }];
  for (const row of rows(ctx)) body.push({ raw: `\\SemioVizRow{edges}{${row.source},${row.target},${row.weight}}` });
  body.push({ raw: "\\SemioVizGraph{edges}[source=source,target=target,weight=weight]" });
  body.push({ raw: "\\SemioVizProbeOn" });
  body.push({
    raw: `\\SemioVizLayout{chord}{edges}{ribbons}[pad-angle=${config.padAngle},`
      + `sort-groups=${config.sortGroups},sort-subgroups=${config.sortSubgroups},symmetric=false]`,
  });
  return body;
}

async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: ["semio-viz-network"], body: statements(ctx) },
    { workDir: ctx.workDir },
  );
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
const scenario = {
  oracle: (ctx: AdapterContext) => ({ projection: d3Projection(ctx) }),
  subject: async (ctx: AdapterContext) => (await subject(ctx)),
};

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: { "plain-matrix": scenario, "padded-and-sorted": scenario },
});
// #endregion 🧭️Adapter
