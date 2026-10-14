import { acquireCompilerCapacityLeaseV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../🏗️build/🎟️capacity/🟦️.ts";
import { buildModePolicy, type BuildMode } from "../../🏗️build/🎚️mode/🟦️.ts";
import { acquireCargoBuildLeaseV1 } from "../🏗️native-build/🔒️lease/🟦️.ts";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { stageArtifacts } from "../📤️publication/🟦️.ts";
import { collectArtifactFiles } from "../🗂️files/🟦️.ts";
import { runOwnedCommand } from "../../🎛️owned-execution/🟦️.ts";
import { admitCargoSelectionV1, type CargoSourceBoundaryPolicyV1 } from "../🏗️native-build/📥️source/🎟️admission/🟦️.ts";

export type WasmPackWebPkg = {
  name: string;
  version?: string;
  files: string[];
  main: string;
  module: string;
  types: string;
  sideEffects?: string[];
};

/** 🪪️ Identifies canonical mode artifacts from any package projection without relying on its link location. */
export type WasmPackageOwnerV1=Readonly<{version:1;sourceOwner:string;mode:BuildMode;output:string}>;

/** 🕸️ Binds the exact source owner, tools, mode, compiler storage and deadline. */
export type WasmBuildPolicyV1 = Readonly<{ version: 1; cwd: string; workspaceRoot: string; sourceBoundary: CargoSourceBoundaryPolicyV1; compilerConfiguration?: string; sourceOwner: string; mode: BuildMode; artifactDirectory: string; buildDirectory: string; leaseDirectory: string; budgetMs: number; packageName: string; searchPath: string; bindgen: Readonly<{ command: string; args: readonly string[] }>; wasmPack: Readonly<{ command: string; args: readonly string[] }> }>;
/** 📦️ Selects one owned browser package publication without discovering repository policy: `rsDir` owns the published package,
 * `componentDir` is the `🧩️component` cdylib crate wasm-bindgen links (a library crate is never a cdylib). */
export type WasmPackWebBuildOptions = Readonly<{ storage?:import("../🏗️native-build/💾️storage/🟦️.ts").NativeStorageReservationV1; control: CompilerAdmissionControl; rsDir: string; componentDir: string; logPrefix: string; pkg: WasmPackWebPkg; wasmBaseName: string; outputDirectory?: string; threads?: boolean; cargoFeatures?: readonly string[]; noDefaultFeatures?: boolean; prototypeOptimizedCrates?: readonly string[] }>;
/** 🔌️ Executes an explicitly selected compiler with bounded cancellation; `admit` refuses the exact Cargo selection before it compiles. */
export type WasmBuildExecutionPortV1 = Readonly<{ run: (command: string, args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; budgetMs: number; label: string; signal: AbortSignal }>) => Promise<void>; admit: (args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; policy: WasmBuildPolicyV1; storage?:import("../🏗️native-build/💾️storage/🟦️.ts").NativeStorageReservationV1; control: CompilerAdmissionControl }>) => Promise<void> }>;

/** 📏️ Admits one wasm-bindgen Cargo selection through the one Source admission owner, keyed by the policy's repository root.
 * @see ../🏗️native-build/📥️source/🎟️admission/🟦️.ts */
export function admitWasmCargoSourcesV1(args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; policy: WasmBuildPolicyV1; storage?:import("../🏗️native-build/💾️storage/🟦️.ts").NativeStorageReservationV1; control: CompilerAdmissionControl }>): Promise<void> {
  if(!options.storage)throw Error("Original enforcing WASM compiler store required");return admitCargoSelectionV1({ workspaceRoot: options.policy.workspaceRoot, cwd: options.cwd, args, environment: options.environment, sourceBoundary: options.policy.sourceBoundary,storage:options.storage }, options.control);
}
const wasmPolicySchema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
function admitWasmPolicy(value: unknown): WasmBuildPolicyV1 {
  const errors = validateJsonSchemaSubset(wasmPolicySchema, value);
  if (errors.length) throw Error(`Invalid WASM compiler policy: ${errors.join("; ")}`);
  return value as WasmBuildPolicyV1;
}
/** 📍️ Admits one portable package's declared compiler owner and mode-qualified canonical output. */
export function readWasmPackageOwnerV1(value:unknown):WasmPackageOwnerV1 {
 const errors=validateJsonSchemaSubset(wasmPolicySchema.definitions.PackageOwner,value);
 const owner=value as WasmPackageOwnerV1;
 if(errors.length||owner.output!==`dist/wasm/${owner.mode}`)throw Error("Invalid WASM package owner");
 return owner;
}

