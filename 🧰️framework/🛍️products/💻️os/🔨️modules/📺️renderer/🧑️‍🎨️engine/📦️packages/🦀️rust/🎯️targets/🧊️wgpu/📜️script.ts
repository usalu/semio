#!/usr/bin/env bun
/** @emoji 🧊️ `@semio-tech/framework-renderer-wgpu` task router. */
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  SEMIO_ASSET_SERVER_PORT,
  SEMIO_ASSET_BASE_URL_ENV,
  buildBudgetMs,
  daemonBudgetOpts,
  getWorkspaceRoot,
  orchestratorBudgetOpts,
  resolveTestLevel,
  runBundleScriptMain,
  runCargoTestBudgeted,
  runCmd,
  runCmdStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  frameworkOsPlaygroundDefaultPort,
  loadFrameworkOsPlaygroundCatalog,
} from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { startAssetServer } from "../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts";
import type { PlaygroundAssetSpec } from "../../../../../../🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts";

const repoRoot = getWorkspaceRoot();
const wasmTarget = "wasm32-unknown-unknown";
const crateName = "semio-framework-os-renderer-wgpu";
const outDir = join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu");
const pluginOutRoot = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules");

//#region 🌐️ DevServer
/** @emoji 👥️ Full `process.env` passthrough for the spawned `trunk` child — raw (unprefixed)
 * `S_HUB_URL`/`S_USER`/`S_DATA_DIR` reach it exactly as set by the launching `dev` process (the wgpu
 * user launchers in `.vscode/🧩️launch.seed.jsonc`'s `devLaunchers.s.users`), since native/wasm-in-trunk
 * code reads `std::env` directly rather than through a `import.meta.env.VITE_*` compile-time define. */
function trunkEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  delete env.NO_COLOR;
  delete env.FORCE_COLOR;
  if (env.SEMIO_PARITY_QUIET_CARGO === "1") env.RUSTFLAGS = [env.RUSTFLAGS, "-Awarnings"].filter(Boolean).join(" ");
  return env;
}

/** 🌐️ Runs a long-lived child without blocking Bun's asset-server event loop. */
async function runInteractiveCommand(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv): Promise<number> {
  const daemon = spawnDaemon(command, args, { cwd, env });
  const terminate = () => daemon.kill();
  process.once("SIGINT", terminate);
  process.once("SIGTERM", terminate);
  try {
    return await new Promise<number>((resolve, reject) => {
      daemon.child.once("error", reject);
      daemon.child.once("exit", (code) => resolve(code ?? 1));
    });
  } finally {
    process.off("SIGINT", terminate);
    process.off("SIGTERM", terminate);
    daemon.kill();
  }
}

function ensureWasmTarget(): void {
  const probe = runProbe("rustup", ["target", "list", "--installed"]);
  if (!probe.stdout.includes(wasmTarget)) {
    runCmd("rustup", ["target", "add", wasmTarget]);
  }
}

function ensureTrunk(): void {
  const probe = runProbe("trunk", ["--version"]);
  if (probe.status !== 0) {
    runCmd("cargo", ["install", "trunk", "--locked"], { budgetMs: buildBudgetMs() });
  }
}

function syncStableRendererArtifacts(): void {
  const artifactPrefix = `${crateName}-`;
  const js = readdirSync(outDir).find((name) => name.startsWith(artifactPrefix) && name.endsWith(".js"));
  const wasm = readdirSync(outDir).find((name) => name.startsWith(artifactPrefix) && name.endsWith("_bg.wasm"));
  if (!js) throw new Error("missing trunk wgpu renderer js artifact");
  copyFileSync(join(outDir, js), join(outDir, "semio_framework_renderer_wgpu.js"));
  if (wasm) copyFileSync(join(outDir, wasm), join(outDir, "semio-framework-renderer-wgpu_bg.wasm"));
}

function assetServerBaseUrl(): string {
  return `http://127.0.0.1:${SEMIO_ASSET_SERVER_PORT}`;
}

/** @emoji 🗂️ The active playground variant's declared asset specs (tile-proxy, mesh-collection, static-dir). */
function variantAssetSpecs(variant: string): readonly PlaygroundAssetSpec[] {
  const row = loadFrameworkOsPlaygroundCatalog().find((entry) => entry.variant === variant);
  return row?.assets ?? [];
}

