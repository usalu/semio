// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { VizForceSimulation, forceVizCenter, forceVizCollide, forceVizLink, forceVizManyBody, forceVizX, forceVizY, type VizForceNode } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🧲️network-force/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

type Edge = Readonly<{ source: string; target: string }>;

/** ⚙️ What one scenario asks the simulation for; the feature's prose is the normative statement. */
type Setup = Readonly<{
  iterations: number;
  forces: readonly string[];
  linkDistance: number;
  charge: number;
  collideRadius: number;
  collideStrength: number;
  axisStrength: number;
}>;

const DEFAULTS: Setup = {
  iterations: 20,
  forces: ["link", "many-body", "center"],
  linkDistance: 30,
  charge: -30,
  collideRadius: 1,
  collideStrength: 0.7,
  axisStrength: 0.1,
};

const SETUPS: Readonly<Record<string, Setup>> = {
  "phyllotaxis-start": { ...DEFAULTS, iterations: 0 },
  "default-forces-20-ticks": DEFAULTS,
  "collide-and-axis-forces": { ...DEFAULTS, iterations: 12, forces: ["link", "many-body", "collide", "x", "y"] },
  "parameterised-link-and-charge": { ...DEFAULTS, iterations: 15, linkDistance: 45, charge: -70 },
};

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function edges(ctx: AdapterContext): Edge[] {
  return rows(ctx).map((row) => {
    if (row.source === undefined || row.target === undefined) throw new Error(`scenario ${ctx.scenario.id} carries a row without source and target`);
    return { source: row.source, target: row.target };
  });
}

/** 🔵 Node ids in first-appearance order, source before target — the order the kernel derives too. */
function nodeIds(list: readonly Edge[]): string[] {
  const seen: string[] = [];
  for (const edge of list) for (const id of [edge.source, edge.target]) if (!seen.includes(id)) seen.push(id);
  return seen;
}

function setup(ctx: AdapterContext): Setup {
  const found = SETUPS[ctx.scenario.id];
  if (found === undefined) throw new Error(`scenario ${ctx.scenario.id} has no simulation setup`);
  return found;
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}

/** 🔮️ The oracle of `network-force`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`network-force declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🎯️Subject
/** 🎯️ The twin's `VizForceSimulation`, stopped and ticked by hand, with many-body made exact. */
function twinPositions(ctx: AdapterContext): number[] {
  const list = edges(ctx);
  const config = setup(ctx);
  const ids = nodeIds(list);
  const nodes: (VizForceNode & { id: string })[] = ids.map((id) => ({ id, index: 0, x: Number.NaN, y: Number.NaN, vx: Number.NaN, vy: Number.NaN }));
  const links = list.map((edge) => ({ source: ids.indexOf(edge.source), target: ids.indexOf(edge.target) }));
  const simulation = new VizForceSimulation(nodes);
  for (const name of config.forces) {
    if (name === "link") simulation.force("link", forceVizLink(links as never[], { distance: config.linkDistance }));
    else if (name === "many-body") simulation.force("many-body", forceVizManyBody({ theta: 0, strength: config.charge }));
    else if (name === "center") simulation.force("center", forceVizCenter(0, 0));
    else if (name === "collide") simulation.force("collide", forceVizCollide({ radius: config.collideRadius, strength: config.collideStrength }));
    else if (name === "x") simulation.force("x", forceVizX({ strength: config.axisStrength }));
    else if (name === "y") simulation.force("y", forceVizY({ strength: config.axisStrength }));
    else throw new Error(`unknown force ${name}`);
  }
  if (config.iterations > 0) simulation.tick(config.iterations);
  const flat: number[] = [];
  for (const node of nodes) flat.push(node.index! + 1, node.x, node.y);
  return flat;
}
// #endregion 🎯️Subject

// #region 🧭️Adapter
const scenario = (id: string) => ({
  oracle: oracle(id),
  /** 🎯️ The twin kernel's force layout on the scenario's own edge list and force list. */
  subject: (ctx: AdapterContext) => ({ projection: { "geometry/layout/force": grid(twinPositions(ctx)) } }),
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "phyllotaxis-start": scenario("phyllotaxis-start"),
    "default-forces-20-ticks": scenario("default-forces-20-ticks"),
    "collide-and-axis-forces": scenario("collide-and-axis-forces"),
    "parameterised-link-and-charge": scenario("parameterised-link-and-charge"),
  },
});
// #endregion 🧭️Adapter
