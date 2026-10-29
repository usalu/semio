/** 📤️ Browser export actions: one shared evaluator feeds SVG, PDF (single page or all seven views), DXF, DWG, PNG sequence, MP4 and WebM; every action reports progress and honours cancellation.
 *
 * Nothing is mislabelled: a format the runtime cannot produce returns `{ status: "unavailable", reason: { en, de } }`, and a video reports the codec and alpha behaviour the browser really chose.
 * Metadata embedded where the format allows: SVG `<metadata>`, PDF info dictionary, DXF `999` comments, PNG `tEXt`, ZIP comment and `metadata.json`.
 *
 * @see ../🧮️pipeline/🟦️.ts — the evaluator every export shares
 * @see ./🖼️raster/🟦️.ts — canvas rasterisation and `MediaRecorder`
 * @see ./🗜️zip/🟦️.ts — ZIP and PNG text chunks
 */
import * as library from "@teaching/staffage";
import { modelScene, writeDxf, writePdf, writeSvg } from "@teaching/staffage";
import type { Asset, DrawingScene, EvaluationQuery, JobControl, ModelScene, Text } from "@teaching/staffage";
import { allViews, depictionOptions, evaluateState, queryAt, sceneOf, structureOf, thermalOf } from "../🧮️pipeline/🟦️.ts";
import type { DepictionRequest } from "../🧮️pipeline/🟦️.ts";
import { browserRasteriser, browserRecorder, canRasterise, recorderMimes } from "./🖼️raster/🟦️.ts";
import type { Rasteriser, Recorder } from "./🖼️raster/🟦️.ts";
import { withPngText, writeZip } from "./🗜️zip/🟦️.ts";

/** 📤️ The formats the archive exports. */
export type ExportFormat = "svg" | "pdf" | "dxf" | "dwg" | "png-sequence" | "mp4" | "webm";

/** 🎛️ What to export: the depiction request plus sheet, frame and raster settings. */
export type ExportOptions = DepictionRequest & {
  readonly sheet?: "single" | "all-views";
  readonly fps?: number;
  readonly durationS?: number;
  readonly pixelsPerMm?: number;
};

/** 🧰️ Replaceable host capabilities (tests, workers, native shells). */
export type ExportHost = { readonly rasterise?: Rasteriser; readonly record?: Recorder; readonly dwg?: (scene: ModelScene) => Promise<Uint8Array> };

/** 🧾️ What an export honestly reports about itself. */
export type ExportDetails = { readonly frames?: number; readonly fps?: number; readonly widthPx?: number; readonly heightPx?: number; readonly pages?: number; readonly codec?: string; readonly alpha?: boolean; readonly metadataEmbedded: boolean; readonly notes: readonly Text[] };

/** 📦️ The outcome of an export. */
export type ExportResult =
  | { readonly status: "done"; readonly format: ExportFormat; readonly filename: string; readonly mime: string; readonly bytes: Uint8Array; readonly blob: Blob; readonly details: ExportDetails }
  | { readonly status: "unavailable"; readonly format: ExportFormat; readonly reason: Text }
  | { readonly status: "cancelled"; readonly format: ExportFormat };

/** 🔎️ What one format can do in this runtime. */
export type FormatCapability = { readonly available: boolean; readonly reason?: Text; readonly codec?: string; readonly alpha?: boolean };

/** 🔎️ The probed capabilities of every format. */
export type ExportCapabilities = Readonly<Record<ExportFormat, FormatCapability>>;

const EXTENSION: Record<ExportFormat, string> = { svg: "svg", pdf: "pdf", dxf: "dxf", dwg: "dwg", "png-sequence": "zip", mp4: "mp4", webm: "webm" };
const MIME: Record<ExportFormat, string> = { svg: "image/svg+xml", pdf: "application/pdf", dxf: "image/vnd.dxf", dwg: "image/vnd.dwg", "png-sequence": "application/zip", mp4: "video/mp4", webm: "video/webm" };
const MAX_SIDE_PX = 8192;
const VIDEO_SIDE_PX = 1280;
const DEFAULT_FPS = 30;
const encoder = new TextEncoder();

const NO_CANVAS: Text = { en: "This browser cannot draw SVG into a canvas, so frames cannot be rasterised.", de: "Dieser Browser kann SVG nicht in ein Canvas zeichnen, daher können keine Bilder gerastert werden." };
const NO_RECORDER = (container: string): Text => ({ en: `This browser's MediaRecorder cannot produce ${container}, and no first-party encoder is bundled for it.`, de: `Der MediaRecorder dieses Browsers kann kein ${container} erzeugen, und es ist kein eigener Encoder dafür enthalten.` });
const NO_CLIP: Text = { en: "Frame sequences and videos need a clip query, not a pose.", de: "Bildfolgen und Videos benötigen eine Clip-Abfrage, keine Pose." };
const NO_DWG: Text = { en: "DWG export needs the DWG encoder, which is not available in this build.", de: "Der DWG-Export braucht den DWG-Encoder, der in diesem Build nicht verfügbar ist." };

