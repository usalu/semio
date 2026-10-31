/** 🖼️ SVG 1.1 writer: serialises a paper-space {@link DrawingScene} one to one into a standalone vector document.
 *
 * `width`/`height` are in millimetres and the `viewBox` is in the same millimetres, so one user unit is one paper millimetre and stroke widths are
 * written in millimetres. Each scene layer becomes an Inkscape layer group; hatch patterns are `<pattern>` defs; legends are ordinary vector items.
 * No raster, no external fonts (generic font stack).
 *
 * @see ../../🔗️contracts/🟦️.ts — {@link DrawingScene}
 * @see https://www.w3.org/TR/SVG11/ — SVG 1.1
 */
import type { DrawingItem, DrawingScene, Fill, Legend, PathCommand } from "../../🔗️contracts/🟦️.ts";

const FONT_STACK = "Anta, 'Segoe UI', 'Helvetica Neue', Helvetica, Arial, 'DejaVu Sans', sans-serif";
const NAMESPACE = "https://semio.tech/staffage/v2";
const PATTERN_SIZE_MM = 1.2;

const num = (v: number): string => {
  const text = (Math.round(v * 10000) / 10000).toString();
  return text === "-0" ? "0" : text;
};

const esc = (text: string): string => text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");

const slug = (text: string): string => text.replace(/[^A-Za-z0-9_.-]/gu, "-");

function pathData(commands: readonly PathCommand[]): string {
  return commands.map((c) => (c.op === "Z" ? "Z" : c.op === "C" ? `C${num(c.x1)} ${num(c.y1)} ${num(c.x2)} ${num(c.y2)} ${num(c.x)} ${num(c.y)}` : `${c.op}${num(c.x)} ${num(c.y)}`)).join("");
}

function inkFor(hex: string): string {
  const v = Number.parseInt(hex.slice(1), 16);
  const lin = (c: number): number => ((c /= 255) <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);
  const l = 0.2126 * lin((v >> 16) & 255) + 0.7152 * lin((v >> 8) & 255) + 0.0722 * lin(v & 255);
  return l > 0.3 ? "#001117" : "#f7f3e3";
}

const patternId = (name: string, ink: string): string => `staffage-pattern-${name}-${ink.slice(1)}`;

function patternDef(name: NonNullable<Fill["pattern"]>, ink: string): string {
  const s = PATTERN_SIZE_MM;
  const line = (x1: number, y1: number, x2: number, y2: number): string => `<line x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" stroke="${ink}" stroke-width="0.12" stroke-linecap="round"/>`;
  const body =
    name === "compression" ? { transform: ` patternTransform="rotate(45)"`, shapes: line(0, 0, 0, s) }
    : name === "tension" ? { transform: "", shapes: `<circle cx="${s / 2}" cy="${s / 2}" r="0.14" fill="${ink}"/>` }
    : name === "hot" ? { transform: ` patternTransform="rotate(-45)"`, shapes: line(0, 0, 0, s) + line(s / 2, 0, s / 2, s) }
    : name === "cold" ? { transform: "", shapes: line(0, s / 2, s, s / 2) + line(s / 2, 0, s / 2, s) }
    : { transform: "", shapes: `<circle cx="${s / 2}" cy="${s / 2}" r="0.3" fill="none" stroke="${ink}" stroke-width="0.1"/>` };
  return `<pattern id="${patternId(name, ink)}" patternUnits="userSpaceOnUse" width="${s}" height="${s}"${body.transform}>${body.shapes}</pattern>`;
}

function itemMarkup(item: DrawingItem, patterns: Set<string>): string {
  const id = slug(item.id);
  if (item.kind === "text") {
    const weight = item.weight === "bold" ? ` font-weight="bold"` : "";
    return `<text id="${id}" x="${num(item.x)}" y="${num(item.y)}" font-family="${FONT_STACK}" font-size="${num(item.sizeMm)}"${weight} text-anchor="${item.anchor}" fill="${item.color}">${esc(item.text)}</text>`;
  }
  const { stroke, fill } = item;
  const d = pathData(item.commands);
  const strokeAttrs = stroke
    ? ` stroke="${stroke.color}" stroke-width="${num(stroke.widthMm)}"${stroke.opacity === undefined ? "" : ` stroke-opacity="${num(stroke.opacity)}"`}${stroke.dashMm ? ` stroke-dasharray="${stroke.dashMm.map(num).join(" ")}"` : ""} stroke-linecap="${stroke.cap ?? "round"}" stroke-linejoin="${stroke.join ?? "round"}"`
    : "";
  const fillAttrs = fill ? ` fill="${fill.color}" fill-rule="evenodd"${fill.opacity === undefined ? "" : ` fill-opacity="${num(fill.opacity)}"`}` : ` fill="none"`;
  const base = `<path d="${d}"${fillAttrs}${strokeAttrs}/>`;
  const title = item.value ? `<title>${esc(`${item.value.label.en} / ${item.value.label.de}: ${num(item.value.value)} ${item.value.unit}`)}</title>` : "";
  const data = item.value ? ` data-quantity="${esc(item.value.quantity)}" data-value="${item.value.value}" data-unit="${esc(item.value.unit)}"` : "";
  const refs = `${item.bone ? ` data-bone="${esc(item.bone)}"` : ""}${item.region ? ` data-region="${esc(item.region)}"` : ""}`;
  if (fill?.pattern) {
    const ink = inkFor(fill.color);
    patterns.add(patternDef(fill.pattern, ink));
    return `<g id="${id}"${data}${refs}>${title}${base}<path d="${d}" fill="url(#${patternId(fill.pattern, ink)})" fill-rule="evenodd" stroke="none"/></g>`;
  }
  if (title || refs) return `<g id="${id}"${data}${refs}>${title}${base}</g>`;
  return base.replace("<path ", `<path id="${id}" `);
}

