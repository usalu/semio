import { chmodSync, copyFileSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { createInterface } from "node:readline";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { startNativeProgress } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { stageArtifacts } from "../🟦️.ts";

/** ✍️ Re-signs a native executable on macOS, ad-hoc, and does nothing anywhere else.
 *
 * A Mach-O binary carries its code signature inside the file, and the kernel validates it against
 * the bytes on disk. A copied or rewritten executable whose signature no longer matches is not
 * rejected with a message — it is `SIGKILL`ed without one. Signing every distribution copy ad-hoc
 * (`--sign -`) is what keeps that from happening. This is *not* Developer ID signing or
 * notarization: a binary signed this way still trips Gatekeeper when it arrives from the internet
 * on someone else's Mac, which needs a signing identity this repository does not have. */
export function signExecutableForDistribution(binary: string): void {
  if (process.platform !== "darwin") return;
  const signed = spawnSync("codesign", ["--force", "--sign", "-", binary], { stdio: ["ignore", "inherit", "inherit"] });
  if (signed.status !== 0) throw new Error(`codesign refused ${binary} (status ${signed.status ?? "unknown"})`);
}

/** 📥️ Places an executable at `destination`: remove, copy, then sign — never an overwrite in place.
 *
 * Writing over a running or previously-signed Mach-O binary is the exact shape that produces a
 * silent `SIGKILL` on the next launch, so the old inode is unlinked first and the fresh copy is
 * signed afterwards. */
export function installExecutable(source: string, destination: string): void {
  mkdirSync(dirname(destination), { recursive: true });
  rmSync(destination, { force: true });
  copyFileSync(source, destination);
  chmodSync(destination, lstatSync(source).mode & 0o777);
  signExecutableForDistribution(destination);
}

/** 🏷️ The one version every crate in this workspace carries (`[workspace.package] version`). */
export function workspaceCargoVersion(repoRoot = getWorkspaceRoot()): string {
  const manifest = readFileSync(join(repoRoot, "Cargo.toml"), "utf8");
  const heading = "[workspace.package]";
  const start = manifest.indexOf(heading);
  if (start < 0) throw new Error("Cargo.toml declares no [workspace.package] section");
  const rest = manifest.slice(start + heading.length);
  const end = rest.indexOf("\n[");
  const version = /^\s*version\s*=\s*"([^"]+)"/m.exec(end < 0 ? rest : rest.slice(0, end));
  if (!version) throw new Error("Cargo.toml declares no [workspace.package] version");
  return version[1];
}

/** 🚚️ Packages one already-built native executable as a versioned, checksummed local tarball.
 *
 * This is the last mile a release build was missing: a compiled binary sitting in a project's
 * `dist/` is not something an operator can be handed. It produces
 * `<output>/<name>-<version>-<platform>-<arch>.tar.gz` plus a `.sha256` next to it, and **uploads
 * nothing anywhere** — where the artifact goes afterwards is a deployment decision this repository
 * does not take. Returns the tarball path. */
export function packageNativeRelease(options: { readonly binary: string; readonly name: string; readonly version: string; readonly output: string; readonly platform?: NodeJS.Platform; readonly arch?: string }): string {
  const platform = options.platform ?? process.platform;
  const arch = options.arch ?? process.arch;
  if (!lstatSync(options.binary).isFile()) throw new Error(`no release binary at ${options.binary}`);
  mkdirSync(options.output, { recursive: true });
  const filename = `${options.name}-${options.version}-${platform}-${arch}.tar.gz`;
  const tarball = join(options.output, filename);
  const payload = mkdtempSync(join(options.output, "payload-"));
  try {
    installExecutable(options.binary, join(payload, `${options.name}-${options.version}`, options.name));
    rmSync(tarball, { force: true });
    const archived = spawnSync("tar", ["-czf", tarball, "-C", payload, `${options.name}-${options.version}`], { stdio: ["ignore", "inherit", "inherit"] });
    if (archived.status !== 0) throw new Error(`tar refused ${tarball} (status ${archived.status ?? "unknown"})`);
  } finally {
    rmSync(payload, { recursive: true, force: true });
  }
  const digest = createHash("sha256").update(readFileSync(tarball)).digest("hex");
  writeFileSync(`${tarball}.sha256`, `${digest}  ${filename}\n`);
  console.log(`[publish] ${tarball}\n[publish] sha256 ${digest}`);
  return tarball;
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
            if (!primary) {
              dependencies.set(key, file);
              continue;
            }
            const captured = join(capture, key);
            mkdirSync(dirname(captured), { recursive: true });
            copyFileSync(file, captured);
            chmodSync(captured, lstatSync(file).mode & 0o777);
            files.set(key, captured);
            hasLibrary ||= file.endsWith(".rlib");
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
    if (hasLibrary)
      for (const [name, file] of dependencies) {
        const captured = join(capture, name);
        mkdirSync(dirname(captured), { recursive: true });
        copyFileSync(file, captured);
        chmodSync(captured, lstatSync(file).mode & 0o777);
        files.set(name, captured);
      }
    options.validate?.(files);
    await stageArtifacts(staging, owner, files);
    console.log(`[nx-native] staged ${files.size} deliverables in ${relative(repoRoot, staging).split(sep).join("/")}`);
  } finally {
    rmSync(capture, { recursive: true, force: true });
  }
}
