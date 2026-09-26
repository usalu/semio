import { existsSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { homedir } from "node:os";
import { join, posix, win32 } from "node:path";
import { BundleScript } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { cargoDirectories } from "../🟦️.ts";

/**
 * 🧾️ One file a unit's encoded dep-info tracks for freshness. `base` says what a relative `path` is joined onto
 * (the unit's package root or the build root); an absolute `path` is used as is, which is how Cargo records every
 * source reached through `..` or `#[path]` outside the package root.
 * https://github.com/rust-lang/cargo/blob/master/src/cargo/core/compiler/fingerprint/dep_info.rs
 */
export interface CargoDepInfoFile {
  readonly base: "package" | "build";
  readonly path: string;
  readonly checksum?: string;
}

/** 🔢️ Cargo's encoded dep-info header: a count of 1 an old reader rejects, the 0xff marker, then the format version. */
const DEP_INFO_HEADER = [0x01, 0x00, 0x00, 0x00, 0xff, 0x01] as const;

/**
 * 🔬️ Decodes Cargo's binary dep-info (`<unit>/fingerprint/dep-*`): header, file count, then per file the base kind,
 * the path bytes and an optional `(length, checksum)`, then the tracked environment. Throws on any other layout, so
 * a Cargo format change surfaces as an error instead of a silent pass.
 */
export function decodeCargoDepInfo(bytes: Uint8Array): CargoDepInfoFile[] {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength),
    text = new TextDecoder("utf-8", { fatal: true });
  let offset = 0;
  const take = (length: number): number => {
    if (offset + length > bytes.byteLength) throw new Error(`dep-info truncated at byte ${offset}`);
    const start = offset;
    offset += length;
    return start;
  };
  const u8 = (): number => view.getUint8(take(1)),
    u32 = (): number => view.getUint32(take(4), true),
    chunk = (): Uint8Array => {
      const length = u32();
      return bytes.subarray(take(length), offset);
    };
  if (DEP_INFO_HEADER.some((byte, index) => bytes[index] !== byte)) throw new Error("dep-info header is not Cargo's encoding version 1");
  take(DEP_INFO_HEADER.length);
  const files: CargoDepInfoFile[] = [];
  for (let remaining = u32(); remaining > 0; remaining--) {
    const kind = u8();
    if (kind > 1) throw new Error(`dep-info path kind ${kind} is unknown`);
    const path = text.decode(chunk());
    if (u8() === 1) {
      take(8);
      files.push({ base: kind === 0 ? "package" : "build", path, checksum: text.decode(chunk()) });
    } else files.push({ base: kind === 0 ? "package" : "build", path });
  }
  for (let remaining = u32(); remaining > 0; remaining--) {
    chunk();
    if (u8() === 1) chunk();
  }
  if (offset !== bytes.byteLength) throw new Error(`dep-info has ${bytes.byteLength - offset} trailing bytes`);
  return files;
}

/** 📐️ `path` lies inside `root` under the path rules of `platform` (Windows compares case-insensitively and accepts either separator). */
export function isInsideRoot(path: string, root: string, platform: NodeJS.Platform): boolean {
  const paths = platform === "win32" ? win32 : posix,
    fold = (value: string): string => (platform === "win32" ? paths.normalize(value).toLowerCase() : paths.normalize(value)),
    child = fold(path),
    parent = fold(root).replace(/[\\/]+$/, "");
  return child === parent || child.startsWith(parent + paths.sep);
}

/** 🚩️ Absolute dep-info paths outside every trusted root: sources another checkout compiled into this shared build-dir. */
export function foreignDepInfoPaths(files: readonly CargoDepInfoFile[], trustedRoots: readonly string[], platform: NodeJS.Platform): string[] {
  const paths = platform === "win32" ? win32 : posix;
  return files.filter((file) => paths.isAbsolute(file.path) && !trustedRoots.some((root) => isInsideRoot(file.path, root, platform))).map((file) => file.path);
}

