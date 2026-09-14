// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizChord } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🪢️network-chord/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
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

function comparator(name: string): ((a: number, b: number) => number) | undefined {
  if (name === "ascending") return (a, b) => (a < b ? -1 : a > b ? 1 : 0);
  if (name === "descending") return (a, b) => (b < a ? -1 : b > a ? 1 : 0);
  return undefined;
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

/** 🔮️ The oracle of `network-chord`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`network-chord declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🎯️Subject
/** 🎯️ The twin's `vizChord` over the same matrix and the same padding and sorting. */
function twinProjection(ctx: AdapterContext): { projection: Record<string, number[]> } {
  const config = options(ctx);
  const layout = vizChord(matrix(ctx), {
    padAngle: config.padAngle,
    sortGroups: comparator(config.sortGroups),
    sortSubgroups: comparator(config.sortSubgroups),
  });
  const groupValues: number[] = [];
  for (const group of layout.groups) groupValues.push(group.index + 1, group.startAngle, group.endAngle, group.value);
  const ribbonValues: number[] = [];
  let ribbon = 0;
  for (const chord of layout.chords) {
    ribbon += 1;
    ribbonValues.push(ribbon, chord.source.index + 1, chord.target.index + 1,
      chord.source.startAngle, chord.source.endAngle, chord.target.startAngle, chord.target.endAngle);
  }
  return { projection: { "geometry/layout/chord-group": grid(groupValues), "geometry/layout/chord-ribbon": grid(ribbonValues) } };
}
// #endregion 🎯️Subject

// #region 🧭️Adapter
const scenario = (id: string) => ({
  oracle: oracle(id),
  /** 🎯️ The twin kernel's chord layout on the scenario's own weighted edge list. */
  subject: (ctx: AdapterContext) => twinProjection(ctx),
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: { "plain-matrix": scenario("plain-matrix"), "padded-and-sorted": scenario("padded-and-sorted") },
});
// #endregion 🧭️Adapter
