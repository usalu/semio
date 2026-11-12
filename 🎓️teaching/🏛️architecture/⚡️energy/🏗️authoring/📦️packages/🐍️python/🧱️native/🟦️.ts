/** 🧱 Native libraries of the Python course environment for hosts where `pycairo` has no wheel (macOS): pinned conda-forge packages (`./🔣️.json`) fetched by exact URL and verified by SHA-256 BEFORE anything is unpacked, installed by a SHA-256-pinned `micromamba` into the machine cache of the repo, so that uv builds `pycairo` from the sdist of `../uv.lock` against them. Linux gets cairo and pango from the system packages of the repo bootstrap and Windows from the wheels of `../uv.lock`; there this module changes nothing. No Homebrew, no sudo, no profile edit, nothing inside the worktree.
 * @see ./🔣️.json
 * @see https://mamba.readthedocs.io/en/latest/user_guide/micromamba.html
 * @see https://pycairo.readthedocs.io/en/latest/ */
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, createReadStream, createWriteStream, existsSync, lstatSync, mkdirSync, openSync, readdirSync, readFileSync, readSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { chmod } from "node:fs/promises";
import { homedir } from "node:os";
import { delimiter, dirname, join } from "node:path";
import { pathToFileURL } from "node:url";

/** 📦️ One pinned conda-forge package of a host. */
export type NativePackage = { readonly host: string; readonly name: string; readonly version: string; readonly build: string; readonly subdir: string; readonly archive: string; readonly url: string; readonly sha256: string; readonly md5: string; readonly bytes: number };

/** 🔧️ One pinned `micromamba` binary of a host. */
export type NativeInstaller = { readonly platform: string; readonly architecture: string; readonly subdir: string; readonly name: string; readonly sha256: string; readonly bytes: number };

/** 📜️ The manifest `./🔣️.json`. */
export type NativeManifest = { readonly version: 1; readonly specs: readonly string[]; readonly micromamba: { readonly version: string; readonly release: string; readonly assets: readonly NativeInstaller[] }; readonly packages: readonly NativePackage[] };

/** 📣️ Progress of one step, `fraction` in 0..1 when the size is known. */
export type NativeProgress = { readonly step: string; readonly fraction?: number };

/** ⚙️ Options of {@link provisionNative}. */
export type NativeOptions = { readonly manifest?: NativeManifest; readonly cacheRoot?: string; readonly toolsRoot?: string; readonly platform?: NodeJS.Platform; readonly architecture?: string; readonly fetcher?: typeof fetch; readonly signal?: AbortSignal; readonly onProgress?: (progress: NativeProgress) => void };

/** 🧱 What a provisioned host adds to the environment of `uv`. */
export type NativeEnvironment = { readonly prefix: string; readonly environment: Readonly<Record<string, string>> };

const MANIFEST_FILE = join(import.meta.dir, "🔣️.json");

