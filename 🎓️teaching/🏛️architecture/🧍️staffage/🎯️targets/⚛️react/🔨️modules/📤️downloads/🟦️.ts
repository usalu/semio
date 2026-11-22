/** 📤️ Browser export actions: one shared evaluator feeds SVG, PDF (single page or all seven views), DXF, DWG, PNG sequence, MP4 and WebM; every action reports progress and honours cancellation.
 *
 * Air flow. The request decides it (`flow`, `flowConditions`, `flowDetail`, `environment`), so a file shows what the screen shows: the flow is developed first (progress, cancellable), SVG and PDF draw it on paper (in a PDF of all views on the thermal page), DXF and DWG carry it as model-space geometry on the `FLOW-…` layers, and the frames of a PNG sequence, MP4 or WebM advance it with their times on one fixed canvas (the union of all frames about the world origin, so nothing jumps). A looping clip closes its loop; a pose with flow plays it for `durationS` (default {@link POSE_PLAY_S} s) on a standing figure.
 *
 * Nothing is mislabelled: a format the runtime cannot produce returns `{ status: "unavailable", reason: { en, de } }`, and a video reports the codec and alpha behaviour the browser really chose.
 * Metadata embedded where the format allows: SVG `<metadata>`, PDF info dictionary, DXF `999` comments, PNG `tEXt`, ZIP comment and `metadata.json`.
 *
 * @see ../🧮️pipeline/🟦️.ts — the evaluator every export shares
 * @see ./🖼️raster/🟦️.ts — canvas rasterisation and `MediaRecorder`
 * @see ../../../../🔨️modules/📤️export/🗜️zip/🟦️.ts — ZIP and PNG text chunks
 */
import * as library from "@teaching/staffage";
import { SVG_NAMESPACE, developFlow, modelScene, shownFlow, transportOf, withPngText, writeDxf, writePdf, writeSvg, writeZip } from "@teaching/staffage";
import type { ExportFormat as SchemaExportFormat } from "@teaching/staffage";
import type { Asset, DrawingScene, EvaluationQuery, JobControl, ModelScene, Slug, Text, View } from "@teaching/staffage";
import { VIEWPOINT_PHRASES } from "../🔭️viewpoints/🟦️.ts";
import { ensureViewpoint } from "../🧮️pipeline/🦥️lazy/🟦️.ts";
import { allViews, depictionOptions, evaluateState, frameOf, queryAt, sceneOf, sceneWithAir, structureOf, thermalOf } from "../🧮️pipeline/🟦️.ts";
import type { DepictionRequest, SceneFrame } from "../🧮️pipeline/🟦️.ts";
import { browserRasteriser, browserRecorder, canRasterise, recorderMimes } from "./🖼️raster/🟦️.ts";
import type { Rasteriser, Recorder } from "./🖼️raster/🟦️.ts";

/** 🗂️ The formats the archive exports: the schema's `ExportFormat` without `component`, which is the embed snippet and no file. */
export type ExportFormat = Exclude<SchemaExportFormat, "component">;

/** 🧊️ The 3D formats: made by the lazily loaded model pipeline, independent of the viewpoint. */
export const SPATIAL_EXPORT_FORMATS = ["glb", "fbx", "model", "model-zip"] as const;

/** 🖨️ One of {@link SPATIAL_EXPORT_FORMATS}. */
export type SpatialExportFormat = (typeof SPATIAL_EXPORT_FORMATS)[number];

/** ❓️ Whether a format is a 3D format. */
export const isSpatialExport = (format: ExportFormat): format is SpatialExportFormat => (SPATIAL_EXPORT_FORMATS as readonly string[]).includes(format);

