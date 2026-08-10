#!/usr/bin/env bun
/**
 * 🧭️ Monorepo command router: `bun ./📜️script.ts <verb> [segments…]` (e.g. `script.ts dev`, `script.ts dev mcp`, `script.ts generate neo4j compose`).
 */
import {
  Script,
  ScriptRouter,
  buildBudgetMs,
  coverageDir,
  coverageEnabled,
  daemonBudgetOpts,
  devToolingEnv,
  discoverOwners,
  discoverPackages,
  dispatchPolicyArgv,
  dispatchSubcommand,
  defineLint,
  loadTaxonomy,
  enforceCoverageThreshold,
  frameworkOsPlaygroundDevEnv,
  getWorkspaceRoot,
  goCoverageArgs,
  goLevelTestArgs,
  goProfileToLcov,
  loadFrameworkOsPlaygroundCatalog,
  mergeLcov,
  orchestratorBudgetOpts,
  parseLcov,
  renderLcov,
  resolveCliBin,
  resolveFrameworkOsPlaygroundPlugin,
  resolveTestLevel,
  runCmd,
  runCmdStatus,
  runProbe,
  runTestBudgeted,
  spawnDaemon,
  summarizeCoverage,
  semioShipEnv,
  installMicroCommitGitHooks,
  runCommit,
  runMicroCommit,
  runWorkspaceScriptMain,
  TechnologyLinter,
  TEST_LEVELS,
  tryRun,
  type BreachRecord,
  type PackageRole,
  type LcovFileRecord,
  type TestLevel,
} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { createHash } from "node:crypto";
import { existsSync, linkSync, mkdirSync, chmodSync, chownSync, copyFileSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, extname, join, relative, resolve } from "node:path";
import { createServer } from "node:net";
import { stat } from "node:fs/promises";
import { resolveActiveScopes, STORY_SCOPES } from "./.storybook/scopes.ts";

const WORKSPACE_ROOT = import.meta.dir;
(() => {
  try {
    const alias = join(WORKSPACE_ROOT, "script.ts");
    const source = readdirSync(WORKSPACE_ROOT).find((name) => /^📜️\uFE0E?script\.ts$/u.test(name));
    if (!source) return;
    if (existsSync(alias)) rmSync(alias, { force: true });
    symlinkSync(source, alias, "file");
  } catch {
    /* ignore alias maintenance failures */
  }
})();

const BUN = process.execPath;
const NATIVE_BOOTSTRAP_DIR = join(WORKSPACE_ROOT, "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap");
const REPO_CLIENT_DIR = join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client");
const REPO_CLIENT_GO = join(REPO_CLIENT_DIR, "⌨️cli");
const REPO_MCP_GO = join(REPO_CLIENT_DIR, "🔌️mcp");
process.env.NX_ISOLATE_PLUGINS ??= "false";

export { Script };

function ensureFrameworkOsPlaygroundCatalog() {
  let catalog = loadFrameworkOsPlaygroundCatalog();
  if (catalog.length > 0) return catalog;
  runCmdStatus("bun", ["nx", "run", "@semio-tech/plugin-registry:generate"], { cwd: WORKSPACE_ROOT, ...orchestratorBudgetOpts() });
  catalog = loadFrameworkOsPlaygroundCatalog();
  if (catalog.length === 0) {
    console.error("[dev] playground catalog is empty after registry generate — check @semio-tech/plugin-registry.");
    process.exit(1);
  }
  return catalog;
}

function resolvePlaygroundDevApp(segments: string[]): { readonly app: string; readonly rest: string[] } | null {
  const resolved = resolveFrameworkOsPlaygroundPlugin(ensureFrameworkOsPlaygroundCatalog(), segments);
  if (!resolved) return null;
  return { app: resolved.plugin, rest: [...resolved.rest] };
}

function runFrameworkOsPlaygroundDev(plugin: string, rest: string[] = []): void {
  runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:dev", "--", plugin, ...rest], {
    cwd: WORKSPACE_ROOT,
    env: frameworkOsPlaygroundDevEnv(ensureFrameworkOsPlaygroundCatalog(), plugin),
    ...daemonBudgetOpts(),
  });
}

//#region 🔖️NativeOsScript
/** 🖥️Runs native bootstrap shells under `repo/native/bootstrap` (setup|start). */
export class NativeOsScript extends Script {
  run(segments: string[]): void {
    const cmd = segments[0] ?? "setup";
    const env = { ...process.env, COMPOSE_REPO_ROOT: this.root };
    if (process.platform === "win32") {
      const ps1 = join(NATIVE_BOOTSTRAP_DIR, "⌨️script.ps1");
      if (!existsSync(ps1)) {
        console.error(`[native] missing ${ps1}; expected repo/native/bootstrap/⌨️script.ps1.`);
        process.exit(1);
      }
      runCmd("powershell.exe", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", ps1, cmd], { cwd: this.root, env });
      return;
    }
    if (process.platform === "darwin" || process.platform === "linux") {
      const sh = join(NATIVE_BOOTSTRAP_DIR, "⌨️script.sh");
      if (!existsSync(sh)) {
        console.error(`[native] missing ${sh}; expected repo/native/bootstrap/⌨️script.sh.`);
        process.exit(1);
      }
      runCmd("bash", [sh, cmd], { cwd: this.root, env });
      return;
    }
    console.error(`[native] unsupported platform ${process.platform}`);
    process.exit(1);
  }
}
//#endregion 🔖️NativeOsScript

//#region 🔖️SccacheSetup
const SCCACHE_VERSION = "0.10.0";

/** ⚡️Ensures `sccache` is on PATH for `.cargo/config.toml` rustc-wrapper. */
function ensureSccache(): void {
  try {
    if (runProbe("sccache", ["--version"]).status === 0) return;
  } catch {
    /* install below */
  }

  const asset = sccacheReleaseAsset();
  if (!asset) {
    console.warn("[setup] sccache auto-install unsupported on this platform; install manually.");
    return;
  }

  const binDir = process.platform === "win32" ? join(process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"), "bin") : join(homedir(), ".local", "bin");
  const binName = process.platform === "win32" ? "sccache.exe" : "sccache";
  const dest = join(binDir, binName);
  if (existsSync(dest)) return;

  const cacheDir = join(WORKSPACE_ROOT, ".🦑️repo", "⚡️cache", "sccache");
  mkdirSync(cacheDir, { recursive: true });
  const archive = join(cacheDir, asset);
  const url = `https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/${asset}`;
  console.log(`[setup] downloading sccache ${SCCACHE_VERSION}…`);
  runCmd("curl", ["-fSL", "-o", archive, url]);

  if (asset.endsWith(".tar.gz")) {
    runCmd("tar", ["-xzf", archive, "-C", cacheDir]);
    const extractedDir = readdirSync(cacheDir).find((name) => name.startsWith("sccache-") && !name.endsWith(".tar.gz"));
    if (!extractedDir) throw new Error("sccache archive extraction failed");
    copyFileSync(join(cacheDir, extractedDir, binName), dest);
  } else {
    const extractDir = join(cacheDir, "extract");
    rmSync(extractDir, { recursive: true, force: true });
    mkdirSync(extractDir, { recursive: true });
    if (process.platform === "win32") {
      runCmd("powershell.exe", ["-NoProfile", "-Command", `Expand-Archive -Path '${archive}' -DestinationPath '${extractDir}' -Force`]);
    } else {
      runCmd("unzip", ["-o", archive, "-d", extractDir]);
    }
    const extractedDir = readdirSync(extractDir).find((name) => name.startsWith("sccache-"));
    if (!extractedDir) throw new Error("sccache archive extraction failed");
    copyFileSync(join(extractDir, extractedDir, binName), dest);
  }

  if (process.platform !== "win32") chmodSync(dest, 0o755);
  console.log(`[setup] installed sccache -> ${dest}`);
}

function sccacheReleaseAsset(): string | null {
  if (process.platform === "darwin") {
    return process.arch === "arm64" ? `sccache-v${SCCACHE_VERSION}-aarch64-apple-darwin.tar.gz` : `sccache-v${SCCACHE_VERSION}-x86_64-apple-darwin.tar.gz`;
  }
  if (process.platform === "linux") {
    const report = process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } | undefined;
    const libc = report?.header?.glibcVersionRuntime ? "gnu" : "musl";
    return process.arch === "arm64" ? `sccache-v${SCCACHE_VERSION}-aarch64-unknown-linux-${libc}.tar.gz` : `sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-${libc}.tar.gz`;
  }
  if (process.platform === "win32") {
    return process.arch === "arm64" ? `sccache-v${SCCACHE_VERSION}-aarch64-pc-windows-msvc.zip` : `sccache-v${SCCACHE_VERSION}-x86_64-pc-windows-msvc.zip`;
  }
  return null;
}
//#endregion 🔖️SccacheSetup

//#region 🔖️SetupScript
export class SetupScript extends Script {
  run(segments: string[]): void {
    if (!segments[0]) {
      this.runFull();
      return;
    }
    dispatchSubcommand(
      segments,
      {
        postinstall: () => this.runPostinstall(),
        git: () => this.runGit(),
        native: (rest) => new NativeOsScript(this.root).run(rest),
      },
      "bun ./📜️script.ts setup [postinstall|git|native]",
    );
  }

  private runPostinstall(): void {
    const pkgPath = join(this.root, "node_modules", "lightningcss", "package.json");
    if (!existsSync(pkgPath)) return;
    const { version } = JSON.parse(readFileSync(pkgPath, "utf8")) as { version: string };
    const report = process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } | undefined;
    const libc = process.platform === "linux" ? (report?.header?.glibcVersionRuntime ? "gnu" : "musl") : "";
    const key = [process.platform, process.arch, libc].filter(Boolean).join("/");
    const pkgByKey: Record<string, string> = {
      "win32/x64": "lightningcss-win32-x64-msvc",
      "win32/arm64": "lightningcss-win32-arm64-msvc",
      "darwin/x64": "lightningcss-darwin-x64",
      "darwin/arm64": "lightningcss-darwin-arm64",
      "linux/x64/gnu": "lightningcss-linux-x64-gnu",
      "linux/x64/musl": "lightningcss-linux-x64-musl",
      "linux/arm64/gnu": "lightningcss-linux-arm64-gnu",
      "linux/arm64/musl": "lightningcss-linux-arm64-musl",
    };
    const platformPkg = pkgByKey[key];
    if (!platformPkg) return;
    if (existsSync(join(this.root, "node_modules", platformPkg))) return;
    runCmd("bun", ["add", "--no-save", `${platformPkg}@${version}`], { cwd: this.root });
  }

  private runGit(): void {
    runCmd("git", ["config", "--local", "core.symlinks", "true"], { cwd: this.root });
    const repoClientPath = resolveCliBin(this.root);
    if (!existsSync(repoClientPath)) {
      runCmd("go", ["build", "-o", repoClientPath, `./${REPO_MCP_GO}`], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
        budgetMs: buildBudgetMs(),
      });
    }
    runCmd(repoClientPath, ["configure"], { cwd: this.root });
    installMicroCommitGitHooks(this.root);
    const source = "AGENTS.md";
    for (const alias of ["CLAUDE.md", "GEMINI.md"]) {
      const aliasPath = join(this.root, alias);
      if (existsSync(aliasPath)) rmSync(aliasPath, { force: true });
      try {
        symlinkSync(source, aliasPath, "file");
      } catch (error) {
        if (process.platform !== "win32") throw error;
        linkSync(join(this.root, source), aliasPath);
      }
    }
    const rootScript = readdirSync(this.root).find((name) => /^📜️\uFE0E?script\.ts$/u.test(name));
    if (rootScript) {
      const scriptAlias = join(this.root, "script.ts");
      if (existsSync(scriptAlias)) rmSync(scriptAlias, { force: true });
      try {
        symlinkSync(rootScript, scriptAlias, "file");
      } catch (error) {
        if (process.platform !== "win32") throw error;
        linkSync(join(this.root, rootScript), scriptAlias);
      }
    }
  }

  private runWorkspaceCodegen(): void {
    console.log("[setup] workspace codegen…");
    const opts = { cwd: this.root, ...orchestratorBudgetOpts() };
    const nx = join(this.root, "node_modules", "nx", "bin", "nx.js");
    const runNx = (target: string) => runCmd("node", [nx, "run", target], opts);
    runNx("@semio-tech/framework-schema:generate");
    runNx("@semio-tech/ui-styling-tokens:generate");
    runCmd("bun", ["./📜️script.ts", "build"], {
      cwd: join(this.root, "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript"),
      ...orchestratorBudgetOpts(),
    });
    runNx("@semio-tech/graph-manifest:generate");
    runNx("@semio-tech/plugin-registry:generate");
  }

  private runFull(): void {
    if (process.argv.includes("--with-native-os")) {
      console.log(`[setup] ${process.platform} native bootstrap…`);
      tryRun(BUN, [join(this.root, "📜️script.ts"), "setup", "native"], { cwd: this.root });
    }

    console.log("[setup] uv sync…");
    tryRun("uv", ["sync", "--all-packages", "--all-groups"]);
    console.log("[setup] neo4j MCP server prefetch (uvx)…");
    tryRun("uvx", ["--quiet", "mcp-neo4j-cypher", "--help"]);
    console.log("[setup] cargo fetch…");
    tryRun("cargo", ["fetch", "--manifest-path", "Cargo.toml"]);
    console.log("[setup] cargo-nextest…");
    tryRun("cargo", ["install", "cargo-nextest", "--locked"]);
    console.log("[setup] C++ toolchain and vcpkg…");
    tryRun("bun", [join(this.root, "📜️script.ts"), "cpp", "setup"], { cwd: this.root });
    console.log("[setup] go build repo client…");
    const clientOut = resolveCliBin(this.root);
    tryRun("go", ["build", "-o", clientOut, `./${REPO_MCP_GO}`], { env: { ...process.env, GOWORK: join(this.root, "go.work") } });
    console.log("[setup] dotnet restore…");
    tryRun("dotnet", ["restore", "Monorepo.sln"]);
    console.log("[setup] rustup wasm target…");
    tryRun("rustup", ["target", "add", "wasm32-unknown-unknown"]);
    console.log("[setup] cargo-llvm-cov (exhaustive-level coverage)…");
    tryRun("cargo", ["install", "cargo-llvm-cov", "--locked"]);
    console.log("[setup] sccache…");
    try {
      ensureSccache();
    } catch (error) {
      console.warn("[setup] sccache install skipped:", error);
    }

    const browsersPath = join(this.root, "node_modules", ".cache", "ms-playwright");
    mkdirSync(browsersPath, { recursive: true });
    console.log("[setup] Playwright browsers…");
    tryRun("bunx", ["playwright", "install", "--with-deps", "chromium"], {
      env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsersPath },
    });

    if (process.platform === "linux") {
      const chromeSandbox = join(this.root, "node_modules", "electron", "dist", "chrome-sandbox");
      if (existsSync(chromeSandbox)) {
        try {
          chownSync(chromeSandbox, 0, 0);
          chmodSync(chromeSandbox, 0o4755);
          console.log("[setup] Electron chrome-sandbox permissions set.");
        } catch (e) {
          console.warn("[setup] chrome-sandbox chmod skipped:", e);
        }
      }
    }

    console.log("[setup] git workspace (symlinks, hook cleanup)…");
    new SetupScript(this.root).run(["git"]);

    this.runWorkspaceCodegen();

    console.log("[setup] VS Code extension build & package…");
    tryRun("bun", ["nx", "run", "@semio-tech/repo-vscode:build"], { cwd: this.root });
    tryRun("bun", ["nx", "run", "@semio-tech/repo-vscode:build-vsix"], { cwd: this.root });
    console.log("[setup] done.");
  }
}
//#endregion 🔖️SetupScript

//#region 🔖️StartScript
export class StartScript extends Script {
  run(_segments: string[]): void {
    process.chdir(this.root);
    const runGenerate = () => {
      if (runCmdStatus(BUN, [join(this.root, "📜️script.ts"), "generate"], { cwd: this.root, ...orchestratorBudgetOpts() }) !== 0) {
        console.log("[start] `bun run generate` did not refresh all `.🦑️repo/🛂️manifest` bundles (Neo4j may be offline).");
      }
    };

    if (!existsSync(join(this.root, "node_modules", "nx", "package.json"))) {
      console.log("[start] node_modules incomplete — run `bun install` and `bun ./📜️script.ts setup`.");
      return;
    }

    if (process.env.DEVCONTAINER === "true") {
      runGenerate();
      console.log("[start] Devcontainer session ready.");
      return;
    }

    if (process.platform === "win32" || process.platform === "darwin" || process.platform === "linux") {
      new NativeOsScript(this.root).run(["start"]);
    } else {
      console.log(`[start] Unsupported platform ${process.platform}.`);
    }
    runGenerate();
  }
}
//#endregion 🔖️StartScript

//#region 🔖️DevScript
export class DevScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "storybook") {
      await this.runStorybook(segments.slice(1));
      return;
    }
    if (segments[0] === "storybook-static") {
      await this.runStorybookStatic();
      return;
    }
    if (segments[0] === "s") {
      runFrameworkOsPlaygroundDev("s", segments.slice(1));
      return;
    }
    if (segments[0] === "multi") {
      // 🐚️ Not a registered playground variant (no matching Cargo.toml), so this deliberately bypasses
      // `runFrameworkOsPlaygroundDev`/`frameworkOsPlaygroundDevEnv` — those resolve `SEMIO_PLUGIN` from
      // the registry catalog, which a non-variant id would only satisfy by accident. `SEMIO_PLUGIN`
      // stays unset here on purpose (see the `os-dev` `dev multi` branch it forwards to).
      runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:dev", "--", "multi", ...segments.slice(1)], {
        cwd: this.root,
        env: { ...process.env, S_OS_PORT: process.env.S_OS_PORT || "6071", SEMIO_RENDERER: process.env.SEMIO_RENDERER ?? "react" },
        ...daemonBudgetOpts(),
      });
      return;
    }
    const playgroundApp = resolvePlaygroundDevApp(segments);
    if (playgroundApp) {
      runFrameworkOsPlaygroundDev(playgroundApp.app, playgroundApp.rest);
      return;
    }
    if (segments[0] === "mcp") {
      this.runMcp(segments.slice(1));
      return;
    }
    if (segments.length > 0) {
      console.error(`[dev] unknown playground app ${JSON.stringify(segments.join(" "))} — regenerate the catalog with bun nx run @semio-tech/plugin-registry:generate if the variant should exist.`);
      process.exit(1);
    }
    runCmd("bun", ["nx", "run", "@semio-tech/compose-desktop:dev"], { cwd: this.root, ...daemonBudgetOpts() });
  }

  private parseStorybookSegments(segments: string[]): { scope: string; args: string[] } {
    const scopeSegments: string[] = [];
    const args: string[] = [];
    let parsingScope = true;
    for (const segment of segments) {
      if (parsingScope && segment !== "--" && !segment.startsWith("-")) {
        scopeSegments.push(segment);
        continue;
      }
      parsingScope = false;
      if (segment !== "--") args.push(segment);
    }
    // A single scope arg may itself be a comma-list composing top-level scopes ("ui,compose/algorithm").
    // Multiple space-separated args keep the old slash-join behavior for hierarchical nx targets ("compose" "algorithm" -> "compose/algorithm").
    const scope = scopeSegments.length === 1 && scopeSegments[0]!.includes(",") ? scopeSegments[0]! : scopeSegments.join("/");
    const scopeIds = scope
      ? scope
          .split(",")
          .map((s) => s.trim())
          .filter(Boolean)
      : [];
    for (const id of scopeIds) {
      if (!/^[a-z0-9][a-z0-9/-]*$/i.test(id)) {
        console.error(`[dev.storybook] invalid scope ${JSON.stringify(id)}`);
        process.exit(1);
      }
    }
    if (scopeIds.length > 0) {
      try {
        resolveActiveScopes(scopeIds.join(","));
      } catch (error) {
        console.error(error instanceof Error ? `[dev.storybook] ${error.message}` : error);
        process.exit(1);
      }
    }
    return { scope: scopeIds.join(","), args };
  }

  private async runStorybook(extra: string[]): Promise<void> {
    const storybook = this.parseStorybookSegments(extra);
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    const port = process.env.STORYBOOK_PORT ?? "6010";
    const useExactPort = process.env.STORYBOOK_EXACT_PORT === "1" || process.env.STORYBOOK_EXACT_PORT === "true";
    const storybookArgs = ["storybook", "dev", "-c", ".storybook", "-p", port, ...(useExactPort ? ["--exact-port"] : []), "--host", host, "--no-open", "--debug", ...storybook.args];
    runCmd("bunx", storybookArgs, {
      cwd: this.root,
      env: {
        ...process.env,
        STORYBOOK_SCOPE: storybook.scope,
        WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
        CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
      },
      ...daemonBudgetOpts(),
    });
  }

  private async runStorybookStatic(): Promise<void> {
    const host = process.env.STORYBOOK_STATIC_HOST ?? "0.0.0.0";
    const port = Number(process.env.STORYBOOK_PORT ?? "6010");
    const documentRoot = resolve(this.root, "storybook-static");
    const server = Bun.serve({
      hostname: host,
      port,
      async fetch(request) {
        const requestUrl = new URL(request.url);
        const requestPath = decodeURIComponent(requestUrl.pathname);
        const candidatePath = resolve(documentRoot, `.${requestPath}`);
        if (!candidatePath.startsWith(documentRoot)) return new Response("Forbidden", { status: 403 });
        const filePath = await (async () => {
          try {
            const fileInfo = await stat(candidatePath);
            if (fileInfo.isDirectory()) return resolve(candidatePath, "🌐️index.html");
            return candidatePath;
          } catch {
            if (extname(candidatePath) === "") return resolve(candidatePath, "🌐️index.html");
            return candidatePath;
          }
        })();
        try {
          const file = Bun.file(filePath);
          if (!(await file.exists())) return new Response("Not Found", { status: 404 });
          return new Response(file);
        } catch {
          return new Response("Not Found", { status: 404 });
        }
      },
    });

    console.log(`storybook-static listening on http://${host}:${port}`);
    await new Promise(() => {});
  }

  private runMcp(segments: string[]): void {
    const a = segments[0];
    if (a === "engine") {
      runCmd("bun", [join(this.root, "compose", "client", "bin", "engine", "📜️script.ts"), "dev", "mcp"], { cwd: this.root, ...daemonBudgetOpts() });
      return;
    }
    if (a === "neo4j") {
      this.runMcpNeo4j(segments.slice(1));
      return;
    }
    if (a === "stdio") {
      this.runMcpStdioRepo(segments.slice(1));
      return;
    }
    const mode = a === "repo" ? "repo" : "default";
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    if (mode === "repo") {
      runCmd("npx", ["--yes", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"], {
        cwd: this.root,
        env: { ...process.env, HOST: host },
        ...daemonBudgetOpts(),
      });
      return;
    }
    runCmd("npx", ["--yes", "@modelcontextprotocol/inspector"], { cwd: this.root, ...daemonBudgetOpts() });
  }

  private runMcpNeo4j(neoSegments: string[]): void {
    const { nameParts, passthrough } = partitionNeo4jGraphCliArgv(neoSegments);
    const hasName = nameParts.length > 0;
    const graphDatabase = hasName ? joinNeo4jGraphDatabaseName(nameParts) : process.env.NEO4J_DATABASE || "compose";
    const args = [...passthrough];
    if (hasName && !args.includes("--namespace")) args.push("--namespace", graphDatabase);
    runCmd("uvx", ["mcp-neo4j-cypher", ...args], {
      cwd: this.root,
      env: {
        ...process.env,
        NEO4J_URI: process.env.NEO4J_URI || "bolt://localhost:7687",
        NEO4J_USERNAME: process.env.NEO4J_USERNAME || "neo4j",
        NEO4J_PASSWORD: process.env.NEO4J_PASSWORD || "password",
        NEO4J_DATABASE: graphDatabase,
        NEO4J_TELEMETRY: process.env.NEO4J_TELEMETRY || "false",
      },
      ...daemonBudgetOpts(),
    });
  }

  private runMcpStdioRepo(slugs: string[]): void {
    const slug = (slugs[0] ?? "client").trim().toLowerCase();
    const extra = slugs.slice(1);
    const bin = resolveCliBin(this.root);
    if (!existsSync(bin)) {
      runCmd("go", ["build", "-o", bin, `./${REPO_MCP_GO}`], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
        budgetMs: buildBudgetMs(),
      });
    }
    runCmd(bin, ["mcp", slug, ...extra], {
      cwd: this.root,
      env: { ...process.env, GOWORK: join(this.root, "go.work") },
      ...daemonBudgetOpts(),
    });
  }
}
//#endregion 🔖️DevScript

//#region 🔖️NxScript
export class NxScript extends Script {
  run(segments: string[]): void {
    runCmd("node", [join(this.root, "node_modules", "nx", "bin", "nx.js"), ...segments], {
      cwd: this.root,
      env: devToolingEnv(),
      ...orchestratorBudgetOpts(),
    });
  }
}
//#endregion 🔖️NxScript

//#region 🔖️GenerateScript
export class GenerateScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "neo4j") {
      new Neo4jCypherExport(this.root).runFromArgv(segments.slice(1));
      return;
    }
    let successes = 0;
    let failures = 0;
    const exporter = new Neo4jCypherExport(this.root);
    for (const spec of getAllNeo4jGraphExportSpecs(process.env)) {
      const joined = joinNeo4jGraphDatabaseName(spec);
      const prev = process.env.NEO4J_DATABASE;
      process.env.NEO4J_DATABASE = joined;
      try {
        if (exporter.tryExportFromArgv([...spec])) successes += 1;
        else {
          failures += 1;
          console.error(`[generate] neo4j (${joined}) failed.`);
        }
      } finally {
        if (prev === undefined) delete process.env.NEO4J_DATABASE;
        else process.env.NEO4J_DATABASE = prev;
      }
    }
    if (successes === 0) {
      console.error("[generate] no Neo4j database could be exported.");
      process.exit(1);
    }
    if (failures > 0) {
      console.error(`[generate] partial success (${successes} ok, ${failures} failed).`);
    }
    console.log(`[generate] Neo4j Cypher export finished (${successes} ok, ${failures} skipped/failed) under .🦑️repo/🛂️manifest.`);
  }
}
//#endregion 🔖️GenerateScript

//#region 🔖️LintScript
export class LintScript extends Script {
  run(segments: string[]): void {
    // nx orchestrators: exempt — total wall time spans every project and legitimately exceeds any single
    // command's budget; each leaf project's own build/test/lint commands are individually budgeted.
    if (segments[0] === "repo") {
      runCmd("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    runCmd("bun", ["nx", "run-many", "-t", "lint", "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bunx", ["dependency-cruiser@16", "compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
  }
}
//#endregion 🔖️LintScript

//#region 🔖️VerifyScript
/** 🧪️Aggregates lint + generated-catalog freshness + region/host-contract script lints (`gate`, the cheap pre-`ticket_close` step every refactor session runs), plus the full test suite for the top-level `verify` verb. */
export class VerifyScript extends Script {
  async run(segments: string[]): Promise<void> {
    await this.runGate();
    if (segments[0] === "gate") return;
    runCmd("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
  }

  private async runGate(): Promise<void> {
    // Deliberately calls dependency-cruiser directly rather than `LintScript`/`nx run-many -t lint --all`:
    // several unrelated projects (repo/client/vscode, compose-js, …) have pre-existing broken eslint configs,
    // and framework-renderer-wgpu:lint has known pending color-literal violations (see spawn_task follow-ups) —
    // this gate must stay a meaningful, currently-green signal for refactor sessions, not inherit that noise.
    console.log("[verify] dependency-cruiser boundaries…");
    runCmd("bunx", ["dependency-cruiser@16", "compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
    console.log("[verify] generated catalog freshness…");
    // nx orchestrators: exempt — leaves individually budgeted.
    runCmd("bun", ["nx", "run", "@semio-tech/plugin-registry:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] region/host-contract script lints…");
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-react:lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:plugin", "lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/ui-styling-tokens:check-no-px"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] framework ts-rs binding freshness…");
    runCmd("bun", ["nx", "run", "@semio-tech/framework-rs:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] ui locale/terminology axes freshness…");
    runCmd("bun", ["nx", "run", "@semio-tech/ui-rs:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] chrome i18n literal scan…");
    runCmd("bun", ["nx", "run", "@semio-tech/ui-react:check-chrome-i18n"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] leveled test target coverage…");
    this.checkLeveledTestTargets();
    console.log("[verify] storybook scope freshness…");
    this.checkStorybookFreshness();
    console.log("[verify] OS exclusive state authority policies…");
    {
      const osBreaches = [
        ...policyOsStateAuthorityBreaches(this.root),
        ...policyDocumentAppShapeBreaches(this.root),
      ];
      if (osBreaches.length > 0) {
        for (const b of osBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${osBreaches.length} OS exclusive state authority policy breach(es)`);
      }
    }
    console.log("[verify] handcrafted grammar P3/M4 policies…");
    {
      const handcraftedBreaches = policyHandcraftedSpecP3Breaches(this.root);
      if (handcraftedBreaches.length > 0) {
        for (const b of handcraftedBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${handcraftedBreaches.length} handcrafted-grammar P3/M4 policy breach(es)`);
      }
    }
    console.log("[verify] artifact-schema facet policies…");
    {
      const artifactSchemaBreaches = policyArtifactSchemaBreaches(this.root);
      if (artifactSchemaBreaches.length > 0) {
        for (const b of artifactSchemaBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${artifactSchemaBreaches.length} artifact-schema policy breach(es)`);
      }
    }
    console.log("[verify] app-schema facet policies…");
    {
      const appSchemaBreaches = policyAppSchemaBreaches(this.root);
      if (appSchemaBreaches.length > 0) {
        for (const b of appSchemaBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${appSchemaBreaches.length} app-schema policy breach(es)`);
      }
    }
    console.log("[verify] dissolve-core / plugin-root policies…");
    {
      const dissolveBreaches = [
        ...policyBannedNameStemBreaches(this.root),
        ...policyEmojiPrefixBreaches(this.root),
        ...policyPluginRootShapeBreaches(this.root),
        ...policyPluginBuilderBreaches(this.root, policyDiscoverCrateDirs(this.root)),
      ].filter((b) => b.priority === "high");
      if (dissolveBreaches.length > 0) {
        for (const b of dissolveBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${dissolveBreaches.length} dissolve-core / plugin-root policy breach(es)`);
      }
    }
    console.log("[verify] dsl fixture laws…");
    // Quick level here: the full repo-wide sweep (parse→print→reparse fixpoint, canonicalize
    // idempotence over every real 📚️examples fixture — @semio-tech/dsl-fixture-sweep-rs) runs at
    // `test dsl`/`test dsl exhaustive`; the gate only needs the engine crates' own quick-level unit tests.
    runCmd("bun", ["nx", "run-many", "-t", "test-quick", "-p", "@semio-tech/dsl-rs", "@semio-tech/dsl-schema-rs", "@semio-tech/dsl-derive-rs", "@semio-tech/dsl-rs"], {
      cwd: this.root,
      ...orchestratorBudgetOpts(),
    });
    console.log("[verify] gate passed.");
  }

  /** 📊️Every `project.json` with a `test` target must also declare `test-quick`/`test-long`/`test-exhaustive` —
   * otherwise `nx run-many -t test-exhaustive` silently skips that project and the exhaustive-level coverage
   * gate under-counts it. Guards against the gap this ticket closed (26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE) reopening one project.json at a time. */
  private checkLeveledTestTargets(): void {
    const offenders: string[] = [];
    const walk = (dir: string): void => {
      for (const entry of readdirSync(dir, { withFileTypes: true })) {
        if (entry.name === "node_modules" || entry.name === "target" || entry.name.startsWith(".")) continue;
        const full = join(dir, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (entry.name !== "📋️project.json") continue;
        let targets: Record<string, unknown>;
        try {
          targets = (JSON.parse(readFileSync(full, "utf8")) as { targets?: Record<string, unknown> }).targets ?? {};
        } catch {
          continue;
        }
        if (!("test" in targets)) continue;
        const missing = (["test-quick", "test-long", "test-exhaustive"] as const).filter((t) => !(t in targets));
        if (missing.length) offenders.push(`${relative(this.root, full)} missing ${missing.join(", ")}`);
      }
    };
    walk(this.root);
    if (offenders.length) {
      console.error(`[verify] ${offenders.length} project.json file(s) have a "test" target without leveled siblings:`);
      for (const o of offenders) console.error(`  ${o}`);
      process.exit(1);
    }
  }

  /** 📖️ Every `StoryScope.sourceRoots`/`storyGlobs` entry across `.storybook/scopes.ts`'s `STORY_SCOPES`
   * (both `HAND_CURATED_SCOPES` and the package-catalog-derived `GENERATED_SCOPES`) must resolve to a real
   * on-disk path — catches exactly the "stale de-emojified sourceRoot" class of bug the W0 finding in
   * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE/📋️master.md` flagged (a plugin/framework
   * dir moves mid-migration and a hand-written scope entry silently keeps pointing at the old location).
   * `GENERATED_SCOPES` entries can't go stale this way (they're re-derived from disk on every import), but
   * a package's own opt-in `sourceRoots` value CAN still name a subdir that no longer exists — checked the
   * same way. See `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`. */
  private checkStorybookFreshness(): void {
    const offenders: string[] = [];
    for (const scope of STORY_SCOPES) {
      for (const root of scope.sourceRoots) {
        if (!existsSync(join(this.root, root))) offenders.push(`scope ${JSON.stringify(scope.id)}: sourceRoot ${JSON.stringify(root)} does not exist`);
      }
      for (const glob of scope.storyGlobs ?? []) {
        const literalPrefix = glob.split(/[*{]/)[0] ?? glob;
        const literalDir = literalPrefix.endsWith("/") ? literalPrefix.slice(0, -1) : dirname(literalPrefix);
        const resolved = literalDir.startsWith("../") || literalDir.startsWith("./") ? resolve(join(this.root, ".storybook"), literalDir) : join(this.root, literalDir);
        if (!existsSync(resolved)) offenders.push(`scope ${JSON.stringify(scope.id)}: storyGlob ${JSON.stringify(glob)} has no matching directory (resolved ${JSON.stringify(relative(this.root, resolved))})`);
      }
    }
    // 🎨️ App-level Tailwind entry CSS files' `@import`/`@source` at-rules are hand-maintained literal paths
    // (Tailwind v4's content-scanning `@source` needs a real filesystem path, not a package specifier) —
    // the same bug class as a de-sandwiched plugin's old `⚡️implementations/<lang>` segment left dangling
    // (see `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`). Also assert each entry
    // reaches the shared UI stylesheet through its import chain so a future app cannot silently ship a
    // stylesheet that never scans `Layout`/`Panel`/`ShellHost` class names.
    this.checkAppTailwindEntries(offenders);
    if (offenders.length) {
      console.error(`[verify] ${offenders.length} Storybook scope / Tailwind entry path(s) are stale:`);
      for (const o of offenders) console.error(`  ${o}`);
      console.error("run-check `.storybook/scopes.ts`'s HAND_CURATED_SCOPES (or the opting-in package's own manifest / app globals.css) — see 26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG.");
      process.exit(1);
    }
  }

  /** @emoji 🎨️ Discovers every app-level Tailwind entry (`globals.css` / `🎨️globals.css`), validates
   * relative `@import`/`@source` literals resolve on disk, and asserts each entry's import chain reaches
   * the shared UI react stylesheet that owns the framework class sources. */
  private checkAppTailwindEntries(offenders: string[]): void {
    const uiGlobalsRel = this.findSharedUiGlobalsRel();
    if (!uiGlobalsRel) {
      offenders.push("shared UI react 🎨️globals.css not found under 🧰️framework/🔨️modules/🖱️ui");
      return;
    }
    const uiGlobalsAbs = join(this.root, uiGlobalsRel);
    const entries = this.listAppTailwindEntries();
    for (const entryAbs of entries) {
      const entryRel = relative(this.root, entryAbs);
      const entryDir = dirname(entryAbs);
      const body = readFileSync(entryAbs, "utf8");
      for (const rawLine of body.split("\n")) {
        const match = rawLine.match(/^@(?:import|source)\s+"([^"]+)"/);
        if (!match) continue;
        const literal = match[1]!;
        if (!literal.startsWith(".")) continue;
        const resolved = resolve(entryDir, literal);
        if (!existsSync(resolved)) offenders.push(`${entryRel}: ${JSON.stringify(literal)} does not exist`);
      }
      // Fragment stylesheets (e.g. animate present's reveal overrides) only declare `@source` / custom
      // rules and are `@import`ed by a real entry — they must not be forced to import the UI chain themselves.
      const importsShared = /@import\s+"(?:\.\.?\/|@semio-tech\/ui-styling)/.test(body);
      if (importsShared && !this.cssImportChainReaches(entryAbs, uiGlobalsAbs, new Set())) {
        offenders.push(`${entryRel}: import chain does not reach shared UI stylesheet ${JSON.stringify(uiGlobalsRel)}`);
      }
    }
  }

  /** @emoji 🎨️ App / storybook / product Tailwind entry CSS files that must inherit the shared UI sources. */
  private listAppTailwindEntries(): string[] {
    const found: string[] = [];
    const visit = (dir: string, depth: number): void => {
      if (depth > 10) return;
      let entries: ReturnType<typeof readdirSync>;
      try {
        entries = readdirSync(dir, { withFileTypes: true });
      } catch {
        return;
      }
      for (const entry of entries) {
        if (entry.name === "node_modules" || entry.name === "dist" || entry.name === "target" || entry.name === ".git") continue;
        const full = join(dir, entry.name);
        if (entry.isDirectory()) {
          visit(full, depth + 1);
          continue;
        }
        if (entry.name === "globals.css" || entry.name === "🎨️globals.css") found.push(full);
      }
    };
    for (const top of [".storybook", "compose", "♻️mit-bestand", "✏️s", "🧰️framework"]) {
      const abs = join(this.root, top);
      if (existsSync(abs)) visit(abs, 0);
    }
    return found.filter((abs) => {
      const rel = relative(this.root, abs).replace(/\\/g, "/");
      // Shared module stylesheets are sources in the chain, not app entries that must import themselves.
      if (rel.includes("/🎯️targets/⚛️react/🎨️globals.css")) return false;
      if (rel.includes("/🎨️styling/")) return false;
      return true;
    });
  }

  /** @emoji 🎨️ Relative path of the shared UI react `🎨️globals.css` from the workspace root. */
  private findSharedUiGlobalsRel(): string | null {
    const candidates = [
      "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️globals.css",
    ];
    for (const rel of candidates) {
      if (existsSync(join(this.root, rel))) return rel;
    }
    return null;
  }

  /** @emoji 🎨️ Walks relative `@import "..."` edges (skipping package specifiers) until `targetAbs` is reached. */
  private cssImportChainReaches(fromAbs: string, targetAbs: string, seen: Set<string>): boolean {
    const normalizedFrom = resolve(fromAbs);
    const normalizedTarget = resolve(targetAbs);
    if (normalizedFrom === normalizedTarget) return true;
    if (seen.has(normalizedFrom)) return false;
    seen.add(normalizedFrom);
    if (!existsSync(normalizedFrom)) return false;
    const body = readFileSync(normalizedFrom, "utf8");
    for (const rawLine of body.split("\n")) {
      const match = rawLine.match(/^@import\s+"([^"]+)"/);
      if (!match) continue;
      const literal = match[1]!;
      let nextAbs: string | null = null;
      if (literal.startsWith(".")) {
        nextAbs = resolve(dirname(normalizedFrom), literal);
      } else if (literal.startsWith("@semio-tech/ui-styling")) {
        // Package import of the styling base — the shared UI globals imports this; reaching UI globals is enough.
        continue;
      } else {
        continue;
      }
      if (this.cssImportChainReaches(nextAbs, normalizedTarget, seen)) return true;
    }
    return false;
  }
}
//#endregion 🔖️VerifyScript

//#region 🔖️FormatScript
export class FormatScript extends Script {
  run(_segments: string[]): void {
    runCmd("bunx", ["prettier", "-w", "."], { cwd: this.root, shell: true });
  }
}
//#endregion 🔖️FormatScript

//#region 🔖️TestScript
/** 🎚️Nx target name for a level — `fundamental` keeps the bare `test` target; others get a `test-<level>` suffix. */
function testTargetForLevel(level: TestLevel): string {
  return level === "fundamental" ? "test" : `test-${level}`;
}

export class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    if (rest[0] === "storybook") {
      await this.runStorybookPlaywright();
      return;
    }
    if (rest[0] === "repo-client") {
      await this.runRepoGoTest(`./${REPO_CLIENT_GO}`, level, rest.slice(1));
      return;
    }
    if (rest[0] === "repo-mcp") {
      const clientOut = resolveCliBin(this.root);
      runCmd("go", ["build", "-o", clientOut, `./${REPO_MCP_GO}`], {
        cwd: this.root,
        env: { ...process.env, GOWORK: join(this.root, "go.work") },
        budgetMs: buildBudgetMs(),
      });
      await this.runRepoGoTest(`./${REPO_CLIENT_GO}`, level, ["-run", "Mcp|MCP|mcp", ...rest.slice(1)]);
      return;
    }
    if (rest[0] === "dsl") {
      // 🗣️ DSL engine crates + the repo-wide fixture-law sweep (parse→print→reparse fixpoint,
      // canonicalize idempotence — dsl-fixture-sweep-rs) over every real shipped 📚️examples fixture.
      runCmd(
        "bun",
        [
          "nx",
          "run-many",
          "-t",
          testTargetForLevel(level),
          "-p",
          "@semio-tech/dsl-rs",
          "@semio-tech/dsl-schema-rs",
          "@semio-tech/dsl-derive-rs",
          "@semio-tech/dsl-rs",
          "@semio-tech/dsl-fixture-sweep-rs",
        ],
        {
          cwd: this.root,
          ...orchestratorBudgetOpts(),
        },
      );
      return;
    }
    const collectingCoverage = level === "exhaustive" && coverageEnabled();
    if (collectingCoverage) {
      // Stale reports from a previous run must never leak into this one's percentage — test-exhaustive is
      // already nx `cache: false`, so this is the only place that needs to clear it.
      for (const kind of ["js", "rust", "go", "py", "dotnet"] as const) rmSync(coverageDir(this.root, kind), { recursive: true, force: true });
    }

    // nx orchestrators: exempt — leaves individually budgeted.
    runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio-tech/compose-js", "@semio-tech/compose-react"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "compose/graphql:build"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run-many", "-t", testTargetForLevel(level), "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) {
      await this.runStorybookPlaywright();
    }

    if (collectingCoverage) this.enforceCoverageGate();
  }

  /** 📊️Walks every `*.lcov`/`lcov.info`/`coverage.info`/`*.cover` file under `.🦑️repo/📊️metrics/coverage/`, merges them into one repo-wide LCOV, writes `summary.json`, and hard-fails below the 95% threshold — the exhaustive-level gate. */
  private enforceCoverageGate(): void {
    const walk = (dir: string, matches: (name: string) => boolean, found: string[] = []): string[] => {
      if (!existsSync(dir)) return found;
      for (const entry of readdirSync(dir, { withFileTypes: true })) {
        const full = join(dir, entry.name);
        if (entry.isDirectory()) walk(full, matches, found);
        else if (matches(entry.name)) found.push(full);
      }
      return found;
    };
    const recordSets: LcovFileRecord[][] = [
      ...walk(coverageDir(this.root, "rust"), (n) => n.endsWith(".lcov")).map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "js"), (n) => n === "lcov.info").map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "py"), (n) => n.endsWith(".lcov")).map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "dotnet"), (n) => n === "coverage.info").map((f) => parseLcov(readFileSync(f, "utf8"))),
      ...walk(coverageDir(this.root, "go"), (n) => n.endsWith(".cover")).map((f) => goProfileToLcov(readFileSync(f, "utf8"))),
    ];
    const merged = mergeLcov(recordSets);
    const summary = summarizeCoverage(merged);
    writeFileSync(join(this.root, ".🦑️repo", "📊️metrics", "coverage", "lcov.info"), renderLcov(merged));
    writeFileSync(join(this.root, ".🦑️repo", "📊️metrics", "coverage", "summary.json"), JSON.stringify(summary, null, 2));
    enforceCoverageThreshold(summary, 95);
  }

  private async waitForUrl(url: string, timeoutMs: number): Promise<void> {
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {
      try {
        const response = await fetch(url);
        if (response.ok) return;
      } catch {
        /* retry */
      }
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
    throw new Error(`Timed out waiting for ${url}`);
  }

  private isTcpPortFree(port: number, host: string): Promise<boolean> {
    return new Promise((resolve) => {
      const server = createServer();
      server.unref();
      server.once("error", () => resolve(false));
      server.listen(port, host, () => {
        server.close(() => resolve(true));
      });
    });
  }

  private async pickStorybookStaticPort(preferred: number, span: number): Promise<number> {
    for (let port = preferred; port < preferred + span; port += 1) {
      if (await this.isTcpPortFree(port, "0.0.0.0")) return port;
    }
    throw new Error(`No free TCP port in ${preferred}..${preferred + span - 1}`);
  }

  /** ⏱️`goLevelTestArgs` keeps `-short` (skipping the `testing.Short()`-gated real-monorepo-scan tests in `repo/client/cli/go/main_test.go`) through `quick` and adds a cumulative `-skip` above the requested level. */
  private async runRepoGoTest(module: string, level: TestLevel, extraArgs: string[]): Promise<void> {
    await runTestBudgeted("go", ["test", module, ...goLevelTestArgs(level), ...goCoverageArgs(this.root, module), ...extraArgs], {
      cwd: this.root,
      env: { ...process.env, GOWORK: join(this.root, "go.work") },
    });
  }

  private async runStorybookPlaywright(): Promise<void> {
    const preferred = Number(process.env.STORYBOOK_PORT ?? 6010);
    const storybookPort = String(await this.pickStorybookStaticPort(preferred, 50));
    const baseUrl = `http://127.0.0.1:${storybookPort}/`;
    runCmd("bun", [join(this.root, "📜️script.ts"), "build", "storybook"], { cwd: this.root, ...orchestratorBudgetOpts() });
    const server = spawnDaemon("bun", [join(this.root, "📜️script.ts"), "dev", "storybook-static"], {
      cwd: this.root,
      env: { ...process.env, STORYBOOK_PORT: storybookPort },
    });
    try {
      await this.waitForUrl(new URL("🌐️index.html", baseUrl).href, 120000);
      runCmd("bunx", ["playwright", "test", ".storybook/puzzle-2d.spec.ts", "--config", ".storybook/playwright.config.ts"], {
        cwd: this.root,
        env: {
          ...process.env,
          PLAYWRIGHT_BASE_URL: baseUrl,
          PLAYWRIGHT_BROWSERS_PATH: process.env.PLAYWRIGHT_BROWSERS_PATH ?? `${this.root}/node_modules/.cache/ms-playwright`,
          STORYBOOK_PORT: storybookPort,
        },
      });
    } finally {
      server.kill();
    }
  }
}
//#endregion 🔖️TestScript

//#region 🔖️BuildScript
export class BuildScript extends Script {
  run(segments: string[]): void {
    const slice = segments[0];
    const single: Record<string, string> = {
      "3dm": "@semio-tech/compose-3dm-ui:build",
      assets: "@semio-tech/assets:build",
      desktop: "@semio-tech/compose-desktop:build",
      engine: "@semio-tech/compose-engine:build",
      storybook: "workspace:build-storybook",
      "coda-desktop": "@semio-tech/coda-desktop:build",
      "repo-cli": "@semio-tech/repo-client:build",
      "repo-server": "@semio-tech/repo-coordinator:build",
      "repo-vscode": "@semio-tech/repo-vscode:build-vsix",
    };

    // nx orchestrators: exempt — leaves individually budgeted.
    if (!slice) {
      runCmd("bun", ["nx", "run-many", "-t", "build", "--all", "--exclude", "workspace"], { cwd: this.root, env: semioShipEnv(), ...orchestratorBudgetOpts() });
      runCmd("bun", ["nx", "run", "workspace:build-storybook"], { cwd: this.root, env: semioShipEnv(), ...orchestratorBudgetOpts() });
      return;
    }
    if (slice === "storybook") {
      runCmd("bunx", ["storybook", "build", "-c", ".storybook", "--output-dir", "storybook-static"], { cwd: this.root });
      return;
    }
    if (slice === "sites") {
      runCmd("bun", ["nx", "run-many", "-t", "build", "-p", "@semio-tech/compose-sketchpad-play", "@semio-tech/compose-sketchpad-docs"], { cwd: this.root, env: semioShipEnv(), ...orchestratorBudgetOpts() });
      return;
    }
    const target = single[slice];
    if (!target) {
      console.error(`[build] unknown slice ${JSON.stringify(slice)}`);
      process.exit(1);
    }
    runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}
//#endregion 🔖️BuildScript

//#region 🔖️CppScriptHelpers
const WINDOWS_CMAKE_GENERATOR = "Visual Studio 18 2026";

function vswhereExecutable(): string {
  const programFilesX86 = process.env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)";
  return join(programFilesX86, "Microsoft Visual Studio", "Installer", "vswhere.exe");
}

function queryVisualStudio2026InstallPath(): string | undefined {
  if (process.platform !== "win32") return undefined;
  const vswhere = vswhereExecutable();
  if (!existsSync(vswhere)) return undefined;
  const result = runProbe(vswhere, ["-latest", "-version", "[18.0,19.0)", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"]);
  if (result.status !== 0) return undefined;
  const installPath = result.stdout.trim();
  return installPath || undefined;
}
//#endregion 🔖️CppScriptHelpers

//#region 🔖️CppScript
export class CppScript extends Script {
  run(segments: string[]): void {
    const preset = this.resolvePreset(segments.slice(1));
    dispatchSubcommand(
      segments,
      {
        setup: () => this.runSetup(),
        configure: () => this.runConfigure(preset),
        build: () => this.runBuild(preset),
        test: () => this.runTest(preset),
        all: () => {
          this.runSetup();
          this.runConfigure(preset);
          this.runBuild(preset);
          this.runTest(preset);
        },
      },
      "bun ./📜️script.ts cpp [setup|configure|build|test|all] [preset]",
      "all",
    );
  }

  private resolvePreset(segments: string[]): string {
    const explicit = segments.find((segment) => !segment.startsWith("-"));
    if (explicit) return explicit;
    if (process.env.CMAKE_PRESET) return process.env.CMAKE_PRESET;
    if (process.platform === "win32") return "windows";
    if (process.platform === "darwin") return "macos";
    return "linux";
  }

  private runSetup(): void {
    this.ensureTool("cmake", "cmake");
    if (process.platform !== "win32") this.ensureTool("ninja", "ninja");
    this.ensureVcpkg();
    if (process.platform === "win32") this.ensureWindowsMsvc();
  }

  private runConfigure(preset: string): void {
    this.runSetup();
    this.purgeStaleCmakeCache(preset);
    runCmd(this.resolveTool("cmake"), ["--preset", preset], { cwd: this.root, env: this.cppEnv(), budgetMs: buildBudgetMs() });
  }

  private runBuild(preset: string): void {
    runCmd(this.resolveTool("cmake"), ["--build", "--preset", preset], { cwd: this.root, env: this.cppEnv(), budgetMs: buildBudgetMs() });
  }

  private runTest(preset: string): void {
    runCmd(this.resolveTool("ctest"), ["--preset", preset], { cwd: this.root, env: this.cppEnv(), budgetMs: buildBudgetMs() });
  }

  private ensureTool(command: string, uvTool: string): void {
    if (this.hasTool(command)) return;
    if (this.hasTool("uv")) {
      runCmd(this.resolveTool("uv"), ["tool", "install", "--upgrade", uvTool], { cwd: this.root });
      if (this.hasTool(command)) return;
    }
    console.error(`[cpp] ${command} is required. Run the native bootstrap or rebuild the devcontainer.`);
    process.exit(1);
  }

  private hasTool(command: string): boolean {
    try {
      return runProbe(this.resolveTool(command), ["--version"], { env: this.cppEnv() }).status === 0;
    } catch {
      return false;
    }
  }

  private resolveTool(command: string): string {
    const extension = process.platform === "win32" ? ".exe" : "";
    const candidates = [command, join(homedir(), ".local", "bin", `${command}${extension}`), join(homedir(), ".local", "bin", command)];
    return candidates.find((candidate) => existsSync(candidate)) ?? command;
  }

  private ensureVcpkg(): void {
    const vcpkgRoot = this.vcpkgRoot();
    const vcpkgExe = join(vcpkgRoot, process.platform === "win32" ? "vcpkg.exe" : "vcpkg");
    if (!existsSync(vcpkgRoot)) {
      mkdirSync(join(this.root, ".🦑️repo", "⚡️cache"), { recursive: true });
      runCmd("git", ["clone", "--depth", "1", "https://github.com/microsoft/vcpkg.git", vcpkgRoot], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
    if (!existsSync(vcpkgExe)) {
      if (process.platform === "win32") {
        runCmd("cmd.exe", ["/c", join(vcpkgRoot, "bootstrap-vcpkg.bat"), "-disableMetrics"], { cwd: vcpkgRoot, budgetMs: buildBudgetMs() });
      } else {
        runCmd("bash", [join(vcpkgRoot, "bootstrap-vcpkg.sh"), "-disableMetrics"], { cwd: vcpkgRoot, budgetMs: buildBudgetMs() });
      }
    }
  }

  private cppEnv(): NodeJS.ProcessEnv {
    return {
      ...devToolingEnv(),
      CMAKE_BUILD_PARALLEL_LEVEL: process.env.CMAKE_BUILD_PARALLEL_LEVEL ?? "4",
      VCPKG_ROOT: this.vcpkgRoot(),
      VCPKG_DISABLE_METRICS: "1",
      VCPKG_MAX_CONCURRENCY: process.env.VCPKG_MAX_CONCURRENCY ?? "4",
    };
  }

  private vcpkgRoot(): string {
    return process.env.VCPKG_ROOT || join(this.root, ".🦑️repo", "⚡️cache", "vcpkg");
  }

  private ensureWindowsMsvc(): void {
    if (process.platform !== "win32") return;
    if (queryVisualStudio2026InstallPath()) return;
    console.error("[cpp] Visual Studio 2026 with the Desktop development with C++ workload is required.");
    console.error("[cpp] On native Windows run: bun ./📜️script.ts setup native");
    process.exit(1);
  }

  private purgeStaleCmakeCache(preset: string): void {
    const cacheDir = join(this.root, ".🦑️repo", "⚡️cache", "cmake", preset);
    const cacheFile = join(cacheDir, "CMakeCache.txt");
    if (!existsSync(cacheFile)) return;
    const content = readFileSync(cacheFile, "utf8");
    const generatorMatch = content.match(/^CMAKE_GENERATOR:INTERNAL=(.+)$/m);
    const cachedGenerator = generatorMatch?.[1]?.trim();
    if (process.platform === "win32" && cachedGenerator && cachedGenerator !== WINDOWS_CMAKE_GENERATOR) {
      console.log(`[cpp] Removing stale CMake cache for preset "${preset}" (generator ${cachedGenerator}).`);
      rmSync(cacheDir, { recursive: true, force: true });
    }
  }
}
//#endregion 🔖️CppScript

//#region 🔖️PublishScript
export class PublishScript extends Script {
  run(segments: string[]): void {
    const slice = segments[0];
    const map: Record<string, string> = {
      desktop: "@semio-tech/compose-desktop:publish",
      play: "@semio-tech/compose-sketchpad-play:publish",
      sketchpad: "@semio-tech/compose-sketchpad:publish",
      docs: "@semio-tech/compose-sketchpad-docs:publish",
      "coda-desktop": "@semio-tech/coda-desktop:publish",
    };
    if (!slice) {
      console.error(`[publish] usage: bun ./📜️script.ts publish <${Object.keys(map).join(" | ")}>`);
      process.exit(1);
    }
    const target = map[slice];
    if (!target) {
      console.error(`[publish] unknown slice ${JSON.stringify(slice)}`);
      process.exit(1);
    }
    runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() }); // nx orchestrator — leaves individually budgeted
  }
}
//#endregion 🔖️PublishScript

//#region 🔖️QueryScript
export class QueryScript extends Script {
  run(segments: string[]): void {
    const sub = segments[0] ?? "test";
    const queryDir = join(this.root, "compose/client/lib/query");
    if (sub === "build") {
      runCmd(bun, [join(queryDir, "📜️script.ts"), "build"], { cwd: this.root });
      return;
    }
    if (sub === "wasm") {
      runCmd(bun, [join(queryDir, "📜️script.ts"), "wasm"], { cwd: this.root });
      return;
    }
    if (sub === "test") {
      runCmd(bun, [join(queryDir, "📜️script.ts"), "test", ...segments.slice(1)], { cwd: this.root });
      return;
    }
    console.error(`[query] unknown subcommand ${JSON.stringify(sub)}`);
    process.exit(1);
  }
}
//#endregion 🔖️QueryScript

//#region 🔖️PurgeScript
export class PurgeScript extends Script {
  run(segments: string[]): void {
    if (segments[0] !== "neo4j") {
      console.error("[purge] usage: bun ./📜️script.ts purge neo4j");
      process.exit(1);
    }
    const database = process.env.NEO4J_DATABASE || "compose";
    const uri = process.env.NEO4J_URI || "bolt://localhost:7687";
    const user = process.env.NEO4J_USERNAME || "neo4j";
    const password = process.env.NEO4J_PASSWORD || "password";
    if (runCmdStatus("cypher-shell", ["-a", uri, "-u", user, "-p", password, "-d", database, "--format", "plain", "RETURN 1 AS ok;"], { cwd: this.root }) !== 0) {
      console.warn("[purge.neo4j] cypher-shell unavailable — skip.");
      process.exit(0);
    }
    console.log("[purge.neo4j] connectivity ok; no_operation.");
  }
}
//#endregion 🔖️PurgeScript

//#region 🔖️MicroCommitScript
/** 🎆️Stages WIP changes and writes deterministic micro-commit templates (GitKraken + CLI). */
export class MicroCommitScript extends Script {
  run(segments: string[]): void {
    runMicroCommit(this.root, segments);
  }
}
//#endregion 🔖️MicroCommitScript

//#region 🔖️CommitScript
/** 🔀️Bundle micro-commits into a signed squash commit with per-bundle summaries. */
export class CommitScript extends Script {
  run(segments: string[]): void {
    runCommit(this.root, segments);
  }
}
//#endregion 🔖️CommitScript

//#region 🔖️OsScript
/** 🧩️One `framework/os/dev` plugin registry row ([[🔣️plugins.json]]) — only the fields `os run`'s preflight needs. */
type OsPluginArtifact = { pluginId: string; wasmOut: string };

/**
 * 🔍️Plugin ids from the generated plugin registry with no built `.wasm` under
 * `target/wasm32-wasip2/{debug,wasm-release}/` — same resolution order as `resolve_plugin_paths` in
 * `semio-framework-os-run`.
 */
const PLUGIN_WASM_TARGET_DIR = "target/wasm32-wasip2";
const PLUGIN_WASM_PROFILE_DIRS = ["debug", "wasm-release"] as const;

function pluginWasmArtifactExists(repoRoot: string, wasmOut: string): boolean {
  for (const profileDir of PLUGIN_WASM_PROFILE_DIRS) {
    if (existsSync(join(repoRoot, PLUGIN_WASM_TARGET_DIR, profileDir, wasmOut))) return true;
  }
  return false;
}

function missingPluginWasmArtifacts(repoRoot: string): string[] {
  const registryPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🔣️plugins.json");
  if (!existsSync(registryPath)) return [];
  const entries = JSON.parse(readFileSync(registryPath, "utf8")) as OsPluginArtifact[];
  return entries.filter((entry) => !pluginWasmArtifactExists(repoRoot, entry.wasmOut)).map((entry) => entry.pluginId);
}

/** 🕸️Headless OS studio commands — computes a workflow without a UI (`os run <bundle>.studio`). */
export class OsScript extends Script {
  run(segments: string[]): void {
    const sub = segments[0];
    if (sub === "run") {
      const rest = segments.slice(1);
      const bundle = rest.find((segment) => !segment.startsWith("--"));
      if (!bundle) {
        console.error("[os.run] usage: bun ./📜️script.ts os run <bundle>.studio [--node <id>] [--watch] [--dry]");
        process.exit(1);
      }
      const repoRoot = getWorkspaceRoot();
      if (!rest.includes("--dry")) {
        const missing = missingPluginWasmArtifacts(repoRoot);
        if (missing.length > 0) {
          console.error(`[os.run] missing built plugin artifact(s): ${missing.join(", ")}`);
          for (const pluginId of missing) console.error(`[os.run]   build it: bun nx run @semio-tech/framework-os-dev:build -- ${pluginId}`);
          process.exit(1);
        }
      }
      const watch = rest.includes("--watch");
      runCmd("cargo", ["run", "-p", "semio-framework-os-run", "--", ...rest], {
        cwd: this.root,
        env: { ...process.env, SEMIO_REPO_ROOT: repoRoot },
        ...(watch ? daemonBudgetOpts() : { budgetMs: buildBudgetMs() }),
      });
      return;
    }
    console.error(`[os] unknown subcommand ${JSON.stringify(sub)}`);
    process.exit(1);
  }
}
//#endregion 🔖️OsScript

//#region 🔖️SemioScript
/** @emoji 🧬 Universal `.semio` file processor (`inspect`, `verify`, `open`, `convert`). */
export class SemioScript extends Script {
  run(segments: string[]): void {
    if (segments.length === 0) {
      console.error("[semio] usage: bun ./📜️script.ts semio <inspect|verify|open|convert> <path>");
      process.exit(1);
    }
    runCmd("cargo", ["run", "-p", "semio-framework-os-kernel-semio", "--bin", "semio", "--", ...segments], {
      cwd: this.root,
      budgetMs: buildBudgetMs(),
    });
  }
}
//#endregion 🔖️SemioScript

//#region 🔖️ExamplesScript
/** 📚️Lists and verifies plugin example units against the emoji-slug assets/tests shape. */
export class ExamplesScript extends Script {
  run(segments: string[]): void {
    const sub = segments[0];
    if (!sub || sub === "help") {
      console.error("[examples] usage: bun ./📜️script.ts examples <list|verify> [plugin]");
      process.exit(sub ? 0 : 1);
    }
    if (sub === "list") {
      this.listExamples(segments[1]);
      return;
    }
    if (sub === "verify") {
      this.verifyExamples(segments[1]);
      return;
    }
    console.error(`[examples] unknown subcommand ${JSON.stringify(sub)}`);
    process.exit(1);
  }

  private listExamples(pluginFilter?: string): void {
    const taxonomy = loadTaxonomy();
    const roots = this.collectExampleRoots(pluginFilter);
    if (roots.length === 0) {
      console.log("[examples.list] (none)");
      return;
    }
    for (const root of roots) {
      const slug = root.split("/").pop() ?? root;
      const hasAssets = existsSync(join(this.root, root, taxonomy.exampleAssetsDirName));
      const hasTests = existsSync(join(this.root, root, taxonomy.exampleTestsDirName));
      const rustLeaf = taxonomy.exampleLeafFilenames?.["🦀️rust"] ?? "🦀️component.rs";
      const hasLeaf = existsSync(join(this.root, root, rustLeaf));
      console.log(`${root}  leaf=${hasLeaf ? "yes" : "no"} assets=${hasAssets ? "yes" : "no"} tests=${hasTests ? "yes" : "no"} slug=${slug}`);
    }
    console.log(`[examples.list] ${roots.length} example unit(s)`);
  }

  private verifyExamples(pluginFilter?: string): void {
    const crateDirs = policyDiscoverCrateDirs(this.root);
    const filtered = pluginFilter
      ? crateDirs.filter((crate) => {
          const id = crate.pluginId || policyStripEmoji(crate.ownerRel.split("/").pop() ?? "");
          const ascii = policyStripEmoji(pluginFilter);
          return id === pluginFilter || id === ascii || crate.ownerRel.endsWith(`/${pluginFilter}`) || policyStripEmoji(crate.ownerRel).includes(ascii);
        })
      : crateDirs;
    const ownerAllow = new Set(filtered.map((crate) => crate.ownerRel));
    const inScope = (scope: string): boolean => {
      if (!pluginFilter) return true;
      const ascii = policyStripEmoji(pluginFilter);
      if (scope.includes(pluginFilter) || policyStripEmoji(scope).includes(ascii)) return true;
      for (const owner of ownerAllow) {
        if (scope.startsWith(`${owner}/`) || scope === owner) return true;
      }
      return false;
    };
    const breaches = [
      ...policySemioArtifactExamplesBreaches(this.root, filtered),
      ...policyDeadExampleLeafBreaches(this.root, filtered).filter((breach) => inScope(breach.scope)),
      ...policyEmptyExampleBreaches(this.root).filter((breach) => inScope(breach.scope)),
    ];
    if (breaches.length === 0) {
      console.log(`[examples.verify] ok${pluginFilter ? ` (${pluginFilter})` : ""}`);
      return;
    }
    for (const breach of breaches) {
      console.error(`[examples.verify] ${breach.priority} ${breach.kind}: ${breach.summary}`);
    }
    console.error(`[examples.verify] ${breaches.length} breach(es)`);
    process.exit(1);
  }

  private collectExampleRoots(pluginFilter?: string): string[] {
    const taxonomy = loadTaxonomy();
    const roots: string[] = [];
    const pluginsRoot = "✏️s/🔌️plugins";
    let plugins: string[] = [];
    try {
      plugins = readdirSync(join(this.root, pluginsRoot)).filter((name) => {
        const abs = join(this.root, pluginsRoot, name);
        return existsSync(abs) && statSync(abs).isDirectory();
      });
    } catch {
      return roots;
    }
    if (pluginFilter) {
      const ascii = policyStripEmoji(pluginFilter);
      plugins = plugins.filter((name) => name === pluginFilter || policyStripEmoji(name) === ascii || name.includes(pluginFilter));
    }
    for (const plugin of plugins) {
      const owner = `${pluginsRoot}/${plugin}`;
      for (const kind of [taxonomy.artifactsDirName, taxonomy.appsDirName] as const) {
        const container = join(this.root, owner, kind);
        if (!existsSync(container)) continue;
        for (const child of readdirSync(container)) {
          const examplesRel = `${owner}/${kind}/${child}/📚️examples`;
          const examplesAbs = join(this.root, examplesRel);
          if (!existsSync(examplesAbs) || !statSync(examplesAbs).isDirectory()) continue;
          for (const slug of readdirSync(examplesAbs)) {
            const slugRel = `${examplesRel}/${slug}`;
            if (!statSync(join(this.root, slugRel)).isDirectory()) continue;
            roots.push(slugRel);
          }
        }
      }
    }
    return roots.sort();
  }
}
//#endregion 🔖️ExamplesScript

//#region 🔖️Dispatch
const router = new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT)
  .register("os", OsScript)
  .register("semio", SemioScript)
  .register("examples", ExamplesScript)
  .register("nx", NxScript)
  .register("setup", SetupScript)
  .register("start", StartScript)
  .register("dev", DevScript)
  .register("generate", GenerateScript)
  .register("lint", LintScript)
  .register("verify", VerifyScript)
  .register("format", FormatScript)
  .register("test", TestScript)
  .register("build", BuildScript)
  .register("cpp", CppScript)
  .register("publish", PublishScript)
  .register("purge", PurgeScript)
  .register("query", QueryScript)
  .register("micro-commit", MicroCommitScript)
  .register("commit", CommitScript);

//#endregion 🔖️Dispatch

//#region 🔖️generate-neo4j-gen
/**
 * 🛂️ Neo4j → `.🦑️repo/🛂️manifest/<graph>.cypher` export (pure module; invoked from root `script.ts`). Product graphs are fixed specs; extra Bolt graphs use `NEO4J_EXTRA_GRAPH_DATABASES` (comma-separated). Argv segments join with `-` via `joinNeo4jGraphDatabaseName`.
 */
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const NEO4J_VERSION = "5.26.26";

/** 🏗️Product graphs only (compose stack); not arbitrary developer databases. */
export const NEO4J_PRODUCT_GRAPH_DATABASE_SPECS = [["compose"], ["elements"], ["coda"], ["reuse"]] as const;

/** 🗑️Env key: comma-separated extra Bolt graph names for `bun run generate` and native `.🦑️repo/🛂️manifest/*.cypher` stubs. */
export const NEO4J_EXTRA_GRAPH_DATABASES_ENV = "NEO4J_EXTRA_GRAPH_DATABASES";

/** 🔗️Bolt user graph name from argv segments after `neo4j` / `generate neo4j` (hyphen join). */
export function joinNeo4jGraphDatabaseName(parts: readonly string[]): string {
  return parts.join("-");
}

/** 🔀️Parses `NEO4J_EXTRA_GRAPH_DATABASES` into trimmed non-empty graph names. */
export function parseExtraNeo4jGraphDatabaseNamesFromEnv(env: NodeJS.ProcessEnv = process.env): string[] {
  const raw = env[NEO4J_EXTRA_GRAPH_DATABASES_ENV]?.trim();
  if (!raw) return [];
  return raw
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

/** 📋️Product graph argv rows plus `[name]` per extra env entry. */
export function getAllNeo4jGraphExportSpecs(env: NodeJS.ProcessEnv = process.env): string[][] {
  const core: string[][] = NEO4J_PRODUCT_GRAPH_DATABASE_SPECS.map((row) => [...row]);
  const extras = parseExtraNeo4jGraphDatabaseNamesFromEnv(env).map((n) => [n]);
  return [...core, ...extras];
}

/** 🧾️Bolt graph names allowed for `generate neo4j …` (product + extras). */
export function neo4jExportDatabaseNameSet(env: NodeJS.ProcessEnv = process.env): Set<string> {
  return new Set(getAllNeo4jGraphExportSpecs(env).map((spec) => joinNeo4jGraphDatabaseName(spec)));
}

/** @deprecated Prefer `getAllNeo4jGraphExportSpecs`; product-only joined names. */
export const NEO4J_GRAPH_DATABASE_NAMES = NEO4J_PRODUCT_GRAPH_DATABASE_SPECS.map((s) => joinNeo4jGraphDatabaseName(s));

export type Neo4jGraphDatabaseName = (typeof NEO4J_GRAPH_DATABASE_NAMES)[number];

export function partitionNeo4jGraphCliArgv(segments: string[]): { nameParts: string[]; passthrough: string[] } {
  const nameParts: string[] = [];
  let i = 0;
  while (i < segments.length && !segments[i]!.startsWith("-")) {
    nameParts.push(segments[i]!);
    i += 1;
  }
  return { nameParts, passthrough: segments.slice(i) };
}

export class Neo4jCypherExport {
  constructor(private readonly repoRoot: string) {}

  resolveCypherShell(): string | null {
    const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
    const cachedShell = join(this.repoRoot, ".🦑️repo", "⚡️cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
    const candidates = [process.env.NEO4J_CYPHER_SHELL, cachedShell, runtimeName].filter((value): value is string => Boolean(value));

    for (const candidate of candidates) {
      if (candidate.includes("/") || candidate.includes("\\")) {
        if (existsSync(candidate)) return candidate;
        continue;
      }
      try {
        if (runProbe(candidate, ["--version"]).status === 0) return candidate;
      } catch {
        /* try next */
      }
    }
    return null;
  }

  buildCypherEnv(): NodeJS.ProcessEnv {
    const env = { ...process.env };
    if (process.platform === "win32") {
      const javaHome = "C:\\Program Files\\Microsoft\\jdk-21.0.11.10-hotspot";
      const javaExecutable = join(javaHome, "bin", "java.exe");
      if (existsSync(javaExecutable)) {
        env.JAVA_HOME = javaHome;
        env.Path = `${join(javaHome, "bin")};${env.Path || ""}`;
      }
    }
    return env;
  }

  runCypher(database: string, cypher: string): { ok: boolean; stdout: string; stderr: string } {
    const shell = this.resolveCypherShell();
    if (!shell) {
      return { ok: false, stdout: "", stderr: "cypher-shell not found (install Neo4j tools or set NEO4J_CYPHER_SHELL)." };
    }

    const queryDir = join(this.repoRoot, ".🦑️repo", "⚡️cache");
    mkdirSync(queryDir, { recursive: true });
    const queryPath = join(queryDir, `neo4j-generate-query-${process.pid}-${Date.now()}.cypher`);
    writeFileSync(queryPath, `${cypher.trim()}\n`, "utf8");

    try {
      const result = runProbe(shell, ["-a", process.env.NEO4J_URI || "bolt://localhost:7687", "-u", process.env.NEO4J_USERNAME || "neo4j", "-p", process.env.NEO4J_PASSWORD || "password", "-d", database, "--format", "plain", "-f", queryPath], {
        cwd: this.repoRoot,
        env: this.buildCypherEnv(),
      });

      return {
        ok: result.status === 0,
        stdout: result.stdout,
        stderr: result.stderr,
      };
    } finally {
      try {
        unlinkSync(queryPath);
      } catch {
        /* temp query cleanup */
      }
    }
  }

  apocExportCypherAllToAbsoluteFile(database: string, absoluteFile: string): { ok: boolean; message: string } {
    const neoPath = absoluteFile.replace(/\\/g, "/");
    const apocTarget = /^[A-Za-z]:\//.test(neoPath) ? `file:${neoPath}` : neoPath.startsWith("/") ? `file://${neoPath}` : neoPath;
    const pathLiteral = JSON.stringify(apocTarget);
    const cypher = [
      `CALL apoc.export.cypher.all(${pathLiteral}, {`,
      `  format: "cypher-shell",`,
      `  writeNodeProperties: true,`,
      `  ifNotExists: true,`,
      `  useOptimizations: { type: "UNWIND_BATCH", unwindBatchSize: 100 }`,
      `})`,
      `YIELD file, batches, source, format, nodes, relationships, properties, time, rows, batchSize`,
      `RETURN file, batches, source, format, nodes, relationships, properties, time, rows, batchSize;`,
    ].join("\n");

    const { ok, stdout, stderr } = this.runCypher(database, cypher);
    if (!ok) {
      return {
        ok: false,
        message: `${stderr || stdout || "unknown error"}\n` + "Ensure APOC is installed, apoc.export.file.enabled=true, and Neo4j may write this absolute path (set apoc.import.file.use_neo4j_config=false on Desktop — see setup scripts).",
      };
    }
    return { ok: true, message: stdout.trim() };
  }

  writeGeneratedCypherBundle(technology: string, database: string, body: string, finalPath: string): void {
    const stamp = new Date().toISOString();
    const header = [
      "// SPDX-License-Identifier: AGPL-3.0-only",
      "// Generated exclusively from the live Neo4j database — do not edit this file by hand.",
      "// Refresh: `bun run generate` (root `script.ts`).",
      `// graph: ${technology} | database: ${database} | generated: ${stamp}`,
      "//",
      "",
    ].join("\n");

    writeFileSync(finalPath, `${header}${body.trim()}\n`, "utf8");
  }

  tryExportFromArgv(argv: string[]): boolean {
    const { nameParts, passthrough } = partitionNeo4jGraphCliArgv(argv);
    if (passthrough.length > 0) {
      console.error(`[generate:neo4j] unexpected extra arguments (use only graph name segments before any -flags): ${JSON.stringify(passthrough)}`);
      return false;
    }
    const joined = nameParts.length > 0 ? joinNeo4jGraphDatabaseName(nameParts) : (process.env.NEO4J_DATABASE ?? "compose");
    const allowed = neo4jExportDatabaseNameSet(process.env);
    if (!allowed.has(joined)) {
      const hint = parseExtraNeo4jGraphDatabaseNamesFromEnv(process.env).length === 0 ? ` Set ${NEO4J_EXTRA_GRAPH_DATABASES_ENV} to a comma-separated list of extra Bolt graph names (e.g. metabolism,mydb).` : "";
      console.error(`[generate:neo4j] graph database must be one of: ${[...allowed].sort().join(", ")} (got ${JSON.stringify(joined)}; argv segments ${JSON.stringify(nameParts)}).${hint}`);
      return false;
    }

    const technology = joined;
    const database = process.env.NEO4J_DATABASE ?? joined;
    const outDir = join(this.repoRoot, ".🦑️repo", "🛂️manifest");
    mkdirSync(outDir, { recursive: true });

    const finalAbs = join(outDir, `${technology}.cypher`);
    const cacheDir = join(this.repoRoot, ".🦑️repo", "⚡️cache");
    mkdirSync(cacheDir, { recursive: true });
    const tmpAbs = join(cacheDir, `.generate-${technology}-${process.pid}.tmp.cypher`);

    const probe = this.runCypher(database, "RETURN 1 AS ok;");
    if (!probe.ok) {
      console.error(`[generate:neo4j] cannot reach database ${JSON.stringify(database)}:\n${probe.stderr || probe.stdout}`);
      return false;
    }

    if (existsSync(tmpAbs)) unlinkSync(tmpAbs);

    const result = this.apocExportCypherAllToAbsoluteFile(database, tmpAbs);
    if (!result.ok) {
      console.error(`[generate:neo4j] apoc.export.cypher.all failed:\n${result.message}`);
      return false;
    }

    if (!existsSync(tmpAbs)) {
      console.error(`[generate:neo4j] expected export file missing at ${tmpAbs} after APOC call.`);
      return false;
    }

    const body = readFileSync(tmpAbs, "utf8");
    unlinkSync(tmpAbs);
    this.writeGeneratedCypherBundle(technology, database, body, finalAbs);

    console.log(`[generate:neo4j] wrote ${finalAbs} (database ${database}).`);
    if (result.message) console.log(result.message);
    return true;
  }

  runFromArgv(argv: string[]): void {
    if (!this.tryExportFromArgv(argv)) process.exit(1);
  }
}
//#endregion 🔖️generate-neo4j-gen

//#region 🔖️Policy
/**
 * ⚖️ Wave 4 app-plugin consistency policy — the machine-checkable subset of the Wave 4 V1 (duplication),
 * V2 (structure), V3 (coupling) audit findings under `.🦑️repo/🎫️tickets/26/07/18/WAVE-4-*-AUDIT`, wired via
 * `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️nx-plugin.mjs` into the synthetic `breach-script_ts` nx lint target (`bun ./📜️script.ts policy`).
 * Judgment-call findings (a real SDK/primitive gap, e.g. the terminology native/reuse Labels axis, or
 * puzzle's icon-based `tree_item_with_action`) are encoded as explicit low-priority allowlisted/tracked
 * breaches, never as a hard `policy` failure — see `POLICY_SDK_GAP_ALLOWLIST` below.
 */

//#region 🔧️PolicyFsScan
const POLICY_SKIP_DIRS = new Set(["node_modules", ".git", ".🦑️repo", "target", "dist", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);

/** 🧹️Drops every non-ASCII codepoint (emoji + variation selectors), e.g. `"📐️cad"` -> `"cad"`, `"🗣️dsl"` -> `"dsl"`. */
function policyStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

/** 🔀️Concept renames that must resolve to the SAME canonical component id on both sides of the taxonomy migration (see the master ticket's discovery contract: `📡️protocol` -> `📡️spr`). */
const POLICY_COMPONENT_ALIASES: Record<string, string> = { protocol: "spr" };

function policyCanonicalComponent(segment: string): string {
  const ascii = policyStripEmoji(segment);
  return POLICY_COMPONENT_ALIASES[ascii] ?? ascii;
}

/** 💾️Memoized `policyPluginOwnerDirs` result per repo root — one lint run asks for it once per crate. */
const policyPluginOwnerCache = new Map<string, readonly string[]>();

/**
 * 🗂️Owner dirs of every discovered `role = "plugin"` package, longest first — the discovery-driven
 * replacement for the old hardcoded `✏️s/🔌️plugins` prefix constant (mechanism step M4): "which plugin
 * owns this path" is answered by the shared package catalog (`🔣️taxonomy.json` + `🟦️discovery.ts`)
 * instead of a path literal this file has to keep in sync. Nested non-plugin owners (trinity's
 * `role = "tool"` jack shell/lsp packages) are deliberately excluded so they keep resolving to their
 * enclosing plugin's scope key, exactly as the prefix form did.
 */
function policyPluginOwnerDirs(repoRoot: string): readonly string[] {
  const cached = policyPluginOwnerCache.get(repoRoot);
  if (cached) return cached;
  const pluginOwners = discoverPackages(repoRoot).flatMap((pkg) => (pkg.role === "plugin" ? [pkg.ownerRel] : []));
  const owners = [...new Set(pluginOwners)].sort((a, b) => b.length - a.length);
  policyPluginOwnerCache.set(repoRoot, owners);
  return owners;
}

/**
 * 🔑Canonical `<pluginId>` scope key for a path inside a discovered plugin owner — the ASCII plugin slug,
 * stable across the legacy per-module-crate layout AND the Shape V2 one-package-per-plugin layout, so
 * allowlists keyed off it survive the crate move untouched (see the master ticket's discovery contract).
 * Returns `""` for a path outside every plugin owner (framework/hub/compose keep their own scope
 * identity — this function is deliberately plugin-scoped, matching every allowlist that uses it).
 */
function policyScopeKey(repoRoot: string, relPath: string): string {
  const norm = relPath.replaceAll("\\", "/");
  for (const owner of policyPluginOwnerDirs(repoRoot)) {
    if (norm === owner || norm.startsWith(`${owner}/`)) return policyStripEmoji(owner.split("/").pop() ?? "");
  }
  return "";
}

/** 🔑The `🎛️apps/<app>` id inside a crate path (ASCII, e.g. `"3d"`, `"5d"`), or `""` when the crate isn't nested under an app dir (plugin-root module/manifest crates). Used as the disambiguator for allowlist keys where the app id itself (not an enclosing `pub mod`, which no longer exists now each app is its own crate) is what distinguishes sibling crates of a multi-app plugin. */
function policyAppIdFromCrateDir(cratePath: string): string {
  const segments = cratePath.split("/");
  const appsIdx = segments.indexOf("🎛️apps");
  return appsIdx >= 0 && segments.length > appsIdx + 1 ? policyStripEmoji(segments[appsIdx + 1]!) : "";
}

/** 🧵Non-default file inside a crate/component dir (e.g. `benches/protocol.rs`) stays distinguishable from its crate's default entry file (`📦️glue.rs` package glue, `🦀️component.rs` domain leaf) instead of collapsing onto the same key — dropped extension, emoji-stripped, dash-joined. */
function policyFileSuffix(tailSegments: readonly string[], defaultFile: string): string {
  if (tailSegments.length === 1 && tailSegments[0] === defaultFile) return "";
  return `#${tailSegments.map((s) => policyStripEmoji(s.replace(/\.rs$/, ""))).join("-")}`;
}

/**
 * 🔑Canonical `<pluginId>/<component>` key for a *source file path* under `✏️s/🔌️plugins/` — collapses
 * both the legacy per-module-crate layout (`…/🔨️modules/<module>/⚡️implementations/🦀️rust/📦️lib.rs`,
 * `…/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs`) and the future taxonomy layout
 * (`…/🗿️artifacts/<artifact>/<module>/🦀️component.rs`) onto the same string, so a completeness
 * allowlist written today still matches the same logical component after a plugin migrates. Widens to
 * `<pluginId>/<appOrArtifact>/<component>` only when that disambiguator differs from the plugin id
 * itself (most plugins are single-app/single-artifact, where it doesn't — the 2-part key is the common
 * case, matching the discovery contract's `<pluginId>/<component>` shape). Falls back to the path
 * unchanged when no recognized owner/component shape is found (e.g. a path this wave's rename doesn't
 * touch, such as `compose/client/lib/rs/lib.rs`).
 */
function policyNormalizeRelPath(relPath: string): string {
  const norm = relPath.startsWith("./") ? relPath.slice(2) : relPath;
  const segments = norm.split("/");

  // Legacy shape: <owner...>/🔨️modules/<module>/⚡️implementations/<lang>/<file>, and the manifest/artifact
  // variant <owner...>/🛂️manifest/🗿️artifact/⚡️implementations/<lang>/<file> (🛂️manifest collapses the
  // same way 🔨️modules does — both are structural markers stripped from the owner chain).
  const implIdx = segments.indexOf("⚡️implementations");
  if (implIdx > 1) {
    const moduleSeg = segments[implIdx - 1]!;
    const suffix = policyFileSuffix(segments.slice(implIdx + 2), "📦️glue.rs");
    const ownerChain = segments.slice(0, implIdx - 1).filter((s) => s !== "🔨️modules" && s !== "🛂️manifest");
    const pluginsIdx = ownerChain.indexOf("🔌️plugins");
    if (pluginsIdx >= 0 && ownerChain.length > pluginsIdx + 1) {
      const pluginId = policyStripEmoji(ownerChain[pluginsIdx + 1]!);
      const appsIdx = ownerChain.indexOf("🎛️apps");
      const appId = appsIdx >= 0 && ownerChain.length > appsIdx + 1 ? policyStripEmoji(ownerChain[appsIdx + 1]!) : undefined;
      const component = policyCanonicalComponent(moduleSeg);
      return (appId && appId !== pluginId ? `${pluginId}/${appId}/${component}` : `${pluginId}/${component}`) + suffix;
    }
    // Non-plugin owner (framework/compose/...): best-effort <ownerId>/<component>, not this rename's
    // primary target this wave, but kept collision-free via the same file suffix.
    const ownerId = policyStripEmoji(ownerChain[ownerChain.length - 1] ?? "");
    if (ownerId) return `${ownerId}/${policyCanonicalComponent(moduleSeg)}${suffix}`;
  }

  // Future taxonomy shape: <owner>/🗿️artifacts/<artifact>/<module>/🦀️component.rs (+ sibling 🦀️<topic>.rs).
  const artifactsIdx = segments.indexOf("🗿️artifacts");
  if (artifactsIdx > 0 && segments.length > artifactsIdx + 2) {
    const pluginId = policyStripEmoji(segments[artifactsIdx - 1] ?? "");
    const artifactId = policyStripEmoji(segments[artifactsIdx + 1] ?? "");
    const component = policyCanonicalComponent(segments[artifactsIdx + 2]!);
    const suffix = policyFileSuffix(segments.slice(artifactsIdx + 3), "🦀️component.rs");
    return (artifactId && artifactId !== pluginId ? `${pluginId}/${artifactId}/${component}` : `${pluginId}/${component}`) + suffix;
  }

  return norm;
}

/**
 * 🏗️One discovered rust crate. `dir` is its `Cargo.toml`-containing directory (repo-relative),
 * `libRelPath` its crate-root source file as the manifest itself declares it (`[lib]`/`[[bin]]`
 * `path`) — never assumed, because the two shapes disagree: Shape V2 keeps the entry beside the
 * manifest (`📦️packages/🦀️rust/📦️glue.rs`) while the Shape V1 leftovers still point two levels up
 * (`path = "../../📦️glue.rs"`). `role`/`ownerRel` come straight from the shared discovery contract;
 * `pluginId` is `""` for every crate outside a plugin owner (framework/hub/s-modules).
 */
type PolicyCrateRef = {
  dir: string;
  libRelPath: string;
  shape: "legacy" | "taxonomy";
  role: PackageRole | "";
  ownerRel: string;
  pluginId: string;
};

/** 🚦️Priority floor for a rule firing on newly-visible surface: framework/hub/s-module crates entered these rules' field of view only with mechanism step M4, so their findings land as tracked (`"low"`) until a dedicated sweep triages them — plugin findings keep the priority the rule already used. */
function policyNewSurfacePriority(crate: PolicyCrateRef, pluginPriority: BreachRecord["priority"]): BreachRecord["priority"] {
  return crate.pluginId ? pluginPriority : "low";
}

/** 🚪️Crate-root source file a `Cargo.toml` declares, resolved repo-relative: reads the `[lib]`/`[[bin]]` `path` key, falling back to the vocabulary's rust entry filenames beside the manifest and then at the owner root. */
function policyCrateEntryPath(repoRoot: string, manifestDirRel: string, ownerRel: string): string {
  const manifestAbs = join(repoRoot, manifestDirRel, "Cargo.toml");
  if (existsSync(manifestAbs)) {
    const lines = readFileSync(manifestAbs, "utf8").split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      if (!/^\s*\[\[?(?:lib|bin)\]?\]\s*$/.test(lines[i]!)) continue;
      for (let j = i + 1; j < lines.length && !/^\s*\[/.test(lines[j]!); j++) {
        const declared = lines[j]!.match(/^\s*path\s*=\s*"([^"]+)"\s*$/)?.[1];
        if (declared) return relative(repoRoot, resolve(join(repoRoot, manifestDirRel), declared)).replaceAll("\\", "/");
      }
    }
  }
  for (const entry of loadTaxonomy().ecosystems["🦀️rust"]?.entryFilenames ?? []) {
    if (existsSync(join(repoRoot, manifestDirRel, entry))) return `${manifestDirRel}/${entry}`;
    if (existsSync(join(repoRoot, ownerRel, entry))) return `${ownerRel}/${entry}`;
  }
  return `${manifestDirRel}/📦️glue.rs`;
}

/**
 * 🔎️Discovers every rust crate these rules can see, driven by the shared package catalog
 * (`discoverPackages`/`discoverOwners` over `🔣️taxonomy.json`) rather than by a hardcoded area path.
 * PRIMARY: every `📦️packages/🦀️rust[/🎯️targets/<target>]/Cargo.toml` carrying a semio role marker —
 * plugins AND, for the first time (mechanism step M4), framework/hub/s-module/tool packages, which the
 * previous Shape-V1-only matcher could never see. SECONDARY (burn-down, deleted at the finalization
 * flip): residual `⚡️implementations/🦀️rust` sandwiches *inside* an already-discovered owner, so a
 * half-migrated owner keeps being checked; an owner that has not adopted `📦️packages` at all is
 * pre-contract and stays out of scope until it does.
 */
function policyDiscoverCrateDirs(repoRoot: string): PolicyCrateRef[] {
  const taxonomy = loadTaxonomy();
  const forbiddenSegments = new Set<string>(taxonomy.forbiddenPathSegments);
  const legacyEntryFilename = taxonomy.entryFilenames["🦀️rust"] ?? "📦️glue.rs";
  const found = new Map<string, PolicyCrateRef>();

  for (const pkg of discoverPackages(repoRoot, taxonomy)) {
    if (pkg.lang !== "🦀️rust") continue;
    found.set(pkg.packageRel, {
      dir: pkg.packageRel,
      libRelPath: policyCrateEntryPath(repoRoot, pkg.packageRel, pkg.ownerRel),
      shape: "taxonomy",
      role: pkg.role,
      ownerRel: pkg.ownerRel,
      pluginId: policyScopeKey(repoRoot, pkg.ownerRel),
    });
  }

  for (const owner of discoverOwners(repoRoot, taxonomy)) {
    const pluginId = policyScopeKey(repoRoot, owner.ownerRel);
    const walkLegacy = (relDir: string): void => {
      let entries: ReturnType<typeof readdirSync>;
      try {
        entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
      } catch {
        return;
      }
      for (const ent of entries) {
        if (!ent.isDirectory() || POLICY_SKIP_DIRS.has(ent.name) || ent.name === taxonomy.packagesDirName) continue;
        const childRel = `${relDir}/${ent.name}`;
        if (ent.name === "🦀️rust" && forbiddenSegments.has(relDir.split("/").pop() ?? "")) {
          if (!found.has(childRel) && existsSync(join(repoRoot, childRel, legacyEntryFilename)) && existsSync(join(repoRoot, childRel, "Cargo.toml"))) {
            found.set(childRel, {
              dir: childRel,
              libRelPath: `${childRel}/${legacyEntryFilename}`,
              shape: "legacy",
              role: pluginId ? "plugin" : (owner.roles[0] ?? ""),
              ownerRel: owner.ownerRel,
              pluginId,
            });
          }
          continue;
        }
        walkLegacy(childRel);
      }
    };
    walkLegacy(owner.ownerRel);
  }

  return [...found.values()].sort((a, b) => a.dir.localeCompare(b.dir));
}
//#endregion 🔧️PolicyFsScan

//#region 🔧️PolicyRegionParsing
type PolicyRegionEvent = { kind: "open" | "close"; line: number; spaceAfterSlashes: boolean; label: string };
type PolicyRegionSpan = { label: string; closeLabel: string; startLine: number; endLine: number };
type PolicyModSpan = { name: string; startLine: number; endLine: number };

const POLICY_REGION_OPEN_RE = /^(\s*)\/\/(\s*)#region(?:\s+(.*))?\s*$/;
const POLICY_REGION_CLOSE_RE = /^(\s*)\/\/(\s*)#endregion(?:\s+(.*))?\s*$/;

function policyParseRegionEvents(lines: readonly string[]): PolicyRegionEvent[] {
  const events: PolicyRegionEvent[] = [];
  lines.forEach((line, i) => {
    const om = line.match(POLICY_REGION_OPEN_RE);
    if (om) {
      events.push({ kind: "open", line: i + 1, spaceAfterSlashes: (om[2] ?? "").length > 0, label: (om[3] ?? "").trim() });
      return;
    }
    const cm = line.match(POLICY_REGION_CLOSE_RE);
    if (cm) events.push({ kind: "close", line: i + 1, spaceAfterSlashes: (cm[2] ?? "").length > 0, label: (cm[3] ?? "").trim() });
  });
  return events;
}

function policyPairRegionSpans(events: readonly PolicyRegionEvent[]): PolicyRegionSpan[] {
  const stack: PolicyRegionEvent[] = [];
  const spans: PolicyRegionSpan[] = [];
  for (const ev of events) {
    if (ev.kind === "open") {
      stack.push(ev);
      continue;
    }
    const open = stack.pop();
    if (!open) continue;
    spans.push({ label: open.label, closeLabel: ev.label, startLine: open.line, endLine: ev.line });
  }
  return spans;
}

/**
 * 🧹️Masks `"..."` string-literal contents and `'x'` char-literal contents (same length, so indices stay
 * aligned) — line-bounded (`\n` excluded from both classes) and the char-literal form requires exactly one
 * char/escape, so a Rust lifetime apostrophe (`&'static`, `'a`) never greedily pairs with an unrelated
 * quote elsewhere in the file (which would otherwise corrupt brace-counting across huge, unrelated spans).
 */
function policyMaskLiterals(line: string): string {
  return line.replace(/"(?:[^"\\\n]|\\.)*"/g, (m) => `"${" ".repeat(Math.max(0, m.length - 2))}"`).replace(/'(?:\\.|[^'\\\n])'/g, (m) => `'${" ".repeat(Math.max(0, m.length - 2))}'`);
}

const POLICY_MOD_OPEN_RE = /^\s*pub mod (\w+)\b.*\{\s*$/;

function policyParseModSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { name: string; startLine: number; depth: number }[] = [];
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const modMatch = raw.match(POLICY_MOD_OPEN_RE);
    if (modMatch) stack.push({ name: modMatch[1]!, startLine: i + 1, depth });
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      spans.push({ name: top.name, startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

function policyModAtLine(modSpans: readonly PolicyModSpan[], lineNo: number): string {
  const containing = modSpans.filter((s) => s.startLine <= lineNo && lineNo <= s.endLine);
  return containing.sort((a, b) => a.endLine - a.startLine - (b.endLine - b.startLine))[0]?.name ?? "";
}

const POLICY_MOD_ANY_OPEN_RE = /^\s*(?:pub\s+)?mod\s+(\w+)\b.*\{\s*$/;

/** 🧪️Brace-spans of `#[cfg(test)] mod … { … }` blocks — synthetic test fixtures (e.g. `App::builder` in a unit test) aren't real app registrations. */
function policyTestModSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { name: string; startLine: number; depth: number; isTest: boolean }[] = [];
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const modMatch = raw.match(POLICY_MOD_ANY_OPEN_RE);
    if (modMatch) {
      const isTest = lines.slice(Math.max(0, i - 2), i).some((l) => /#\[cfg\(test\)\]/.test(l)) || modMatch[1] === "tests";
      stack.push({ name: modMatch[1]!, startLine: i + 1, depth, isTest });
    }
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      if (top.isTest) spans.push({ name: top.name, startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

/** 🏷️Strips a leading non-letter (emoji/sigil) prefix off a region label, e.g. "🔖️Tests" -> "Tests". */
function policyLabelName(label: string): string {
  return label.replace(/^[^\p{L}]+/u, "").trim();
}
//#endregion 🔧️PolicyRegionParsing

//#region 🔧️PolicyFnParsing
/** 🧱️Extracts a `{ … }` function body starting the brace-scan at/after `fromIdx`, string-literal-safe. */
function policyExtractFnBody(content: string, fromIdx: number): string {
  const masked = policyMaskLiterals(content);
  const openIdx = masked.indexOf("{", fromIdx);
  if (openIdx === -1) return "";
  let depth = 0;
  for (let i = openIdx; i < masked.length; i++) {
    if (masked[i] === "{") depth++;
    else if (masked[i] === "}") {
      depth--;
      if (depth === 0) return content.slice(openIdx, i + 1);
    }
  }
  return content.slice(openIdx);
}

function policyLineOfIndex(content: string, idx: number): number {
  return content.slice(0, idx).split("\n").length;
}

/** 🐫️PascalCase(app id) + "App", e.g. "gis2d-play" -> "Gis2dPlayApp". */
function policyPascalAppStructName(id: string): string {
  const parts = id.split(/[-_]+/).filter(Boolean);
  return `${parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("")}App`;
}
//#endregion 🔧️PolicyFnParsing

//#region 🔧️PolicyAllowlists
/**
 * 🎫️ Wave 4 V1 duplication audit (`.🦑️repo/🎫️tickets/26/07/18/WAVE-4-V1-DUPLICATION-HUNTER-AUDIT`): both crates
 * resolve a second "terminology" axis (native/reuse) the SDK's locale-only `app_labels!`/`LocaleLabels`
 * primitive can't express. Flagged for a Wave-4 decision (extend the primitive to two axes, or formally
 * accept the gap) — tracked here as a low-priority, non-failing breach until that decision lands.
 * Keyed `<pluginId>#<StructName>` via `policyScopeKey` — the struct name alone already disambiguates
 * puzzle's three apps (Puzzle2dLabels/Puzzle3dLabels/Puzzle5dLabels), so no app segment is needed.
 */
const POLICY_LABELS_TWO_AXIS_ALLOWLIST = new Set<string>(["cad#CadLabels", "puzzle#Puzzle2dLabels", "puzzle#Puzzle3dLabels", "puzzle#Puzzle5dLabels"]);

/**
 * 🎫️ Wave 4 V1 duplication audit: puzzle's 3d/5d `tree_item_with_action` redefinitions add an `icon_id`
 * param the SDK's description-based primitive can't express (icon rendering) — documented real gap,
 * tracked here as a low-priority, non-failing breach rather than a should-fix duplicate. Keyed
 * `<pluginId>#<appId>` via `policyScopeKey` + `policyAppIdFromCrateDir` — the original `puzzle/plugin/rs#d3`
 * form assumed one shared crate with `pub mod d3 { … }`/`pub mod d5 { … }` wrappers; today (and in the
 * future taxonomy) each app is its own crate, so the app id in the crate's own path is the disambiguator,
 * not an enclosing `pub mod`.
 */
const POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST = new Set<string>(["puzzle#3d", "puzzle#5d"]);

/**
 * 🎫️ Wave 4 V3 coupling audit (`.🦑️repo/🎫️tickets/26/07/18/WAVE-4-APP-TO-APP-COUPLING-AND-FRAMEWORK-IDENTITY-LEAK-AUDIT`):
 * these crates are neutral shared domain/library crates that also happen to ship their own minimal
 * playground app (documented via each crate's `AGENTS.md`) — depending on them is not app-to-app coupling.
 */
const POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST = new Set<string>(["flow", "trinity_jack", "trinity_ram", "mathematical_graph_drawing", "mathematical_geometry", "infinite_board_port_directed", "infinite_board_port_directed_dag"]);

/**
 * 🎫️ dsl/ derive-engine migration lock step: technologies whose example/*.json fixture has not yet
 * been converted to its own DSL-text extension (e.g. `.puzzle2d`, `.flow`). Tracked here as a documented,
 * still-open follow-up rather than a silent exception — remove an entry once that technology's fixture
 * is migrated (see an already-migrated sibling crate's `*_fixture` round-trip test for the pattern).
 */
const POLICY_JSON_FIXTURE_ALLOWLIST = new Set<string>([]);

/** 🛡️Path prefixes exempt from the no-JSON-fixture rule structurally — not a technology document fixture (e.g. `coda/`'s `example/` holds a whole simulated ACC project's run/iteration artifacts, unrelated to the dsl/ migration). */
const POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST = ["coda/"];

/**
 * 🎫️ pack/ binary-document-layer rollout lock step (`.🦑️repo/🎫️tickets/26/07/27/
 * PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS`): every `*.rs` file below that already calls
 * `assert_dsl_round_trip(`/`assert_document_text_round_trip(` but does not yet ALSO call
 * `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(` on the same fixtures — seeded at
 * wave 1 with every file that fails the check today (wave 0 only built the `pack` crate family itself;
 * wave 1 only wired `vcs`/`dsl_derive`/`framework`, proving the mechanism on `vcs/rs/lib.rs`'s own
 * `VcsSnapshot` fixture, which is why `vcs/rs/lib.rs` is NOT in this list). Wave 2's per-app-family
 * agents add the pack-equivalence assertions beside each technology's existing DSL round-trip tests and
 * remove that file from this list; wave 3 verifies it has shrunk to empty. Still empty today, but keyed
 * by `policyNormalizeRelPath` (canonical `<pluginId>/<component>`) like its siblings below, so the first
 * entry ever added here is already taxonomy-move-proof.
 */
const POLICY_PACK_COMPLETENESS_ALLOWLIST = new Set<string>([]);

/**
 * 🎫️ CW7 command-envelope law lock step (`.🦑️repo/🎫️tickets/26/07/27/
 * INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`): every `*.rs` file that already calls
 * `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(` but does not yet ALSO call
 * `vcs::test_support::assert_command_envelope_round_trip` (added in CW7) on the same fixtures — seeded
 * at CW8 with every file that fails the check today, exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST`
 * was seeded for the dsl/pack lock step (`vcs/rs/lib.rs` itself proves the mechanism on its own
 * `VcsSnapshot`/`LossyMutation` fixtures, which is why it is NOT in this list). Remove an entry
 * once that file adds the command-envelope-round-trip call. Keyed by `policyNormalizeRelPath` (canonical
 * `<pluginId>/<component>`, not the raw repo-relative path) so a plugin's move to the taxonomy layout
 * never requires touching this list — matched against every discovered file via `policyNormalizeRelPath`
 * at lookup time, so both the legacy and future path shape resolve to the same entry.
 */
const POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST = new Set<string>([
  "architect/spine",
  "compose/client/lib/rs/lib.rs",
  "os/dsl",
  "products/os",
  "lowpoly/spr",
  "os/workflow",
  "animate/present/spr",
  "gis/artifact",
  "mathematical/artifact",
  "note/artifact",
  "raster/artifact",
  "reasoning/artifact",
  "space/artifact",
  "vcs/artifact",
]);

/**
 * 🎫️ dsl/ derive-engine migration lock step: known generic bridges whose `DocumentDsl`/`OpText` coverage
 * cannot be seen as a `#[derive(dsl::Dsl...)]` attribute directly on the type (a blanket/generic impl
 * elsewhere covers them) — accepted as DSL-complete by `policyDslCompletenessBreaches` without a
 * hand-rolled-impl grep hit under that exact type name.
 * - `Value` (`serde_json::Value`): blanket `impl DocumentDsl for serde_json::Value` in `vcs/rs/lib.rs`
 *   (the schema-less escape hatch for a technology whose `DocumentApp::Snapshot` predates its own
 *   typed DSL derive — see `puzzle_2d`'s `🔖️ValueBridge` region).
 * - `SetDocumentMutation` (`norm_core::SetDocumentMutation<D>`): one hand-rolled generic
 *   `impl<D: DocumentDsl> OpText for SetDocumentMutation<D>` in `norm/core/rs/lib.rs`, shared by every
 *   norm family's `Mutation` type instead of a per-family derive.
 */
const POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST = new Set<string>(["Value", "SetDocumentMutation"]);

/**
 * 🎫️ W1 grammar-engine wave (`.claude/plans/the-final-goal-for-jolly-spindle.md` `## Master wave
 * plan` `W1 — Grammar engine`, design ruling B-R4): every `*.rs` file that defines a real
 * `impl protocol::MutationDiff<...>` diff type but does not yet ALSO give that type a
 * `protocol::DiffCodec` impl (via `#[derive(dsl::DslDiff)]` or a hand-rolled impl) — seeded at W1
 * with every file that fails the check today. `DiffCodec` is deliberately NOT (yet) a hard
 * supertrait bound of `MutationDiff` (see that trait's doc comment in `protocol_command/rs/lib.rs`)
 * — this allowlist is the shrinking-list enforcement mechanism instead, exactly like
 * `POLICY_PACK_COMPLETENESS_ALLOWLIST`/`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST` before it.
 * `writer_op/rs/lib.rs` (`WriterDiff`) and `note_op/rs/lib.rs` (`NoteDiff`) are the two W1
 * proof-of-mechanism types and are deliberately NOT in this list. A handful of entries are
 * permanently-test-fixture files (`protocol_command`/`protocol_causal`/`protocol_crdt`/
 * `protocol_testkit`/`plugin`/`db_document`/`db_engine`'s own `AddDiff`/`DummyDiff`/`TestDiff`/
 * `RegisterDiff`/`GraphDiff`/`CausalAddDiff`/`HashDiff`/`BenchDiff` law/bench fixtures, used to test
 * the trait machinery itself, never a real document type) — those are expected to stay allowlisted forever,
 * mirrored by `POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`'s precedent for permanent entries.
 * Full coverage (all real diff types deriving/implementing `DiffCodec`, then `DiffCodec` promoted to
 * a hard supertrait bound) is wave 6's "Lane C (B5)" item. Remove an entry once that file's diff
 * type gets a `DiffCodec` impl.
 */
const POLICY_DIFF_COMPLETENESS_ALLOWLIST = new Set<string>([
  // permanent: trait-machinery test fixtures, never real document types
  "os/plugin",
  "db/document",
  "db/engine",
  "protocol/testkit",
  "protocol/testkit#benches-protocol",
  "protocol/crdt",
  "protocol/command",
  "protocol/causal",
  "store/sync",
  // deferred to W6: real diff types not yet covered by #[derive(dsl::DslDiff)]
  "flow/core",
  "directed/dag",
  "os/playbook",
  "products/os",
  "os/store",
  "compose/client/lib/rs/lib.rs",
  "trinity/rewrite/op",
  "trinity/ram",
  "remodel/op",
  "raster/op",
  "process/3d/op",
  "norm/core",
  "norm/artifact",
  "cad/op",
  "block/3d/op",
  "block/5d/op",
  "block/2d/op",
  "reasoning/wires/op",
  "sequence/op",
  "animate/present/op",
  "space/home/op",
  "procedural/3d/op",
  "procedural/2d/op",
  "vcs/op",
  "vcs/engine",
  "gis/3d/op",
  "gis/2d/op",
  "imperative/op",
  "sourcing/curate/op",
  "architect/artifact",
  "architect/spine",
  "shooting/op",
  "mathematical/op",
  "layout/op",
  "puzzle/3d/op",
  "puzzle/5d/op",
  "puzzle/2d/op",
  "fem/3d/op",
  "fem/2d/op",
  "draw/op",
  "playbook/procedural",
  "lowpoly/op",
]);

/**
 * 🎫️ Handcrafted `.grammar.semio` / `.protocol.semio` specs per artifact facet (see ticket
 * `26/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`). Paths listed here are still on generic/stub
 * specs; remove each entry once the facet's normative spec + recognizer sweep is committed.
 */
const POLICY_GRAMMAR_FILE_ALLOWLIST = new Set<string>([]);

/** @emoji 📡️ Stub `.protocol.semio` files not yet backed by a byte-level recognizer proof. */
const POLICY_PROTOCOL_FILE_ALLOWLIST = new Set<string>([]);

/**
 * ⚖️ Constitutional artifact facets whose `🟦️component.ts` may remain a WASM scaffold stub.
 * Stubs under these facets (and under `🧬️mutations/<mut>/{🦠️mutation,🔺️diff,↩️inverse}`) are accepted
 * structurally — never tracked in a per-file allowlist (Wave 2b / OPERATIONS-TO-MUTATIONS).
 */
const POLICY_TS_FACADE_CONSTITUTIONAL_FACETS = new Set<string>([
  "🗣️dsl",
  "🔧️op",
  "🔺️diff",
  "🎒️pack",
  "📡️spr",
  "🧬️mutations",
  "⚙️engine",
]);
const POLICY_MUTATION_TRIAD_DIRS = ["🦠️mutation", "🔺️diff", "↩️inverse"] as const;
const POLICY_MUTATIONS_FACET = "🧬️mutations";
const POLICY_ENGINE_FACET = "⚙️engine";
const POLICY_OP_FACET = "🔧️op";
const POLICY_TS_COMPONENT_LEAF = "🟦️component.ts";
const POLICY_RS_COMPONENT_LEAF_NAME = "🦀️component.rs";
/**
 * ⚖️P3/M4: colliding .grammar.semio/.protocol.semio after name/start normalization. Remove a path once its normalized hash is unique.
 * Seeded 0 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_SPEC_DISTINCTNESS_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: generic catch-all grammar specs under ✏️s/. Remove once handcrafted.
 * Seeded 0 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_GENERIC_SPEC_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: grammars with use family-X that never reference a family production. Remove once wired.
 * Seeded 0 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_DECLARED_USE_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: facet 🦀️component.rs missing include_str! of sibling spec. Remove once wired.
 * Seeded 220 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: artifacts with facet specs but no register_language under the artifact. Remove once engines register.
 * Seeded 44 paths at P3 — must shrink to empty by P6 (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS = new Set<string>([]);

/**
 * ⚖️P3/M4: empty/stub `.semio` under `📚️examples/** /🖼️assets/` (size≤64). Kept empty — stubs are breaches.
 */
const POLICY_EMPTY_EXAMPLE_EXEMPTIONS = new Set<string>([]);
/**
 * ⚖️P6: empty — DocumentDsl/OpText/DocumentPack/OpBinary emission deleted from DslDocument/DslOps.
 * Scanner now flags residual __rt/op_rt codec calls (ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT).
 */
const POLICY_GENERIC_CODEC_DERIVE_EXEMPTIONS = new Set<string>([]);

//#endregion 🔧️PolicyAllowlists

//#region 🔧️PolicyRuleRegionFormat
/** 📏️V2 rule: `//#region 🔖️Name` / `//#endregion 🔖️Name` (no space after `//`), tests region must be `🧪️Tests`. */
function policyRegionFormatBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const events = policyParseRegionEvents(lines);
  for (const ev of events) {
    const marker = ev.kind === "open" ? "region" : "endregion";
    if (ev.spaceAfterSlashes) {
      breaches.push({
        id: `region-format-space-${scope}-${ev.line}`,
        summary: `Region marker has a space after "//" (must be "//#${marker}", not "// #${marker}")`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: ev.line,
        // 🎫️ W0 (Crate Consolidation & Plugin Taxonomy Restructure, root-script-policy-revival ticket):
        // downgraded from "high" the moment the discoverer revival first ran this rule against real
        // crates — ~1850 pre-existing "// #region" (space) markers across the plugin tree had never
        // been checked before (the old discoverer matched nothing), so this was never a hard gate in
        // practice. Fixing that many markers spans hundreds of files outside this ticket's script.ts-only
        // ownership; WARN-only until a dedicated formatting sweep ticket cleans it up, then flip back to "high".
        priority: "medium",
        reason: "Wave 4 V2 structure audit: region markers must be //#region 🔖️Name / //#endregion 🔖️Name, no space after //.",
        solution: `Remove the space between "//" and "#${marker}" on line ${ev.line}.`,
      });
    }
    if (!ev.label) {
      breaches.push({
        id: `region-format-missing-label-${scope}-${ev.line}`,
        summary: `//#${marker} marker is missing its label`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: ev.line,
        priority: "medium",
        reason: "Every region open/close marker must carry its emoji+name label so pairs stay self-documenting.",
        solution: `Add the matching label after //#${marker} on line ${ev.line}.`,
      });
    }
  }
  for (const span of policyPairRegionSpans(events)) {
    const bare = policyLabelName(span.label);
    if (/^tests?$/i.test(bare) && span.label !== "🧪️Tests") {
      breaches.push({
        id: `region-tests-label-${scope}-${span.startLine}`,
        summary: `Tests region labeled "${span.label}" must be exactly "🧪️Tests"`,
        kind: "app-plugin/region-tests-label",
        scope,
        line: span.startLine,
        priority: "medium",
        reason: "Wave 4 V2 structure audit: the tests region sigil is reserved as 🧪️Tests.",
        solution: `Rename the region label at line ${span.startLine} (and its matching //#endregion) to "🧪️Tests".`,
      });
    }
    if (span.closeLabel && span.closeLabel !== span.label) {
      breaches.push({
        id: `region-label-mismatch-${scope}-${span.startLine}`,
        summary: `//#endregion label "${span.closeLabel}" does not match its //#region label "${span.label}"`,
        kind: "app-plugin/region-marker-format",
        scope,
        line: span.endLine,
        priority: "medium",
        reason: "Region open/close labels must match so the pairing stays unambiguous.",
        solution: `Fix the //#endregion label at line ${span.endLine} to read "${span.label}".`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleRegionFormat

//#region 🔧️PolicyRuleManifestRegion
/** 📏️V2 rule: every `App::builder(...)` call must be enclosed in a region labeled exactly `🔖️Manifest`. */
function policyManifestRegionBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const spans = policyPairRegionSpans(policyParseRegionEvents(lines));
  const testSpans = policyTestModSpans(lines);
  const breaches: BreachRecord[] = [];
  lines.forEach((line, i) => {
    if (!/App::builder\(/.test(line)) return;
    const lineNo = i + 1;
    if (testSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine)) return; // synthetic test fixture, not a real app registration
    const enclosed = spans.some((s) => s.label === "🔖️Manifest" && s.startLine <= lineNo && lineNo <= s.endLine);
    if (!enclosed) {
      breaches.push({
        id: `manifest-region-${scope}-${lineNo}`,
        summary: `App::builder(...) call is not enclosed in a "🔖️Manifest" region`,
        kind: "app-plugin/manifest-region",
        scope,
        line: lineNo,
        priority: "medium",
        reason: "Wave 4 V2 structure audit: each app's App::builder(...) registration must live inside its own //#region 🔖️Manifest.",
        solution: `Wrap the App::builder(...) call at line ${lineNo} in a dedicated //#region 🔖️Manifest / //#endregion 🔖️Manifest.`,
      });
    }
  });
  return breaches;
}
//#endregion 🔧️PolicyRuleManifestRegion

//#region 🔧️PolicyRuleStructNaming
/** 📏️V2 rule: the `DocumentApp` struct backing `<PREFIX>_APP_ID` must be named `PascalCase(id) + "App"`. */
function policyStructNamingBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const idsByConst = new Map<string, string>();
  const constRe = /const\s+([A-Z][A-Z0-9_]*_APP_ID)\s*:\s*&str\s*=\s*"([^"]+)"/g;
  let cm: RegExpExecArray | null;
  while ((cm = constRe.exec(content))) idsByConst.set(cm[1]!, cm[2]!);

  const implRe = /impl\s+DocumentApp\s+for\s+(\w+)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = implRe.exec(content))) {
    const structName = m[1]!;
    const window = content.slice(m.index + m[0].length, m.index + m[0].length + 2000);
    const constMatch = window.match(/\b([A-Z][A-Z0-9_]*_APP_ID)\b/);
    const id = constMatch ? idsByConst.get(constMatch[1]!) : undefined;
    if (!id) continue;
    const expected = policyPascalAppStructName(id);
    if (structName !== expected) {
      breaches.push({
        id: `struct-naming-${scope}-${structName}`,
        summary: `App struct "${structName}" for id "${id}" should be named "${expected}"`,
        kind: "app-plugin/struct-naming",
        scope,
        line: policyLineOfIndex(content, m.index),
        priority: "medium",
        reason: 'Wave 4 V2 structure audit: app struct name must be PascalCase(app id) + "App".',
        solution: `Rename struct ${structName} to ${expected} (and all its references) in ${scope}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleStructNaming

//#region 🔧️PolicyRuleModLayout
/** 📏️V2 rule: crates declaring 2+ apps must isolate each `impl DocumentApp for` inside its own `pub mod`. */
function policyModLayoutBreaches(scope: string, lines: readonly string[]): BreachRecord[] {
  const implLines: number[] = [];
  const implRe = /impl\s+DocumentApp\s+for\s+\w+\s*\{/;
  lines.forEach((line, i) => {
    if (implRe.test(line)) implLines.push(i + 1);
  });
  if (implLines.length < 2) return [];
  const modSpans = policyParseModSpans(lines);
  const breaches: BreachRecord[] = [];
  for (const lineNo of implLines) {
    if (!modSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine)) {
      breaches.push({
        id: `mod-layout-${scope}-${lineNo}`,
        summary: `Multi-app crate declares an app (DocumentApp impl at line ${lineNo}) outside any "pub mod app_<name>" wrapper`,
        kind: "app-plugin/mod-layout",
        scope,
        line: lineNo,
        priority: "high",
        reason: "Wave 4 V2 structure audit: crates with 2+ apps must isolate each app inside its own pub mod (app_2d/app_3d, d2/d3/d5, ...).",
        solution: `Wrap the app declared at line ${lineNo} in its own pub mod block, matching the pattern used by gis/procedural/trinity/puzzle/s.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleModLayout

//#region 🔧️PolicyRuleSdkMechanisms
/** 🔎️Resolves a `use ... importedName as alias` rename so delegation checks accept the aliased call form too. */
function policyResolveImportAlias(content: string, importedName: string): string | undefined {
  return content.match(new RegExp(`\\b${importedName}\\s+as\\s+(\\w+)\\b`))?.[1];
}

/** 📏️V1 rule: local `fn selection_ids` must delegate to `semio_framework_plugin::selection_ids` before adding a fallback key. */
function policySelectionIdsBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const alias = policyResolveImportAlias(content, "selection_ids");
  const re = /fn\s+selection_ids\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) {
    const body = policyExtractFnBody(content, m.index);
    if (body.includes("semio_framework_plugin::selection_ids(") || (alias && new RegExp(`\\b${alias}\\(`).test(body))) continue;
    const lineNo = policyLineOfIndex(content, m.index);
    breaches.push({
      id: `sdk-selection-ids-${scope}-${lineNo}`,
      summary: `Local "fn selection_ids" does not delegate to semio_framework_plugin::selection_ids`,
      kind: "app-plugin/sdk-selection-ids",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: apps needing an extra fallback key must still call the SDK's selection_ids core first (see procedural::app_3d, reasoning/mindmap for the reference shape).",
      solution: `Rewrite selection_ids at line ${lineNo} to call semio_framework_plugin::selection_ids(args) first and only add the extra fallback key on top.`,
    });
  }
  return breaches;
}

/** 📏️V1 rule: local `fn new_app`/`fn new_app_with_registry`/`fn meta` must stay thin typed delegates to the SDK testkit (allowing a `::<Turbofish>` generic before the call's `(`). */
function policyTestkitDelegateBreaches(scope: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const specs: readonly { re: RegExp; mustRe: RegExp; label: string }[] = [
    { re: /fn\s+new_app_with_registry\s*[<(]/g, mustRe: /testkit::new_app_with_registry(?:::<[^>]*>)?\s*\(/, label: "testkit::new_app_with_registry" },
    { re: /fn\s+new_app\s*[<(]/g, mustRe: /testkit::new_app(?:::<[^>]*>)?\s*\(/, label: "testkit::new_app" },
    { re: /fn\s+meta\s*\(/g, mustRe: /testkit::meta\s*\(/, label: "testkit::meta" },
  ];
  for (const { re, mustRe, label } of specs) {
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) {
      const body = policyExtractFnBody(content, m.index);
      if (mustRe.test(body)) continue;
      const lineNo = policyLineOfIndex(content, m.index);
      breaches.push({
        id: `sdk-testkit-delegate-${scope}-${lineNo}`,
        summary: `Local "${m[0]!.trim()}" does not delegate to semio_framework_plugin::${label}`,
        kind: "app-plugin/sdk-testkit-delegate",
        scope,
        line: lineNo,
        priority: "medium",
        reason: "Wave 4 V1 duplication audit: per-type new_app/new_app_with_registry/meta helpers must stay thin typed delegates to the SDK testkit.",
        solution: `Make the helper at line ${lineNo} call ${label}(...) instead of reimplementing it.`,
      });
    }
  }
  return breaches;
}

/** 📏️V1 rule: local `tree_item_with_action` redefinitions need an allowlisted SDK gap; other `tree_item_*` wrappers must delegate to it. */
function policyTreeItemBreaches(crate: PolicyCrateRef, content: string, lines: readonly string[]): BreachRecord[] {
  const scope = crate.dir;
  const breaches: BreachRecord[] = [];
  const modSpans = policyParseModSpans(lines);

  const redefRe = /fn\s+tree_item_with_action\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = redefRe.exec(content))) {
    const lineNo = policyLineOfIndex(content, m.index);
    // 🌱️ The original allowlist keyed off an enclosing `pub mod d3 { … }` wrapper (one shared crate,
    // multiple apps namespaced by mod); today (and in the future taxonomy) each app is its own crate,
    // so fall back to the app id from the crate's own path when there's no such wrapper.
    const mod = policyModAtLine(modSpans, lineNo) || policyAppIdFromCrateDir(scope);
    if (POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST.has(`${crate.pluginId}#${mod}`)) continue;
    breaches.push({
      id: `sdk-tree-item-redefinition-${scope}-${lineNo}`,
      summary: `Local "fn tree_item_with_action" shadows the SDK primitive of the same name`,
      kind: "app-plugin/sdk-tree-item",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: redefining tree_item_with_action locally is only accepted for a documented SDK gap (e.g. icon rendering).",
      solution: `Delete this and call semio_framework_plugin::tree_item_with_action directly, or if it exists for a genuine SDK gap, add it to POLICY_TREE_ITEM_REDEFINITION_ALLOWLIST citing the ticket.`,
    });
  }

  const wrapRe = /fn\s+(tree_item_\w+)\s*\(/g;
  while ((m = wrapRe.exec(content))) {
    const fnName = m[1]!;
    if (fnName === "tree_item_with_action") continue;
    const signatureEnd = content.indexOf(")", m.index);
    const signature = signatureEnd === -1 ? "" : content.slice(m.index, content.indexOf("{", signatureEnd));
    if (!signature.includes("ActionDescriptor")) continue; // structurally different helper (e.g. no action param) — not a delegation candidate
    const body = policyExtractFnBody(content, m.index);
    if (body.includes("tree_item_with_action(")) continue;
    const lineNo = policyLineOfIndex(content, m.index);
    breaches.push({
      id: `sdk-tree-item-wrapper-${scope}-${lineNo}`,
      summary: `Local "fn ${fnName}" does not delegate to tree_item_with_action`,
      kind: "app-plugin/sdk-tree-item",
      scope,
      line: lineNo,
      priority: "medium",
      reason: "Wave 4 V1 duplication audit: tree_item_* wrapper helpers must build on tree_item_with_action via struct-update syntax.",
      solution: `Rewrite ${fnName} at line ${lineNo} to construct its result via tree_item_with_action(...) plus a struct-update override.`,
    });
  }
  return breaches;
}

/** 📏️V1 rule: `struct XLabels` must be defined inside `semio_framework_plugin::app_labels! { ... }`, unless allowlisted as a documented SDK gap. */
function policyLabelsStructBreaches(crate: PolicyCrateRef, content: string): BreachRecord[] {
  const scope = crate.dir;
  const breaches: BreachRecord[] = [];
  const re = /struct\s+(\w*Labels)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) {
    const structName = m[1]!;
    // 🌱️ Line-based lookback (not a fixed char window): the `app_labels!` invocation can sit several
    // lines above the struct decl behind a multi-line /// doc comment (e.g. remodel, procedural).
    const lineNo = policyLineOfIndex(content, m.index);
    const precedingLines = content.split("\n").slice(Math.max(0, lineNo - 8), lineNo - 1);
    if (precedingLines.some((l) => l.includes("app_labels!"))) continue;
    const allowed = POLICY_LABELS_TWO_AXIS_ALLOWLIST.has(`${crate.pluginId}#${structName}`);
    breaches.push({
      id: `sdk-labels-struct-${scope}-${structName}`,
      summary: allowed ? `"struct ${structName}" is a tracked SDK-primitive gap (terminology axis) — Wave 4 decision pending` : `"struct ${structName}" hand-rolls its label set instead of semio_framework_plugin::app_labels!/LocaleLabels`,
      kind: "app-plugin/sdk-labels-struct",
      scope,
      line: lineNo,
      priority: allowed ? "low" : "medium",
      reason: allowed
        ? "Wave 4 V1 duplication audit flagged this for a Wave-4 design decision (extend LocaleLabels/app_labels! to a two-axis resolver, or formally accept the gap) — tracked, not a lint failure."
        : "Wave 4 V1 duplication audit: hand-rolled Labels structs (NATIVE/REUSE-style consts + resolver fn) should route through semio_framework_plugin::app_labels!/LocaleLabels unless there's a documented SDK-primitive gap.",
      solution: allowed ? `See .🦑️repo/🎫️tickets/26/07/18/WAVE-4-V1-DUPLICATION-HUNTER-AUDIT for the pending decision; if formally accepted, keep this allowlisted with that citation.` : `Route ${structName} through semio_framework_plugin::app_labels! { ... }, or if it needs a second axis, add it to POLICY_LABELS_TWO_AXIS_ALLOWLIST citing a ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleSdkMechanisms

//#region 🔧️PolicyRuleCargoArtifacts
/**
 * 📏️V2 rule: no stray `Cargo.lock` checked into a discovered crate dir (the workspace root owns the
 * single lockfile; a nested one is the classic leftover of an isolated verification overlay).
 * `target/` is deliberately NOT checked: the repo-wide `.gitignore` `target/` entry makes an on-disk
 * `target/` a local build product that can never be "checked into" anything, so flagging it would turn
 * every developer's own `cargo build` into a high-priority breach.
 */
function policyCargoArtifactBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (!existsSync(join(repoRoot, crate.dir, "Cargo.lock"))) continue;
    breaches.push({
      id: `stray-cargo-artifact-${crate.dir}-Cargo.lock`,
      summary: `Stray "Cargo.lock" checked into ${crate.dir}/`,
      kind: "app-plugin/stray-cargo-artifact",
      scope: crate.dir,
      priority: policyNewSurfacePriority(crate, "high"),
      reason: "Wave 4 V2 structure audit: crates must not carry their own Cargo.lock (workspace-managed) — a nested one silently pins a second dependency graph.",
      solution: `Remove ${crate.dir}/Cargo.lock (leftover of an isolated verification overlay).`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleCargoArtifacts

//#region 🔧️PolicyRuleAppCoupling
const POLICY_CARGO_DEP_RE = /^([\w.-]+)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$/gm;

/**
 * 📏️V3 rule: a plugin crate's path-dependencies must not reach into another plugin's crate (blocking)
 * or undocumented domain path (tracked). "Always allowed" used to be a hand-maintained ASCII prefix
 * list (`framework/`, `vcs/`, `protocol/`, …) that never matched this repo's real emoji-prefixed
 * directories (dead in the same way the discoverer below it was) — replaced with the structural check
 * that actually expresses the rule's intent: coupling is only a concern for dependencies that resolve
 * *inside* a discovered plugin owner (`policyScopeKey` returns `""` for anything else —
 * framework/compose/hub/repo infra is always allowed by construction, no prefix list to keep in sync).
 */
function policyAppCouplingBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const crateDirSet = new Set(crates.map((c) => c.dir));
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    const { dir, pluginId: selfPluginId } = crate;
    const cargoTomlAbs = join(repoRoot, dir, "Cargo.toml");
    if (!existsSync(cargoTomlAbs)) continue;
    const text = readFileSync(cargoTomlAbs, "utf8");
    let m: RegExpExecArray | null;
    POLICY_CARGO_DEP_RE.lastIndex = 0;
    while ((m = POLICY_CARGO_DEP_RE.exec(text))) {
      const depName = m[1]!;
      const depPath = m[2]!;
      const resolvedAbs = resolve(join(repoRoot, dir), depPath);
      const resolvedRel = relative(repoRoot, resolvedAbs).split("\\").join("/");
      const otherPluginId = policyScopeKey(repoRoot, resolvedRel);
      if (!otherPluginId || otherPluginId === selfPluginId) continue; // outside the plugins tree entirely, or the same plugin's own tree
      if (crateDirSet.has(resolvedRel)) {
        breaches.push({
          id: `app-coupling-plugin-${dir}-${depName}`,
          summary: `${dir} depends directly on another plugin's crate (${depName} -> ${resolvedRel})`,
          kind: "app-plugin/app-coupling",
          scope: dir,
          // 🎫️ W0: downgraded from "high" — the discoverer revival surfaced ~20 real, pre-existing
          // cross-plugin dependencies (✒️writer, 📕️norm's en1992/en1993, 💠️lowpoly, and 15 for
          // 🎪️demonstrator, which the plan explicitly documents as depending on other plugins' crates by
          // design and merges last for exactly that reason — see the master plan's crate-list rulings).
          // None of these are mine to repoint from script.ts; WARN-only until each dependency is
          // resolved by its owning plugin's wave agent, then flip back to "high".
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: "Wave 4 V3 coupling audit: no plugin crate may depend on another plugin's crate.",
          solution: `Remove the "${depName}" dependency from ${dir}/Cargo.toml, or move the shared logic into a neutral domain crate outside any plugin's tree (e.g. under 🧰️framework).`,
        });
        continue;
      }
      if (POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST.has(depName)) continue;
      breaches.push({
        id: `app-coupling-domain-${dir}-${depName}`,
        summary: `${dir} depends on a path under another plugin's tree (${depName} -> ${resolvedRel}) not yet vetted as shared infra`,
        kind: "app-plugin/app-coupling",
        scope: dir,
        priority: "low",
        reason: "Wave 4 V3 coupling audit: dependencies into another plugin's folder are only acceptable for documented neutral shared domain crates (see that plugin's AGENTS.md); everything else needs a look.",
        solution: `If "${depName}" is genuinely a shared domain/library crate (documented via AGENTS.md), add it to POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST citing the ticket; otherwise remove the dependency.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleAppCoupling

//#region 🔧️PolicyRuleNoJsonFixtures
/** 🔎️Repo-wide `.../example/*.json` file paths (repo-relative), skipping the same dirs `policyDiscoverCrateDirs` skips. */
function policyDiscoverExampleJsonFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (relDir.split("/").pop() === "example" && ent.name.endsWith(".json")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 📏️dsl/ migration lock-step rule: no example/*.json fixture may exist — every technology's example fixtures moved to its own DSL-text extension (`.puzzle2d`, `.flow`, `.draw`, …); a documented allowlist covers not-yet-migrated technologies. */
function policyJsonFixtureBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverExampleJsonFiles(repoRoot)) {
    if (POLICY_JSON_FIXTURE_ALLOWLIST.has(relPath)) continue;
    if (POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST.some((p) => relPath.startsWith(p))) continue;
    breaches.push({
      id: `no-json-fixture-${relPath}`,
      summary: `"${relPath}" is a JSON example fixture — technologies now use DSL-text fixtures`,
      kind: "dsl-migration/no-json-fixture",
      scope: relPath,
      priority: "high",
      reason: "The dsl/ derive-engine migration moved every technology's example fixtures from JSON to its own DSL-text extension (e.g. .puzzle2d, .flow, .draw); new */example/*.json files should never be added.",
      solution: `Convert ${relPath} to its technology's DSL-text extension (see an already-migrated sibling crate's *_fixture round-trip test for the pattern), or if this technology is not yet converted, add it to POLICY_JSON_FIXTURE_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoJsonFixtures

//#region 🔧️PolicyRuleOpsGrammar
/** 🔎️Repo-wide `*.ops` file paths (repo-relative). */
function policyDiscoverOpsFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".ops")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🏷️`.ops` op-log structural line keywords (see `vcs::print_document_text`/`parse_document_text`). */
const POLICY_OPS_STRUCTURAL_KEYWORDS = ["@doc", "@edit", "@change", "@checkpoint", "@alternative", "@active"];

/**
 * 📏️dsl/ migration lock-step rule: basic `.ops` op-log grammar sanity — one structural `@doc`/`@edit`/
 * `@change`/`@checkpoint`/`@alternative`/`@active` line, or one 2-space-indented mutation-text (op grammar)
 * line under a preceding `@edit`, per line; never blank. The `.ops` brand is the compact op-log format
 * (not the document `Mutation` trait name). Forward-looking: no `*.ops` fixture exists in the repo yet
 * (`FolderTextStorage`'s `.ops` companion file is additive/unwired — see `vcs/rs/lib.rs`), but the check
 * is ready the day one lands.
 */
function policyOpsGrammarBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverOpsFiles(repoRoot)) {
    const text = readFileSync(join(repoRoot, relPath), "utf8");
    const rawLines = text.split(/\r?\n/);
    const lines = rawLines.length > 0 && rawLines[rawLines.length - 1] === "" ? rawLines.slice(0, -1) : rawLines;
    lines.forEach((line, i) => {
      const lineNo = i + 1;
      if (line.trim().length === 0) {
        breaches.push({
          id: `ops-grammar-blank-${relPath}-${lineNo}`,
          summary: `"${relPath}" line ${lineNo} is blank`,
          kind: "dsl-migration/ops-grammar",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: "The .ops op-log grammar is one structural @-line or one indented mutation-text (op grammar) line per line — no blank lines.",
          solution: `Remove the blank line at ${relPath}:${lineNo}.`,
        });
        return;
      }
      const isStructural = POLICY_OPS_STRUCTURAL_KEYWORDS.some((kw) => line.startsWith(kw));
      const isIndentedOp = line.startsWith("  ") && line.trim().length > 0;
      if (!isStructural && !isIndentedOp) {
        breaches.push({
          id: `ops-grammar-line-${relPath}-${lineNo}`,
          summary: `"${relPath}" line ${lineNo} is neither a structural @-line nor a 2-space-indented mutation-text (op grammar) line`,
          kind: "dsl-migration/ops-grammar",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: "Every .ops line must be a recognized @doc/@edit/@change/@checkpoint/@alternative/@active structural line or a 2-space-indented mutation-text line (op grammar) under an @edit block.",
          solution: `Fix ${relPath}:${lineNo} to match the .ops grammar (see vcs::print_document_text/parse_document_text).`,
        });
      }
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleOpsGrammar

//#region 🔧️PolicyRuleDslCompleteness
/** 🔎️Repo-wide `*.rs` file paths (repo-relative), skipping the same dirs `policyDiscoverCrateDirs` skips. */
function policyAllRustFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".rs")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

type PolicyDocumentAppUsage = { scope: string; line: number; appType: string; snapshotType: string; mutationType: string };

const POLICY_DOCUMENT_APP_IMPL_RE = /impl\s+DocumentApp\s+for\s+(\w+)\s*\{/g;

/** 🔎️Extracts `type Snapshot = X;` / `type Mutation = Y;` (generic args stripped) from every `impl DocumentApp for … { … }` block in one file's content. */
function policyDocumentAppUsages(scope: string, content: string): PolicyDocumentAppUsage[] {
  const usages: PolicyDocumentAppUsage[] = [];
  let m: RegExpExecArray | null;
  POLICY_DOCUMENT_APP_IMPL_RE.lastIndex = 0;
  while ((m = POLICY_DOCUMENT_APP_IMPL_RE.exec(content))) {
    const body = policyExtractFnBody(content, m.index);
    const snapshotMatch = body.match(/type\s+Snapshot\s*=\s*([\w:<>]+)\s*;/);
    const mutationMatch = body.match(/type\s+Mutation\s*=\s*([\w:<>]+)\s*;/);
    if (!snapshotMatch || !mutationMatch) continue;
    usages.push({
      scope,
      line: policyLineOfIndex(content, m.index),
      appType: m[1]!,
      snapshotType: snapshotMatch[1]!.split("::").pop()!.replace(/<.*$/, ""),
      mutationType: mutationMatch[1]!.split("::").pop()!.replace(/<.*$/, ""),
    });
  }
  return usages;
}

const POLICY_STRUCT_OR_ENUM_DECL_RE = /^\s*(?:pub\s+)?(?:struct|enum)\s+(\w+)\b/gm;
const POLICY_DSL_DERIVE_RE = /derive\([^)]*\bDsl(?:Document|Ops|Record|Enum|Scalar)\b[^)]*\)/;
// 🎞️ CW9: `OpText` moved from `vcs` to `protocol` (see protocol/command/rs) — CW7's import-path sweep
// rewrote every hand-rolled `impl vcs::OpText for X` to `impl protocol::OpText for X`, so this must
// recognize both prefixes (`DocumentDsl` stays vcs-owned, but allowing `protocol::` there too is
// harmless since no such impl exists).
const POLICY_HAND_ROLLED_IMPL_RE = /impl(?:<[^>]*>)?\s+(?:vcs::|protocol::)?(DocumentDsl|OpText)\s+for\s+(?:vcs::|protocol::)?(\w+)/g;

/** 🧬️One O(total content) pass building every type name that's DSL-complete — via `#[derive(dsl::Dsl…)]` a few lines above its own `struct`/`enum` declaration, or a hand-rolled `impl (vcs::|protocol::)?DocumentDsl`/`impl (vcs::|protocol::)?OpText` (an explicit generic impl also counts, e.g. `impl<D> OpText for SetDocumentMutation<D>`) — split by trait since a type may satisfy one but not the other. */
function policyDslCompleteTypeNames(files: readonly { content: string }[]): { documentDsl: Set<string>; opText: Set<string> } {
  const documentDsl = new Set<string>();
  const opText = new Set<string>();
  for (const { content } of files) {
    const lines = content.split("\n");
    let m: RegExpExecArray | null;
    POLICY_STRUCT_OR_ENUM_DECL_RE.lastIndex = 0;
    while ((m = POLICY_STRUCT_OR_ENUM_DECL_RE.exec(content))) {
      const lineNo = policyLineOfIndex(content, m.index);
      const precedingLines = lines.slice(Math.max(0, lineNo - 6), lineNo - 1).join("\n");
      if (POLICY_DSL_DERIVE_RE.test(precedingLines)) {
        documentDsl.add(m[1]!);
        opText.add(m[1]!);
      }
    }
    POLICY_HAND_ROLLED_IMPL_RE.lastIndex = 0;
    while ((m = POLICY_HAND_ROLLED_IMPL_RE.exec(content))) {
      (m[1] === "DocumentDsl" ? documentDsl : opText).add(m[2]!);
    }
  }
  return { documentDsl, opText };
}

const POLICY_USE_ALIAS_RE = /\b(\w+)\s+as\s+(\w+)\b/g;

/**
 * 🔎️One O(total content) pass building every `RealName as AliasName` rename seen in any `use` item
 * (e.g. `use raster_core::{RasterSnapshot as RasterDocument};`) — a technology's block-kind wrapper
 * commonly re-exports another crate's already-DSL-complete type under a locally-meaningful alias
 * (`forms`'s `PlaybookSpec as FormSpec`, `raster/plugin`'s `RasterSnapshot as RasterDocument`), so a
 * plain per-file struct/impl scan alone would report a false gap on the alias name.
 */
function policyTypeAliasMap(files: readonly { content: string }[]): Map<string, string> {
  const aliasOf = new Map<string, string>();
  for (const { content } of files) {
    let m: RegExpExecArray | null;
    POLICY_USE_ALIAS_RE.lastIndex = 0;
    while ((m = POLICY_USE_ALIAS_RE.exec(content))) aliasOf.set(m[2]!, m[1]!);
  }
  return aliasOf;
}

/** 🔎️Resolves `typeName` through `aliasOf` (`RealName as AliasName` renames) up to a handful of hops, so an alias chain still bottoms out at its real declaration. */
function policyResolveAlias(aliasOf: ReadonlyMap<string, string>, typeName: string): string {
  let resolved = typeName;
  for (let hop = 0; hop < 5; hop++) {
    const next = aliasOf.get(resolved);
    if (!next || next === resolved) break;
    resolved = next;
  }
  return resolved;
}

/**
 * 📏️dsl/ migration lock-step rule: every `impl DocumentApp for X` app's `Snapshot`/`Mutation` type
 * must be DSL-complete — `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type itself (or
 * on the real type behind a `RealName as AliasName` import rename), a hand-rolled `impl DocumentDsl`/
 * `impl OpText`, or a documented generic bridge (see `POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`).
 * Advisory/textual — the real compile-time gate is `DocumentApp`'s `Snapshot: vcs::DocumentDsl` /
 * `Mutation: vcs::OpText` bounds in `framework/plugin/rs/lib.rs`; this catches the same gap without
 * needing a full `cargo build`. A single pass builds the DSL-complete type-name sets once
 * (`policyDslCompleteTypeNames`) so checking every app's usage stays O(1) instead of re-scanning the
 * whole corpus per usage.
 */
function policyDslCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const paths = policyAllRustFiles(repoRoot);
  const files = paths.map((relPath) => ({ relPath, content: readFileSync(join(repoRoot, relPath), "utf8") }));
  const complete = policyDslCompleteTypeNames(files);
  const aliasOf = policyTypeAliasMap(files);

  for (const { relPath, content } of files) {
    for (const usage of policyDocumentAppUsages(relPath, content)) {
      const checks: readonly { label: "Snapshot" | "Mutation"; typeName: string; trait: "DocumentDsl" | "OpText"; completeNames: Set<string> }[] = [
        { label: "Snapshot", typeName: usage.snapshotType, trait: "DocumentDsl", completeNames: complete.documentDsl },
        { label: "Mutation", typeName: usage.mutationType, trait: "OpText", completeNames: complete.opText },
      ];
      for (const { label, typeName, trait, completeNames } of checks) {
        if (POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST.has(typeName)) continue;
        if (completeNames.has(typeName)) continue;
        if (completeNames.has(policyResolveAlias(aliasOf, typeName))) continue;
        breaches.push({
          id: `dsl-completeness-${relPath}-${usage.line}-${label}`,
          summary: `"${usage.scope}"'s DocumentApp::${label} = ${typeName} has neither a #[derive(dsl::Dsl...)] nor a hand-rolled impl ${trait}`,
          kind: "dsl-migration/completeness",
          scope: relPath,
          line: usage.line,
          priority: "high",
          reason: "Every DocumentApp app's Snapshot must implement vcs::DocumentDsl and Mutation must implement vcs::OpText (compiler-enforced since the Lock step) — via #[derive(dsl::DslDocument)]/#[derive(dsl::DslOps)], a hand-rolled impl, or a documented generic bridge.",
          solution: `Add #[derive(dsl::DslDocument)] (Snapshot) / #[derive(dsl::DslOps)] (Mutation) to ${typeName}, write a hand-rolled impl ${trait} for ${typeName}, or if it's a genuine generic bridge add it to POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST citing why.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDslCompleteness

//#region 🔧️PolicyRulePackCompleteness
/**
 * 📏️pack/ rollout lock-step rule: every `*.rs` file that calls `assert_dsl_round_trip(`/
 * `assert_document_text_round_trip(` must ALSO call `assert_dsl_pack_equivalence(`/
 * `assert_document_pack_round_trip(` somewhere in the same file — dsl and pack are two projections of
 * the same value model (see `vcs::DocumentPack`'s LAW doc comment), so a technology proving its DSL
 * round trip without also proving its pack round trip is an incomplete migration. A documented
 * allowlist (`POLICY_PACK_COMPLETENESS_ALLOWLIST`) tracks not-yet-converted files exactly like
 * `POLICY_JSON_FIXTURE_ALLOWLIST` does for the dsl/ migration — remove an entry once that file adds the
 * pack-equivalence call (see `vcs/rs/lib.rs`'s own `demo_dsl_pack_equivalence`/
 * `document_text_round_trips_after_apply_and_checkpoint` tests for the pattern).
 */
function policyPackCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_PACK_COMPLETENESS_ALLOWLIST.has(policyNormalizeRelPath(relPath))) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const hasDslCheck = content.includes("assert_dsl_round_trip(") || content.includes("assert_document_text_round_trip(");
    if (!hasDslCheck) continue;
    const hasPackCheck = content.includes("assert_dsl_pack_equivalence(") || content.includes("assert_document_pack_round_trip(");
    if (hasPackCheck) continue;
    breaches.push({
      id: `pack-completeness-${relPath}`,
      summary: `"${relPath}" calls assert_dsl_round_trip/assert_document_text_round_trip but never assert_dsl_pack_equivalence/assert_document_pack_round_trip`,
      kind: "pack-migration/completeness",
      scope: relPath,
      priority: "high",
      reason: "Every DSL round-trip test must have a pack-round-trip sibling on the same fixture(s), at the same test level — dsl and pack are two projections of the same (RecordSpec, RecordValue) value model, never two independently-maintained sources of truth.",
      solution: `Add an assert_dsl_pack_equivalence(...)/assert_document_pack_round_trip(...) call beside ${relPath}'s existing DSL round-trip test(s), or if this technology hasn't adopted pack yet, add "${relPath}" to POLICY_PACK_COMPLETENESS_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRulePackCompleteness

//#region 🔧️PolicyRuleCommandEnvelopeCompleteness
/**
 * 📏️CW7 command-envelope law rule — mirrors `policyPackCompletenessBreaches`'s shrinking-allowlist
 * pattern one step further: every file that already proves the dsl/pack round-trip laws
 * (`assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(`) must ALSO call
 * `vcs::test_support::assert_command_envelope_round_trip` beside them — a technology's `Edit<Mutation>`
 * round-tripping through `protocol::MutationEnvelope`s is a third projection of the same value model,
 * not an optional extra. `POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST` tracks not-yet-converted
 * files exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST` does for the pack lock step.
 */
function policyCommandEnvelopeCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST.has(policyNormalizeRelPath(relPath))) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const hasPackCheck = content.includes("assert_dsl_pack_equivalence(") || content.includes("assert_document_pack_round_trip(");
    if (!hasPackCheck) continue;
    if (content.includes("assert_command_envelope_round_trip")) continue;
    breaches.push({
      id: `command-envelope-completeness-${relPath}`,
      summary: `"${relPath}" proves the pack round-trip law but never calls assert_command_envelope_round_trip`,
      kind: "protocol-migration/command-envelope-completeness",
      scope: relPath,
      priority: "high",
      reason: "CW7 added vcs::test_support::assert_command_envelope_round_trip as the command-envelope law every technology's Edit/Mutation pair must also prove, beside its existing dsl/pack round-trip laws.",
      solution: `Add a vcs::test_support::assert_command_envelope_round_trip::<Snapshot, Mutation>(...) call beside ${relPath}'s existing pack round-trip test(s), or if this technology hasn't wired the command-envelope law yet, add "${relPath}" to POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleCommandEnvelopeCompleteness

//#region 🔧️PolicyRuleDiffCompleteness
/**
 * 📏️W1 grammar-engine wave rule (design ruling B-R4) — mirrors `policyCommandEnvelopeCompletenessBreaches`'s
 * shrinking-allowlist pattern one step further: every file that defines a real `impl
 * protocol::MutationDiff<...>` for some type must ALSO give that same type a `protocol::DiffCodec`
 * impl (via `#[derive(dsl::DslDiff)]` or a hand-rolled `impl DiffCodec for`) — a diff is a first-class
 * grammared value now, not serde-only. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` tracks not-yet-converted
 * (or permanently-exempt trait-machinery-fixture) files exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST`
 * does for the pack lock step. File-level (not per-type) detection, matching this file's established
 * convention: a file "has a diff impl" if some line matches `impl ... MutationDiff<...>`, and "has
 * DiffCodec coverage" if it mentions `dsl::DslDiff` (the derive) or `DiffCodec for` (a hand-rolled impl)
 * anywhere in the same file. Paths may still contain `/op` (the grammar facet folder stays).
 */
function policyDiffCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const diffImplPattern = /\bimpl\b[^\n{]*\bMutationDiff\s*</;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_DIFF_COMPLETENESS_ALLOWLIST.has(policyNormalizeRelPath(relPath))) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    if (!diffImplPattern.test(content)) continue;
    const hasDiffCodec = content.includes("dsl::DslDiff") || content.includes("DiffCodec for");
    if (hasDiffCodec) continue;
    breaches.push({
      id: `diff-completeness-${relPath}`,
      summary: `"${relPath}" implements protocol::MutationDiff but never gives that diff type a protocol::DiffCodec impl`,
      kind: "dsl-migration/diff-completeness",
      scope: relPath,
      priority: "high",
      reason: "Design ruling B-R4: every MutationDiff type must also be a grammared DiffCodec value (print/parse/encode/decode_diff) — via #[derive(dsl::DslDiff)] or a hand-rolled impl.",
      solution: `Add #[derive(dsl::DslDiff)] to ${relPath}'s diff type (or a hand-rolled impl DiffCodec for it), or if it's a genuine trait-machinery test fixture / not-yet-converted real type, add "${relPath}" to POLICY_DIFF_COMPLETENESS_ALLOWLIST citing why.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDiffCompleteness

//#region 🔧️PolicyRuleGrammarFileCompleteness
/**
 * 📏️W1 grammar-engine wave rule (design ruling B-R2) — the mirror-image of every other completeness
 * rule in this file: instead of scanning for real files that are missing something, this one simply
 * asserts `POLICY_GRAMMAR_FILE_ALLOWLIST` shrinks to empty over time (see that constant's doc comment
 * for the current, deliberately-empty W1 state — no app has committed a `.grammar` file yet, so
 * there is nothing to enumerate as a breach today). Kept as its own real (if currently vacuous)
 * policy function — not a bare constant check inline elsewhere — so a later wave that starts seeding
 * this allowlist with committed-but-stale `.grammar` files gets real breach reporting for free.
 */
function policyGrammarFileBreaches(_repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of POLICY_GRAMMAR_FILE_ALLOWLIST) {
    breaches.push({
      id: `grammar-file-completeness-${relPath}`,
      summary: `"${relPath}" is tracked in POLICY_GRAMMAR_FILE_ALLOWLIST as missing/stale its handcrafted .grammar.semio spec`,
      kind: "dsl-migration/grammar-file-completeness",
      scope: relPath,
      priority: "low",
      reason: "Handcrafted grammar program: each artifact facet must commit a normative .grammar.semio checked by dsl_grammar::Recognizer.",
      solution: `Handcraft and commit ${relPath}, run the conformance sweep, then remove it from POLICY_GRAMMAR_FILE_ALLOWLIST.`,
    });
  }
  return breaches;
}

function policyProtocolFileBreaches(_repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of POLICY_PROTOCOL_FILE_ALLOWLIST) {
    breaches.push({
      id: `protocol-file-completeness-${relPath}`,
      summary: `"${relPath}" is tracked in POLICY_PROTOCOL_FILE_ALLOWLIST as missing/stale its .protocol.semio spec`,
      kind: "dsl-migration/protocol-file-completeness",
      scope: relPath,
      priority: "low",
      reason: "Handcrafted protocol program: pack/spr facets must commit .protocol.semio verified by verify_protocol_bytes.",
      solution: `Handcraft and commit ${relPath}, prove recognizer agreement, then remove it from POLICY_PROTOCOL_FILE_ALLOWLIST.`,
    });
  }
  return breaches;
}

/**
 * 🏷️True when `relPath` is a constitutional artifact TS facade (facet root or mutation triad leaf).
 * Stubs under these paths are accepted — no per-file allowlist (Wave 2b).
 */
function policyIsConstitutionalTsFacadePath(relPath: string): boolean {
  const parts = relPath.replaceAll("\\", "/").split("/");
  if (parts[parts.length - 1] !== POLICY_TS_COMPONENT_LEAF) return false;
  if (!parts.includes("🗿️artifacts")) return false;
  const parent = parts[parts.length - 2] ?? "";
  if (POLICY_TS_FACADE_CONSTITUTIONAL_FACETS.has(parent)) return true;
  if ((POLICY_MUTATION_TRIAD_DIRS as readonly string[]).includes(parent) && parts.includes(POLICY_MUTATIONS_FACET)) return true;
  return false;
}

/** 🧪True when a TS facade still throws the WASM scaffold placeholder. */
function policyTsFacadeIsScaffoldStub(content: string): boolean {
  return /wire\s+.+\s+to plugin WASM/i.test(content) || /throw new Error\(\s*["'`][^"'`]*WASM[^"'`]*["'`]\s*\)/.test(content);
}

/**
 * 📏️Structural TS-facade rule (replaces POLICY_TS_FACADE_ALLOWLIST): scaffold stubs are accepted under
 * constitutional facets (`🗣️dsl`/`🔧️op`/`🔺️diff`/`🎒️pack`/`📡️spr`/`🧬️mutations`/`⚙️engine`) and under
 * `🧬️mutations/<mut>/{🦠️mutation,🔺️diff,↩️inverse}`. A stub outside those slots is a breach.
 */
function policyTsFacadeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files: string[] = [];
  const walk = (relDir: string): void => {
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name) || ent.name.startsWith(".")) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === POLICY_TS_COMPONENT_LEAF) files.push(childRel);
    }
  };
  walk("✏️s");
  for (const relPath of files.sort()) {
    if (!relPath.replaceAll("\\", "/").includes("/🗿️artifacts/")) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    if (!policyTsFacadeIsScaffoldStub(content)) continue;
    if (policyIsConstitutionalTsFacadePath(relPath)) continue;
    breaches.push({
      id: `ts-facade-misplaced-stub-${relPath}`,
      summary: `"${relPath}" is a WASM scaffold stub outside constitutional artifact facets`,
      kind: "dsl-migration/ts-facade-completeness",
      scope: relPath,
      priority: "medium",
      reason: "Scaffold stubs are only accepted under constitutional facets (including 🧬️mutations and 🔧️op) or mutation triad leaves — never elsewhere.",
      solution: `Move ${relPath} under a constitutional facet / mutation triad, or wire it to plugin WASM.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleGrammarFileCompleteness

//#region 🔧️PolicyRuleProtocolMigration
/**
 * 🔒️Names that must never again be reached through a re-created `vcs::` shim: the temporary CW3
 * `pub use protocol::{...}` re-export deleted in CW8 (`Mutation`/`MutationDiff`/`MutationMeta`/
 * `OpText`/`Edit`/`merge_concurrent_diffs`/`ReconcileReport`/`ReconcileSeverity`), the CW3-extracted
 * `semio_framework_core` types (`HybridLogicalTimestamp`/`MutationEnvelope`/`MutationDag`/`UndoPolicy`/
 * `MergeStrategyKind`), and the hub wire-frame enums (`HubClientFrame`/`HubServerFrame`).
 */
const POLICY_PROTOCOL_MIGRATION_NAMES = [
  "Mutation",
  "MutationDiff",
  "MutationMeta",
  "OpText",
  "Edit",
  "merge_concurrent_diffs",
  "ReconcileReport",
  "ReconcileSeverity",
  "HybridLogicalTimestamp",
  "MutationEnvelope",
  "MutationDag",
  "UndoPolicy",
  "MergeStrategyKind",
  "HubClientFrame",
  "HubServerFrame",
] as const;
const POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE = new RegExp(`\\bvcs::(${POLICY_PROTOCOL_MIGRATION_NAMES.join("|")})\\b`, "g");
const POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE = /use\s+(?:::)?vcs::\{([^}]*)\}/gs;

/**
 * 📏️CW8 regression-prevention rule: `vcs/rs/lib.rs`'s temporary CW3 `pub use protocol::{Mutation,
 * MutationDiff, OpText, MutationMeta, Edit, merge_concurrent_diffs, ReconcileReport,
 * ReconcileSeverity}` shim is deleted — every dependent crate now imports `protocol::` directly (see
 * `policyCommandEnvelopeCompletenessBreaches` above and CW7's import sweep). This rule fires the
 * moment `vcs::` is asked to re-supply any of those names again, or any of the other CW3-extracted
 * `semio_framework_core` types or the hub wire-frame enums — none of which `vcs` has ever re-exported,
 * so today this is pure regression prevention, not an open migration.
 */
function policyProtocolMigrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (relPath === "./🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/⚡️implementations/🦀️rust/📦️lib.rs") continue; // the crate that used to own the shim; never reaches itself via "vcs::"
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    const isCommentLine = (line: number): boolean => /^\s*(\/\/|\*)/.test(content.split(/\r?\n/)[line - 1] ?? "");

    let m: RegExpExecArray | null;
    POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE.lastIndex = 0;
    while ((m = POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE.exec(content))) {
      // Skip "crate::vcs::X" — a crate's own nested module happening to be named "vcs" (e.g.
      // compose's GraphQL "vcs" entity module) is not the external "vcs" crate's root.
      if (content.slice(Math.max(0, m.index - 7), m.index) === "crate::") continue;
      const line = lineOf(m.index);
      if (isCommentLine(line) || seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `protocol-migration-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references vcs::${m[1]} — the CW8 shim is gone, import protocol::${m[1]} directly`,
        kind: "protocol-migration/vcs-shim-regression",
        scope: relPath,
        line,
        priority: "high",
        reason: "CW8 deleted vcs/rs/lib.rs's temporary pub-use shim for protocol-owned Mutation/OpText/MutationDiff/MutationMeta/Edit/ReconcileReport/ReconcileSeverity and never re-exported the CW3-extracted framework_core types or hub wire frames — vcs:: must never resolve any of them again.",
        solution: `Import the name from "protocol" directly (e.g. "use protocol::${m[1]};") instead of "vcs::${m[1]}".`,
      });
    }

    POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE.lastIndex = 0;
    while ((m = POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE.exec(content))) {
      const tokens = new Set((m[1] ?? "").split(/[^\w]+/).filter(Boolean));
      const hit = POLICY_PROTOCOL_MIGRATION_NAMES.find((name) => tokens.has(name));
      if (!hit) continue;
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `protocol-migration-use-${relPath}-${line}`,
        summary: `"${relPath}:${line}" imports ${hit} from vcs:: — the CW8 shim is gone, import protocol::${hit} directly`,
        kind: "protocol-migration/vcs-shim-regression",
        scope: relPath,
        line,
        priority: "high",
        reason: "CW8 deleted vcs/rs/lib.rs's temporary pub-use shim; a multi-name \"use vcs::{...}\" block must not re-import a name that only protocol:: still owns.",
        solution: `Split the "use vcs::{...}" block at ${relPath}:${line}: keep vcs's own names there and add a separate "use protocol::{${hit}, ...};" for ${hit}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleProtocolMigration

//#region 🔧️PolicyRuleDbServerOnly
/** 🔎️Repo-wide `Cargo.toml` file paths (repo-relative), same walker + `POLICY_SKIP_DIRS` as the other policy file discoverers. */
function policyDiscoverCargoTomlFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "Cargo.toml") found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🛡️Directory prefixes always allowed to carry a `db`/`db_*` Cargo dependency: the db family itself and the os hub server. Compose hub crates (`compose/**​/hub/**`) are matched structurally in `policyDbAllowedDir` since they aren't one fixed prefix. */
const POLICY_DB_SERVER_ONLY_ALLOWED_PREFIXES = ["db/", "hub/"];

/** 🛡️True if `dir` (a repo-relative directory, trailing "/") may depend on `db`/`db_*`: under `db/` itself, under the os hub server, or any compose crate whose path runs through a `hub/` segment. */
function policyDbAllowedDir(dir: string): boolean {
  if (POLICY_DB_SERVER_ONLY_ALLOWED_PREFIXES.some((p) => dir.startsWith(p))) return true;
  const segments = dir.split("/").filter(Boolean);
  return segments[0] === "compose" && segments.includes("hub");
}

const POLICY_DB_DEP_RE = /^(db(?:_[a-z0-9]+)*)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$/gm;

/**
 * 📏️db/ server-only rule: no `db`/`db_*` family Cargo dependency may live outside `db/` itself, the
 * `os-hub` server (`hub/`), or a compose hub crate (`compose/**​/hub/**`) — db is
 * server-side storage for the hubs; clients keep local-first backbones (`vcs` + `store`) and
 * only ever reach db indirectly over the wire. `policyAppCouplingBreaches`'s plugin-tree coupling check
 * deliberately does not special-case `db/` so this stays the one gate that enforces it.
 */
function policyDbServerOnlyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverCargoTomlFiles(repoRoot)) {
    const lastSlash = relPath.lastIndexOf("/");
    const dir = lastSlash === -1 ? "" : relPath.slice(0, lastSlash + 1);
    if (policyDbAllowedDir(dir)) continue;
    const text = readFileSync(join(repoRoot, relPath), "utf8");
    let m: RegExpExecArray | null;
    POLICY_DB_DEP_RE.lastIndex = 0;
    while ((m = POLICY_DB_DEP_RE.exec(text))) {
      const depName = m[1]!;
      breaches.push({
        id: `db-server-only-${relPath}-${depName}`,
        summary: `"${relPath}" depends on "${depName}" outside db/'s server-only boundary`,
        kind: "protocol-migration/db-server-only",
        scope: relPath,
        priority: "high",
        reason: "db is server-side storage for the hubs — only db/ itself, hub/, and compose's hub crates may depend on a db/db_* crate; clients keep local-first backbones (vcs + store) and only ever reach db indirectly over the wire.",
        solution: `Remove the "${depName}" dependency from ${relPath}, or if this crate genuinely is a hub server, move/confirm it under db/, hub/, or a compose/**/hub/** directory.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDbServerOnly

//#region 🔧️PolicyRuleOsStateAuthority
/** 🏛️OS product tree — the only place authoritative/session/host state may live after Wave 3 lands. */
const POLICY_OS_STATE_AUTHORITY_ROOT = "🧰️framework/🛍️products/💻️os/";
/** 🚫️Separate technologies excluded from OS-state scans (compose / mit-bestand / hub). */
const POLICY_OS_STATE_AUTHORITY_EXCLUDED_PREFIXES = ["compose/", "♻️mit-bestand/", "🌎️hub/"] as const;
const POLICY_OS_STATE_AUTHORITY_NAME_RE = /(Store|Registry|Host|Session|Engine|Kernel|World|Scene|State|Cache)$/;
const POLICY_OS_STATE_INTERIOR_TY_RE = /\b(?:OnceLock|OnceCell|LazyLock|Mutex|RwLock|RefCell|Cell)\s*</;
const POLICY_OS_STATE_STATIC_MUT_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+mut\b/;
const POLICY_OS_STATE_STATIC_CONST_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:static|const)\s+(?:mut\s+)?\w+\s*:\s*([^;=]+)/;
const POLICY_OS_STATE_FN_ITEM_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+\w+/;
const POLICY_OS_STATE_STRUCT_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+(\w+)\b/;
const POLICY_OS_STATE_FIELD_MAP_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:r#)?\w+\s*:\s*[^=\n]*(?:\bHashMap\s*<|\bBTreeMap\s*<)/;
const POLICY_OS_STATE_SEQ_U32_RE = /\bseq\s*:\s*u32\b/;
const POLICY_OS_STATE_ATOMIC_ID_RE = /\bAtomicU(?:32|64)\b/;
const POLICY_OS_STATE_THREAD_LOCAL_RE = /\bthread_local!\s*(?:\{|\()/;
const POLICY_OS_STATE_LAZY_STATIC_RE = /\blazy_static!\s*\{/;

/** 🛡️True when `relPath` is in scope for OS-exclusive state authority (outside OS product; not compose/mit-bestand/hub). */
function policyOsStateAuthorityPathInScope(relPath: string): boolean {
  const n = relPath.replace(/^\.\//, "");
  if (n.startsWith(POLICY_OS_STATE_AUTHORITY_ROOT)) return false;
  if (POLICY_OS_STATE_AUTHORITY_EXCLUDED_PREFIXES.some((p) => n.startsWith(p))) return false;
  return true;
}

/** 🏷️True when `lineNo` sits inside a `#[cfg(test)] mod …` / `mod tests` brace span. */
function policyLineInTestMod(testSpans: readonly PolicyModSpan[], lineNo: number): boolean {
  return testSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine);
}

/**
 * 📏️OS-exclusive state authority: outside `🧰️framework/🛍️products/💻️os/`, no item-scope interior
 * mutability / TLS / lazy globals, no Store|Registry|Host|… types with HashMap/BTreeMap fields, and no
 * `seq: u32` / `AtomicU32`/`AtomicU64` id minting. Skips test mods; excludes compose/mit-bestand/hub.
 */
function policyOsStateAuthorityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!policyOsStateAuthorityPathInScope(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const lines = content.split(/\r?\n/);
    const testSpans = policyTestModSpans(lines);
    let depth = 0;
    const fnBodyDepths: number[] = [];
    const structStack: { name: string; depth: number; authority: boolean }[] = [];

    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      if (policyLineInTestMod(testSpans, lineNo)) {
        const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
        depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
        while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
        while (structStack.length > 0 && depth <= structStack[structStack.length - 1]!.depth) structStack.pop();
        return;
      }

      const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
      const inFn = fnBodyDepths.some((d) => depth >= d);
      const opensFn = !inFn && POLICY_OS_STATE_FN_ITEM_RE.test(raw) && /\{/.test(codeOnly);
      const structMatch = !inFn ? raw.match(POLICY_OS_STATE_STRUCT_RE) : null;

      if (!inFn) {
        if (POLICY_OS_STATE_STATIC_MUT_RE.test(raw)) {
          breaches.push({
            id: `os-state-authority-static-mut-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" declares item-scope static mut outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: item-scope static mut is forbidden outside 🧰️framework/🛍️products/💻️os/ — host DocumentStore / engine cache own authoritative state.",
            solution: `Move the state behind an OS host API (DocumentStore dispatch, draft lane, or content-addressed engine), or relocate the item under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        if (POLICY_OS_STATE_THREAD_LOCAL_RE.test(codeOnly)) {
          breaches.push({
            id: `os-state-authority-thread-local-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" uses thread_local! outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: thread_local! registries/caches are forbidden outside the OS product.",
            solution: `Replace thread_local! with host-owned session/draft state under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        if (POLICY_OS_STATE_LAZY_STATIC_RE.test(codeOnly)) {
          breaches.push({
            id: `os-state-authority-lazy-static-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" uses lazy_static! outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: lazy_static! process globals are forbidden outside the OS product.",
            solution: `Replace lazy_static! with an OS host registry / engine cache under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        const staticTy = raw.match(POLICY_OS_STATE_STATIC_CONST_RE)?.[1] ?? "";
        if (staticTy && POLICY_OS_STATE_INTERIOR_TY_RE.test(staticTy)) {
          const ty = staticTy.match(/\b(OnceLock|OnceCell|LazyLock|Mutex|RwLock|RefCell|Cell)\s*</)?.[1] ?? "interior";
          breaches.push({
            id: `os-state-authority-static-${ty}-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" declares item-scope ${ty}<…> outside the OS product`,
            kind: "os-state-authority/item-scope-global",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: item-scope OnceLock/OnceCell/LazyLock/Mutex/RwLock/RefCell/Cell are forbidden outside the OS product.",
            solution: `Delete the static/const and route ownership through the OS host (DocumentStore, draft lane, or EngineCache) under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        if (POLICY_OS_STATE_SEQ_U32_RE.test(codeOnly) || POLICY_OS_STATE_ATOMIC_ID_RE.test(codeOnly)) {
          const kind = POLICY_OS_STATE_SEQ_U32_RE.test(codeOnly) ? "seq: u32" : "AtomicU32/64";
          breaches.push({
            id: `os-state-authority-id-mint-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" mints ids via ${kind} outside the OS product`,
            kind: "os-state-authority/id-minting",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: handle/id counters (seq: u32, AtomicU32/AtomicU64) must live in the OS host, not in plugins/s-modules/framework leaves.",
            solution: `Replace local id minting with content-addressed handles or OS-issued ids under ${POLICY_OS_STATE_AUTHORITY_ROOT}.`,
          });
        }
        const authorityStruct = structStack.length > 0 && depth > structStack[structStack.length - 1]!.depth ? structStack[structStack.length - 1] : undefined;
        if (authorityStruct?.authority && POLICY_OS_STATE_FIELD_MAP_RE.test(raw)) {
          breaches.push({
            id: `os-state-authority-map-field-${relPath}-${lineNo}-${authorityStruct.name}`,
            summary: `"${relPath}:${lineNo}" ${authorityStruct.name} holds a HashMap/BTreeMap field outside the OS product`,
            kind: "os-state-authority/authority-struct-map",
            scope: relPath,
            line: lineNo,
            priority: "high",
            reason: "OS-exclusive state authority: types named *Store|*Registry|*Host|*Session|*Engine|*Kernel|*World|*Scene|*State|*Cache must not own HashMap/BTreeMap maps outside the OS product.",
            solution: `Move ${authorityStruct.name}'s map ownership into OS DocumentStore / host session / engine cache, or rename if it is not authoritative state.`,
          });
        }
      }

      if (opensFn) fnBodyDepths.push(depth + 1);
      if (structMatch && /\{/.test(codeOnly)) {
        structStack.push({ name: structMatch[1]!, depth, authority: POLICY_OS_STATE_AUTHORITY_NAME_RE.test(structMatch[1]!) });
      }

      depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
      while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
      while (structStack.length > 0 && depth <= structStack[structStack.length - 1]!.depth) structStack.pop();
    });
  }
  return breaches;
}

/** 🧬️True when `struct Name` in `content` declares at least one field (non-ZST unit/`{}` structs return false). */
function policyStructDeclaresFields(content: string, structName: string): { line: number; hasFields: boolean } | undefined {
  const re = new RegExp(String.raw`(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+${structName}\b`, "g");
  const m = re.exec(content);
  if (!m) return undefined;
  const line = policyLineOfIndex(content, m.index);
  const rest = content.slice(m.index + m[0].length);
  const open = rest.match(/^\s*([(;{])/);
  if (!open) return { line, hasFields: false };
  if (open[1] === ";") return { line, hasFields: false };
  if (open[1] === "(") {
    const tup = rest.match(/^\s*\(([^)]*)\)/);
    const inner = (tup?.[1] ?? "").replace(/\/\/[^\n]*/g, "").trim();
    return { line, hasFields: inner.length > 0 };
  }
  const body = policyExtractFnBody(content, m.index);
  if (!body.startsWith("{")) return { line, hasFields: false };
  const stripped = body
    .slice(1, -1)
    .replace(/\/\/[^\n]*/g, "")
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .trim();
  return { line, hasFields: stripped.length > 0 };
}

/**
 * 📏️DocumentApp shape law: every `impl DocumentApp for X` app type must be a ZST (no fields).
 * `register_document_app(` factory-closure args are checked only after the receiverless API lands —
 * today `PluginBundle::register_document_app` still takes `factory: impl Fn() -> A`, so that arm is a
 * deliberate no-op until Wave 1b deletes the factory.
 */
function policyDocumentAppShapeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const n = relPath.replace(/^\.\//, "");
    if (POLICY_OS_STATE_AUTHORITY_EXCLUDED_PREFIXES.some((p) => n.startsWith(p))) continue;

    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const lines = content.split(/\r?\n/);
    const testSpans = policyTestModSpans(lines);

    for (const usage of policyDocumentAppUsages(relPath, content)) {
      if (policyLineInTestMod(testSpans, usage.line)) continue;
      const decl = policyStructDeclaresFields(content, usage.appType);
      if (!decl?.hasFields) continue;
      breaches.push({
        id: `document-app-shape-${relPath}-${usage.appType}`,
        summary: `"${relPath}" impl DocumentApp for ${usage.appType} but ${usage.appType} declares fields (must be a ZST)`,
        kind: "os-state-authority/document-app-shape",
        scope: relPath,
        line: decl.line,
        priority: "high",
        reason: "DocumentApp types must be receiverless ZSTs — fields become guest-owned state that bypasses the host DocumentStore.",
        solution: `Strip fields from ${usage.appType} (move session/scratch into OS draft lane / host DocumentSession) so it is a unit struct.`,
      });
    }

    // Receiverless register_document_app API not landed yet (`factory: impl Fn() -> A` still required) —
    // factory-argument breach scanning stays a no-op until Wave 1b deletes the factory parameter.
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleOsStateAuthority

//#region 🔧️PolicyRuleNoPackFiles
/** 🔎️Repo-wide `*.pack` file paths (repo-relative), same walker + `POLICY_SKIP_DIRS` as `policyDiscoverExampleJsonFiles`/`policyDiscoverOpsFiles`. */
function policyDiscoverPackFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".pack")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/**
 * 📏️pack/ rollout rule: no `*.pack` binary file may ever be committed. Pack is authoritative-but-
 * regeneratable local storage + export/import (`FolderTextStorage`/`FolderSqliteStorage`'s pack
 * columns/files, both dev-disposable), never a checked-in artifact — examples stay committed DSL text
 * so diffs and golden hashes stay human-legible (golden fixtures are text blake3 hashes, not binary
 * blobs). `target/` and other build-artifact dirs are already excluded via `POLICY_SKIP_DIRS`.
 */
function policyNoPackFilesBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverPackFiles(repoRoot)) {
    breaches.push({
      id: `no-pack-file-${relPath}`,
      summary: `"${relPath}" is a committed *.pack binary file — pack is dev-disposable and must never be committed`,
      kind: "pack-migration/no-pack-file",
      scope: relPath,
      priority: "high",
      reason: "pack's disk role is regeneratable local storage + export/import, not a committed artifact; committing binary .pack blobs would also break the text-diffability the DSL-mirror/golden-hash design relies on.",
      solution: `Delete ${relPath} from the repo (git rm) and rely on its DSL-text mirror / regenerate it via DocumentPack::encode_pack.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoPackFiles

//#region 🔧️PolicyRuleNoRawSpawn
const POLICY_RAW_SPAWN_EXEMPT = new Set(["./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts"]);
const POLICY_RAW_SPAWN_RE = /\b(spawnSync|execSync|execFileSync|Bun\.spawn|spawn)\s*\(/g;

/** 🔎️Strips TS/JS comments and string literals so policy regexes only see executable code. */
function policyStripTsCommentsAndStrings(content: string): string {
  const out: string[] = [];
  let i = 0;
  while (i < content.length) {
    const ch = content[i]!;
    const next = content[i + 1];
    if (ch === "/" && next === "/") {
      while (i < content.length && content[i] !== "\n") {
        out.push(" ");
        i++;
      }
      continue;
    }
    if (ch === "/" && next === "*") {
      i += 2;
      while (i < content.length && !(content[i] === "*" && content[i + 1] === "/")) {
        out.push(" ");
        i++;
      }
      if (i < content.length) i += 2;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      const quote = ch;
      out.push(" ");
      i++;
      while (i < content.length) {
        if (content[i] === "\\") {
          out.push(" ");
          out.push(" ");
          i += 2;
          continue;
        }
        if (content[i] === quote) {
          out.push(" ");
          i++;
          break;
        }
        out.push(" ");
        i++;
      }
      continue;
    }
    out.push(ch);
    i++;
  }
  return out.join("");
}

/** 🔎️Repo-wide `script.ts` file paths (repo-relative), skipping node_modules/target/.🦑️repo and other policy skip dirs. */
function policyDiscoverScriptTsFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "📜️script.ts") found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/**
 * 📏️Budgeted-spawn rule: every `script.ts` must route subprocesses through the budgeted runners in
 * `repo/lib/js/index.ts` — raw `spawn`/`spawnSync`/`execSync`/`execFileSync`/`Bun.spawn` bypass
 * wall-clock budgets and are forbidden outside the budget implementation itself.
 */
function policyRawSpawnBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverScriptTsFiles(repoRoot)) {
    if (POLICY_RAW_SPAWN_EXEMPT.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const stripped = policyStripTsCommentsAndStrings(content);
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    let m: RegExpExecArray | null;
    POLICY_RAW_SPAWN_RE.lastIndex = 0;
    while ((m = POLICY_RAW_SPAWN_RE.exec(stripped))) {
      const token = m[1] ?? "spawn";
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-raw-spawn-${relPath}-${line}`,
        summary: `"${relPath}:${line}" uses raw ${token}( — route subprocesses through runCmd/runCmdStatus/runProbe/runTestBudgeted from repo/lib/js/index.ts`,
        kind: "budget/no-raw-spawn",
        scope: relPath,
        line,
        priority: "high",
        reason: "Raw child_process/Bun.spawn calls bypass the repo's wall-clock budget layer; script.ts files must use the budgeted runners so orchestrators, tool calls and tests cannot hang forever.",
        solution: `Replace ${token}( at ${relPath}:${line} with runCmd, runCmdStatus, runProbe, or runTestBudgeted from repo/lib/js/index.ts (or orchestratorBudgetOpts/daemonBudgetOpts for long-running classes).`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoRawSpawn

//#region 🔧️PolicyRuleNoBudgetNull
/**
 * 📏️Budget-null rule: `budgetMs: null` is not a supported escape hatch — use orchestratorBudgetOpts()
 * or daemonBudgetOpts() (or an explicit positive budget) so every subprocess has a named budget class.
 */
function policyBudgetNullBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverScriptTsFiles(repoRoot)) {
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    const re = /budgetMs:\s*null/g;
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) {
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-budget-null-${relPath}-${line}`,
        summary: `"${relPath}:${line}" passes budgetMs: null — use orchestratorBudgetOpts() or daemonBudgetOpts() instead`,
        kind: "budget/no-budget-null",
        scope: relPath,
        line,
        priority: "high",
        reason: "budgetMs: null disables wall-clock budgeting for that subprocess; named orchestrator/daemon budget classes keep long-running work bounded without anonymous unbounded escapes.",
        solution: `At ${relPath}:${line}, replace budgetMs: null with orchestratorBudgetOpts() for nx/script fan-outs or daemonBudgetOpts() for dev servers/MCP stdio, or pass an explicit positive budgetMs.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoBudgetNull

//#region 🔧️PolicyRuleMcpConfig
const POLICY_MCP_CONFIG_PATHS = [".cursor/mcp.json", ".mcp.json", ".vscode/mcp.json", ".windsurf/mcp.json", ".kiro/settings/mcp.json", ".codex/config.toml"] as const;

type PolicyMcpServerEntry = { type?: string; command?: string; args?: string[] };

/** 🔎️True when a repo MCP server uses the cross-platform bootstrap that builds the native CLI on demand. */
function policyMcpRepoServerUsesBootstrap(entry: PolicyMcpServerEntry): boolean {
  if ((entry.type ?? "stdio") !== "stdio") return false;
  const cmd = (entry.command ?? "").trim();
  const args = entry.args ?? [];
  return cmd === "bun" && args[0] === "./📜️script.ts" && args[1] === "dev" && args[2] === "mcp" && args[3] === "stdio" && Boolean(args[4]);
}

function policyMcpRepoServerFromJson(doc: unknown): PolicyMcpServerEntry | undefined {
  if (!doc || typeof doc !== "object") return undefined;
  const root = doc as Record<string, unknown>;
  const servers = (root.mcpServers ?? root.servers) as Record<string, PolicyMcpServerEntry> | undefined;
  return servers?.repo;
}

function policyMcpRepoServerFromToml(content: string): PolicyMcpServerEntry | undefined {
  const section = /\[mcp_servers\.repo\]([\s\S]*?)(?:\n\[|$)/.exec(content);
  if (!section) return undefined;
  const block = section[1] ?? "";
  const command = /^\s*command\s*=\s*"([^"]*)"/m.exec(block)?.[1];
  const argsMatch = /^\s*args\s*=\s*\[([^\]]*)\]/m.exec(block);
  const args = argsMatch?.[1]
    ?.split(",")
    .map((s) => s.trim().replace(/^"|"$/g, ""))
    .filter(Boolean);
  const type = /^\s*type\s*=\s*"([^"]*)"/m.exec(block)?.[1];
  return { type, command, args };
}

/**
 * 📏️MCP-config rule: every client launches through the Bun bootstrap so missing platform-specific binaries
 * are built before stdio is handed to the native repo MCP server.
 */
function policyMcpConfigBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of POLICY_MCP_CONFIG_PATHS) {
    const abs = join(repoRoot, relPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const entry = relPath.endsWith(".toml") ? policyMcpRepoServerFromToml(content) : policyMcpRepoServerFromJson(JSON.parse(content) as unknown);
    if (entry && policyMcpRepoServerUsesBootstrap(entry)) continue;
    breaches.push({
      id: `mcp-config-${relPath}`,
      summary: `"${relPath}" does not use the portable repo MCP bootstrap`,
      kind: "runtime/mcp-config-repo-bootstrap",
      scope: relPath,
      priority: "high",
      reason: "A checked-in native executable is platform-specific and may be absent in a fresh checkout; the Bun router resolves or builds the correct binary without writing protocol noise to stdout.",
      solution: `In ${relPath}, set command to "bun" and args to ["./📜️script.ts", "dev", "mcp", "stdio", "<kind>"].`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleMcpConfig

//#region 🔧️PolicyRuleTaxonomy
/**
 * 🎫️ Structural rules over the Shape V2 taxonomy tree (`<owner>/📦️packages/<lang>[/🎯️targets/<t>]`,
 * `PolicyCrateRef.shape === "taxonomy"`), master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`. Their vocabulary is no longer
 * duplicated here: every directory name, leaf filename and line budget below is read from the shared
 * `🔣️taxonomy.json` via `loadTaxonomy()` (mechanism step M1), so a vocabulary edit can never leave the
 * registry validator and this policy disagreeing again.
 *
 * 🚦️Severity: WARN-only (never `"high"`) until the finalization flip promotes them. Findings on
 * framework/hub/s-module owners — surface these rules could not see at all before mechanism step M4
 * removed the Shape-V1-only, plugins-only discoverer — land one notch lower still (`"low"`, via
 * `policyNewSurfacePriority`), because nothing has ever triaged them.
 */

function policyReaddirSafe(repoRoot: string, relDir: string): { name: string; isDirectory: boolean }[] {
  try {
    return readdirSync(join(repoRoot, relDir), { withFileTypes: true })
      .filter((e) => !POLICY_SKIP_DIRS.has(e.name))
      .map((e) => ({ name: e.name, isDirectory: e.isDirectory() }));
  } catch {
    return [];
  }
}

/**
 * 📏️Taxonomy validator, discovery-contract clause 1: every `🗿️artifacts/<a>/` may only contain the known
 * artifact child vocabulary (`taxonomy.artifactChildDirs`, plus its own leaf file), and every
 * `🪟️windows/<w>/` may only contain `taxonomy.windowChildDirs` (plus its own leaf file).
 */
function policyTaxonomyDirsBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;
    const scopeId = crate.pluginId || policyStripEmoji(ownerRoot.split("/").pop() ?? "");

    const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
    const snapshotChildDirs = taxonomy.snapshotChildDirs ?? [];
    const diffChildDirs = taxonomy.diffChildDirs ?? [];
    for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
      const artifactDir = `${artifactsRoot}/${artifact.name}`;
      for (const child of policyReaddirSafe(repoRoot, artifactDir).filter((e) => e.isDirectory)) {
        if (taxonomy.artifactChildDirs.includes(child.name)) {
          //#region NestedFacetWalk
          if (child.name === "📸️snapshot") {
            const nestedRoot = `${artifactDir}/${child.name}`;
            for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
              if (snapshotChildDirs.includes(nested.name)) continue;
              breaches.push({
                id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
                summary: `"${nestedRoot}/${nested.name}" is not a recognized snapshot child dir`,
                kind: "taxonomy/dirs",
                scope: `${scopeId}/${policyStripEmoji(artifact.name)}`,
                priority: policyNewSurfacePriority(crate, "medium"),
                reason: `Discovery contract: 📸️snapshot may only contain ${snapshotChildDirs.join(", ")}.`,
                solution: `Move "${nested.name}" into a recognized snapshotChildDirs member, or add it to 🔣️taxonomy.json's snapshotChildDirs with a ticket citation.`,
              });
            }
          } else if (child.name === "🔺️diff") {
            const nestedRoot = `${artifactDir}/${child.name}`;
            for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
              if (diffChildDirs.includes(nested.name)) continue;
              breaches.push({
                id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
                summary: `"${nestedRoot}/${nested.name}" is not a recognized diff child dir`,
                kind: "taxonomy/dirs",
                scope: `${scopeId}/${policyStripEmoji(artifact.name)}`,
                priority: policyNewSurfacePriority(crate, "medium"),
                reason: `Discovery contract: 🔺️diff may only contain nested dirs from ${diffChildDirs.join(", ")} (grammar/codec leaves stay at the diff root).`,
                solution: `Move "${nested.name}" into a recognized diffChildDirs member, or add it to 🔣️taxonomy.json's diffChildDirs with a ticket citation.`,
              });
            }
          } else if (child.name === "🚪️io") {
            const formatDirs = new Set(Object.values((taxonomy as { mediaFormatDirs?: Record<string, string> }).mediaFormatDirs ?? {}));
            const ioFormatChildDirs = ((taxonomy as { ioFormatChildDirs?: string[] }).ioFormatChildDirs ?? []) as string[];
            const nestedRoot = `${artifactDir}/${child.name}`;
            for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
              if (!formatDirs.has(nested.name)) {
                breaches.push({
                  id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
                  summary: `"${nestedRoot}/${nested.name}" is not a recognized media format dir`,
                  kind: "taxonomy/dirs",
                  scope: `${scopeId}/${policyStripEmoji(artifact.name)}`,
                  priority: policyNewSurfacePriority(crate, "medium"),
                  reason: `Discovery contract: 🚪️io may only contain dirs from taxonomy.mediaFormatDirs.`,
                  solution: `Rename "${nested.name}" to a mediaFormatDirs value, or add it to 🔣️taxonomy.json's mediaFormatDirs with a ticket citation.`,
                });
                continue;
              }
              const formatRoot = `${nestedRoot}/${nested.name}`;
              for (const leaf of policyReaddirSafe(repoRoot, formatRoot).filter((e) => e.isDirectory)) {
                if (ioFormatChildDirs.includes(leaf.name)) continue;
                breaches.push({
                  id: `taxonomy-dirs-artifact-${formatRoot}-${leaf.name}`,
                  summary: `"${formatRoot}/${leaf.name}" is not a recognized io format child`,
                  kind: "taxonomy/dirs",
                  scope: `${scopeId}/${policyStripEmoji(artifact.name)}`,
                  priority: policyNewSurfacePriority(crate, "medium"),
                  reason: `Discovery contract: each 🚪️io/<format>/ may only contain ${ioFormatChildDirs.join(", ")}.`,
                  solution: `Move "${leaf.name}" into 📥️import or 📤️export, or update ioFormatChildDirs.`,
                });
              }
            }
          }
          //#endregion NestedFacetWalk
          continue;
        }
        breaches.push({
          id: `taxonomy-dirs-artifact-${artifactDir}-${child.name}`,
          summary: `"${artifactDir}/${child.name}" is not a recognized artifact component dir`,
          kind: "taxonomy/dirs",
          scope: `${scopeId}/${policyStripEmoji(artifact.name)}`,
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: `Discovery contract: an artifact dir may only contain ${taxonomy.artifactChildDirs.join(", ")}.`,
          solution: `Move "${child.name}" into a recognized component dir, or if it's a genuinely new taxonomy vocabulary word, add it to 🔣️taxonomy.json's artifactChildDirs with a ticket citation.`,
        });
      }
    }

    const walkForWindows = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir).filter((e) => e.isDirectory)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.name === taxonomy.windowsDirName) {
          for (const w of policyReaddirSafe(repoRoot, childRel).filter((e) => e.isDirectory)) {
            const windowDir = `${childRel}/${w.name}`;
            for (const child of policyReaddirSafe(repoRoot, windowDir).filter((e) => e.isDirectory)) {
              if (taxonomy.windowChildDirs.includes(child.name)) continue;
              breaches.push({
                id: `taxonomy-dirs-window-${windowDir}-${child.name}`,
                summary: `"${windowDir}/${child.name}" is not a recognized window child dir`,
                kind: "taxonomy/dirs",
                scope: `${scopeId}/${policyStripEmoji(w.name)}`,
                priority: policyNewSurfacePriority(crate, "medium"),
                reason: `Discovery contract: a window dir may only contain ${taxonomy.windowChildDirs.join(", ")}.`,
                solution: `Move "${child.name}" into a recognized window-child dir, or if it's a genuinely new taxonomy vocabulary word, add it to 🔣️taxonomy.json's windowChildDirs with a ticket citation.`,
              });
            }
          }
          continue;
        }
        walkForWindows(childRel);
      }
    };
    walkForWindows(`${ownerRoot}/${taxonomy.appsDirName}`);
  }
  return breaches;
}

/**
 * 📏️ Per-example unit shape: `📚️examples/<emoji-slug>/{definition leaves, 🖼️assets/, 🧪️tests/}` under
 * every artifact and every app (apps own examples directly — never under `⚙️engine`). Plugin-root
 * `📚️examples` and plural facet dirs are forbidden.
 */
function policyExampleSlugOk(slug: string, taxonomy: ReturnType<typeof loadTaxonomy>): boolean {
  try {
    return new RegExp(taxonomy.exampleSlugPattern, "u").test(slug);
  } catch {
    return false;
  }
}

/** 📚️ True when `relDir` is an immediate child of `📚️examples` (dynamic emoji-slug leaf parent). */
function policyIsExampleSlugDir(relDir: string): boolean {
  const parts = relDir.replaceAll("\\", "/").split("/");
  return parts.length >= 2 && parts[parts.length - 2] === "📚️examples";
}

function policyValidateExampleUnit(
  repoRoot: string,
  exampleRel: string,
  scope: string,
  priority: BreachRecord["priority"],
): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const slug = exampleRel.split("/").pop() ?? "";
  if ((taxonomy.forbiddenExampleSlugs ?? []).includes(slug) || !policyExampleSlugOk(slug, taxonomy)) {
    breaches.push({
      id: `semio-examples-slug-${exampleRel}`,
      summary: `"${exampleRel}" is not a valid emoji+VS16+kebab example slug`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: `Example dirs must match exampleSlugPattern and must not be placeholders (${(taxonomy.forbiddenExampleSlugs ?? []).join(", ")}).`,
      solution: `Rename ${exampleRel} to an emoji+VS16+kebab slug that describes the scenario.`,
    });
  }
  for (const plural of taxonomy.forbiddenExamplePluralDirs ?? []) {
    if (existsSync(join(repoRoot, exampleRel, plural))) {
      breaches.push({
        id: `semio-examples-plural-${exampleRel}-${plural}`,
        summary: `"${exampleRel}/${plural}" plural example facet dir is forbidden`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example assets live flat under 🖼️assets/ with kind-emoji prefixes — plural dsls/packs/ops/sprs dirs are gone.",
        solution: `Move files from ${exampleRel}/${plural}/ into ${exampleRel}/${taxonomy.exampleAssetsDirName}/ and delete the plural dir.`,
      });
    }
  }
  const rustLeaf = taxonomy.exampleLeafFilenames?.["🦀️rust"] ?? "🦀️component.rs";
  const tsLeaf = taxonomy.exampleLeafFilenames?.["🟦️typescript"] ?? "🟦️component.ts";
  if (!existsSync(join(repoRoot, exampleRel, rustLeaf))) {
    breaches.push({
      id: `semio-examples-leaf-rs-${exampleRel}`,
      summary: `"${exampleRel}" is missing definition leaf ${rustLeaf}`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit needs a Rust definition leaf as the single source of truth for id/labels/assets.",
      solution: `Add ${exampleRel}/${rustLeaf}.`,
    });
  }
  if (!existsSync(join(repoRoot, exampleRel, tsLeaf))) {
    breaches.push({
      id: `semio-examples-leaf-ts-${exampleRel}`,
      summary: `"${exampleRel}" is missing definition leaf ${tsLeaf}`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit needs a TypeScript definition leaf for TS consumers.",
      solution: `Add ${exampleRel}/${tsLeaf}.`,
    });
  }
  const assetsRel = `${exampleRel}/${taxonomy.exampleAssetsDirName}`;
  if (!existsSync(join(repoRoot, assetsRel))) {
    breaches.push({
      id: `semio-examples-assets-${exampleRel}`,
      summary: `"${exampleRel}" is missing ${taxonomy.exampleAssetsDirName}/`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit carries its assets under 🖼️assets/.",
      solution: `Add ${assetsRel}/ with kind-prefixed asset files.`,
    });
  }
  const testsRel = `${exampleRel}/${taxonomy.exampleTestsDirName}`;
  if (!existsSync(join(repoRoot, testsRel))) {
    breaches.push({
      id: `semio-examples-tests-${exampleRel}`,
      summary: `"${exampleRel}" is missing ${taxonomy.exampleTestsDirName}/`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit carries co-located tests under 🧪️tests/.",
      solution: `Add ${testsRel}/ with ${taxonomy.exampleTestLeafFilenames?.["🦀️rust"] ?? "🦀️test.rs"} and ${taxonomy.exampleTestLeafFilenames?.["🟦️typescript"] ?? "🟦️test.ts"}.`,
    });
  } else {
    const rustTest = taxonomy.exampleTestLeafFilenames?.["🦀️rust"] ?? "🦀️test.rs";
    const tsTest = taxonomy.exampleTestLeafFilenames?.["🟦️typescript"] ?? "🟦️test.ts";
    if (!existsSync(join(repoRoot, testsRel, rustTest))) {
      breaches.push({
        id: `semio-examples-test-rs-${exampleRel}`,
        summary: `"${testsRel}" is missing ${rustTest}`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example tests must include the Rust test leaf.",
        solution: `Add ${testsRel}/${rustTest}.`,
      });
    }
    if (!existsSync(join(repoRoot, testsRel, tsTest))) {
      breaches.push({
        id: `semio-examples-test-ts-${exampleRel}`,
        summary: `"${testsRel}" is missing ${tsTest}`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example tests must include the TypeScript test leaf.",
        solution: `Add ${testsRel}/${tsTest}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️ Semio example layout: ≥1 emoji-slug under each artifact and each app `📚️examples/`
 * (not under `⚙️engine`); slug matches pattern; definition leaf + 🖼️assets/ + 🧪️tests/; no plural
 * dirs; no plugin-root `📚️examples`.
 */
function policySemioArtifactExamplesBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const examplesDir = "📚️examples";
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;
    const scopeId = crate.pluginId || policyStripEmoji(ownerRoot.split("/").pop() ?? "");
    const priority = policyNewSurfacePriority(crate, "medium");
    if (existsSync(join(repoRoot, ownerRoot, examplesDir))) {
      breaches.push({
        id: `semio-examples-plugin-root-${ownerRoot}`,
        summary: `"${ownerRoot}/${examplesDir}" must not exist at plugin root`,
        kind: "taxonomy/semio-examples",
        scope: scopeId,
        priority: policyNewSurfacePriority(crate, "high"),
        reason: "Examples belong under 🗿️artifacts/<artifact>/📚️examples or 🎛️apps/<app>/📚️examples — never at the plugin root.",
        solution: `Move fixtures into artifact or app ${examplesDir}/<emoji-slug>/ units.`,
      });
    }
    const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
    for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
      const artifactRel = `${artifactsRoot}/${artifact.name}`;
      const examplesRel = `${artifactRel}/${examplesDir}`;
      const artScope = `${scopeId}/${policyStripEmoji(artifact.name)}`;
      if (!existsSync(join(repoRoot, examplesRel))) {
        breaches.push({
          id: `semio-examples-missing-${examplesRel}`,
          summary: `"${examplesRel}" is missing`,
          kind: "taxonomy/semio-examples",
          scope: artScope,
          priority,
          reason: "Every artifact must ship at least one emoji-slug example unit.",
          solution: `Add ${examplesRel}/<emoji-slug>/{${taxonomy.exampleLeafFilenames?.["🦀️rust"] ?? "🦀️component.rs"}, ${taxonomy.exampleAssetsDirName}/, ${taxonomy.exampleTestsDirName}/}.`,
        });
        continue;
      }
      const sets = policyReaddirSafe(repoRoot, examplesRel).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-empty-${examplesRel}`,
          summary: `"${examplesRel}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: artScope,
          priority,
          reason: "Artifact examples must contain at least one emoji-slug example unit.",
          solution: `Add ${examplesRel}/<emoji-slug>/ with definition leaves, assets, and tests.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${examplesRel}/${set.name}`, artScope, priority));
      }
    }
    const appsRoot = `${ownerRoot}/${taxonomy.appsDirName}`;
    for (const app of policyReaddirSafe(repoRoot, appsRoot).filter((e) => e.isDirectory)) {
      const appScope = `${scopeId}/${policyStripEmoji(app.name)}`;
      const appExamples = `${appsRoot}/${app.name}/${examplesDir}`;
      const engineExamples = `${appsRoot}/${app.name}/⚙️engine/${examplesDir}`;
      if (existsSync(join(repoRoot, engineExamples))) {
        breaches.push({
          id: `semio-examples-engine-${engineExamples}`,
          summary: `"${engineExamples}" must not live under ⚙️engine — move to the app root`,
          kind: "taxonomy/semio-examples",
          scope: appScope,
          priority,
          reason: "App examples live at 🎛️apps/<app>/📚️examples, not under ⚙️engine.",
          solution: `Move ${engineExamples} to ${appExamples}.`,
        });
      }
      if (!existsSync(join(repoRoot, appExamples))) {
        breaches.push({
          id: `semio-examples-app-missing-${appExamples}`,
          summary: `"${appExamples}" is missing`,
          kind: "taxonomy/semio-examples",
          scope: appScope,
          priority,
          reason: "Every app must ship at least one emoji-slug example unit under 📚️examples/.",
          solution: `Add ${appExamples}/<emoji-slug>/{definition leaves, ${taxonomy.exampleAssetsDirName}/, ${taxonomy.exampleTestsDirName}/}.`,
        });
        continue;
      }
      const sets = policyReaddirSafe(repoRoot, appExamples).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-app-empty-${appExamples}`,
          summary: `"${appExamples}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: appScope,
          priority,
          reason: "App examples must contain at least one emoji-slug example unit.",
          solution: `Add ${appExamples}/<emoji-slug>/ with definition leaves, assets, and tests.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${appExamples}/${set.name}`, appScope, priority));
      }
    }
  }
  return breaches;
}

function policyComponentFileBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const leafFilename = taxonomy.taxonomyLeafFilenames["🦀️rust"] ?? "🦀️component.rs";
  const sourceExtension = taxonomy.ecosystems["🦀️rust"]?.sourceExtension ?? ".rs";
  const tsLeafFilename = taxonomy.ecosystems["🟦️typescript"]?.leafFilename ?? taxonomy.taxonomyLeafFilenames["🟦️typescript"] ?? "🟦️component.ts";
  const tsSourceExtension = taxonomy.ecosystems["🟦️typescript"]?.sourceExtension ?? ".ts";
  const artifactFacetDirs = new Set(taxonomy.artifactComponentDirs ?? []);
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;

    const walk = (relDir: string): void => {
      const entries = policyReaddirSafe(repoRoot, relDir);
      const parentName = relDir.split("/").pop() ?? "";
      const hasRustFile = entries.some((e) => !e.isDirectory && e.name.endsWith(sourceExtension));
      const hasTsFile = entries.some((e) => !e.isDirectory && e.name.endsWith(tsSourceExtension));
      const exampleRustLeaf = taxonomy.exampleLeafFilenames["🦀️rust"] ?? leafFilename;
      const exampleTsLeaf = taxonomy.exampleLeafFilenames["🟦️typescript"] ?? tsLeafFilename;
      if (policyIsExampleSlugDir(relDir)) {
        if (hasRustFile && !entries.some((e) => !e.isDirectory && e.name === exampleRustLeaf)) {
          breaches.push({
            id: `component-file-example-${relDir}`,
            summary: `"${relDir}" has ${sourceExtension} source but no ${exampleRustLeaf}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Example slug dirs (grandparent 📚️examples) use ${exampleRustLeaf} as the definition leaf.`,
            solution: `Rename ${relDir}'s primary source file to ${exampleRustLeaf}.`,
          });
        }
        if (hasTsFile && !entries.some((e) => !e.isDirectory && e.name === exampleTsLeaf)) {
          breaches.push({
            id: `component-file-example-ts-${relDir}`,
            summary: `"${relDir}" has ${tsSourceExtension} source but no ${exampleTsLeaf}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Example slug dirs use ${exampleTsLeaf} as the TypeScript definition leaf.`,
            solution: `Rename ${relDir}'s primary TypeScript file to ${exampleTsLeaf}.`,
          });
        }
      } else if (taxonomy.taxonomyLeafParentDirs.includes(parentName)) {
        // relDir itself is a vocabulary dir (e.g. 🔧️op/) whose immediate .rs files are the leaf — or,
        // for 🎮️commands/🛠️tools/📌️panels, an extra <name>/ nesting level holds the leaf instead.
        if (hasRustFile && !entries.some((e) => !e.isDirectory && e.name === leafFilename)) {
          breaches.push({
            id: `component-file-${relDir}`,
            summary: `"${relDir}" has ${sourceExtension} source but no ${leafFilename}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: every taxonomy leaf's primary file must be literally named ${leafFilename}.`,
            solution: `Rename ${relDir}'s primary source file to ${leafFilename} (sibling 🦀️<topic>${sourceExtension} files may stay).`,
          });
        }
        if (artifactFacetDirs.has(parentName) && hasTsFile && !entries.some((e) => !e.isDirectory && e.name === tsLeafFilename)) {
          breaches.push({
            id: `component-file-ts-${relDir}`,
            summary: `"${relDir}" has ${tsSourceExtension} source but no ${tsLeafFilename}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: artifact facet dirs carry the TypeScript taxonomy leaf (${tsLeafFilename}) alongside ${leafFilename} and any mapped artifactSpecFilenames entry.`,
            solution: `Rename ${relDir}'s primary TypeScript facade to ${tsLeafFilename} (normative 📖️/📡️ *.semio specs mapped in artifactSpecFilenames may stay as siblings).`,
          });
        }
      }
      for (const entry of entries.filter((e) => e.isDirectory)) walk(`${relDir}/${entry.name}`);
    };
    walk(`${ownerRoot}/${taxonomy.artifactsDirName}`);
    walk(`${ownerRoot}/${taxonomy.appsDirName}`);
  }
  return breaches;
}

/**
 * 📏️Anti-inlining tripwire (Single-File-Repo hazard ruling, master ticket): a migrated package's entry
 * `📦️glue.rs` must stay wiring-only (`#[path]` mod declarations + `plugin_exports!(plugin::plugin)`) — no
 * non-trivial `fn`/`impl` body content beyond `taxonomy.libWiringLineBudget`. Catches the exact
 * regression this repo has hit twice before: an agent following the (now-scoped) "single file repo" goal
 * inlining split `#[path]` modules back into `glue.rs`. Plugin identity lives in `🔌️plugin/` via
 * `Plugin::builder`, never `semio_plugin!{}`.
 */
const POLICY_FN_OR_IMPL_OPEN_RE = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn\s+\w+\s*(?:<[^>]*>)?\s*\([^;{]*\)[^;{]*|impl(?:<[^>]*>)?\s+[^;{]+)\{\s*$/;

function policyTaxonomyLibShapeBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const lineBudget = loadTaxonomy().libWiringLineBudget;
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const lines = content.split(/\r?\n/);
    const lineStarts: number[] = [];
    for (let offset = 0, li = 0; li < lines.length; li++) {
      lineStarts.push(offset);
      offset += lines[li]!.length + 1;
    }

    let bodyLines = 0;
    let coveredUntilLine = 0; // 🌱️ skip fn/impl bodies nested inside an already-counted outer body — avoids double-counting a fn declared inside an impl block.
    lines.forEach((line, i) => {
      const lineNo = i + 1;
      if (lineNo <= coveredUntilLine || !POLICY_FN_OR_IMPL_OPEN_RE.test(line)) return;
      const body = policyExtractFnBody(content, lineStarts[i]!);
      const bodyLineList = body.split(/\r?\n/);
      bodyLines += bodyLineList.filter((l) => l.trim() && !l.trim().startsWith("//")).length;
      coveredUntilLine = lineNo + bodyLineList.length - 1;
    });
    if (bodyLines <= lineBudget) continue;
    breaches.push({
      id: `taxonomy-lib-shape-${crate.libRelPath}`,
      summary: `"${crate.libRelPath}" has ~${bodyLines} lines of fn/impl body content — a migrated package's glue.rs must stay wiring-only`,
      kind: "taxonomy/lib-shape",
      scope: crate.pluginId || crate.ownerRel,
      priority: policyNewSurfacePriority(crate, "medium"),
      reason: "Single-File-Repo hazard ruling: a taxonomy package's 📦️glue.rs is #[path] mod wiring + plugin_exports!(plugin::plugin) only — real logic lives in taxonomy component files / 🔌️plugin/, never inlined back.",
      solution: `Move the non-trivial fn/impl bodies out of ${crate.libRelPath} into their owning taxonomy component file(s) or 🔌️plugin/; glue.rs should only declare "#[path = \\"...\\"] mod ...;" and call plugin_exports!(plugin::plugin).`,
    });
  }
  return breaches;
}

/**
 * 📏️TS analogue of `policyTaxonomyLibShapeBreaches`, added by ticket
 * `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE` (Single-File-Repo hazard ruling, scope note 2): a
 * `"taxonomy"`-area TypeScript package's entry file (`taxonomy.entryFilenames["🟦️typescript"]`, e.g.
 * `🟦️glue.ts`) must stay a wiring-only re-export barrel — counts non-import/export/comment/blank lines
 * and flags a breach past `libWiringLineBudget`. Warn-only (`priority: "medium"`) and still vacuous
 * today: the graduated area state is `"clean"` and no area has graduated yet (that flip is the W6
 * activation step of that ticket). The literal it compares against used to be `"taxonomy"`, a value that
 * is not in `taxonomy.areaStates` at all and therefore could never match — fixed here to `"clean"` while
 * mechanism step M4 was in this region (see `🔣️taxonomy.json`'s `_areaStateComment`).
 */
const POLICY_BARREL_GRADUATED_AREA_STATE = "clean";
const POLICY_BARREL_WIRING_LINE_RE = /^\s*(\/\/.*)?$|^\s*export\s+(type\s+)?(\*|\{[^}]*\})\s+from\s+"[^"]+";?\s*(\/\/.*)?$|^\s*import\s+.+\s+from\s+"[^"]+";?\s*(\/\/.*)?$/;

function policyTaxonomyBarrelShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const entryFilename = taxonomy.entryFilenames["🟦️typescript"];
  if (!entryFilename) return [];
  const breaches: BreachRecord[] = [];
  for (const pkg of discoverPackages(repoRoot, taxonomy)) {
    if (pkg.lang !== "🟦️typescript" || pkg.area !== POLICY_BARREL_GRADUATED_AREA_STATE) continue;
    const entryAbs = join(repoRoot, pkg.packageRel, entryFilename);
    if (!existsSync(entryAbs)) continue;
    const lines = readFileSync(entryAbs, "utf8").split(/\r?\n/);
    const nonWiring = lines.filter((l) => l.trim() && !POLICY_BARREL_WIRING_LINE_RE.test(l));
    if (nonWiring.length <= taxonomy.libWiringLineBudget) continue;
    breaches.push({
      id: `taxonomy-barrel-shape-${pkg.manifestPath}`,
      summary: `"${pkg.packageRel}/${entryFilename}" has ~${nonWiring.length} non-wiring lines — a taxonomy package's entry file must stay a re-export barrel`,
      kind: "taxonomy/barrel-shape",
      scope: pkg.id,
      priority: "medium",
      reason: "Single-File-Repo hazard ruling (TS extension, ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE): a taxonomy package's entry barrel is import/export wiring only — real component logic lives in 🧱️elements/<Element>/ leaf files, never inlined back.",
      solution: `Move the non-wiring content out of ${entryFilename} into its owning 🧱️elements/<Element>/ leaf file(s); the entry file should only import/re-export.`,
    });
  }
  return breaches;
}

const POLICY_PROTOCOL_SEGMENT_RE = /^protocol$/i;

const POLICY_BANNED_STEM_SCAN_ROOTS = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand"] as const;
const POLICY_VS16 = "\uFE0F";

/**
 * 🚫️ Bans emoji-stripped directory/file stems listed in `taxonomy.bannedNameStems` (e.g. `core`, `shared`).
 * Medium during migration; Wave 4 flips to high once cores are dissolved.
 */
function policyBannedNameStemBreaches(repoRoot: string): BreachRecord[] {
  const stems = new Set((loadTaxonomy().bannedNameStems ?? []).map((s) => s.toLowerCase()));
  if (stems.size === 0) return [];
  const breaches: BreachRecord[] = [];
  const walk = (relDir: string): void => {
    for (const entry of policyReaddirSafe(repoRoot, relDir)) {
      if (entry.name === "node_modules" || entry.name === "target" || entry.name === "dist" || entry.name === "pkg") continue;
      const childRel = `${relDir}/${entry.name}`;
      const stem = policyStripEmoji(entry.name.replace(/\.(rs|ts|tsx|js|mjs|cjs|go|py|cs)$/, "")).toLowerCase();
      if (stems.has(stem)) {
        breaches.push({
          id: `banned-name-stem-${childRel}`,
          summary: `"${childRel}" uses banned name stem "${stem}" — use a domain concept folder instead`,
          kind: "taxonomy/banned-name-stem",
          scope: childRel,
          priority: "high",
          reason: "Clean mechanism: bannedNameStems forbids vague grab-bag folders (core/shared/util/…).",
          solution: `Rename "${entry.name}" to a domain-specific concept folder and update #[path]/imports.`,
        });
      }
      if (entry.isDirectory) walk(childRel);
    }
  };
  for (const root of POLICY_BANNED_STEM_SCAN_ROOTS) walk(root);
  return breaches;
}

/**
 * ✅️ Taxonomy directories under plugin/framework/hub areas must carry an emoji prefix that includes U+FE0F.
 */
function policyEmojiPrefixBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  if (taxonomy.requireEmojiPrefixWithVs16 !== true) return [];
  const breaches: BreachRecord[] = [];
  const hasEmojiPrefix = (name: string): boolean => {
    if (!name) return false;
    const first = [...name][0];
    if (!first || first === "." || first === "_") return true;
    const cp = first.codePointAt(0) ?? 0;
    const isEmoji = cp > 0x7f;
    if (!isEmoji) return false;
    return name.includes(POLICY_VS16) || /[\u{1F300}-\u{1FAFF}]/u.test(first);
  };
  const needsVs16 = (name: string): boolean => {
    const chars = [...name];
    if (chars.length < 2) return false;
    const first = chars[0]!;
    const cp = first.codePointAt(0) ?? 0;
    if (cp <= 0x7f) return false;
    // Allow ASCII-only tooling dirs
    if (name.startsWith(".")) return false;
    // Require VS16 when the stem after emoji is latin (e.g. 🧩core, 🔌️Ports)
    const rest = name.slice(first.length);
    if (/^[A-Za-z]/.test(rest) && !name.includes(POLICY_VS16)) return true;
    return false;
  };
  const walk = (relDir: string): void => {
    for (const entry of policyReaddirSafe(repoRoot, relDir)) {
      if (!entry.isDirectory) continue;
      if (entry.name === "node_modules" || entry.name === "target" || entry.name === "dist" || entry.name === "pkg") continue;
      const childRel = `${relDir}/${entry.name}`;
      if (needsVs16(entry.name)) {
        breaches.push({
          id: `emoji-vs16-${childRel}`,
          summary: `"${childRel}" is missing U+FE0F variation selector on its emoji prefix`,
          kind: "taxonomy/emoji-prefix",
          scope: childRel,
          priority: "high",
          reason: "taxonomy.requireEmojiPrefixWithVs16: emoji-prefixed taxonomy dirs must include U+FE0F.",
          solution: `Rename so the emoji prefix includes ${POLICY_VS16} (e.g. 🧩${POLICY_VS16}core → dissolve; 🔌️Ports → 🔌${POLICY_VS16}Ports).`,
        });
      }
      walk(childRel);
    }
  };
  for (const root of POLICY_BANNED_STEM_SCAN_ROOTS) walk(root);
  void hasEmojiPrefix;
  return breaches;
}

/**
 * 🔌️ Every plugin owner under `✏️s/🔌️plugins/` must eventually carry `🔌️plugin/` with required children.
 * Medium while Wave 3 migrates; Wave 4 flips to high.
 */
function policyPluginRootShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const pluginDir = taxonomy.pluginDirName ?? "🔌️plugin";
  const children = taxonomy.pluginChildDirs ?? [];
  const pluginsRoot = "✏️s/🔌️plugins";
  const breaches: BreachRecord[] = [];
  for (const entry of policyReaddirSafe(repoRoot, pluginsRoot)) {
    if (!entry.isDirectory) continue;
    const ownerRel = `${pluginsRoot}/${entry.name}`;
    const contractRel = `${ownerRel}/${pluginDir}`;
    if (!existsSync(join(repoRoot, contractRel))) {
      breaches.push({
        id: `plugin-root-missing-${ownerRel}`,
        summary: `"${ownerRel}" is missing required ${pluginDir}/ root contract folder`,
        kind: "taxonomy/plugin-root-shape",
        scope: ownerRel,
        priority: "high",
        reason: "Every plugin must expose general plugin code under 🔌️plugin/ via Plugin::builder.",
        solution: `Create ${contractRel}/🦀️component.rs plus ${children.map((c) => c + "/🦀️component.rs").join(", ")}.`,
      });
      continue;
    }
    if (!existsSync(join(repoRoot, contractRel, "🦀️component.rs"))) {
      breaches.push({
        id: `plugin-root-leaf-${contractRel}`,
        summary: `"${contractRel}" is missing 🦀️component.rs`,
        kind: "taxonomy/plugin-root-shape",
        scope: ownerRel,
        priority: "high",
        reason: "🔌️plugin/ must have a leaf component that returns Plugin via Plugin::builder.",
        solution: `Add ${contractRel}/🦀️component.rs exporting pub fn plugin() -> Plugin.`,
      });
    }
    for (const child of children) {
      const childLeaf = join(repoRoot, contractRel, child, "🦀️component.rs");
      if (!existsSync(childLeaf)) {
        breaches.push({
          id: `plugin-root-child-${contractRel}/${child}`,
          summary: `"${contractRel}" is missing ${child}/🦀️component.rs`,
          kind: "taxonomy/plugin-root-shape",
          scope: ownerRel,
          priority: "high",
          reason: "🔌️plugin/ required children: manifest, capabilities, setup, apps.",
          solution: `Add ${contractRel}/${child}/🦀️component.rs.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🏗️ Bans `semio_plugin!` and `PluginBundle::new` outside the SDK; prefers `Plugin::builder(`.
 * Medium until Wave 3 finishes migrating all plugins.
 */
function policyPluginBuilderBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const sdkRel = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
  for (const crate of crates) {
    if (crate.role !== "plugin" && crate.role !== "extension") continue;
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    if (crate.libRelPath.replaceAll("\\", "/").startsWith(sdkRel)) continue;
    const content = readFileSync(abs, "utf8");
    if (content.includes("semio_plugin!")) {
      breaches.push({
        id: `plugin-builder-macro-${crate.libRelPath}`,
        summary: `"${crate.libRelPath}" still uses semio_plugin! — migrate to Plugin::builder in 🔌️plugin/`,
        kind: "taxonomy/plugin-builder",
        scope: crate.pluginId || crate.ownerRel,
        priority: "high",
        reason: "semio_plugin! is retired; plugin identity lives under 🔌️plugin/ via typestate PluginBuilder.",
        solution: "Move registration into 🔌️plugin/🦀️component.rs using Plugin::builder(...).build() and call plugin_exports!(plugin::plugin).",
      });
    }
    if (content.includes("PluginBundle::new") || content.includes("PluginBundle {")) {
      breaches.push({
        id: `plugin-builder-bundle-${crate.libRelPath}`,
        summary: `"${crate.libRelPath}" still references PluginBundle — use Plugin::builder / Plugin::new`,
        kind: "taxonomy/plugin-builder",
        scope: crate.pluginId || crate.ownerRel,
        priority: "high",
        reason: "PluginBundle was renamed to Plugin; registration goes through Plugin::builder.",
        solution: "Replace PluginBundle with Plugin::builder(...).",
      });
    }
  }
  return breaches;
}

/**
 * 📏️Discovery-contract clause 4: no path segment (dir or file stem) or `mod`/`struct`/`enum`/`type`
 * identifier fragment named "protocol" may exist under a migrated plugin — `📡️spr` is the only accepted
 * name for that concept going forward. Scoped to `role = "plugin"` owners on purpose: the rename is a
 * plugin-tree contract today, while the framework's own `📡️protocol` kernel module is renamed by the
 * os-kernel merge (plan wave W8c) — flagging it here would report a rename that is scheduled, not late.
 */
function policySprNamingBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy" || crate.role !== "plugin") continue;
    const ownerRoot = crate.ownerRel;
    const pluginId = crate.pluginId;

    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir)) {
        const childRel = `${relDir}/${entry.name}`;
        const stem = policyStripEmoji(entry.name.replace(/\.rs$/, ""));
        if (POLICY_PROTOCOL_SEGMENT_RE.test(stem)) {
          breaches.push({
            id: `spr-naming-path-${childRel}`,
            summary: `"${childRel}" uses the retired "protocol" name — the taxonomy concept is "spr" (state patch representation)`,
            kind: "taxonomy/spr-naming",
            scope: pluginId,
            priority: "medium",
            reason: "Discovery contract: no 📡️protocol path segment may remain under a migrated plugin; 📡️spr is the only accepted name.",
            solution: `Rename "${entry.name}" to its 📡️spr equivalent.`,
          });
        }
        if (entry.isDirectory) walk(childRel);
      }
    };
    walk(ownerRoot);

    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const identRe = /\b(?:mod|struct|enum|type)\s+(\w*[Pp]rotocol\w*)\b/g;
    let m: RegExpExecArray | null;
    while ((m = identRe.exec(content))) {
      breaches.push({
        id: `spr-naming-ident-${crate.libRelPath}-${m.index}`,
        summary: `"${crate.libRelPath}" declares "${m[1]}" — identifiers naming the protocol concept must say "Spr"/"spr" instead`,
        kind: "taxonomy/spr-naming",
        scope: pluginId,
        line: policyLineOfIndex(content, m.index),
        priority: "medium",
        reason: "Discovery contract: no protocol-named identifier fragment tied to the app protocol module may remain under a migrated plugin.",
        solution: `Rename "${m[1]}" to its 📡️spr equivalent.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleTaxonomy

//#region 🔧️PolicyRuleHandcraftedSpecP3
/** ⚖️P3/M4 handcrafted-grammar policy scanners (distinctness / generic / declared-use / wiring / empty examples). Exemptions shrink to empty by P6 — see ticket HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT. */

const POLICY_HANDCRAFTED_SPEC_ROOTS = ["✏️s", "🧰️framework"] as const;
const POLICY_HANDCRAFTED_FACETS = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr", "🧬️mutations"] as const;
const POLICY_GRAMMAR_SPEC_LEAF = "📖️component.grammar.semio";
const POLICY_PROTOCOL_SPEC_LEAF = "📡️component.protocol.semio";
const POLICY_RS_COMPONENT_LEAF = "🦀️component.rs";
const POLICY_FAMILY_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family";

/** 🧹Normalizes grammar/protocol headers so distinctness compares body shape, not dialect names. */
function policyNormalizeSpecContent(content: string): string {
  return content
    .split(/\r?\n/)
    .map((line) => {
      if (/^(grammar|protocol|extension|schema)\s+\S+/.test(line)) {
        return line.replace(/^(grammar|protocol|extension|schema)\s+\S+/, "$1 __NAME__");
      }
      if (/^start\s+\S+/.test(line)) return "start __START__";
      return line;
    })
    .join("\n");
}

function policyHashSpecContent(content: string): string {
  return createHash("sha256").update(content).digest("hex");
}

/** 🔎️Walks `relRoots` for files matching `pred`, skipping `POLICY_SKIP_DIRS` / dotted dirs. */
function policyWalkRelFiles(repoRoot: string, relRoots: readonly string[], pred: (relPath: string, name: string) => boolean): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name) || ent.name.startsWith(".")) continue;
        walk(childRel);
        continue;
      }
      if (pred(childRel, ent.name)) found.push(childRel);
    }
  };
  for (const root of relRoots) walk(root);
  return found.sort();
}

function policyDiscoverGrammarAndProtocolSpecs(repoRoot: string): string[] {
  return policyWalkRelFiles(repoRoot, POLICY_HANDCRAFTED_SPEC_ROOTS, (_p, name) => name.endsWith(".grammar.semio") || name.endsWith(".protocol.semio"));
}

/**
 * 📏️Normalized-hash collision rule: two `.grammar.semio`/`.protocol.semio` under ✏️s/ or 🧰️framework/
 * must not share a body after dialect name/start normalization.
 */
function policySpecDistinctnessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const byHash = new Map<string, string[]>();
  for (const relPath of policyDiscoverGrammarAndProtocolSpecs(repoRoot)) {
    const raw = readFileSync(join(repoRoot, relPath), "utf8");
    const hash = policyHashSpecContent(policyNormalizeSpecContent(raw));
    const group = byHash.get(hash);
    if (group) group.push(relPath);
    else byHash.set(hash, [relPath]);
  }
  for (const [hash, group] of byHash) {
    if (group.length < 2) continue;
    const sorted = [...group].sort();
    for (let i = 0; i < sorted.length; i++) {
      for (let j = i + 1; j < sorted.length; j++) {
        const a = sorted[i]!;
        const b = sorted[j]!;
        if (POLICY_SPEC_DISTINCTNESS_EXEMPTIONS.has(a) || POLICY_SPEC_DISTINCTNESS_EXEMPTIONS.has(b)) continue;
        breaches.push({
          id: `spec-distinctness-${hash.slice(0, 12)}-${a}-${b}`,
          summary: `Normalized spec collision between "${a}" and "${b}"`,
          kind: "handcrafted-grammar/spec-distinctness",
          scope: a,
          priority: "high",
          reason: "Each artifact facet must own a distinct grammar/protocol body; shared stubs after name/start normalization are forbidden.",
          solution: `Handcraft distinct specs for both paths, then remove them from POLICY_SPEC_DISTINCTNESS_EXEMPTIONS.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️Flags catch-all generic grammar shapes under ✏️s/ (prop IDENT=, untyped list/map/value with prop,
 * bare IDENT assign* shells, json/blob/base64/payload field names).
 */
function policyGenericSpecBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const grammars = policyWalkRelFiles(repoRoot, ["✏️s"], (_p, name) => name.endsWith(".grammar.semio"));
  for (const relPath of grammars) {
    if (POLICY_GENERIC_SPEC_EXEMPTIONS.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const reasons: string[] = [];
    if (/prop\s*=\s*IDENT\s*"="\s*\(/.test(content) || /prop\s*=\s*IDENT\s*=/.test(content)) {
      reasons.push("catch-all prop = IDENT");
    }
    const untyped: string[] = [];
    for (const name of ["list", "map", "value"] as const) {
      if (new RegExp(`^\\s*${name}\\s*=`, "m").test(content)) untyped.push(name);
    }
    if (/\bprop\b/.test(content) && untyped.length > 0) {
      reasons.push(`untyped catch-all production(s) ${untyped.join(", ")} with prop`);
    }
    if (/=\s*IDENT\s+assign\*\s+block\?/.test(content) || /=\s*IDENT\s+assign\*/.test(content)) {
      reasons.push("bare IDENT assign* statement shell");
    }
    if (/-(json|blob|base64|payload)"/.test(content) || /-(json|blob|base64|payload)\b/.test(content)) {
      reasons.push("json/blob/base64/payload field name");
    }
    if (reasons.length === 0) continue;
    breaches.push({
      id: `generic-spec-${relPath}`,
      summary: `"${relPath}" still uses generic grammar shape: ${reasons.join("; ")}`,
      kind: "handcrafted-grammar/generic-spec",
      scope: relPath,
      priority: "high",
      reason: "Artifact grammars must be domain-true; catch-all prop/list/map/value and bare statement shells are migration leftovers.",
      solution: `Handcraft ${relPath}, then remove it from POLICY_GENERIC_SPEC_EXEMPTIONS.`,
    });
  }
  return breaches;
}

/** 🔎️Loads `📖️family-*.grammar.semio` keyed by `family-*` id. */
function policyLoadFamilyFragments(repoRoot: string): Map<string, { relPath: string; productions: readonly string[] }> {
  const out = new Map<string, { relPath: string; productions: readonly string[] }>();
  const files = policyWalkRelFiles(repoRoot, [POLICY_FAMILY_ROOT], (_p, name) => name.startsWith("📖️family-") && name.endsWith(".grammar.semio"));
  for (const relPath of files) {
    const leaf = relPath.split("/").pop()!;
    const id = leaf.replace(/^📖️/, "").replace(/\.grammar\.semio$/, "");
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const productions: string[] = [];
    for (const m of content.matchAll(/^([A-Za-z_][\w-]*)\s*=/gm)) {
      const name = m[1]!;
      if (name === "grammar" || name === "start" || name === "extension" || name === "dialect" || name === "use" || name === "schema" || name === "protocol") continue;
      productions.push(name);
    }
    out.set(id, { relPath, productions });
  }
  return out;
}

/**
 * 📏️Every `use family-X` must reference at least one production symbol from that family fragment.
 */
function policyDeclaredUseBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const families = policyLoadFamilyFragments(repoRoot);
  const grammars = policyWalkRelFiles(repoRoot, ["✏️s"], (_p, name) => name.endsWith(".grammar.semio"));
  for (const relPath of grammars) {
    if (POLICY_DECLARED_USE_EXEMPTIONS.has(relPath)) continue;
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    const uses = [...content.matchAll(/^use\s+(family-[\w-]+)/gm)].map((m) => m[1]!);
    if (uses.length === 0) continue;
    for (const fam of uses) {
      const frag = families.get(fam);
      if (!frag) {
        breaches.push({
          id: `declared-use-missing-${relPath}-${fam}`,
          summary: `"${relPath}" declares use ${fam} but no family fragment was found under ${POLICY_FAMILY_ROOT}`,
          kind: "handcrafted-grammar/declared-use",
          scope: relPath,
          priority: "high",
          reason: "Declared family imports must resolve to a 📖️family-*.grammar.semio fragment.",
          solution: `Add the family fragment or fix the use line, then remove ${relPath} from POLICY_DECLARED_USE_EXEMPTIONS if listed.`,
        });
        continue;
      }
      if (frag.productions.length === 0) continue;
      const referenced = frag.productions.some((prod) => new RegExp(`\b${prod.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\b`).test(content));
      if (referenced) continue;
      breaches.push({
        id: `declared-use-${relPath}-${fam}`,
        summary: `"${relPath}" declares use ${fam} but never references any production from ${frag.relPath}`,
        kind: "handcrafted-grammar/declared-use",
        scope: relPath,
        priority: "high",
        reason: "A use family-X line that never mentions a family production is dead wiring.",
        solution: `Reference at least one production from ${fam} in the artifact grammar, then remove ${relPath} from POLICY_DECLARED_USE_EXEMPTIONS.`,
      });
    }
  }
  return breaches;
}

function policyListPluginArtifactDirs(repoRoot: string): string[] {
  const out: string[] = [];
  const pluginsRoot = "✏️s/🔌️plugins";
  for (const plugin of policyReaddirSafe(repoRoot, pluginsRoot)) {
    if (!plugin.isDirectory) continue;
    const artifactsRel = `${pluginsRoot}/${plugin.name}/🗿️artifacts`;
    for (const art of policyReaddirSafe(repoRoot, artifactsRel)) {
      if (!art.isDirectory) continue;
      out.push(`${artifactsRel}/${art.name}`);
    }
  }
  return out.sort();
}

function policyArtifactHasRegisterLanguage(repoRoot: string, artRel: string): boolean {
  const rsFiles = policyWalkRelFiles(repoRoot, [artRel], (_p, name) => name.endsWith(".rs"));
  for (const rel of rsFiles) {
    if (readFileSync(join(repoRoot, rel), "utf8").includes("register_language")) return true;
  }
  return false;
}

/**
 * 📏️Facet `🦀️component.rs` must `include_str!` its sibling grammar/protocol spec; artifacts with any
 * facet specs must call `register_language` somewhere under the artifact (engine preferred).
 */
function policySpecWiringBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    let hasAnySpec = false;
    for (const facet of POLICY_HANDCRAFTED_FACETS) {
      const facetRel = `${artRel}/${facet}`;
      if (!existsSync(join(repoRoot, facetRel))) continue;
      const grammarRel = `${facetRel}/${POLICY_GRAMMAR_SPEC_LEAF}`;
      const protocolRel = `${facetRel}/${POLICY_PROTOCOL_SPEC_LEAF}`;
      const hasGrammar = existsSync(join(repoRoot, grammarRel));
      const hasProtocol = existsSync(join(repoRoot, protocolRel));
      if (!hasGrammar && !hasProtocol) continue;
      hasAnySpec = true;
      const rsRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF}`;
      if (!existsSync(join(repoRoot, rsRel))) continue;
      if (POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS.has(rsRel)) continue;
      const rsBody = readFileSync(join(repoRoot, rsRel), "utf8");
      const specs: string[] = [];
      if (hasGrammar) specs.push(POLICY_GRAMMAR_SPEC_LEAF);
      if (hasProtocol) specs.push(POLICY_PROTOCOL_SPEC_LEAF);
      for (const specLeaf of specs) {
        const wired = rsBody.includes("include_str!") && rsBody.includes(specLeaf);
        if (wired) continue;
        breaches.push({
          id: `spec-wiring-include-${rsRel}-${specLeaf}`,
          summary: `"${rsRel}" does not include_str! sibling spec "${specLeaf}"`,
          kind: "handcrafted-grammar/spec-wiring-include",
          scope: rsRel,
          priority: "high",
          reason: "Facet Rust components must embed their normative .grammar.semio/.protocol.semio via include_str!.",
          solution: `Add include_str!("…/${specLeaf}") to ${rsRel}, then remove it from POLICY_SPEC_WIRING_INCLUDE_EXEMPTIONS.`,
        });
      }
    }
    if (!hasAnySpec) continue;
    if (POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS.has(artRel)) continue;
    if (policyArtifactHasRegisterLanguage(repoRoot, artRel)) continue;
    breaches.push({
      id: `spec-wiring-register-${artRel}`,
      summary: `Artifact "${artRel}" has facet specs but never calls register_language`,
      kind: "handcrafted-grammar/spec-wiring-register",
      scope: artRel,
      priority: "high",
      reason: "Artifacts that ship grammar/protocol specs must register the language with the shared DSL host (typically in ⚙️engine/🦀️component.rs).",
      solution: `Call register_language from the artifact engine (or another artifact .rs), then remove ${artRel} from POLICY_SPEC_WIRING_REGISTER_EXEMPTIONS.`,
    });
  }
  return breaches;
}

/**
 * 📏️ Any asset under `🖼️assets/` inside `📚️examples` must carry real payload (size > 64), covering every
 * kind suffix (`.dsl.semio`, `.op.semio`, `.spr.semio`, `.pack.semio`, `.diff.semio`, `.cmd.semio`) and media.
 */
/**
 * 📏️Empty/stub `.semio` under `📚️examples/** /🖼️assets/` (all kinds) — size ≤ 64 is a breach.
 * `POLICY_EMPTY_EXAMPLE_EXEMPTIONS` stays empty.
 */
function policyEmptyExampleBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const assetsDir = taxonomy.exampleAssetsDirName;
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, [""], (relPath, name) => {
    if (!name.endsWith(".semio")) return false;
    const norm = relPath.replaceAll("\\", "/");
    if (!(norm.includes("/📚️examples/") || norm.startsWith("📚️examples/"))) return false;
    return norm.includes(`/${assetsDir}/`);
  });
  for (const relPath of files) {
    if (POLICY_EMPTY_EXAMPLE_EXEMPTIONS.has(relPath)) continue;
    let size = 0;
    try {
      size = statSync(join(repoRoot, relPath)).size;
    } catch {
      continue;
    }
    if (size > 64) continue;
    breaches.push({
      id: `empty-example-${relPath}`,
      summary: `"${relPath}" is an empty/envelope-only example asset (${size} bytes ≤ 64)`,
      kind: "handcrafted-grammar/empty-example",
      scope: relPath,
      priority: "high",
      reason: "Example `.semio` assets under 🖼️assets/ must include a real payload, not just an empty envelope or stub.",
      solution: `Seed a non-empty example at ${relPath}, then remove it from POLICY_EMPTY_EXAMPLE_EXEMPTIONS.`,
    });
  }
  return breaches;
}

function policyGenericCodecDeriveBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, ["✏️s/🔌️plugins"], (relPath, name) => {
    return relPath.includes("/🗿️artifacts/") && name.endsWith(".rs");
  });
  const banned = [
    { re: /dsl::__rt::parse_document_record|dsl::__rt::print_document_record|dsl::__rt::parse_inline_record|dsl::__rt::print_inline_record/g, label: "__rt codec wrapper" },
    { re: /dsl::op_rt::|store::op_rt::/g, label: "op_rt generic OpBinary" },
  ];
  for (const relPath of files) {
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    for (const { re, label } of banned) {
      re.lastIndex = 0;
      let match: RegExpExecArray | null;
      while ((match = re.exec(content)) !== null) {
        const before = content.slice(0, match.index);
        const line = before.split(/\r?\n/).length;
        breaches.push({
          id: `generic-codec-runtime-${relPath}-${line}`,
          summary: `Residual generic codec path (${label}) in "${relPath}" (line ${line})`,
          kind: "handcrafted-grammar/generic-codec-derive",
          scope: relPath,
          line,
          priority: "high",
          reason: "P6 deleted derive-emitted DocumentDsl/OpText/DocumentPack/OpBinary and their __rt/op_rt entrypoints; artifacts must use handcrafted codecs.",
          solution: `Replace ${label} usage in ${relPath} with the artifact's handcrafted DocumentDsl/OpText/DocumentPack/OpBinary impl.`,
        });
      }
    }
  }
  return breaches;
}

/** 🔗 Cumulative `#[path]` targets declared by one `📦️glue.rs` (absolute resolved paths). */
function policyCollectGluePathTargets(glueAbs: string): Set<string> {
  const declared = new Set<string>();
  if (!existsSync(glueAbs)) return declared;
  const libDir = dirname(glueAbs);
  const libText = readFileSync(glueAbs, "utf8");
  const baseStack: string[] = [libDir];
  let pendingPath: string | null = null;
  for (const rawLine of libText.split(/\r?\n/)) {
    const line = rawLine.trim();
    const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
    if (pathMatch) {
      pendingPath = pathMatch[1] ?? null;
      continue;
    }
    const modMatch = line.match(/^(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)/);
    if (modMatch) {
      const modName = modMatch[1]!;
      const base = baseStack[baseStack.length - 1] ?? libDir;
      let resolved: string;
      if (pendingPath === null) {
        resolved = join(base, modName);
      } else if (pendingPath === ".") {
        resolved = base;
      } else {
        resolved = join(base, pendingPath);
      }
      pendingPath = null;
      const asFile = resolved.endsWith(".rs") ? resolved : `${resolved}.rs`;
      const asModFile = join(resolved, "mod.rs");
      if (existsSync(asFile)) declared.add(resolve(asFile));
      else if (existsSync(asModFile)) declared.add(resolve(asModFile));
      else declared.add(resolve(asFile));
      if (line.includes("{")) baseStack.push(resolved.endsWith(".rs") ? dirname(resolved) : resolved);
      continue;
    }
    pendingPath = null;
    const opens = (line.match(/\{/g) ?? []).length;
    const closes = (line.match(/\}/g) ?? []).length;
    for (let i = 0; i < opens; i++) baseStack.push(baseStack[baseStack.length - 1] ?? libDir);
    for (let i = 0; i < closes; i++) {
      if (baseStack.length > 1) baseStack.pop();
    }
  }
  return declared;
}

/**
 * ☠️ Any `.rs` under `📚️examples` must be reachable via `#[path]` from a `📦️glue.rs` — dead definition
 * or test shims are forbidden.
 */
function policyCollectGluePathTargets(glueAbs: string): Set<string> {
  const declared = new Set<string>();
  if (!existsSync(glueAbs)) return declared;
  const libDir = dirname(glueAbs);
  const libText = readFileSync(glueAbs, "utf8");
  const baseStack: string[] = [libDir];
  let pendingPath: string | null = null;
  for (const rawLine of libText.split(/\r?\n/)) {
    const line = rawLine.trim();
    const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
    if (pathMatch) {
      pendingPath = pathMatch[1] ?? null;
      continue;
    }
    const modMatch = line.match(/^(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)/);
    if (modMatch) {
      const modName = modMatch[1]!;
      const base = baseStack[baseStack.length - 1] ?? libDir;
      let resolved: string;
      if (pendingPath === null) {
        resolved = join(base, modName);
      } else if (pendingPath === ".") {
        resolved = base;
      } else {
        resolved = join(base, pendingPath);
      }
      pendingPath = null;
      const asFile = resolved.endsWith(".rs") ? resolved : `${resolved}.rs`;
      const asModFile = join(resolved, "mod.rs");
      if (existsSync(asFile)) declared.add(resolve(asFile));
      else if (existsSync(asModFile)) declared.add(resolve(asModFile));
      else declared.add(resolve(asFile));
      if (line.includes("{")) baseStack.push(resolved.endsWith(".rs") ? dirname(resolved) : resolved);
      continue;
    }
    pendingPath = null;
    const opens = (line.match(/\{/g) ?? []).length;
    const closes = (line.match(/\}/g) ?? []).length;
    for (let i = 0; i < opens; i++) baseStack.push(baseStack[baseStack.length - 1] ?? libDir);
    for (let i = 0; i < closes; i++) {
      if (baseStack.length > 1) baseStack.pop();
    }
  }
  return declared;
}

function policyDeadExampleLeafBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const reachable = new Set<string>();
  const owners = [...new Set(crates.filter((crate) => crate.shape === "taxonomy").map((crate) => crate.ownerRel).filter(Boolean))];
  const glueRoots = owners.length > 0 ? owners : ["✏️s/🔌️plugins"];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    for (const target of policyCollectGluePathTargets(join(repoRoot, crate.libRelPath))) reachable.add(target);
  }
  for (const glueRel of policyWalkRelFiles(repoRoot, glueRoots, (_p, name) => name === "📦️glue.rs")) {
    for (const target of policyCollectGluePathTargets(join(repoRoot, glueRel))) reachable.add(target);
  }
  const fw = readdirSync(repoRoot).find((name) => name.endsWith("framework"));
  const exampleRoots = owners.length > 0 ? owners : fw ? ["✏️s/🔌️plugins", fw] : ["✏️s/🔌️plugins"];
  const exampleRs = policyWalkRelFiles(repoRoot, exampleRoots, (relPath, name) => {
    if (!name.endsWith(".rs")) return false;
    return relPath.replaceAll("\\", "/").includes("/📚️examples/");
  });
  for (const relPath of exampleRs) {
    const abs = resolve(join(repoRoot, relPath));
    if (reachable.has(abs)) continue;
    breaches.push({
      id: `dead-example-leaf-${relPath}`,
      summary: `"${relPath}" is not reachable via #[path] from any 📦️glue.rs`,
      kind: "taxonomy/dead-example-leaf",
      scope: relPath,
      priority: "high",
      reason: "Every .rs under 📚️examples must be wired from the plugin 📦️glue.rs (definition leaf or cfg(test) test leaf).",
      solution: `Add a #[path] mod declaration for ${relPath} under //#region 📚️Examples in the owning 📦️glue.rs, or delete the dead file.`,
    });
  }
  return breaches;
}

/** ⚖️Aggregates all P3/M4 handcrafted-grammar high-priority scanners for policy + verify gate. */
function policyHandcraftedSpecP3Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policySpecDistinctnessBreaches(repoRoot),
    ...policyGenericSpecBreaches(repoRoot),
    ...policyDeclaredUseBreaches(repoRoot),
    ...policySpecWiringBreaches(repoRoot),
    ...policyEmptyExampleBreaches(repoRoot),
    ...policyGenericCodecDeriveBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleHandcraftedSpecP3

//#region 🔧️PolicyRuleMutationArtifactEngines
/**
 * 🧬️Wave 2b mutation / artifact-engine scanners (OPERATIONS-TO-MUTATIONS).
 * Missing `🧬️mutations` / triad / `⚙️engine` / `start mutation` report as breaches so Wave 2+ can track
 * unmigrated artifacts; dispatch-enum coverage stays a deliberate placeholder until Wave 3 pilot lands.
 */

/** 🏷️Leading emoji prefix of a taxonomy dir name (everything before the ASCII stem). */
function policyLeadingEmojiPrefix(name: string): string {
  const ascii = policyStripEmoji(name);
  if (!ascii) return name;
  const idx = name.indexOf(ascii);
  return idx > 0 ? name.slice(0, idx) : "";
}

/** 🔎️Mutation-specific dirs under `🧬️mutations/` (skips leaf files and reserved kind names). */
function policyListMutationDirs(repoRoot: string, mutationsRel: string): string[] {
  const reserved = new Set<string>([...POLICY_MUTATION_TRIAD_DIRS, "📚️examples"]);
  return policyReaddirSafe(repoRoot, mutationsRel)
    .filter((e) => e.isDirectory && !reserved.has(e.name) && !e.name.startsWith("."))
    .map((e) => e.name)
    .sort();
}

/**
 * 📏️Every artifact must own `🧬️mutations/`; each concrete mutation dir must carry the triad
 * `🦠️mutation` / `🔺️diff` / `↩️inverse` (leaves optional until fan-out — directory presence is the gate).
 */
function policyMutationTriadCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const mutationsRel = `${artRel}/${POLICY_MUTATIONS_FACET}`;
    if (!existsSync(join(repoRoot, mutationsRel))) {
      breaches.push({
        id: `mutation-facet-missing-${artRel}`,
        summary: `"${artRel}" is missing required ${POLICY_MUTATIONS_FACET}/ facet`,
        kind: "mutation-migration/triad-completeness",
        scope: artRel,
        priority: "high",
        reason: "Every artifact must decompose document changes into 🧬️mutations/<mut>/{🦠️mutation,🔺️diff,↩️inverse}.",
        solution: `Create ${mutationsRel}/ with a dispatch 🦀️component.rs and one triad dir per mutation.`,
      });
      continue;
    }
    const mutDirs = policyListMutationDirs(repoRoot, mutationsRel);
    if (mutDirs.length === 0) {
      breaches.push({
        id: `mutation-dirs-empty-${artRel}`,
        summary: `"${mutationsRel}" has no concrete mutation directories yet`,
        kind: "mutation-migration/triad-completeness",
        scope: artRel,
        priority: "medium",
        reason: "The 🧬️mutations facet must contain at least one emoji-prefixed mutation directory with the triad.",
        solution: `Add mutation dirs under ${mutationsRel}/ each with 🦠️mutation/, 🔺️diff/, and ↩️inverse/.`,
      });
      continue;
    }
    for (const mutName of mutDirs) {
      const mutRel = `${mutationsRel}/${mutName}`;
      for (const kind of POLICY_MUTATION_TRIAD_DIRS) {
        const kindRel = `${mutRel}/${kind}`;
        if (existsSync(join(repoRoot, kindRel))) continue;
        breaches.push({
          id: `mutation-triad-missing-${kindRel}`,
          summary: `"${mutRel}" is missing triad kind ${kind}/`,
          kind: "mutation-migration/triad-completeness",
          scope: artRel,
          priority: "high",
          reason: "Each concrete mutation must expose 🦠️mutation + 🔺️diff + ↩️inverse directories.",
          solution: `Create ${kindRel}/ with 🦀️component.rs (and 🟦️component.ts stub if needed).`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️When a `🦠️mutation/🦀️component.rs` exists, it should mention `impl`…`Mutation` (advisory while
 * Wave 3 pilot lands the first real impls).
 */
function policyMutationImplPresenceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const implPattern = /\bimpl\b[^\n{]*\bMutation\s*</;
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const mutationsRel = `${artRel}/${POLICY_MUTATIONS_FACET}`;
    if (!existsSync(join(repoRoot, mutationsRel))) continue;
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const rsRel = `${mutationsRel}/${mutName}/🦠️mutation/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const abs = join(repoRoot, rsRel);
      if (!existsSync(abs)) continue;
      const content = readFileSync(abs, "utf8");
      if (implPattern.test(content)) continue;
      breaches.push({
        id: `mutation-impl-missing-${rsRel}`,
        summary: `"${rsRel}" does not yet implement Mutation<…>`,
        kind: "mutation-migration/impl-presence",
        scope: artRel,
        priority: "medium",
        reason: "Each concrete mutation struct must implement Mutation<P> (or a helper the dispatch enum delegates to).",
        solution: `Add impl Mutation<Snapshot> for the mutation struct in ${rsRel}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Every artifact must own `⚙️engine/` and that engine must eventually implement `ArtifactEngine`
 * (folder missing = high; trait missing = medium until Wave 3/4 rewrite engines as state machines).
 */
function policyArtifactEnginePresenceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const engineImplPattern = /\bimpl\b[^\n{]*\bArtifactEngine\b/;
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const engineRel = `${artRel}/${POLICY_ENGINE_FACET}`;
    if (!existsSync(join(repoRoot, engineRel))) {
      breaches.push({
        id: `artifact-engine-folder-missing-${artRel}`,
        summary: `"${artRel}" is missing required ${POLICY_ENGINE_FACET}/ facet`,
        kind: "mutation-migration/artifact-engine",
        scope: artRel,
        priority: "high",
        reason: "Every artifact must expose an ArtifactEngine state machine under ⚙️engine/.",
        solution: `Create ${engineRel}/${POLICY_RS_COMPONENT_LEAF_NAME} implementing ArtifactEngine.`,
      });
      continue;
    }
    const rsFiles = policyWalkRelFiles(repoRoot, [engineRel], (_p, name) => name.endsWith(".rs"));
    const hasImpl = rsFiles.some((rel) => engineImplPattern.test(readFileSync(join(repoRoot, rel), "utf8")));
    if (hasImpl) continue;
    breaches.push({
      id: `artifact-engine-impl-missing-${artRel}`,
      summary: `"${engineRel}" has no impl ArtifactEngine yet`,
      kind: "mutation-migration/artifact-engine",
      scope: artRel,
      priority: "medium",
      reason: "⚙️engine must implement ArtifactEngine (UI-independent apply/inverse over Mutation).",
      solution: `Add impl ArtifactEngine for the artifact engine type under ${engineRel}.`,
    });
  }
  return breaches;
}

/**
 * 📏️Every `🔧️op/*.grammar.semio` must declare `start mutation` (production `mutation =` follows in fan-out).
 */
function policyOpGrammarStartMutationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const grammars = policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => {
    if (!name.endsWith(".grammar.semio")) return false;
    return relPath.replaceAll("\\", "/").includes(`/${POLICY_OP_FACET}/`);
  });
  for (const relPath of grammars) {
    const content = readFileSync(join(repoRoot, relPath), "utf8");
    if (/^start\s+mutation\b/m.test(content)) continue;
    const legacy = /^start\s+operation\b/m.test(content);
    breaches.push({
      id: `op-grammar-start-mutation-${relPath}`,
      summary: legacy
        ? `"${relPath}" still declares start operation — must be start mutation`
        : `"${relPath}" is missing start mutation`,
      kind: "mutation-migration/op-grammar-start",
      scope: relPath,
      priority: "high",
      reason: "The 🔧️op grammar start symbol is the Mutation production (`start mutation`); OpText/OpBinary brand stays.",
      solution: `Change the start line in ${relPath} to "start mutation" (and rename production operation = to mutation =).`,
    });
  }
  return breaches;
}

/**
 * 📏️Specific mutation directory emojis must be unique within one artifact's `🧬️mutations/` tree.
 */
function policyMutationEmojiUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const mutationsRel = `${artRel}/${POLICY_MUTATIONS_FACET}`;
    if (!existsSync(join(repoRoot, mutationsRel))) continue;
    const seen = new Map<string, string>();
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const emoji = policyLeadingEmojiPrefix(mutName);
      if (!emoji) {
        breaches.push({
          id: `mutation-emoji-missing-${mutationsRel}/${mutName}`,
          summary: `"${mutationsRel}/${mutName}" has no leading emoji prefix`,
          kind: "mutation-migration/emoji-uniqueness",
          scope: artRel,
          priority: "high",
          reason: "Each concrete mutation directory must pick a unique emoji within its artifact.",
          solution: `Rename ${mutName} to include a unique emoji prefix (e.g. ➕️objects-add).`,
        });
        continue;
      }
      const prev = seen.get(emoji);
      if (prev) {
        breaches.push({
          id: `mutation-emoji-dup-${artRel}-${emoji}-${mutName}`,
          summary: `"${mutationsRel}/${mutName}" reuses emoji "${emoji}" already used by "${prev}"`,
          kind: "mutation-migration/emoji-uniqueness",
          scope: artRel,
          priority: "high",
          reason: "Specific mutation dir emojis must be unique within an artifact.",
          solution: `Give ${mutName} a different emoji than ${prev}.`,
        });
        continue;
      }
      seen.set(emoji, mutName);
    }
  }
  return breaches;
}

/**
 * 📏️Placeholder: dispatch-enum variant coverage vs mutation dirs (Wave 3 pilot will flesh this out).
 * Kept as a real policy function so Wave 6 can tighten without inventing a new export slot.
 */
function policyMutationDispatchCoverageBreaches(_repoRoot: string): BreachRecord[] {
  return [];
}

/** ⚖️Aggregates Wave 2b mutation / ArtifactEngine / op-grammar scanners. */
function policyMutationArtifactEngineBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyMutationTriadCompletenessBreaches(repoRoot),
    ...policyMutationImplPresenceBreaches(repoRoot),
    ...policyArtifactEnginePresenceBreaches(repoRoot),
    ...policyOpGrammarStartMutationBreaches(repoRoot),
    ...policyMutationEmojiUniquenessBreaches(repoRoot),
    ...policyMutationDispatchCoverageBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleMutationArtifactEngines

//#region 🔧️PolicyRuleArtifactSchemas
/**
 * 🧬️Wave W2 artifact-schema facet scanners (ARTIFACT-SCHEMA-FACETS).
 * Three facets × five `schemaFormats` leaves must agree on canonical camelCase fields; extractors are the
 * compiler this design deliberately does not have. Nested `📸️snapshot` / `🔺️diff` children are recognized
 * via taxonomy `snapshotChildDirs` / `diffChildDirs` in `policyTaxonomyDirsBreaches`.
 */

/** 🪪Canonical field shape shared by every schemaFormats extractor. */
export type PolicySchemaFieldCardinality = "scalar" | "list" | "fixedList" | "map";

/** 🧬️One normalised field from a schema leaf. */
export type PolicySchemaFieldShape = {
  name: string;
  optional: boolean;
  cardinality: PolicySchemaFieldCardinality;
  scalar: string;
  state: string;
};

/** 📦Extracted top-level type + fields from one leaf. */
export type PolicySchemaLeafExtract = {
  typeName: string;
  fields: PolicySchemaFieldShape[];
};

/** 🧭️§2 facet paths relative to an artifact root. */
const POLICY_SCHEMA_FACET_RELS = ["🧬️schema", "📸️snapshot/🧬️schema", "🔺️diff/🧬️schema"] as const;

/** 🏷️§10 prefix table keyed by `policyStripEmoji(plugin)/policyStripEmoji(artifact)`. */
const POLICY_ARTIFACT_SCHEMA_PREFIXES: Readonly<Record<string, string>> = {
  "writer/writer": "Writer",
  "mathematical/mathematical": "Mathematical",
  "procedural/procedural2d": "Procedural2d",
  "procedural/procedural3d": "Procedural3d",
  "flow/flow": "Flow",
  "gis/gisterrain": "GisTerrain",
  "gis/gismap": "GisMap",
  "vcs/vcs": "Vcs",
  "animate/present": "Present",
  "shooting/shooting": "Shooting",
  "demonstrator/playground": "Playground",
  "sequence/sequence": "Sequence",
  "fem/2d": "Fem2d",
  "fem/3d": "Fem3d",
  "architect/program": "Program",
  "process/process3d": "Process3d",
  "lowpoly/lowpoly": "Lowpoly",
  "reasoning/wires": "Wires",
  "forms/forms": "Forms",
  "layout/layout": "Layout",
  "cad/cad": "Cad",
  "norm/iso16757": "Iso16757",
  "norm/vdi3805": "Vdi3805",
  "norm/din4108": "Din4108",
  "norm/din16798": "Din16798",
  "norm/en1990": "En1990",
  "norm/en1991": "En1991",
  "norm/en1992": "En1992",
  "norm/en1993": "En1993",
  "norm/en1994": "En1994",
  "norm/en1995": "En1995",
  "norm/en1996": "En1996",
  "norm/en1997": "En1997",
  "norm/en1998": "En1998",
  "norm/en1999": "En1999",
  "norm/din18599": "Din18599",
  "playbook/playbook": "Playbook",
  "imperative/imperative": "Imperative",
  "remodel/remodel": "Remodel",
  "energy/model": "EnergyModel",
  "trinity/rewrite": "Rewrite",
  "trinity/jack": "Jack",
  "dag/dag": "Dag",
  "draw/draw": "Draw",
  "raster/raster": "Raster",
  "note/note": "Note",
  "puzzle/2d": "Puzzle2d",
  "puzzle/5d": "Puzzle5d",
  "puzzle/3d": "Puzzle3d",
  "block/2d": "Block2d",
  "block/5d": "Block5d",
  "block/3d": "Block3d",
  "space/home": "SHome",
  "sourcing/curate": "Curate",
};

/** 🔤snake_case → camelCase canonical field name. */
function policySnakeToCamel(name: string): string {
  return name.replace(/_([a-z0-9])/g, (_, c: string) => c.toUpperCase());
}

/** 🔤Normalize state-class tokens to kebab (persistent / shared-ui / …). */
function policyCanonicalState(raw: string): string {
  return raw.trim().toLowerCase().replace(/_/g, "-");
}

/** 🔤Map language type tokens onto §6 canonical scalar ids when exact. */
function policyCanonicalScalar(raw: string): string {
  const t = raw.replace(/\s+/g, "").trim();
  const table: Record<string, string> = {
    String: "string",
    string: "string",
    bool: "bool",
    boolean: "bool",
    Boolean: "bool",
    i32: "int32",
    u32: "uint32",
    i64: "int64",
    f32: "float32",
    f64: "float64",
    Int: "int32",
    Float: "float64",
    bytes: "bytes",
    "Vec<u8>": "bytes",
  };
  return table[t] ?? t;
}

/** 🏷️§10 prefix for an artifact rel path, or null when the artifact is absent from the table. */
function policyArtifactSchemaPrefix(artRel: string): string | null {
  const parts = artRel.replaceAll("\\", "/").split("/");
  const artifactsIdx = parts.indexOf("🗿️artifacts");
  if (artifactsIdx < 1 || artifactsIdx + 1 >= parts.length) return null;
  const plugin = policyStripEmoji(parts[artifactsIdx - 1] ?? "");
  const artifact = policyStripEmoji(parts[artifactsIdx + 1] ?? "");
  return POLICY_ARTIFACT_SCHEMA_PREFIXES[`${plugin}/${artifact}`] ?? null;
}

/** 🏷️Expected type name for a facet path given prefix X. */
function policyExpectedSchemaTypeName(prefix: string, facetRel: string): string {
  if (facetRel === "🧬️schema") return `${prefix}Artifact`;
  if (facetRel === "📸️snapshot/🧬️schema") return `${prefix}Snapshot`;
  return `${prefix}Diff`;
}

/**
 * 🔎Locate the declaration a schema leaf is expected to carry, by name rather than by position.
 * Helper types may legally precede the facet type in a leaf, so scanning for the first declaration
 * would silently compare the wrong body; when `expected` is absent or undeclared the first
 * declaration is returned so the type-name-parity rule still reports the mismatch.
 */
function policyFindSchemaDeclaration(
  text: string,
  declRe: RegExp,
  expected: string | null,
): { typeName: string; bodyStart: number } | null {
  const re = new RegExp(declRe.source, declRe.flags.includes("g") ? declRe.flags : `${declRe.flags}g`);
  let first: { typeName: string; bodyStart: number } | null = null;
  let m: RegExpExecArray | null;
  while ((m = re.exec(text))) {
    const found = { typeName: m[1]!, bodyStart: m.index + m[0].length };
    first ??= found;
    if (expected && found.typeName === expected) return found;
  }
  return first;
}

/** 🏷️Expected type name for a facet path such as `…/🗿️artifacts/X/📸️snapshot/🧬️schema`. */
function policyExpectedSchemaTypeNameForFacetPath(facetAbs: string): string | null {
  const rel = facetAbs.replaceAll("\\", "/");
  const facetRel = [...POLICY_SCHEMA_FACET_RELS]
    .sort((a, b) => b.length - a.length)
    .find((f) => rel.endsWith(`/${f}`));
  if (!facetRel) return null;
  const prefix = policyArtifactSchemaPrefix(rel.slice(0, rel.length - facetRel.length - 1));
  return prefix ? policyExpectedSchemaTypeName(prefix, facetRel) : null;
}

/** 🧩Parse Rust type into optional/cardinality/scalar. */
function policyParseRustFieldType(typeText: string): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  let t = typeText.replace(/\s+/g, " ").trim();
  let optional = false;
  if (/^Option\s*</.test(t)) {
    optional = true;
    t = t.replace(/^Option\s*<\s*/, "").replace(/\s*>\s*$/, "");
  }
  const mapMatch = /^(?:BTreeMap|HashMap)\s*<\s*String\s*,\s*(.+)\s*>$/.exec(t);
  if (mapMatch) {
    return { optional, cardinality: "map", scalar: policyCanonicalScalar(mapMatch[1]!.trim()) };
  }
  const fixedMatch = /^\[\s*(.+?)\s*;\s*\d+\s*\]$/.exec(t);
  if (fixedMatch) {
    return { optional, cardinality: "fixedList", scalar: policyCanonicalScalar(fixedMatch[1]!.trim()) };
  }
  if (/^Vec\s*<\s*u8\s*>$/.test(t)) {
    return { optional, cardinality: "scalar", scalar: "bytes" };
  }
  const vecMatch = /^Vec\s*<\s*(.+)\s*>$/.exec(t);
  if (vecMatch) {
    return { optional, cardinality: "list", scalar: policyCanonicalScalar(vecMatch[1]!.trim()) };
  }
  return { optional, cardinality: "scalar", scalar: policyCanonicalScalar(t) };
}

/**
 * 🦀️Extract pub fields of the single top-level pub struct, reading state field attributes.
 */
export function policyExtractRustSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  const decl = policyFindSchemaDeclaration(text, /\bpub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
  if (!decl) return { typeName: "", fields: [] };
  const { typeName, bodyStart } = decl;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldStartRe = /(?:#\[state\(([^\)]*)\)\]\s*)?pub\s+([a-z][a-z0-9_]*)\s*:\s*/g;
  let m: RegExpExecArray | null;
  while ((m = fieldStartRe.exec(body))) {
    const stateRaw = (m[1] ?? "").trim();
    const snake = m[2]!;
    let typeStart = fieldStartRe.lastIndex;
    let depthAngle = 0;
    let depthSquare = 0;
    let j = typeStart;
    for (; j < body.length; j++) {
      const ch = body[j]!;
      if (ch === "<") depthAngle++;
      else if (ch === ">") depthAngle = Math.max(0, depthAngle - 1);
      else if (ch === "[") depthSquare++;
      else if (ch === "]") depthSquare = Math.max(0, depthSquare - 1);
      else if (ch === "," && depthAngle === 0 && depthSquare === 0) break;
      else if (ch === "}" && depthAngle === 0 && depthSquare === 0) break;
    }
    const typeText = body.slice(typeStart, j).trim();
    fieldStartRe.lastIndex = j;
    const parsed = policyParseRustFieldType(typeText);
    fields.push({
      name: policySnakeToCamel(snake),
      optional: parsed.optional,
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: stateRaw ? policyCanonicalState(stateRaw) : "",
    });
  }
  return { typeName, fields };
}

/** 🧩Parse a TypeScript property type into optional/cardinality/scalar. */
function policyParseTsFieldType(typeText: string, optionalMark: boolean): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  let t = typeText.replace(/\s+/g, " ").trim().replace(/;$/, "");
  const optional = optionalMark || t.endsWith("| undefined") || t.endsWith("| null");
  t = t.replace(/\s*\|\s*undefined$/, "").replace(/\s*\|\s*null$/, "").trim();
  const recordMatch = /^Record\s*<\s*string\s*,\s*(.+)\s*>$/.exec(t);
  if (recordMatch) {
    return { optional, cardinality: "map", scalar: policyCanonicalScalar(recordMatch[1]!.trim()) };
  }
  const tupleMatch = /^\[\s*(.+?)\s*(?:,\s*\1\s*)+\]$/.exec(t);
  if (tupleMatch && t.includes(",")) {
    const inner = tupleMatch[1]!.trim();
    return { optional, cardinality: "fixedList", scalar: policyCanonicalScalar(inner) };
  }
  const arrMatch = /^(?:Array\s*<\s*(.+)\s*>|(.+)\[\])$/.exec(t);
  if (arrMatch) {
    return { optional, cardinality: "list", scalar: policyCanonicalScalar((arrMatch[1] ?? arrMatch[2]!).trim()) };
  }
  return { optional, cardinality: "scalar", scalar: policyCanonicalScalar(t) };
}

/**
 * 🟦️Extract members of the single exported interface, reading the state JSDoc tag above each property.
 */
export function policyExtractTypescriptSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  const decl = policyFindSchemaDeclaration(text, /\bexport\s+interface\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
  if (!decl) return { typeName: "", fields: [] };
  const { typeName, bodyStart } = decl;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /(?:\/\*\*\s*@state\s+([a-z0-9_-]+)\s*\*\/\s*)?([A-Za-z_][A-Za-z0-9_]*)(\?)?\s*:\s*([^;]+);/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const parsed = policyParseTsFieldType(m[4]!.trim(), Boolean(m[3]));
    fields.push({
      name: m[2]!,
      optional: parsed.optional,
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: m[1] ? policyCanonicalState(m[1]) : "",
    });
  }
  return { typeName, fields };
}

/**
 * 🔗️Extract fields of the single GraphQL type, reading the state directive on each field.
 */
export function policyExtractGraphqlSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  const decl = policyFindSchemaDeclaration(text, /\btype\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
  if (!decl) return { typeName: "", fields: [] };
  const { typeName, bodyStart } = decl;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(\[[^\]]+\]|[A-Za-z_][A-Za-z0-9_]*)(!)?(?:\s*@state\s*\(\s*class\s*:\s*([A-Z_]+)\s*\))?/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const name = m[1]!;
    const typeTok = m[2]!;
    const required = Boolean(m[3]);
    const stateRaw = m[4] ?? "";
    let cardinality: PolicySchemaFieldCardinality = "scalar";
    let scalar = typeTok;
    const listMatch = /^\[\s*(.+?)\s*!?\s*\]$/.exec(typeTok);
    if (listMatch) {
      const inner = listMatch[1]!.replace(/!$/, "").trim();
      if (/Entry$/.test(inner)) {
        cardinality = "map";
        scalar = inner.replace(/Entry$/, "");
      } else {
        cardinality = "list";
        scalar = inner;
      }
    }
    fields.push({
      name,
      optional: !required,
      cardinality,
      scalar: policyCanonicalScalar(scalar),
      state: stateRaw ? policyCanonicalState(stateRaw) : "",
    });
  }
  return { typeName, fields };
}

/** 🧩Walk a JSON Schema property schema into cardinality + scalar. */
function policyParseJsonSchemaProperty(prop: Record<string, unknown>): Pick<PolicySchemaFieldShape, "cardinality" | "scalar"> {
  const typ = prop.type;
  if (typ === "array") {
    const minItems = prop.minItems;
    const maxItems = prop.maxItems;
    const items = prop.items as Record<string, unknown> | undefined;
    const scalar = items ? policyJsonSchemaScalar(items) : "unknown";
    if (typeof minItems === "number" && minItems === maxItems) {
      return { cardinality: "fixedList", scalar };
    }
    return { cardinality: "list", scalar };
  }
  if (typ === "object" && prop.additionalProperties != null && prop.additionalProperties !== false) {
    const add = prop.additionalProperties;
    const scalar = typeof add === "object" && add ? policyJsonSchemaScalar(add as Record<string, unknown>) : "unknown";
    return { cardinality: "map", scalar };
  }
  return { cardinality: "scalar", scalar: policyJsonSchemaScalar(prop) };
}

/** 🔤Canonical scalar id from a JSON Schema schema object. */
function policyJsonSchemaScalar(schema: Record<string, unknown>): string {
  if (schema.contentEncoding === "base64") return "bytes";
  if (schema.contentMediaType === "application/json") return "string";
  const typ = schema.type;
  const format = schema.format;
  if (typ === "string") return "string";
  if (typ === "boolean") return "bool";
  if (typ === "integer") {
    if (format === "int32") return "int32";
    if (format === "uint32") return "uint32";
    if (format === "int64") return "int64";
    return "int32";
  }
  if (typ === "number") {
    if (format === "float") return "float32";
    if (format === "double") return "float64";
    return "float64";
  }
  if (typeof schema.$ref === "string") {
    const ref = schema.$ref as string;
    return ref.split("/").pop() ?? ref;
  }
  if (typeof schema.title === "string") return schema.title;
  return typeof typ === "string" ? typ : "unknown";
}

/**
 * 🔣️Extract properties, required, and x-semio-state from the normative JSON Schema leaf.
 */
export function policyExtractJsonSchemaFields(text: string): PolicySchemaLeafExtract {
  let doc: Record<string, unknown>;
  try {
    doc = JSON.parse(text) as Record<string, unknown>;
  } catch {
    return { typeName: "", fields: [] };
  }
  const typeName = typeof doc.title === "string" ? doc.title : "";
  const properties = (doc.properties ?? {}) as Record<string, Record<string, unknown>>;
  const required = new Set<string>(Array.isArray(doc.required) ? (doc.required as string[]) : []);
  const fields: PolicySchemaFieldShape[] = [];
  for (const [name, prop] of Object.entries(properties)) {
    const parsed = policyParseJsonSchemaProperty(prop ?? {});
    const stateRaw = typeof prop?.["x-semio-state"] === "string" ? (prop["x-semio-state"] as string) : "";
    fields.push({
      name,
      optional: !required.has(name),
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: stateRaw ? policyCanonicalState(stateRaw) : "",
    });
  }
  return { typeName, fields };
}

/** 🧩Parse a protobuf field type into optional/cardinality/scalar. */
function policyParseProtoFieldType(
  typeText: string,
  optionalKw: boolean,
  repeatedKw: boolean,
): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  const mapMatch = /^map\s*<\s*string\s*,\s*(.+)\s*>$/.exec(typeText.trim());
  if (mapMatch) {
    return { optional: optionalKw, cardinality: "map", scalar: policyCanonicalScalar(mapMatch[1]!.trim()) };
  }
  if (repeatedKw) {
    return { optional: optionalKw, cardinality: "list", scalar: policyCanonicalScalar(typeText.trim()) };
  }
  return { optional: optionalKw, cardinality: "scalar", scalar: policyCanonicalScalar(typeText.trim()) };
}

/**
 * 🛰️Extract fields of the single protobuf message, reading leading state comments on each field.
 */
export function policyExtractProtobufSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  const decl = policyFindSchemaDeclaration(text, /\bmessage\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
  if (!decl) return { typeName: "", fields: [] };
  const { typeName, bodyStart } = decl;
  let depth = 1;
  let i = bodyStart;
  for (; i < text.length; i++) {
    const ch = text[i];
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) break;
    }
  }
  const body = text.slice(bodyStart, i);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldRe = /(?:\/\/\s*@state\s+([a-z0-9_-]+)\s*\n\s*)?(optional\s+)?(repeated\s+)?(map\s*<\s*string\s*,\s*[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/g;
  let m: RegExpExecArray | null;
  while ((m = fieldRe.exec(body))) {
    const parsed = policyParseProtoFieldType(m[4]!, Boolean(m[2]), Boolean(m[3]));
    fields.push({
      name: policySnakeToCamel(m[5]!),
      optional: parsed.optional,
      cardinality: parsed.cardinality,
      scalar: parsed.scalar,
      state: m[1] ? policyCanonicalState(m[1]) : "",
    });
  }
  return { typeName, fields };
}

/** 🗂️Load every schemaFormats leaf for one facet; returns null entries for missing files. */
function policyLoadSchemaFacetLeaves(
  repoRoot: string,
  facetRel: string,
): { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] {
  const taxonomy = loadTaxonomy();
  const formats = taxonomy.schemaFormats ?? {};
  const expected = policyExpectedSchemaTypeNameForFacetPath(facetRel);
  const out: { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] = [];
  for (const [formatId, format] of Object.entries(formats)) {
    const leafFilename = format.leafFilename;
    const relPath = `${facetRel}/${leafFilename}`;
    const abs = join(repoRoot, relPath);
    if (!existsSync(abs)) {
      out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, extract: null });
      continue;
    }
    const text = readFileSync(abs, "utf8");
    let extract: PolicySchemaLeafExtract;
    switch (formatId) {
      case "🦀️rust":
        extract = policyExtractRustSchemaFields(text, expected);
        break;
      case "🟦️typescript":
        extract = policyExtractTypescriptSchemaFields(text, expected);
        break;
      case "🔗️graphql":
        extract = policyExtractGraphqlSchemaFields(text, expected);
        break;
      case "🔣️jsonschema":
        extract = policyExtractJsonSchemaFields(text);
        break;
      case "🛰️protobuf":
        extract = policyExtractProtobufSchemaFields(text, expected);
        break;
      default:
        extract = { typeName: "", fields: [] };
        break;
    }
    out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, extract });
  }
  return out;
}

/**
 * 📏️Facet completeness + normative leaf: all three facet dirs, each with every schemaFormats leaf
 * and the `artifactSchemaSpecFilenames` normative JSON Schema leaf.
 */
function policyArtifactSchemaFacetCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const formats = Object.entries(taxonomy.schemaFormats ?? {});
  const normativeByFacet = taxonomy.artifactSchemaSpecFilenames ?? {};
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (!existsSync(join(repoRoot, facetAbs))) {
        breaches.push({
          id: `artifact-schema-facet-missing-${facetAbs}`,
          summary: `"${artRel}" is missing required schema facet ${facetRel}/`,
          kind: "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: "Every artifact must expose 🧬️schema, 📸️snapshot/🧬️schema, and 🔺️diff/🧬️schema facets.",
          solution: `Create ${facetAbs}/ with all five schemaFormats leaves (and the normative 🔣️component.json).`,
        });
        continue;
      }
      for (const [formatId, format] of formats) {
        const leafRel = `${facetAbs}/${format.leafFilename}`;
        if (existsSync(join(repoRoot, leafRel))) continue;
        breaches.push({
          id: `artifact-schema-leaf-missing-${leafRel}`,
          summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
          kind: "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: "Each schema facet must carry every schemaFormats leaf filename from 🔣️taxonomy.json.",
          solution: `Add handcrafted ${leafRel}.`,
        });
      }
      const normative = normativeByFacet[facetRel];
      if (normative) {
        const normativeRel = `${facetAbs}/${normative}`;
        if (!existsSync(join(repoRoot, normativeRel))) {
          breaches.push({
            id: `artifact-schema-normative-missing-${normativeRel}`,
            summary: `"${facetAbs}" is missing normative artifactSchemaSpecFilenames leaf ${normative}`,
            kind: "artifact-schema/normative-leaf",
            scope: artRel,
            priority: "high",
            reason: "Within a facet the 🔣️component.json JSON Schema leaf is normative; the other four mirror it.",
            solution: `Add ${normativeRel} as the source of truth for this facet's fields.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️Field parity: all five leaves of one facet declare the identical canonical field set with identical
 * optionality and cardinality; JSON Schema is the truth when others disagree. Optionality of a `map`
 * field is exempt for protobuf only, because proto3 rejects an `optional` label on a map entry field
 * and therefore cannot express presence for it at all.
 * @see https://protobuf.dev/programming-guides/proto3/#maps
 */
function policyArtifactSchemaFieldParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const leaves = policyLoadSchemaFacetLeaves(repoRoot, facetAbs);
      if (leaves.some((l) => l.extract === null)) continue;
      const jsonLeaf = leaves.find((l) => l.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      const truth = new Map(jsonLeaf.extract.fields.map((f) => [f.name, f]));
      for (const leaf of leaves) {
        if (leaf.formatId === "🔣️jsonschema" || !leaf.extract) continue;
        const seen = new Map(leaf.extract.fields.map((f) => [f.name, f]));
        for (const [name, truthField] of truth) {
          const other = seen.get(name);
          if (!other) {
            breaches.push({
              id: `artifact-schema-field-parity-missing-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" is missing field "${name}" present in normative JSON Schema`,
              kind: "artifact-schema/field-parity",
              scope: artRel,
              priority: "high",
              reason: `Field parity requires identical canonical fields across all five leaves; JSON Schema is normative (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
              solution: `Add field "${name}" to ${leaf.relPath} matching ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}, scalar=${truthField.scalar}).`,
            });
            continue;
          }
          const optionalityComparable = !(leaf.formatId === "🛰️protobuf" && truthField.cardinality === "map");
          const cardinalityComparable = !(
            truthField.cardinality === "fixedList"
            && other.cardinality === "list"
            && (leaf.formatId === "🟦️typescript" || leaf.formatId === "🔗️graphql" || leaf.formatId === "🛰️protobuf")
          );
          if ((optionalityComparable && other.optional !== truthField.optional) || (cardinalityComparable && other.cardinality !== truthField.cardinality)) {
            breaches.push({
              id: `artifact-schema-field-parity-shape-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" field "${name}" disagrees with normative JSON Schema optionality/cardinality`,
              kind: "artifact-schema/field-parity",
              scope: artRel,
              priority: "high",
              reason: `Normative ${jsonLeaf.relPath} declares "${name}" as optional=${truthField.optional}, cardinality=${truthField.cardinality}; ${leaf.formatId} has optional=${other.optional}, cardinality=${other.cardinality}.`,
              solution: `Change "${name}" in ${leaf.relPath} to match ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
            });
          }
        }
        for (const name of seen.keys()) {
          if (truth.has(name)) continue;
          breaches.push({
            id: `artifact-schema-field-parity-extra-${leaf.relPath}-${name}`,
            summary: `"${leaf.relPath}" declares extra field "${name}" absent from normative JSON Schema`,
            kind: "artifact-schema/field-parity",
            scope: artRel,
            priority: "high",
            reason: `JSON Schema at ${jsonLeaf.relPath} is normative; extra fields in other formats break cross-format identity.`,
            solution: `Remove "${name}" from ${leaf.relPath}, or add it to ${jsonLeaf.relPath} if it is a real artifact field.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️State-class parity: snapshot facet fields equal exactly the persistent fields of the artifact facet.
 */
function policyArtifactSchemaStateParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const artifactFacet = `${artRel}/🧬️schema`;
    const snapshotFacet = `${artRel}/📸️snapshot/🧬️schema`;
    if (!existsSync(join(repoRoot, artifactFacet)) || !existsSync(join(repoRoot, snapshotFacet))) continue;
    const artLeaves = policyLoadSchemaFacetLeaves(repoRoot, artifactFacet);
    const snapLeaves = policyLoadSchemaFacetLeaves(repoRoot, snapshotFacet);
    const artJson = artLeaves.find((l) => l.formatId === "🔣️jsonschema")?.extract;
    const snapJson = snapLeaves.find((l) => l.formatId === "🔣️jsonschema")?.extract;
    if (!artJson || !snapJson) continue;
    const persistent = artJson.fields.filter((f) => f.state === "persistent");
    const snapMap = new Map(snapJson.fields.map((f) => [f.name, f]));
    const persMap = new Map(persistent.map((f) => [f.name, f]));
    for (const f of persistent) {
      const s = snapMap.get(f.name);
      if (!s) {
        breaches.push({
          id: `artifact-schema-state-parity-missing-${artRel}-${f.name}`,
          summary: `Snapshot facet is missing persistent artifact field "${f.name}"`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: "XSnapshot must equal exactly the persistent fields of XArtifact (equality, not subset).",
          solution: `Add "${f.name}" to ${snapshotFacet}/🔣️component.json (and the other four leaves) matching the artifact facet.`,
        });
        continue;
      }
      if (s.optional !== f.optional || s.cardinality !== f.cardinality) {
        breaches.push({
          id: `artifact-schema-state-parity-shape-${artRel}-${f.name}`,
          summary: `Snapshot field "${f.name}" shape differs from persistent artifact field`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: `Persistent artifact field "${f.name}" is optional=${f.optional}, cardinality=${f.cardinality}; snapshot has optional=${s.optional}, cardinality=${s.cardinality}.`,
          solution: `Align "${f.name}" in ${snapshotFacet}/🔣️component.json with ${artifactFacet}/🔣️component.json.`,
        });
      }
    }
    for (const name of snapMap.keys()) {
      if (persMap.has(name)) continue;
      breaches.push({
        id: `artifact-schema-state-parity-extra-${artRel}-${name}`,
        summary: `Snapshot facet has non-persistent field "${name}"`,
        kind: "artifact-schema/state-parity",
        scope: artRel,
        priority: "high",
        reason: "XSnapshot may only contain the persistent fields of XArtifact.",
        solution: `Remove "${name}" from ${snapshotFacet}/🔣️component.json, or mark it persistent on the artifact facet if it belongs there.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Diff coverage: every non-effect artifact field has a diff entry; no effect field does; `artifact` exists.
 */
function policyArtifactSchemaDiffCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const artifactFacet = `${artRel}/🧬️schema`;
    const diffFacet = `${artRel}/🔺️diff/🧬️schema`;
    if (!existsSync(join(repoRoot, artifactFacet)) || !existsSync(join(repoRoot, diffFacet))) continue;
    const artJson = policyLoadSchemaFacetLeaves(repoRoot, artifactFacet).find((l) => l.formatId === "🔣️jsonschema")?.extract;
    const diffJson = policyLoadSchemaFacetLeaves(repoRoot, diffFacet).find((l) => l.formatId === "🔣️jsonschema")?.extract;
    if (!artJson || !diffJson) continue;
    const diffNames = new Set(diffJson.fields.map((f) => f.name));
    if (!diffNames.has("artifact")) {
      breaches.push({
        id: `artifact-schema-diff-artifact-entry-${artRel}`,
        summary: `Diff facet is missing whole-replacement field "artifact"`,
        kind: "artifact-schema/diff-coverage",
        scope: artRel,
        priority: "high",
        reason: "XDiff must include `artifact: Option<Box<XArtifact>>` for whole-artifact replacement.",
        solution: `Add field "artifact" to ${diffFacet}/🔣️component.json (and the other four leaves).`,
      });
    }
    for (const f of artJson.fields) {
      if (f.state === "effect") {
        if (diffNames.has(f.name)) {
          breaches.push({
            id: `artifact-schema-diff-effect-${artRel}-${f.name}`,
            summary: `Diff facet must not cover effect field "${f.name}"`,
            kind: "artifact-schema/diff-coverage",
            scope: artRel,
            priority: "high",
            reason: "Effect fields are fire-and-forget and must not appear in XDiff.",
            solution: `Remove "${f.name}" from ${diffFacet}/🔣️component.json.`,
          });
        }
        continue;
      }
      if (!diffNames.has(f.name)) {
        breaches.push({
          id: `artifact-schema-diff-coverage-${artRel}-${f.name}`,
          summary: `Diff facet is missing entry for non-effect artifact field "${f.name}"`,
          kind: "artifact-schema/diff-coverage",
          scope: artRel,
          priority: "high",
          reason: "Every artifact field whose state class is not effect must have a same-named diff entry.",
          solution: `Add sparse diff entry "${f.name}" to ${diffFacet}/🔣️component.json matching §7.3 cardinality rules.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️Type-name parity: XArtifact / XSnapshot / XDiff spelled identically across all five leaves of their facet.
 */
function policyArtifactSchemaTypeNameParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const prefix = policyArtifactSchemaPrefix(artRel);
    if (!prefix) {
      breaches.push({
        id: `artifact-schema-prefix-unknown-${artRel}`,
        summary: `"${artRel}" has no §10 schema type prefix mapping`,
        kind: "artifact-schema/type-name-parity",
        scope: artRel,
        priority: "high",
        reason: "Type-name parity derives the expected XArtifact/XSnapshot/XDiff names from the normative §10 prefix table — never by guessing.",
        solution: `Add a prefix entry for this artifact to POLICY_ARTIFACT_SCHEMA_PREFIXES in 📜️script.ts (see normative-spec §10).`,
      });
      continue;
    }
    for (const facetRel of POLICY_SCHEMA_FACET_RELS) {
      const facetAbs = `${artRel}/${facetRel}`;
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const expected = policyExpectedSchemaTypeName(prefix, facetRel);
      const leaves = policyLoadSchemaFacetLeaves(repoRoot, facetAbs);
      for (const leaf of leaves) {
        if (!leaf.extract) continue;
        if (!leaf.extract.typeName) {
          breaches.push({
            id: `artifact-schema-type-name-missing-${leaf.relPath}`,
            summary: `"${leaf.relPath}" does not declare top-level type ${expected}`,
            kind: "artifact-schema/type-name-parity",
            scope: artRel,
            priority: "high",
            reason: `Every leaf of facet ${facetRel} must declare the same top-level type name ${expected}.`,
            solution: `Declare ${expected} as the top-level type in ${leaf.relPath}.`,
          });
          continue;
        }
        if (leaf.extract.typeName !== expected) {
          breaches.push({
            id: `artifact-schema-type-name-${leaf.relPath}`,
            summary: `"${leaf.relPath}" declares ${leaf.extract.typeName} but §10 expects ${expected}`,
            kind: "artifact-schema/type-name-parity",
            scope: artRel,
            priority: "high",
            reason: `Type-name parity requires ${expected} in all five leaves of ${facetRel} (prefix ${prefix} from §10).`,
            solution: `Rename the top-level type in ${leaf.relPath} to ${expected}.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️Pack relocation: no `🎒️pack` may sit directly under an artifact root (it lives under 📸️snapshot).
 */
function policyArtifactSchemaPackRelocationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const packRel = `${artRel}/🎒️pack`;
    if (!existsSync(join(repoRoot, packRel))) continue;
    breaches.push({
      id: `artifact-schema-pack-root-${artRel}`,
      summary: `"${packRel}" must move under 📸️snapshot/🎒️pack`,
      kind: "artifact-schema/pack-relocation",
      scope: artRel,
      priority: "high",
      reason: "A pack encodes exactly the snapshot; bare 🎒️pack on the artifact root is forbidden.",
      solution: `Move ${packRel}/ to ${artRel}/📸️snapshot/🎒️pack/ and update glue #[path] mounts.`,
    });
  }
  return breaches;
}

/** ⚖️Aggregates artifact-schema facet scanners (completeness, parity, coverage, pack relocation). */
export function policyArtifactSchemaBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyArtifactSchemaFacetCompletenessBreaches(repoRoot),
    ...policyArtifactSchemaFieldParityBreaches(repoRoot),
    ...policyArtifactSchemaStateParityBreaches(repoRoot),
    ...policyArtifactSchemaDiffCoverageBreaches(repoRoot),
    ...policyArtifactSchemaTypeNameParityBreaches(repoRoot),
    ...policyArtifactSchemaPackRelocationBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleArtifactSchemas

//#region 🔧️PolicyRuleAppSchemas
/**
 * 🧬️Wave A2 app-schema facet scanners (APP-SCHEMA-FACETS).
 * Two facets (config + presence) × five `schemaFormats` leaves; the five per-format extractors from
 * `PolicyRuleArtifactSchemas` are reused unchanged. Owners are derived from each app's
 * `type Config = …` binding — never a hand-maintained prefix table.
 */

/** 🎚️Canonical app config dir (level-slider). */
const POLICY_APP_CONFIG_DIR = "🎚️config";
/** 🧮Legacy abacus config dir — forbidden by `app-schema/config-relocation`. */
const POLICY_APP_CONFIG_LEGACY_DIR = "🧮️config";
/** 👥️App presence dir, sibling of the config owner. */
const POLICY_APP_PRESENCE_DIR = "👥️presence";
/** 🕸️Legacy wasm dir — forbidden by `app-schema/config-relocation`. */
const POLICY_APP_WASM_LEGACY_DIR = "🕸️wasm";
/** 🧬️Schema facet folder under a config or presence owner. */
const POLICY_APP_SCHEMA_FACET = "🧬️schema";

/** 🪪One discovered app-schema owner (deduped by owner path). */
export type PolicyAppSchemaOwner = {
  ownerRel: string;
  configType: string;
  presenceType: string;
  presenceRel: string;
  apps: string[];
};

/** 🏷️`XPresence` from `XConfig` by replacing the trailing `Config`. */
function policyAppPresenceTypeName(configType: string): string {
  return configType.endsWith("Config")
    ? `${configType.slice(0, -"Config".length)}Presence`
    : `${configType}Presence`;
}

/**
 * 🗂️Walk every plugin app `🦀️component.rs`, parse `type Config = XConfig;`, and resolve
 * the config owner dir (app `🎚️config`, else legacy `🧮️config`, else plugin-level `🎚️config` that
 * declares `pub struct XConfig`). Presence owner is the sibling `👥️presence` under the same parent.
 */
export function policyDiscoverAppSchemaOwners(repoRoot: string): PolicyAppSchemaOwner[] {
  const pluginsRoot = "✏️s/🔌️plugins";
  const byOwner = new Map<string, PolicyAppSchemaOwner>();
  for (const plugin of policyReaddirSafe(repoRoot, pluginsRoot)) {
    if (!plugin.isDirectory) continue;
    const appsRel = `${pluginsRoot}/${plugin.name}/🎛️apps`;
    for (const app of policyReaddirSafe(repoRoot, appsRel)) {
      if (!app.isDirectory) continue;
      const appRel = `${appsRel}/${app.name}`;
      const componentRel = `${appRel}/🦀️component.rs`;
      if (!existsSync(join(repoRoot, componentRel))) continue;
      const text = readFileSync(join(repoRoot, componentRel), "utf8");
      const m = /\btype\s+Config\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;/.exec(text);
      if (!m) continue;
      const configType = m[1]!;
      const sliderRel = `${appRel}/${POLICY_APP_CONFIG_DIR}`;
      const legacyRel = `${appRel}/${POLICY_APP_CONFIG_LEGACY_DIR}`;
      const pluginConfigRel = `${pluginsRoot}/${plugin.name}/${POLICY_APP_CONFIG_DIR}`;
      let ownerRel: string | null = null;
      if (existsSync(join(repoRoot, sliderRel))) {
        ownerRel = sliderRel;
      } else if (existsSync(join(repoRoot, legacyRel))) {
        ownerRel = legacyRel;
      } else {
        const pluginCfgRs = `${pluginConfigRel}/🦀️component.rs`;
        if (
          existsSync(join(repoRoot, pluginCfgRs)) &&
          new RegExp(`\\bpub\\s+struct\\s+${configType}\\b`).test(readFileSync(join(repoRoot, pluginCfgRs), "utf8"))
        ) {
          ownerRel = pluginConfigRel;
        }
      }
      if (!ownerRel) continue;
      const presenceType = policyAppPresenceTypeName(configType);
      const parentRel = ownerRel.split("/").slice(0, -1).join("/");
      const presenceRel = `${parentRel}/${POLICY_APP_PRESENCE_DIR}`;
      const appId = `${plugin.name}/${app.name}`;
      const existing = byOwner.get(ownerRel);
      if (existing) {
        existing.apps.push(appId);
        continue;
      }
      byOwner.set(ownerRel, { ownerRel, configType, presenceType, presenceRel, apps: [appId] });
    }
  }
  return [...byOwner.values()].sort((a, b) => a.ownerRel.localeCompare(b.ownerRel));
}

/**
 * 🗂️Load every schemaFormats leaf for one app facet; reuses the five artifact extractors unchanged,
 * selecting the declared type by `expectedTypeName` via `policyFindSchemaDeclaration`.
 */
function policyLoadAppSchemaFacetLeaves(
  repoRoot: string,
  facetAbs: string,
  expectedTypeName: string | null,
): { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] {
  const taxonomy = loadTaxonomy();
  const formats = taxonomy.schemaFormats ?? {};
  const out: { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] = [];
  for (const [formatId, format] of Object.entries(formats)) {
    const leafFilename = format.leafFilename;
    const relPath = `${facetAbs}/${leafFilename}`;
    const abs = join(repoRoot, relPath);
    if (!existsSync(abs)) {
      out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, extract: null });
      continue;
    }
    const text = readFileSync(abs, "utf8");
    let extract: PolicySchemaLeafExtract;
    switch (formatId) {
      case "🦀️rust":
        extract = policyExtractRustSchemaFields(text, expectedTypeName);
        break;
      case "🟦️typescript":
        extract = policyExtractTypescriptSchemaFields(text, expectedTypeName);
        break;
      case "🔗️graphql":
        extract = policyExtractGraphqlSchemaFields(text, expectedTypeName);
        break;
      case "🔣️jsonschema":
        extract = policyExtractJsonSchemaFields(text);
        break;
      case "🛰️protobuf":
        extract = policyExtractProtobufSchemaFields(text, expectedTypeName);
        break;
      default:
        extract = { typeName: "", fields: [] };
        break;
    }
    out.push({ formatId, leafFilename, fieldCasing: format.fieldCasing, relPath, extract });
  }
  return out;
}

/** 🧭️Taxonomy `appSchemaSpecFilenames` key for a config or presence facet. */
function policyAppSchemaFacetRole(kind: "config" | "presence"): string {
  return kind === "config"
    ? `${POLICY_APP_CONFIG_DIR}/${POLICY_APP_SCHEMA_FACET}`
    : `${POLICY_APP_PRESENCE_DIR}/${POLICY_APP_SCHEMA_FACET}`;
}

/**
 * 📏️Facet completeness + normative leaf: both config and presence schema facets, each with every
 * schemaFormats leaf and the `appSchemaSpecFilenames` normative JSON Schema leaf.
 */
function policyAppSchemaFacetCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const formats = Object.entries(taxonomy.schemaFormats ?? {});
  const normativeByFacet = taxonomy.appSchemaSpecFilenames ?? {};
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot)) {
    const facets: { kind: "config" | "presence"; facetAbs: string }[] = [
      { kind: "config", facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}` },
      { kind: "presence", facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}` },
    ];
    for (const { kind, facetAbs } of facets) {
      if (!existsSync(join(repoRoot, facetAbs))) {
        breaches.push({
          id: `app-schema-facet-missing-${facetAbs}`,
          summary: `"${owner.ownerRel}" is missing required ${kind} schema facet ${facetAbs}/`,
          kind: "app-schema/facet-completeness",
          scope: owner.ownerRel,
          priority: "high",
          reason: "Every app-schema owner must expose 🎚️config/🧬️schema and 👥️presence/🧬️schema facets.",
          solution: `Create ${facetAbs}/ with all five schemaFormats leaves (and the normative 🔣️component.json).`,
        });
        continue;
      }
      for (const [formatId, format] of formats) {
        const leafRel = `${facetAbs}/${format.leafFilename}`;
        if (existsSync(join(repoRoot, leafRel))) continue;
        breaches.push({
          id: `app-schema-leaf-missing-${leafRel}`,
          summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
          kind: "app-schema/facet-completeness",
          scope: owner.ownerRel,
          priority: "high",
          reason: "Each schema facet must carry every schemaFormats leaf filename from 🔣️taxonomy.json.",
          solution: `Add handcrafted ${leafRel}.`,
        });
      }
      const normative = normativeByFacet[policyAppSchemaFacetRole(kind)] ?? "🔣️component.json";
      const normativeRel = `${facetAbs}/${normative}`;
      if (!existsSync(join(repoRoot, normativeRel))) {
        breaches.push({
          id: `app-schema-normative-missing-${normativeRel}`,
          summary: `"${facetAbs}" is missing normative appSchemaSpecFilenames leaf ${normative}`,
          kind: "app-schema/facet-completeness",
          scope: owner.ownerRel,
          priority: "high",
          reason: "Within a facet the 🔣️component.json JSON Schema leaf is normative; the other four mirror it.",
          solution: `Add ${normativeRel} as the source of truth for this facet's fields.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️Field parity: all five leaves of one facet declare the identical canonical field set with identical
 * optionality and cardinality; JSON Schema is the truth when others disagree. Optionality of a `map`
 * field is exempt for protobuf only (proto3 rejects `optional` on a map entry field).
 * @see https://protobuf.dev/programming-guides/proto3/#maps
 */
function policyAppSchemaFieldParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot)) {
    const facets: { facetAbs: string; expectedTypeName: string }[] = [
      { facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`, expectedTypeName: owner.configType },
      { facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`, expectedTypeName: owner.presenceType },
    ];
    for (const { facetAbs, expectedTypeName } of facets) {
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const leaves = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, expectedTypeName);
      if (leaves.some((l) => l.extract === null)) continue;
      const jsonLeaf = leaves.find((l) => l.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      const truth = new Map(jsonLeaf.extract.fields.map((f) => [f.name, f]));
      for (const leaf of leaves) {
        if (leaf.formatId === "🔣️jsonschema" || !leaf.extract) continue;
        const seen = new Map(leaf.extract.fields.map((f) => [f.name, f]));
        for (const [name, truthField] of truth) {
          const other = seen.get(name);
          if (!other) {
            breaches.push({
              id: `app-schema-field-parity-missing-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" is missing field "${name}" present in normative JSON Schema`,
              kind: "app-schema/field-parity",
              scope: owner.ownerRel,
              priority: "high",
              reason: `Field parity requires identical canonical fields across all five leaves; JSON Schema is normative (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
              solution: `Add field "${name}" to ${leaf.relPath} matching ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}, scalar=${truthField.scalar}).`,
            });
            continue;
          }
          const optionalityComparable = !(leaf.formatId === "🛰️protobuf" && truthField.cardinality === "map");
          const cardinalityComparable = !(
            truthField.cardinality === "fixedList"
            && other.cardinality === "list"
            && (leaf.formatId === "🟦️typescript" || leaf.formatId === "🔗️graphql" || leaf.formatId === "🛰️protobuf")
          );
          if ((optionalityComparable && other.optional !== truthField.optional) || (cardinalityComparable && other.cardinality !== truthField.cardinality)) {
            breaches.push({
              id: `app-schema-field-parity-shape-${leaf.relPath}-${name}`,
              summary: `"${leaf.relPath}" field "${name}" disagrees with normative JSON Schema optionality/cardinality`,
              kind: "app-schema/field-parity",
              scope: owner.ownerRel,
              priority: "high",
              reason: `Normative ${jsonLeaf.relPath} declares "${name}" as optional=${truthField.optional}, cardinality=${truthField.cardinality}; ${leaf.formatId} has optional=${other.optional}, cardinality=${other.cardinality}.`,
              solution: `Change "${name}" in ${leaf.relPath} to match ${jsonLeaf.relPath} (optional=${truthField.optional}, cardinality=${truthField.cardinality}).`,
            });
          }
        }
        for (const name of seen.keys()) {
          if (truth.has(name)) continue;
          breaches.push({
            id: `app-schema-field-parity-extra-${leaf.relPath}-${name}`,
            summary: `"${leaf.relPath}" declares extra field "${name}" absent from normative JSON Schema`,
            kind: "app-schema/field-parity",
            scope: owner.ownerRel,
            priority: "high",
            reason: `JSON Schema at ${jsonLeaf.relPath} is normative; extra fields in other formats break cross-format identity.`,
            solution: `Remove "${name}" from ${leaf.relPath}, or add it to ${jsonLeaf.relPath} if it is a real app field.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️Config fidelity: the config facet's normative field set equals the fields of the owner's real
 * `XConfig` Rust struct in `🎚️config/🦀️component.rs` (or legacy `🧮️config`).
 */
function policyAppSchemaConfigFidelityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot)) {
    const cfgRs = `${owner.ownerRel}/🦀️component.rs`;
    const facetAbs = `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`;
    if (!existsSync(join(repoRoot, cfgRs)) || !existsSync(join(repoRoot, facetAbs))) continue;
    const real = policyExtractRustSchemaFields(readFileSync(join(repoRoot, cfgRs), "utf8"), owner.configType);
    const jsonLeaf = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, owner.configType).find((l) => l.formatId === "🔣️jsonschema");
    if (!jsonLeaf?.extract) continue;
    const truth = new Map(real.fields.map((f) => [f.name, f]));
    const seen = new Map(jsonLeaf.extract.fields.map((f) => [f.name, f]));
    for (const [name, realField] of truth) {
      const facetField = seen.get(name);
      if (!facetField) {
        breaches.push({
          id: `app-schema-config-fidelity-missing-${owner.ownerRel}-${name}`,
          summary: `Config facet is missing field "${name}" from real ${owner.configType}`,
          kind: "app-schema/config-fidelity",
          scope: owner.ownerRel,
          priority: "high",
          reason: `The config facet must document exactly the fields of ${owner.configType} in ${cfgRs}.`,
          solution: `Add "${name}" to ${jsonLeaf.relPath} (and the other four leaves) matching ${cfgRs} (optional=${realField.optional}, cardinality=${realField.cardinality}).`,
        });
        continue;
      }
      if (facetField.optional !== realField.optional || facetField.cardinality !== realField.cardinality) {
        breaches.push({
          id: `app-schema-config-fidelity-shape-${owner.ownerRel}-${name}`,
          summary: `Config facet field "${name}" disagrees with real ${owner.configType}`,
          kind: "app-schema/config-fidelity",
          scope: owner.ownerRel,
          priority: "high",
          reason: `Real ${owner.configType}.${name} is optional=${realField.optional}, cardinality=${realField.cardinality}; facet has optional=${facetField.optional}, cardinality=${facetField.cardinality}.`,
          solution: `Align "${name}" in ${jsonLeaf.relPath} with ${cfgRs}.`,
        });
      }
    }
    for (const name of seen.keys()) {
      if (truth.has(name)) continue;
      breaches.push({
        id: `app-schema-config-fidelity-extra-${owner.ownerRel}-${name}`,
        summary: `Config facet declares extra field "${name}" absent from real ${owner.configType}`,
        kind: "app-schema/config-fidelity",
        scope: owner.ownerRel,
        priority: "high",
        reason: `The config facet may not invent fields beyond ${owner.configType} in ${cfgRs}.`,
        solution: `Remove "${name}" from ${jsonLeaf.relPath}, or add it to ${cfgRs} if it belongs on the real struct.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️State purity: every config-facet field is `local-ui`; every presence-facet field is `shared-ui`.
 */
function policyAppSchemaStatePurityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot)) {
    const checks: { facetAbs: string; expectedState: string; expectedTypeName: string; label: string }[] = [
      {
        facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`,
        expectedState: "local-ui",
        expectedTypeName: owner.configType,
        label: "config",
      },
      {
        facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`,
        expectedState: "shared-ui",
        expectedTypeName: owner.presenceType,
        label: "presence",
      },
    ];
    for (const { facetAbs, expectedState, expectedTypeName, label } of checks) {
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const jsonLeaf = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, expectedTypeName).find((l) => l.formatId === "🔣️jsonschema");
      if (!jsonLeaf?.extract) continue;
      for (const field of jsonLeaf.extract.fields) {
        if (field.state === expectedState) continue;
        breaches.push({
          id: `app-schema-state-purity-${facetAbs}-${field.name}`,
          summary: `${label} facet field "${field.name}" must be ${expectedState} (got ${field.state || "missing"})`,
          kind: "app-schema/state-purity",
          scope: owner.ownerRel,
          priority: "high",
          reason: `App ${label} facet fields are by definition ${expectedState}; other state classes belong elsewhere.`,
          solution: `Set x-semio-state (and the matching per-format state annotation) for "${field.name}" in ${jsonLeaf.relPath} to ${expectedState}.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️Type-name parity: `XConfig` / `XPresence` spelled identically across all five leaves of their facet;
 * `XPresence` is derived from the owner's `type Config` binding (trailing `Config` → `Presence`).
 */
function policyAppSchemaTypeNameParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot)) {
    const facets: { facetAbs: string; expected: string }[] = [
      { facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`, expected: owner.configType },
      { facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`, expected: owner.presenceType },
    ];
    for (const { facetAbs, expected } of facets) {
      if (!existsSync(join(repoRoot, facetAbs))) continue;
      const leaves = policyLoadAppSchemaFacetLeaves(repoRoot, facetAbs, expected);
      for (const leaf of leaves) {
        if (!leaf.extract) continue;
        if (!leaf.extract.typeName) {
          breaches.push({
            id: `app-schema-type-name-missing-${leaf.relPath}`,
            summary: `"${leaf.relPath}" does not declare top-level type ${expected}`,
            kind: "app-schema/type-name-parity",
            scope: owner.ownerRel,
            priority: "high",
            reason: `Every leaf of this facet must declare the same top-level type name ${expected}.`,
            solution: `Declare ${expected} as the top-level type in ${leaf.relPath}.`,
          });
          continue;
        }
        if (leaf.extract.typeName !== expected) {
          breaches.push({
            id: `app-schema-type-name-${leaf.relPath}`,
            summary: `"${leaf.relPath}" declares ${leaf.extract.typeName} but expects ${expected}`,
            kind: "app-schema/type-name-parity",
            scope: owner.ownerRel,
            priority: "high",
            reason: `Type-name parity requires ${expected} in all five leaves (from the app's type Config binding).`,
            solution: `Rename the top-level type in ${leaf.relPath} to ${expected}.`,
          });
        }
      }
    }
  }
  return breaches;
}

/**
 * 📏️Config relocation: no `🧮️config` (abacus) and no `🕸️wasm` anywhere under `✏️s/🔌️plugins`.
 */
function policyAppSchemaConfigRelocationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const banned = new Set([POLICY_APP_CONFIG_LEGACY_DIR, POLICY_APP_WASM_LEGACY_DIR]);
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      if (!ent.isDirectory()) continue;
      if (POLICY_SKIP_DIRS.has(ent.name) || ent.name.startsWith(".")) continue;
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (banned.has(ent.name)) {
        const kindLabel = ent.name === POLICY_APP_CONFIG_LEGACY_DIR ? "legacy abacus config" : "legacy spider-web wasm";
        const replacement = ent.name === POLICY_APP_CONFIG_LEGACY_DIR ? POLICY_APP_CONFIG_DIR : "🌉️wasm";
        breaches.push({
          id: `app-schema-config-relocation-${childRel}`,
          summary: `"${childRel}" must move to ${replacement}`,
          kind: "app-schema/config-relocation",
          scope: childRel,
          priority: "high",
          reason: `${kindLabel} dirs are forbidden under ✏️s/🔌️plugins; consolidate onto the canonical emoji.`,
          solution: `Rename ${childRel}/ to use ${replacement}/ and update glue #[path] mounts.`,
        });
      }
      walk(childRel);
    }
  };
  walk("✏️s/🔌️plugins");
  return breaches;
}

/** ⚖️Aggregates app-schema facet scanners (completeness, parity, fidelity, purity, relocation). */
export function policyAppSchemaBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyAppSchemaFacetCompletenessBreaches(repoRoot),
    ...policyAppSchemaFieldParityBreaches(repoRoot),
    ...policyAppSchemaConfigFidelityBreaches(repoRoot),
    ...policyAppSchemaStatePurityBreaches(repoRoot),
    ...policyAppSchemaTypeNameParityBreaches(repoRoot),
    ...policyAppSchemaConfigRelocationBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleAppSchemas

//#region 🔧️PolicyRuleArtifactIo
/** ⚖️Closed MediaFormat catalog must match 📋️mimes.csv (ticket 26/08/10/ARTIFACT-IO-FACETS). */
const MEDIA_FORMAT_CATALOG_VARIANTS = [
  "glb", "gltf", "stl", "obj", "ply", "las", "step", "ifc", "dwg", "dxf", "svg", "png", "jpg", "gif",
  "bmp", "tiff", "pdf", "docx", "pptx", "csv", "xlsx", "md", "txt", "zip", "bcf", "json",
] as const;

function policyMediaFormatCatalogBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const mimesPath = join(repoRoot, "🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes.csv");
  // tolerate either list emoji spelling used historically
  const alt = join(repoRoot, "🧰️framework/🔨️modules/🖼️assets/📄️list/📋️mimes.csv");
  let csvPath = mimesPath;
  if (!existsSync(csvPath)) {
    const candidates = [
      join(repoRoot, "🧰️framework/🔨️modules/🖼️assets/📄️list/📋️mimes.csv"),
      join(repoRoot, "🧰️framework/🔨️modules/🖼️assets/📄️list/📋️mimes.csv"),
    ];
    // resolve via walk of assets list dir
    const assets = join(repoRoot, "🧰️framework/🔨️modules/🖼️assets");
    const found: string[] = [];
    const walk = (dir: string) => {
      if (!existsSync(dir)) return;
      for (const name of readdirSync(dir)) {
        const p = join(dir, name);
        if (statSync(p).isDirectory()) walk(p);
        else if (name === "📋️mimes.csv") found.push(p);
      }
    };
    walk(assets);
    if (found.length === 0) {
      breaches.push({
        kind: "artifact-io/catalog-parity",
        scope: "🧰️framework/🔨️modules/🖼️assets",
        summary: "📋️mimes.csv missing",
        reason: "MediaFormat catalog CSV is the single source of truth.",
        solution: "Restore 📋️mimes.csv under 🖼️assets.",
        autofixable: false,
      } as BreachRecord);
      return breaches;
    }
    csvPath = found[0]!;
  }
  const lines = readFileSync(csvPath, "utf8").split(/\r?\n/).filter((l) => l.trim() && !l.startsWith("MIME"));
  const csvExts = new Set(
    lines
      .map((l) => l.split(",")[1]?.trim().replace(/^\./, "").toLowerCase())
      .filter((x): x is string => !!x),
  );
  // .stp is an alias for step — not a catalog row
  for (const v of MEDIA_FORMAT_CATALOG_VARIANTS) {
    if (!csvExts.has(v)) {
      breaches.push({
        kind: "artifact-io/catalog-parity",
        scope: csvPath,
        summary: `MediaFormat.${v} missing from 📋️mimes.csv`,
        reason: "Every MediaFormat variant must appear as an Extension in the CSV.",
        solution: `Add a .${v} row to �♣️mimes.csv or remove the enum variant.`,
        autofixable: false,
      } as BreachRecord);
    }
  }
  for (const ext of csvExts) {
    if (!(MEDIA_FORMAT_CATALOG_VARIANTS as readonly string[]).includes(ext) && ext !== "stp") {
      breaches.push({
        kind: "artifact-io/catalog-parity",
        scope: csvPath,
        summary: `CSV extension .${ext} has no MediaFormat variant`,
        reason: "CSV and MediaFormat must stay closed and equal.",
        solution: `Add MediaFormat::${ext[0]!.toUpperCase()}${ext.slice(1)} or remove the CSV row.`,
        autofixable: false,
      } as BreachRecord);
    }
  }
  return breaches;
}

/** ⚖️Artifact 🚪️io facet scanners (catalog parity + facet completeness + leaf coverage). */
export function policyArtifactIoBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyMediaFormatCatalogBreaches(repoRoot),
    ...policyArtifactIoFacetCompletenessBreaches(repoRoot),
    ...policyArtifactIoLeafParityBreaches(repoRoot),
    ...policyArtifactIoNoEngineIoBreaches(repoRoot),
  ];
}

function policyArtifactIoFacetCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot)) {
    const artifactsRoot = join(pluginsRoot, plugin, "🗿️artifacts");
    if (!existsSync(artifactsRoot) || !statSync(artifactsRoot).isDirectory()) continue;
    for (const artifact of readdirSync(artifactsRoot)) {
      const artifactDir = join(artifactsRoot, artifact);
      if (!statSync(artifactDir).isDirectory()) continue;
      const ioDir = join(artifactDir, "🚪️io");
      const scope = `${plugin}/${artifact}`;
      if (!existsSync(ioDir) || !statSync(ioDir).isDirectory()) {
        breaches.push({
          kind: "artifact-io/facet-completeness",
          scope,
          summary: `missing 🚪️io facet under ${scope}`,
          reason: "Every artifact must carry the 🚪️io facet (ticket 26/08/10/ARTIFACT-IO-FACETS).",
          solution: `Create ${ioDir} with 🦀️component.rs implementing ArtifactIo and per-format import/export leaves.`,
          autofixable: false,
        } as BreachRecord);
        continue;
      }
      const rs = join(ioDir, "🦀️component.rs");
      if (!existsSync(rs)) {
        breaches.push({
          kind: "artifact-io/facet-completeness",
          scope,
          summary: `missing 🚪️io/🦀️component.rs under ${scope}`,
          reason: "The io facet root must declare ArtifactIo.",
          solution: `Add ${rs}.`,
          autofixable: false,
        } as BreachRecord);
      } else {
        const body = readFileSync(rs, "utf8");
        if (!body.includes("ArtifactIo")) {
          breaches.push({
            kind: "artifact-io/facet-completeness",
            scope,
            summary: `🚪️io root does not mention ArtifactIo under ${scope}`,
            reason: "The io facet root must declare ArtifactIo.",
            solution: `Implement ArtifactIo in ${rs}.`,
            autofixable: false,
          } as BreachRecord);
        }
      }
    }
  }
  return breaches;
}

function policyArtifactIoLeafParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomyPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  if (!existsSync(taxonomyPath)) return breaches;
  const taxonomy = JSON.parse(readFileSync(taxonomyPath, "utf8")) as {
    mediaFormatDirs?: Record<string, string>;
    ioFormatChildDirs?: string[];
  };
  const mediaFormatDirs = taxonomy.mediaFormatDirs ?? {};
  const leafParents = taxonomy.ioFormatChildDirs ?? ["📥️import", "📤️export"];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot)) {
    const artifactsRoot = join(pluginsRoot, plugin, "🗿️artifacts");
    if (!existsSync(artifactsRoot) || !statSync(artifactsRoot).isDirectory()) continue;
    for (const artifact of readdirSync(artifactsRoot)) {
      const ioDir = join(artifactsRoot, artifact, "🚪️io");
      if (!existsSync(ioDir) || !statSync(ioDir).isDirectory()) continue;
      const scope = `${plugin}/${artifact}`;
      for (const name of readdirSync(ioDir)) {
        const child = join(ioDir, name);
        if (!statSync(child).isDirectory()) continue;
        if (name.endsWith("component.rs") || name.endsWith("component.ts")) continue;
        const known = Object.values(mediaFormatDirs).includes(name);
        if (!known) {
          breaches.push({
            kind: "artifact-io/leaf-parity",
            scope,
            summary: `unknown format dir ${name} under ${scope}/🚪️io`,
            reason: "Format dirs must come from taxonomy.mediaFormatDirs.",
            solution: `Rename ${name} to a catalog dir or update mediaFormatDirs.`,
            autofixable: false,
          } as BreachRecord);
          continue;
        }
        for (const leaf of leafParents) {
          const leafDir = join(child, leaf);
          const rs = join(leafDir, "🦀️component.rs");
          if (!existsSync(rs)) {
            breaches.push({
              kind: "artifact-io/leaf-parity",
              scope,
              summary: `missing ${leaf}/🦀️component.rs for ${name} under ${scope}`,
              reason: "Each format must provide both import and export leaves.",
              solution: `Add ${rs}.`,
              autofixable: false,
            } as BreachRecord);
          }
        }
      }
    }
  }
  return breaches;
}

function policyArtifactIoNoEngineIoBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const banned = [
    "register_2d_export_handlers",
    "register_mesh_exporter",
    "register_mesh_importer",
    "register_solid_exporter",
    "register_solid_importer",
    "register_mesh_dwg_export_handler",
    "register_dwg_import_handler",
  ];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot)) {
    const artifactsRoot = join(pluginsRoot, plugin, "🗿️artifacts");
    if (!existsSync(artifactsRoot) || !statSync(artifactsRoot).isDirectory()) continue;
    for (const artifact of readdirSync(artifactsRoot)) {
      const eng = join(artifactsRoot, artifact, "⚙️engine", "🦀️component.rs");
      if (!existsSync(eng)) continue;
      const body = readFileSync(eng, "utf8")
        .split(/\r?\n/)
        .filter((line) => {
          const trimmed = line.trim();
          return trimmed.length > 0 && !trimmed.startsWith("//") && !trimmed.startsWith("///") && !trimmed.startsWith("*");
        })
        .join("\n");
      const scope = `${plugin}/${artifact}`;
      for (const fn of banned) {
        if (body.includes(fn)) {
          breaches.push({
            kind: "artifact-io/no-engine-io",
            scope,
            summary: `⚙️engine still calls ${fn} under ${scope}`,
            reason: "Media registration must live in 🚪️io and be invoked via io::register().",
            solution: `Delete ${fn} from engine and call crate::artifacts::<ascii>::io::register().`,
            autofixable: false,
          } as BreachRecord);
        }
      }
      if (!body.includes("io::register()")) {
        breaches.push({
          kind: "artifact-io/registration",
          scope,
          summary: `⚙️engine does not call io::register() under ${scope}`,
          reason: "Engine register must delegate media handlers to the io facet.",
          solution: `Add crate::artifacts::<ascii>::io::register() inside engine::register().`,
          autofixable: false,
        } as BreachRecord);
      }
    }
  }
  return breaches;
}

//#endregion 🔧️PolicyRuleArtifactIo

//#region 🔖️PolicyExport
/**
 * ⚖️Runs every Wave 4 app-plugin rule over every discovered crate that belongs to a plugin, plus the
 * taxonomy rules (`PolicyRuleTaxonomy` region) over EVERY discovered Shape V2 rust package repo-wide.
 * Discovery is the shared package catalog (`policyDiscoverCrateDirs` → `discoverPackages`), so the
 * plugins/framework/hub split is expressed by each package's declared `role`, not by a path literal:
 * the Wave 4 rules encode plugin-app conventions (`App::builder`, `Plugin::builder`, the SDK testkit) and
 * stay `role = "plugin"`, while the structural taxonomy rules apply to every owner that has adopted the
 * shape. The framework SDK crate is excluded by role, exactly as the old plugins-only path scoping did.
 */
export const policy = defineLint("@semio-tech/workspace-app-plugin-consistency", (_l: TechnologyLinter): BreachRecord[] => {
  const repoRoot = getWorkspaceRoot();
  const crateDirs = policyDiscoverCrateDirs(repoRoot);
  const breaches: BreachRecord[] = [];

  for (const crate of crateDirs) {
    if (crate.role !== "plugin") continue;
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue; // tolerant: an in-flight taxonomy crate whose lib.rs hasn't landed yet must not crash the lint
    const content = readFileSync(abs, "utf8");
    const lines = content.split(/\r?\n/);

    breaches.push(...policyRegionFormatBreaches(crate.dir, lines));
    breaches.push(...policyManifestRegionBreaches(crate.dir, lines));
    breaches.push(...policyStructNamingBreaches(crate.dir, content));
    breaches.push(...policyModLayoutBreaches(crate.dir, lines));
    breaches.push(...policySelectionIdsBreaches(crate.dir, content));
    breaches.push(...policyTestkitDelegateBreaches(crate.dir, content));
    breaches.push(...policyTreeItemBreaches(crate, content, lines));
    breaches.push(...policyLabelsStructBreaches(crate, content));
  }

  breaches.push(...policyCargoArtifactBreaches(repoRoot, crateDirs));
  breaches.push(...policyAppCouplingBreaches(repoRoot, crateDirs));
  breaches.push(...policyTaxonomyDirsBreaches(repoRoot, crateDirs));
  breaches.push(...policySemioArtifactExamplesBreaches(repoRoot, crateDirs));
  breaches.push(...policyDeadExampleLeafBreaches(repoRoot, crateDirs));
  breaches.push(...policyComponentFileBreaches(repoRoot, crateDirs));
  breaches.push(...policyTaxonomyLibShapeBreaches(repoRoot, crateDirs));
  breaches.push(...policyTaxonomyBarrelShapeBreaches(repoRoot));
  breaches.push(...policySprNamingBreaches(repoRoot, crateDirs));
  breaches.push(...policyBannedNameStemBreaches(repoRoot));
  breaches.push(...policyEmojiPrefixBreaches(repoRoot));
  breaches.push(...policyPluginRootShapeBreaches(repoRoot));
  breaches.push(...policyPluginBuilderBreaches(repoRoot, crateDirs));
  breaches.push(...policyJsonFixtureBreaches(repoRoot));
  breaches.push(...policyOpsGrammarBreaches(repoRoot));
  breaches.push(...policyDslCompletenessBreaches(repoRoot));
  breaches.push(...policyPackCompletenessBreaches(repoRoot));
  breaches.push(...policyCommandEnvelopeCompletenessBreaches(repoRoot));
  breaches.push(...policyDiffCompletenessBreaches(repoRoot));
  breaches.push(...policyGrammarFileBreaches(repoRoot));
  breaches.push(...policyProtocolFileBreaches(repoRoot));
  breaches.push(...policyTsFacadeBreaches(repoRoot));
  breaches.push(...policyMutationArtifactEngineBreaches(repoRoot));
  breaches.push(...policyHandcraftedSpecP3Breaches(repoRoot));
  breaches.push(...policyArtifactSchemaBreaches(repoRoot));
  breaches.push(...policyAppSchemaBreaches(repoRoot));
  breaches.push(...policyArtifactIoBreaches(repoRoot));
  breaches.push(...policyProtocolMigrationBreaches(repoRoot));
  breaches.push(...policyDbServerOnlyBreaches(repoRoot));
  breaches.push(...policyOsStateAuthorityBreaches(repoRoot));
  breaches.push(...policyDocumentAppShapeBreaches(repoRoot));
  breaches.push(...policyNoPackFilesBreaches(repoRoot));
  breaches.push(...policyRawSpawnBreaches(repoRoot));
  breaches.push(...policyBudgetNullBreaches(repoRoot));
  breaches.push(...policyMcpConfigBreaches(repoRoot));
  return breaches;
});
//#endregion 🔖️PolicyExport
//#endregion 🔖️Policy

if (import.meta.main) {
  if (!(await dispatchPolicyArgv(process.argv.slice(2), import.meta.url))) {
    await runWorkspaceScriptMain(router);
  }
}
