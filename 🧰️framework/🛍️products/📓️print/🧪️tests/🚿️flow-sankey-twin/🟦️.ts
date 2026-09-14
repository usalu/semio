// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizSankey, type VizSankeyAlignment } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🚰️flow-sankey/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;
const NODE_WIDTH = 0.05;
const NODE_PADDING = 0.03;
const ITERATIONS = 6;

const ALIGNS: Readonly<Record<string, VizSankeyAlignment>> = {
  "align-justify": "justify",
  "align-left": "left",
  "align-right": "right",
  "align-center": "center",
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

function align(ctx: AdapterContext): VizSankeyAlignment {
  const found = ALIGNS[ctx.scenario.id];
  if (found === undefined) throw new Error(`scenario ${ctx.scenario.id} has no alignment`);
  return found;
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

/** 🔮️ The oracle of `flow-sankey`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`flow-sankey declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🎯️Subject
/** 🎯️ The twin's `vizSankey`, in the same unit extent and with the same four alignments. */
function twinProjection(ctx: AdapterContext): { projection: Record<string, number[]> } {
  const nodes = nodeIds(ctx).map((id) => ({ name: id }));
  const links = rows(ctx).map((row) => ({ source: row.source!, target: row.target!, value: Number(row.value) }));
  const layout = vizSankey({ nodes, links }, {
    nodeWidth: NODE_WIDTH,
    nodePadding: NODE_PADDING,
    align: align(ctx),
    iterations: ITERATIONS,
    extent: [[0, 0], [1, 1]],
  });
  const nodeValues: number[] = [];
  for (const node of layout.nodes) nodeValues.push(node.index + 1, node.x0, node.y0, node.x1, node.y1);
  const ordered = [...layout.links].sort((a, b) => a.index - b.index);
  const linkValues: number[] = [];
  for (const link of ordered) linkValues.push(link.index + 1, link.y0, link.y1, link.width);
  return { projection: { "geometry/layout/sankey-node": grid(nodeValues), "geometry/layout/sankey-link": grid(linkValues) } };
}
// #endregion 🎯️Subject

// #region 🧭️Adapter
const scenario = (id: string) => ({
  oracle: oracle(id),
  /** 🎯️ The twin kernel's sankey layout on the scenario's own flow and alignment. */
  subject: (ctx: AdapterContext) => twinProjection(ctx),
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "align-justify": scenario("align-justify"),
    "align-left": scenario("align-left"),
    "align-right": scenario("align-right"),
    "align-center": scenario("align-center"),
  },
});
// #endregion 🧭️Adapter
