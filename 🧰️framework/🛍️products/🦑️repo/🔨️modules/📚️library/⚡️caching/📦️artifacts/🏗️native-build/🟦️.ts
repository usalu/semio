import { chmodSync, copyFileSync, lstatSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { startNativeProgress } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { stageArtifacts } from "../🟦️.ts";

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state. */
export async function buildCargoArtifacts(manifest: string, args: string[] = [], repoRoot = getWorkspaceRoot(), options: { command?: "build" | "rustc"; output?: string; validate?: (files: ReadonlyMap<string, string>) => void } = {}): Promise<void> {
  const path = resolve(repoRoot, manifest);
  const sourceRoot = dirname(path);
  const staging = resolve(sourceRoot, options.output ?? "dist/build");
  if (!staging.startsWith(sourceRoot + sep)) throw new Error("Cargo deliverables must belong to their source project");
  const captureParent = process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(process.env.SEMIO_TEST_ARTIFACT_DIR) : dirname(staging);
  mkdirSync(captureParent, { recursive: true });
  const capture = mkdtempSync(join(captureParent, "cargo-artifacts-"));
  const owner = relative(repoRoot, path).split(sep).join("/");
  const files = new Map<string, string>();
  const dependencies = new Map<string, string>();
  let hasLibrary = false;
  let cancelled = false;
  let forceKill: ReturnType<typeof setTimeout> | undefined;
  const delimiter = args.indexOf("--"),
    compilerArgs = delimiter < 0 ? [] : args.slice(delimiter),
    cargoArgs = delimiter < 0 ? args : args.slice(0, delimiter);
  const child = spawn("cargo", [options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], {
    cwd: repoRoot,
    env: { ...process.env, CARGO_TARGET_DIR: join(capture, "target") },
    detached: process.platform !== "win32",
    stdio: ["inherit", "pipe", "inherit"],
  });
  const cancel = (): void => {
    cancelled = true;
    if (!child.pid) return;
    if (process.platform === "win32") (globalThis as any).Bun.spawnSync(["taskkill", "/pid", String(child.pid), "/t", "/f"], { stdout: "ignore", stderr: "ignore" });
    else {
      try {
        process.kill(-child.pid, "SIGTERM");
      } catch {}
      forceKill = setTimeout(() => {
        try {
          process.kill(-child.pid!, "SIGKILL");
        } catch {}
      }, 2_000);
      forceKill.unref();
    }
  };
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const stopProgress = startNativeProgress(`artifact-rust:${owner}:build`);
  const status = new Promise<number>((accept) => {
    child.once("error", (error) => {
      console.error(error.message);
      accept(1);
    });
    child.once("close", (code) => accept(code ?? 1));
  });
  try {
    try {
      try {
        for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) {
          let message;
          try {
            message = JSON.parse(line);
          } catch {
            process.stdout.write(line + "\n");
            continue;
          }
          if (message.reason !== "compiler-artifact" || message.target?.kind?.includes("custom-build")) continue;
          const packageUrl = message.package_id?.split("#")[0]?.replace(/^path\+/, "");
          const bin = args.indexOf("--bin"),
            example = args.indexOf("--example");
          const selected =
            bin >= 0
              ? message.target?.kind?.includes("bin") && message.target.name === args[bin + 1]
              : example >= 0
                ? message.target?.kind?.includes("example") && message.target.name === args[example + 1]
                : args.includes("--bins")
                  ? message.target?.kind?.includes("bin")
                  : true;
          const primary = selected && packageUrl !== undefined && packageUrl.startsWith("file:") && resolve(fileURLToPath(packageUrl)) === sourceRoot;
          for (const file of message.filenames ?? []) {
            if (file.endsWith(".d")) continue;
            const library = primary && file.endsWith(".rmeta") ? message.filenames.find((candidate: string) => candidate.endsWith(".rlib")) : undefined;
            const name = (library ? library.replace(/\.rlib$/, ".rmeta") : file).split(/[\\/]/).at(-1)!;
            const key = primary ? name : /\.(rlib|rmeta|so|dylib|dll|lib)$/.test(file) ? `deps/${name}` : undefined;
            if (!key) continue;
            const captured = join(capture, key);
            mkdirSync(dirname(captured), { recursive: true });
            copyFileSync(file, captured);
            chmodSync(captured, lstatSync(file).mode & 0o777);
            if (primary) {
              files.set(key, captured);
              hasLibrary ||= file.endsWith(".rlib");
            } else dependencies.set(key, captured);
          }
        }
      } catch (error) {
        cancel();
        await status;
        throw error;
      }
      if ((await status) !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
    } finally {
      stopProgress();
      if (forceKill) clearTimeout(forceKill);
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
    if (files.size === 0) throw new Error(`Cargo emitted no final artifacts for ${owner}`);
    if (hasLibrary) for (const [name, file] of dependencies) files.set(name, file);
    options.validate?.(files);
    await stageArtifacts(staging, owner, files);
    console.log(`[nx-native] staged ${files.size} deliverables in ${relative(repoRoot, staging).split(sep).join("/")}`);
  } finally {
    rmSync(capture, { recursive: true, force: true });
  }
}
