// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { forceCenter, forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY } from "d3-force";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "network-force";
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

/** 🔢️ Rounds an oracle's own numbers onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
/** 🔮️ d3-force itself, stopped and ticked by hand, with the many-body force made exact. */
function d3Positions(ctx: AdapterContext): number[] {
  const list = edges(ctx);
  const config = setup(ctx);
  const nodes = nodeIds(list).map((id) => ({ id }));
  const links = list.map((edge) => ({ source: edge.source, target: edge.target }));
  const simulation = forceSimulation(nodes as never[]).stop();
  for (const name of config.forces) {
    if (name === "link") simulation.force("link", forceLink(links as never[]).id((node: never) => (node as { id: string }).id).distance(config.linkDistance));
    else if (name === "many-body") simulation.force("many-body", forceManyBody().theta(0).strength(config.charge));
    else if (name === "center") simulation.force("center", forceCenter());
    else if (name === "collide") simulation.force("collide", forceCollide(config.collideRadius).strength(config.collideStrength));
    else if (name === "x") simulation.force("x", forceX().strength(config.axisStrength));
    else if (name === "y") simulation.force("y", forceY().strength(config.axisStrength));
    else throw new Error(`unknown force ${name}`);
  }
  if (config.iterations > 0) simulation.tick(config.iterations);
  const flat: number[] = [];
  for (const node of nodes as unknown as { index: number; x: number; y: number }[]) flat.push(node.index + 1, node.x, node.y);
  return flat;
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
/** 🧪️ The probe document: the same edge list, the same forces, the kernel's own `force` layout. */
function statements(ctx: AdapterContext): VizProbeStatement[] {
  const list = edges(ctx);
  const config = setup(ctx);
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizTable{edges}{source,target}" }];
  for (const edge of list) body.push({ raw: `\\SemioVizRow{edges}{${edge.source},${edge.target}}` });
  body.push({ raw: "\\SemioVizGraph{edges}[source=source,target=target]" });
  body.push({ raw: "\\SemioVizProbeOn" });
  body.push({
    raw: `\\SemioVizLayout{force}{edges}{positions}[iterations=${config.iterations},`
      + `forces={${config.forces.join(", ")}},link-distance=${config.linkDistance},`
      + `many-body-strength=${config.charge},collide-radius=${config.collideRadius},`
      + `collide-strength=${config.collideStrength},x-strength=${config.axisStrength},`
      + `y-strength=${config.axisStrength},seed=1]`,
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
  oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/force": grid(d3Positions(ctx)) } }),
  subject: async (ctx: AdapterContext) => (await subject(ctx)),
};

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "phyllotaxis-start": scenario,
    "default-forces-20-ticks": scenario,
    "collide-and-axis-forces": scenario,
    "parameterised-link-and-charge": scenario,
  },
});
// #endregion 🧭️Adapter