/** @emoji 🌐️ Generic dev-time asset server bootstrap driven by the active playground's declared
 * asset specs — mesh-collection/static-dir are served here so Trunk proxies and native
 * `SEMIO_ASSET_BASE_URL` can resolve `/mesh/*` (and fixture routes) without Vite. */
function ensureAssetServer(variant: string): void {
  const specs = variantAssetSpecs(variant);
  if (specs.length === 0) return;
  startAssetServer(repoRoot, SEMIO_ASSET_SERVER_PORT, specs);
  console.log(`asset server serving at ${assetServerBaseUrl()} (${specs.map((s) => `${s.kind}:${s.route}`).join(", ")})`);
}

/** 🎯️Resolves the `--app <appId>` args for `semio-wgpu-native` from the catalog row matching `filterPlugin`, or `[]` when the row has no `app`. */
function resolveNativeAppArgs(catalog: ReturnType<typeof loadFrameworkOsPlaygroundCatalog>, filterPlugin: string): string[] {
  const row = catalog.find((r) => r.variant === filterPlugin);
  return row?.app ? ["--app", row.app] : [];
}

/** @emoji 🥖️ Rejects browser code generation unless it uses the repository's exact Bun toolchain. */
export function assertPinnedBunVersion(actualVersion: string = Bun.version): string {
  const packageManager = (JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8")) as { readonly packageManager?: string }).packageManager ?? "";
  const expectedVersion = /^bun@(\d+\.\d+\.\d+)$/u.exec(packageManager)?.[1];
  if (!expectedVersion) throw new Error(`wgpu-frame-worker requires an exact root packageManager bun@x.y.z pin; received ${JSON.stringify(packageManager)}`);
  if (actualVersion !== expectedVersion) throw new Error(`wgpu-frame-worker requires Bun ${expectedVersion} from root packageManager, received ${actualVersion}`);
  return expectedVersion;
}

/** @emoji 🧾️ Bundles one browser entry entirely in memory for identical generate/check bytes. */
async function renderBrowserEntry(entryPath: string): Promise<string> {
  assertPinnedBunVersion();
  const runtime = globalThis as typeof globalThis & { Bun: { build(options: { entrypoints: string[]; target: "browser"; format: "esm" }): Promise<{ success: boolean; logs: unknown[]; outputs: { text(): Promise<string> }[] }> } };
  const result = await runtime.Bun.build({ entrypoints: [entryPath], target: "browser", format: "esm" });
  if (!result.success || result.outputs.length !== 1) throw new Error(`browser bundle render failed for ${entryPath}: ${result.logs.map(String).join("\n")}`);
  return await result.outputs[0]!.text();
}

async function buildBootScript(bundleRoot: string): Promise<void> {
  const bootTs = join(bundleRoot, "🟦️typescript/🟦️boot.ts");
  const bootJs = join(bundleRoot, "🟦️typescript/🟨️boot.js");
  writeFileSync(bootJs, await renderBrowserEntry(bootTs), "utf8");
}

/** @emoji 🧵️ Renders the frame worker without invoking Trunk, Cargo, or the WASM build. */
export async function renderFrameWorker(bundleRoot: string): Promise<{ path: string; content: string }> {
  const workerTs = join(bundleRoot, "🟦️typescript/🧵️frame-worker.ts");
  const workerJs = join(bundleRoot, "🟦️typescript/🟨️frame-worker.js");
  return { path: workerJs, content: await renderBrowserEntry(workerTs) };
}

async function generateFrameWorker(bundleRoot: string): Promise<void> {
  const artifact = await renderFrameWorker(bundleRoot);
  writeFileSync(artifact.path, artifact.content, "utf8");
}

async function checkFrameWorker(bundleRoot: string): Promise<void> {
  const artifact = await renderFrameWorker(bundleRoot);
  if (!existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content) throw new Error("🟨️frame-worker.js is stale; run the generate-frame-worker target");
}

class TrunkBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    ensureTrunk();
    ensureWasmTarget();
    await buildBootScript(this.root);
    await checkFrameWorker(this.root);
    mkdirSync(outDir, { recursive: true });
    const release = segments.includes("--release") || segments.includes("--dist");
    const args = ["build", "--config", "Trunk.toml"];
    if (release) args.push("--release");
    if (runCmdStatus("trunk", args, { cwd: this.root, env: trunkEnv(), budgetMs: buildBudgetMs() }) !== 0) throw new Error("trunk build failed for wgpu renderer");
    syncStableRendererArtifacts();
    console.log(`trunk built wgpu renderer -> ${outDir}`);
  }
}

class TrunkServeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    ensureTrunk();
    ensureWasmTarget();
    await buildBootScript(this.root);
    await checkFrameWorker(this.root);
    const program = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    ensureAssetServer(program);
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(catalog, program, "wgpu"));
    const port = process.env.S_OS_PORT ?? defaultPort;
    const extra = segments.filter((segment, index, all) => segment !== "--port" && all[index - 1] !== "--port");
    const args = ["serve", "--config", "Trunk.toml", "--port", port, ...extra];
    if (process.env.SEMIO_PARITY_QUIET_CARGO === "1") args.push("--ignore", pluginOutRoot);
    if ((await runInteractiveCommand("trunk", args, this.root, trunkEnv())) !== 0) throw new Error("trunk serve failed for wgpu renderer");
  }
}
//#endregion 🌐️ DevServer

/** 🔖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b-bench): `--scale <registry.json>` selects the
 * headless scale-bench mode (`scale_bench::run` in `📦️glue.rs`) — no ShellState/GPU/winit, no plugin
 * catalog, so `NativeBuildScript`/`NativeRunScript` skip the plugin-wasm-program build and asset
 * server entirely in this mode and just build/run `semio-wgpu-native` itself with the scale flags
 * passed straight through, mirroring `--smoke`'s existing pass-through idiom. */
function scaleModeArgValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  return index >= 0 ? segments[index + 1] : undefined;
}

function scaleModePassthroughArgs(segments: readonly string[]): string[] {
  const args: string[] = [];
  for (const flag of ["--scale", "--scale-wasm", "--shards", "--report"]) {
    const value = scaleModeArgValue(segments, flag);
    if (value) args.push(flag, value);
  }
  return args;
}

class NativeBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const ship = segments.includes("--dist") || segments.includes("--release");
    const cargoArgs = ["build", "-p", crateName, "--bin", "semio-wgpu-native", "--features", "native-bin"];
    if (ship) cargoArgs.push("--release");
    if (runCmdStatus("cargo", cargoArgs, { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
      throw new Error("native wgpu renderer build failed");
    }
    if (segments.includes("--scale")) {
      console.log("built native wgpu renderer (scale-bench mode — no plugin catalog build)");
      return;
    }
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    const osDevScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts");
    // Recurses into os/dev's own `program` build loop, whose per-plugin `cargo build` calls are individually budgeted.
    const program = runCmdStatus("bun", [osDevScript, "plugin", filterPlugin], {
      cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript"),
      env: { ...process.env, SEMIO_RENDERER: "wgpu", SEMIO_PLUGIN: filterPlugin },
      ...orchestratorBudgetOpts(),
    });
    if (program !== 0) throw new Error(`wasm program build failed: ${filterPlugin}`);
    console.log(`built native wgpu renderer and wasm programs for ${filterPlugin}`);
  }
}

class NativeRunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const ship = segments.includes("--dist") || segments.includes("--release");
    if (segments.includes("--scale")) {
      await new NativeBuildScript(this.root).run(segments);
      const cargoArgs = ["run"];
      if (ship) cargoArgs.push("--release");
      cargoArgs.push("-p", crateName, "--bin", "semio-wgpu-native", "--features", "native-bin", "--", ...scaleModePassthroughArgs(segments));
      if (runCmdStatus("cargo", cargoArgs, { cwd: repoRoot, env: process.env, ...daemonBudgetOpts() }) !== 0) {
        throw new Error("native wgpu scale-bench run failed");
      }
      return;
    }
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    const buildSegments = segments[0] ? segments : [filterPlugin];
    await new NativeBuildScript(this.root).run(buildSegments);
    ensureAssetServer(filterPlugin);
    const nativeEnv: NodeJS.ProcessEnv = {
      ...process.env,
      SEMIO_PLUGIN_MODULES: pluginOutRoot,
    };
    if (variantAssetSpecs(filterPlugin).length > 0) {
      nativeEnv[SEMIO_ASSET_BASE_URL_ENV] = assetServerBaseUrl();
    }
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const appArgs = resolveNativeAppArgs(catalog, filterPlugin);
    // 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — `--smoke` passes straight
    // through to `semio-wgpu-native` (boots headless, dumps the widget tree as JSON, exits) instead of
    // opening a real window; an honest way to drive/observe this shell in an environment that cannot.
    const smokeArgs = segments.includes("--smoke") ? ["--smoke"] : [];
    const cargoArgs = ["run"];
    if (ship) cargoArgs.push("--release");
    cargoArgs.push("-p", crateName, "--bin", "semio-wgpu-native", "--features", "native-bin", "--", "--plugin", filterPlugin, ...appArgs, ...smokeArgs);
    if (runCmdStatus("cargo", cargoArgs, { cwd: repoRoot, env: nativeEnv, ...daemonBudgetOpts() }) !== 0) {
      throw new Error("native wgpu renderer run failed");
    }
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([crateName], this.repoRoot, rest);
    await runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

