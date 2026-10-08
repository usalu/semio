/** 🚪️ Export and import through the served `s` React shell, per kind, as a user drives them (acceptance 1.7), plus the
 * live half of command reachability (1.8): every verb that was `BatchOnlyPendingRewrite` must dispatch from its rail row.
 *
 * One row per editor program the shell's own catalog probe lists (`window.__semioOsCatalogProbe.programs`), opened from the
 * Home landing through the command palette, its first non-empty example seated from the navbar. Every row drives the
 * framework's document pair from its Actions rail rows — Export Document, Import Document through the host file picker (a new window
 * of the same program), Export Document again from that window — and the canonical archive must come back byte for byte.
 * Per program with pins (`🌎️hub/🧫️fixtures/🚪️io-matrix.json`, the kind's own formats): every pinned export is pressed in
 * the Actions rail and must hand the browser at least one downloaded file that a third-party parser accepts (Chromium's
 * DOMParser / image decoder, pdf.js, three.js' mesh loaders, fflate, FFmpeg; STEP, CSV, text and glTF containers by their
 * published structure); every pinned import opens the host file picker (or stages the file text into its argument), reads
 * back the file an export produced after the program's matrix verb moved the document, and the same export pressed again
 * must write the same file (bytes, or the same JSON / text up to whitespace). `reach` verbs are pressed with their first
 * live options and must not be refused as `interactive-job.not-ui-safe`.
 *
 * `--hub <url>` joins the serve to that hub, signs in with `OS_HUB_PROBE_EMAIL`/`OS_HUB_PROBE_PASSWORD`, creates one
 * document per pinned creatable kind in the space the hub-document sweep uses and drives the same pins on it.
 * @see ../🧮️program-matrix/🟦️.ts — open, rail, staged-argument and witness helpers
 * @see ../🗂️hub-document-sweep/🟦️.ts — sign-in, space and creation journey reused by `--hub`
 */

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import type { Download, FileChooser, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { FAULT, NOISE, awaitBeacon, click, clickUncovered, dismissIntroduction, driveRenderedEdit, fillStagedArgument, keyOf, readMatrixPins, readShell, roleOf, seatLocale, unfoldActionsRail, windowIds, withDevServe, witness, type MatrixPins, type MatrixProgram, type MatrixRenderedEdit } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { createKind, openSweepSpace, stagedKinds, type HubDocumentSweepOptions, type HubSweepRow } from "../🗂️document-sweep/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { decodeDocumentArchiveBytes } from "../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID } from "../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";

//#region 🔖️Pins
/** 🧾️ A file format a third-party parser must accept; `any` sniffs the format from the downloaded file's name. */
export type IoFormat = "json" | "csv" | "text" | "svg" | "png" | "pdf" | "obj" | "stl" | "ply" | "gltf" | "glb" | "step" | "zip" | "mp4" | "archive" | "any";

/** 📤️ One export control: the rail verb, its staged arguments and the format its file must parse as. */
export type IoExportPin = Readonly<{ id: string; verb: string; args?: Readonly<Record<string, string>>; format: IoFormat }>;

/** 📥️ One import control: the verb that opens the host file picker, or the argument the file text is staged into. */
export type IoImportPin = Readonly<{ id: string; verb: string; via: "picker" | "argument"; argument?: string; args?: Readonly<Record<string, string>>; from: string }>;

/** 📌️ One program's export, import and reach pins. */
export type IoProgramPins = Readonly<{ exports: readonly IoExportPin[]; imports: readonly IoImportPin[]; reach: readonly string[] }>;

/** 📌️ `semio.os-dev.io-matrix/v1`. */
export type IoMatrixPins = Readonly<{ schema: "semio.os-dev.io-matrix/v1"; programs: Readonly<Record<string, IoProgramPins>> }>;

const IO_FORMATS: readonly IoFormat[] = ["json", "csv", "text", "svg", "png", "pdf", "obj", "stl", "ply", "gltf", "glb", "step", "zip", "mp4", "archive", "any"];

/** 📌️ Reads and checks the pins: every import reads a declared export of its own program, every format is known. */
export function readIoMatrixPins(): IoMatrixPins {
  const pins = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "🚪️io-matrix.json"), "utf8")) as IoMatrixPins;
  if (pins.schema !== "semio.os-dev.io-matrix/v1") throw new Error(`io matrix pins: unexpected schema ${String(pins.schema)}`);
  for (const [key, program] of Object.entries(pins.programs)) {
    const exports = new Set(program.exports.map((pin) => pin.id));
    if (exports.size !== program.exports.length) throw new Error(`io matrix pins: ${key} repeats an export id`);
    for (const pin of program.exports) if (!IO_FORMATS.includes(pin.format)) throw new Error(`io matrix pins: ${key}.${pin.id} has unknown format ${pin.format}`);
    for (const pin of program.imports) {
      if (!exports.has(pin.from)) throw new Error(`io matrix pins: ${key} import ${pin.id} reads undeclared export ${pin.from}`);
      if (pin.via === "argument" && !pin.argument) throw new Error(`io matrix pins: ${key} import ${pin.id} stages no argument`);
    }
  }
  return pins;
}

/** 🗺️ The program matrix's own move between a program's export and its import: the verb, its staged rail arguments, and —
 * for a revision-bound verb (`set-cell`, `set-node`) — the rendered edit that drives it through the program's own editor,
 * resolved like the matrix does (`<key>` beats `<base>` beats the origin plugin's pin). */
type MatrixMove = Readonly<{ verb: string | null; args: Readonly<Record<string, string>>; edit: MatrixRenderedEdit | undefined }>;

function matrixVerbOf(pins: MatrixPins, key: string, pluginId: string): MatrixMove {
  const base = key.split("/").slice(0, 2).join("/");
  const origin = pins.kindOrigin[base] ?? pluginId;
  const verb = pins.kindVerbs[key] ?? pins.kindVerbs[base] ?? pins.pluginVerbs[origin] ?? null;
  if (verb === null) return { verb, args: {}, edit: undefined };
  const args = pins.kindArgs[`${key}.${verb}`] ?? pins.kindArgs[`${base}.${verb}`] ?? pins.pluginArgs[`${origin}.${verb}`] ?? {};
  return { verb, args, edit: pins.kindEdits[`${key}.${verb}`] ?? pins.kindEdits[`${base}.${verb}`] ?? pins.pluginEdits[`${origin}.${verb}`] };
}
//#endregion 🔖️Pins

//#region 🔖️Oracles
/** 🔎️ The format a file of `any` is judged as, from its name. */
export function formatOfFileName(name: string): IoFormat {
  const lower = name.toLowerCase();
  const table: readonly (readonly [RegExp, IoFormat])[] = [
    [/\.json$/u, "json"],
    [/\.csv$/u, "csv"],
    [/\.svg$/u, "svg"],
    [/\.png$/u, "png"],
    [/\.pdf$/u, "pdf"],
    [/\.obj$/u, "obj"],
    [/\.stl$/u, "stl"],
    [/\.ply$/u, "ply"],
    [/\.gltf$/u, "gltf"],
    [/\.glb$/u, "glb"],
    [/\.(step|stp)$/u, "step"],
    [/\.zip$/u, "zip"],
    [/\.mp4$/u, "mp4"],
    [/\.semio-archive$/u, "archive"],
  ];
  return table.find(([pattern]) => pattern.test(lower))?.[1] ?? "text";
}

/** ⚖️ One file's verdict: which parser judged it and what it read. */
export type IoOracleVerdict = Readonly<{ format: IoFormat; oracle: string; ok: boolean; detail: string }>;

