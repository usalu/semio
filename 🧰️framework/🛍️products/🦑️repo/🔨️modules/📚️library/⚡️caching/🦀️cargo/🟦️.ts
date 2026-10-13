import { buildModePolicy, readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { admitReleasePipelineV1, admitReleasePlanV1, type ReleaseGenerationV1, type ReleaseJobV1, type ReleasePipelineV1, type ReleasePlanV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { ownerEnvironmentV1 } from "../📦️artifacts/📋️native-orchestration/🟦️.ts";
import { repoCacheDirectory } from "../🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, SHIPPED_COMPONENT_PROFILE } from "../../../../../💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";

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

//#region 🧩️ReleaseComponentPlan
/** 🎚️ The one component profile a release compiles (ticket 26/09/23 wave A2): the shipped `wasm-release`, staged into the
 * schema-owned deliverable directory (`dist/components`) that describe, receipts, materialization and the trusted catalog read. */

//#endregion

/** 🧱️ One Cargo workspace and the plugin and extension component packages it owns. */
export type ComponentWorkspaceV1 = Readonly<{ manifest: string; packages: readonly string[]; manifests: Readonly<Record<string, string>>; pluginIds: Readonly<Record<string, string>> }>;

/** 🔎️ Every plugin and extension component of the repository, grouped by the workspace that owns it, read from the manifests the Nx plugin's component targets read
 * (`[package.metadata.component]` with a `plugin`/`extension` component kind; the owning workspace is `package.workspace` or the nearest `[workspace]` ancestor).
 * `workspaces` restricts the result to those workspace directories. */
export function componentWorkspacesV1(repoRoot: string, workspaces?: readonly string[]): ComponentWorkspaceV1[] {
  const skipped = new Set<string>([...(JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8")).generatedDirectories as string[]), ".🧬semio", "temp"]);
  const owners = new Map<string, Map<string, string>>(), pluginIds = new Map<string, string>(), toml = (path: string): any => Bun.TOML.parse(readFileSync(path, "utf8"));
  const workspaceOf = (manifest: string, declared: unknown): string => {
    if (typeof declared === "string") return resolve(dirname(manifest), declared, "Cargo.toml");
    for (let directory = dirname(manifest); ; directory = dirname(directory)) {
      const candidate = join(directory, "Cargo.toml");
      if (existsSync(candidate) && toml(candidate).workspace) return candidate;
      if (dirname(directory) === directory) return manifest;
    }
  };
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (entry.isDirectory() && !skipped.has(entry.name)) walk(join(directory, entry.name));
      else if (entry.isFile() && entry.name === "Cargo.toml") {
        const manifest = join(directory, entry.name), parsed = toml(manifest), metadata = parsed.package?.metadata;
        if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
        const workspace = workspaceOf(manifest, parsed.package.workspace);
        owners.set(workspace, (owners.get(workspace) ?? new Map()).set(parsed.package.name, manifest));
        pluginIds.set(parsed.package.name, String(metadata.component.package).replace(/^semio:/u, ""));
      }
    }
  };
  walk(repoRoot);
  const selected = workspaces?.map((workspace) => resolve(repoRoot, workspace, "Cargo.toml"));
  return [...owners].filter(([manifest]) => !selected || selected.includes(manifest)).sort(([left], [right]) => (selected ? selected.indexOf(left) - selected.indexOf(right) : 0) || owners.get(right)!.size - owners.get(left)!.size || left.localeCompare(right))
    .map(([manifest, packages]) => ({ manifest, packages: [...packages.keys()].sort(), manifests: Object.fromEntries([...packages].sort(([left], [right]) => left.localeCompare(right))), pluginIds: Object.fromEntries([...packages.keys()].sort().map((name) => [name, pluginIds.get(name)!])) }));
}

/** 🗺️ The release driver's plan: one pass per component workspace and release profile, staging each component into its profile's deliverable directory.
 * `pluginIds` restricts the passes to exactly those components (a dev launch builds only its closure); unknown ids are refused. */
export function componentPassPlanV1(repoRoot: string, pluginIds?: ReadonlySet<string>, mode: BuildMode = readBuildMode(process.env.SEMIO_BUILD_MODE)): ReleasePlanV1 {
  const workspaces = componentWorkspacesV1(repoRoot), known = new Set(workspaces.flatMap((workspace) => Object.values(workspace.pluginIds)));
  const unknown = [...pluginIds ?? []].filter((id) => !known.has(id));
  if (unknown.length) throw Error(`Release plan names no discovered component: ${unknown.join(", ")}`);
  return admitReleasePlanV1({ version: 1, passes: [buildModePolicy(mode).wasmProfile].flatMap((profile) => workspaces.flatMap((workspace, index) => {
    const packages = workspace.packages.filter((name) => !pluginIds || pluginIds.has(workspace.pluginIds[name]!));
    return packages.length ? [{ id: `${profile}-${String(index).padStart(2, "0")}`, manifest: workspace.manifest, packages: packages.map((name) => ({ name, manifest: workspace.manifests[name]! })), target: "wasm32-wasip2", profile, output: COMPONENT_PROFILE_POLICIES[profile].directory }] : [];
  })) });
}

/** 🚀️ A product's release plan: one Cargo pass per component workspace restricted to `components` (every discovered component when omitted) and the
 * pipeline's jobs with absolute working directories, plus the hook that gives each job its owner policies inside the driver's generation. */
export function releasePipelinePlanV1(repoRoot: string, pipeline: ReleasePipelineV1, components?: ReadonlySet<string>): Readonly<{ plan: ReleasePlanV1; jobEnvironment: (job: ReleaseJobV1, generation: ReleaseGenerationV1) => Readonly<Record<string, string>> }> {
  const admitted = admitReleasePipelineV1(pipeline), passes = componentPassPlanV1(repoRoot, components).passes, policies = new Map(admitted.jobs.map((job) => [job.id, job.policies]));
  const policy = buildModePolicy(readBuildMode(process.env.SEMIO_BUILD_MODE)), profile = (value: string): string => value === "wasm-release" || value === "wasm-prototype" ? policy.wasmProfile : value === "release" || value === "prototype" ? policy.cargoProfile : value;
  const jobs: ReleaseJobV1[] = admitted.jobs.map((job) => ({ id: job.id, cwd: resolve(repoRoot, job.root), command: job.command, after: job.after, profile: profile(job.profile), target: job.target, ...(job.lane === undefined ? {} : { lane: profile(job.lane) }) }));
  return Object.freeze({ plan: admitReleasePlanV1({ version: 1, passes, jobs, ...(admitted.stageAfter ? { stageAfter: admitted.stageAfter } : {}) }), jobEnvironment: (job: ReleaseJobV1, generation: ReleaseGenerationV1) => ownerEnvironmentV1(job.cwd, policies.get(job.id) ?? [], generation.build) });
}