function legendMetadata(legends: readonly Legend[]): string {
  return legends.map((l) => `<staffage:legend quantity="${esc(l.quantity)}" unit="${esc(l.unit)}" min="${l.min}" max="${l.max}" title-en="${esc(l.title.en)}" title-de="${esc(l.title.de)}" convention-en="${esc(l.convention.en)}" convention-de="${esc(l.convention.de)}" interpretation-en="${esc(l.interpretation.en)}" interpretation-de="${esc(l.interpretation.de)}"/>`).join("");
}

function representationText(scene: DrawingScene): string {
  const r = scene.metadata.representation;
  return r.kind === "scale" ? `scale:${r.denominator}${r.exact ? "" : `~${r.requested}`}` : r.kind === "symbol" ? `symbol:${r.symbol}` : "none";
}

function queryText(scene: DrawingScene): string {
  const q = scene.metadata.query;
  return q.kind === "pose" ? `pose:${q.pose}` : q.kind === "clip" ? `clip:${q.clip}@${q.timeS}` : `transition:${q.from}>${q.to}@${q.timeS}`;
}

/** 🧩️ The inner markup of a scene (`<defs>`, background rectangle, layer groups) for inline use inside an `<svg>` whose viewBox is `0 0 widthMm heightMm`. */
export function svgMarkup(scene: DrawingScene): string {
  const patterns = new Set<string>();
  const layers = scene.layers
    .map((layer) => `<g id="staffage-${layer.id}" inkscape:groupmode="layer" inkscape:label="${esc(layer.title.en)}">${layer.items.map((item) => itemMarkup(item, patterns)).join("")}</g>`)
    .join("");
  const defs = patterns.size ? `<defs>${[...patterns].join("")}</defs>` : "";
  const background = scene.background === null ? "" : `<rect id="staffage-background" x="0" y="0" width="${num(scene.widthMm)}" height="${num(scene.heightMm)}" fill="${scene.background}"/>`;
  return `${defs}${background}${layers}`;
}

/** 🖼️ A standalone SVG 1.1 document: size and viewBox in millimetres, `<title>`, `<desc>`, `<metadata>` (asset, query, time, view, scale, resolution, representation, print profile, units, legends), layer groups. */
export function writeSvg(scene: DrawingScene): string {
  const m = scene.metadata;
  const title = `${m.title.en} / ${m.title.de} — ${m.view} 1:${m.scale.denominator}`;
  const desc = `Architectural staffage "${m.asset}", ${queryText(scene)}, t = ${m.timeS} s, view ${m.view} (${m.viewpoint}), scale 1:${m.scale.denominator}. Representation ${representationText(scene)} (${m.resolution}), print profile ${m.profile}. Units: paper millimetres (1 user unit = 1 mm), model metres, ${num(m.mmPerMetre)} mm per metre.`;
  const meta = `<staffage:scene asset="${esc(m.asset)}" query="${esc(queryText(scene))}" time-s="${m.timeS}" view="${m.view}" viewpoint="${m.viewpoint}" scale="1:${m.scale.denominator}" resolution="${m.resolution}" representation="${esc(representationText(scene))}" profile="${esc(m.profile)}" paper-units="mm" model-units="m" mm-per-metre="${m.mmPerMetre}" origin-x-mm="${m.origin.xMm}" origin-y-mm="${m.origin.yMm}">${legendMetadata(scene.legends)}</staffage:scene>`;
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns:staffage="${NAMESPACE}" version="1.1" width="${num(scene.widthMm)}mm" height="${num(scene.heightMm)}mm" viewBox="0 0 ${num(scene.widthMm)} ${num(scene.heightMm)}">\n<title>${esc(title)}</title>\n<desc>${esc(desc)}</desc>\n<metadata>${meta}</metadata>\n${svgMarkup(scene)}\n</svg>\n`;
}
