#!/usr/bin/env bun
import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { BundleScript, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { buildBudgetMs, runCmdStatus } from "../../🏃️process/🟦️.ts";
import { stageArtifacts } from "../📦️artifacts/🟦️.ts";
const slash = (path: string): string => path.split(sep).join("/");

/** 🏃️ Runs a bounded owned command with progress and process-tree cancellation. */
async function runOwnedCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs()): Promise<void> {
  const child = spawn(command, args, { cwd, env: process.env, detached: process.platform !== "win32", stdio: "inherit", windowsHide: true });
  let stopped = "", forceKill: ReturnType<typeof setTimeout> | undefined;
  const terminate = (reason: string): void => {
    stopped ||= reason;
    if (!child.pid) return;
    if (process.platform === "win32") Bun.spawnSync(["taskkill", "/pid", String(child.pid), "/t", "/f"], { stdout: "ignore", stderr: "ignore" });
    else {
      try { process.kill(-child.pid, "SIGTERM"); } catch {}
      forceKill ??= setTimeout(() => { try { process.kill(-child.pid!, "SIGKILL"); } catch {} }, 2000);
      forceKill.unref();
    }
  };
  const interrupt = (): void => terminate("SIGINT");
  const stop = (): void => terminate("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", stop);
  const started = Date.now();
  const progress = setInterval(() => console.log(`[${label}] running elapsedMs=${Date.now() - started}`), 10_000);
  const timeout = timeoutMs > 0 ? setTimeout(() => terminate(`timeout ${timeoutMs}ms`), timeoutMs) : undefined;
  try {
    const status = await new Promise<number>((accept) => { child.once("error", (error) => { console.error(error.message); accept(1); }); child.once("close", (code) => accept(code ?? 1)); });
    if (stopped) throw new Error(`${label} stopped: ${stopped}`);
    if (status !== 0) throw new Error(`${label} failed (${status})`);
  } finally {
    clearInterval(progress);
    if (timeout) clearTimeout(timeout);
    if (forceKill) clearTimeout(forceKill);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}

/** 🎯️ Keeps caller arguments within the selected Nx leaf's package and production/test input contract. */
export function validateNativeCargoArguments(operation: "build" | "check" | "test", args: readonly string[]): void {
  const delimiter = args.indexOf("--"), cargo = delimiter < 0 ? args : args.slice(0, delimiter);
  for (const argument of cargo) if (/^(?:--(?:workspace|package|manifest-path|exclude|config)(?:=|$)|-p)/.test(argument) || operation !== "test" && /^--(?:all-targets|tests?|examples?|benches|bench)(?:=|$)/.test(argument)) throw new Error(`Argument ${argument} changes the native input contract; use a dedicated Nx target for that selection`);
}

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state. */
export async function buildCargoArtifacts(manifest: string, args: string[] = [], repoRoot = getWorkspaceRoot(), options: { command?: "build" | "rustc"; output?: string; validate?: (files: ReadonlyMap<string, string>) => void } = {}): Promise<void> {
  const path = resolve(repoRoot, manifest);
  const sourceRoot = dirname(path);
  const staging = resolve(sourceRoot, options.output ?? "dist/build");
  if (!staging.startsWith(sourceRoot + sep)) throw new Error("Cargo deliverables must belong to their source project");
  const owner = slash(relative(repoRoot, path));
  const files = new Map<string, string>();
  const dependencies = new Map<string, string>();
  let hasLibrary = false;
  let cancelled = false;
  let forceKill: ReturnType<typeof setTimeout> | undefined;
  const delimiter = args.indexOf("--"), compilerArgs = delimiter < 0 ? [] : args.slice(delimiter);
  const cargoArgs = delimiter < 0 ? args : args.slice(0, delimiter);
  const child = spawn("cargo", [options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], { cwd: repoRoot, env: process.env, detached: process.platform !== "win32", stdio: ["inherit", "pipe", "inherit"] });
  const cancel = (): void => {
    cancelled = true;
    if (!child.pid) return;
    if (process.platform === "win32") Bun.spawnSync(["taskkill", "/pid", String(child.pid), "/t", "/f"], { stdout: "ignore", stderr: "ignore" });
    else {
      try { process.kill(-child.pid, "SIGTERM"); } catch {}
      forceKill = setTimeout(() => { try { process.kill(-child.pid!, "SIGKILL"); } catch {} }, 2000);
      forceKill.unref();
    }
  };
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const status = new Promise<number>((accept) => { child.once("error", (error) => { console.error(error.message); accept(1); }); child.once("close", (code) => accept(code ?? 1)); });
  try {
    for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) {
      let message;
      try { message = JSON.parse(line); } catch { process.stdout.write(line + "\n"); continue; }
      if (message.reason !== "compiler-artifact" || message.target?.kind?.includes("custom-build")) continue;
      const packageUrl = message.package_id?.split("#")[0]?.replace(/^path\+/, "");
      const bin = args.indexOf("--bin"), example = args.indexOf("--example");
      const selected = bin >= 0 ? message.target?.kind?.includes("bin") && message.target.name === args[bin + 1] : example >= 0 ? message.target?.kind?.includes("example") && message.target.name === args[example + 1] : args.includes("--bins") ? message.target?.kind?.includes("bin") : true;
      const primary = selected && packageUrl?.startsWith("file:") && resolve(fileURLToPath(packageUrl)) === sourceRoot;
      for (const file of message.filenames ?? []) {
        if (file.endsWith(".d")) continue;
        const library = primary && file.endsWith(".rmeta") ? message.filenames.find((path: string) => path.endsWith(".rlib")) : undefined;
        const name = (library ? library.replace(/\.rlib$/, ".rmeta") : file).split(/[\\/]/).at(-1)!;
        if (primary) { files.set(name, file); hasLibrary ||= file.endsWith(".rlib"); }
        else if (/\.(rlib|rmeta|so|dylib|dll|lib)$/.test(file)) dependencies.set(`deps/${name}`, file);
      }
    }
    if (await status !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
  } finally { if (forceKill) clearTimeout(forceKill); process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  if (files.size === 0) throw new Error(`Cargo emitted no final artifacts for ${owner}`);
  if (hasLibrary) for (const [name, file] of dependencies) files.set(name, file);
  options.validate?.(files);
  stageArtifacts(staging, owner, files);
  console.log(`[nx-native] staged ${files.size} deliverables in ${slash(relative(repoRoot, staging))}`);
}

/** 📦️ Runs an independently owned Rust artifact package through the shared Nx-native contract. */
export async function runArtifactRustPackageMain(packageRoot: string, cargoName: string): Promise<void> {
  class BuildScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      await buildCargoArtifacts(relative(this.repoRoot, resolve(this.root, "Cargo.toml")), segments, this.repoRoot);
    }
  }
  class CheckScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      await runOwnedCommand("cargo", ["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml"), ...segments], this.repoRoot, `artifact-rust:${cargoName}:check`);
    }
  }
  class TestScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const levels = new Set(["fundamental", "quick", "long", "exhaustive"]), [first, ...rest] = segments;
      const extra = first && levels.has(first) ? rest : segments;
      if (first && levels.has(first)) process.env.SEMIO_TEST_LEVEL = first;
      await runOwnedCommand("cargo", ["test", "--locked", "-p", cargoName, ...extra], this.repoRoot, `artifact-rust:${cargoName}:test`);
    }
  }
  const packageRouter = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  const segments = process.argv.slice(2);
  await packageRouter.run(segments.length ? segments : ["test"]);
}

