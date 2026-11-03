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
import { VIEW_TITLES } from "../../🎨️depiction/🟦️.ts";
import { viewpointCaption } from "../../🎨️depiction/✂️section/🟦️.ts";
import { FIDELITY_NAMES, PROJECTION_NAMES, isSheet } from "../📑️sheet/🟦️.ts";

const FONT_STACK = "Anta, 'Segoe UI', 'Helvetica Neue', Helvetica, Arial, 'DejaVu Sans', sans-serif";
const NAMESPACE = "https://semio.tech/staffage/v2";
const PATTERN_SIZE_MM = 1.2;

const num = (v: number): string => {
  const text = (Math.round(v * 10000) / 10000).toString();
  return text === "-0" ? "0" : text;
};

/** 🔢 A measured value to 6 significant digits, so floating-point noise such as 22.000000000000014 does not bloat every shape of a frame. */
const significant = (v: number): string => String(Number(v.toPrecision(6)));

const esc = (text: string): string => text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");

const slug = (text: string): string => text.replace(/[^A-Za-z0-9_.-]/gu, "-");

/** 🗜️ Options of the writer: `compact` hoists the presentation attributes every stroke and fill shares (round caps and joins, even-odd fill) onto the layer group, drops the ids nobody can address (paths without a group) and writes path data in the shortest exact absolute spelling; the picture is identical. */
export type SvgOptions = { readonly compact?: boolean };

const HOISTED = ` fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round"`;

function pathData(commands: readonly PathCommand[]): string {
  return commands.map((c) => (c.op === "Z" ? "Z" : c.op === "C" ? `C${num(c.x1)} ${num(c.y1)} ${num(c.x2)} ${num(c.y2)} ${num(c.x)} ${num(c.y)}` : `${c.op}${num(c.x)} ${num(c.y)}`)).join("");
}

const short = (v: number): string => num(v).replace(/^(-?)0\./u, "$1.");

/** 📏 Absolute path data in the shortest exact spelling: a run of one `L` or `C` command names it once, leading zeros and the space before a minus sign are left out. Same values as {@link pathData}, so every renderer draws the same pixels. */
function compactData(commands: readonly PathCommand[]): string {
  let text = "";
  let previous = "";
  for (const c of commands) {
    if (c.op === "Z") {
      text += "Z";
      previous = "Z";
      continue;
    }
    text += c.op === previous && c.op !== "M" ? "" : c.op;
    previous = c.op;
    const values = c.op === "C" ? [c.x1, c.y1, c.x2, c.y2, c.x, c.y] : [c.x, c.y];
    for (const value of values.map(short)) text += (/[\d.]$/u.test(text) && !value.startsWith("-") ? " " : "") + value;
  }
  return text;
}

/** 🖋️ The ink of last resort for a hatch whose item has no stroke: light or dark by the luminance of the fill. A hatch with an outline uses the outline colour, which is the ink of the scene theme, so a light-theme drawing never carries a dark-theme ink. */
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

