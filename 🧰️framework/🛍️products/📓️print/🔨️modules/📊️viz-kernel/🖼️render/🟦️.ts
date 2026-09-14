/** 🖼️ Rendering: a §79 chart specification into two independent outputs — the dependency-free 2D
 * scene graph of `🧰️framework/🔨️modules/◻️2d` and TikZ source text for `semio-viz`. Both emitters
 * read the same resolved item list, so a scene and a TikZ picture can never disagree.
 * @see ../../../../../🔨️modules/◻️2d/🟦️.ts
 * @see ../../../🖋️latex/semio-viz-plot.sty
 */
import type { DrawingScene, SceneNode } from "../../../../../🔨️modules/◻️2d/🟦️.ts";
import { drawVizSymbol, vizPathRecorder, type VizPathCommand } from "../✒️mark/🟦️.ts";
import { vizArc, vizLine } from "../🥧shape/🟦️.ts";
import { buildVizCoordinate } from "../🧭coordinate/🟦️.ts";
import { buildVizScale, type VizScale } from "../📐scale/🟦️.ts";
import { vizCategoricalColor, vizParseColor, vizTheme, type VizTheme } from "../🎨theme/🟦️.ts";
import type { VizChartSpecification, VizExtent, VizLayerSpec, VizPoint, VizRow, VizSymbolKind, VizTable } from "../🧬️schema/🟦️.ts";

//#region 🔖️Items
/** 🖼️ One resolved drawing primitive, independent of the output format. */
export type VizRenderItem =
  | { readonly kind: "rect"; readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "circle"; readonly cx: number; readonly cy: number; readonly r: number; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "line"; readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number; readonly stroke?: string; readonly strokeWidth?: number; readonly dash?: readonly number[] }
  | { readonly kind: "polygon"; readonly points: readonly VizPoint[]; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "path"; readonly commands: readonly VizPathCommand[]; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "text"; readonly x: number; readonly y: number; readonly content: string; readonly size: number; readonly fill?: string; readonly anchor?: "start" | "middle" | "end" };

/** 🖼️ A chart resolved into primitives: the frame it was laid out in and the items to draw. */
export type VizRenderPlan = { readonly width: number; readonly height: number; readonly frame: VizExtent; readonly theme: VizTheme; readonly items: readonly VizRenderItem[] };
//#endregion 🔖️Items

//#region 🔖️Layout
function tableOf(spec: VizChartSpecification, name?: string): VizTable {
  const tables = spec.tables ?? [];
  const table = name === undefined ? tables[0] : tables.find((entry) => entry.name === name);
  if (table === undefined) throw new Error(`the specification declares no data table ${name === undefined ? "" : name}`.trim());
  return table;
}

function scalesOf(spec: VizChartSpecification): Map<string, VizScale<never, never>> {
  const out = new Map<string, VizScale<never, never>>();
  for (const scale of spec.scales ?? []) out.set(scale.name, buildVizScale(scale));
  return out;
}

function channelValue(scales: Map<string, VizScale<never, never>>, layer: VizLayerSpec, channel: string, row: VizRow, fallback: number): number {
  const encoding = layer.encodings?.[channel as never] as { column?: string; scale?: string; value?: string | number | boolean } | undefined;
  if (encoding === undefined) return fallback;
  if (encoding.value !== undefined) return Number(encoding.value);
  const raw = encoding.column === undefined ? fallback : row[encoding.column];
  if (encoding.scale === undefined) return Number(raw);
  const scale = scales.get(encoding.scale);
  if (scale === undefined) throw new Error(`layer references unknown scale ${encoding.scale}`);
  return Number((scale as unknown as (value: unknown) => number)(raw));
}

function channelText(layer: VizLayerSpec, channel: string, row: VizRow): string {
  const encoding = layer.encodings?.[channel as never] as { column?: string; value?: string | number | boolean } | undefined;
  if (encoding === undefined) return "";
  if (encoding.value !== undefined) return String(encoding.value);
  return encoding.column === undefined ? "" : String(row[encoding.column] ?? "");
}