/** 🎛️ What to export: the depiction request plus sheet, frame and raster settings. */
export type ExportOptions = DepictionRequest & {
  readonly sheet?: "single" | "all-views";
  readonly fps?: number;
  readonly durationS?: number;
  readonly pixelsPerMm?: number;
  readonly pose?: Slug;
  readonly clip?: Slug;
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

/** 📥️ The probed capabilities of every format. */
export type ExportCapabilities = Readonly<Record<ExportFormat, FormatCapability>>;

const EXTENSION: Record<ExportFormat, string> = { svg: "svg", pdf: "pdf", dxf: "dxf", dwg: "dwg", "png-sequence": "zip", mp4: "mp4", webm: "webm", glb: "glb", fbx: "fbx", model: "json", "model-zip": "zip" };
const MIME: Record<ExportFormat, string> = { svg: "image/svg+xml", pdf: "application/pdf", dxf: "image/vnd.dxf", dwg: "image/vnd.dwg", "png-sequence": "application/zip", mp4: "video/mp4", webm: "video/webm", glb: "model/gltf-binary", fbx: "application/octet-stream", model: "application/json", "model-zip": "application/zip" };
const MAX_SIDE_PX = 8192;
const VIDEO_SIDE_PX = 1280;
const DEFAULT_FPS = 30;

/** ▶️ Seconds a pose plays its air flow as a frame sequence or video when the options name no duration. */
export const POSE_PLAY_S = 6;
const encoder = new TextEncoder();

const NO_CANVAS: Text = { en: "This browser cannot draw SVG into a canvas, so frames cannot be rasterised.", de: "Dieser Browser kann SVG nicht in ein Canvas zeichnen, daher können keine Bilder gerastert werden." };
const NO_RECORDER = (container: string): Text => ({ en: `This browser's MediaRecorder cannot produce ${container}, and no first-party encoder is bundled for it.`, de: `Der MediaRecorder dieses Browsers kann kein ${container} erzeugen, und es ist kein eigener Encoder dafür enthalten.` });
const NO_CLIP: Text = { en: "Frame sequences and videos need a clip, or a pose whose air flow plays (thermal view with flow layers), not a still pose.", de: "Bildfolgen und Videos benötigen einen Clip oder eine Pose, deren Luftströmung abläuft (thermische Ansicht mit Strömungsebenen), keine stehende Pose." };
const NO_DWG: Text = { en: "DWG export needs the DWG encoder, which is not available in this build.", de: "Der DWG-Export braucht den DWG-Encoder, der in diesem Build nicht verfügbar ist." };

type DwgWriter = (scene: ModelScene) => Promise<Uint8Array>;

function dwgWriter(host?: ExportHost): DwgWriter | null {
  if (host?.dwg !== undefined) return host.dwg;
  const candidate = (library as unknown as Record<string, unknown>).writeDwg;
  return typeof candidate === "function" ? (candidate as DwgWriter) : null;
}

/** ⏱️ Probes the real support of every format in this runtime; DWG is probed by encoding an empty drawing. */
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
      const bytes = await dwg({ units: "m", asset: "probe", timeS: 0, representation: { kind: "none" }, layers: [] });
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
    glb: { available: true },
    fbx: { available: true },
    model: { available: true },
    "model-zip": { available: true },
    mp4: raster && (recorder || mp4.length > 0) ? { available: true, codec: mp4[0] ?? "host recorder", alpha: false } : { available: false, reason: raster ? NO_RECORDER("MP4") : NO_CANVAS },
    webm: raster && (recorder || webm.length > 0) ? { available: true, codec: webm[0] ?? "host recorder", alpha: false } : { available: false, reason: raster ? NO_RECORDER("WebM") : NO_CANVAS },
  };
}

const slug = (text: string): string => text.replace(/[^A-Za-z0-9.-]+/gu, "-");

function timeTag(seconds: number): string {
  return `t${seconds.toFixed(2).replace(".", "p")}`;
}

/** 🏷️ The file name of a 3D download, the very name the library, the batch and the command line give the same file: `<asset>.<pose>.glb` (every authored scale and mode as scenes), `<asset>.<pose>.pose.json` or `<asset>.<clip>.clip.json` (the multi-scene model), `<asset>.<clip>.s<scale>.<view>.fbx` (one clip at one scale in one view) and `<asset>.zip`. */
export const spatialFilename = (asset: Asset, format: SpatialExportFormat, what: { readonly pose?: Slug; readonly clip?: Slug }, scale: number, view: View): string => {
  switch (format) {
    case "glb":
      return library.glbName(asset.id, what.pose!);
    case "fbx":
      return library.fbxName(asset.id, what.clip!, scale, view);
    case "model":
      return what.pose === undefined ? library.modelName(asset.id, "clip", what.clip!) : library.modelName(asset.id, "pose", what.pose);
    case "model-zip":
      return library.zipName(asset.id);
  }
};

