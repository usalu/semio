/** 📕️ Vector PDF 1.7 writer: one page per {@link DrawingScene}, MediaBox = scene size in points (1 mm = 72/25.4 pt), paths as
 * m/l/c/h with fill/stroke/dash/cap/join, opacity through ExtGState, hatch fills as uncoloured tiling patterns, text in
 * Helvetica (standard 14, WinAnsiEncoding), one optional-content group per drawing layer. Uncompressed and deterministic
 * (no creation date, ID derived from the content). Browser-compatible, dependency-free.
 * <https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf> · <https://pdfa.org/resource/iso-32000-2/> */
import type { DrawingItem, DrawingScene, JobControl, PathCommand } from "../../🔗️contracts/🟦️.ts";

//#region 🔖️Units
/** 📏️ Points per millimetre. */
export const PT_PER_MM = 72 / 25.4;

const num = (value: number): string => {
  const text = value.toFixed(4);
  return text.includes(".") ? text.replace(/\.?0+$/u, "") || "0" : text;
};
//#endregion 🔖️Units

//#region 🔖️WinAnsi
const CP1252_HIGH: Readonly<Record<number, number>> = { 0x20ac: 0x80, 0x201a: 0x82, 0x0192: 0x83, 0x201e: 0x84, 0x2026: 0x85, 0x2020: 0x86, 0x2021: 0x87, 0x02c6: 0x88, 0x2030: 0x89, 0x0160: 0x8a, 0x2039: 0x8b, 0x0152: 0x8c, 0x017d: 0x8e, 0x2018: 0x91, 0x2019: 0x92, 0x201c: 0x93, 0x201d: 0x94, 0x2022: 0x95, 0x2013: 0x96, 0x2014: 0x97, 0x02dc: 0x98, 0x2122: 0x99, 0x0161: 0x9a, 0x203a: 0x9b, 0x0153: 0x9c, 0x017e: 0x9e, 0x0178: 0x9f };

const TRANSLITERATION: Readonly<Record<string, string>> = { "−": "-", "‐": "-", "‑": "-", "‒": "-", " ": " ", " ": " ", " ": " ", " ": " ", "≤": "<=", "≥": ">=", "≈": "~", "≠": "!=", "→": "->", "←": "<-", "↑": "^", "↓": "v", "∆": "Delta ", "Δ": "Delta ", "ρ": "rho", "λ": "lambda", "α": "alpha", "β": "beta", "σ": "sigma", "τ": "tau", "φ": "phi", "θ": "theta", "π": "pi", "Ω": "Ohm", "Ω": "Ohm", "³": "³", "⁴": "^4", "⁰": "^0", "ⁱ": "^i", "⁵": "^5", "⁶": "^6", "⁷": "^7", "⁸": "^8", "⁹": "^9", "⁻": "-", "₀": "_0", "₁": "_1", "₂": "_2", "₃": "_3", "₄": "_4", "•": "•", "∶": ":", "⋅": "·", "∙": "·", "­": "-" };

/** 🔡️ The WinAnsi byte of one code point, or `null` when it has none. */
function winAnsiByte(codePoint: number): number | null {
  if ((codePoint >= 0x20 && codePoint <= 0x7e) || (codePoint >= 0xa0 && codePoint <= 0xff)) return codePoint;
  return CP1252_HIGH[codePoint] ?? null;
}

/** 🔡️ WinAnsi bytes of `text`: direct where WinAnsi has the glyph (`·` 0xB7, `²` 0xB2, `°` 0xB0 …), else transliterated (accent stripped, `−` → `-`, `≤` → `<=`), else `?`. */
export function winAnsiBytes(text: string): number[] {
  const out: number[] = [];
  for (const character of text) {
    const direct = winAnsiByte(character.codePointAt(0)!);
    if (direct !== null) out.push(direct);
    else if (character === "\n" || character === "\t") out.push(0x20);
    else {
      const replacement = TRANSLITERATION[character] ?? [...character.normalize("NFD")].filter((part) => winAnsiByte(part.codePointAt(0)!) !== null && !/\p{M}/u.test(part)).join("");
      for (const part of replacement || "?") out.push(winAnsiByte(part.codePointAt(0)!) ?? 0x3f);
    }
  }
  return out;
}