/** 🟦️ Builds and resolves a declaration-only TypeScript artifact package from its taxonomy source. */
export async function runArtifactTypeScriptPackageMain(packageRoot: string, packageName: string): Promise<void> {
  const source = resolve(packageRoot, "../../🟦️.ts"), output = resolve(packageRoot, "dist");
  const typeScript = async (entry: string, args: string[], skipLibraries = true): Promise<void> => {
    await runOwnedCommand(process.execPath, ["x", "tsc", entry, ...args, "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", ...(skipLibraries ? ["--skipLibCheck"] : []), "--target", "ES2022"], getWorkspaceRoot(), `artifact-typescript:${packageName}:tsc`, 120_000);
  };
  const copyDeclarationAssets = (): number => {
    const declaration = join(output, "🟦️.d.ts"), compiler = createRequire(import.meta.url)("typescript");
    let count = 0;
    for (const imported of compiler.preProcessFile(readFileSync(declaration, "utf8"), true, true).importedFiles) {
      if (!imported.fileName.startsWith(".") || !imported.fileName.endsWith(".json")) continue;
      const from = resolve(dirname(source), imported.fileName), to = resolve(dirname(declaration), imported.fileName), local = relative(output, to);
      if (local === ".." || local.startsWith(`..${sep}`)) throw new Error(`Declaration asset escapes ${packageName}: ${imported.fileName}`);
      mkdirSync(dirname(to), { recursive: true });
      copyFileSync(from, to);
      count++;
    }
    return count;
  };
  const build = async (): Promise<void> => {
    rmSync(output, { recursive: true, force: true });
    mkdirSync(output, { recursive: true });
    const result = await Bun.build({ entrypoints: [source], outdir: output, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
    if (!result.success) throw new AggregateError(result.logs, `Failed to build ${packageName}`);
    await typeScript(source, ["--declaration", "--emitDeclarationOnly", "--outDir", output]);
    const assets = copyDeclarationAssets();
    console.log(`[artifact-typescript] built ${packageName} outputs=${result.outputs.length + 1 + assets}`);
  };
  class BuildScript extends BundleScript { async run(): Promise<void> { await build(); } }
  class CheckScript extends BundleScript {
    async run(): Promise<void> {
      const result = await Bun.build({ entrypoints: [source], write: false, target: "bun", format: "esm" });
      if (!result.success) throw new AggregateError(result.logs, `Failed to check ${packageName}`);
      await typeScript(source, ["--noEmit"]);
      console.log(`[artifact-typescript] checked ${packageName}`);
    }
  }
  class TestScript extends BundleScript {
    async run(): Promise<void> {
      await build();
      const artifact = await import(packageName);
      const probe = join(output, "🧪️consumer.ts"), typeRoots = join(output, "🧪️types");
      mkdirSync(typeRoots);
      const assertion = Object.hasOwn(artifact, "definition") ? "const definitionId: typeof artifact.definition.id = artifact.definition.id;\nvoid definitionId;" : `const artifactModule: typeof import(${JSON.stringify(packageName)}) = artifact;\nvoid artifactModule;`;
      writeFileSync(probe, `import * as artifact from ${JSON.stringify(packageName)};\n${assertion}\n`);
      try { await typeScript(probe, ["--noEmit", "--typeRoots", typeRoots], false); } finally { rmSync(probe, { force: true }); rmSync(typeRoots, { recursive: true, force: true }); }
      assert.equal(typeof artifact, "object", `${packageName} did not resolve as an ES module`);
      console.log(`[artifact-typescript] tested ${packageName} exports=${Object.keys(artifact).length}`);
    }
  }
  const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  const segments = process.argv.slice(2);
  await router.run(segments.length ? segments : ["test"]);
}

class NativeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [tool, operation] = args;
    const index = args.indexOf("--manifest");
    const manifest = index >= 0 ? args[index + 1] : undefined;
    if (tool === "component") {
      if (!["dev", "release"].includes(operation) || index !== 2 || args.length !== 4 || !manifest) throw new Error("native component dev|release --manifest <Cargo.toml>");
      const cargo = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(resolve(this.repoRoot, manifest), "utf8"));
      if (!cargo.package?.metadata?.component?.package || !["plugin", "extension"].includes(cargo.package?.metadata?.semio?.role)) throw new Error(`Not a plugin component manifest: ${manifest}`);
      return buildCargoArtifacts(manifest, ["-p", cargo.package.name, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", `wasm-${operation}`, "--", "-C", "link-arg=-zstack-size=8388608", ...(process.env.SEMIO_PLUGIN_SYMBOLS === "1" ? ["-C", "strip=none"] : [])], this.repoRoot, {
        command: "rustc",
        output: `dist/component-${operation}`,
        validate: (files) => {
          assert.equal(files.size, 1, "Component output must contain only the linked WASM component");
          const [name, path] = [...files][0];
          assert.equal(name, `${cargo.package.name.replaceAll("-", "_")}.wasm`);
          assert.deepEqual([...readFileSync(path).subarray(0, 8)], [0, 97, 115, 109, 13, 0, 1, 0], "Invalid WASI component header");
        },
      });
    }
    if (tool !== "cargo" || operation !== "build" && operation !== "check" && operation !== "test" || !manifest) throw new Error("native cargo build|check|test --manifest <Cargo.toml>");
    const extra = args.slice(index + 2);
    validateNativeCargoArguments(operation, extra);
    if (operation === "build") return buildCargoArtifacts(manifest, extra, this.repoRoot);
    const status = runCmdStatus("cargo", [operation, "--locked", "--manifest-path", resolve(this.repoRoot, manifest), ...extra], { cwd: this.repoRoot });
    if (status) throw new Error(`cargo ${operation} failed (${status})`);
  }
}

const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("native", NativeScript);
if (import.meta.main) await router.run(process.argv.slice(2));