const utf8 = (bytes: Uint8Array): string | null => {
  try {
    return new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    return null;
  }
};

const arrayBufferOf = (bytes: Uint8Array): ArrayBuffer => bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;

/** 🎞️ FFmpeg as the MP4 oracle: `ffprobe` must read an H.264 video stream with a size and at least one decoded frame, and
 * `ffmpeg` must decode the whole file without an error.
 * @see https://ffmpeg.org/ffprobe.html */
export function judgeMp4File(bytes: Uint8Array): Readonly<{ ok: boolean; detail: string }> {
  const dir = mkdtempSync(join(tmpdir(), "io-matrix-mp4-"));
  const path = join(dir, "video.mp4");
  try {
    writeFileSync(path, bytes);
    const probe = spawnSync("ffprobe", ["-v", "error", "-count_frames", "-select_streams", "v:0", "-show_entries", "stream=codec_name,width,height,nb_read_frames", "-of", "json", path], { encoding: "utf8" });
    if (probe.error) return { ok: false, detail: `ffprobe unavailable: ${probe.error.message}` };
    if (probe.status !== 0) return { ok: false, detail: `ffprobe exit ${String(probe.status)}: ${probe.stderr.trim().split("\n")[0] ?? ""}`.slice(0, 160) };
    const stream = (JSON.parse(probe.stdout) as { streams?: { codec_name?: string; width?: number; height?: number; nb_read_frames?: string }[] }).streams?.[0];
    const frames = Number(stream?.nb_read_frames ?? 0);
    const decode = spawnSync("ffmpeg", ["-v", "error", "-i", path, "-f", "null", "-"], { encoding: "utf8" });
    const decoded = decode.status === 0 && decode.stderr.trim().length === 0;
    const ok = stream?.codec_name === "h264" && (stream.width ?? 0) > 0 && (stream.height ?? 0) > 0 && frames > 0 && decoded;
    return { ok, detail: `${String(stream?.codec_name)} ${String(stream?.width)}x${String(stream?.height)}, ${frames} frames, ffmpeg decode ${decoded ? "clean" : `exit ${String(decode.status)} ${decode.stderr.trim().split("\n")[0] ?? ""}`.slice(0, 80)}` };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

/** ⚖️ Judges one downloaded file with a parser that is not ours: the page's Chromium for SVG and raster images, pdf.js for
 * PDF, three.js' loaders for OBJ/STL/PLY, fflate for ZIP, FFmpeg for MP4; STEP, CSV, text and glTF containers by their
 * published structure. */
export async function judgeFile(page: Page, name: string, bytes: Uint8Array, declared: IoFormat): Promise<IoOracleVerdict> {
  const format = declared === "any" ? formatOfFileName(name) : declared;
  if (bytes.byteLength === 0) return { format, oracle: "size", ok: false, detail: "empty file" };
  const text = utf8(bytes);
  try {
    switch (format) {
      case "json": {
        const value = JSON.parse(text ?? "") as unknown;
        return { format, oracle: "JSON.parse", ok: value !== null && typeof value === "object", detail: Array.isArray(value) ? `array[${value.length}]` : `object{${Object.keys(value as object).length}}` };
      }
      case "csv": {
        const rows = (text ?? "").split(/\r?\n/u).filter((line) => line.length > 0);
        return { format, oracle: "rfc4180-structure", ok: text !== null && rows.length > 0, detail: `${rows.length} rows` };
      }
      case "text":
        return { format, oracle: "utf-8", ok: text !== null && text.trim().length > 0, detail: `${text?.length ?? 0} chars` };
      case "step": {
        const ok = text !== null && /^\s*ISO-10303-21;/u.test(text) && /END-ISO-10303-21;\s*$/u.test(text) && /\bDATA;/u.test(text);
        return { format, oracle: "iso-10303-21-structure", ok, detail: `${text?.split("\n").length ?? 0} lines` };
      }
      case "gltf": {
        const asset = (JSON.parse(text ?? "") as { asset?: { version?: string } }).asset;
        return { format, oracle: "gltf-2.0-structure", ok: asset?.version === "2.0", detail: `asset.version=${String(asset?.version)}` };
      }
      case "glb": {
        const view = new DataView(arrayBufferOf(bytes));
        const magic = view.getUint32(0, true) === 0x46546c67;
        const version = view.getUint32(4, true);
        const jsonLength = view.getUint32(12, true);
        const jsonType = view.getUint32(16, true) === 0x4e4f534a;
        const asset = magic && jsonType ? (JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + jsonLength))) as { asset?: { version?: string } }).asset : undefined;
        return { format, oracle: "glb-2.0-structure", ok: magic && version === 2 && asset?.version === "2.0", detail: `magic=${magic} version=${version}` };
      }
      case "pdf": {
        const pdfjs = (await import("pdfjs-dist/legacy/build/pdf.mjs")) as { getDocument: (source: { data: Uint8Array; isEvalSupported?: boolean; useSystemFonts?: boolean }) => { promise: Promise<{ numPages: number; destroy: () => Promise<void> }> } };
        const document = await pdfjs.getDocument({ data: new Uint8Array(bytes), isEvalSupported: false, useSystemFonts: false }).promise;
        const pages = document.numPages;
        await document.destroy();
        return { format, oracle: "pdf.js", ok: pages > 0, detail: `${pages} pages` };
      }
      case "obj": {
        const { OBJLoader } = (await import("three/examples/jsm/loaders/OBJLoader.js")) as { OBJLoader: new () => { parse: (text: string) => { children: { geometry?: { attributes?: { position?: { count: number } } } }[] } } };
        const group = new OBJLoader().parse(text ?? "");
        const vertices = group.children.reduce((sum, child) => sum + (child.geometry?.attributes?.position?.count ?? 0), 0);
        return { format, oracle: "three.OBJLoader", ok: vertices > 0, detail: `${group.children.length} meshes, ${vertices} vertices` };
      }
      case "stl": {
        const { STLLoader } = (await import("three/examples/jsm/loaders/STLLoader.js")) as { STLLoader: new () => { parse: (data: ArrayBuffer) => { attributes: { position?: { count: number } } } } };
        const vertices = new STLLoader().parse(arrayBufferOf(bytes)).attributes.position?.count ?? 0;
        return { format, oracle: "three.STLLoader", ok: vertices > 0, detail: `${vertices} vertices` };
      }
      case "ply": {
        const { PLYLoader } = (await import("three/examples/jsm/loaders/PLYLoader.js")) as { PLYLoader: new () => { parse: (data: ArrayBuffer) => { attributes: { position?: { count: number } } } } };
        const vertices = new PLYLoader().parse(arrayBufferOf(bytes)).attributes.position?.count ?? 0;
        return { format, oracle: "three.PLYLoader", ok: vertices > 0, detail: `${vertices} vertices` };
      }
      case "svg": {
        const parsed = await page.evaluate((source) => {
          const document = new DOMParser().parseFromString(source, "image/svg+xml");
          return { error: document.querySelector("parsererror") !== null, root: document.documentElement.localName, children: document.documentElement.childElementCount };
        }, text ?? "");
        return { format, oracle: "chromium.DOMParser", ok: !parsed.error && parsed.root === "svg", detail: `<${parsed.root}> ${parsed.children} children` };
      }
      case "png": {
        const size = await page.evaluate(async (base64) => {
          const binary = Uint8Array.from(atob(base64), (character) => character.charCodeAt(0));
          const bitmap = await createImageBitmap(new Blob([binary]));
          return { width: bitmap.width, height: bitmap.height };
        }, Buffer.from(bytes).toString("base64"));
        return { format, oracle: "chromium.createImageBitmap", ok: size.width > 0 && size.height > 0, detail: `${size.width}x${size.height}` };
      }
      case "zip": {
        const { unzipSync } = (await import("fflate")) as { unzipSync: (data: Uint8Array) => Record<string, Uint8Array> };
        const entries = Object.keys(unzipSync(new Uint8Array(bytes)));
        return { format, oracle: "fflate.unzipSync", ok: entries.length > 0, detail: `${entries.length} entries: ${entries.slice(0, 4).join(", ")}` };
      }
      case "mp4":
        return { format, oracle: "ffprobe+ffmpeg", ...judgeMp4File(bytes) };
      case "archive": {
        const archive = decodeDocumentArchiveBytes(bytes);
        return { format, oracle: "document-archive-v1", ok: archive.parent_pack.length > 0 && archive.parent_spr.length > 0, detail: `root ${archive.parent_pack.length} B, op log ${archive.parent_spr.length} B, ${archive.members.length} members` };
      }
      case "any":
        return { format, oracle: "none", ok: false, detail: "unreachable" };
    }
  } catch (error) {
    return { format, oracle: "parser", ok: false, detail: String(error instanceof Error ? error.message : error).split("\n")[0]!.slice(0, 160) };
  }
}

