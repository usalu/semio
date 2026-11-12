/** 🧰️ Shared helpers of the export tests: fixtures, scratch directories and the tool interface of the repo registry (`uv`). The third-party oracles themselves are in `./🐍️.ts`. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene, ModelScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { DWG_WASM_URL } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";

/** 📂️ The staffage root. */
export const STAFFAGE_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 📥️ A JSON fixture of `🧫️fixtures/<case>/🔣️.json`. */
export const fixture = <T>(name: string): T => JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🧫️fixtures", name, "🔣️.json"), "utf8")) as T;

/** 🖨️ The hand-made drawing scene fixture. */
export const drawingScene = (): DrawingScene => fixture<DrawingScene>("📕️pdf-reopen");

/** 📐️ The hand-made model scene fixture. */
export const modelScene = (): ModelScene => fixture<ModelScene>("📐️model-scene");

/** 🚫️ The asset with one scale declared unavailable (limiting feature of 0.05 m, alternatives: the scales it still has authored and the symbol `position-dot`), for the tests of unavailable resolutions, whose reasons are stated in both languages. */
export function withUnavailableScale(asset: Asset, denominator: 500 | 200 | 100 | 50): Asset {
  const authored = (["200", "100", "50"] as const).filter((key) => key !== `${denominator}` && asset.representations[key]?.status === "authored");
  return {
    ...asset,
    representations: {
      ...asset.representations,
      [`${denominator}`]: {
        status: "unavailable",
        denominator,
        cause: "below-printable-feature",
        reason: { en: "The limbs of the figure fall below the feature limit of the print profile.", de: "Die Gliedmaßen der Figur unterschreiten die Mindestbreite des Druckprofils." },
        limitingFeature: { en: "Limb width", de: "Gliedmaßenbreite" },
        limitingSizeM: 0.05,
        alternatives: [...authored.map((key) => ({ kind: "scale" as const, denominator: Number(key) as 200 | 100 | 50 })), { kind: "symbol" as const, symbol: "position-dot" }],
      },
    },
  };
}

