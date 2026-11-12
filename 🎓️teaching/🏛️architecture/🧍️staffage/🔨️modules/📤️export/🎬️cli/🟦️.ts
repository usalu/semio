/** 🎬️ Node command line of the archive exports:
 * `repo tools exec bun -- 🟦️.ts --asset slug --clip slug [--time s]|--pose slug --view dark-line --viewpoint elevation|front|plan|axonometric [--camera isometric|dimetric|<azimuth>,<elevation>] --scale 100 --fps 25 --format svg|pdf|dxf|dwg|png-sequence|mp4|webm --lang en|de --out dir`
 * and the aligned multiview sheet `--format sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg --projection first|third [--views elevation,front,plan,axonometric]` (plan, front elevation and side elevation of one evaluated state, no `--viewpoint`; the optional fourth cell is the axonometric view in the free quadrant).
 * The axonometric viewpoint draws the 3D volume of the asset (see `🎨️depiction/🧊️axonometric`) through `--camera`: `isometric` (default), `dimetric` (ISO 5456-3, 1 : ½ : 1) or `<azimuth>,<elevation>` in degrees; the vertical axis is always true to scale.
 *
 * Every export names its viewpoint (or the sheet projection), representation (`--view`), scale and pose or clip instant; nothing defaults and no file name omits them, so the exports of two viewpoints never overwrite each other.
 * `--time` writes the single instant of a clip instead of its frame schedule.
 *
 * One frame schedule feeds every format. SVG, DXF and DWG write one file per frame, PDF one page per frame. Raster formats render each SVG
 * frame with `rsvg-convert` onto one fixed canvas (all frames aligned on the world origin, so nothing jumps), then pack a PNG ZIP or encode
 * with `ffmpeg`: MP4 is libx264 yuv420p (no alpha, opaque background), WebM is libvpx-vp9 yuva420p (alpha), both at the exact constant fps.
 * The thermal view solves under `--environment <preset>` if given, else the asset's `thermal.referenceEnvironment`, else its `thermal.environment`.
 * External programs are reached only through {@link ToolPort}. Progress goes to stderr; SIGINT cancels and kills the running child.
 * <https://gitlab.gnome.org/GNOME/librsvg> · <https://ffmpeg.org/ffmpeg.html> */
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { VIEWS, type AxonometricSpec, type DepictionOptions, type DrawingScene, type EvaluationQuery, type JobControl, type View } from "../../🔗️contracts/🟦️.ts";
import { LANGUAGES, PLANAR_VIEWPOINTS, VIEWPOINTS, type Asset, type Language, type Slug, type Text, type Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { loadCatalogFromFiles, loadEnvironments, DEFAULT_CATALOG_ROOT } from "../../🗂️catalog/📥️files/🟦️.ts";
import { resolveThermalEnvironment } from "../../🗂️catalog/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { axonometricCamera } from "../../🎨️depiction/🔭️views/🟦️.ts";
import { defaultDepictionOptions, depict, loadAxonometric, modelScene, type Analysis } from "../../🎨️depiction/🟦️.ts";
import { solveStructure } from "../../🏗️structure/🟦️.ts";
import { solveThermalAt } from "../../🌡️thermal/🟦️.ts";
import { writeSvg } from "../🖼️svg/🟦️.ts";
import { writePdf } from "../📕️pdf/🟦️.ts";
import { writeDxf } from "../📐️dxf/🟦️.ts";
import { configureDwg, DwgUnavailableError, writeDwg } from "../🖊️dwg/🟦️.ts";
import { frameName, frameSchedule, writePngSequenceZip, type FrameSchedule } from "../🎞️frames/🟦️.ts";
import { PROJECTIONS, modelSheet, resolveSheet, withNote, type Projection } from "../📑️sheet/🟦️.ts";
import { ModelRequestError } from "../../🧊️volume/🟦️.ts";
import { writeGlb } from "../🧊️glb/🟦️.ts";
import { writeFbx } from "../🎬️fbx/🟦️.ts";
import { modelExtension, modelStem, writeModel } from "../🗿️model/🟦️.ts";
import { writeModelBundle } from "../🗜️zip/📦️models/🟦️.ts";
import { repoToolPort, type ToolPort } from "../🔌️tools/🟦️.ts";


//#region 🔖️Options
/** 📄️ The formats. */
export const CLI_FORMATS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "sheet-svg", "sheet-pdf", "sheet-dxf", "sheet-dwg", "glb", "fbx", "model", "model-zip"] as const;
/** 🧊️ The formats of the 3D model pipeline: a GLB per static pose, an FBX per clip, the neutral JSON model of a pose or clip, and the ZIP bundle of all of them. */
export const MODEL_FORMATS = ["glb", "fbx", "model", "model-zip"] as const;
/** 📤️ One of {@link MODEL_FORMATS}. */
export type ModelFormat = (typeof MODEL_FORMATS)[number];
/** 📦️ Whether a CLI format belongs to the 3D model pipeline. */
export const isModelFormat = (format: CliFormat): format is ModelFormat => (MODEL_FORMATS as readonly string[]).includes(format);
/** 🗂️ One of {@link CLI_FORMATS}. */
export type CliFormat = (typeof CLI_FORMATS)[number];

