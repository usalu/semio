/** 🖨️ Preview CLI v2 for content authors (Node-only): renders one asset pose or clip time at an authored scale in any of the seven representations to SVG and PNG, a contact sheet of all poses × representations,
 * or a print-size proof at 1:1 paper millimetres with a 10 mm calibration bar.
 *
 * `repo tools exec bun -- <this file> --asset <id> [--catalog <root> | --file <asset json>] [--scale 100] [--pose id | --clip id --time s | --symbol id] [--view v] (--viewpoint elevation|front|plan|axonometric [--camera isometric|dimetric|<azimuth>,<elevation>] | --views[ elevation,front,plan,axonometric] --projection first|third) [--sheet] [--print-size] [--profile id] [--language en|de] [--dpi 300] [--no-png] [--transport parcels,pathlines,streamlines,condensation] --out <dir>`
 *
 * `--viewpoint` draws the side elevation (`elevation`), the front elevation (`front`), the plan (top view, `plan`) or the axonometric view of the 3D volume (`axonometric`, `--camera isometric|dimetric|<azimuth>,<elevation>`); `--views --projection first|third` writes one sheet per representation in the D7 arrangement of `📤️export/📑️sheet` (plan above or below the side elevation, front beside it) with the three views (side elevation, front elevation, plan) of the same pose or clip instant at the same scale.
 *
 * `--print-size` renders at `--dpi` (default 600) with the scene at its true paper size, so a printout at 100 % measures as drawn; the calibration bar is exactly 10 mm. An unavailable or unsuitable
 * scale writes an explanation card instead of a drawing and exits with code 1. `rsvg-convert` is not a registered repo tool, so the system binary is used behind the {@link Rasterizer} and the owned `ToolPort`; its absence is a named, bilingual `ToolMissingError`.
 *
 * @see ../🟦️.ts — `depict`
 * @see ../../📤️export/🖼️svg/🟦️.ts — `writeSvg`, `svgMarkup`
 * @see ../../🖨️representation/🟦️.ts — `resolveRepresentation`
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { PLANAR_VIEWPOINTS, VIEWPOINTS, type Asset, type AssetDocument, type Language, type Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { VIEWS, type AxonometricSpec, type DepictionOptions, type DrawingScene, type EvaluationQuery, type RepresentationSelector, type View } from "../../🔗️contracts/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, PRINT_PROFILES, RepresentationError, resolveRepresentation } from "../../🖨️representation/🟦️.ts";
import { PROJECTIONS, resolveSheet, type Projection } from "../../📤️export/📑️sheet/🟦️.ts";
import { solveStructure } from "../../🏗️structure/🟦️.ts";
import { solveThermalAt } from "../../🌡️thermal/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from "../../🗂️catalog/📥️files/🟦️.ts";
import { resolveThermalEnvironment } from "../../🗂️catalog/🟦️.ts";
import { svgMarkup, writeSvg } from "../../📤️export/🖼️svg/🟦️.ts";
import { defaultDepictionOptions, depict, loadAxonometric } from "../🟦️.ts";
import { textWidthMm } from "../../📐️units/🟦️.ts";
import { ModelRequestError } from "../../🧊️volume/🚫️refusal/🟦️.ts";
import { explanationBox } from "../🪧️explanation/🟦️.ts";
import { repoToolPort, type ToolPort } from "../../📤️export/🔌️tools/🟦️.ts";

/** 🖼️ Turns an SVG file into a PNG at a resolution; rejects with the signal's reason when cancelled. */
export interface Rasterizer {
  rasterize(svgPath: string, pngPath: string, dpi: number, signal?: AbortSignal): Promise<void>;
}

/** 🌟️ The `rsvg-convert` implementation of {@link Rasterizer} behind a {@link ToolPort}: a missing program is a named, bilingual `ToolMissingError`, there is no second rasteriser to fall back on. */
export const rsvgRasterizer = (tools: ToolPort = repoToolPort): Rasterizer => ({
  async rasterize(svgPath, pngPath, dpi, signal) {
    await tools.run("rsvg-convert", ["-d", String(dpi), "-p", String(dpi), "-f", "png", "-o", pngPath, svgPath], signal);
  },
});