/** 🗄️ The per-machine cache root of the repo: `~/Library/Caches/semio` on macOS, `%LOCALAPPDATA%\semio\Cache` on Windows, `$XDG_CACHE_HOME/semio` or `~/.cache/semio` elsewhere. Never inside the worktree. */
export const machineCache = (platform: NodeJS.Platform = process.platform): string => (platform === "darwin" ? join(homedir(), "Library", "Caches", "semio") : platform === "win32" ? join(process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"), "semio", "Cache") : join(process.env.XDG_CACHE_HOME ?? join(homedir(), ".cache"), "semio"));

/** 🧰️ The `tools` partition of the repo (`<workspace>/.cache/b/t/m`, git-ignored build output, where the registry keeps `uv` and the other pinned binaries); the `micromamba` row of the catalog `provisioned` locator resolves inside it. */
export function toolsPartition(start: string = import.meta.dir): string {
  let directory = start;
  while (!existsSync(join(directory, "🧰️framework"))) {
    const parent = dirname(directory);
    if (parent === directory) throw new Error("Workspace root not found");
    directory = parent;
  }
  return join(directory, ".cache", "b", "t", "m");
}

/** 📖️ Reads and returns the manifest. */
export const readManifest = (file: string = MANIFEST_FILE): NativeManifest => JSON.parse(readFileSync(file, "utf8")) as NativeManifest;

/** 🧭️ The conda subdir of a host, or `undefined` when the host needs no native libraries from here. */
export const hostSubdir = (platform: NodeJS.Platform, architecture: string): string | undefined => (platform === "darwin" ? (architecture === "arm64" ? "osx-arm64" : architecture === "x64" ? "osx-64" : undefined) : undefined);

/** 🔑️ The SHA-256 (hex) of a file, streamed. */
export const sha256File = (file: string): Promise<string> => new Promise((resolve, reject) => {
  const hash = createHash("sha256");
  createReadStream(file).on("data", chunk => hash.update(chunk)).on("error", reject).on("end", () => resolve(hash.digest("hex")));
});

/** 📥️ Downloads `url` to `file` over a temporary name, refuses it unless its SHA-256 and size equal the pin, then renames it into place. A file that already verifies is kept. */
export async function fetchVerified(url: string, file: string, sha256: string, bytes: number, options: Pick<NativeOptions, "fetcher" | "signal" | "onProgress"> = {}): Promise<void> {
  if (!url.startsWith("https://")) throw new Error(`Refusing non-https source ${url}`);
  if (existsSync(file) && (await sha256File(file)) === sha256) return;
  const temporary = `${file}.part`;
  const response = await (options.fetcher ?? fetch)(url, { signal: options.signal });
  if (!response.ok || !response.body) throw new Error(`Download of ${url} failed: HTTP ${response.status}`);
  const hash = createHash("sha256");
  const sink = createWriteStream(temporary);
  let received = 0;
  try {
    for await (const chunk of response.body as unknown as AsyncIterable<Uint8Array>) {
      hash.update(chunk);
      received += chunk.byteLength;
      if (!sink.write(chunk)) await new Promise(resolve => sink.once("drain", resolve));
      options.onProgress?.({ step: url.slice(url.lastIndexOf("/") + 1), fraction: Math.min(1, received / bytes) });
    }
    await new Promise<void>((resolve, reject) => sink.end((error?: Error | null) => (error ? reject(error) : resolve())));
  } catch (error) {
    sink.destroy();
    rmSync(temporary, { force: true });
    throw error;
  }
  const actual = hash.digest("hex");
  if (actual !== sha256 || received !== bytes) {
    rmSync(temporary, { force: true });
    throw new Error(`Digest mismatch for ${url}: expected sha256 ${sha256} (${bytes} bytes), got ${actual} (${received} bytes)`);
  }
  renameSync(temporary, file);
}

/** 🏃️ Runs a program to completion, cancellable, and throws with its last output when it fails. */
function run(program: string, args: readonly string[], environment: NodeJS.ProcessEnv, signal?: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    let output = "";
    const child = spawn(program, args, { env: environment, stdio: ["ignore", "pipe", "pipe"], signal, windowsHide: true });
    const collect = (chunk: Buffer): void => { output = (output + chunk.toString()).slice(-4000); };
    child.stdout.on("data", collect);
    child.stderr.on("data", collect);
    child.on("error", reject);
    child.on("exit", code => (code === 0 ? resolve() : reject(new Error(`${program} exited with ${code}: ${output}`))));
  });
}

/** 🔏️ Every Mach-O file below `directory` (regular files whose first bytes are a Mach-O or fat magic). */
export function machOFiles(directory: string): string[] {
  const found: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) found.push(...machOFiles(path));
    else if (entry.isFile() && !lstatSync(path).isSymbolicLink() && isMachO(path)) found.push(path);
  }
  return found;
}

const isMachO = (file: string): boolean => {
  const descriptor = openSync(file, "r");
  try {
    const head = Buffer.alloc(4);
    return readSync(descriptor, head, 0, 4, 0) === 4 && ["cffaedfe", "cefaedfe", "cafebabe", "bebafeca"].includes(head.toString("hex"));
  } finally {
    closeSync(descriptor);
  }
};

/** 🔏️ Signs ad hoc every Mach-O file of a prefix whose signature the prefix relocation of micromamba invalidated (arm64 macOS kills such binaries with SIGKILL). Uses the `codesign` of the operating system. */
export async function resignInvalid(prefix: string, signal?: AbortSignal): Promise<readonly string[]> {
  const invalid: string[] = [];
  for (const file of machOFiles(prefix)) {
    try { await run("codesign", ["--verify", file], process.env, signal); } catch { invalid.push(file); }
  }
  if (invalid.length > 0) await run("codesign", ["--force", "--sign", "-", ...invalid], process.env, signal);
  return invalid;
}

/** 🧮️ The fingerprint of what a host's prefix must contain: the pinned installer and every pinned package row. */
export const fingerprint = (manifest: NativeManifest, subdir: string): string => createHash("sha256").update(JSON.stringify([manifest.specs, manifest.micromamba.assets.filter(asset => asset.subdir === subdir), manifest.packages.filter(row => row.host === subdir)])).digest("hex");

