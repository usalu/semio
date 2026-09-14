// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { range } from "d3-array";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "network-algorithms";
const DECIMALS = 0;

type Graph = Readonly<{ count: number; edges: readonly (readonly [number, number])[] }>;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function edgeRows(ctx: AdapterContext): { source: string; target: string }[] {
  return rows(ctx)
    .filter((row) => (row.source ?? "") !== "" && (row.target ?? "") !== "")
    .map((row) => ({ source: row.source!, target: row.target! }));
}

/** 🔵 Node ids in first appearance order, source before target — the order the kernel derives too. */
function nodeIds(ctx: AdapterContext): string[] {
  const seen: string[] = [];
  for (const edge of edgeRows(ctx)) for (const id of [edge.source, edge.target]) if (!seen.includes(id)) seen.push(id);
  return seen;
}

/** 🕸️ The graph as zero-based index pairs in declaration order. */
function graph(ctx: AdapterContext): Graph {
  const ids = nodeIds(ctx);
  return { count: ids.length, edges: edgeRows(ctx).map((edge) => [ids.indexOf(edge.source), ids.indexOf(edge.target)] as const) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
/** 🔮️ An independent reading of the same definitions: adjacency arrays over `d3-array`'s index range. */
function adjacency(g: Graph, direction: "both" | "out"): number[][] {
  const lists = range(g.count).map(() => [] as number[]);
  for (const [source, target] of g.edges) {
    lists[source]!.push(target);
    if (direction === "both") lists[target]!.push(source);
  }
  return lists;
}

function degrees(g: Graph): number[] {
  const flat: number[] = [];
  const total = range(g.count).map(() => 0);
  const incoming = range(g.count).map(() => 0);
  const outgoing = range(g.count).map(() => 0);
  for (const [source, target] of g.edges) {
    total[source] += 1;
    total[target] += 1;
    outgoing[source] += 1;
    incoming[target] += 1;
  }
  for (const index of range(g.count)) flat.push(index + 1, total[index]!, incoming[index]!, outgoing[index]!);
  return flat;
}

function components(g: Graph): number[] {
  const lists = adjacency(g, "both");
  const label = range(g.count).map(() => 0);
  let current = 0;
  for (const start of range(g.count)) {
    if (label[start] !== 0) continue;
    current += 1;
    const queue = [start];
    label[start] = current;
    while (queue.length > 0) {
      const node = queue.shift()!;
      for (const next of lists[node]!) if (label[next] === 0) { label[next] = current; queue.push(next); }
    }
  }
  const flat: number[] = [];
  for (const index of range(g.count)) flat.push(index + 1, label[index]!);
  return flat;
}

function topological(g: Graph): number[] {
  const lists = adjacency(g, "out");
  const pending = range(g.count).map(() => 0);
  for (const [, target] of g.edges) pending[target] += 1;
  const done = range(g.count).map(() => false);
  const flat: number[] = [];
  for (const step of range(g.count)) {
    const next = range(g.count).find((index) => !done[index] && pending[index] === 0);
    if (next === undefined) break;
    done[next] = true;
    for (const successor of lists[next]!) pending[successor] -= 1;
    flat.push(step + 1, next + 1);
  }
  return flat;
}

function breadthFirst(g: Graph): number[] {
  const lists = adjacency(g, "both");
  const seen = range(g.count).map(() => false);
  const depth = range(g.count).map(() => 0);
  const flat: number[] = [];
  let step = 0;
  for (const start of range(g.count)) {
    if (seen[start]) continue;
    seen[start] = true;
    const queue = [start];
    while (queue.length > 0) {
      const node = queue.shift()!;
      step += 1;
      flat.push(step, node + 1, depth[node]!);
      for (const next of lists[node]!) if (!seen[next]) { seen[next] = true; depth[next] = depth[node]! + 1; queue.push(next); }
    }
  }
  return flat;
}

function depthFirst(g: Graph): number[] {
  const lists = adjacency(g, "both");
  const seen = range(g.count).map(() => false);
  const flat: number[] = [];
  let step = 0;
  for (const start of range(g.count)) {
    if (seen[start]) continue;
    const stack = [start];
    while (stack.length > 0) {
      const node = stack.shift()!;
      if (seen[node]) continue;
      seen[node] = true;
      step += 1;
      flat.push(step, node + 1);
      stack.unshift(...lists[node]!);
    }
  }
  return flat;
}

/** 📋️ The generator stream the feature writes out. */
function namedStream(ctx: AdapterContext): number[] {
  return rows(ctx).map((row) => Number(row.state));
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
function preamble(ctx: AdapterContext): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizTable{edges}{source,target}" }];
  for (const edge of edgeRows(ctx)) body.push({ raw: `\\SemioVizRow{edges}{${edge.source},${edge.target}}` });
  body.push({ raw: "\\SemioVizGraph{edges}[source=source,target=target]" });
  body.push({ raw: "\\SemioVizProbeOn" });
  return body;
}

async function records(ctx: AdapterContext, body: readonly VizProbeStatement[]): Promise<{ projection: ProbeProjection }> {
  const parsed = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: ["semio-viz-network"], body: [...body] },
    { workDir: ctx.workDir },
  );
  return { projection: probeProjection(roundProbeNumbers(parsed, DECIMALS)) };
}