const PLANAR_NAMES = ["elevation", "front", "plan"] as const;
const FONT = "Helvetica, Arial, sans-serif";
const USAGE = "usage: --asset <id> [--catalog <root> | --file <asset json>] [--scale 100] [--pose id | --clip id --time s | --symbol id] [--view v] (--viewpoint elevation|front|plan|axonometric [--camera isometric|dimetric|<azimuth>,<elevation>] | --views[ elevation,front,plan,axonometric] --projection first|third) [--sheet] [--print-size] [--profile id] [--language en|de] [--dpi 300] [--no-png] [--transport parcels,pathlines,streamlines,condensation] --out <dir>";

type Args = { readonly flags: ReadonlyMap<string, string> };

function parse(argv: readonly string[]): Args {
  const flags = new Map<string, string>();
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]!;
    if (!a.startsWith("--")) continue;
    const next = argv[i + 1];
    if (next === undefined || next.startsWith("--")) flags.set(a.slice(2), "true");
    else {
      flags.set(a.slice(2), next);
      i++;
    }
  }
  return { flags };
}

const num = (value: number): string => (Math.round(value * 1e4) / 1e4).toString();

/** 📏️ A scene wrapped for printing at 100 %: the scene at its true paper size plus a labelled 10 mm calibration bar below it; the document size is in millimetres. */
export function printSized(scene: DrawingScene): string {
  const width = Math.max(scene.widthMm, 60);
  const height = scene.heightMm + 12;
  const y = scene.heightMm + 5;
  const bar = `<g id="calibration"><path d="M2 ${num(y)}H12M2 ${num(y - 1.5)}V${num(y + 1.5)}M12 ${num(y - 1.5)}V${num(y + 1.5)}" fill="none" stroke="#000" stroke-width="0.25"/><rect x="2" y="${num(y - 0.5)}" width="10" height="1" fill="#000"/><text x="14" y="${num(y + 0.7)}" font-family="${FONT}" font-size="2.2" fill="#000">10 mm — print at 100 % (no scaling)</text></g>`;
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" version="1.1" width="${num(width)}mm" height="${num(height)}mm" viewBox="0 0 ${num(width)} ${num(height)}"><rect width="${num(width)}" height="${num(height)}" fill="#ffffff"/><svg x="0" y="0" width="${num(scene.widthMm)}" height="${num(scene.heightMm)}" viewBox="0 0 ${num(scene.widthMm)} ${num(scene.heightMm)}">${svgMarkup(scene)}</svg>${bar}</svg>\n`;
}

/** ✍️ Breaks a caption into lines no wider than a cell, measured with the Helvetica metrics of the drawings; a single word wider than the cell stays on its own line. */
export function captionLines(caption: string, widthMm: number, sizeMm = CAPTION_MM): string[] {
  const lines: string[] = [];
  let line = "";
  for (const word of caption.split(/\s+/u)) {
    const next = line === "" ? word : `${line} ${word}`;
    if (line !== "" && textWidthMm(next, sizeMm) > widthMm) {
      lines.push(line);
      line = word;
    } else line = next;
  }
  return [...lines, line];
}

const CAPTION_MM = 2.5;

function sheet(rows: readonly { readonly label: string; readonly scenes: readonly DrawingScene[] }[]): string {
  const cellW = Math.max(...rows.flatMap((r) => r.scenes.map((s) => s.widthMm)));
  const cellH = Math.max(...rows.flatMap((r) => r.scenes.map((s) => s.heightMm)));
  const labelW = 30;
  const head = 8;
  const width = labelW + cellW * VIEWS.length;
  const height = head + cellH * rows.length;
  const cells = rows.map((row, r) => row.scenes.map((scene, c) => `<svg x="${labelW + c * cellW}" y="${head + r * cellH}" width="${scene.widthMm}" height="${scene.heightMm}" viewBox="0 0 ${scene.widthMm} ${scene.heightMm}">${svgMarkup(scene)}</svg>`).join("")).join("");
  const text = (x: number, y: number, t: string): string => `<text x="${x}" y="${y}" font-size="2.5" font-family="${FONT}" fill="#001117">${t}</text>`;
  const cols = VIEWS.map((v, c) => text(labelW + c * cellW + 2, 5, v)).join("");
  const labels = rows.map((row, r) => text(2, head + r * cellH + 6, row.label)).join("");
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" version="1.1" width="${width}mm" height="${height}mm" viewBox="0 0 ${width} ${height}"><rect width="${width}" height="${height}" fill="#ffffff"/>${cols}${labels}${cells}</svg>\n`;
}