function itemMarkup(item: DrawingItem, patterns: Set<string>, compact: boolean): string {
  const id = slug(item.id);
  if (item.kind === "text") {
    const weight = item.weight === "bold" ? ` font-weight="bold"` : "";
    return `<text id="${id}" x="${num(item.x)}" y="${num(item.y)}" font-family="${FONT_STACK}" font-size="${num(item.sizeMm)}"${weight} text-anchor="${item.anchor}" fill="${item.color}">${esc(item.text)}</text>`;
  }
  const { stroke, fill } = item;
  const d = compact ? compactData(item.commands) : pathData(item.commands);
  const cap = compact && (stroke?.cap ?? "round") === "round" ? "" : ` stroke-linecap="${stroke?.cap ?? "round"}"`;
  const join = compact && (stroke?.join ?? "round") === "round" ? "" : ` stroke-linejoin="${stroke?.join ?? "round"}"`;
  const strokeAttrs = stroke
    ? ` stroke="${stroke.color}" stroke-width="${num(stroke.widthMm)}"${stroke.opacity === undefined ? "" : ` stroke-opacity="${num(stroke.opacity)}"`}${stroke.dashMm ? ` stroke-dasharray="${stroke.dashMm.map(num).join(" ")}"` : ""}${cap}${join}`
    : "";
  const rule = compact ? "" : ` fill-rule="evenodd"`;
  const fillAttrs = fill ? ` fill="${fill.color}"${rule}${fill.opacity === undefined ? "" : ` fill-opacity="${num(fill.opacity)}"`}` : ` fill="none"`;
  const base = `<path d="${d}"${fillAttrs}${strokeAttrs}/>`;
  const title = item.value ? `<title>${esc(`${item.value.label.en} / ${item.value.label.de}: ${num(item.value.value)} ${item.value.unit}`)}</title>` : "";
  const data = item.value ? ` data-quantity="${esc(item.value.quantity)}" data-value="${significant(item.value.value)}" data-unit="${esc(item.value.unit)}"` : "";
  const refs = `${item.bone ? ` data-bone="${esc(item.bone)}"` : ""}${item.region ? ` data-region="${esc(item.region)}"` : ""}`;
  if (fill?.pattern) {
    const ink = stroke?.color ?? inkFor(fill.color);
    patterns.add(patternDef(fill.pattern, ink));
    return `<g id="${id}"${data}${refs}>${title}${base}<path d="${d}" fill="url(#${patternId(fill.pattern, ink)})"${rule} stroke="none"/></g>`;
  }
  if (title || refs) return `<g id="${id}"${data}${refs}>${title}${base}</g>`;
  return compact ? base : base.replace("<path ", `<path id="${id}" `);
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
export function svgMarkup(scene: DrawingScene, options: SvgOptions = {}): string {
  const compact = options.compact === true;
  const patterns = new Set<string>();
  const layers = scene.layers
    .map((layer) => `<g id="staffage-${layer.id}" inkscape:groupmode="layer" inkscape:label="${esc(layer.title.en)}"${compact ? HOISTED : ""}>${layer.items.map((item) => itemMarkup(item, patterns, compact)).join("")}</g>`)
    .join("");
  const defs = patterns.size ? `<defs>${[...patterns].join("")}</defs>` : "";
  const background = scene.background === null ? "" : `<rect id="staffage-background" x="0" y="0" width="${num(scene.widthMm)}" height="${num(scene.heightMm)}" fill="${scene.background}"/>`;
  return `${defs}${background}${layers}`;
}

/** ✍️ A standalone SVG 1.1 document: size and viewBox in millimetres, `<title>`, `<desc>`, `<metadata>` (asset, query, time, representation, viewpoint, scale, resolution, print profile, units, legends), layer groups. Prose names the representation and the viewpoint in both languages; the machine ids stay in the metadata attributes. A multiview sheet states its projection and the origin of each view. */
export function writeSvg(scene: DrawingScene, options: SvgOptions = {}): string {
  const m = scene.metadata;
  const sheet = isSheet(scene) ? scene.sheet : undefined;
  const mode = (language: "en" | "de"): string => VIEW_TITLES[m.view][language];
  const where = (language: "en" | "de"): string => (sheet ? { en: "plan, front elevation and side elevation", de: "Draufsicht, Vorderansicht und Seitenansicht" }[language] : viewpointCaption(m.viewpoint, undefined, language));
  const title = `${m.title.en} / ${m.title.de} — ${mode("en")} / ${mode("de")}, ${where("en")} / ${where("de")} 1:${m.scale.denominator}`;
  const projection = sheet ? ` Projection: ${PROJECTION_NAMES[sheet.projection].en} / ${PROJECTION_NAMES[sheet.projection].de}.` : "";
  const fidelity = ` Fidelity: ${FIDELITY_NAMES[m.fidelity].en} / ${FIDELITY_NAMES[m.fidelity].de}.`;
  const desc = `Architectural staffage "${m.asset}", ${queryText(scene)}, t = ${m.timeS} s, representation ${mode("en")} / ${mode("de")}, viewpoint ${where("en")} / ${where("de")}, scale 1:${m.scale.denominator}.${projection}${fidelity} Representation ${representationText(scene)} (${m.resolution}), print profile ${m.profile}. Units: paper millimetres (1 user unit = 1 mm), model metres, ${num(m.mmPerMetre)} mm per metre.`;
  const labels = `view-title-en="${esc(mode("en"))}" view-title-de="${esc(mode("de"))}" viewpoint-title-en="${esc(where("en"))}" viewpoint-title-de="${esc(where("de"))}" fidelity="${m.fidelity}" fidelity-title-en="${esc(FIDELITY_NAMES[m.fidelity].en)}" fidelity-title-de="${esc(FIDELITY_NAMES[m.fidelity].de)}"`;
  const sheetAttrs = sheet ? ` sheet="true" projection="${sheet.projection}" projection-title-en="${esc(PROJECTION_NAMES[sheet.projection].en)}" projection-title-de="${esc(PROJECTION_NAMES[sheet.projection].de)}"` : "";
  const origins = sheet ? sheet.viewpoints.map((viewpoint) => `<staffage:view viewpoint="${viewpoint}" fidelity="${sheet.fidelity[viewpoint]}" origin-x-mm="${num(sheet.originsMm[viewpoint].xMm)}" origin-y-mm="${num(sheet.originsMm[viewpoint].yMm)}"/>`).join("") : "";
  const extents = sheet ? `<staffage:extents-m elevation-width="${significant(sheet.extentsM.elevation.widthM)}" elevation-height="${significant(sheet.extentsM.elevation.heightM)}" front-depth="${significant(sheet.extentsM.front.depthM)}" front-height="${significant(sheet.extentsM.front.heightM)}" plan-width="${significant(sheet.extentsM.plan.widthM)}" plan-depth="${significant(sheet.extentsM.plan.depthM)}"/>` : "";
  const meta = `<staffage:scene asset="${esc(m.asset)}" query="${esc(queryText(scene))}" time-s="${m.timeS}" view="${m.view}" viewpoint="${sheet ? "elevation+front+plan" : m.viewpoint}" ${labels}${sheetAttrs} scale="1:${m.scale.denominator}" resolution="${m.resolution}" representation="${esc(representationText(scene))}" profile="${esc(m.profile)}" paper-units="mm" model-units="m" mm-per-metre="${m.mmPerMetre}" origin-x-mm="${m.origin.xMm}" origin-y-mm="${m.origin.yMm}">${origins}${extents}${legendMetadata(scene.legends)}</staffage:scene>`;
  return `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns:staffage="${NAMESPACE}" version="1.1" width="${num(scene.widthMm)}mm" height="${num(scene.heightMm)}mm" viewBox="0 0 ${num(scene.widthMm)} ${num(scene.heightMm)}">
<title>${esc(title)}</title>
<desc>${esc(desc)}</desc>
<metadata>${meta}</metadata>
${svgMarkup(scene, options)}
</svg>
`;
}