/** 📏️ The export file name: `<asset>_<pose | clip-t…>_<viewpoint>_<representation>_1-<denominator>.<ext>`, so the outputs of the three viewpoints never collide; a 3D format is named by {@link spatialFilename}, the name of the library. */
export function exportFilename(asset: Asset, query: EvaluationQuery, request: DepictionRequest, format: ExportFormat, sheet: "single" | "all-views" = "single"): string {
  if (isSpatialExport(format)) return spatialFilename(asset, format, query.kind === "pose" ? { pose: query.pose } : query.kind === "clip" ? { clip: query.clip } : {}, request.scale.denominator, request.view);
  const what = query.kind === "pose" ? query.pose : query.kind === "clip" ? (format === "png-sequence" || format === "mp4" || format === "webm" ? query.clip : `${query.clip}-${timeTag(query.timeS)}`) : `${query.from}-to-${query.to}`;
  const view = sheet === "all-views" && format === "pdf" ? "all-views" : request.view;
  return `${slug(asset.id)}_${slug(what)}_${request.viewpoint}_${view}_1-${request.scale.denominator}.${EXTENSION[format]}`;
}

function metadataText(scene: DrawingScene): Record<string, string> {
  const { metadata } = scene;
  return {
    Title: `${metadata.title.en} / ${metadata.title.de}`,
    Software: "semio staffage archive",
    Description: `asset ${metadata.asset}, representation ${metadata.view}, viewpoint ${VIEWPOINT_PHRASES[metadata.viewpoint].en} (${metadata.viewpoint}), scale 1:${metadata.scale.denominator}, t ${metadata.timeS} s, paper ${scene.widthMm} x ${scene.heightMm} mm`,
    Comment: JSON.stringify({ asset: metadata.asset, query: metadata.query, timeS: metadata.timeS, view: metadata.view, viewpoint: metadata.viewpoint, scale: metadata.scale.denominator, mmPerMetre: metadata.mmPerMetre }),
  };
}