/** 🔁️ Whether two exports of the same control wrote the same file: `identical` bytes, `equivalent` JSON or text up to
 * whitespace, else `differs` with the first differing byte. */
export function compareExports(first: Uint8Array, second: Uint8Array): Readonly<{ verdict: "identical" | "equivalent" | "differs"; detail: string }> {
  const withoutDefaults = (value: unknown): unknown => {
    if (Array.isArray(value)) return value.map(withoutDefaults);
    if (value === null || typeof value !== "object") return value;
    const kept = Object.entries(value as Record<string, unknown>)
      .map(([key, entry]) => [key, withoutDefaults(entry)] as const)
      .filter(([, entry]) => entry !== null && !(Array.isArray(entry) && entry.length === 0) && !(typeof entry === "object" && !Array.isArray(entry) && Object.keys(entry as object).length === 0))
      .sort(([left], [right]) => left.localeCompare(right));
    return Object.fromEntries(kept);
  };
  if (first.byteLength === second.byteLength && first.every((value, index) => value === second[index])) return { verdict: "identical", detail: `${first.byteLength} bytes` };
  const left = utf8(first);
  const right = utf8(second);
  if (left !== null && right !== null) {
    try {
      const leftValue = JSON.parse(left) as unknown;
      const rightValue = JSON.parse(right) as unknown;
      if (JSON.stringify(leftValue) === JSON.stringify(rightValue)) return { verdict: "equivalent", detail: "same JSON value" };
      if (JSON.stringify(withoutDefaults(leftValue)) === JSON.stringify(withoutDefaults(rightValue))) return { verdict: "equivalent", detail: "same JSON value once empty defaults are dropped" };
    } catch {}
    if (left.replace(/\s+/gu, " ").trim() === right.replace(/\s+/gu, " ").trim()) return { verdict: "equivalent", detail: "same text up to whitespace" };
  }
  const limit = Math.min(first.byteLength, second.byteLength);
  let offset = 0;
  while (offset < limit && first[offset] === second[offset]) offset += 1;
  return { verdict: "differs", detail: `${first.byteLength} vs ${second.byteLength} bytes, first difference at byte ${offset}` };
}
/** 🔺️ The triangles a mesh file holds, as three.js reads it: every triangle's corners rounded to 1e-5 and sorted, so two
 * files that store the same surface with other vertex orders, indices or group names compare equal. */
export async function meshTriangles(format: IoFormat, bytes: Uint8Array): Promise<string[] | null> {
  type Geometry = { index: { array: ArrayLike<number> } | null; attributes: { position?: { array: ArrayLike<number>; count: number } } };
  const geometries: Geometry[] = [];
  if (format === "obj") {
    const { OBJLoader } = (await import("three/examples/jsm/loaders/OBJLoader.js")) as { OBJLoader: new () => { parse: (text: string) => { children: { geometry?: Geometry }[] } } };
    for (const child of new OBJLoader().parse(new TextDecoder().decode(bytes)).children) if (child.geometry) geometries.push(child.geometry);
  } else if (format === "stl") {
    const { STLLoader } = (await import("three/examples/jsm/loaders/STLLoader.js")) as { STLLoader: new () => { parse: (data: ArrayBuffer) => Geometry } };
    geometries.push(new STLLoader().parse(arrayBufferOf(bytes)));
  } else if (format === "ply") {
    const { PLYLoader } = (await import("three/examples/jsm/loaders/PLYLoader.js")) as { PLYLoader: new () => { parse: (data: ArrayBuffer) => Geometry } };
    geometries.push(new PLYLoader().parse(arrayBufferOf(bytes)));
  } else return null;
  const triangles: string[] = [];
  for (const geometry of geometries) {
    const position = geometry.attributes.position;
    if (!position) continue;
    const corner = (index: number): string => [0, 1, 2].map((axis) => Math.round(Number(position.array[index * 3 + axis]) * 1e5) / 1e5).join(",");
    const count = geometry.index ? geometry.index.array.length : position.count;
    for (let offset = 0; offset + 2 < count; offset += 3) {
      const at = (step: number): number => (geometry.index ? Number(geometry.index.array[offset + step]) : offset + step);
      triangles.push([corner(at(0)), corner(at(1)), corner(at(2))].sort().join(" "));
    }
  }
  return triangles.sort();
}
//#endregion 🔖️Oracles

//#region 🔖️Driver
/** 📦️ One downloaded file as the harness kept it. */
type SavedFile = { name: string; path: string; bytes: number; sha256: string; verdict: IoOracleVerdict };

/** 🧾️ What one press of a rail verb did. */
type Press = { verb: string; row: string; filled: string[]; submit: string; refusals: string[]; notices: string[] };

/** 🎛️ The page-level recorders one run shares: downloads, file choosers, console refusals/faults, transient notices. */
type Recorders = {
  console: string[];
  downloads: Download[];
  chooser: { pending: string | null; log: string[] };
  refusals: string[];
  faults: string[];
  notices: () => Promise<string[]>;
};

const REFUSAL = /refused|rejected|dropped action|not-ui-safe|interactive-job\.|unsupported host effect/iu;
const DEAD = /not-ui-safe|BatchOnlyPendingRewrite|dropped action|unsupported host effect/u;

