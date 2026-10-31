/** 🎬️ Node command line of the archive exports:
 * `repo tools exec bun -- 🟦️.ts --asset slug --clip slug|--pose slug --view dark-line --scale 100 --fps 25 --format svg|pdf|dxf|dwg|png-sequence|mp4|webm --out dir`.
 *
 * One frame schedule feeds every format. SVG, DXF and DWG write one file per frame, PDF one page per frame. Raster formats render each SVG
 * frame with `rsvg-convert` onto one fixed canvas (all frames aligned on the world origin, so nothing jumps), then pack a PNG ZIP or encode
 * with `ffmpeg`: MP4 is libx264 yuv420p (no alpha, opaque background), WebM is libvpx-vp9 yuva420p (alpha), both at the exact constant fps.
 * External programs are reached only through {@link ToolPort}. Progress goes to stderr; SIGINT cancels and kills the running child.
 * <https://gitlab.gnome.org/GNOME/librsvg> · <https://ffmpeg.org/ffmpeg.html> */
import { spawn } from "node:child_process";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { VIEWS, type DrawingScene, type EvaluatedState, type JobControl, type View } from "../../🔗️contracts/🟦️.ts";
import { LANGUAGES, VIEWPOINTS, type Asset, type Language, type Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict, modelScene } from "../../🎨️depiction/🟦️.ts";
import { writeSvg } from "../🖼️svg/🟦️.ts";
import { writePdf } from "../📕️pdf/🟦️.ts";
import { writeDxf } from "../📐️dxf/🟦️.ts";
import { configureDwg, DwgUnavailableError, writeDwg } from "../🖊️dwg/🟦️.ts";
import { frameName, frameSchedule, writePngSequenceZip, type FrameSchedule } from "../🎞️frames/🟦️.ts";

//#region 🔖️Tools
/** 🔌️ The tool-independent interface to external programs. */
export type ToolPort = { readonly run: (tool: string, args: readonly string[], signal?: AbortSignal) => Promise<string> };

const REPO_TOOLS = new Set(["ffmpeg"]);

/** 🔌️ Runs registered tools through `repo tools exec <tool> -- args` and system tools (`rsvg-convert`, `staffage-dwg`) from PATH. */
export const repoToolPort: ToolPort = {
  run(tool, args, signal) {
    const [command, argv] = REPO_TOOLS.has(tool) ? ["repo", ["tools", "exec", tool, "--", ...args]] : [tool, [...args]];
    return new Promise((done, fail) => {
      const child = spawn(command!, argv as string[], { stdio: ["ignore", "pipe", "pipe"], ...(signal ? { signal } : {}) });
      let output = "";
      child.stdout.on("data", (chunk: Buffer) => (output += chunk.toString("utf8")));
      child.stderr.on("data", (chunk: Buffer) => (output += chunk.toString("utf8")));
      child.on("error", fail);
      child.on("close", (code) => (code === 0 ? done(output) : fail(new Error(`${tool} exited ${code}\n${output}`))));
    });
  },
};
//#endregion 🔖️Tools

//#region 🔖️Options
/** 🎬️ The formats. */
export const CLI_FORMATS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm"] as const;
/** 🎬️ One of {@link CLI_FORMATS}. */
export type CliFormat = (typeof CLI_FORMATS)[number];

/** ⚙️ The parsed command line. */
export type CliOptions = { readonly asset: string; readonly clip?: string; readonly pose?: string; readonly view: View; readonly viewpoint: Viewpoint; readonly scale: number; readonly fps?: number; readonly format: CliFormat; readonly out: string; readonly catalog: string; readonly language: Language; readonly pxPerMm: number; readonly background: string };

const USAGE = "usage: --asset <slug> (--clip <slug> | --pose <slug>) --view <view> --scale <denominator> [--fps <n>] --format <svg|pdf|dxf|dwg|png-sequence|mp4|webm> --out <dir> --lang <en|de> [--viewpoint elevation|plan] [--catalog <dir>] [--px-per-mm <n>] [--background #rrggbb]";

/** ⚙️ Parses `--key value` arguments; every refusal names the offending flag. */
export function parseArguments(argv: readonly string[]): CliOptions {
  const values = new Map<string, string>();
  for (let index = 0; index < argv.length; index += 2) {
    const key = argv[index];
    const value = argv[index + 1];
    if (!key?.startsWith("--") || value === undefined) throw new Error(`bad argument ${key ?? ""}\n${USAGE}`);
    values.set(key.slice(2), value);
  }
  const need = (key: string): string => values.get(key) ?? (() => { throw new Error(`missing --${key}\n${USAGE}`); })();
  const oneOf = <T extends string>(key: string, allowed: readonly T[]): T => { const value = need(key); if (!(allowed as readonly string[]).includes(value)) throw new Error(`--${key} must be one of ${allowed.join(", ")}, got ${value}`); return value as T; };
  const positive = (key: string, fallback?: number): number => { const raw = values.get(key); const value = raw === undefined ? fallback : Number(raw); if (value === undefined || !(value > 0) || !Number.isFinite(value)) throw new Error(`--${key} must be a positive number`); return value; };
  const clip = values.get("clip");
  const pose = values.get("pose");
  if ((clip === undefined) === (pose === undefined)) throw new Error(`give exactly one of --clip and --pose\n${USAGE}`);
  const fps = values.get("fps");
  return { asset: need("asset"), ...(clip === undefined ? {} : { clip }), ...(pose === undefined ? {} : { pose }), view: oneOf("view", VIEWS), viewpoint: (values.has("viewpoint") ? oneOf("viewpoint", VIEWPOINTS) : "elevation"), scale: positive("scale"), ...(fps === undefined ? {} : { fps: positive("fps") }), format: oneOf("format", CLI_FORMATS), out: resolve(need("out")), catalog: resolve(values.get("catalog") ?? DEFAULT_CATALOG_ROOT), language: oneOf("lang", LANGUAGES), pxPerMm: positive("px-per-mm", 8), background: values.get("background") ?? "#ffffff" };
}
//#endregion 🔖️Options

