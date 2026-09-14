// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import dagre from "dagre";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "network-layered";
const DECIMALS = 0;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** ➖ The edge rows; a table may carry fewer edges than expectation rows, so blanks are skipped. */
function edges(ctx: AdapterContext): { source: string; target: string }[] {
  return rows(ctx)
    .filter((row) => (row.source ?? "") !== "" && (row.target ?? "") !== "")
    .map((row) => ({ source: row.source!, target: row.target! }));
}

/** 🔵 Node ids in first-appearance order, source before target — the order the kernel derives too. */
function nodeIds(ctx: AdapterContext): string[] {
  const seen: string[] = [];
  for (const edge of edges(ctx)) for (const id of [edge.source, edge.target]) if (!seen.includes(id)) seen.push(id);
  return seen;
}

/** 📋️ The expectation rows of a conformance scenario, keyed by node id. */
function expectations(ctx: AdapterContext): Record<string, Record<string, string>> {
  const out: Record<string, Record<string, string>> = {};
  for (const row of rows(ctx)) if ((row.node ?? "") !== "") out[row.node!] = row;
  return out;
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
/** 🔮️ dagre with the same ranker; the rank is read back as the position of the node's
 * row among the distinct row centres, because dagre drops `rank` from the label once it has
 * turned it into a coordinate. */
function dagreLayers(ctx: AdapterContext): number[] {
  const graph = new dagre.graphlib.Graph();
  graph.setGraph({ ranker: "longest-path" });
  graph.setDefaultEdgeLabel(() => ({}));
  for (const id of nodeIds(ctx)) graph.setNode(id, { width: 1, height: 1 });
  for (const edge of edges(ctx)) graph.setEdge(edge.source, edge.target);
  dagre.layout(graph);
  const centres = nodeIds(ctx).map((id) => (graph.node(id) as unknown as { y: number }).y);
  const layers = [...new Set(centres)].sort((a, b) => a - b);
  const flat: number[] = [];
  centres.forEach((centre, index) => flat.push(index + 1, layers.indexOf(centre)));
  return flat;
}

/** 📋️ The layers the conformance table names, in node-index order. */
function namedLayers(ctx: AdapterContext): number[] {
  const table = expectations(ctx);
  const flat: number[] = [];
  nodeIds(ctx).forEach((id, index) => {
    const row = table[id];
    if (row === undefined) throw new Error(`scenario ${ctx.scenario.id} names no expected layer for ${id}`);
    flat.push(index + 1, Number(row.layer));
  });
  return flat;
}

/** 📋️ The layers and positions the conformance table names, in node-index order. */
function namedOrder(ctx: AdapterContext): number[] {
  const table = expectations(ctx);
  const flat: number[] = [];
  nodeIds(ctx).forEach((id, index) => {
    const row = table[id];
    if (row === undefined) throw new Error(`scenario ${ctx.scenario.id} names no expected position for ${id}`);
    flat.push(index + 1, Number(row.layer), Number(row.position));
  });
  return flat;
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
/** 🧪️ The probe document: the same edge list through the kernel's own `layered` layout. */
function statements(ctx: AdapterContext): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizTable{edges}{source,target}" }];
  for (const edge of edges(ctx)) body.push({ raw: `\\SemioVizRow{edges}{${edge.source},${edge.target}}` });
  body.push({ raw: "\\SemioVizGraph{edges}[source=source,target=target]" });
  body.push({ raw: "\\SemioVizProbeOn" });
  body.push({ raw: "\\SemioVizLayout{layered}{edges}{ranks}[sweeps=4,orientation=horizontal]" });
  return body;
}

/** 🎯️ The probe emits index, layer, position, x and y per node; a scenario keeps the columns it asserts. */
async function subject(ctx: AdapterContext, columns: readonly number[]): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: ["semio-viz-network"], body: statements(ctx) },
    { workDir: ctx.workDir },
  );
  const projection = probeProjection(roundProbeNumbers(records, DECIMALS));
  const values = projection["geometry/layout/layered"] ?? [];
  const kept: (number | string)[] = [];
  for (let index = 0; index < values.length; index += 5) for (const column of columns) kept.push(values[index + column]!);
  return { projection: { "geometry/layout/layered": kept } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "ranks-agree-with-dagre": {
      /** 🔮️ dagre's own longest-path ranker over the same DAG. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/layered": dagreLayers(ctx) } }),
      /** 🎯️ Node index and layer from the kernel's layered layout. */
      subject: async (ctx: AdapterContext) => (await subject(ctx, [0, 1])),
    },
    "slack-ranks-are-as-soon-as-possible": {
      /** 📋️ The as-soon-as-possible layers the feature specifies. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/layered": namedLayers(ctx) } }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, [0, 1])),
    },
    "barycenter-ordering": {
      /** 📋️ The layer and within-layer position the feature specifies. */
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/layered": namedOrder(ctx) } }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, [0, 1, 2])),
    },
  },
});
// #endregion 🧭️Adapter
