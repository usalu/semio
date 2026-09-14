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
const CASE = "network-circular-arc";
const DECIMALS = 6;
const TAU = 6.283185307179586;
const RADIUS = 10;
const INNER_RADIUS = 3;
const RADIUS_STEP = 5;
const SHELLS = [2, 4] as const;
const SPACING = 6;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

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

/** 🔢️ Rounds an expectation onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}
// #endregion 🧫️Vectors

// #region 📋️Specification
/** 📋️ Circular: equal angles over the full turn, built from `d3-array`'s own index sequence. */
function circularExpectation(ctx: AdapterContext): number[] {
  const count = nodeIds(ctx).length;
  const flat: number[] = [];
  for (const k of range(count)) flat.push(k + 1, RADIUS * Math.cos((TAU * k) / count), RADIUS * Math.sin((TAU * k) / count));
  return flat;
}

/** 📋️ Shell: the ordering cut into the declared rings, each ring itself equally spaced. */
function shellExpectation(ctx: AdapterContext): number[] {
  const count = nodeIds(ctx).length;
  const flat: number[] = [];
  let start = 0;
  SHELLS.forEach((size, shell) => {
    const radius = INNER_RADIUS + RADIUS_STEP * shell;
    for (const j of range(size)) {
      const index = start + j;
      if (index >= count) return;
      flat.push(index + 1, radius * Math.cos((TAU * j) / size), radius * Math.sin((TAU * j) / size));
    }
    start += size;
  });
  return flat;
}

/** 📋️ Arc: the ordering laid out along one axis at a constant spacing. */
function arcExpectation(ctx: AdapterContext): number[] {
  const flat: number[] = [];
  for (const k of range(nodeIds(ctx).length)) flat.push(k + 1, SPACING * k, 0);
  return flat;
}

/** 📋️ Adjacency: only the permutation, as the feature's table names it. */
function rankExpectation(ctx: AdapterContext): number[] {
  const table: Record<string, string> = {};
  for (const row of rows(ctx)) if ((row.node ?? "") !== "") table[row.node!] = row.rank!;
  const flat: number[] = [];
  nodeIds(ctx).forEach((id, index) => {
    const rank = table[id];
    if (rank === undefined) throw new Error(`scenario ${ctx.scenario.id} names no expected rank for ${id}`);
    flat.push(index + 1, Number(rank));
  });
  return flat;
}
// #endregion 📋️Specification

// #region 🧪️Probe
function preamble(ctx: AdapterContext): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizTable{edges}{source,target}" }];
  for (const edge of edges(ctx)) body.push({ raw: `\\SemioVizRow{edges}{${edge.source},${edge.target}}` });
  body.push({ raw: "\\SemioVizGraph{edges}[source=source,target=target]" });
  body.push({ raw: "\\SemioVizProbeOn" });
  return body;
}

async function subject(ctx: AdapterContext, call: string, key: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: ["semio-viz-network"], body: [...preamble(ctx), { raw: call }] },
    { workDir: ctx.workDir },
  );
  const projection = probeProjection(roundProbeNumbers(records, DECIMALS));
  return { projection: { [key]: projection[key] ?? [] } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "circular-equal-spacing": {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/circular": grid(circularExpectation(ctx)) } }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, `\\SemioVizLayout{circular}{edges}{ring}[radius=${RADIUS},order=index]`, "geometry/layout/circular")),
    },
    "shell-rings": {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/shell": grid(shellExpectation(ctx)) } }),
      subject: async (ctx: AdapterContext) => (await subject(
          ctx,
          `\\SemioVizLayout{shell}{edges}{rings}[shells={${SHELLS.join(",")}},inner-radius=${INNER_RADIUS},radius-step=${RADIUS_STEP},order=index]`,
          "geometry/layout/shell",
        )),
    },
    "arc-spacing": {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/arc": grid(arcExpectation(ctx)) } }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, `\\SemioVizLayout{arc}{edges}{axis}[spacing=${SPACING},order=index]`, "geometry/layout/arc")),
    },
    "adjacency-degree-order": {
      oracle: (ctx: AdapterContext) => ({ projection: { "geometry/layout/adjacency": rankExpectation(ctx) } }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, "\\SemioVizLayout{adjacency}{edges}{order}[order=degree]", "geometry/layout/adjacency")),
    },
  },
});
// #endregion 🧭️Adapter