async function installRecorders(page: Page): Promise<Recorders> {
  await page.addInitScript(() => {
    const seen: string[] = [];
    Object.defineProperty(window, "__ioNotices", { value: seen });
    new MutationObserver(() => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
        const line = `${element.getAttribute("data-notice-code") ?? ""}|${(element.firstChild?.textContent ?? element.textContent ?? "").trim()}`.slice(0, 200);
        if (seen.at(-1) !== line) seen.push(line);
      }
    }).observe(document, { subtree: true, childList: true, characterData: true });
  });
  const recorders: Recorders = {
    console: [],
    downloads: [],
    chooser: { pending: null, log: [] },
    refusals: [],
    faults: [],
    notices: () => page.evaluate(() => [...((window as unknown as { __ioNotices?: string[] }).__ioNotices ?? [])]).catch(() => []),
  };
  page.on("download", (download) => {
    recorders.downloads.push(download);
    recorders.console.push(`${new Date().toISOString().slice(11, 23)} download: ${download.suggestedFilename()}`);
  });
  page.on("filechooser", (chooser: FileChooser) => {
    const pending = recorders.chooser.pending;
    recorders.chooser.pending = null;
    recorders.chooser.log.push(pending === null ? "unexpected" : `set:${pending.split("/").at(-1)}`);
    recorders.console.push(`${new Date().toISOString().slice(11, 23)} filechooser: ${pending === null ? "unexpected" : "answered"}`);
    void chooser.setFiles(pending === null ? [] : [pending]).catch((error: unknown) => recorders.chooser.log.push(`set-failed:${String(error).split("\n")[0]!.slice(0, 80)}`));
  });
  page.on("pageerror", (error) => recorders.faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
  page.on("console", (message) => {
    const text = message.text();
    if (/WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u.test(text)) return;
    recorders.console.push(`${new Date().toISOString().slice(11, 23)} ${message.type()}: ${text}`.slice(0, 400));
    if (REFUSAL.test(text)) recorders.refusals.push(text.slice(0, 260));
    if (FAULT.test(text) && !NOISE.test(text)) recorders.faults.push(`${message.type()}: ${text}`.slice(0, 300));
  });
  return recorders;
}

/** 🖱️ Presses one rail verb as a person does: its row fires a zero-argument verb, or unfolds its staged form, which is
 * filled and executed. Refusals and notices the press caused are part of the answer. */
async function pressVerb(page: Page, recorders: Recorders, verb: string, args: Readonly<Record<string, string>>, liveId: string, settleMs = 2_500): Promise<Press> {
  const refusalCursor = recorders.refusals.length;
  const noticeCursor = (await recorders.notices()).length;
  const rowSelector = `[data-slot="window-action-pane"] [id="action.${verb}"]`;
  const executeSelector = `[data-slot="window-action-pane"] [id$=".action.${verb}.execute"]`;
  if ((await page.locator(rowSelector).count()) === 0) await unfoldActionsRail(page, 12_000);
  const row = await clickUncovered(page, rowSelector);
  await page.waitForTimeout(700);
  const staged = ((await page.locator(rowSelector).first().textContent().catch(() => "")) ?? "").trim().endsWith("…");
  if (staged && (await page.locator(executeSelector).count()) === 0) {
    await clickUncovered(page, rowSelector);
    await page.waitForTimeout(700);
  }
  const filled: string[] = [];
  for (const [key, value] of Object.entries(args)) filled.push(await fillStagedArgument(page, key, value, liveId));
  const submit = staged ? await click(page, executeSelector) : row === "ok" ? "direct" : row;
  await page.waitForTimeout(settleMs);
  return { verb, row, filled, submit, refusals: recorders.refusals.slice(refusalCursor), notices: (await recorders.notices()).slice(noticeCursor) };
}

/** ✍️ Moves the document through the program's own rendered editor (a revision-bound verb no rail argument can stage);
 * refusals and notices the edit caused are part of the answer, exactly as for {@link pressVerb}. */
async function driveMove(page: Page, recorders: Recorders, verb: string, edit: MatrixRenderedEdit, settleMs = 2_500): Promise<Press> {
  const refusalCursor = recorders.refusals.length;
  const noticeCursor = (await recorders.notices()).length;
  const submit = await driveRenderedEdit(page, edit);
  await page.waitForTimeout(settleMs);
  return { verb, row: "rendered", filled: [], submit, refusals: recorders.refusals.slice(refusalCursor), notices: (await recorders.notices()).slice(noticeCursor) };
}

/** 📥️ Waits for the downloads a press started: the first within `firstMs`, then any sibling files within `quietMs`. */
async function takeDownloads(page: Page, recorders: Recorders, cursor: number, firstMs: number, quietMs: number): Promise<Download[]> {
  const firstDeadline = Date.now() + firstMs;
  while (recorders.downloads.length === cursor && Date.now() < firstDeadline) await page.waitForTimeout(250);
  if (recorders.downloads.length === cursor) return [];
  let count = recorders.downloads.length;
  let quietSince = Date.now();
  while (Date.now() - quietSince < quietMs) {
    await page.waitForTimeout(250);
    if (recorders.downloads.length !== count) {
      count = recorders.downloads.length;
      quietSince = Date.now();
    }
  }
  return recorders.downloads.slice(cursor);
}

