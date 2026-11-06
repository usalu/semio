import { runToolSyncV1, startToolV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { accessSync, constants as fsConstants, readFileSync, statSync } from "node:fs";
import { posix, win32 } from "node:path";
import { createInterface } from "node:readline";
import { cargoTargetDirectory } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { CARGO_BUILD_OWNER_PID_ENV, cargoBinarySourcesFreshnessV1, parseCargoBinarySourcesV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";

//#region 🔖️BinaryPath
/** 📦️ The build target owns the executable separately from mutable compiler state. */
export const MCP_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build";
/** 📦️ Where the optimized distribution binary is staged. Separate from {@link MCP_ARTIFACT_REL} so a
 * release build never clobbers the dev-loop artifact `.mcp.json` and every black-box gate exec. */
export const MCP_RELEASE_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release";
export const MCP_CARGO_PACKAGE = "semio-os-mcp";
export const MCP_BINARY_NAME = "semio-os-mcp";
/** 🏗️ The cargo profiles this crate's build targets produce. */
export type McpBuildProfile = "debug" | "release";

function pathApi(platform: NodeJS.Platform): typeof posix {
  return platform === "win32" ? win32 : posix;
}

/** 📁️ Absolute Cargo target root shared by the build and black-box test gates. */
export function resolveMcpTargetDirectory(repoRoot: string, env: NodeJS.ProcessEnv = process.env, _platform: NodeJS.Platform = process.platform): string {
  return cargoTargetDirectory(repoRoot, env);
}

/** 📦️ The exact artifact Cargo's MCP build command must produce for `profile`, ignoring test overrides. */
export function resolveBuiltMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform, profile: McpBuildProfile = "debug"): string {
  const filename = platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME;
  return pathApi(platform).join(resolveMcpTargetDirectory(repoRoot, env, platform), profile, filename);
}

/** 📁️ The staged release executable — the one a tarball ships and an end user installs. */
export function resolveStagedReleaseMcpBinaryPath(repoRoot: string, platform: NodeJS.Platform = process.platform): string {
  return pathApi(platform).resolve(repoRoot, MCP_RELEASE_ARTIFACT_REL, platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME);
}

/** 📍️ Selects the executable from its owning admitted compiler publication. */
export function resolvePublishedMcpBinaryPathV1(publication: string, platform: NodeJS.Platform = process.platform): string {
 return pathApi(platform).join(publication, platform === "win32" ? MCP_BINARY_NAME + ".exe" : MCP_BINARY_NAME);
}

/** 📁️ Resolves the staged executable or an explicit independent binary override. */
export function resolveMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const override = env.SEMIO_OS_MCP_BIN;
  return override ? pathApi(platform).resolve(repoRoot, override) : pathApi(platform).resolve(repoRoot, MCP_ARTIFACT_REL, platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME);
}

/** 🎯 The neutral target that stages the debug `semio-os-mcp` every `.mcp.json` / `dev mcp` route reads. */
/** 🌱️ The authored interactive MCP compiler intent, independent of shell configuration. */
export const MCP_STDIO_BUILD_MODE_V1 = "prototype" as const;

export const MCP_BINARY_TARGET = "@semio-tech/framework-os-mcp-rs:build";

/** 🧾️ The sources record the build stages beside the binary (`buildCargoArtifacts`' `sourcesRecord`):
 * every file Cargo compiled it from and when that build started. */
export const MCP_BINARY_SOURCES_FILE = "semio-os-mcp.sources.json";

/** 🧾️ Where the sources record of `binary` lives — beside it, in the same atomic publication. */
export function resolveMcpBinarySourcesPath(binary: string, platform: NodeJS.Platform = process.platform): string {
  return pathApi(platform).join(pathApi(platform).dirname(binary), MCP_BINARY_SOURCES_FILE);
}

/** 🏗️ Injectable staging boundary: one attempt returning the neutral build target's exit status. */
export type McpBinaryStaging = Readonly<{ stage: (progress: (line: string) => void) => number }>;