/** 🌐️ The environment that makes `uv` build `pycairo` against a prefix. */
export function buildEnvironment(prefix: string, base: NodeJS.ProcessEnv = process.env): Record<string, string> {
  const flags = (name: string, value: string): string => [base[name], value].filter(Boolean).join(" ");
  return {
    PATH: [join(prefix, "bin"), base.PATH ?? ""].join(delimiter),
    PKG_CONFIG: join(prefix, "bin", "pkg-config"),
    PKG_CONFIG_PATH: [join(prefix, "lib", "pkgconfig"), join(prefix, "share", "pkgconfig")].join(delimiter),
    LDFLAGS: flags("LDFLAGS", `-Wl,-rpath,${join(prefix, "lib")}`),
  };
}

/** 🧱 Makes the pinned native libraries available and returns the environment additions for `uv`. Idempotent: a prefix whose marker carries the current fingerprint is reused without any network access. Returns `undefined` on hosts that need nothing (Linux, Windows). Throws on a digest mismatch before unpacking anything. */
export async function provisionNative(options: NativeOptions = {}): Promise<NativeEnvironment | undefined> {
  const subdir = hostSubdir(options.platform ?? process.platform, options.architecture ?? process.arch);
  if (!subdir) return undefined;
  const manifest = options.manifest ?? readManifest();
  const root = join(options.cacheRoot ?? machineCache(options.platform), "teaching-python", "native");
  const prefix = join(root, "prefix");
  const marker = join(prefix, ".semio-native");
  const print = fingerprint(manifest, subdir);
  const done = (): NativeEnvironment => ({ prefix, environment: buildEnvironment(prefix) });
  if (existsSync(marker) && readFileSync(marker, "utf8").trim() === print) return done();
  const installer = manifest.micromamba.assets.find(asset => asset.subdir === subdir);
  const packages = manifest.packages.filter(row => row.host === subdir);
  if (!installer || packages.length === 0) throw new Error(`The native manifest has no rows for ${subdir}`);
  const downloads = join(root, "downloads");
  mkdirSync(downloads, { recursive: true });
  const binary = join(options.toolsRoot ?? toolsPartition(), "native", "micromamba", manifest.micromamba.version, `${installer.platform}-${installer.architecture}-${installer.sha256.slice(0, 16)}`, process.platform === "win32" ? "micromamba.exe" : "micromamba");
  mkdirSync(join(binary, ".."), { recursive: true });
  const verified = { fetcher: options.fetcher, signal: options.signal, onProgress: options.onProgress };
  await fetchVerified(`${manifest.micromamba.release}/${installer.name}`, binary, installer.sha256, installer.bytes, verified);
  await chmod(binary, 0o755);
  for (const row of packages) await fetchVerified(row.url, join(downloads, row.archive), row.sha256, row.bytes, verified);
  const explicit = join(root, `explicit-${subdir}.txt`);
  writeFileSync(explicit, ["@EXPLICIT", ...packages.map(row => `${pathToFileURL(join(downloads, row.archive)).href}#${row.md5}`), ""].join("\n"));
  rmSync(prefix, { recursive: true, force: true });
  options.onProgress?.({ step: "install" });
  const mamba = { ...process.env, MAMBA_ROOT_PREFIX: join(root, "mamba"), CONDA_PKGS_DIRS: join(root, "pkgs") };
  try {
    await run(binary, ["create", "--no-rc", "--yes", "--quiet", "--prefix", prefix, "--file", explicit], mamba, options.signal);
    if (subdir.startsWith("osx")) await resignInvalid(prefix, options.signal);
    if (!existsSync(join(prefix, "bin", "pkg-config")) || !existsSync(join(prefix, "lib", "pkgconfig", "cairo.pc"))) throw new Error(`The installed prefix ${prefix} has no pkg-config or cairo.pc`);
  } catch (error) {
    rmSync(prefix, { recursive: true, force: true });
    throw error;
  }
  writeFileSync(marker, `${print}\n`);
  return done();
}

if (import.meta.main) {
  const controller = new AbortController();
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => controller.abort());
  const seen = new Map<string, number>();
  const report = ({ step, fraction }: NativeProgress): void => {
    const decile = fraction === undefined ? -1 : Math.floor(fraction * 10);
    if (seen.get(step) === decile) return;
    seen.set(step, decile);
    process.stderr.write(`[native] ${step}${fraction === undefined ? "" : ` ${decile * 10}%`}\n`);
  };
  provisionNative({ signal: controller.signal, onProgress: report }).then(result => {
    process.stdout.write(`${JSON.stringify(result ?? null)}\n`);
  }, (error: Error) => {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 1;
  });
}