type DwgWriter = (scene: ModelScene) => Promise<Uint8Array>;

function dwgWriter(host?: ExportHost): DwgWriter | null {
  if (host?.dwg !== undefined) return host.dwg;
  const candidate = (library as unknown as Record<string, unknown>).writeDwg;
  return typeof candidate === "function" ? (candidate as DwgWriter) : null;
}

/** 🔎️ Probes the real support of every format in this runtime; DWG is probed by encoding an empty drawing. */
export async function exportCapabilities(host?: ExportHost): Promise<ExportCapabilities> {
  const raster = host?.rasterise !== undefined || canRasterise();
  const mp4 = recorderMimes("mp4");
  const webm = recorderMimes("webm");
  const recorder = host?.record !== undefined;
  const dwg = dwgWriter(host);
  let dwgReason: Text | undefined;
  if (dwg === null) dwgReason = NO_DWG;
  else
    try {
      const bytes = await dwg({ units: "m", asset: "probe", timeS: 0, layers: [] });
      if (bytes.length < 6 || !String.fromCharCode(...bytes.subarray(0, 6)).startsWith("AC10")) dwgReason = NO_DWG;
    } catch {
      dwgReason = NO_DWG;
    }
  return {
    svg: { available: true },
    pdf: { available: true },
    dxf: { available: true },
    dwg: dwgReason === undefined ? { available: true } : { available: false, reason: dwgReason },
    "png-sequence": raster ? { available: true, alpha: true } : { available: false, reason: NO_CANVAS },
    mp4: raster && (recorder || mp4.length > 0) ? { available: true, codec: mp4[0] ?? "host recorder", alpha: false } : { available: false, reason: raster ? NO_RECORDER("MP4") : NO_CANVAS },
    webm: raster && (recorder || webm.length > 0) ? { available: true, codec: webm[0] ?? "host recorder", alpha: false } : { available: false, reason: raster ? NO_RECORDER("WebM") : NO_CANVAS },
  };
}

const slug = (text: string): string => text.replace(/[^A-Za-z0-9.-]+/gu, "-");

function timeTag(seconds: number): string {
  return `t${seconds.toFixed(2).replace(".", "p")}`;
}

/** 🏷️ The export file name: `<asset>_<pose | clip-t…>_<view>_1-<denominator>.<ext>`. */
export function exportFilename(asset: Asset, query: EvaluationQuery, request: DepictionRequest, format: ExportFormat, sheet: "single" | "all-views" = "single"): string {
  const what = query.kind === "pose" ? query.pose : query.kind === "clip" ? (format === "png-sequence" || format === "mp4" || format === "webm" ? query.clip : `${query.clip}-${timeTag(query.timeS)}`) : `${query.from}-to-${query.to}`;
  const view = sheet === "all-views" && format === "pdf" ? "all-views" : request.view;
  return `${slug(asset.id)}_${slug(what)}_${view}_1-${request.scale.denominator}.${EXTENSION[format]}`;
}

function metadataText(scene: DrawingScene): Record<string, string> {
  const { metadata } = scene;
  return {
    Title: `${metadata.title.en} / ${metadata.title.de}`,
    Software: "semio staffage archive",
    Description: `asset ${metadata.asset}, view ${metadata.view}, viewpoint ${metadata.viewpoint}, scale 1:${metadata.scale.denominator}, t ${metadata.timeS} s, paper ${scene.widthMm} x ${scene.heightMm} mm`,
    Comment: JSON.stringify({ asset: metadata.asset, query: metadata.query, timeS: metadata.timeS, view: metadata.view, viewpoint: metadata.viewpoint, scale: metadata.scale.denominator, mmPerMetre: metadata.mmPerMetre }),
  };
}