/** 🖼️ Resolves a specification into primitives: scales, coordinate system, marks and guides. */
export function planVizChart(spec: VizChartSpecification): VizRenderPlan {
  const margin = spec.margin ?? { top: 8, right: 8, bottom: 14, left: 16 };
  const frame: VizExtent = { x0: margin.left, y0: margin.top, x1: spec.width - margin.right, y1: spec.height - margin.bottom };
  const theme = vizTheme(spec.theme?.appearance ?? "light", spec.theme?.name ?? "semio");
  const palette = spec.theme?.palette ?? theme.palette.categorical;
  const color = (index: number): string => palette[((index % palette.length) + palette.length) % palette.length] ?? vizCategoricalColor(theme.palette, index);
  const scales = scalesOf(spec);
  const coordinate = buildVizCoordinate(spec.coordinate?.kind ?? "cartesian", frame, spec.coordinate?.options ?? {});
  const items: VizRenderItem[] = [];
  for (const guide of spec.guides ?? []) {
    const scale = guide.scale === undefined ? undefined : scales.get(guide.scale);
    if (scale === undefined) continue;
    const values = guide.tickValues ?? scale.ticks?.(guide.ticks ?? 5) ?? [];
    const horizontal = guide.orient === "bottom" || guide.orient === "top";
    const tickSize = guide.tickSize ?? 2;
    for (const value of values) {
      const at = Number((scale as unknown as (value: unknown) => number)(value)) + (scale.bandwidth === undefined ? 0 : scale.bandwidth() / 2);
      if (horizontal) {
        items.push({ kind: "line", x1: at, y1: frame.y1, x2: at, y2: frame.y1 + tickSize, stroke: "#808080", strokeWidth: theme.strokes.gridMinor ?? 0.1 });
        if (guide.grid === true) items.push({ kind: "line", x1: at, y1: frame.y0, x2: at, y2: frame.y1, stroke: "#c0c0c0", strokeWidth: theme.strokes.gridMinor ?? 0.1 });
        items.push({ kind: "text", x: at, y: frame.y1 + tickSize + 3, content: String(value), size: theme.typography.textXsPx ?? 8, anchor: "middle" });
      } else {
        items.push({ kind: "line", x1: frame.x0 - tickSize, y1: at, x2: frame.x0, y2: at, stroke: "#808080", strokeWidth: theme.strokes.gridMinor ?? 0.1 });
        if (guide.grid === true) items.push({ kind: "line", x1: frame.x0, y1: at, x2: frame.x1, y2: at, stroke: "#c0c0c0", strokeWidth: theme.strokes.gridMinor ?? 0.1 });
        items.push({ kind: "text", x: frame.x0 - tickSize - 1, y: at, content: String(value), size: theme.typography.textXsPx ?? 8, anchor: "end" });
      }
    }
  }
  spec.layers.forEach((layer, layerIndex) => {
    const table = tableOf(spec, layer.data);
    const fill = color(layerIndex);
    switch (layer.mark) {
      case "bar": {
        const width = Number(layer.options?.width ?? 6);
        table.rows.forEach((row) => {
          const x = channelValue(scales, layer, "x", row, 0);
          const y = channelValue(scales, layer, "y", row, frame.y1);
          const base = channelValue(scales, layer, "y2", row, frame.y1);
          const bandwidth = (layer.encodings?.x?.scale === undefined ? undefined : scales.get(layer.encodings.x.scale)?.bandwidth?.()) ?? width;
          items.push({ kind: "rect", x, y: Math.min(y, base), width: bandwidth, height: Math.abs(base - y), fill });
        });
        return;
      }
      case "point": {
        const symbol = String(layer.options?.symbol ?? "circle") as VizSymbolKind;
        const size = Number(layer.options?.size ?? 16);
        table.rows.forEach((row) => {
          const x = channelValue(scales, layer, "x", row, 0);
          const y = channelValue(scales, layer, "y", row, 0);
          if (symbol === "circle") {
            items.push({ kind: "circle", cx: x, cy: y, r: Math.sqrt(size / Math.PI), fill });
            return;
          }
          const recorder = vizPathRecorder();
          drawVizSymbol(symbol, { ...recorder, moveTo: (px, py) => recorder.moveTo(px + x, py + y), lineTo: (px, py) => recorder.lineTo(px + x, py + y), rect: (px, py, w, h) => recorder.rect(px + x, py + y, w, h), arc: (px, py, r, a0, a1, ccw) => recorder.arc(px + x, py + y, r, a0, a1, ccw) }, size);
          items.push({ kind: "path", commands: recorder.commands, fill });
        });
        return;
      }
      case "line":
      case "area": {
        const points = table.rows.map((row) => [channelValue(scales, layer, "x", row, 0), channelValue(scales, layer, "y", row, 0)] as VizPoint);
        const commands = vizLine(points);
        items.push(layer.mark === "line" ? { kind: "path", commands, stroke: fill, strokeWidth: theme.strokes.gridMajor ?? 0.3 } : { kind: "polygon", points: [...points, [points[points.length - 1]![0], frame.y1], [points[0]![0], frame.y1]], fill, opacity: 0.6 });
        return;
      }
      case "arc": {
        const innerRadius = Number(layer.options?.innerRadius ?? 0);
        const outerRadius = Number(layer.options?.outerRadius ?? Math.min(frame.x1 - frame.x0, frame.y1 - frame.y0) / 2);
        const centre = coordinate.project(0.5, 0);
        table.rows.forEach((row, index) => {
          const startAngle = channelValue(scales, layer, "angle", row, 0);
          const endAngle = channelValue(scales, layer, "x2", row, startAngle);
          const commands = vizArc({ innerRadius, outerRadius, startAngle, endAngle, padAngle: Number(layer.options?.padAngle ?? 0), cornerRadius: Number(layer.options?.cornerRadius ?? 0) }).map((command) => ({ ...command, args: command.args.map((value, i) => (command.op === "arc" && i > 1 ? value : i % 2 === 0 ? value + centre[0] : value + centre[1])) }) as VizPathCommand);
          items.push({ kind: "path", commands, fill: color(index) });
        });
        return;
      }
      case "text": {
        table.rows.forEach((row) => {
          items.push({ kind: "text", x: channelValue(scales, layer, "x", row, 0), y: channelValue(scales, layer, "y", row, 0), content: channelText(layer, "text", row), size: Number(layer.options?.size ?? theme.typography.textSmPx ?? 10), anchor: "middle" });
        });
        return;
      }
      case "rule": {
        table.rows.forEach((row) => {
          const y = channelValue(scales, layer, "y", row, frame.y1);
          items.push({ kind: "line", x1: frame.x0, y1: y, x2: frame.x1, y2: y, stroke: fill, strokeWidth: theme.strokes.gridMajor ?? 0.3 });
        });
        return;
      }
      default:
        throw new Error(`unknown mark ${layer.mark}`);
    }
  });
  return { width: spec.width, height: spec.height, frame, theme, items };
}
//#endregion 🔖️Layout

