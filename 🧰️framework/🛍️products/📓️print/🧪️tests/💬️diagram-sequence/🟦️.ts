// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, roundProjectionNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import { DIAGRAM_PACKAGES, DIAGRAM_PREAMBLE, FRAME, rows } from "../🚦️diagram-routing/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "diagram-sequence";
const DECIMALS = 4;
const PARTICIPANTS = ["p", "q", "r"] as const;

/** 📋️ The lifeline x and message y the scenario writes out, flattened in message order. */
function namedMessages(ctx: AdapterContext): number[] {
  const flat: number[] = [];
  for (const row of rows(ctx)) flat.push(Number(row.xfrom), Number(row.y), Number(row.xto), Number(row.y));
  return flat;
}
// #endregion 🧫️Vectors

// #region 🧪️Probe
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const body: VizProbeStatement[] = [{ raw: "\\SemioVizTable{t-parts}{id,label,shape}" }];
  for (const id of PARTICIPANTS) body.push({ raw: `\\SemioVizRow{t-parts}{${id},${id.toUpperCase()},process}` });
  body.push({ raw: "\\SemioVizTable{t-msgs}{from,to,label,kind,activate}" });
  for (const row of rows(ctx)) body.push({ raw: `\\SemioVizRow{t-msgs}{${row.from},${row.to},m,${row.kind},}` });
  body.push({ raw: FRAME.open });
  body.push({ raw: "\\SemioVizRunFamily{uml-sequence}[data=t-parts,messages=t-msgs,width=80,height=48,head=6,step=0]" });
  body.push({ raw: FRAME.close });
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: DIAGRAM_PACKAGES, preamble: DIAGRAM_PREAMBLE, geometry: true, body },
    { workDir: ctx.workDir },
  );
  const projection = probeProjection(roundProbeNumbers(records, DECIMALS));
  return { projection: { "geometry/diagram-message": projection["geometry/diagram-message"] ?? [] } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "message-y-positions": {
      /** 📋️ The lifeline positions and message steps the feature specifies. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-message": namedMessages(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "self-call-keeps-its-step": {
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-message": namedMessages(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