async function saveDownloads(page: Page, downloads: readonly Download[], dir: string, prefix: string, format: IoFormat): Promise<SavedFile[]> {
  mkdirSync(dir, { recursive: true });
  const saved: SavedFile[] = [];
  for (const [index, download] of downloads.entries()) {
    const name = download.suggestedFilename();
    const path = join(dir, `${prefix}-${index}-${name.replace(/[^A-Za-z0-9._-]+/gu, "_")}`);
    const failure = await download.failure();
    if (failure !== null) {
      saved.push({ name, path, bytes: 0, sha256: "", verdict: { format, oracle: "download", ok: false, detail: `download failed: ${failure}` } });
      continue;
    }
    await download.saveAs(path);
    const bytes = new Uint8Array(readFileSync(path));
    saved.push({ name, path, bytes: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex"), verdict: await judgeFile(page, name, bytes, format) });
  }
  return saved;
}

/** 📤️ One export control, pressed once. */
async function runExport(page: Page, recorders: Recorders, pin: IoExportPin, liveId: string, dir: string, prefix: string) {
  const cursor = recorders.downloads.length;
  const press = await pressVerb(page, recorders, pin.verb, pin.args ?? {}, liveId, 500);
  const files = await saveDownloads(page, await takeDownloads(page, recorders, cursor, 60_000, 3_000), dir, prefix, pin.format);
  return { id: pin.id, press, files, ok: files.length > 0 && files.every((file) => file.verdict.ok) };
}

/** 🔁️ One import control against the file its export wrote: the program's matrix verb moves the document first, then the
 * file is read back through the picker (or the staged argument) and the export is pressed again. */
async function runImport(page: Page, recorders: Recorders, pin: IoImportPin, exportPin: IoExportPin, first: SavedFile | undefined, move: MatrixMove, liveId: string, dir: string) {
  if (first === undefined) return { id: pin.id, ok: false, detail: `export ${pin.from} wrote no file to read back` };
  const moved = move.verb === null ? null : move.edit === undefined ? await pressVerb(page, recorders, move.verb, move.args, liveId) : await driveMove(page, recorders, move.verb, move.edit);
  const divergedExport = await runExport(page, recorders, exportPin, liveId, dir, `${pin.id}-moved`);
  const diverged = divergedExport.files[0] === undefined ? null : compareExports(new Uint8Array(readFileSync(first.path)), new Uint8Array(readFileSync(divergedExport.files[0].path))).verdict !== "identical";
  const before = witness(await readShell(page));
  const chooserCursor = recorders.chooser.log.length;
  let press: Press;
  if (pin.via === "picker") {
    recorders.chooser.pending = first.path;
    press = await pressVerb(page, recorders, pin.verb, pin.args ?? {}, liveId, 1_000);
    const chooserDeadline = Date.now() + 30_000;
    while (recorders.chooser.pending !== null && Date.now() < chooserDeadline) await page.waitForTimeout(250);
    recorders.chooser.pending = null;
    await page.waitForTimeout(8_000);
  } else {
    press = await pressVerb(page, recorders, pin.verb, { ...(pin.args ?? {}), [pin.argument!]: readFileSync(first.path, "utf8") }, liveId, 8_000);
  }
  const after = witness(await readShell(page));
  const second = await runExport(page, recorders, exportPin, liveId, dir, `${pin.id}-reimported`);
  let roundTrip: Readonly<{ verdict: "identical" | "equivalent" | "differs"; detail: string }> = second.files[0] === undefined ? { verdict: "differs", detail: "the export after the import wrote no file" } : compareExports(new Uint8Array(readFileSync(first.path)), new Uint8Array(readFileSync(second.files[0].path)));
  if (roundTrip.verdict === "differs" && second.files[0] !== undefined) {
    const format = exportPin.format === "any" ? formatOfFileName(first.name) : exportPin.format;
    const [left, right] = await Promise.all([meshTriangles(format, new Uint8Array(readFileSync(first.path))), meshTriangles(format, new Uint8Array(readFileSync(second.files[0].path)))]);
    if (left !== null && right !== null && left.length > 0 && left.join("\n") === right.join("\n")) roundTrip = { verdict: "equivalent", detail: `same ${left.length} triangles (three.js), other vertex order` };
  }
  const chooser = recorders.chooser.log.slice(chooserCursor);
  const pickerOk = pin.via === "argument" || chooser.some((line) => line.startsWith("set:"));
  return {
    id: pin.id,
    via: pin.via,
    moved: moved === null ? null : { verb: moved.verb, submit: moved.submit, refusals: moved.refusals.slice(0, 3) },
    diverged,
    press,
    chooser,
    documentChanged: after.edits !== before.edits || after.render !== before.render || after.applied !== before.applied,
    roundTrip,
    ok: pickerOk && roundTrip.verdict !== "differs" && press.refusals.every((line) => !DEAD.test(line)),
  };
}

/** 🎯️ One formerly batch-only verb pressed from its rail row with its first live options: it must reach its handler. */
async function runReach(page: Page, recorders: Recorders, verb: string, liveId: string) {
  const argIds = await page.evaluate((id) => [...document.querySelectorAll(`[data-slot="window-action-pane"] [id*=".action.${id}.arg."], [data-slot="window-action-pane"] [id^="action.${id}.arg."]`)].map((element) => /\.arg\.([^.]+)$/u.exec(element.id)?.[1] ?? "").filter(Boolean), verb);
  const press = await pressVerb(page, recorders, verb, Object.fromEntries(argIds.map((id) => [id, liveId])), liveId);
  const dead = press.refusals.find((line) => DEAD.test(line)) ?? null;
  return { verb, press, dispatched: press.row === "ok" && dead === null, dead };
}

/** 📤️ Export Document from its rail row: the focused program's archive must download and decode. */
async function exportDocumentArchive(page: Page, recorders: Recorders, dir: string, prefix: string) {
  const cursor = recorders.downloads.length;
  const noticeCursor = (await recorders.notices()).length;
  const pressed = (await pressVerb(page, recorders, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, {}, "", 500)).row;
  const files = pressed === "ok" ? await saveDownloads(page, await takeDownloads(page, recorders, cursor, 30_000, 1_500), dir, prefix, "archive") : [];
  return { pressed, files, notices: (await recorders.notices()).slice(noticeCursor).slice(0, 3), ok: files.length === 1 && files[0]!.verdict.ok };
}

/** 🔁️ The framework document round trip every editor offers: Export Document, Import Document through the host file
 * picker (a NEW window of the same program, loaded as a cancellable task), Export Document again from that window — the
 * canonical archive must come back byte for byte. The imported window is closed again. */
async function runDocumentTransfer(page: Page, recorders: Recorders, dir: string) {
  const exported = await exportDocumentArchive(page, recorders, dir, "document");
  const first = exported.files[0];
  if (!exported.ok || first === undefined) return { export: exported, import: null, ok: false };
  const before = await windowIds(page);
  const chooserCursor = recorders.chooser.log.length;
  const noticeCursor = (await recorders.notices()).length;
  recorders.chooser.pending = first.path;
  const pressed = (await pressVerb(page, recorders, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, {}, "", 500)).row;
  const began = Date.now();
  let outcome: string | null = null;
  while (pressed === "ok" && outcome === null && Date.now() - began < 180_000) {
    await page.waitForTimeout(500);
    outcome = (await recorders.notices()).slice(noticeCursor).map((line) => /^shell\.documentTransfer\.(imported|import-failed|import-unreadable|import-cancelled)\|/u.exec(line)?.[1] ?? null).find((code) => code !== null) ?? null;
  }
  recorders.chooser.pending = null;
  await page.waitForTimeout(2_500);
  const opened = (await windowIds(page)).filter((id) => !before.includes(id));
  const second = outcome === "imported" ? await exportDocumentArchive(page, recorders, dir, "document-reimported") : null;
  const roundTrip = second?.files[0] === undefined ? null : compareExports(new Uint8Array(readFileSync(first.path)), new Uint8Array(readFileSync(second.files[0].path)));
  await closeWindows(page, opened);
  const imported = { pressed, chooser: recorders.chooser.log.slice(chooserCursor), outcome: outcome ?? "no outcome within 180 s", loadMs: Date.now() - began, openedWindows: opened, reexport: second, roundTrip };
  return { export: exported, import: imported, ok: outcome === "imported" && opened.length > 0 && second?.ok === true && roundTrip?.verdict === "identical" };
}

/** 📚️ Seats the program's first non-empty example from the navbar picker, when it offers one. */
async function seatFirstExample(page: Page): Promise<string> {
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  if ((await picker.count()) === 0) return "no example picker";
  await picker.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  const options = await page.locator('[role="option"]').evaluateAll((rows) => rows.map((row) => ({ value: row.getAttribute("data-value") ?? "", text: (row.textContent ?? "").trim(), selected: row.getAttribute("aria-selected") === "true" })));
  const wanted = options.find((option) => option.value !== "" && !/^_+none_+$/u.test(option.value) && !/^(empty|leer|none|keine|no example|kein beispiel)$/iu.test(option.text));
  if (!wanted) {
    await page.keyboard.press("Escape");
    return `no non-empty example among ${options.length}`;
  }
  if (!wanted.selected) await page.locator('[role="option"]').nth(options.indexOf(wanted)).click({ force: true }).catch(() => undefined);
  else await page.keyboard.press("Escape");
  await page.waitForTimeout(8_000);
  return `${wanted.value}=${wanted.text}`;
}
//#endregion 🔖️Driver

//#region 🔖️Matrix
/** 🎛️ One run. */
export type IoMatrixOptions = Readonly<{
  baseUrl: string;
  hubUrl: string | null;
  email: string | null;
  password: string | null;
  spaceName: string;
  tag: string;
  locale: string;
  only: readonly string[];
  skip: readonly string[];
  resume: boolean;
  outDir: string;
  headed: boolean;
  installBudgetMs: number;
  signal: AbortSignal;
}>;

/** 🧾️ One program's row. */
export type IoMatrixRow = Record<string, unknown> & { key: string; appId: string; pinned: boolean; pass: boolean; reach: { verb: string; dispatched: boolean; dead: string | null }[] };

/** 📊️ The whole run, rewritten after every row. */
export type IoMatrixReport = { baseUrl: string; hubUrl: string | null; tag: string; locale: string; started: string; finished?: string; census?: unknown; rows: IoMatrixRow[]; fatal?: string; cancelled?: boolean; faultsTail?: string[] };

/** 📌️ A kind with no extra formats of its own: only the framework document pair is driven. */
const NO_KIND_PINS: IoProgramPins = { exports: [], imports: [], reach: [] };

type CatalogProbe = Readonly<{ plugins: readonly Readonly<{ pluginId: string; status: string }>[]; programs: readonly MatrixProgram[] }>;

async function openPalette(page: Page) {
  await dismissIntroduction(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  return input;
}

async function openProgram(page: Page, program: MatrixProgram, firstOfPlugin: boolean): Promise<{ windowIds: string[]; detail: string | null }> {
  const before = await windowIds(page);
  const input = await openPalette(page);
  if ((await input.count()) === 0) return { windowIds: [], detail: "command palette never opened" };
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(program.appId)?.[1] ?? program.pluginId);
  await page.waitForTimeout(1_200);
  const ids = [`spawn.${program.pluginId}.${program.appId}`, ...(firstOfPlugin ? [`spawn.${program.pluginId}`] : [])];
  let chosen = false;
  for (const id of ids) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) continue;
    chosen = true;
    await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
    break;
  }
  if (!chosen) {
    await page.keyboard.press("Escape");
    return { windowIds: [], detail: `no ${ids.join(" | ")} palette row` };
  }
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    const fresh = (await windowIds(page)).filter((id) => !before.includes(id));
    if (fresh.length > 0) {
      await page.waitForTimeout(3_500);
      return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), detail: null };
    }
    await page.waitForTimeout(250);
  }
  return { windowIds: [], detail: "palette row pressed, no new window" };
}

