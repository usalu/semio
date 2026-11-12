/** 🐍️ The oracle interface of the staffage tests: third-party reference implementations (shapely, PyMuPDF, pikepdf, PDFium, PyAV, ffmpeg, ezdwg) run from wheels that `🐍️oracles/uv.lock` pins with hashes for macOS, Linux and Windows, through the pinned `uv` of the repo tool registry. No Homebrew, apt or system binary is involved, and nothing here is imported by product code.
 * @see ./🐍️oracles/pyproject.toml
 * @see https://docs.astral.sh/uv/concepts/projects/layout/#the-lockfile */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { repoToolPort, type ToolPort } from "../../🔨️modules/📤️export/🔌️tools/🟦️.ts";
import { parsePpm, STAFFAGE_ROOT, tool, type Ppm } from "./🧰️.ts";

/** 📚️ The locked dependency groups of `🐍️oracles/pyproject.toml`: geometry (shapely, numpy, scipy, svgelements), documents (PyMuPDF, pikepdf, pypdfium2, lxml, ezdxf, reportlab, pillow), media (PyAV, imageio-ffmpeg) and dwg (ezdwg, a DWG reader that shares no code with the writer; CPython 3.10 or newer). */
export type OracleSet = "geometry" | "documents" | "media" | "dwg";