//#region 🔖️SceneGraph
const IDENTITY: readonly [number, number, number, number, number, number] = [1, 0, 0, 1, 0, 0];

function sceneColor(hex: string, opacity = 1): readonly [number, number, number, number] {
  const [r, g, b, a] = vizParseColor(hex);
  return [r / 255, g / 255, b / 255, (a / 255) * opacity];
}

function sceneNode(item: VizRenderItem): SceneNode {
  const fill = "fill" in item && item.fill !== undefined ? { kind: "solid" as const, color: sceneColor(item.fill, "opacity" in item ? (item.opacity ?? 1) : 1) } : undefined;
  const stroke = "stroke" in item && item.stroke !== undefined ? { color: sceneColor(item.stroke), width: ("strokeWidth" in item ? item.strokeWidth : undefined) ?? 0.2, cap: "butt" as const, join: "miter" as const, ...("dash" in item && item.dash !== undefined && item.dash.length > 0 ? { dash: [...item.dash] } : {}) } : undefined;
  switch (item.kind) {
    case "rect":
      return { transform: IDENTITY, node: { kind: "rect", x: item.x, y: item.y, width: item.width, height: item.height }, fill, stroke };
    case "circle":
      return { transform: IDENTITY, node: { kind: "circle", cx: item.cx, cy: item.cy, r: item.r }, fill, stroke };
    case "line":
      return { transform: IDENTITY, node: { kind: "line", x1: item.x1, y1: item.y1, x2: item.x2, y2: item.y2 }, stroke };
    case "polygon":
      return { transform: IDENTITY, node: { kind: "polygon", points: item.points.map((point) => [point[0], point[1]] as const) }, fill, stroke };
    case "text":
      return { transform: IDENTITY, node: { kind: "text", x: item.x, y: item.y, content: item.content, size: item.size }, fill: fill ?? { kind: "solid", color: [0, 0, 0, 1] } };
    default:
      return { transform: IDENTITY, node: { kind: "path", segments: pathSegments(item.commands) }, fill, stroke };
  }
}