/** 🔐️ Refuses missing compiler policy or a policy belonging to a different source owner. */
export function readWasmBuildPolicyV1(environment: Readonly<Record<string,string|undefined>>, cwd: string): WasmBuildPolicyV1 {
  if (!environment.SEMIO_WASM_BUILD_POLICY) throw Error("Explicit WASM compiler policy required");
  const policy = admitWasmPolicy(JSON.parse(environment.SEMIO_WASM_BUILD_POLICY));
  if (resolve(policy.cwd) !== resolve(cwd)) throw Error("WASM compiler policy belongs to a different owner");
  return policy;
}

/** 📦️Collect wasm-bindgen snippet paths produced by threaded builds. */
function wasmPackSnippetFiles(pkgDir: string): string[] {
  const snippetsDir = join(pkgDir, "snippets");
  if (!existsSync(snippetsDir)) return [];
  const out: string[] = [];
  const walk = (dir: string, prefix: string) => {
    for (const entry of readdirSync(dir)) {
      const rel = prefix ? `${prefix}/${entry}` : entry;
      const abs = join(dir, entry);
      if (statSync(abs).isDirectory()) {
        walk(abs, rel);
      } else {
        out.push(`snippets/${rel}`);
      }
    }
  };
  walk(snippetsDir, "");
  return out;
}