/** 🏠️ The roots a unit of this repository may legitimately name: the checkout, both Cargo directories, the Cargo home and the rustup home. */
export function trustedBuildRoots(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string[] {
  const { target, build } = cargoDirectories(repoRoot, env);
  return [repoRoot, target, build, env.CARGO_HOME ?? join(homedir(), ".cargo"), env.RUSTUP_HOME ?? join(homedir(), ".rustup")];
}

/** 🧫️ One unit whose dep-info names a foreign source (`target` = `<triple>/<profile>` or `<profile>`). */
export interface ForeignDepInfoUnit {
  readonly target: string;
  readonly depInfo: string;
  readonly foreign: readonly string[];
}

/** 📊️ A provenance scan of a shared build-dir: dep-info files read per target, foreign units, and files Cargo's format no longer matches. */
export interface BuildDirProvenance {
  readonly scanned: Readonly<Record<string, number>>;
  readonly foreign: readonly ForeignDepInfoUnit[];
  readonly undecodable: readonly { readonly depInfo: string; readonly reason: string }[];
}

/** 🗺️ Every `<triple>/<profile>` or `<profile>` directory of a build-dir that holds units (`build/<package>/<hash>`). */
function buildDirTargets(buildDir: string): { readonly name: string; readonly path: string }[] {
  const entries = (path: string): string[] => (existsSync(path) ? readdirSync(path, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort() : []);
  return entries(buildDir).flatMap((first) =>
    existsSync(join(buildDir, first, "build")) ? [{ name: first, path: join(buildDir, first) }] : entries(join(buildDir, first)).filter((second) => existsSync(join(buildDir, first, second, "build"))).map((second) => ({ name: `${first}/${second}`, path: join(buildDir, first, second) })),
  );
}

/**
 * 🔎️ Reads every unit's dep-info in `buildDir` (Cargo's `build-dir-new-layout`: `<target>/build/<package>/<hash>/fingerprint/dep-*`)
 * and reports the units that track a source outside `trustedRoots`. Cancellable between units; reports progress per package.
 */
export function scanBuildDirProvenance(buildDir: string, trustedRoots: readonly string[], platform: NodeJS.Platform, signal: AbortSignal, onPackage?: (target: string, done: number, total: number) => void): BuildDirProvenance {
  const scanned: Record<string, number> = {},
    foreign: ForeignDepInfoUnit[] = [],
    undecodable: { depInfo: string; reason: string }[] = [];
  for (const target of buildDirTargets(buildDir)) {
    const packages = readdirSync(join(target.path, "build")).sort();
    scanned[target.name] = 0;
    packages.forEach((name, index) => {
      signal.throwIfAborted();
      const packageDir = join(target.path, "build", name);
      for (const unit of existsSync(packageDir) ? readdirSync(packageDir).sort() : []) {
        const fingerprint = join(packageDir, unit, "fingerprint");
        for (const file of existsSync(fingerprint) ? readdirSync(fingerprint).filter((entry) => entry.startsWith("dep-")).sort() : []) {
          const depInfo = join(fingerprint, file);
          scanned[target.name]!++;
          try {
            const paths = foreignDepInfoPaths(decodeCargoDepInfo(readFileSync(depInfo)), trustedRoots, platform);
            if (paths.length) foreign.push({ target: target.name, depInfo, foreign: paths });
          } catch (error) {
            if ((error as NodeJS.ErrnoException).code !== "ENOENT") undecodable.push({ depInfo, reason: error instanceof Error ? error.message : String(error) });
          }
        }
      }
      onPackage?.(target.name, index + 1, packages.length);
    });
  }
  return { scanned, foreign, undecodable };
}

/** 🧹️ Deletes the dep-info of every foreign unit: Cargo then finds no dep-info, treats the unit as stale and rebuilds it from this checkout. */
export function invalidateForeignUnits(units: readonly ForeignDepInfoUnit[], signal: AbortSignal): number {
  let removed = 0;
  for (const unit of units) {
    signal.throwIfAborted();
    if (existsSync(unit.depInfo)) {
      rmSync(unit.depInfo, { force: true });
      removed++;
    }
  }
  return removed;
}

/**
 * 🛡️ `cargo-provenance check|repair [--json]`: the shared build-dir gate. A scratch clone or second checkout that builds
 * into this repository's build-dir records its own absolute source paths in shared units; Cargo then reports those
 * units Fresh against this checkout although its sources changed (stale metadata, false greens). `check` fails when
 * any unit names a foreign source; `repair` deletes exactly those units' dep-info so Cargo rebuilds them. Scratch
 * clones must build into their own build-dir (`CARGO_BUILD_BUILD_DIR=<clone>/…`), never this one.
 */
export class CargoProvenanceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const mode = args[0];
    if (mode !== "check" && mode !== "repair") throw new Error("usage: cargo-provenance <check|repair> [--json]");
    const json = args.includes("--json"),
      controller = new AbortController(),
      cancel = (): void => controller.abort(new Error("Cargo provenance scan cancelled"));
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      const buildDir = cargoDirectories(this.repoRoot).build,
        report = scanBuildDirProvenance(buildDir, trustedBuildRoots(this.repoRoot), process.platform, controller.signal, json ? undefined : (target, done, total) => done % 250 === 0 || done === total ? console.log(`[cargo-provenance] ${target}: ${done}/${total} packages`) : undefined),
        perTarget = Object.fromEntries(Object.entries(report.scanned).map(([target, depInfos]) => [target, { depInfos, foreign: report.foreign.filter((unit) => unit.target === target).length }])),
        roots = [...new Set(report.foreign.flatMap((unit) => unit.foreign.map((path) => foreignRoot(path))))].sort(),
        removed = mode === "repair" ? invalidateForeignUnits(report.foreign, controller.signal) : 0;
      if (json) console.log(JSON.stringify({ buildDir, perTarget, foreignRoots: roots, foreign: report.foreign, undecodable: report.undecodable, removed }));
      else {
        for (const [target, counts] of Object.entries(perTarget)) console.log(`[cargo-provenance] ${target}: ${counts.depInfos} dep-info files, ${counts.foreign} foreign`);
        for (const root of roots) console.log(`[cargo-provenance] foreign source root: ${root}`);
        for (const unit of report.undecodable) console.log(`[cargo-provenance] undecodable ${unit.depInfo}: ${unit.reason}`);
        console.log(`[cargo-provenance] ${report.foreign.length} foreign units${mode === "repair" ? `; invalidated ${removed}` : ""}`);
      }
      if (report.undecodable.length || (mode === "check" && report.foreign.length)) process.exitCode = 1;
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
  }
}

/** 🏷️ The checkout a foreign path belongs to, for the report: everything before its first taxonomy segment or `src`. */
function foreignRoot(path: string): string {
  const segments = path.split(/[\\/]/),
    stop = segments.findIndex((segment, index) => index > 0 && (/^\p{Extended_Pictographic}/u.test(segment) || segment === "src"));
  return stop > 0 ? segments.slice(0, stop).join("/") : path;
}
