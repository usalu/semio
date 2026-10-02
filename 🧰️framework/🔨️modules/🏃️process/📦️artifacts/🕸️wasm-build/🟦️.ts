import { acquireCargoBuildLeaseV1 } from "../🏗️native-build/🔒️lease/🟦️.ts";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { runOwnedCommand } from "../../🎛️owned-execution/🟦️.ts";

export type WasmPackWebPkg = {
  name: string;
  version?: string;
  files: string[];
  main: string;
  module: string;
  types: string;
  sideEffects?: string[];
};

/** 🕸️ Binds the exact source owner, tools, mode, compiler storage and deadline. */
export type WasmBuildPolicyV1 = Readonly<{ version: 1; cwd: string; mode: "dev" | "ship"; artifactDirectory: string; buildDirectory: string; leaseDirectory: string; budgetMs: number; packageName: string; searchPath: string; bindgen: Readonly<{ command: string; args: readonly string[] }>; wasmPack: Readonly<{ command: string; args: readonly string[] }> }>;
/** 📦️ Selects one owned browser package publication without discovering repository policy. */
export type WasmPackWebBuildOptions = Readonly<{ rsDir: string; logPrefix: string; pkg: WasmPackWebPkg; wasmBaseName: string; outputDirectory?: string; threads?: boolean; cargoFeatures?: readonly string[]; noDefaultFeatures?: boolean; shipProfile?: string; devOptimizedCrates?: readonly string[]; signal?: AbortSignal }>;
/** 🔌️ Executes an explicitly selected compiler with bounded cancellation. */
export type WasmBuildExecutionPortV1 = Readonly<{ run: (command: string, args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; budgetMs: number; label: string; signal?: AbortSignal }>) => Promise<void> }>;
const wasmPolicySchema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
function admitWasmPolicy(value: unknown): WasmBuildPolicyV1 {
  const errors = validateJsonSchemaSubset(wasmPolicySchema, value);
  if (errors.length) throw Error(`Invalid WASM compiler policy: ${errors.join("; ")}`);
  return value as WasmBuildPolicyV1;
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
export function devOptimizedCargoConfigArgs(crates: readonly string[]): string[] {
  return crates.flatMap((crate) => ["--config", `profile.dev.package.${crate}.opt-level=3`]);
}

/** 📦️`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
export async function buildWasmWebV1(opts: WasmPackWebBuildOptions, policy: WasmBuildPolicyV1, port: WasmBuildExecutionPortV1 = { run: async (command, args, options) => { await runOwnedCommand(command, [...args], options.cwd, options.label, options.budgetMs, { env: options.environment, signal: options.signal }); } }): Promise<void> {
  policy = admitWasmPolicy(policy);
  if (resolve(policy.cwd) !== resolve(opts.rsDir)) throw Error("WASM policy belongs to a different owner");
  const { rsDir, logPrefix, pkg, wasmBaseName, outputDirectory = "pkg", threads = false, cargoFeatures = [], noDefaultFeatures = false, shipProfile = "release", devOptimizedCrates = [] } = opts;
  const captureRoot = policy.artifactDirectory;
  mkdirSync(captureRoot, { recursive: true });
  const cargoOutput = mkdtempSync(join(captureRoot, "wasm-cargo-"));
  const controller = new AbortController(), abort = () => controller.abort();
  opts.signal?.addEventListener("abort",abort,{once:true});
  process.once("SIGINT",abort); process.once("SIGTERM",abort);
  if (opts.signal?.aborted) abort();
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    const pkgDir = wasmOutputDirectory(rsDir, outputDirectory);
    const profile = policy.mode === "ship" ? shipProfile : "dev";
    lease = await acquireCargoBuildLeaseV1({ directory: policy.leaseDirectory, buildDirectory: policy.buildDirectory, args: ["--profile",profile], signal: controller.signal });
    controller.signal.throwIfAborted();
    const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
    const { pack: packProfileArgs, cargo: cargoProfileArgs } = wasmBuildArguments(profile);
    const profileOutDir = profile === "dev" ? "debug" : profile;
    const buildEnv = { ...process.env, PATH: policy.searchPath, CARGO_TARGET_DIR: cargoOutput, CARGO_BUILD_BUILD_DIR: policy.buildDirectory };
    const invoke = async (command: string, args: readonly string[]) => { await port.run(command, args, { cwd: rsDir, environment: buildEnv, budgetMs: policy.budgetMs, label: logPrefix, signal: controller.signal }); };
    const buildLabel = threads ? "cargo build (threaded) + wasm-bindgen" : "wasm-pack build";
    console.log(`[${logPrefix}] ${buildLabel} ${packProfileArgs.join(" ")} --target web --out-dir ${outputDirectory} --out-name ${wasmBaseName} --no-pack`);
    const t0 = Date.now();
    const featureArgs = [...(noDefaultFeatures ? (["--no-default-features"] as const) : []), ...cargoFeatures.flatMap((feature) => ["--features", feature]), ...(profile === "dev" ? devOptimizedCargoConfigArgs(devOptimizedCrates) : [])];
    if (threads) {
      const crateName = policy.packageName;
      if (!crateName) {
        throw new Error(`[${logPrefix}] missing package name in Cargo.toml`);
      }
      const cargoWasm = join(cargoOutput, `wasm32-unknown-unknown/${profileOutDir}`, `${crateName.replace(/-/g, "_")}.wasm`);
      const threadedCargoArgs = ["build", "--locked", ...cargoProfileArgs, "--target", "wasm32-unknown-unknown", "-Z", "build-std=std,panic_abort", ...featureArgs];
      await invoke("cargo", threadedCargoArgs);
      if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
      await invoke(policy.bindgen.command, [...policy.bindgen.args, cargoWasm, "--out-dir", outputDirectory, "--typescript", "--target", "web", "--out-name", wasmBaseName]);
    } else {
      const buildArgs = ["build", "--mode", "no-install", ...packProfileArgs, "--target", "web", "--out-dir", outputDirectory, "--out-name", wasmBaseName, "--no-pack", "--", "--locked", ...featureArgs];
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
      files: [...new Set([...pkg.files, ...snippetFiles])],
    };
    writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

    if (existsSync(wasmPath)) {
      const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
      console.log(`[${logPrefix}] pkg/${wasmBaseName}_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
    } else {
      throw new Error(`[${logPrefix}] expected wasm output missing: ${wasmPath}`);
    }
  } finally {
    try { rmSync(cargoOutput, { recursive: true, force: true }); }
    finally { lease?.release(); opts.signal?.removeEventListener("abort",abort); process.removeListener("SIGINT",abort); process.removeListener("SIGTERM",abort); }
  }
}

