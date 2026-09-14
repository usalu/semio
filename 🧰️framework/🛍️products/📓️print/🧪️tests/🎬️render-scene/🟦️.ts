// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { planVizChart, renderVizScene, renderVizTikz } from "../../🔨️modules/📊️viz-kernel/🖼️render/🟦️.ts";
import type { VizChartSpecification } from "../../🔨️modules/📊️viz-kernel/🧬️schema/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 📊️ The demo bar chart the whole case is written against — the specification, not a fixture. */
const DEMO_SPEC: VizChartSpecification = {
  width: 160,
  height: 100,
  margin: { top: 8, right: 8, bottom: 14, left: 16 },
  theme: { appearance: "light" },
  language: "en",
  tables: [
    {
      name: "demo",
      columns: ["label", "value"],
      rows: [
        { label: "a", value: 12 },
        { label: "b", value: 30 },
        { label: "c", value: 7 },
        { label: "d", value: 22 },
      ],
    },
  ],
  scales: [
    { name: "x", kind: "band", domain: ["a", "b", "c", "d"], range: [16, 152], options: { padding: 0.2 } },
    { name: "y", kind: "linear", domain: [0, 30], range: [86, 8] },
  ],
  coordinate: { kind: "cartesian" },
  layers: [
    { mark: "bar", data: "demo", encodings: { x: { column: "label", scale: "x" }, y: { column: "value", scale: "y" }, y2: { value: 86 } } },
    { mark: "point", data: "demo", options: { symbol: "diamond", size: 20 }, encodings: { x: { column: "label", scale: "x" }, y: { column: "value", scale: "y" } } },
    { mark: "text", data: "demo", encodings: { x: { column: "label", scale: "x" }, y: { value: 6 }, text: { column: "label" } } },
  ],
  guides: [
    { kind: "axis", scale: "x", orient: "bottom", ticks: 4 },
    { kind: "axis", scale: "y", orient: "left", ticks: 4, grid: true },
  ],
};

/** 🧫️ The scenario's data table as records — the feature owns every number this case fixes. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 📐️ The `key | values` table of a conformance scenario, as numbers. */
function specified(ctx: AdapterContext): Record<string, number[]> {
  return Object.fromEntries(rows(ctx).map((row) => [row.key!.trim(), row.values!.split(",").map((value) => Number(value.trim()))]));
}

/** ✅️ Compares the rendered projection against the specification and reports every disagreement,
 * because a recorded no-oracle decision means no other handler will do it. */
function assertMatches(expected: Record<string, number[]>, actual: Record<string, number[]>, decimals = 6): Record<string, number[]> {
  const round = (value: number): number => Math.round(value * 10 ** decimals) / 10 ** decimals;
  const problems: string[] = [];
  for (const [key, values] of Object.entries(expected)) {
    const produced = actual[key];
    if (produced === undefined) {
      problems.push(`${key} was specified but not rendered`);
      continue;
    }
    if (produced.length !== values.length) problems.push(`${key} has ${produced.length} values, the specification fixes ${values.length}`);
    values.forEach((value, index) => {
      if (round(produced[index] ?? Number.NaN) !== round(value)) problems.push(`${key}[${index}] rendered ${produced[index]}, the specification fixes ${value}`);
    });
  }
  if (problems.length > 0) throw new Error(problems.join("; "));
  return Object.fromEntries(Object.entries(actual).map(([key, values]) => [key, values.map(round)]));
}

/** 📊️ The scene's node kinds, counted. */
function sceneCounts(): Record<string, number> {
  const scene = renderVizScene(DEMO_SPEC);
  const counts: Record<string, number> = {};
  for (const node of scene.nodes) counts[node.node.kind] = (counts[node.node.kind] ?? 0) + 1;
  return counts;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "scene-graph-primitives": {
      /** 📐️ The specified scene: the frame, the primitive census and the two structural invariants. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ What the TypeScript kernel actually rendered. */
      subject: (ctx: AdapterContext) => {
        const scene = renderVizScene(DEMO_SPEC);
        const counts = sceneCounts();
        const solidFills = scene.nodes.map((node) => node.fill).filter((fill): fill is { kind: "solid"; color: readonly [number, number, number, number] } => fill?.kind === "solid");
        const projection = {
          size: [scene.width, scene.height],
          "count/line": [counts.line ?? 0],
          "count/text": [counts.text ?? 0],
          "count/rect": [counts.rect ?? 0],
          "count/path": [counts.path ?? 0],
          identity: [scene.nodes.every((node) => node.transform.join(",") === "1,0,0,1,0,0") ? 1 : 0],
          unitColors: [solidFills.length > 0 && solidFills.every((fill) => fill.color.every((channel) => channel >= 0 && channel <= 1)) ? 1 : 0],
        };
        return { projection: assertMatches(specified(ctx), projection) };
      },
    },
    "bar-rectangles": {
      /** 📐️ The rectangles the band and linear scales imply, written out. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The rectangles the scene graph carries, in node order. */
      subject: (ctx: AdapterContext) => {
        const scene = renderVizScene(DEMO_SPEC);
        const rects = scene.nodes.filter((node) => node.node.kind === "rect").map((node) => node.node as { x: number; y: number; width: number; height: number });
        const projection = Object.fromEntries(rects.map((rect, index) => [`rect/${index}`, [rect.x, rect.y, rect.width, rect.height]]));
        return { projection: assertMatches(specified(ctx), projection) };
      },
    },
    "tikz-mirrors-the-scene": {
      /** 📐️ One TikZ statement per resolved item, in a millimetre-scaled picture. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The emitted picture, measured against the same resolved item list. */
      subject: (ctx: AdapterContext) => {
        const tikz = renderVizTikz(DEMO_SPEC);
        const plan = planVizChart(DEMO_SPEC);
        const body = tikz.split("\n").filter((line) => line.startsWith("\\path") || line.startsWith("\\node"));
        const projection = {
          statements: [body.length],
          opens: [tikz.startsWith("\\begin{tikzpicture}[x=1mm,y=-1mm]") ? 1 : 0],
          closes: [tikz.trimEnd().endsWith("\\end{tikzpicture}") ? 1 : 0],
          unitless: [body.every((line) => !/\(\s*-?[\d.]+mm/.test(line)) ? 1 : 0],
        };
        if (body.length !== plan.items.length) throw new Error(`the TikZ emitter wrote ${body.length} statements for ${plan.items.length} resolved items`);
        return { projection: assertMatches(specified(ctx), projection) };
      },
    },
    "options-change-the-projection": {
      /** 📐️ Two option sets of one mark must not render identically. */
      oracle: (ctx: AdapterContext) => ({ projection: specified(ctx) }),
      /** 🎯️ The same chart at a wider band padding. */
      subject: (ctx: AdapterContext) => {
        const widths = (spec: VizChartSpecification): number[] => planVizChart(spec).items.filter((item) => item.kind === "rect").map((item) => (item as { width: number }).width);
        const narrow = widths(DEMO_SPEC);
        const padded = widths({ ...DEMO_SPEC, scales: [{ name: "x", kind: "band", domain: ["a", "b", "c", "d"], range: [16, 152], options: { padding: 0.6 } }, DEMO_SPEC.scales![1]!] });
        const projection = { differs: [narrow.length === padded.length && narrow.every((width, index) => width !== padded[index]) ? 1 : 0] };
        return { projection: assertMatches(specified(ctx), projection) };
      },
    },
  },
});
// #endregion 🧭️Adapter
