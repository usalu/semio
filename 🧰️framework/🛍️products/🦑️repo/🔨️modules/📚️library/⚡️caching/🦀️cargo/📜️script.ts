#!/usr/bin/env bun
import assert from "node:assert/strict";
import { chmodSync, copyFileSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
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

/** ⏱️ Keeps long native queue waits observable until the owning operation completes. */
export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => console.log(line)): () => void {
  const started = Date.now(), progress = setInterval(() => output(`[${label}] running elapsedMs=${Date.now() - started}`), intervalMs);
  return () => clearInterval(progress);
}

/** 🏃️ Runs a bounded owned command with progress and process-tree cancellation. */
export async function runOwnedCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs(), options: { stdout?: "inherit" | "ignore" } = {}): Promise<void> {
  const child = spawn(command, args, { cwd, env: process.env, detached: process.platform !== "win32", stdio: ["inherit", options.stdout ?? "inherit", "inherit"], windowsHide: true });
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
  const stopProgress = startNativeProgress(label);
  const timeout = timeoutMs > 0 ? setTimeout(() => terminate(`timeout ${timeoutMs}ms`), timeoutMs) : undefined;
  try {
    const status = await new Promise<number>((accept) => { child.once("error", (error) => { console.error(error.message); accept(1); }); child.once("close", (code) => accept(code ?? 1)); });
    if (stopped) throw new Error(`${label} stopped: ${stopped}`);
    if (status !== 0) throw new Error(`${label} failed (${status})`);
  } finally {
    stopProgress();
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

/** 🎯️ Normalizes an artifact router invocation before Cargo can observe caller-controlled selectors. */
export function artifactRustCargoArguments(operation: "build" | "check" | "test", segments: readonly string[]): { cargoArgs: string[]; testLevel?: string } {
  const levels = new Set(["fundamental", "quick", "long", "exhaustive"]), [first, ...rest] = segments;
  const testLevel = operation === "test" && first && levels.has(first) ? first : undefined;
  const cargoArgs = testLevel ? rest : [...segments];
  validateNativeCargoArguments(operation, cargoArgs);
  return { cargoArgs, testLevel };
}

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state. */
export async function buildCargoArtifacts(manifest: string, args: string[] = [], repoRoot = getWorkspaceRoot(), options: { command?: "build" | "rustc"; output?: string; validate?: (files: ReadonlyMap<string, string>) => void } = {}): Promise<void> {
  const path = resolve(repoRoot, manifest);
  const sourceRoot = dirname(path);
  const staging = resolve(sourceRoot, options.output ?? "dist/build");
  if (!staging.startsWith(sourceRoot + sep)) throw new Error("Cargo deliverables must belong to their source project");
  const captureParent = process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(process.env.SEMIO_TEST_ARTIFACT_DIR) : dirname(staging);
  mkdirSync(captureParent, { recursive: true });
  const capture = mkdtempSync(join(captureParent, "cargo-artifacts-"));
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
  const stopProgress = startNativeProgress(`artifact-rust:${owner}:build`);
  const status = new Promise<number>((accept) => { child.once("error", (error) => { console.error(error.message); accept(1); }); child.once("close", (code) => accept(code ?? 1)); });
  try {
    try {
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
            const key = primary ? name : /\.(rlib|rmeta|so|dylib|dll|lib)$/.test(file) ? `deps/${name}` : undefined;
            if (!key) continue;
            const captured = join(capture, key);
            mkdirSync(dirname(captured), { recursive: true });
            copyFileSync(file, captured);
            chmodSync(captured, lstatSync(file).mode & 0o777);
            if (primary) { files.set(key, captured); hasLibrary ||= file.endsWith(".rlib"); }
            else dependencies.set(key, captured);
          }
        }
      } catch (error) { cancel(); await status; throw error; }
      if (await status !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
    } finally { stopProgress(); if (forceKill) clearTimeout(forceKill); process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
    if (files.size === 0) throw new Error(`Cargo emitted no final artifacts for ${owner}`);
    if (hasLibrary) for (const [name, file] of dependencies) files.set(name, file);
    options.validate?.(files);
    stageArtifacts(staging, owner, files);
    console.log(`[nx-native] staged ${files.size} deliverables in ${slash(relative(repoRoot, staging))}`);
  } finally {
    rmSync(capture, { recursive: true, force: true });
  }
}


class NativeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [tool, operation] = args;
    const index = args.indexOf("--manifest");
    const manifest = index >= 0 ? args[index + 1] : undefined;
    if (tool === "cargo" && operation === "metadata") {
      if (index !== 2 || args.length !== 4 || !manifest) throw new Error("native cargo metadata --manifest <Cargo.toml>");
      const path = resolve(this.repoRoot, manifest);
      await runOwnedCommand("cargo", ["metadata", "--locked", "--format-version=1", "--manifest-path", path], dirname(path), "cargo:locked-metadata", buildBudgetMs(), { stdout: "ignore" });
      console.log("[cargo:locked-metadata] dependency lock validated");
      return;
    }
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