/** ⚙️ The parsed command line. */
export type CliOptions = { readonly asset: string; readonly clip?: string; readonly pose?: string; readonly view: View; readonly viewpoint?: Viewpoint; readonly projection?: Projection; readonly axonometric?: AxonometricSpec; readonly sheetAxonometric?: boolean; readonly time?: number; readonly environment?: Slug; readonly scale: number; readonly fps?: number; readonly format: CliFormat; readonly out: string; readonly catalog: string; readonly language: Language; readonly pxPerMm: number; readonly background: string; readonly spanK?: number };

const SYNTAX = "--asset <slug> (--clip <slug> | --pose <slug>) --view <view> --scale <denominator> [--fps <n>] --format <svg|pdf|dxf|dwg|png-sequence|mp4|webm|sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg|glb|fbx|model|model-zip> --out <dir> --lang <en|de> (--viewpoint elevation|front|plan|axonometric [--camera isometric|dimetric|<azimuth>,<elevation>] | --projection first|third [--views elevation,front,plan,axonometric]) [--time <s>] [--environment <preset>] [--catalog <dir>] [--px-per-mm <n>] [--background #rrggbb]";
const USAGE: Text = { en: `usage: ${SYNTAX}`, de: `Aufruf: ${SYNTAX}` };

const SYNTAX_MODEL = "--asset <slug> --scale <denominator> --lang <en|de> --out <dir> --format glb --pose <slug> | --format fbx --clip <slug> [--view <view>] | --format model (--pose <slug> | --clip <slug>) | --format model-zip, each with [--environment <preset>] [--span-k <kelvin>] [--catalog <dir>]";
const USAGE_MODEL: Text = { en: `3D models: ${SYNTAX_MODEL}`, de: `3D-Modelle: ${SYNTAX_MODEL}` };

/** 🚫️ Thrown for a malformed command line: the message carries English then German, `reason` both. */
export class CliUsageError extends Error {
  constructor(readonly reason: Text) {
    super(`${reason.en}\n${reason.de}`);
    this.name = "CliUsageError";
  }
}

const refuse = (en: string, de: string, usage: Text = USAGE): CliUsageError => new CliUsageError({ en: `${en}\n${usage.en}`, de: `${de}\n${usage.de}` });