//#region 🔖️Pipeline
const PROGRESS = { en: "Exporting", de: "Exportiere" } as const;
const even = (value: number): number => Math.ceil(value / 2) * 2;

/** 🧮️ The evaluated frames of the command: one state per scheduled time (a pose is one frame at t = 0). */
export function evaluateFrames(asset: Asset, options: CliOptions, control: JobControl): { readonly schedule: FrameSchedule; readonly states: EvaluatedState[] } {
  const selector = { kind: "scale", scale: { denominator: options.scale } } as const;
  if (options.pose !== undefined) {
    const schedule: FrameSchedule = { fps: options.fps ?? 1, durationS: 0, loop: false, count: 1, times: [0] };
    return { schedule, states: [evaluate(asset, { kind: "pose", pose: options.pose }, selector)] };
  }
  const clip = asset.clips.find((candidate) => candidate.id === options.clip);
  if (!clip) throw new Error(`asset ${asset.id} has no clip ${options.clip}; clips: ${asset.clips.map((candidate) => candidate.id).join(", ")}`);
  const schedule = frameSchedule(clip, options.fps ?? clip.fps);
  const states: EvaluatedState[] = [];
  for (const timeS of schedule.times) {
    control.signal?.throwIfAborted();
    states.push(evaluate(asset, { kind: "clip", clip: clip.id, timeS }, selector));
    control.progress?.(states.length, schedule.count, { en: "Evaluating frames", de: "Bilder werden ausgewertet" });
  }
  return { schedule, states };
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

/** 🚀️ Runs one export command; returns the files written. */
export async function runExport(options: CliOptions, tools: ToolPort = repoToolPort, control: JobControl = {}): Promise<readonly string[]> {
  const catalog = await loadCatalogFromFiles(options.catalog, control.signal);
  const asset = catalog.assets.find((candidate) => candidate.id === options.asset);
  if (!asset) throw new Error(`asset ${options.asset} is not in the catalog ${options.catalog}`);
  const { schedule, states } = evaluateFrames(asset, options, control);
  const drawing = defaultDepictionOptions(options.view, options.language, options.scale, options.viewpoint);
  const transparentRaster = options.format === "png-sequence" || options.format === "webm";
  const scenes = states.map((state) => depict(state, { ...drawing, transparent: transparentRaster }));
  await mkdir(options.out, { recursive: true });
  const stem = `${options.asset}-${options.clip ?? options.pose}-${options.view}-1to${options.scale}`;
  const written: string[] = [];
  const emit = async (name: string, data: string | Uint8Array): Promise<void> => { const path = join(options.out, name); await writeFile(path, data); written.push(path); };
  switch (options.format) {
    case "svg":
      for (const [index, scene] of scenes.entries()) await emit(`${stem}-${frameName(index).replace(".png", ".svg")}`, writeSvg(scene));
      break;
    case "pdf":
      await emit(`${stem}.pdf`, writePdf(scenes, control));
      break;
    case "dxf":
      for (const [index, state] of states.entries()) await emit(`${stem}-${frameName(index).replace(".png", ".dxf")}`, writeDxf(modelScene(state, { viewpoint: options.viewpoint })));
      break;
    case "dwg": {
      const cli = process.env.SEMIO_STAFFAGE_DWG_CLI;
      if (cli) configureDwg({ cli: async (json) => { const input = join(options.out, ".scene.json"); const output = join(options.out, ".scene.dwg"); await writeFile(input, json); await tools.run(cli, [input, output], control.signal); const bytes = new Uint8Array(await readFile(output)); await rm(input); await rm(output); return bytes; } });
      for (const [index, state] of states.entries()) await emit(`${stem}-${frameName(index).replace(".png", ".dwg")}`, await writeDwg(modelScene(state, { viewpoint: options.viewpoint }), control));
      break;
    }
    default: {
      const work = join(options.out, `.${stem}-frames`);
      try {
        const canvas = await rasterise(tools, scenes, work, options, options.format === "mp4", control);
        if (options.format === "png-sequence") {
          const frames = async function* (): AsyncGenerator<Uint8Array> { for (let index = 0; index < scenes.length; index += 1) yield new Uint8Array(await readFile(join(work, frameName(index)))); };
          await emit(`${stem}.zip`, await writePngSequenceZip(schedule, { asset: options.asset, clip: options.clip ?? options.pose!, view: options.view, scaleDenominator: options.scale, widthPx: canvas.widthPx, heightPx: canvas.heightPx, transparent: true }, frames(), control));
        } else {
          const target = join(options.out, `${stem}.${options.format}`);
          const input = ["-y", "-framerate", String(schedule.fps), "-i", join(work, "frame-%06d.png")];
          const codec = options.format === "mp4" ? ["-c:v", "libx264", "-pix_fmt", "yuv420p", "-movflags", "+faststart"] : ["-c:v", "libvpx-vp9", "-pix_fmt", "yuva420p", "-auto-alt-ref", "0", "-b:v", "0", "-crf", "30"];
          await tools.run("ffmpeg", [...input, ...codec, "-r", String(schedule.fps), "-fps_mode", "cfr", target], control.signal);
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