const HELVETICA = [278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556, 1015, 667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 278, 278, 278, 469, 556, 333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, 556, 556, 556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584];
const HELVETICA_BOLD = [278, 333, 474, 556, 556, 889, 722, 238, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 333, 333, 584, 584, 584, 611, 975, 722, 722, 722, 722, 667, 611, 778, 722, 278, 556, 722, 611, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 333, 278, 333, 584, 556, 333, 556, 611, 556, 611, 556, 333, 611, 611, 278, 278, 556, 278, 889, 611, 611, 611, 611, 389, 556, 333, 611, 556, 778, 556, 556, 500, 389, 280, 389, 584];
const WIDE: Readonly<Record<number, readonly [number, number]>> = { 0xb0: [400, 400], 0xb7: [278, 278], 0xb2: [333, 333], 0xb3: [333, 333], 0xb5: [556, 611], 0xd7: [584, 584], 0xb1: [584, 584], 0x96: [556, 556], 0x97: [1000, 1000], 0x80: [556, 556], 0xa0: [278, 278], 0x95: [350, 350], 0x85: [1000, 1000], 0xa9: [737, 737], 0xb9: [333, 333], 0xbc: [834, 834], 0xbd: [834, 834], 0xbe: [834, 834] };

/** 📏️ Advance width of `text` in Helvetica, in 1/1000 em, over the WinAnsi bytes written for it (accented letters take their base letter's width). */
export function helveticaWidth(text: string, bold: boolean): number {
  let total = 0;
  for (const byte of winAnsiBytes(text)) {
    const table = bold ? HELVETICA_BOLD : HELVETICA;
    const wide = WIDE[byte];
    if (byte >= 0x20 && byte <= 0x7e) total += table[byte - 0x20]!;
    else if (wide) total += wide[bold ? 1 : 0];
    else {
      const base = String.fromCharCode(byte).normalize("NFD")[0]!.charCodeAt(0);
      total += base >= 0x20 && base <= 0x7e ? table[base - 0x20]! : 556;
    }
  }
  return total;
}

const pdfString = (bytes: readonly number[]): string => "(" + bytes.map((byte) => (byte === 0x28 || byte === 0x29 || byte === 0x5c ? "\\" + String.fromCharCode(byte) : byte < 0x20 || byte > 0x7e ? "\\" + byte.toString(8).padStart(3, "0") : String.fromCharCode(byte))).join("") + ")";

const textString = (text: string): string => {
  if (/^[\x20-\x7e]*$/u.test(text)) return pdfString([...text].map((character) => character.charCodeAt(0)));
  let hex = "FEFF";
  for (let index = 0; index < text.length; index += 1) hex += text.charCodeAt(index).toString(16).padStart(4, "0").toUpperCase();
  return `<${hex}>`;
};
//#endregion 🔖️WinAnsi

//#region 🔖️Colour
const rgb = (color: string): string => {
  const match = /^#([0-9a-f]{3}|[0-9a-f]{6})$/iu.exec(color.trim());
  if (!match) throw new Error(`PDF colour must be #rgb or #rrggbb, got ${JSON.stringify(color)}`);
  const hex = match[1]!.length === 3 ? [...match[1]!].map((digit) => digit + digit).join("") : match[1]!;
  return [0, 2, 4].map((offset) => num(Number.parseInt(hex.slice(offset, offset + 2), 16) / 255)).join(" ");
};

const darken = (color: string, factor: number): string => {
  const parts = rgb(color).split(" ").map((part) => num(Number(part) * factor));
  return parts.join(" ");
};
//#endregion 🔖️Colour

//#region 🔖️Hatch
type HatchPattern = NonNullable<Extract<DrawingItem, { kind: "path" }>["fill"]>["pattern"] & string;