/** 🔧️ Parses `--key value` arguments; every refusal names the offending flag. */
export function parseArguments(argv: readonly string[]): CliOptions {
  const values = new Map<string, string>();
  for (let index = 0; index < argv.length; index += 2) {
    const key = argv[index];
    const value = argv[index + 1];
    if (!key?.startsWith("--") || value === undefined) throw refuse(`bad argument ${key ?? ""}`, `ungültiges Argument ${key ?? ""}`);
    values.set(key.slice(2), value);
  }
  const need = (key: string): string => values.get(key) ?? (() => { throw refuse(`missing --${key}`, `--${key} fehlt`); })();
  const oneOf = <T extends string>(key: string, allowed: readonly T[]): T => { const value = need(key); if (!(allowed as readonly string[]).includes(value)) throw refuse(`--${key} must be one of ${allowed.join(", ")}, got ${value}`, `--${key} muss eines von ${allowed.join(", ")} sein, erhalten ${value}`); return value as T; };
  const positive = (key: string, fallback?: number): number => { const raw = values.get(key); const value = raw === undefined ? fallback : Number(raw); if (value === undefined || !(value > 0) || !Number.isFinite(value)) throw refuse(`--${key} must be a positive number`, `--${key} muss eine positive Zahl sein`); return value; };
  const nonNegative = (key: string): number => { const value = Number(need(key)); if (!(value >= 0) || !Number.isFinite(value)) throw refuse(`--${key} must be a number of seconds >= 0`, `--${key} muss eine Zahl von Sekunden >= 0 sein`); return value; };
  const clip = values.get("clip");
  const pose = values.get("pose");
  const asked = values.get("format");
  if (asked !== undefined && (MODEL_FORMATS as readonly string[]).includes(asked)) {
    const format = asked as ModelFormat;
    if (values.has("viewpoint") || values.has("projection")) throw new ModelRequestError("model-no-viewpoint");
    if (values.has("time") || values.has("fps")) throw new ModelRequestError("model-no-time");
    if (format === "glb" && (pose === undefined || clip !== undefined)) throw new ModelRequestError("glb-needs-pose");
    if (format === "fbx" && (clip === undefined || pose !== undefined)) throw new ModelRequestError("fbx-needs-clip");
    if (format === "model" && (clip === undefined) === (pose === undefined)) throw new ModelRequestError("model-needs-one");
    if (format === "model-zip" && (clip !== undefined || pose !== undefined)) throw new ModelRequestError("bundle-takes-neither");
    if (format !== "fbx" && values.has("view")) throw new ModelRequestError("model-all-views");
    if (format === "fbx" && values.has("environment") && values.get("view") !== "thermal") throw new ModelRequestError("model-environment-view");
    return { asset: need("asset"), ...(clip === undefined ? {} : { clip }), ...(pose === undefined ? {} : { pose }), view: values.has("view") ? oneOf("view", VIEWS) : "light-solid", ...(values.has("environment") ? { environment: values.get("environment")! as Slug } : {}), ...(values.has("span-k") ? { spanK: positive("span-k") } : {}), scale: positive("scale"), format, out: resolve(need("out")), catalog: resolve(values.get("catalog") ?? DEFAULT_CATALOG_ROOT), language: oneOf("lang", LANGUAGES), pxPerMm: positive("px-per-mm", 8), background: values.get("background") ?? "#ffffff" };
  }
  if (values.has("span-k")) throw refuse("--span-k applies to the 3D model formats only", "--span-k gilt nur für die 3D-Modellformate", USAGE_MODEL);
  if ((clip === undefined) === (pose === undefined)) throw refuse("give exactly one of --clip and --pose", "genau eines von --clip und --pose angeben");
  if (values.has("environment") && values.get("view") !== "thermal") throw refuse("--environment applies to --view thermal only", "--environment gilt nur für --view thermal");
  const fps = values.get("fps");
  const format = oneOf("format", CLI_FORMATS);
  const sheet = format.startsWith("sheet-");
  if (sheet && values.has("viewpoint")) throw refuse(`--viewpoint does not apply to ${format}, a sheet draws all three views`, `--viewpoint gilt nicht für ${format}, ein Blatt zeigt alle drei Ansichten`);
  if (!sheet && values.has("projection")) throw refuse("--projection applies to the sheet formats only", "--projection gilt nur für die Blattformate");
  if (!sheet && values.has("views")) throw refuse("--views applies to the sheet formats only", "--views gilt nur für die Blattformate");
  const sheetAxonometric = sheet && values.has("views") ? parseSheetViews(need("views")) : false;
  const drawsAxonometric = sheet ? sheetAxonometric : values.get("viewpoint") === "axonometric";
  if (values.has("camera") && !drawsAxonometric) throw refuse("--camera applies to the axonometric view only: --viewpoint axonometric, or a sheet with --views elevation,front,plan,axonometric", "--camera gilt nur für die axonometrische Ansicht: --viewpoint axonometric oder ein Blatt mit --views elevation,front,plan,axonometric");
  if (values.has("time") && clip === undefined) throw refuse("--time needs --clip", "--time braucht --clip");
  if (values.has("time") && (fps !== undefined || !["svg", "pdf", "dxf", "dwg", "sheet-svg", "sheet-pdf", "sheet-dxf", "sheet-dwg"].includes(format))) throw refuse("--time writes one instant as a still format and excludes --fps", "--time schreibt einen Zeitpunkt als Standbildformat und schliesst --fps aus");
  return { asset: need("asset"), ...(clip === undefined ? {} : { clip }), ...(pose === undefined ? {} : { pose }), view: oneOf("view", VIEWS), ...(values.has("environment") ? { environment: values.get("environment")! as Slug } : {}), ...(sheet ? { projection: oneOf("projection", PROJECTIONS), ...(sheetAxonometric ? { sheetAxonometric: true } : {}) } : { viewpoint: oneOf("viewpoint", VIEWPOINTS) }), ...(values.has("camera") ? { axonometric: parseCamera(need("camera")) } : {}), ...(values.has("time") ? { time: nonNegative("time") } : {}), scale: positive("scale"), ...(fps === undefined ? {} : { fps: positive("fps") }), format, out: resolve(need("out")), catalog: resolve(values.get("catalog") ?? DEFAULT_CATALOG_ROOT), language: oneOf("lang", LANGUAGES), pxPerMm: positive("px-per-mm", 8), background: values.get("background") ?? "#ffffff" };
}