const escapeXml = (text: string): string => text.replace(/[&<>"]/gu, (c) => `&#${c.charCodeAt(0)};`);

function svgWithMetadata(scene: DrawingScene): string {
  const markup = writeSvg(scene);
  if (markup.includes("<metadata")) return markup;
  const entries = Object.entries(metadataText(scene)).map(([key, value]) => `<staffage:${key.toLowerCase()}>${escapeXml(value)}</staffage:${key.toLowerCase()}>`).join("");
  const block = `<metadata xmlns:staffage="https://semio.tech/staffage/v1">${entries}</metadata>`;
  return markup.replace(/(<svg\b[^>]*>)/u, `$1${block}`);
}

function dxfWithMetadata(dxf: string, scene: DrawingScene): string {
  const comments = Object.entries(metadataText(scene)).map(([key, value]) => `999\n${key}: ${[...value].map((c) => (c.charCodeAt(0) < 0x7f && c.charCodeAt(0) >= 0x20 ? c : "?")).join("")}`).join("\n");
  return `${comments}\n${dxf}`;
}

function done(format: ExportFormat, filename: string, bytes: Uint8Array, details: Partial<ExportDetails> = {}): ExportResult {
  return { status: "done", format, filename, mime: MIME[format], bytes, blob: new Blob([bytes as BlobPart], { type: MIME[format] }), details: { metadataEmbedded: true, notes: [], ...details } };
}

function check(control?: JobControl): void {
  if (control?.signal?.aborted) throw new DOMException("Export cancelled", "AbortError");
}

const isAbort = (error: unknown): boolean => error instanceof DOMException && error.name === "AbortError";

function rasterSize(scene: DrawingScene, pixelsPerMm: number, maxSide: number): { widthPx: number; heightPx: number; svgScale: number } {
  const fit = Math.min(1, maxSide / Math.max(scene.widthMm * pixelsPerMm, scene.heightMm * pixelsPerMm));
  return { widthPx: Math.max(1, Math.round(scene.widthMm * pixelsPerMm * fit)), heightPx: Math.max(1, Math.round(scene.heightMm * pixelsPerMm * fit)), svgScale: fit };
}

async function* renderFrames(asset: Asset, query: EvaluationQuery, options: ExportOptions, count: number, fps: number, size: { widthPx: number; heightPx: number }, rasterise: Rasteriser, control: JobControl | undefined, label: Text, flatten: boolean): AsyncGenerator<{ readonly index: number; readonly png: Uint8Array; readonly scene: DrawingScene }> {
  for (let n = 0; n < count; n++) {
    check(control);
    const state = evaluateState(asset, queryAt(query, n / fps));
    const scene = sceneOf(state, depictionOptions(options));
    const svg = flatten && scene.background === null ? writeSvg({ ...scene, background: "#ffffff" }) : writeSvg(scene);
    const png = await rasterise(svg, size.widthPx, size.heightPx, control?.signal);
    control?.progress?.(n + 1, count, label);
    yield { index: n, png, scene };
  }
}

function clipDuration(asset: Asset, query: EvaluationQuery, options: ExportOptions): number | null {
  if (options.durationS !== undefined) return options.durationS;
  if (query.kind === "clip") return asset.clips.find((clip) => clip.id === query.clip)?.durationS ?? null;
  if (query.kind === "transition") return query.durationS;
  return null;
}

/** 📤️ Exports one figure (or its clip) in `format`; shares the memoised evaluator with every on-screen figure. */
export async function exportFigure(asset: Asset, query: EvaluationQuery, options: ExportOptions, format: ExportFormat, control?: JobControl, host?: ExportHost): Promise<ExportResult> {
  try {
    check(control);
    const sheet = options.sheet ?? "single";
    const state = evaluateState(asset, query);
    const label = (en: string, de: string): Text => ({ en, de });

    if (format === "svg") {
      const scene = sceneOf(state, depictionOptions(options));
      control?.progress?.(1, 1, label("SVG written", "SVG geschrieben"));
      return done(format, exportFilename(asset, query, options, format), encoder.encode(svgWithMetadata(scene)));
    }

    if (format === "pdf") {
      const scenes = sheet === "all-views" ? allViews(state, options) : [sceneOf(state, depictionOptions(options))];
      const bytes = writePdf(scenes, { ...(control?.signal === undefined ? {} : { signal: control.signal }), progress: (d, t, l) => control?.progress?.(d, t, l) });
      return done(format, exportFilename(asset, query, options, format, sheet), bytes, { pages: scenes.length });
    }

    if (format === "dxf" || format === "dwg") {
      const request = depictionOptions(options);
      const analysis = request.view === "thermal" ? { thermal: thermalOf(state, request.environment) } : request.view === "forces" || request.view === "moments" ? { structure: structureOf(state) } : undefined;
      const diagrams = request.view === "forces" || request.view === "moments" ? (["axial", "shear", "moment"] as const).filter((kind) => request.diagrams[kind]) : [];
      const model = modelScene(state, { viewpoint: request.viewpoint, ...(analysis === undefined ? {} : { analysis }), diagrams });
      const scene = sceneOf(state, request);
      if (format === "dxf") {
        control?.progress?.(1, 1, label("DXF written", "DXF geschrieben"));
        return done(format, exportFilename(asset, query, options, format), encoder.encode(dxfWithMetadata(writeDxf(model), scene)));
      }
      const write = dwgWriter(host);
      if (write === null) return { status: "unavailable", format, reason: NO_DWG };
      let bytes: Uint8Array;
      try {
        bytes = await write(model);
      } catch (error) {
        if (isAbort(error)) throw error;
        return { status: "unavailable", format, reason: { en: `${NO_DWG.en} (${String(error)})`, de: `${NO_DWG.de} (${String(error)})` } };
      }
      control?.progress?.(1, 1, label("DWG written", "DWG geschrieben"));
      return done(format, exportFilename(asset, query, options, format), bytes, { metadataEmbedded: false, notes: [label("The DWG carries layers and geometry; descriptive metadata is in the file name only.", "Die DWG enthält Layer und Geometrie; beschreibende Metadaten stehen nur im Dateinamen.")] });
    }

    const rasterise = host?.rasterise ?? (canRasterise() ? browserRasteriser : null);
    if (rasterise === null) return { status: "unavailable", format, reason: NO_CANVAS };
    const durationS = clipDuration(asset, query, options);
    if (query.kind === "pose" && durationS === null) return { status: "unavailable", format, reason: NO_CLIP };
    const fps = options.fps ?? DEFAULT_FPS;
    const count = Math.max(1, Math.round((durationS ?? 0) * fps));
    const first = sceneOf(evaluateState(asset, queryAt(query, 0)), depictionOptions(options));
    const video = format === "mp4" || format === "webm";
    const ppm = options.pixelsPerMm ?? 96 / 25.4 * 2;
    const size = rasterSize(first, ppm, video ? VIDEO_SIDE_PX : MAX_SIDE_PX);
    const even = video ? { widthPx: size.widthPx + (size.widthPx % 2), heightPx: size.heightPx + (size.heightPx % 2) } : size;

    if (format === "png-sequence") {
      const text = metadataText(first);
      const entries: { name: string; bytes: Uint8Array }[] = [];
      const base = exportFilename(asset, query, options, format).replace(/\.zip$/u, "");
      const width = String(count).length;
      for await (const frame of renderFrames(asset, query, options, count, fps, even, rasterise, control, label("Rendering frames", "Bilder werden gerendert"), false))
        entries.push({ name: `${base}_${String(frame.index + 1).padStart(Math.max(width, 4), "0")}.png`, bytes: withPngText(frame.png, { ...text, Comment: JSON.stringify({ ...JSON.parse(text.Comment!), frame: frame.index + 1, fps, timeS: frame.index / fps }) }) });
      const info = { ...JSON.parse(text.Comment!), frames: count, fps, widthPx: even.widthPx, heightPx: even.heightPx, alpha: first.background === null };
      entries.push({ name: "metadata.json", bytes: encoder.encode(JSON.stringify(info, null, 2)) });
      return done(format, exportFilename(asset, query, options, format), writeZip(entries, text.Description), { frames: count, fps, widthPx: even.widthPx, heightPx: even.heightPx, alpha: first.background === null });
    }

    const record = host?.record ?? (recorderMimes(format).length > 0 ? browserRecorder : null);
    const mime = recorderMimes(format)[0] ?? `video/${format}`;
    if (record === null) return { status: "unavailable", format, reason: NO_RECORDER(format === "mp4" ? "MP4" : "WebM") };
    const rendered: Uint8Array[] = [];
    for await (const frame of renderFrames(asset, query, options, count, fps, even, rasterise, control, label("Rendering frames", "Bilder werden gerendert"), true)) rendered.push(frame.png);
    const frames = (async function* () {
      yield* rendered;
    })();
    const recorded = await record(frames, even, fps, mime, control?.signal);
    const notes: Text[] = [label("Video containers from MediaRecorder carry no descriptive metadata; it is in the file name.", "Videocontainer des MediaRecorders tragen keine beschreibenden Metadaten; sie stehen im Dateinamen.")];
    if (first.background === null) notes.push(label("Alpha is not preserved: transparent frames are flattened onto white.", "Alpha bleibt nicht erhalten: transparente Bilder werden auf Weiß abgeflacht."));
    return done(format, exportFilename(asset, query, options, format), recorded.bytes, { frames: count, fps, widthPx: even.widthPx, heightPx: even.heightPx, codec: recorded.mime, alpha: false, metadataEmbedded: false, notes });
  } catch (error) {
    if (isAbort(error)) return { status: "cancelled", format };
    throw error;
  }
}

/** 💾️ Hands a finished export to the browser as a download. */
export function saveExport(result: ExportResult): void {
  if (result.status !== "done") return;
  const url = URL.createObjectURL(result.blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = result.filename;
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 0);
}
