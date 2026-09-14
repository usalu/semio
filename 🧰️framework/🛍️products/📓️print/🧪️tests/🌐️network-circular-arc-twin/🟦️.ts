// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizArcLayout, vizCircularLayout, type VizGraph } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../⭕️network-circular-arc/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;
const RADIUS = 10;
const SPACING = 6;

/** 🔮️ The oracle of `⭕️network-circular-arc`, reused so both subjects meet the same placements. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`network-circular-arc declares no oracle for ${id}`);
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

/** 🔢️ Rounds a placement onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

/** 📋️ One flattened `index, x, y` triple per node, one-based as the probe numbers them. */
function placements(nodes: readonly { x: number; y: number }[]): number[] {
  return grid(nodes.flatMap((node, index) => [index + 1, node.x, node.y]));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "circular-equal-spacing": {
      oracle: oracle("circular-equal-spacing"),
      /** 🎯️ The twin's ring, started at three o'clock so both sides measure the same angle zero. */
      subject: (ctx: AdapterContext) => ({ projection: { "geometry/layout/circular": placements(vizCircularLayout(graph(ctx), { radius: RADIUS, startAngle: Math.PI / 2 })) } }),
    },
    "arc-spacing": {
      oracle: oracle("arc-spacing"),
      /** 🎯️ The twin's axis, whose length is the spacing times the gaps between the nodes. */
      subject: (ctx: AdapterContext) => {
        const g = graph(ctx);
        return { projection: { "geometry/layout/arc": placements(vizArcLayout(g, { length: SPACING * Math.max(0, g.nodes.length - 1) })) } };
      },
    },
  },
});
// #endregion 🧭️Adapter