/** 🦄 The camera of `--camera`: `isometric`, `dimetric` or `<azimuth>,<elevation>` in degrees (azimuth −180…180 from ahead towards the asset's right, elevation 0…75). */
export function parseCamera(text: string): AxonometricSpec {
  if (text === "isometric" || text === "dimetric") return { kind: text };
  const [azimuth, elevation, ...rest] = text.split(",");
  const azimuthDeg = Number(azimuth);
  const elevationDeg = Number(elevation);
  if (azimuth === undefined || elevation === undefined || rest.length > 0 || azimuth.trim() === "" || elevation.trim() === "" || !Number.isFinite(azimuthDeg) || !Number.isFinite(elevationDeg)) throw refuse(`--camera must be isometric, dimetric or <azimuth>,<elevation> in degrees, got ${text}`, `--camera muss isometric, dimetric oder <Azimut>,<Elevation> in Grad sein, erhalten ${text}`);
  axonometricCamera({ kind: "free", azimuthDeg, elevationDeg });
  return { kind: "free", azimuthDeg, elevationDeg };
}

/** 🏷️ The file-name part of a camera, so two cameras never write the same file: empty for the default isometric one, `-dimetric`, or `-az<azimuth>-el<elevation>`. */
export function cameraTag(spec: AxonometricSpec | undefined): string {
  if (spec === undefined || spec.kind === "isometric") return "";
  return spec.kind === "dimetric" ? "-dimetric" : `-az${spec.azimuthDeg}-el${spec.elevationDeg}`;
}