/** @emoji 🧵️ Runs the browser Worker transport protocol without invoking Cargo. */
class BrowserWorkerTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, ["🧪️browser-frame-transport.test.ts", "🧪️browser-interactive-job-port.test.ts", ...segments], "🧪️vitest.config.ts");
  }
}

/** @emoji 🧾️ Runs the deterministic in-memory frame-worker owner contract. */
class PreviewGeneratedTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, ["🧪️index.test.ts", ...segments], "🧪️vitest.config.ts");
  }
}

/** @emoji 🧵️ Bundles both browser isolates without invoking Cargo or Trunk. */
class BrowserWorkerCheckScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await buildBootScript(this.root);
    await checkFrameWorker(this.root);
  }
}

/** @emoji 🧵️ Generates only the deterministic browser frame-worker artifact. */
class GenerateFrameWorkerScript extends BundleScript {
  async run(): Promise<void> {
    await generateFrameWorker(this.root);
    console.log("framework-renderer-wgpu: generated 🟨️frame-worker.js");
  }
}

/** 🧾️ Emits the canonical read-only generator protocol from the same in-memory browser bundle. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const artifact = await renderFrameWorker(this.root);
    const nodes = [{ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, artifact.path).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: "wgpu-frame-worker", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

/** @emoji ✅️ Checks the frame-worker bytes without invoking any renderer build. */
class CheckFrameWorkerScript extends BundleScript {
  async run(): Promise<void> {
    await checkFrameWorker(this.root);
    console.log("framework-renderer-wgpu: 🟨️frame-worker.js is fresh");
  }
}

//#region 🔖️LintScript
/** 🎨️Raw color-construction calls (`Rgba::new`/`from_srgb8`) must live only inside `framework/ui/wgpu`'s theme module — the renderer takes every color via `ui_wgpu::Theme`. */
function collectWgpuColorLiteralViolations(bundleRoot: string): string[] {
  const libPath = join(bundleRoot, "📦️glue.rs");
  if (!existsSync(libPath)) return [];
  const text = readFileSync(libPath, "utf8");
  const violations: string[] = [];
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]!;
    if (/\bRgba::new\(|\bfrom_srgb8\(/.test(line)) {
      violations.push(`📦️glue.rs:${i + 1}: ${line.trim()}`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = collectWgpuColorLiteralViolations(this.root);
    if (violations.length === 0) {
      console.log("framework-renderer-wgpu: color-literal lint passed");
      return;
    }
    console.error(`framework-renderer-wgpu: found ${violations.length} raw color-construction call(s) outside framework/ui/wgpu theme:`);
    for (const v of violations.slice(0, 40)) console.error(`  ${v}`);
    if (violations.length > 40) console.error(`  … and ${violations.length - 40} more`);
    process.exit(1);
  }
}
//#endregion 🔖️LintScript

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", TrunkBuildScript)
  .register("build", TrunkBuildScript)
  .register("serve", TrunkServeScript)
  .register("dev", TrunkServeScript)
  .register("native", NativeRunScript)
  .register("native-build", NativeBuildScript)
  .register("test", TestScript)
  .register("test-browser-worker", BrowserWorkerTestScript)
  .register("test-preview-generated", PreviewGeneratedTestScript)
  .register("check-browser-worker", BrowserWorkerCheckScript)
  .register("generate-frame-worker", GenerateFrameWorkerScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-frame-worker", CheckFrameWorkerScript)
  .register("lint", LintScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
}
