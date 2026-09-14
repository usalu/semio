// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, roundProjectionNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "diagram-routing";
const DECIMALS = 4;

/** 📦️ The LaTeX packages a diagram probe loads: the kernel plus the namespace families. */
export const DIAGRAM_PACKAGES = [
  "semio-viz-diagram-flowchart",
  "semio-viz-diagram-uml",
  "semio-viz-diagram-architecture",
  "semio-viz-diagram-process",
] as const;

/** 🎨️ A probe document has no `semio` class, so the chrome aliases `semio-core` normally sets are
 *  bound here; the diagram families read their fills and strokes through them. */
export const DIAGRAM_PREAMBLE = [
  "\\colorlet{semio-chrome-base}{semio-chrome-light-base}",
  "\\colorlet{semio-chrome-window}{semio-chrome-light-window}",
  "\\colorlet{semio-chrome-canvas}{semio-chrome-light-canvas}",
  "\\colorlet{semio-chrome-panel}{semio-chrome-light-panel}",
  "\\colorlet{semio-chrome-border-normal}{semio-chrome-light-border-normal}",
  "\\colorlet{semio-chrome-border-emphasized}{semio-chrome-light-border-emphasized}",
  "\\colorlet{semio-chrome-active-base}{semio-chrome-light-active-base}",
  "\\colorlet{semio-chrome-active-foreground}{semio-chrome-light-active-foreground}",
  "\\colorlet{semio-chrome-foreground}{semio-chrome-light-foreground}",
  "\\colorlet{semio-chrome-text-normal}{semio-chrome-light-border-normal}",
  "\\ExplSyntaxOn",
  "\\NewDocumentEnvironment{VizProbeFrame}{}{ \\bool_set_true:N \\l_semio_viz_in_figure_bool \\begin{tikzpicture}[x=1mm,y=1mm] }{ \\end{tikzpicture} \\bool_set_false:N \\l_semio_viz_in_figure_bool }",
  "\\ExplSyntaxOff",
] as const;

/** 🖼️ The millimetre canvas a family draws into outside a full `VizFigure`. */
export const FRAME = { open: "\\begin{VizProbeFrame}", close: "\\end{VizProbeFrame}" } as const;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
export function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🔵 The four-node grid every routing scenario routes over; the same table as diagram-layout-lanes. */
const NODES: readonly (readonly [string, number, number])[] = [
  ["a", 1, 1],
  ["b", 1, 2],
  ["c", 2, 2],
  ["d", 3, 1],
];

function nodeTable(): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ raw: "\\SemioVizTable{t-nodes}{id,label,row,col}" }];
  for (const [id, row, col] of NODES) body.push({ raw: `\\SemioVizRow{t-nodes}{${id},${id},${row},${col}}` });
  return body;
}

function edgeTable(ctx: AdapterContext): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ raw: "\\SemioVizTable{t-edges}{from,to,kind,label}" }];
  for (const row of rows(ctx)) body.push({ raw: `\\SemioVizRow{t-edges}{${row.from},${row.to},plain,}` });
  return body;
}

/** 📋️ The waypoint lists the scenario writes out, each prefixed by its own point count. */
function namedRoutes(ctx: AdapterContext): number[] {
  const flat: number[] = [];
  for (const row of rows(ctx)) {
    flat.push(Number(row.count));
    for (const value of row.waypoints!.split(",")) flat.push(Number(value.trim()));
  }
  return flat;
}
// #endregion 🧫️Vectors

// #region 🧪️Probe
async function subject(ctx: AdapterContext, routing: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    {
      case: CASE,
      scenario: ctx.scenario.id,
      packages: DIAGRAM_PACKAGES,
      preamble: DIAGRAM_PREAMBLE,
      geometry: true,
      body: [
        ...nodeTable(),
        ...edgeTable(ctx),
        { raw: FRAME.open },
        { raw: `\\SemioVizDiagram[nodes=t-nodes,edges=t-edges,width=80,height=48,pad=3,col-gap=4,row-gap=4,routing=${routing}]` },
        { raw: FRAME.close },
      ],
    },
    { workDir: ctx.workDir },
  );
  const projection = probeProjection(roundProbeNumbers(records, DECIMALS));
  return { projection: { "geometry/diagram-route": projection["geometry/diagram-route"] ?? [] } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "orthogonal-vertical": {
      /** 📋️ The waypoints the orthogonal rule produces on this grid. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-route": namedRoutes(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, "orthogonal")),
    },
    "straight-ports": {
      /** 📋️ The rectangle/ray intersections the straight rule produces on this grid. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-route": namedRoutes(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx, "straight")),
    },
  },
});
// #endregion 🧭️Adapter