const TILE_PT = 3.4;
const PATTERN_STREAMS: Readonly<Record<HatchPattern, string>> = (() => {
  const s = num(TILE_PT);
  const lines = (segments: readonly (readonly [number, number, number, number])[]): string => `0.35 w\n${segments.map(([a, b, c, d]) => `${num(a)} ${num(b)} m ${num(c)} ${num(d)} l S`).join("\n")}`;
  const dot = (cx: number, cy: number, r: number): string => {
    const k = 0.5523 * r;
    return `${num(cx + r)} ${num(cy)} m ${num(cx + r)} ${num(cy + k)} ${num(cx + k)} ${num(cy + r)} ${num(cx)} ${num(cy + r)} c ${num(cx - k)} ${num(cy + r)} ${num(cx - r)} ${num(cy + k)} ${num(cx - r)} ${num(cy)} c ${num(cx - r)} ${num(cy - k)} ${num(cx - k)} ${num(cy - r)} ${num(cx)} ${num(cy - r)} c ${num(cx + k)} ${num(cy - r)} ${num(cx + r)} ${num(cy - k)} ${num(cx + r)} ${num(cy)} c f`;
  };
  const t = TILE_PT;
  return {
    compression: lines([[0, 0, t, t], [-t / 2, t / 2, t / 2, 1.5 * t], [t / 2, -t / 2, 1.5 * t, t / 2]]),
    hot: lines([[0, t, t, 0], [-t / 2, t / 2, t / 2, -t / 2], [t / 2, 1.5 * t, 1.5 * t, t / 2]]),
    cold: lines([[0, t / 2, t, t / 2]]),
    vapour: lines([[t / 2, 0, t / 2, t]]),
    tension: `${dot(t / 2, t / 2, 0.45)}\n${dot(0, 0, 0.45)}\n${dot(t, 0, 0.45)}\n${dot(0, t, 0.45)}\n${dot(t, t, 0.45)}`,
  };
})();
//#endregion 🔖️Hatch

//#region 🔖️Bytes
const encoder = new TextEncoder();

class Sink {
  private readonly chunks: Uint8Array[] = [];
  length = 0;
  text(value: string): void {
    const bytes = encoder.encode(value);
    this.chunks.push(bytes);
    this.length += bytes.length;
  }
  bytes(): Uint8Array {
    const out = new Uint8Array(this.length);
    let offset = 0;
    for (const chunk of this.chunks) {
      out.set(chunk, offset);
      offset += chunk.length;
    }
    return out;
  }
}

