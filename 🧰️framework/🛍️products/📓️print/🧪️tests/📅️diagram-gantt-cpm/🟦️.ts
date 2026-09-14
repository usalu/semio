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
const CASE = "diagram-gantt-cpm";
const DECIMALS = 4;

type Task = { id: string; duration: number; deps: string[] };

/** 🧫️ The task network the scenario declares. */
function tasks(ctx: AdapterContext): Task[] {
  return rows(ctx).map((row) => ({
    id: row.id!,
    duration: Number(row.duration),
    deps: (row.deps ?? "").split(" ").map((dep) => dep.trim()).filter((dep) => dep.length > 0),
  }));
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
/** 🔮️ An independent critical-path implementation, written from the definitions in the feature:
 *  forward pass to ES/EF, project end, backward pass to LS/LF, total float as LS − ES. */
function criticalPath(network: readonly Task[]): number[] {
  const duration = new Map(network.map((task) => [task.id, task.duration]));
  const earliestStart = new Map(network.map((task) => [task.id, 0]));
  const earliestFinish = new Map<string, number>();
  for (let pass = 0; pass < network.length; pass += 1) {
    for (const task of network) {
      let start = 0;
      for (const dep of task.deps) start = Math.max(start, earliestFinish.get(dep) ?? 0);
      earliestStart.set(task.id, start);
      earliestFinish.set(task.id, start + (duration.get(task.id) ?? 0));
    }
  }
  const projectEnd = Math.max(0, ...network.map((task) => earliestFinish.get(task.id) ?? 0));
  const latestFinish = new Map(network.map((task) => [task.id, projectEnd]));
  const latestStart = new Map<string, number>();
  for (let pass = 0; pass < network.length; pass += 1) {
    for (const task of network) latestStart.set(task.id, (latestFinish.get(task.id) ?? 0) - (duration.get(task.id) ?? 0));
    for (const task of network) {
      const start = latestStart.get(task.id) ?? 0;
      for (const dep of task.deps) if (start < (latestFinish.get(dep) ?? Infinity)) latestFinish.set(dep, start);
    }
  }
  const flat: number[] = [];
  for (const task of network) {
    const es = earliestStart.get(task.id) ?? 0;
    const ls = latestStart.get(task.id) ?? 0;
    flat.push(es, earliestFinish.get(task.id) ?? 0, ls, latestFinish.get(task.id) ?? 0, ls - es);
  }
  return flat;
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const body: VizProbeStatement[] = [{ raw: "\\SemioVizTable{t-tasks}{id,label,duration,deps}" }];
  for (const task of tasks(ctx)) body.push({ raw: `\\SemioVizRow{t-tasks}{${task.id},${task.id},${task.duration},${task.deps.join(" ")}}` });
  body.push({ raw: FRAME.open });
  body.push({ raw: "\\SemioVizRunFamily{pm-gantt}[data=t-tasks,width=80,height=48,axis=0]" });
  body.push({ raw: FRAME.close });
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: DIAGRAM_PACKAGES, preamble: DIAGRAM_PREAMBLE, geometry: true, body },
    { workDir: ctx.workDir },
  );
  const projection = probeProjection(roundProbeNumbers(records, DECIMALS));
  return { projection: { "geometry/diagram-cpm": projection["geometry/diagram-cpm"] ?? [] } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "forward-and-backward-pass": {
      /** 🔮️ The independent CPM reference over the same network. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-cpm": criticalPath(tasks(ctx)) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "independent-cpm-reference": {
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-cpm": criticalPath(tasks(ctx)) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