async function loadAsset(flags: ReadonlyMap<string, string>): Promise<Asset> {
  const file = flags.get("file");
  if (file !== undefined) return (JSON.parse(readFileSync(resolve(file), "utf8")) as AssetDocument).asset;
  const catalog = await loadCatalogFromFiles(flags.get("catalog") ? resolve(flags.get("catalog")!) : DEFAULT_CATALOG_ROOT);
  const id = flags.get("asset");
  const asset = id ? catalog.assets.find((a) => a.id === id) : catalog.assets.length === 1 ? catalog.assets[0] : undefined;
  if (!asset) throw new Error(`asset not found; available: ${catalog.assets.map((a) => a.id).join(", ")}`);
  return asset;
}

/** ▶️ Runs the preview CLI with the given arguments; returns the written file paths. An unavailable or unsuitable scale writes an explanation card and sets `process.exitCode = 1`. */
export async function runPreview(argv: readonly string[], rasterizer: Rasterizer = rsvgRasterizer(), signal?: AbortSignal): Promise<string[]> {
  const { flags } = parse(argv);
  const out = flags.get("out");
  if (!out || (!flags.has("asset") && !flags.has("file"))) throw new Error(USAGE);
  const asset = await loadAsset(flags);
  if (flags.get("viewpoint") === "axonometric" || flags.has("views")) await loadAxonometric();
  const denominator = Number(flags.get("scale") ?? asset.presentation.defaultScale);
  const language = (flags.get("language") ?? "en") as Language;
  const printSize = flags.has("print-size");
  const dpi = Number(flags.get("dpi") ?? (printSize ? 600 : 300));
  const profile = flags.has("profile") ? PRINT_PROFILES.profiles.find((candidate) => candidate.id === flags.get("profile")) ?? (() => { throw new Error(`unknown profile "${flags.get("profile")}"`); })() : DEFAULT_PRINT_PROFILE;
  const png = !flags.has("no-png");
  const views = flags.has("view") ? [flags.get("view") as View] : [...VIEWS];
  const selector: RepresentationSelector = flags.has("symbol") ? { kind: "symbol", symbol: flags.get("symbol")! } : { kind: "scale", scale: { denominator } };
  mkdirSync(out, { recursive: true });
  const written: string[] = [];
  const emit = async (name: string, svg: string): Promise<void> => {
    const svgPath = join(out, `${name}.svg`);
    writeFileSync(svgPath, svg);
    written.push(svgPath);
    if (png) {
      const pngPath = join(out, `${name}.png`);
      await rasterizer.rasterize(svgPath, pngPath, dpi, signal);
      written.push(pngPath);
    }
  };
  const transport = (flags.get("transport") ?? "").split(",").filter((layer) => layer !== "") as DepictionOptions["transport"];
  const chosen = flags.get("viewpoint");
  if (chosen === undefined && !flags.has("views")) throw new Error(`--viewpoint elevation|front|plan is required (or --views for all three)
${USAGE}`);
  if (chosen !== undefined && !(VIEWPOINTS as readonly string[]).includes(chosen)) throw new Error(`unknown viewpoint "${chosen}"; one of ${VIEWPOINTS.join(", ")}`);
  const cameraText = flags.get("camera");
  const camera: AxonometricSpec | undefined = cameraText === undefined ? undefined : cameraText === "isometric" || cameraText === "dimetric" ? { kind: cameraText } : (() => { const [azimuthDeg, elevationDeg] = cameraText.split(",").map(Number); if (azimuthDeg === undefined || elevationDeg === undefined || !Number.isFinite(azimuthDeg) || !Number.isFinite(elevationDeg)) throw new ModelRequestError("camera-format", cameraText); return { kind: "free", azimuthDeg, elevationDeg } as const; })();
  const viewpoint = (chosen ?? "elevation") as Viewpoint;
  const presets = new Map(((await loadEnvironments(flags.get("catalog") ? resolve(flags.get("catalog")!) : DEFAULT_CATALOG_ROOT))?.presets ?? []).map((preset) => [preset.id, preset]));
  const environment = resolveThermalEnvironment(asset, presets);
  const options = (view: View, legend: boolean, at: Viewpoint = viewpoint): DepictionOptions => {
    const o = defaultDepictionOptions(view, language, denominator, at);
    return { ...o, ...(camera ? { axonometric: camera } : {}), transport, profile, representation: selector, ...(environment === undefined ? {} : { environment }), annotations: { ...o.annotations, legend } };
  };
  const scenesFor = (query: EvaluationQuery, which: readonly View[], legend: boolean): DrawingScene[] => {
    const state = evaluate(asset, query, selector);
    const analysis = { thermal: solveThermalAt(state.asset, query, environment), structure: solveStructure(state) };
    return which.map((view) => depict(state, options(view, legend), analysis));
  };
  const viewpointScenes = (query: EvaluationQuery, view: View): DrawingScene[] => {
    const state = evaluate(asset, query, selector);
    const analysis = { thermal: solveThermalAt(state.asset, query, environment), structure: solveStructure(state) };
    return PLANAR_VIEWPOINTS.map((at) => depict(state, options(view, false, at), analysis));
  };
  const tag = flags.has("symbol") ? `symbol-${flags.get("symbol")}` : `1-${denominator}`;
  try {
    if (selector.kind === "scale") {
      const resolution = resolveRepresentation(asset, selector.scale, profile);
      if (resolution.kind === "unavailable" || resolution.kind === "unsuitable") throw new RepresentationError(resolution, asset.id);
    }
    if (flags.has("sheet")) {
      const rows = asset.poses.filter((pose) => !pose.key).map((pose) => ({ label: pose.id, scenes: scenesFor({ kind: "pose", pose: pose.id }, VIEWS, false) }));
      await emit(`${asset.id}--sheet-${viewpoint}-${tag}`, sheet(rows));
      return written;
    }
    const query: EvaluationQuery = flags.has("clip") ? { kind: "clip", clip: flags.get("clip")!, timeS: Number(flags.get("time") ?? 0) } : { kind: "pose", pose: flags.get("pose") ?? asset.presentation.defaultPose };
    const when = query.kind === "clip" ? `${query.clip}@${query.timeS}` : query.kind === "pose" ? query.pose : "transition";
    if (flags.has("views")) {
      const projection = flags.get("projection") as Projection | undefined;
      if (projection === undefined || !(PROJECTIONS as readonly string[]).includes(projection)) throw new Error(`--views needs --projection ${PROJECTIONS.join("|")}\n${USAGE}`);
      const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...drawOptions } = options("dark-line", true, "elevation");
      const source = selector.kind === "symbol" ? ({ kind: "symbol", symbol: selector.symbol, denominator } as const) : ({ kind: "scale", denominator } as const);
      const listed = flags.get("views")!;
      const names = listed === "true" ? [...PLANAR_NAMES] : listed.split(",");
      if (names.some((name) => !(VIEWPOINTS as readonly string[]).includes(name)) || !PLANAR_NAMES.every((name) => names.includes(name))) throw new ModelRequestError("views-bad", listed);
      const axonometric = names.includes("axonometric") ? (camera ?? ({ kind: "isometric" } as const)) : false;
      for (const view of flags.has("view") ? views : (["dark-line"] as const)) await emit(`${asset.id}--${when}--views${axonometric === false ? "" : "-axonometric"}-${view}-${projection}-${tag}`, writeSvg(resolveSheet(asset, query, source, view, drawOptions as never, projection, language, axonometric).sheet));
      return written;
    }
    for (const [i, scene] of scenesFor(query, views, true).entries()) await emit(`${asset.id}--${when}--${views[i]}-${viewpoint}-${tag}${printSize ? "-print" : ""}`, printSize ? printSized(scene) : writeSvg(scene));
  } catch (error) {
    if (!(error instanceof RepresentationError)) throw error;
    const box = explanationBox(asset, error.resolution, language);
    await emit(`${asset.id}--${tag}--explanation`, `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="${box.widthMm}mm" height="${box.heightMm}mm" viewBox="0 0 ${box.widthMm} ${box.heightMm}">${box.markup}</svg>\n`);
    process.exitCode = 1;
  }
  return written;
}

if (import.meta.main) {
  for (const path of await runPreview(process.argv.slice(2))) console.log(path);
}