async function closeWindows(page: Page, ids: readonly string[]): Promise<void> {
  for (const id of ids) {
    await page.evaluate((windowId) => {
      const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
      const button = tab?.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab?.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]');
      if (button instanceof HTMLElement) button.click();
    }, id);
    await page.waitForTimeout(400);
  }
  await page.waitForTimeout(1_200);
}

/** 🗝️ The program-matrix row key of one creatable hub kind: its encoded choice names the dialect `s.<plugin>.<kind>`. */
export function hubKindKey(kind: Readonly<{ kindId: string; plugin: string; value: string }>): string {
  const dialect = (JSON.parse(kind.value) as { dialect?: { artifactKind?: string } }).dialect?.artifactKind ?? "";
  const match = /^s\.([^.]+)\.([^.@]+)/u.exec(dialect);
  return match ? `${match[1]}/${match[2]}` : `${kind.plugin}/${kind.kindId}`;
}

/** 🚪️ Drives one opened program's pins, then the framework document round trip, and returns its row fields. */
async function driveProgram(page: Page, recorders: Recorders, key: string, pluginId: string, pins: IoProgramPins, matrixPins: MatrixPins, dir: string) {
  const liveId = matrixPins.liveId;
  const example = await seatFirstExample(page);
  const railToggles = await unfoldActionsRail(page);
  const rail = (await readShell(page)).actions.map((id) => id.replace(/^action\./u, ""));
  const exports: Awaited<ReturnType<typeof runExport>>[] = [];
  for (const pin of pins.exports) exports.push(await runExport(page, recorders, pin, liveId, dir, pin.id));
  const reach: Awaited<ReturnType<typeof runReach>>[] = [];
  for (const verb of pins.reach) reach.push(await runReach(page, recorders, verb, liveId));
  const imports: Awaited<ReturnType<typeof runImport>>[] = [];
  const move = matrixVerbOf(matrixPins, key, pluginId);
  for (const pin of pins.imports) {
    const exportPin = pins.exports.find((candidate) => candidate.id === pin.from)!;
    imports.push(await runImport(page, recorders, pin, exportPin, exports.find((entry) => entry.id === pin.from)?.files[0], move, liveId, dir));
  }
  const document = await runDocumentTransfer(page, recorders, dir);
  return {
    example,
    railToggles,
    railRows: rail.length,
    missingRows: [...new Set([...pins.exports.map((pin) => pin.verb), ...pins.imports.map((pin) => pin.verb), ...pins.reach])].filter((verb) => !rail.includes(verb)),
    exports,
    imports,
    reach: reach.map((entry) => ({ verb: entry.verb, dispatched: entry.dispatched, dead: entry.dead, row: entry.press.row, submit: entry.press.submit, refusals: entry.press.refusals.slice(0, 3), notices: entry.press.notices.slice(0, 3) })),
    ioOk: exports.every((entry) => entry.ok) && imports.every((entry) => entry.ok),
    document,
  };
}

/** 🚪️ Runs the matrix and returns its report; the report is rewritten after every row, so a cancelled or crashed run keeps
 * every finished row and `resume` continues from them. Cancellation (the signal) ends the run after the current row. */
