// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizLayeredLayout, type VizGraph } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🎚️network-layered/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const SWEEPS = 4;

/** 🔮️ The oracle of `🎚️network-layered`, reused so both subjects meet the same ranks. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`network-layered declares no oracle for ${id}`);
  return handler;
}

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🕸️ The graph of the scenario table: node ids in first-appearance order, source before target. */
function graph(ctx: AdapterContext): VizGraph {
  const edges = rows(ctx)
    .filter((row) => (row.source ?? "") !== "" && (row.target ?? "") !== "")
    .map((row) => [row.source!, row.target!] as [string, string]);
  const nodes: string[] = [];
  for (const [source, target] of edges) for (const id of [source, target]) if (!nodes.includes(id)) nodes.push(id);
  return { nodes, edges };
}

/** 📋️ One flattened record per node in declaration order; the index and the within-layer position
 *  are one-based as the probe numbers them, the layer zero-based as both sides count it. */
function layered(ctx: AdapterContext, withPosition: boolean): number[] {
  const g = graph(ctx);
  const layout = vizLayeredLayout(g, { sweeps: SWEEPS });
  const byId = new Map(layout.nodes.map((node) => [node.id, node] as const));
  return g.nodes.flatMap((id, index) => {
    const node = byId.get(id);
    if (node === undefined) throw new Error(`the twin placed no node for ${id}`);
    return withPosition ? [index + 1, node.layer ?? 0, (node.order ?? 0) + 1] : [index + 1, node.layer ?? 0];
  });
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "ranks-agree-with-dagre": {
      oracle: oracle("ranks-agree-with-dagre"),
      /** 🎯️ The twin's longest-path layering of the same directed acyclic graph. */
      subject: (ctx: AdapterContext) => ({ projection: { "geometry/layout/layered": layered(ctx, false) } }),
    },
    "slack-ranks-are-as-soon-as-possible": {
      oracle: oracle("slack-ranks-are-as-soon-as-possible"),
      /** 🎯️ The twin's layers, which the specification says are as soon as possible. */
      subject: (ctx: AdapterContext) => ({ projection: { "geometry/layout/layered": layered(ctx, false) } }),
    },
    "barycenter-ordering": {
      oracle: oracle("barycenter-ordering"),
      /** 🎯️ The twin's layer and within-layer position after the same four barycentre sweeps. */
      subject: (ctx: AdapterContext) => ({ projection: { "geometry/layout/layered": layered(ctx, true) } }),
    },
  },
});
// #endregion 🧭️Adapter