const mcpStagingEnvironment = (environment: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv => ({ ...environment, CARGO_INCREMENTAL: environment.CARGO_INCREMENTAL ?? "0", [CARGO_BUILD_OWNER_PID_ENV]: String(process.pid) });

/** 🏗️ Non-blocking staging: the neutral build target runs as a child while the caller keeps serving; resolves with its exit status, `-1` when it cannot start. */
export function stageMcpBinaryAsync(repoRoot: string, progress: (line: string) => void, signal?: AbortSignal, target: string = MCP_BINARY_TARGET, environment: NodeJS.ProcessEnv = process.env): Promise<number> {
  return new Promise((resolveStatus) => {
    progress(`[semio-os-mcp] staging via repo run ${target}`);
    const child = startToolV1("repo", ["run", target, "--output=stream"], { cwd: repoRoot, shell: false, env: mcpStagingEnvironment(environment), stdio: ["ignore", "pipe", "pipe"], signal });
    for (const stream of [child.stdout, child.stderr]) createInterface({ input: stream }).on("line", (line) => void (line.trim() && progress(line)));
    child.on("error", (error) => {
      progress(`[semio-os-mcp] ${error.message}`);
      resolveStatus(-1);
    });
    child.on("close", (code) => resolveStatus(code ?? -1));
  });
}

const nativeMcpBinaryStagingFor = (repoRoot: string): McpBinaryStaging => ({
  stage: (progress) => {
    progress(`[semio-os-mcp] staging via repo run ${MCP_BINARY_TARGET}`);
    const result = runToolSyncV1("repo", ["run", MCP_BINARY_TARGET, "--output=stream"], {
      cwd: repoRoot,
      encoding: "utf8",
      maxBuffer: 16 * 1024 * 1024,
      shell: false,
      env: mcpStagingEnvironment(),
    });
    const combined = `${result.stdout ?? ""}${result.stderr ?? ""}`;
    for (const line of combined.split(/\r?\n/)) {
      if (line.trim()) progress(line);
    }
    return result.status ?? -1;
  },
});

/** 🕰️ Whether the staged binary is still what its sources build, and if not, the first source that
 * says so: a missing binary or record, a vanished source, or one modified after the build started. */
export function mcpBinaryFreshness(binary: string, platform: NodeJS.Platform = process.platform, modifiedAtMs: (path: string) => number | null = (path) => {
  try {
    return statSync(path).mtimeMs;
  } catch {
    return null;
  }
}): { readonly fresh: boolean; readonly reason: string } {
  if (modifiedAtMs(binary) === null) return { fresh: false, reason: "no staged binary" };
  let text: string;
  try {
    text = readFileSync(resolveMcpBinarySourcesPath(binary, platform), "utf8");
  } catch {
    return { fresh: false, reason: "the staged binary carries no sources record" };
  }
  const record = parseCargoBinarySourcesV1(text);
  if (record === null) return { fresh: false, reason: "the staged binary's sources record is malformed" };
  const verdict = cargoBinarySourcesFreshnessV1(record, modifiedAtMs);
  return verdict.fresh ? { fresh: true, reason: `${record.sources.length} sources unchanged since ${new Date(record.builtAtMs).toISOString()}` } : { fresh: false, reason: `${verdict.changed} changed after the staged build` };
}

/** 🛡️ Resolves and verifies the real executable so a missing black-box subject cannot skip green. */
export function requireMcpBinary(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const binary = resolveMcpBinaryPath(repoRoot, env, platform);
  try {
    if (!statSync(binary).isFile()) throw new Error("path is not a file");
    if (platform !== "win32") accessSync(binary, fsConstants.X_OK);
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    throw new Error(`semio-os-mcp binary gate failed at ${binary}: ${detail}`);
  }
  return binary;
}

/** 📦️ Stages `semio-os-mcp` when it is missing or stale, reports progress on stderr, returns the verified path.
 *
 * Explicit `SEMIO_OS_MCP_BIN` overrides skip staging (tests inject a subject). Freshness is the
 * staged build's own sources record ({@link mcpBinaryFreshness}): Cargo's dep-info for the binary,
 * i.e. the whole dependency closure — a kernel or plugin-host change the gateway links is as stale as
 * an edit in this module — stat-checked, so a warm tree answers in milliseconds. Any build of the neutral
 * target (a gate's `dependsOn`, `repo ./📜️script.ts build`) publishes the record with the binary, so
 * `.mcp.json`'s `initialize` never re-stages a binary a build just produced. `workspace:setup` /
 * `repo ./📜️script.ts setup` calls this to stage ahead of time; a client that starts first is served by
 * {@link serveWarmMcpStdio}, which never blocks `initialize` on a build. */
export function ensureMcpBinary(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform, staging: McpBinaryStaging = nativeMcpBinaryStagingFor(repoRoot)): string {
  if (env.SEMIO_OS_MCP_BIN) return requireMcpBinary(repoRoot, env, platform);
  const binary = resolveMcpBinaryPath(repoRoot, env, platform);
  const freshness = mcpBinaryFreshness(binary, platform);
  if (freshness.fresh) {
    console.error(`[semio-os-mcp] staged binary is fresh (${freshness.reason})`);
    return requireMcpBinary(repoRoot, env, platform);
  }
  console.error(`[semio-os-mcp] staging binary (${freshness.reason}) — progress on stderr`);
  const status = staging.stage((line) => console.error(line));
  if (status !== 0) throw new Error(`semio-os-mcp staging via ${MCP_BINARY_TARGET} exited with status ${status}`);
  console.error(`[semio-os-mcp] staged ${binary}`);
  return requireMcpBinary(repoRoot, env, platform);
}