/** 📑️ The cells of `--views`: the three planar views with or without the axonometric one (in any order, each once); returns whether the axonometric cell is asked for. */
export function parseSheetViews(text: string): boolean {
  const names = text.split(",");
  const allowed: readonly string[] = VIEWPOINTS;
  if (names.some((name) => !allowed.includes(name)) || new Set(names).size !== names.length || !PLANAR_VIEWPOINTS.every((viewpoint) => names.includes(viewpoint))) throw refuse(`--views must name elevation, front and plan, and may add axonometric, got ${text}`, `--views muss elevation, front und plan nennen und darf axonometric ergänzen, erhalten ${text}`);
  return names.includes("axonometric");
}
//#endregion 🔖️Options

//#region 🔖️Pipeline
const catalogs = new Map<string, ReturnType<typeof loadCatalogFromFiles>>();

/** 🧪️ The catalog of a folder, read once per process: a command that exports several files (or a test run of many commands) does not parse and validate the 149 assets again for each; a failed or cancelled read is forgotten. */
function catalogOf(root: string, signal: AbortSignal | undefined): ReturnType<typeof loadCatalogFromFiles> {
  let known = catalogs.get(root);
  if (known === undefined) {
    known = loadCatalogFromFiles(root, signal);
    catalogs.set(root, known);
    known.catch(() => catalogs.delete(root));
  }
  return known;
}

const PROGRESS = { en: "Exporting", de: "Exportiere" } as const;
const even = (value: number): number => Math.ceil(value / 2) * 2;

/** 🗓️ The queries of the command: one per scheduled time (a pose is one frame at t = 0), with the schedule that times them. */
export function frameQueries(asset: Asset, options: CliOptions): { readonly schedule: FrameSchedule; readonly queries: readonly EvaluationQuery[] } {
  if (options.pose !== undefined) {
    const schedule: FrameSchedule = { fps: options.fps ?? 1, durationS: 0, loop: false, count: 1, times: [0] };
    return { schedule, queries: [{ kind: "pose", pose: options.pose }] };
  }
  const clip = asset.clips.find((candidate) => candidate.id === options.clip);
  if (!clip) throw new Error(`asset ${asset.id} has no clip ${options.clip}; clips: ${asset.clips.map((candidate) => candidate.id).join(", ")}`);
  const schedule = options.time === undefined ? frameSchedule(clip, options.fps ?? clip.fps) : ({ fps: 1, durationS: clip.durationS, loop: clip.loop, count: 1, times: [options.time] } satisfies FrameSchedule);
  return { schedule, queries: schedule.times.map((timeS) => ({ kind: "clip", clip: clip.id, timeS }) as const) };
}

/** 🎯️ What the command draws: one viewpoint, or the sheet of all three in a projection (with the axonometric cell when asked); exactly one is set by {@link parseArguments}. */
export function targetOf(options: CliOptions): { readonly kind: "view"; readonly viewpoint: Viewpoint } | { readonly kind: "sheet"; readonly projection: Projection; readonly axonometric: boolean } {
  if (options.projection !== undefined) return { kind: "sheet", projection: options.projection, axonometric: options.sheetAxonometric === true };
  if (options.viewpoint !== undefined) return { kind: "view", viewpoint: options.viewpoint };
  throw new Error("name --viewpoint or --projection");
}

type Canvas = { readonly widthPx: number; readonly heightPx: number; readonly offsets: readonly { readonly left: number; readonly top: number }[] };

/** 🧱️ One fixed pixel canvas for all scenes, aligned on the world origin of each scene. */
export function fixedCanvas(scenes: readonly DrawingScene[], pxPerMm: number): Canvas {
  const left = Math.max(...scenes.map((scene) => scene.metadata.origin.xMm));
  const top = Math.max(...scenes.map((scene) => scene.metadata.origin.yMm));
  const right = Math.max(...scenes.map((scene) => scene.widthMm - scene.metadata.origin.xMm));
  const bottom = Math.max(...scenes.map((scene) => scene.heightMm - scene.metadata.origin.yMm));
  return { widthPx: even((left + right) * pxPerMm), heightPx: even((top + bottom) * pxPerMm), offsets: scenes.map((scene) => ({ left: Math.round((left - scene.metadata.origin.xMm) * pxPerMm), top: Math.round((top - scene.metadata.origin.yMm) * pxPerMm) })) };
}

