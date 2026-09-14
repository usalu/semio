// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { sankey as d3Sankey, sankeyCenter, sankeyJustify, sankeyLeft, sankeyRight } from "d3-sankey";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "flow-sankey";
const DECIMALS = 6;
const NODE_WIDTH = 0.05;
const NODE_PADDING = 0.03;
const ITERATIONS = 6;

const ALIGNS: Readonly<Record<string, string>> = {
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

function align(ctx: AdapterContext): string {
  const found = ALIGNS[ctx.scenario.id];
  if (found === undefined) throw new Error(`scenario ${ctx.scenario.id} has no alignment`);
  return found;
}

/** 🔢️ Rounds an oracle's own numbers onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
/** 🔮️ d3-sankey itself, in the same unit extent and with the same four alignments. */
function d3Projection(ctx: AdapterContext): { projection: ProbeProjection } {
  const alignments = { justify: sankeyJustify, left: sankeyLeft, right: sankeyRight, center: sankeyCenter };
  const nodes = nodeIds(ctx).map((id) => ({ id }));
  const links = rows(ctx).map((row) => ({ source: row.source!, target: row.target!, value: Number(row.value) }));
  const layout = d3Sankey<{ id: string }, { source: string; target: string; value: number }>()
    .nodeId((node) => node.id)
    .nodeWidth(NODE_WIDTH)
    .nodePadding(NODE_PADDING)
    .nodeAlign(alignments[align(ctx) as keyof typeof alignments])
    .iterations(ITERATIONS)
    .extent([[0, 0], [1, 1]]);
  const graph = layout({ nodes, links } as never);
  const nodeValues: number[] = [];
  for (const node of graph.nodes as unknown as { index: number; x0: number; y0: number; x1: number; y1: number }[]) {
    nodeValues.push(node.index + 1, node.x0, node.y0, node.x1, node.y1);
  }
  const ordered = [...(graph.links as unknown as { index: number; y0: number; y1: number; width: number }[])]
    .sort((a, b) => a.index - b.index);
  const linkValues: number[] = [];
  for (const link of ordered) linkValues.push(link.index + 1, link.y0, link.y1, link.width);
  return { "geometry/layout/sankey-node": grid(nodeValues), "geometry/layout/sankey-link": grid(linkValues) };
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
/** 🧪️ The probe document: the same flow through the kernel's own `sankey` layout. */
function statements(ctx: AdapterContext): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizTable{flow}{source,target,value}" }];
  for (const row of rows(ctx)) body.push({ raw: `\\SemioVizRow{flow}{${row.source},${row.target},${row.value}}` });
  body.push({ raw: "\\SemioVizGraph{flow}[source=source,target=target,weight=value]" });
  body.push({ raw: "\\SemioVizProbeOn" });
  body.push({
    raw: `\\SemioVizLayout{sankey}{flow}{geometry}[node-width=${NODE_WIDTH},node-padding=${NODE_PADDING},`
      + `node-align=${align(ctx)},iterations=${ITERATIONS},x0=0,y0=0,x1=1,y1=1]`,
  });
  return body;
}

async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: ["semio-viz-flow"], body: statements(ctx) },
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
  scenarios: {
    "align-justify": scenario,
    "align-left": scenario,
    "align-right": scenario,
    "align-center": scenario,
  },
});
// #endregion 🧭️Adapter