/** 🗄️ The per-machine cache root of the repo (where its provisioned runtimes live): `~/Library/Caches/semio` on macOS, `%LOCALAPPDATA%\semio\Cache` on Windows, `$XDG_CACHE_HOME/semio` or `~/.cache/semio` elsewhere. Never inside the worktree. */
const machineCache = (): string => (process.platform === "darwin" ? join(homedir(), "Library", "Caches", "semio") : process.platform === "win32" ? join(process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"), "semio", "Cache") : join(process.env.XDG_CACHE_HOME ?? join(homedir(), ".cache"), "semio"));

const ORACLES = join(STAFFAGE_ROOT, "🧪️tests/🧰️support/🐍️oracles");
const script = (name: string): string => join(ORACLES, name);

/** 🧪️ Runs a python oracle and returns its stdout. A set runs in the hash-locked environment of that group (one environment per group in the machine cache of the repo, built on first use, never inside the worktree); `null` runs the script on the bare interpreter (standard library only). */
export function python(file: string, set: OracleSet | null, args: readonly string[] = []): string {
  const prefix = set === null ? ["--no-project"] : ["--project", ORACLES, "--locked", "--only-group", set, ...(set === "dwg" ? ["--python", ">=3.10"] : [])];
  const environment = { UV_PYTHON_DOWNLOADS: "never", ...(set === null ? {} : { UV_PROJECT_ENVIRONMENT: join(machineCache(), "staffage-oracles", set) }) };
  return tool("uv", ["run", "--quiet", ...prefix, "python", "-I", file, ...args], "stdout", environment);
}

const json = <T>(file: string, set: OracleSet, args: readonly string[]): T => JSON.parse(python(script(file), set, args)) as T;

/** 🖼️ How big an SVG element renders. */
export type SvgRaster = { readonly widthPx: number; readonly heightPx: number; readonly canvasWidthPx: number; readonly canvasHeightPx: number };

/** 🗂️ Rasterises one element (by id) of an SVG file at `pixelsPerMm` with MuPDF and measures the pixels that are at least half covered: the extent of the figure at a known pixel density, independent of the SVG writer. */
export const rasteriseSvg = (svgFile: string, elementId: string, pixelsPerMm: number): SvgRaster => json<SvgRaster>("🖼️raster.py", "documents", ["measure", svgFile, elementId, String(pixelsPerMm)]);

/** 📐️ One shape of an SVG file as svgelements places it. */
export type SvgShapeBox = { readonly id: string | null; readonly fill: string; readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

/** 🔭️ The bounding box of every shape of an SVG file in the user units of the root, the transforms and the viewports of nested svg elements applied, by svgelements: where each drawn rectangle really lands, independent of the SVG writer. */
export const svgShapeBoxes = (svgFile: string): readonly SvgShapeBox[] => JSON.parse(python(script("📐️svgbox.py"), "geometry", [svgFile])) as SvgShapeBox[];

/** 📕️ What qpdf's engine (pikepdf) reads back from a PDF. `problems` is empty when the file structure and every stream decode cleanly (the facts of `qpdf --check`). */
export type PdfReport = { readonly problems: readonly string[]; readonly version: string; readonly pages: number; readonly title: string; readonly subject: string; readonly mediaBoxes: readonly (readonly number[])[] };

/** 📤️ Checks the structure and the streams of a PDF and reads its version, page count, document info and MediaBoxes (points). */
export const inspectPdf = (pdfFile: string): PdfReport => json<PdfReport>("📕️pdf.py", "documents", ["inspect", pdfFile]);

/** 📝️ The text of every page, extracted by PDFium and joined by newlines. */
export const pdfText = (pdfFile: string): string => json<{ pages: string[] }>("📕️pdf.py", "documents", ["text", pdfFile]).pages.join("\n");

/** 🖨️ Renders one page (1-based) with PDFium at `dpi` into pixels. */
export function renderPdf(pdfFile: string, dpi: number, page = 1): Ppm {
  const target = `${pdfFile}.ppm`;
  json<unknown>("📕️pdf.py", "documents", ["render", pdfFile, String(dpi), String(page), target]);
  return parsePpm(readFileSync(target));
}

/** 🎞️ The ffprobe facts of a clip as PyAV reads them. */
export type VideoReport = { readonly codec: string; readonly pixelFormat: string; readonly width: number; readonly height: number; readonly rFrameRate: string; readonly framesRead: number; readonly durationS: number; readonly tags: Readonly<Record<string, string>>; readonly packetTimesS: readonly number[] };

/** 📁️ Probes a video file: codec, pixel format, size, constant frame rate as `n/d`, decoded frame count, container duration, stream tags and the presentation time of every packet in seconds. */
export const probeVideo = (videoFile: string): VideoReport => json<VideoReport>("🎞️video.py", "media", ["probe", videoFile]);

/** 🏗️ One entity of a DWG as ezdwg reads it: an LWPOLYLINE (`closed`, `points`) or a TEXT (`text`, `at`, `height`), with its layer name and colours (index 256 is by layer, `resolvedColorIndex` the colour through the layer). */
export type DwgEntity = { readonly type: "LWPOLYLINE" | "TEXT"; readonly handle: number; readonly layer: string | null; readonly colorIndex: number; readonly resolvedColorIndex: number; readonly closed?: boolean; readonly points?: readonly (readonly [number, number])[]; readonly text?: string; readonly at?: readonly [number, number]; readonly height?: number };

/** 📜️ What ezdwg reads from a DWG file. `lineweight` is in 1/100 mm (-3 is the default lineweight). */
export type DwgReport = { readonly version: string; readonly units: string; readonly insunits: number | null; readonly extmin: readonly number[] | null; readonly extmax: readonly number[] | null; readonly layers: Readonly<Record<string, { readonly colorIndex: number; readonly lineweight: number; readonly off: boolean; readonly frozen: boolean }>>; readonly layouts: readonly string[]; readonly totalEntities: number; readonly entities: readonly DwgEntity[] };

/** 🖊️ Opens a DWG file with ezdwg (R13 to R2018 reader, no code shared with the staffage writer) and reports version, units, extents, layers, layouts and every model space LWPOLYLINE and TEXT. */
export const readDwg = (dwgFile: string): DwgReport => json<DwgReport>("📐️dwg.py", "dwg", [dwgFile]);

let pinnedFfmpeg: string | undefined;

/** 🎬️ The pinned ffmpeg executable (the static build that the imageio-ffmpeg wheel ships for this platform, with libx264 and libvpx-vp9). It is also published as `SEMIO_TOOL_FFMPEG`, the first locator of the repo tool registry, so `repo tools exec ffmpeg` (the product export's own route) resolves to it. THROWS with the tool named when the wheel cannot be run, so a missing tool is a failing test and never a silent skip. */
export function ffmpegExecutable(): string {
  if (pinnedFfmpeg === undefined) {
    try {
      pinnedFfmpeg = json<{ ffmpeg: string }>("🎞️video.py", "media", ["ffmpeg"]).ffmpeg;
    } catch (error) {
      throw new Error(`required tool "ffmpeg" is not available: the pinned imageio-ffmpeg wheel of the media oracle set could not be run (${(error as Error).message.split("\n")[0]}); provision uv with \`repo tools setup toolchains uv\``);
    }
  }
  process.env.SEMIO_TOOL_FFMPEG = pinnedFfmpeg;
  return pinnedFfmpeg;
}

/** 🚨️ Runs the pinned ffmpeg and returns its stdout bytes; throws on a non-zero exit. */
export const ffmpegBytes = (args: readonly string[]): Buffer => execFileSync(ffmpegExecutable(), [...args], { maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "pipe"] });

/** 🔌️ The tool port of the export CLI for tests: `rsvg-convert` (a system binary with no pinned source) is answered by the MuPDF rasteriser of the documents set with the same arguments, every other tool goes through the product's own {@link repoToolPort}, with ffmpeg resolved to the pinned executable. */
export const oracleToolPort: ToolPort = {
  async run(name, args, signal) {
    if (name === "rsvg-convert") return python(script("🖼️raster.py"), "documents", ["rsvg", ...args]);
    if (name === "ffmpeg") ffmpegExecutable();
    return repoToolPort.run(name, args, signal);
  },
};