export async function runIoMatrix(repoRoot: string, options: IoMatrixOptions): Promise<IoMatrixReport> {
  const pins = readIoMatrixPins();
  const matrixPins = readMatrixPins();
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const out = join(outDir, "io-matrix.json");
  const log = (line: string): void => console.log(`[io-matrix] ${line}`);
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: !options.headed, args: ["--use-angle=metal"] });
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true, locale: options.locale === "de" ? "de-DE" : "en-US" });
  const page = await context.newPage();
  page.setDefaultNavigationTimeout(180_000);
  const recorders = await installRecorders(page);
  const previous: IoMatrixReport | null = options.resume && existsSync(out) ? (JSON.parse(readFileSync(out, "utf8")) as IoMatrixReport) : null;
  const report: IoMatrixReport = { baseUrl: options.baseUrl, hubUrl: options.hubUrl, tag: options.tag, locale: options.locale, started: previous?.started ?? new Date().toISOString(), rows: (previous?.rows ?? []).filter((row) => row.pass) };
  const done = new Set(report.rows.map((row) => row.key));
  const flush = (): void => writeFileSync(out, JSON.stringify(report, null, 1));
  const selected = (key: string, pluginId: string): boolean => (options.only.length === 0 || options.only.some((entry) => entry === pluginId || entry === key)) && !options.skip.some((entry) => entry === pluginId || entry === key);
  const boot = async (): Promise<string> => {
    await page.goto(options.baseUrl, { waitUntil: "commit", timeout: 300_000 });
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    await dismissIntroduction(page);
    await page.waitForTimeout(3_000);
    const seated = await seatLocale(page, options.locale);
    await page.keyboard.press("Escape").catch(() => undefined);
    return `${String(beacon)} ${seated}`;
  };
  const finishRow = (row: IoMatrixRow, faultCursor: number, consoleCursor: number): void => {
    writeFileSync(join(outDir, `${row.key.replace(/[^A-Za-z0-9]+/gu, "-")}.console.txt`), `${recorders.console.slice(consoleCursor).slice(-600).join("\n")}\n`);
    row.faultLines = recorders.faults.slice(faultCursor).filter((line) => !REFUSAL.test(line) || DEAD.test(line)).slice(0, 6);
    row.faultCount = (row.faultLines as string[]).length;
    report.rows.push(row);
    flush();
    const exports = (row.exports as { id: string; ok: boolean; files: SavedFile[] }[] | undefined) ?? [];
    const imports = (row.imports as { id: string; ok: boolean; roundTrip?: { verdict: string } }[] | undefined) ?? [];
    log(`${row.pass ? "PASS" : "FAIL"} ${row.key} document=${documentCellV1(row)} pinned=${row.pinned} exports=${exports.map((entry) => `${entry.id}:${entry.ok ? "ok" : "red"}(${entry.files.map((file) => `${file.bytes}B ${file.verdict.oracle}`).join(",")})`).join(" ")} imports=${imports.map((entry) => `${entry.id}:${entry.ok ? "ok" : "red"}:${entry.roundTrip?.verdict ?? "-"}`).join(" ")} reach=${row.reach.map((entry) => `${entry.verb}:${entry.dispatched ? "ok" : "dead"}`).join(" ")} faults=${String(row.faultCount)} ${String(row.detail ?? "")}`);
  };
  try {
    if (options.hubUrl === null) {
      report.census = { boot: await boot() };
      const began = Date.now();
      let probe: CatalogProbe | null = null;
      while (Date.now() - began < options.installBudgetMs && !options.signal.aborted) {
        probe = (await page.evaluate(() => ((window as unknown as { __semioOsCatalogProbe?: unknown }).__semioOsCatalogProbe ?? null) as never)) as CatalogProbe | null;
        if (probe !== null && probe.plugins.length > 0 && probe.plugins.every((entry) => ["loaded", "failed", "crashed", "available"].includes(entry.status))) break;
        await page.waitForTimeout(3_000);
      }
      if (probe === null) throw new Error("the shell exposed no window.__semioOsCatalogProbe");
      const programs = probe.programs.filter((program) => roleOf(program.appId) === "editor" && !matrixPins.excludedKinds.includes(keyOf(program).split("/").slice(0, 2).join("/")));
      report.census = { ...(report.census as object), plugins: probe.plugins.length, loaded: probe.plugins.filter((entry) => entry.status === "loaded").length, notLoaded: probe.plugins.filter((entry) => entry.status !== "loaded").map((entry) => `${entry.pluginId}:${entry.status}`), editors: programs.length, pinned: programs.filter((program) => pins.programs[keyOf(program)] !== undefined).length, pinsWithoutProgram: Object.keys(pins.programs).filter((key) => !programs.some((program) => keyOf(program) === key)) };
      log(`census ${JSON.stringify(report.census)}`);
      for (const program of programs) {
        const key = keyOf(program);
        if (options.signal.aborted) {
          report.cancelled = true;
          break;
        }
        if (done.has(key) || !selected(key, program.pluginId)) continue;
        const programPins = pins.programs[key];
        const faultCursor = recorders.faults.length;
        const consoleCursor = recorders.console.length;
        const opened = await openProgram(page, program, programs.find((entry) => entry.pluginId === program.pluginId)?.appId === program.appId);
        const row: IoMatrixRow = { key, appId: program.appId, pinned: programPins !== undefined, pass: false, reach: [], windowIds: opened.windowIds, detail: opened.detail };
        if (opened.windowIds.length > 0) {
          const driven = await driveProgram(page, recorders, key, program.pluginId, programPins ?? NO_KIND_PINS, matrixPins, join(outDir, key.replace(/[^A-Za-z0-9]+/gu, "-"))).catch((error: unknown) => ({ error: String(error).split("\n")[0]!.slice(0, 200) }));
          Object.assign(row, driven);
          row.pass = !("error" in driven) && driven.document.ok && driven.ioOk && driven.reach.every((entry) => entry.dispatched);
          await page.screenshot({ path: join(outDir, `${key.replace(/[^A-Za-z0-9]+/gu, "-")}.png`) }).catch(() => undefined);
          await closeWindows(page, opened.windowIds);
        }
        finishRow(row, faultCursor, consoleCursor);
        if (!row.pass) report.census = { ...(report.census as object), reboots: [...(((report.census as { reboots?: string[] }).reboots) ?? []), await boot()] };
      }
    } else {
      if (options.email === null || options.password === null) throw new Error("--hub needs OS_HUB_PROBE_EMAIL and OS_HUB_PROBE_PASSWORD in the environment");
      const sweep: HubDocumentSweepOptions = { baseUrl: options.baseUrl, locale: options.locale === "de" ? "de" : "en", email: options.email, password: options.password, spaceName: options.spaceName, kinds: [], sagaMs: 300_000, puzzleSagaMs: 900_000, reopen: false, cancel: null, profileDir: "", outDir, signal: options.signal };
      const probeRow: HubSweepRow = { kindId: "-", plugin: "-", pass: false, faults: [], notices: [] };
      const blocked = await openSweepSpace(page, sweep, probeRow);
      if (blocked) throw new Error(blocked);
      await unfoldActionsRail(page);
      await click(page, '[data-slot="window-action-pane"] [id="action.createArtifact"]');
      await page.waitForTimeout(2_000);
      const offered = await stagedKinds(page);
      report.census = { space: probeRow.spaceId, creatable: offered.map((kind) => kind.kindId) };
      log(`census ${JSON.stringify(report.census)}`);
      for (const kind of offered) {
        const key = hubKindKey(kind);
        if (options.signal.aborted) {
          report.cancelled = true;
          break;
        }
        if (done.has(key) || !selected(key, kind.plugin)) continue;
        const programPins = pins.programs[key];
        const faultCursor = recorders.faults.length;
        const consoleCursor = recorders.console.length;
        const row: IoMatrixRow = { key, appId: kind.kindId, pinned: programPins !== undefined, pass: false, reach: [] };
        const reopened = await openSweepSpace(page, sweep, probeRow);
        if (reopened) row.detail = reopened;
        else {
          await unfoldActionsRail(page);
          row.submitted = await createKind(page, kind.value, `IO Matrix ${kind.kindId} ${Date.now() % 100_000}`);
          const deadline = Date.now() + (kind.kindId.includes("puzzle") ? 900_000 : 300_000);
          let opened: string[] = [];
          while (Date.now() < deadline && opened.length === 0 && !options.signal.aborted) {
            opened = (await windowIds(page)).filter((id) => id !== "framework.window.table" && id !== "s-home-main");
            await page.waitForTimeout(1_000);
          }
          row.windowIds = opened;
          if (opened.length === 0) row.detail = "the created document never opened";
          else {
            await page.waitForTimeout(4_000);
            const driven = await driveProgram(page, recorders, key, kind.plugin, programPins ?? NO_KIND_PINS, matrixPins, join(outDir, key.replace(/[^A-Za-z0-9]+/gu, "-"))).catch((error: unknown) => ({ error: String(error).split("\n")[0]!.slice(0, 200) }));
            Object.assign(row, driven);
            row.pass = !("error" in driven) && driven.document.ok && driven.ioOk && driven.reach.every((entry) => entry.dispatched);
          }
          await page.screenshot({ path: join(outDir, `${key.replace(/[^A-Za-z0-9]+/gu, "-")}.png`) }).catch(() => undefined);
        }
        finishRow(row, faultCursor, consoleCursor);
      }
    }
  } catch (error) {
    report.fatal = String(error instanceof Error ? error.message : error).split("\n")[0]!.slice(0, 400);
    log(`FATAL ${report.fatal}`);
  } finally {
    report.finished = new Date().toISOString();
    report.faultsTail = recorders.faults.slice(-20);
    flush();
    await browser.close();
  }
  return report;
}

/** 🗃️ The framework document round trip of one row in a few words. */
export function documentCellV1(row: IoMatrixRow): string {
  const document = row.document as { export: { pressed: string; files: SavedFile[]; ok: boolean }; import: { outcome: string; roundTrip: { verdict: string; detail: string } | null } | null; ok: boolean } | undefined;
  if (document === undefined) return "-";
  const exported = document.export.files[0];
  const first = exported === undefined ? `export ${document.export.pressed === "ok" ? "wrote no file" : document.export.pressed}` : `export ${exported.bytes}B ${exported.verdict.ok ? "ok" : `red (${exported.verdict.detail})`}`;
  if (document.import === null) return `${document.ok ? "ok" : "red"}: ${first}`;
  return `${document.ok ? "ok" : "red"}: ${first}, import ${document.import.outcome}, re-export ${document.import.roundTrip ? `${document.import.roundTrip.verdict} (${document.import.roundTrip.detail})` : "none"}`;
}