function pathSegments(commands: readonly VizPathCommand[]): SceneNode["node"] extends { kind: "path"; segments: infer S } ? S : never {
  const segments: unknown[] = [];
  let cursor: VizPoint = [0, 0];
  for (const command of commands) {
    switch (command.op) {
      case "moveTo":
        cursor = [command.args[0], command.args[1]];
        segments.push({ kind: "move", to: [cursor[0], cursor[1]] });
        break;
      case "lineTo":
        cursor = [command.args[0], command.args[1]];
        segments.push({ kind: "line", to: [cursor[0], cursor[1]] });
        break;
      case "quadraticCurveTo":
        cursor = [command.args[2], command.args[3]];
        segments.push({ kind: "quad", ctrl: [command.args[0], command.args[1]], to: [cursor[0], cursor[1]] });
        break;
      case "bezierCurveTo":
        cursor = [command.args[4], command.args[5]];
        segments.push({ kind: "cubic", ctrl1: [command.args[0], command.args[1]], ctrl2: [command.args[2], command.args[3]], to: [cursor[0], cursor[1]] });
        break;
      case "arc": {
        const [cx, cy, r, a0, a1, ccw] = command.args;
        const to: VizPoint = [cx + r * Math.cos(a1), cy + r * Math.sin(a1)];
        const sweep = ccw === 0;
        let delta = a1 - a0;
        if (sweep) while (delta < 0) delta += 2 * Math.PI;
        else while (delta > 0) delta -= 2 * Math.PI;
        segments.push({ kind: "arc", rx: r, ry: r, rotation: 0, largeArc: Math.abs(delta) > Math.PI, sweep, to: [to[0], to[1]] });
        cursor = to;
        break;
      }
      case "rect":
        segments.push({ kind: "move", to: [command.args[0], command.args[1]] });
        segments.push({ kind: "line", to: [command.args[0] + command.args[2], command.args[1]] });
        segments.push({ kind: "line", to: [command.args[0] + command.args[2], command.args[1] + command.args[3]] });
        segments.push({ kind: "line", to: [command.args[0], command.args[1] + command.args[3]] });
        segments.push({ kind: "close" });
        break;
      default:
        segments.push({ kind: "close" });
    }
  }
  return segments as never;
}

/** 🖼️ Renders a chart specification into the dependency-free 2D scene graph. */
export function renderVizScene(spec: VizChartSpecification): DrawingScene {
  const plan = planVizChart(spec);
  return { width: plan.width, height: plan.height, nodes: plan.items.map(sceneNode) };
}
//#endregion 🔖️SceneGraph

//#region 🔖️Tikz
function tikzNumber(value: number): string {
  return (Math.round(value * 1e4) / 1e4).toString();
}

function tikzColor(hex: string): string {
  const [r, g, b] = vizParseColor(hex);
  return `{rgb,255:red,${r};green,${g};blue,${b}}`;
}

