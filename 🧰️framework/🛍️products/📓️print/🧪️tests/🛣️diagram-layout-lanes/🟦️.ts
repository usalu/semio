// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, roundProjectionNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import { DIAGRAM_PREAMBLE, DIAGRAM_PACKAGES, FRAME, rows } from "../🚦️diagram-routing/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "diagram-layout-lanes";
const DECIMALS = 4;

/** 🧾️ The node table every scenario shares, as `\SemioVizRow` statements. */
function nodeTable(ctx: AdapterContext): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ raw: "\\SemioVizTable{t-nodes}{id,label,row,col,lane}" }];
  for (const row of rows(ctx)) body.push({ raw: `\\SemioVizRow{t-nodes}{${row.id},${row.id},${row.row},${row.col},${row.lane}}` });
  return body;
}

/** 📋️ The centres and sizes the scenario writes out, flattened in table order. */
function namedNodes(ctx: AdapterContext): number[] {
  const flat: number[] = [];
  for (const row of rows(ctx)) flat.push(Number(row.cx), Number(row.cy), Number(row.w), Number(row.h));
  return flat;
}

/** 📋️ The lane bands the scenario writes out; a blank row is a lane that repeats an earlier one. */
function namedLanes(ctx: AdapterContext): number[] {
  const flat: number[] = [];
  for (const row of rows(ctx)) if ((row.bottom ?? "") !== "") flat.push(Number(row.bottom), Number(row.top));
  return flat;
}
// #endregion 🧫️Vectors

// #region 🧪️Probe
/** 🎯️ Runs the kernel on the scenario's node table and returns one geometry key. */
async function subject(ctx: AdapterContext, options: string, key: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    {
      case: CASE,
      scenario: ctx.scenario.id,
      packages: DIAGRAM_PACKAGES,
      preamble: DIAGRAM_PREAMBLE,
      geometry: true,
      body: [
        ...nodeTable(ctx),
        { raw: FRAME.open },
        { raw: `\\SemioVizDiagram[nodes=t-nodes,edges=,width=80,height=48,pad=3,col-gap=4,row-gap=4,${options}]` },
        { raw: FRAME.close },
      ],
    },
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
    "grid-placement": {
      /** 📋️ The millimetres the placement formula gives for this table. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-node": namedNodes(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, "routing=orthogonal", "geometry/diagram-node")),
    },
    "direction-transposes-the-grid": {
      /** 📋️ The same formula on the transposed grid. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-node": namedNodes(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, "direction=right", "geometry/diagram-node")),
    },
    "lane-bands": {
      /** 📋️ The band edges the feature specifies, one pair per distinct lane. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-lane": namedLanes(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, "lanes=row", "geometry/diagram-lane")),
    },
  },
});
// #endregion 🧭️Adapter