async function rasterise(tools: ToolPort, scenes: readonly DrawingScene[], directory: string, options: CliOptions, opaque: boolean, control: JobControl): Promise<Canvas> {
  const canvas = fixedCanvas(scenes, options.pxPerMm);
  await mkdir(directory, { recursive: true });
  for (const [index, scene] of scenes.entries()) {
    control.signal?.throwIfAborted();
    const svg = join(directory, frameName(index).replace(".png", ".svg"));
    await writeFile(svg, writeSvg(scene));
    const offset = canvas.offsets[index]!;
    await tools.run("rsvg-convert", ["-d", String(25.4 * options.pxPerMm), "-p", String(25.4 * options.pxPerMm), "--page-width", String(canvas.widthPx), "--page-height", String(canvas.heightPx), "--left", String(offset.left), "--top", String(offset.top), ...(opaque ? ["-b", options.background] : []), "-o", join(directory, frameName(index)), svg], control.signal);
    await rm(svg);
    control.progress?.(index + 1, scenes.length, { en: "Rasterising frames", de: "Bilder werden gerastert" });
  }
  return canvas;
}

/** 🌟️ Runs one 3D model command: a GLB of a static pose, an FBX of a clip, the neutral JSON of a pose or clip, or the ZIP of all of them; the thermal variant solves under `--environment`, else the asset's reference environment, else its own, with `--span-k` as the thermal span. Returns the files written. */
export async function runModelExport(options: CliOptions, format: ModelFormat, asset: Asset, control: JobControl = {}): Promise<readonly string[]> {
  const presets = new Map(((await loadEnvironments(options.catalog, control.signal))?.presets ?? []).map((preset) => [preset.id, preset]));
  const environment = resolveThermalEnvironment(asset, presets, options.environment);
  const shared = { scale: options.scale, ...(environment === undefined ? {} : { environment }), ...(options.spanK === undefined ? {} : { thermalSpanK: options.spanK }) };
  await mkdir(options.out, { recursive: true });
  const [name, extension, bytes] = await (async (): Promise<readonly [string, string, string | Uint8Array]> => {
    switch (format) {
      case "glb":
        return [modelStem(options.asset, options.pose!, "all", options.scale), "glb", await writeGlb(asset, { ...shared, pose: options.pose! }, control)];
      case "fbx":
        return [modelStem(options.asset, options.clip!, options.view, options.scale), "fbx", await writeFbx(asset, { ...shared, clip: options.clip!, view: options.view }, control)];
      case "model":
        return [modelStem(options.asset, options.clip ?? options.pose!, "all", options.scale), modelExtension(options.clip === undefined ? "pose" : "clip"), await writeModel(asset, { ...shared, ...(options.clip === undefined ? { pose: options.pose! } : { clip: options.clip }) }, control)];
      case "model-zip":
        return [`${options.asset}-all-model-1to${options.scale}`, "zip", await writeModelBundle(asset, { ...shared, view: options.view }, control)];
    }
  })();
  const path = join(options.out, `${name}.${extension}`);
  await writeFile(path, bytes);
  control.progress?.(1, 1, PROGRESS);
  return [path];
}