/** 🗑️ Runs `body` with a scratch directory that is removed afterwards. */
export async function withScratch<T>(body: (directory: string) => Promise<T> | T): Promise<T> {
  const directory = mkdtempSync(join(tmpdir(), "staffage-"));
  try {
    return await body(directory);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

const REPO_TOOLS: ReadonlySet<string> = new Set(["uv"]);

const UV_EXECUTABLE = process.platform === "win32" ? "uv.exe" : "uv";

/** 📍️ The `uv` of the registry's `provisioned` locator, read without the project graph: `SEMIO_TOOL_UV`, then `<repo>/.cache/b/t/m/native/uv/<version>/<host>-<digest>/uv` (the folder `repo tools setup toolchains uv` fills), then `~/.local/bin`; `undefined` when none exists. */
const provisionedUv = (): string | undefined => {
  const given = process.env.SEMIO_TOOL_UV;
  if (given && existsSync(given)) return given;
  const base = resolve(STAFFAGE_ROOT, "../../../.cache/b/t/m/native/uv");
  const found = existsSync(base) ? readdirSync(base).flatMap((version) => readdirSync(join(base, version)).map((host) => join(base, version, host, UV_EXECUTABLE))).filter((path) => existsSync(path)) : [];
  return found[0] ?? [join(homedir(), ".local", "bin", UV_EXECUTABLE)].find((path) => existsSync(path));
};

/** 🧭️ Resolves a command through the repo tool registry: its provisioned locator first (no graph read, no capacity queue), `repo tools exec` when only the registry can find it, otherwise PATH. */
const resolved = (command: string, args: readonly string[]): [string, string[]] => {
  if (command === "uv") {
    const path = provisionedUv();
    if (path) return [path, [...args]];
  }
  return REPO_TOOLS.has(command) ? ["repo", ["tools", "exec", "--light", command, "--", ...args]] : [command, [...args]];
};

/** 🔧️ Runs an external tool (with extra environment variables) and returns stdout + stderr; throws with both when it exits non-zero. */
export function tool(command: string, args: readonly string[], streams: "both" | "stdout" = "both", environment: Readonly<Record<string, string>> = {}): string {
  const [program, argv] = resolved(command, args);
  const result = spawnSync(program, argv, { encoding: "utf8", maxBuffer: 1 << 28, env: { ...process.env, ...environment } });
  if (result.error) throw new Error(`${command}: ${result.error.message}`);
  if (result.status !== 0) throw new Error(`${command} ${args.join(" ")} exited ${result.status}\n${result.stdout}\n${result.stderr}`);
  return streams === "stdout" ? result.stdout : `${result.stdout}${result.stderr}`;
}

const availability = new Map<string, boolean>();

/** 🔍️ Requires a registered tool: probes `--version` through `repo tools exec`, caches the answer per process and THROWS with the tool named and the way to provision it when it is missing, so a missing tool is a failing test and never a silent skip. Returns `true` so it can stand where a capability flag stood. The python oracles need only `uv` (provisioned and pinned by the registry); everything else they use comes from the hash-locked wheels of `./🐍️oracles`. */
export function requireTool(name: "uv"): true {
  let answered = availability.get(name);
  if (answered === undefined) {
    const [command, args] = resolved(name, ["--version"]);
    const result = spawnSync(command, args, { encoding: "utf8", timeout: 120_000 });
    answered = !result.error && result.status === 0;
    availability.set(name, answered);
  }
  if (!answered) throw new Error(`required tool "${name}" is not available: provision it with \`repo tools setup toolchains ${name}\`; these tests never skip silently`);
  return true;
}

/** 🖼️ A decoded binary PPM (P6). */
export type Ppm = { readonly width: number; readonly height: number; readonly data: Uint8Array };

/** 🧾️ Parses the binary PPM that the PDF oracle writes. */
export function parsePpm(bytes: Uint8Array): Ppm {
  const header = Buffer.from(bytes.subarray(0, 64)).toString("latin1");
  const match = /^P6\s+(\d+)\s+(\d+)\s+255\s/u.exec(header);
  if (!match) throw new Error("not a P6 PPM");
  return { width: Number(match[1]), height: Number(match[2]), data: bytes.subarray(match[0].length) };
}

/** 🎨️ The pixel at (x, y). */
export const pixel = (image: Ppm, x: number, y: number): readonly [number, number, number] => { const at = (y * image.width + x) * 3; return [image.data[at]!, image.data[at + 1]!, image.data[at + 2]!]; };

const DWG_CRATE = join(STAFFAGE_ROOT, "🔨️modules/📤️export/🖊️dwg/📦️packages/🦀️rust");

const newestSource = (directory: string): number =>
  readdirSync(directory, { withFileTypes: true }).reduce((newest, entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) return entry.name === "target" ? newest : Math.max(newest, newestSource(path));
    return /\.(rs|toml)$/u.test(entry.name) ? Math.max(newest, statSync(path).mtimeMs) : newest;
  }, 0);

const DWG_LOCK = join(tmpdir(), "staffage-dwg-wasm.lock");
const BUILD_LIMIT_MS = 20 * 60_000;

const upToDate = (wasm: string): boolean => existsSync(wasm) && statSync(wasm).mtimeMs >= newestSource(DWG_CRATE);

/** 🔒️ Holds the build lock of the module while `body` runs: test files run in parallel workers, and two `repo cargo` runs on one lane refuse each other, so the second waits for the first and then finds the module built. A lock older than the build limit is taken over. */
function whileBuilding<T>(body: () => T): T {
  const started = Date.now();
  for (;;) {
    try {
      mkdirSync(DWG_LOCK);
      break;
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      if (Date.now() - statSync(DWG_LOCK).mtimeMs > BUILD_LIMIT_MS || Date.now() - started > BUILD_LIMIT_MS) rmSync(DWG_LOCK, { recursive: true, force: true });
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 500);
    }
  }
  try {
    return body();
  } finally {
    rmSync(DWG_LOCK, { recursive: true, force: true });
  }
}

/** 🕸️ The DWG writer module `🖊️dwg/staffage_dwg.wasm` (git-ignored build output): built through the crate's own target (`repo ./📜️script.ts wasm`, which compiles with the repo Cargo ingress for `wasm32-unknown-unknown`) when it is missing or older than a source of the crate, so the tests need no manual step. THROWS with the build output when the build fails, never skips. Returns the file. */
export function ensureDwgWasm(): URL {
  const wasm = fileURLToPath(DWG_WASM_URL);
  if (upToDate(wasm)) return DWG_WASM_URL;
  return whileBuilding(() => {
    if (upToDate(wasm)) return DWG_WASM_URL;
    const result = spawnSync("repo", ["./📜️script.ts", "wasm"], { cwd: DWG_CRATE, encoding: "utf8", maxBuffer: 1 << 28, shell: process.platform === "win32" });
    if (result.error || result.status !== 0) throw new Error(`the DWG module could not be built with \`repo ./📜️script.ts wasm\` in ${DWG_CRATE}: ${result.error?.message ?? ""}\n${result.stdout}\n${result.stderr}`);
    return DWG_WASM_URL;
  });
}