/** 📝️ One markdown table row per program. */
export function ioMatrixTable(report: IoMatrixReport): string {
  const lines = ["| program | pass | document (framework) | exports | imports (round trip) | reach | faults | detail |", "|---|---|---|---|---|---|---|---|"];
  for (const row of report.rows) {
    const exports = ((row.exports as { id: string; ok: boolean; files: SavedFile[] }[] | undefined) ?? []).map((entry) => `${entry.id} ${entry.ok ? "ok" : "red"} ${entry.files.map((file) => `${file.verdict.format}/${file.verdict.oracle}: ${file.verdict.detail}`).join("; ") || "no file"}`).join(" · ");
    const imports = ((row.imports as { id: string; ok: boolean; roundTrip?: { verdict: string; detail: string }; detail?: string }[] | undefined) ?? []).map((entry) => `${entry.id} ${entry.ok ? "ok" : "red"} ${entry.roundTrip ? `${entry.roundTrip.verdict} (${entry.roundTrip.detail})` : (entry.detail ?? "")}`).join(" · ");
    const reach = row.reach.map((entry) => `${entry.verb} ${entry.dispatched ? "ok" : "dead"}`).join(" · ");
    lines.push(`| ${row.key} | ${row.pass ? "PASS" : "FAIL"} | ${documentCellV1(row).replaceAll("|", "\\|")} | ${exports.replaceAll("|", "\\|")} | ${imports.replaceAll("|", "\\|")} | ${reach} | ${String(row.faultCount ?? 0)} | ${String(row.detail ?? (row.missingRows as string[] | undefined)?.join(", ") ?? "").replaceAll("|", "\\|").slice(0, 160)} |`);
  }
  return `${lines.join("\n")}\n`;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `io-matrix --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--only <plugin|plugin/kind>,…] [--skip …]
 * [--space <name>] [--resume] [--install-budget-ms <n>] [--out <dir>] [--headed]` — runs the export/import matrix against the
 * serve `--serve` names (reused, or started and stopped by {@link withDevServe}; joined to `--hub` when given), writes
 * `io-matrix.json`, `table.md`, every downloaded file and one screenshot per row under `<out>/<tag>/`, publishes the
 * ONE `io-matrix` acceptance record (the kinds' export/import column and the live command-reachability column; `fail` when
 * either fails) and exits non-zero unless it passes. */
export async function runIoMatrixCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const serveUrl = flagValue(segments, "--serve");
  if (!serveUrl) throw new Error("usage: io-matrix --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--only …] [--skip …] [--space <name>] [--resume] [--install-budget-ms <n>] [--out <dir>] [--headed]");
  const hubUrl = flagValue(segments, "--hub") ?? null;
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const tag = flagValue(segments, "--tag") ?? `${hubUrl === null ? "local" : "hub"}-${locale}`;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  const blockedWithoutCredentials = hubUrl !== null && (!process.env.OS_HUB_PROBE_EMAIL || !process.env.OS_HUB_PROBE_PASSWORD);
  await withAcceptanceRecord(repoRoot, "io-matrix", async () => {
    if (blockedWithoutCredentials) {
      publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({ check: "io-matrix", status: "blocked", startedAt, measured: { hub: hubUrl ?? "" }, summary: { en: "--hub needs OS_HUB_PROBE_EMAIL and OS_HUB_PROBE_PASSWORD", de: "--hub braucht OS_HUB_PROBE_EMAIL und OS_HUB_PROBE_PASSWORD" } }));
      process.exitCode = 1;
      return;
    }
    await withDevServe(repoRoot, "io-matrix", { serveUrl, hubUrl: hubUrl ?? undefined, locale, signal: controller.signal, startedAt }, async (baseUrl) => {
      const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
      const report = await runIoMatrix(repoRoot, {
        baseUrl,
        hubUrl,
        email: process.env.OS_HUB_PROBE_EMAIL ?? null,
        password: process.env.OS_HUB_PROBE_PASSWORD ?? null,
        spaceName: flagValue(segments, "--space") ?? "IO Matrix",
        tag,
        locale,
        only: (flagValue(segments, "--only") ?? "").split(",").filter(Boolean),
        skip: (flagValue(segments, "--skip") ?? "").split(",").filter(Boolean),
        resume: segments.includes("--resume"),
        outDir,
        headed: segments.includes("--headed"),
        installBudgetMs: Number(flagValue(segments, "--install-budget-ms") ?? 600_000),
        signal: controller.signal,
      });
      writeFileSync(join(outDir, tag, "table.md"), ioMatrixTable(report));
      const pinned = report.rows.filter((row) => row.pinned);
      const passed = report.rows.filter((row) => row.pass).length;
      const documents = report.rows.filter((row) => (row.document as { ok?: boolean } | undefined)?.ok === true).length;
      const formats = pinned.filter((row) => row.pass).length;
      const failed = report.rows.filter((row) => !row.pass).map((row) => row.key);
      const unreachable = report.rows.length === 0 && /ERR_CONNECTION_REFUSED|ECONNREFUSED|Unable to connect/u.test(report.fatal ?? "");
      const reach = report.rows.flatMap((row) => row.reach.map((entry) => ({ ...entry, key: row.key })));
      const dead = reach.filter((entry) => !entry.dispatched).map((entry) => `${entry.key}:${entry.verb}`);
      const filtered = segments.includes("--only") || segments.includes("--skip");
      const reachOk = dead.length === 0 && (reach.length > 0 || filtered);
      const status = unreachable ? "blocked" : report.fatal || report.cancelled || report.rows.length === 0 ? "fail" : failed.length === 0 && reachOk ? "pass" : "fail";
      publishAcceptanceCheckResult(
        repoRoot,
        acceptanceCheckResult({
          check: "io-matrix",
          status,
          startedAt,
          measured: { locale, hub: hubUrl ?? "", programs: report.rows.length, passed, documents, pinned: pinned.length, formats, failed: failed.length, reachVerbs: reach.length, reachDispatched: reach.length - dead.length, reachDead: dead.length, fatal: Boolean(report.fatal), cancelled: Boolean(report.cancelled) },
          summary: {
            en: `${passed}/${report.rows.length} kinds export and import through s; document round trip ${documents}/${report.rows.length}, own formats ${formats}/${pinned.length}${failed.length ? `; failing: ${failed.slice(0, 8).join(", ")}` : ""}; ${reach.length - dead.length}/${reach.length} formerly batch-only verbs dispatch from their rail row${dead.length ? `; dead: ${dead.slice(0, 8).join(", ")}` : ""}${report.fatal ? `; fatal: ${report.fatal.slice(0, 160)}` : ""}`,
            de: `${passed}/${report.rows.length} Arten exportieren und importieren über s; Dokument-Rundreise ${documents}/${report.rows.length}, eigene Formate ${formats}/${pinned.length}${failed.length ? `; fehlgeschlagen: ${failed.slice(0, 8).join(", ")}` : ""}; ${reach.length - dead.length}/${reach.length} früher nur im Stapel ausführbare Befehle lösen aus ihrer Aktionszeile aus${dead.length ? `; tot: ${dead.slice(0, 8).join(", ")}` : ""}${report.fatal ? `; Abbruch: ${report.fatal.slice(0, 160)}` : ""}`,
          },
          evidence: [join(outDir, tag, "io-matrix.json"), join(outDir, tag, "table.md")],
        }),
      );
      console.log(`[io-matrix] === ${tag}: ${passed}/${report.rows.length} kinds pass, document round trip ${documents}/${report.rows.length}, own formats ${formats}/${pinned.length}, reach ${reach.length - dead.length}/${reach.length} → ${join(outDir, tag)} ===`);
      if (status !== "pass") process.exitCode = 1;
    });
  });
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
//#endregion 🔖️Matrix
