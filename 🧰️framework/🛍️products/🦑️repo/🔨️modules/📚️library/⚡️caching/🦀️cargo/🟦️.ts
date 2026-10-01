import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { repoCacheDirectory } from "../🟦️.ts";

/** 🏛️ Caller-owned values forwarded to a canonical execution without reading ambient state. */
export interface CanonicalExecutionEnvironment {
  readonly [key: string]: string | undefined;
}

/** 🔒️ Gives canonical Cargo producers one absolute deliverable and compiler-unit root. */
export function canonicalArchitectureEnvironment(repoRoot: string, env: CanonicalExecutionEnvironment): CanonicalExecutionEnvironment {
  const cargoRoot = resolve(repoRoot, env.CARGO_TARGET_DIR ?? repoCacheDirectory(repoRoot, "cargo", "canonical-architecture"));
  return { ...env, CARGO_TARGET_DIR: cargoRoot, CARGO_BUILD_BUILD_DIR: cargoRoot };
}

/** 🔒️ The binding generator must have the same identity as the locked Rust crate. */
export function wasmBindgenVersion(lock: string): string {
  const versions = [...lock.matchAll(/^name = "wasm-bindgen"\r?\nversion = "([^"]+)"$/gm)].map((match) => match[1]!);
  if (versions.length !== 1) throw new Error("Cargo.lock must resolve exactly one wasm-bindgen version");
  return versions[0]!;
}

/** 🗃️ Where Cargo writes for this repository: uplifted deliverables (`target`) and shared intermediates (`build`). */
export interface CargoDirectories {
  readonly target: string;
  readonly build: string;
}

const resolved = new Map<string, CargoDirectories>();

/**
 * 🧭️ Resolves Cargo's effective directories with Cargo's own precedence (environment over the repository
 * `.cargo/config.toml`), so every script reads deliverables exactly where Cargo wrote them.
 * https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir
 * https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-dir
 */
export function cargoDirectories(repoRoot: string, env: NodeJS.ProcessEnv = process.env): CargoDirectories {
  const overrideTarget = env.CARGO_TARGET_DIR ?? env.CARGO_BUILD_TARGET_DIR, overrideBuild = env.CARGO_BUILD_BUILD_DIR;
  const key = `${resolve(repoRoot)}\0${overrideTarget ?? ""}\0${overrideBuild ?? ""}`;
  const cached = resolved.get(key);
  if (cached) return cached;
  const path = join(repoRoot, ".cargo", "config.toml");
  const build = (existsSync(path) ? Bun.TOML.parse(readFileSync(path, "utf8")) as { build?: { "target-dir"?: string; "build-dir"?: string } } : {}).build ?? {};
  const target = resolve(repoRoot, overrideTarget ?? build["target-dir"] ?? "target");
  const directories = { target, build: overrideBuild ? resolve(repoRoot, overrideBuild) : build["build-dir"] ? resolve(repoRoot, build["build-dir"]) : target };
  resolved.set(key, directories);
  return directories;
}

/** 🎯️ Cargo's uplifted deliverable root (`<target>/<triple?>/<profile>/…`). */
export function cargoTargetDirectory(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string {
  return cargoDirectories(repoRoot, env).target;
}

/** 🏗️ Cargo's shared intermediate root (`<build>/<triple?>/<profile>/build/<package>/<hash>/…`). */
export function cargoBuildDirectory(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string {
  return cargoDirectories(repoRoot, env).build;
}

/** 🦀️ The canonical component publication profile used by compiled descriptor receipts. */
export const WASM_PUBLICATION_PROFILE = "wasm-release";

/** 🔐 Selects the canonical publication profile from the caller's actual Cargo deliverable root. */
export function publicationWasmPath(repoRoot: string, wasmOut: string): string {
  return join(cargoTargetDirectory(repoRoot), "wasm32-wasip2", WASM_PUBLICATION_PROFILE, wasmOut);
}