function tikzPath(commands: readonly VizPathCommand[]): string {
  const parts: string[] = [];
  for (const command of commands) {
    switch (command.op) {
      case "moveTo":
        parts.push(`(${tikzNumber(command.args[0])},${tikzNumber(command.args[1])})`);
        break;
      case "lineTo":
        parts.push(`-- (${tikzNumber(command.args[0])},${tikzNumber(command.args[1])})`);
        break;
      case "quadraticCurveTo":
        parts.push(`.. controls (${tikzNumber(command.args[0])},${tikzNumber(command.args[1])}) .. (${tikzNumber(command.args[2])},${tikzNumber(command.args[3])})`);
        break;
      case "bezierCurveTo":
        parts.push(`.. controls (${tikzNumber(command.args[0])},${tikzNumber(command.args[1])}) and (${tikzNumber(command.args[2])},${tikzNumber(command.args[3])}) .. (${tikzNumber(command.args[4])},${tikzNumber(command.args[5])})`);
        break;
      case "arc": {
        const [cx, cy, r, a0, a1] = command.args;
        parts.push(`-- (${tikzNumber(cx + r * Math.cos(a0))},${tikzNumber(cy + r * Math.sin(a0))}) arc (${tikzNumber((a0 * 180) / Math.PI)}:${tikzNumber((a1 * 180) / Math.PI)}:${tikzNumber(r)})`);
        break;
      }
      case "rect":
        parts.push(`(${tikzNumber(command.args[0])},${tikzNumber(command.args[1])}) rectangle (${tikzNumber(command.args[0] + command.args[2])},${tikzNumber(command.args[1] + command.args[3])})`);
        break;
      default:
        parts.push("-- cycle");
    }
  }
  return parts.join(" ");
}

function tikzItem(item: VizRenderItem): string {
  const paint: string[] = [];
  if ("fill" in item && item.fill !== undefined) paint.push(`fill=${tikzColor(item.fill)}`);
  if ("stroke" in item && item.stroke !== undefined) {
    paint.push(`draw=${tikzColor(item.stroke)}`);
    if ("strokeWidth" in item && item.strokeWidth !== undefined) paint.push(`line width=${tikzNumber(item.strokeWidth)}mm`);
  }
  if ("opacity" in item && item.opacity !== undefined && item.opacity < 1) paint.push(`opacity=${tikzNumber(item.opacity)}`);
  const options = paint.length > 0 ? `[${paint.join(",")}]` : "";
  switch (item.kind) {
    case "rect":
      return `\\path${options} (${tikzNumber(item.x)},${tikzNumber(item.y)}) rectangle ++(${tikzNumber(item.width)},${tikzNumber(item.height)});`;
    case "circle":
      return `\\path${options} (${tikzNumber(item.cx)},${tikzNumber(item.cy)}) circle[radius=${tikzNumber(item.r)}mm];`;
    case "line":
      return `\\path${options} (${tikzNumber(item.x1)},${tikzNumber(item.y1)}) -- (${tikzNumber(item.x2)},${tikzNumber(item.y2)});`;
    case "polygon":
      return `\\path${options} ${item.points.map((point) => `(${tikzNumber(point[0])},${tikzNumber(point[1])})`).join(" -- ")} -- cycle;`;
    case "text":
      return `\\node[anchor=${item.anchor === "start" ? "west" : item.anchor === "end" ? "east" : "center"},font=\\fontsize{${tikzNumber(item.size)}}{${tikzNumber(item.size * 1.2)}}\\selectfont] at (${tikzNumber(item.x)},${tikzNumber(item.y)}) {${item.content}};`;
    default:
      return `\\path${options} ${tikzPath(item.commands)};`;
  }
}

/** 🖼️ Renders a chart specification into TikZ source text, in figure millimetres. */
export function renderVizTikz(spec: VizChartSpecification): string {
  const plan = planVizChart(spec);
  const lines = [`\\begin{tikzpicture}[x=1mm,y=-1mm]`, `% ${plan.width}mm × ${plan.height}mm, ${plan.theme.appearance} appearance`, ...plan.items.map(tikzItem), `\\end{tikzpicture}`];
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Tikz