const CRC_TABLE = (() => Uint32Array.from({ length: 256 }, (_, n) => { let c = n; for (let k = 0; k < 8; k += 1) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; }))();
const crc32 = (bytes: Uint8Array, seed = 0xffffffff): number => { let c = seed; for (const byte of bytes) c = CRC_TABLE[(c ^ byte) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
//#endregion 🔖️Bytes

//#region 🔖️Writer
type Document = { readonly objects: string[]; readonly patterns: Map<string, number>; readonly graphicsStates: Map<string, number> };
type Used = { readonly patterns: Set<HatchPattern>; readonly states: Set<string> };

const allocate = (document: Document, body = ""): number => document.objects.push(body);
const reference = (id: number): string => `${id} 0 R`;

const stream = (dictionary: string, body: string): string => `<< ${dictionary} /Length ${encoder.encode(body).length} >>\nstream\n${body}\nendstream`;

function pathData(commands: readonly PathCommand[], k: number, height: number): string {
  const x = (value: number): string => num(value * k);
  const y = (value: number): string => num(height - value * k);
  return commands.map((command) => (command.op === "Z" ? "h" : command.op === "C" ? `${x(command.x1)} ${y(command.y1)} ${x(command.x2)} ${y(command.y2)} ${x(command.x)} ${y(command.y)} c` : `${x(command.x)} ${y(command.y)} ${command.op === "M" ? "m" : "l"}`)).join("\n");
}

function patternObject(document: Document, pattern: HatchPattern): string {
  let id = document.patterns.get(pattern);
  if (id === undefined) {
    id = allocate(document, stream(`/Type /Pattern /PatternType 1 /PaintType 2 /TilingType 1 /BBox [0 0 ${num(TILE_PT)} ${num(TILE_PT)}] /XStep ${num(TILE_PT)} /YStep ${num(TILE_PT)} /Resources << >>`, PATTERN_STREAMS[pattern]));
    document.patterns.set(pattern, id);
  }
  return `/P_${pattern}`;
}

function graphicsState(document: Document, stroke: number, fill: number): string {
  const name = `/GS_${num(stroke)}_${num(fill)}`.replaceAll(".", "p");
  if (!document.graphicsStates.has(name)) document.graphicsStates.set(name, allocate(document, `<< /Type /ExtGState /CA ${num(stroke)} /ca ${num(fill)} >>`));
  return name;
}

function drawItem(document: Document, item: DrawingItem, k: number, height: number, used: Used): string {
  if (item.kind === "text") {
    if (item.text.length === 0) return "";
    const bold = item.weight === "bold";
    const size = item.sizeMm * k;
    const width = (helveticaWidth(item.text, bold) / 1000) * size;
    const shift = item.anchor === "middle" ? width / 2 : item.anchor === "end" ? width : 0;
    return `BT\n/${bold ? "F2" : "F1"} ${num(size)} Tf\n${rgb(item.color)} rg\n${num(item.x * k - shift)} ${num(height - item.y * k)} Td\n${pdfString(winAnsiBytes(item.text))} Tj\nET`;
  }
  if (item.commands.length === 0 || (!item.stroke && !item.fill)) return "";
  const path = pathData(item.commands, k, height);
  const parts = ["q"];
  const strokeAlpha = item.stroke?.opacity ?? 1;
  const fillAlpha = item.fill?.opacity ?? 1;
  if (strokeAlpha !== 1 || fillAlpha !== 1) {
    const name = graphicsState(document, strokeAlpha, fillAlpha);
    used.states.add(name);
    parts.push(`${name} gs`);
  }
  if (item.fill) {
    parts.push(`${rgb(item.fill.color)} rg`, path, "f");
    if (item.fill.pattern) {
      const name = patternObject(document, item.fill.pattern);
      used.patterns.add(item.fill.pattern);
      parts.push("/CsP cs", `${darken(item.fill.color, 0.45)} ${name} scn`, path, "f");
    }
  }
  if (item.stroke) {
    parts.push(`${rgb(item.stroke.color)} RG`, `${num(item.stroke.widthMm * k)} w`, `${item.stroke.cap === "round" ? 1 : 0} J`, `${item.stroke.join === "round" ? 1 : 0} j`);
    parts.push(item.stroke.dashMm && item.stroke.dashMm.length > 0 ? `[${item.stroke.dashMm.map((dash) => num(dash * k)).join(" ")}] 0 d` : "[] 0 d", path, "S");
  }
  parts.push("Q");
  return parts.join("\n");
}

const bilingual = (text: { readonly en: string; readonly de: string }): string => `${text.en} / ${text.de}`;

/** 📕️ Writes `pages` as a PDF 1.7 document, one page per scene; reports `(page, total)` and honours `control.signal` between pages. */
export function writePdf(pages: readonly DrawingScene[], control?: JobControl): Uint8Array {
  if (pages.length === 0) throw new Error("writePdf needs at least one scene");
  const document: Document = { objects: [], patterns: new Map(), graphicsStates: new Map() };
  const catalog = allocate(document);
  const tree = allocate(document);
  const info = allocate(document);
  const fonts = [allocate(document, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"), allocate(document, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>")];
  const groups = new Map<string, number>();
  const group = (title: { readonly en: string; readonly de: string }, id: string): number => {
    let object = groups.get(id);
    if (object === undefined) {
      object = allocate(document, `<< /Type /OCG /Name ${textString(title.en)} >>`);
      groups.set(id, object);
    }
    return object;
  };
  const kids: number[] = [];
  const first = pages[0]!;
  pages.forEach((scene, index) => {
    control?.signal?.throwIfAborted();
    const k = PT_PER_MM;
    const width = scene.widthMm * k;
    const height = scene.heightMm * k;
    const used = { patterns: new Set<HatchPattern>(), states: new Set<string>() };
    const properties: string[] = [];
    const body: string[] = [];
    if (scene.background) body.push(`q ${rgb(scene.background)} rg 0 0 ${num(width)} ${num(height)} re f Q`);
    scene.layers.forEach((layer, layerIndex) => {
      const items = layer.items.map((item) => drawItem(document, item, k, height, used)).filter((code) => code.length > 0);
      if (items.length === 0) return;
      const name = `L${layerIndex}`;
      properties.push(`/${name} ${reference(group(layer.title, layer.id))}`);
      body.push(`/OC /${name} BDC`, ...items, "EMC");
    });
    const content = allocate(document, stream("", body.join("\n")));
    const patternResources = [...used.patterns].map((pattern) => `/P_${pattern} ${reference(document.patterns.get(pattern)!)}`).join(" ");
    const stateResources = [...used.states].map((name) => `${name} ${reference(document.graphicsStates.get(name)!)}`).join(" ");
    const resources = `<< /Font << /F1 ${reference(fonts[0]!)} /F2 ${reference(fonts[1]!)} >> /ProcSet [/PDF /Text] ${used.patterns.size > 0 ? `/ColorSpace << /CsP [/Pattern /DeviceRGB] >> /Pattern << ${patternResources} >>` : ""} ${used.states.size > 0 ? `/ExtGState << ${stateResources} >>` : ""} ${properties.length > 0 ? `/Properties << ${properties.join(" ")} >>` : ""} >>`;
    kids.push(allocate(document, `<< /Type /Page /Parent ${reference(tree)} /MediaBox [0 0 ${num(width)} ${num(height)}] /UserUnit 1 /Contents ${reference(content)} /Resources ${resources} /Tabs /S >>`));
    control?.progress?.(index + 1, pages.length, { en: "Writing PDF pages", de: "PDF-Seiten werden geschrieben" });
  });
  const order = [...groups.values()].map(reference).join(" ");
  document.objects[tree - 1] = `<< /Type /Pages /Count ${kids.length} /Kids [${kids.map(reference).join(" ")}] >>`;
  document.objects[catalog - 1] = `<< /Type /Catalog /Pages ${reference(tree)} /PageMode /UseNone /ViewerPreferences << /DisplayDocTitle true >> /OCProperties << /OCGs [${order}] /D << /BaseState /ON /Order [${order}] >> >> >>`;
  const meta = first.metadata;
  const subject = `Staffage ${meta.asset}, scale 1:${num(meta.scale.denominator)} (${num(meta.mmPerMetre)} mm/m), t = ${num(meta.timeS)} s, view ${meta.view}, ${meta.viewpoint}`;
  document.objects[info - 1] = `<< /Title ${textString(bilingual(meta.title))} /Subject ${textString(subject)} /Keywords ${textString(`${meta.asset}, 1:${num(meta.scale.denominator)}, ${meta.view}`)} /Creator ${textString("semio staffage archive")} /Producer ${textString("semio staffage PDF writer")} >>`;
  const out = new Sink();
  out.text("%PDF-1.7\n%âãÏÓ\n");
  const offsets: number[] = [];
  document.objects.forEach((body, index) => {
    offsets.push(out.length);
    out.text(`${index + 1} 0 obj\n${body}\nendobj\n`);
  });
  const xref = out.length;
  const digest = crc32(out.bytes());
  const id = (digest.toString(16).padStart(8, "0") + crc32(out.bytes(), digest ^ 0xffffffff).toString(16).padStart(8, "0")).repeat(2);
  out.text(`xref\n0 ${document.objects.length + 1}\n0000000000 65535 f \n${offsets.map((offset) => `${String(offset).padStart(10, "0")} 00000 n \n`).join("")}trailer\n<< /Size ${document.objects.length + 1} /Root ${reference(catalog)} /Info ${reference(info)} /ID [<${id}> <${id}>] >>\nstartxref\n${xref}\n%%EOF\n`);
  return out.bytes();
}
//#endregion 🔖️Writer