/** 🎯️ Keeps the named columns of a fixed-width record stream. */
function columns(values: readonly (number | string)[], width: number, kept: readonly number[]): (number | string)[] {
  const out: (number | string)[] = [];
  for (let index = 0; index < values.length; index += width) for (const column of kept) out.push(values[index + column]!);
  return out;
}

async function metrics(ctx: AdapterContext, kept: readonly number[]): Promise<{ projection: ProbeProjection }> {
  const projection = (await records(ctx, [...preamble(ctx), { raw: "\\SemioVizGraphMetrics{edges}{metrics}" }])).projection;
  return { projection: { "geometry/graph/metrics": columns(projection["geometry/graph/metrics"] ?? [], 6, kept) } };
}

async function order(ctx: AdapterContext, mode: string, kept: readonly number[]): Promise<{ projection: ProbeProjection }> {
  const projection = (await records(ctx, [...preamble(ctx), { raw: `\\SemioVizGraphOrder{edges}{order}[mode=${mode}]` }])).projection;
  return { projection: { "geometry/graph/order": columns(projection["geometry/graph/order"] ?? [], 3, kept) } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    degrees: {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/graph/metrics": degrees(graph(ctx)) } }),
      subject: async (ctx: AdapterContext) => (await metrics(ctx, [0, 1, 2, 3])),
    },
    "connected-components": {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/graph/metrics": components(graph(ctx)) } }),
      subject: async (ctx: AdapterContext) => (await metrics(ctx, [0, 4])),
    },
    "topological-order": {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/graph/order": topological(graph(ctx)) } }),
      subject: async (ctx: AdapterContext) => (await order(ctx, "topological", [0, 1])),
    },
    "traversal-order": {
      oracle: (ctx: AdapterContext) => ({
        projection: {
          "geometry/graph/order": breadthFirst(graph(ctx)),
          "geometry/graph/order-depth-first": depthFirst(graph(ctx)),
        },
      }),
      subject: async (ctx: AdapterContext) => {
        const breadth = (await order(ctx, "bfs", [0, 1, 2])).projection;
        const depth = (await order(ctx, "dfs", [0, 1])).projection;
        return {
          projection: {
            "geometry/graph/order": breadth["geometry/graph/order"] ?? [],
            "geometry/graph/order-depth-first": depth["geometry/graph/order"] ?? [],
          },
        };
      },
    },
    "lcg-stream": {
      /** 📋️ The stream the feature writes out; it is d3-force's generator, seeded with one. */
      oracle: (ctx: AdapterContext) => ({ projection: { random: namedStream(ctx) } }),
      subject: async (ctx: AdapterContext) => (await records(ctx, [
          { precision: DECIMALS },
          { raw: "\\SemioVizGraphSeed{1}" },
          { raw: "\\ExplSyntaxOn" },
          { raw: "\\int_step_inline:nn { 5 } { \\SemioVizGraphRandom \\semio_viz_probe_values:nx { random } { \\fp_use:N \\g_semio_viz_network_lcg_fp } }" },
          { raw: "\\ExplSyntaxOff" },
        ])),
    },
  },
});
// #endregion 🧭️Adapter