const escapeXml = (text: string): string => text.replace(/[&<>"]/gu, (c) => `&#${c.charCodeAt(0)};`);

function svgWithMetadata(scene: DrawingScene): string {
  const markup = writeSvg(scene);
  if (markup.includes("<metadata")) return markup;
  const entries = Object.entries(metadataText(scene)).map(([key, value]) => `<staffage:${key.toLowerCase()}>${escapeXml(value)}</staffage:${key.toLowerCase()}>`).join("");
  const block = `<metadata xmlns:staffage="${SVG_NAMESPACE}">${entries}</metadata>`;
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

const isAbort = (error: unknown): boolean => typeof error === "object" && error !== null && (error as { readonly name?: unknown }).name === "AbortError";

function rasterSize(frame: SceneFrame, pixelsPerMm: number, maxSide: number): { widthPx: number; heightPx: number; svgScale: number } {
  const widthMm = frame.maxX - frame.minX;
  const heightMm = frame.maxY - frame.minY;
  const fit = Math.min(1, maxSide / Math.max(widthMm * pixelsPerMm, heightMm * pixelsPerMm));
  return { widthPx: Math.max(1, Math.round(widthMm * pixelsPerMm * fit)), heightPx: Math.max(1, Math.round(heightMm * pixelsPerMm * fit)), svgScale: fit };
}

const ROOT_TAG = /<svg\b[^>]*>/u;

/** 🪟️ The SVG of a scene on a fixed window about its world origin: the root takes the size and viewBox of `frame`, and `background` (when not `null`) is painted over the whole window first, so every frame of a sequence has the same canvas. */
export function framedSvg(scene: DrawingScene, frame: SceneFrame, background: string | null): string {
  const x = scene.metadata.origin.xMm + frame.minX;
  const y = scene.metadata.origin.yMm + frame.minY;
  const width = frame.maxX - frame.minX;
  const height = frame.maxY - frame.minY;
  const root = (ROOT_TAG.exec(writeSvg(scene))?.[0] ?? "<svg>").replace(/\s(?:width|height|viewBox)="[^"]*"/gu, "").replace(/>$/u, ` width="${width}mm" height="${height}mm" viewBox="${x} ${y} ${width} ${height}">`);
  return writeSvg(scene).replace(ROOT_TAG, root + (background === null ? "" : `<rect x="${x}" y="${y}" width="${width}" height="${height}" fill="${background}"/>`));
}

const macrotask = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));

/** 🎞️ The scenes of a frame schedule: a clip or transition is evaluated at every frame time (its flow advances with it), a pose keeps its figure and only its air moves on. Yields to the event loop between frames, so progress shows and a cancellation lands. */
async function frameScenes(asset: Asset, query: EvaluationQuery, options: ExportOptions, count: number, fps: number, control: JobControl | undefined, label: Text): Promise<DrawingScene[]> {
  const request = depictionOptions(options);
  const pose = query.kind === "pose" ? query.pose : null;
  const still = pose === null ? null : evaluateState(asset, { kind: "pose", pose }, request.representation);
  const airy = (request.flow?.length ?? 0) > 0;
  const scenes: DrawingScene[] = [];
  for (let n = 0; n < count; n++) {
    check(control);
    scenes.push(still === null || pose === null ? sceneOf(evaluateState(asset, queryAt(query, n / fps), request.representation), request) : airy ? sceneWithAir(still, request, transportOf({ asset, query: { kind: "pose", pose, timeS: n / fps } }, request)) : sceneOf(still, request));
    control?.progress?.(n + 1, count, label);
    await macrotask();
  }
  return scenes;
}

async function* renderFrames(scenes: readonly DrawingScene[], frame: SceneFrame, size: { widthPx: number; heightPx: number }, rasterise: Rasteriser, control: JobControl | undefined, label: Text, flatten: boolean): AsyncGenerator<{ readonly index: number; readonly png: Uint8Array; readonly scene: DrawingScene }> {
  for (const [n, scene] of scenes.entries()) {
    check(control);
    const png = await rasterise(framedSvg(scene, frame, scene.background ?? (flatten ? "#ffffff" : null)), size.widthPx, size.heightPx, control?.signal);
    control?.progress?.(n + 1, scenes.length, label);
    yield { index: n, png, scene };
  }
}

function clipDuration(asset: Asset, query: EvaluationQuery, options: ExportOptions): number | null {
  if (options.durationS !== undefined) return options.durationS;
  if (query.kind === "clip") return asset.clips.find((clip) => clip.id === query.clip)?.durationS ?? null;
  if (query.kind === "transition") return query.durationS;
  return null;
}

/** 🧠️ Exports one figure (or its clip) in `format`; shares the memoised evaluator with every on-screen figure. */
export async function exportFigure(asset: Asset, query: EvaluationQuery, options: ExportOptions, format: ExportFormat, control?: JobControl, host?: ExportHost): Promise<ExportResult> {
  try {
    check(control);
    const label = (en: string, de: string): Text => ({ en, de });
    if (isSpatialExport(format)) return await exportSpatialFigure(asset, query, options, format, control);
    await ensureViewpoint(options.viewpoint);
    const sheet = options.sheet ?? "single";
    const state = evaluateState(asset, query, depictionOptions(options).representation);
    const airy = (depictionOptions(options).flow?.length ?? 0) > 0;
    if (airy && query.kind !== "transition") await developFlow(state, depictionOptions(options), control ?? {});

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
      const solved = airy ? transportOf(state, request).flow : null;
      const field = solved === null ? null : shownFlow(state, solved);
      const model = modelScene(state, { viewpoint: request.viewpoint, ...(analysis === undefined ? {} : { analysis }), diagrams, ...(field === null ? {} : { flow: { field, layers: request.flow!, denominator: request.scale.denominator, range: request.range, ...(request.flowDetail === undefined ? {} : { detail: request.flowDetail }) } }) });
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
    const durationS = clipDuration(asset, query, options) ?? (query.kind === "pose" && airy ? POSE_PLAY_S : null);
    if (query.kind === "pose" && durationS === null) return { status: "unavailable", format, reason: NO_CLIP };
    const fps = options.fps ?? DEFAULT_FPS;
    const count = Math.max(1, Math.round((durationS ?? 0) * fps));
    const scenes = await frameScenes(asset, query, options, count, fps, control, label("Evaluating frames", "Bilder werden ausgewertet"));
    const first = scenes[0]!;
    const window = frameOf(scenes);
    const video = format === "mp4" || format === "webm";
    const ppm = options.pixelsPerMm ?? 96 / 25.4 * 2;
    const size = rasterSize(window, ppm, video ? VIDEO_SIDE_PX : MAX_SIDE_PX);
    const even = video ? { widthPx: size.widthPx + (size.widthPx % 2), heightPx: size.heightPx + (size.heightPx % 2) } : size;

    if (format === "png-sequence") {
      const text = metadataText(first);
      const entries: { name: string; bytes: Uint8Array }[] = [];
      const base = exportFilename(asset, query, options, format).replace(/\.zip$/u, "");
      const width = String(count).length;
      for await (const frame of renderFrames(scenes, window, even, rasterise, control, label("Rendering frames", "Bilder werden gerendert"), false))
        entries.push({ name: `${base}_${String(frame.index + 1).padStart(Math.max(width, 4), "0")}.png`, bytes: withPngText(frame.png, { ...text, Comment: JSON.stringify({ ...JSON.parse(text.Comment!), frame: frame.index + 1, fps, timeS: frame.index / fps }) }) });
      const info = { ...JSON.parse(text.Comment!), frames: count, fps, widthPx: even.widthPx, heightPx: even.heightPx, alpha: first.background === null };
      entries.push({ name: "metadata.json", bytes: encoder.encode(JSON.stringify(info, null, 2)) });
      return done(format, exportFilename(asset, query, options, format), writeZip(entries, text.Description), { frames: count, fps, widthPx: even.widthPx, heightPx: even.heightPx, alpha: first.background === null });
    }

    const record = host?.record ?? (recorderMimes(format).length > 0 ? browserRecorder : null);
    const mime = recorderMimes(format)[0] ?? `video/${format}`;
    if (record === null) return { status: "unavailable", format, reason: NO_RECORDER(format === "mp4" ? "MP4" : "WebM") };
    const rendered: Uint8Array[] = [];
    for await (const frame of renderFrames(scenes, window, even, rasterise, control, label("Rendering frames", "Bilder werden gerendert"), true)) rendered.push(frame.png);
    const frames = (async function* () {
      yield* rendered;
    })();
    const recorded = await record(frames, even, fps, mime, control?.signal);
    const notes: Text[] = [label("Video containers from MediaRecorder carry no descriptive metadata; it is in the file name.", "Videocontainer des MediaRecorders tragen keine beschreibenden Metadaten; sie stehen im Dateinamen.")];
    if (first.background === null) notes.push(label("Alpha is not preserved: transparent frames are flattened onto white.", "Alpha bleibt nicht erhalten: transparente Bilder werden auf Weiß abgeflacht."));
    return done(format, exportFilename(asset, query, options, format), recorded.bytes, { frames: count, fps, widthPx: even.widthPx, heightPx: even.heightPx, codec: recorded.mime, alpha: false, metadataEmbedded: false, notes });
  } catch (error) {
    if (isAbort(error)) return { status: "cancelled", format };
    if ((error instanceof Error && error.name === "ModelRequestError" && "reason" in error)) return { status: "unavailable", format, reason: (error as Error & { reason: Text }).reason };
    throw error;
  }
}

/** 📁️ One 3D download: the pose of a GLB, the clip of an FBX, the pose or clip of a model JSON (each named by the options, else by the current query, else the asset's default), or the bundle of every model; made by the lazily loaded pipeline. */
async function exportSpatialFigure(asset: Asset, query: EvaluationQuery, options: ExportOptions, format: SpatialExportFormat, control: JobControl | undefined): Promise<ExportResult> {
  const { exportSpatial } = await import("../🧊️spatial/📤️exports/🟦️.ts");
  const pose = options.pose ?? (query.kind === "pose" ? query.pose : asset.presentation.defaultPose);
  const clip = options.clip ?? (query.kind === "clip" ? query.clip : asset.presentation.defaultClip);
  const request = { scale: options.scale.denominator, view: options.view, ...(options.environment === undefined ? {} : { environment: options.environment }), ...(options.thermalQuantity === undefined ? {} : { thermalQuantity: options.thermalQuantity }), ...(options.forceQuantity === undefined ? {} : { forceQuantity: options.forceQuantity }), ...(options.range === undefined ? {} : { range: options.range }), ...(options.theme === undefined ? {} : { theme: options.theme }) };
  const which = format === "glb" ? { pose } : format === "fbx" ? { clip } : format === "model" ? (options.clip !== undefined || (options.pose === undefined && query.kind === "clip") ? { clip } : { pose }) : {};
  const made = await exportSpatial(asset, format, { ...request, ...which }, { ...(control?.signal === undefined ? {} : { signal: control.signal }), progress: (d, t, l) => control?.progress?.(Math.round((d / Math.max(t, 1)) * 100), 100, l) });
  return { status: "done", format, filename: made.filename, mime: made.mime, bytes: made.bytes, blob: new Blob([made.bytes as BlobPart], { type: made.mime }), details: { metadataEmbedded: format !== "fbx", notes: made.notes } };
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