/** 📂️ Validates one portable compiler output owner without executing a compiler. */
export function wasmOutputDirectory(rsDir: string, outputDirectory: string): string {
  if (!outputDirectory || /[/\\:*?"<>|\u0000]|[. ]$/u.test(outputDirectory)) throw new Error("WASM outputDirectory must be one portable literal directory name");
  return join(rsDir, outputDirectory);
}

/** 🎚️ Keeps wasm-pack profile flags separate from Cargo's explicit profile selection. */
export function wasmBuildArguments(profile: string): { pack: string[]; cargo: string[] } {
  return { pack: profile === "release" ? ["--release"] : profile === "dev" ? ["--dev"] : ["--profile", profile], cargo: ["--profile", profile] };
}

/** ⚙️ The `cargo --config` overrides that optimize `crates` in the dev profile of one build. */
export function prototypeOptimizedCargoConfigArgs(crates: readonly string[]): string[] {
  return crates.flatMap((crate) => ["--config", `profile.wasm-prototype.package.${crate}.opt-level=3`]);
}

/** 📦️`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
export async function buildWasmWebV1(opts: WasmPackWebBuildOptions, policy: WasmBuildPolicyV1, port: WasmBuildExecutionPortV1 = { run: async (command, args, options) => { await runOwnedCommand(command, [...args], options.cwd, options.label, options.budgetMs, { env: options.environment, signal: options.signal }); }, admit: admitWasmCargoSourcesV1 }): Promise<void> {
  if(port.admit===admitWasmCargoSourcesV1&&!opts.storage)throw Error("Original enforcing WASM compiler store required");policy = admitWasmPolicy(policy);
  const control = requestedCompilerAdmissionControlV1(opts.control, policy.budgetMs), signal = control.signal;
  if (resolve(policy.cwd) !== resolve(opts.rsDir)) throw Error("WASM policy belongs to a different owner");
  const { rsDir, componentDir, logPrefix, pkg, wasmBaseName, outputDirectory = "pkg", threads = false, cargoFeatures = [], noDefaultFeatures = false, prototypeOptimizedCrates = [] } = opts;
  const captureRoot = policy.artifactDirectory;
  mkdirSync(captureRoot, { recursive: true });
  const cargoOutput = mkdtempSync(join(captureRoot, "wasm-cargo-"));
  const packageOutput = mkdtempSync(join(captureRoot, "wasm-package-"));
  let capacity: Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>> | undefined;
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    const activeDirectory = wasmOutputDirectory(rsDir, outputDirectory), pkgDir = packageOutput;
    const profile = buildModePolicy(policy.mode).wasmProfile;
    capacity = await acquireCompilerCapacityLeaseV1({ directory: policy.leaseDirectory, control });
    lease = await acquireCargoBuildLeaseV1({ directory: policy.leaseDirectory, buildDirectory: policy.buildDirectory, targetDirectory: cargoOutput, args: ["--profile",profile], signal, control });
    signal.throwIfAborted();
    const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
    const { pack: packProfileArgs, cargo: cargoProfileArgs } = wasmBuildArguments(profile);
    const profileOutDir = profile;
    const buildEnv:Record<string,string|undefined> = { ...process.env, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(capacity.parallelism), PATH: policy.searchPath, CARGO_TARGET_DIR: cargoOutput, CARGO_BUILD_BUILD_DIR: policy.buildDirectory };
    if(!buildEnv.MAKEFLAGS&&!buildEnv.CARGO_MAKEFLAGS)buildEnv.CARGO_BUILD_JOBS=String(capacity.parallelism);
    const invoke = async (command: string, args: readonly string[]) => { await port.run(command, args, { cwd: componentDir, environment: buildEnv, budgetMs: compilerAdmissionBudgetV1(control), label: logPrefix, signal: signal }); };
    const buildLabel = threads ? "cargo build (threaded) + wasm-bindgen" : "wasm-pack build";
    console.log(`[${logPrefix}] ${buildLabel} ${packProfileArgs.join(" ")} --target web --out-dir ${outputDirectory} --out-name ${wasmBaseName} --no-pack`);
    const t0 = Date.now();
    const featureArgs = [...(policy.compilerConfiguration?["--config",policy.compilerConfiguration]:[]),...(noDefaultFeatures ? (["--no-default-features"] as const) : []), ...cargoFeatures.flatMap((feature) => ["--features", feature]), ...(policy.mode === "prototype" ? prototypeOptimizedCargoConfigArgs(prototypeOptimizedCrates) : [])];
    await port.admit(threads ? ["build", "--locked", "--manifest-path", join(componentDir, "Cargo.toml"), ...cargoProfileArgs, "--target", "wasm32-unknown-unknown", "-Z", "build-std=std,panic_abort", ...featureArgs] : ["build", "--locked", "--lib", "--manifest-path", join(componentDir, "Cargo.toml"), ...cargoProfileArgs, "--target", "wasm32-unknown-unknown", ...featureArgs], { cwd: componentDir, environment: buildEnv, policy, storage:opts.storage, control });
    if (threads) {
      const crateName = readFileSync(join(componentDir, "Cargo.toml"), "utf8").match(/^name\s*=\s*"([^"]+)"/m)?.[1];
      if (!crateName) {
        throw new Error(`[${logPrefix}] missing package name in ${join(componentDir, "Cargo.toml")}`);
      }
      const cargoWasm = join(cargoOutput, `wasm32-unknown-unknown/${profileOutDir}`, `${crateName.replace(/-/g, "_")}.wasm`);
      const threadedCargoArgs = ["build", "--locked", ...cargoProfileArgs, "--target", "wasm32-unknown-unknown", "-Z", "build-std=std,panic_abort", ...featureArgs];
      await invoke("cargo", threadedCargoArgs);
      if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
      await invoke(policy.bindgen.command, [...policy.bindgen.args, cargoWasm, "--out-dir", pkgDir, "--typescript", "--target", "web", "--out-name", wasmBaseName]);
    } else {
      const buildArgs = ["build", "--mode", "no-install", ...packProfileArgs, "--target", "web", "--out-dir", pkgDir, "--out-name", wasmBaseName, "--no-pack", ...(policy.mode === "prototype" ? ["--no-opt"] : []), "--", "--locked", ...featureArgs];
      await invoke(policy.wasmPack.command, [...policy.wasmPack.args, ...buildArgs]);
    }
    console.log(`[${logPrefix}] wasm build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

    if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
    const snippetFiles = wasmPackSnippetFiles(pkgDir);
    const pkgJson = {
      type: "module",
      version: pkg.version ?? "0.1.0",
      sideEffects: pkg.sideEffects ?? ["./snippets/*"],
      ...pkg,
      semioWasmOwner: { version: 1, sourceOwner: policy.sourceOwner, mode: policy.mode, output: `dist/wasm/${policy.mode}` },
      files: [...new Set([...pkg.files, ...snippetFiles])],
    };
    writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

    if (existsSync(wasmPath)) {
      const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
      const files = await collectArtifactFiles(pkgDir, signal);
      await stageArtifacts(join(rsDir, "dist/wasm", policy.mode), `${policy.packageName}:wasm:${policy.mode}`, files, { signal: signal, leaseDirectory: policy.leaseDirectory });
      await stageArtifacts(activeDirectory, `${policy.packageName}:wasm:active`, files, { signal: signal, leaseDirectory: policy.leaseDirectory });
      console.log(`[${logPrefix}] dist/wasm/${policy.mode}/${wasmBaseName}_bg.wasm ready (${sz} MiB)`);
    } else {
      throw new Error(`[${logPrefix}] expected wasm output missing: ${wasmPath}`);
    }
  } finally {
    try { rmSync(cargoOutput, { recursive: true, force: true }); rmSync(packageOutput, { recursive: true, force: true }); }
    finally { lease?.release(); capacity?.release(); }
  }
}