/** 🚀️ Runs one export command; returns the files written. */
export async function runExport(options: CliOptions, tools: ToolPort = repoToolPort, control: JobControl = {}): Promise<readonly string[]> {
  const catalog = await catalogOf(options.catalog, control.signal);
  const asset = catalog.assets.find((candidate) => candidate.id === options.asset);
  if (!asset) throw new Error(`asset ${options.asset} is not in the catalog ${options.catalog}`);
  if (isModelFormat(options.format)) return runModelExport(options, options.format, asset, control);
  const target = targetOf(options);
  if ((target.kind === "view" && target.viewpoint === "axonometric") || (target.kind === "sheet" && target.axonometric)) await loadAxonometric();
  const { schedule, queries } = frameQueries(asset, options);
  const presets = new Map(((options.view === "thermal" ? await loadEnvironments(options.catalog, control.signal) : null)?.presets ?? []).map((preset) => [preset.id, preset]));
  const environment = options.view === "thermal" ? resolveThermalEnvironment(asset, presets, options.environment) : undefined;
  const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...common } = defaultDepictionOptions(options.view, options.language, options.scale, "elevation");
  const drawing = { ...common, ...(environment === undefined ? {} : { environment }) };
  const source = { kind: "scale", denominator: options.scale } as const;
  const selector = { kind: "scale", scale: { denominator: options.scale } } as const;
  const transparentRaster = options.format === "png-sequence" || options.format === "webm";
  const frames = queries.map((query, index) => {
    control.signal?.throwIfAborted();
    control.progress?.(index + 1, queries.length, { en: "Evaluating frames", de: "Bilder werden ausgewertet" });
    if (target.kind === "sheet") {
      const resolved = resolveSheet(asset, query, source, options.view, drawing, target.projection, options.language, target.axonometric ? (options.axonometric ?? { kind: "isometric" }) : false);
      return { state: resolved.state, analysis: resolved.analysis, scene: resolved.sheet as DrawingScene };
    }
    const state = evaluate(asset, query, selector);
    const analysis: Analysis = options.view === "thermal" ? { thermal: solveThermalAt(state.asset, state.query, drawing.environmentTimeline ?? drawing.environment) } : options.view === "forces" || options.view === "moments" ? { structure: solveStructure(state) } : {};
    const depiction = { ...drawing, view: options.view, viewpoint: target.viewpoint, scale: { denominator: options.scale }, representation: selector, transparent: transparentRaster, ...(options.axonometric ? { axonometric: options.axonometric } : {}) } as DepictionOptions;
    return { state, analysis, scene: depict(state, depiction, analysis, control) };
  });
  const scenes = frames.map((frame) => frame.scene);
  await mkdir(options.out, { recursive: true });
  const where = target.kind === "sheet" ? `sheet-${target.projection}${target.axonometric ? `-axonometric${cameraTag(options.axonometric)}` : ""}` : `${target.viewpoint}${target.viewpoint === "axonometric" ? cameraTag(options.axonometric) : ""}`;
  const stem = `${options.asset}-${options.clip ?? options.pose}${options.time === undefined ? "" : `-t${options.time}`}-${options.view}-${where}-1to${options.scale}`;
  const diagrams = options.view === "forces" || options.view === "moments" ? (["axial", "shear", "moment"] as const).filter((kind) => common.diagrams[kind]) : [];
  const noteOf = (index: number): string => `staffage ${options.asset} ${options.clip ?? options.pose}${options.time === undefined ? "" : ` t=${options.time}`} ${options.view} ${where} 1:${options.scale} ${scenes[index]!.metadata.fidelity}`;
  const modelOf = (index: number) => {
    const { state, analysis } = frames[index]!;
    const model = target.kind === "sheet" ? modelSheet(state, { projection: target.projection, denominator: options.scale, language: options.language, analysis, diagrams, axonometric: target.axonometric ? (options.axonometric ?? { kind: "isometric" }) : false }) : modelScene(state, { viewpoint: target.viewpoint, analysis, diagrams, ...(options.axonometric ? { axonometric: options.axonometric } : {}) });
    return withNote(model, noteOf(index), options.scale);
  };
  const written: string[] = [];
  const emit = async (name: string, data: string | Uint8Array): Promise<void> => { control.signal?.throwIfAborted(); const path = join(options.out, name); await writeFile(path, data); written.push(path); };
  switch (options.format) {
    case "svg":
    case "sheet-svg":
      for (const [index, scene] of scenes.entries()) { control.signal?.throwIfAborted(); await emit(`${stem}-${frameName(index).replace(".png", ".svg")}`, writeSvg(scene)); }
      break;
    case "pdf":
    case "sheet-pdf":
      await emit(`${stem}.pdf`, writePdf(scenes, control));
      break;
    case "dxf":
    case "sheet-dxf":
      for (const index of scenes.keys()) { control.signal?.throwIfAborted(); await emit(`${stem}-${frameName(index).replace(".png", ".dxf")}`, writeDxf(modelOf(index), noteOf(index))); }
      break;
    case "dwg":
    case "sheet-dwg": {
      const cli = process.env.SEMIO_STAFFAGE_DWG_CLI;
      if (cli) configureDwg({ cli: async (json) => { const input = join(options.out, ".scene.json"); const output = join(options.out, ".scene.dwg"); await writeFile(input, json); await tools.run(cli, [input, output], control.signal); const bytes = new Uint8Array(await readFile(output)); await rm(input); await rm(output); return bytes; } });
      for (const index of scenes.keys()) { control.signal?.throwIfAborted(); await emit(`${stem}-${frameName(index).replace(".png", ".dwg")}`, await writeDwg(modelOf(index), control)); }
      break;
    }
    default: {
      const work = join(options.out, `.${stem}-frames`);
      try {
        const canvas = await rasterise(tools, scenes, work, options, options.format === "mp4", control);
        if (options.format === "png-sequence") {
          const frames = async function* (): AsyncGenerator<Uint8Array> { for (let index = 0; index < scenes.length; index += 1) yield new Uint8Array(await readFile(join(work, frameName(index)))); };
          await emit(`${stem}.zip`, await writePngSequenceZip(schedule, { asset: options.asset, clip: options.clip ?? options.pose!, view: options.view, viewpoint: target.kind === "view" ? target.viewpoint : (() => { throw new Error("a PNG sequence draws one viewpoint"); })(), scaleDenominator: options.scale, widthPx: canvas.widthPx, heightPx: canvas.heightPx, transparent: true }, frames(), control));
        } else {
          const target = join(options.out, `${stem}.${options.format}`);
          const input = ["-y", "-framerate", String(schedule.fps), "-i", join(work, "frame-%06d.png")];
          const codec = options.format === "mp4" ? ["-c:v", "libx264", "-pix_fmt", "yuv420p", "-movflags", "+faststart"] : ["-c:v", "libvpx-vp9", "-pix_fmt", "yuva420p", "-auto-alt-ref", "0", "-b:v", "0", "-crf", "30"];
          const tags = ["-metadata", `title=${options.asset} ${options.clip ?? options.pose}`, "-metadata", `comment=${noteOf(0)}`];
          await tools.run("ffmpeg", [...input, ...codec, ...tags, "-r", String(schedule.fps), "-fps_mode", "cfr", target], control.signal);
          written.push(target);
        }
      } finally {
        await rm(work, { recursive: true, force: true });
      }
    }
  }
  control.progress?.(1, 1, PROGRESS);
  return written;
}
//#endregion 🔖️Pipeline

//#region 🔖️Main
async function main(): Promise<void> {
  const controller = new AbortController();
  process.on("SIGINT", () => controller.abort());
  try {
    const options = parseArguments(process.argv.slice(2));
    const files = await runExport(options, repoToolPort, { signal: controller.signal, progress: (done, total, label) => process.stderr.write(`[${done}/${total}] ${label[options.language]}\n`) });
    for (const file of files) process.stdout.write(`${file}\n`);
  } catch (error) {
    process.stderr.write(`${(error as Error).message}\n`);
    process.exitCode = error instanceof DwgUnavailableError ? 3 : 1;
  }
}

if (process.argv[1] !== undefined && fileURLToPath(import.meta.url) === resolve(process.argv[1])) await main();
//#endregion 🔖️Main
