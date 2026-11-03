/** 🧰️ Shared helpers of the export tests: fixtures, scratch directories and the tool interface (qpdf, pdftoppm, ffprobe, python oracles). */
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { DrawingScene, ModelScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";

/** 📂️ The staffage root. */
export const STAFFAGE_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 📥️ A JSON fixture of `🧫️fixtures/<case>/🔣️.json`. */
export const fixture = <T>(name: string): T => JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🧫️fixtures", name, "🔣️.json"), "utf8")) as T;

/** 🖨️ The hand-made drawing scene fixture. */
export const drawingScene = (): DrawingScene => fixture<DrawingScene>("📕️pdf-reopen");

/** 📐️ The hand-made model scene fixture. */
export const modelScene = (): ModelScene => fixture<ModelScene>("📐️model-scene");

/** 🗑️ Runs `body` with a scratch directory that is removed afterwards. */
export async function withScratch<T>(body: (directory: string) => Promise<T> | T): Promise<T> {
  const directory = mkdtempSync(join(tmpdir(), "staffage-"));
  try {
    return await body(directory);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

/** 🔧️ Runs an external tool and returns stdout + stderr; throws with both when it exits non-zero. */
export function tool(command: string, args: readonly string[]): string {
  const result = spawnSync(command, [...args], { encoding: "utf8", maxBuffer: 1 << 28 });
  if (result.error) throw new Error(`${command}: ${result.error.message}`);
  if (result.status !== 0) throw new Error(`${command} ${args.join(" ")} exited ${result.status}\n${result.stdout}\n${result.stderr}`);
  return `${result.stdout}${result.stderr}`;
}

const availability = new Map<string, boolean>();

/** 🔍️ Requires an external tool: probes `-version`/`--version` (registered repo tools such as `ffmpeg` through `repo tools exec`, system tools from PATH), caches the answer per process and THROWS with the tool named and the way to provision it when it is missing, so a missing tool is a failing test and never a silent skip. Returns `true` so it can stand where a capability flag stood. */
export function requireTool(name: string): true {
  let answered = availability.get(name);
  if (answered === undefined) {
    const [command, args] = name === "ffmpeg" ? ["repo", ["tools", "exec", "--light", "ffmpeg", "--", "-version"]] : [name, [name === "ffprobe" ? "-version" : "--version"]];
    const result = spawnSync(command!, args as string[], { encoding: "utf8", timeout: 120_000 });
    answered = !result.error && result.status === 0;
    availability.set(name, answered);
  }
  if (!answered) throw new Error(`required tool "${name}" is not available: provision registered tools with \`repo tools setup all\` (ffmpeg, uv) or install the system tool (rsvg-convert from librsvg, qpdf, ffprobe with ffmpeg); these tests never skip silently`);
  return true;
}

/** 🔩️ Runs an external tool and returns stdout bytes. */
export const toolBytes = (command: string, args: readonly string[]): Buffer => execFileSync(command, [...args], { maxBuffer: 1 << 28 });

/** 🐍️ Runs a python oracle through uv with the given third-party packages, returns its stdout. */
export function python(script: string, packages: readonly string[], args: readonly string[] = []): string {
  return tool("uv", ["run", "--quiet", ...packages.flatMap((name) => ["--with", name]), "python", "-I", script, ...args]);
}

/** 🖼️ A decoded binary PPM (P6). */
export type Ppm = { readonly width: number; readonly height: number; readonly data: Uint8Array };

/** 🧾️ Parses `pdftoppm -ppm` output. */
export function parsePpm(bytes: Uint8Array): Ppm {
  const header = Buffer.from(bytes.subarray(0, 64)).toString("latin1");
  const match = /^P6\s+(\d+)\s+(\d+)\s+255\s/u.exec(header);
  if (!match) throw new Error("not a P6 PPM");
  return { width: Number(match[1]), height: Number(match[2]), data: bytes.subarray(match[0].length) };
}

/** 🎨️ The pixel at (x, y). */
export const pixel = (image: Ppm, x: number, y: number): readonly [number, number, number] => { const at = (y * image.width + x) * 3; return [image.data[at]!, image.data[at + 1]!, image.data[at + 2]!]; };
