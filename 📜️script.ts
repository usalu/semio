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
  discoverPackageProblems,
  dispatchPolicyArgv,
  dispatchSubcommand,
  defineLint,
  loadTaxonomy,
  schemaFacetFormatEntries,
  enforceCoverageThreshold,
  frameworkOsPlaygroundDevEnv,
  getWorkspaceRoot,
  getRepoMetaDir,
  getMapCacheDir,
  getSemioRoot,
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
import {
  buildSemanticCensus,
  renderSemanticCensusJson,
  renderSemanticCensusMarkdown,
  renderSemanticDuplicatesJson,
  renderSemanticDuplicatesMarkdown,
  renderSemanticTaxonomyReport,
} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";
import { createHash } from "node:crypto";
import { existsSync, linkSync, mkdirSync, chmodSync, chownSync, copyFileSync, readFileSync, readdirSync, realpathSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
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

/** 🦑️Builds the repo MCP client from the current source before execution. */
function buildRepoMcpClient(root: string): string {
  const bin = resolveCliBin(root);
  runCmd("go", ["build", "-o", bin, `./${REPO_MCP_GO}`], {
    cwd: root,
    env: { ...process.env, GOWORK: join(root, "go.work") },
    budgetMs: buildBudgetMs(),
  });
  return bin;
}

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
      const ps1 = join(NATIVE_BOOTSTRAP_DIR, "🪟️script.ps1");
      if (!existsSync(ps1)) {
        console.error(`[native] missing ${ps1}; expected repo/native/bootstrap/🪟️script.ps1.`);
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

  const cacheDir = join(getRepoMetaDir(WORKSPACE_ROOT), "⚡️cache", "sccache");
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
    const repoClientPath = buildRepoMcpClient(this.root);
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
        console.log("[start] `bun run generate` did not refresh all `.🧬semio/🦑️repo/🛂️manifest` bundles (Neo4j may be offline).");
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
    const bin = buildRepoMcpClient(this.root);
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
function taxonomyOption(args: readonly string[], name: string): string | undefined {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
}

/** 🎫️ Resolves a ticket id through actual Unicode directory entries instead of constructing emoji mounts. */
function taxonomyTicketDirectory(repoRoot: string, ticketId: string): string {
  const parts = ticketId.split("/").filter(Boolean);
  if (parts.length !== 4) throw new Error(`[taxonomy] ticket id must be YYYY/MM/DD/TICKETSLUG, got ${JSON.stringify(ticketId)}.`);
  let current = realpathSync(join(getRepoMetaDir(repoRoot), "🎫️tickets"));
  for (const part of parts) {
    const entry = readdirSync(current, { withFileTypes: true })
      .filter((candidate) => candidate.isDirectory())
      .sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0)
      .find((candidate) => candidate.name === part || candidate.name.replace(/^\p{Extended_Pictographic}\uFE0F?/u, "") === part);
    if (!entry) throw new Error(`[taxonomy] ticket segment ${JSON.stringify(part)} does not exist below ${current}.`);
    current = realpathSync(join(current, entry.name));
  }
  return current;
}

export class GenerateScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "taxonomy") {
      this.generateTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "neo4j") {
      new Neo4jCypherExport(this.root).runFromArgv(segments.slice(1));
      return;
    }
    if (segments[0] === "plugin-glue") {
      this.generatePluginGlue(segments.slice(1));
      return;
    }
    if (segments[0] === "scale-fixture") {
      runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:generate-scale-fixture", ...segments.slice(1)], { cwd: this.root });
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
    console.log(`[generate] Neo4j Cypher export finished (${successes} ok, ${failures} skipped/failed) under .🧬semio/🦑️repo/🛂️manifest.`);
  }

  /** 🧩️ Writes deterministic census or duplicate evidence with its Markdown companion. */
  private generateTaxonomy(args: string[]): void {
    const operation = args[0];
    if (operation !== "census" && operation !== "duplicates") throw new Error(`[generate taxonomy] expected census or duplicates, got ${JSON.stringify(operation)}.`);
    const ticketId = taxonomyOption(args, "--ticket");
    if (!ticketId) throw new Error(`[generate taxonomy ${operation}] --ticket <ticket-id> is required.`);
    const ticketDir = taxonomyTicketDirectory(this.root, ticketId);
    const census = buildSemanticCensus(this.root);
    if (operation === "census") {
      writeFileSync(join(ticketDir, "📊️semantic-census.json"), renderSemanticCensusJson(census));
      writeFileSync(join(ticketDir, "📓️semantic-census.md"), renderSemanticCensusMarkdown(census));
    } else {
      writeFileSync(join(ticketDir, "📊️semantic-duplicates.json"), renderSemanticDuplicatesJson(census));
      writeFileSync(join(ticketDir, "📓️semantic-duplicates.md"), renderSemanticDuplicatesMarkdown(census));
    }
    console.log(`[generate taxonomy ${operation}] ${census.records.length} components, ${census.problems.length} findings -> ${ticketDir}`);
  }

  /** 🧬️ Emit deterministic #[path] wiring comments for subset/example modules (dry by default). */
  private generatePluginGlue(args: string[]): void {
    const dry = args.includes("--dry") || args.includes("--dry-run") || args.length === 0;
    const pluginFilter = args.find((a) => !a.startsWith("--"));
    const pluginsRoot = join(this.root, "✏️s/🔌️plugins");
    const plugins = readdirSync(pluginsRoot, { withFileTypes: true }).filter((d) => d.isDirectory()).map((d) => d.name);
    let emitted = 0;
    for (const plugin of plugins) {
      if (pluginFilter && plugin !== pluginFilter && !plugin.includes(pluginFilter)) continue;
      const owned = policyListTopLevelSubsetDirs(this.root).filter((s) => s.split("/")[2] === plugin);
      const lines: string[] = [
        `// GENERATED-BY: bun ./📜️script.ts generate plugin-glue ${plugin}`,
        `// subsets=${owned.length} (registration body still hand-authored until serializer wave; this command validates discoverability)`,
      ];
      for (const sub of owned) {
        lines.push(`// subset: ${sub}`);
        emitted += 1;
      }
      const glue = join(pluginsRoot, plugin, "📦️packages/🦀️rust/📦️glue.rs");
      if (!existsSync(glue)) {
        console.warn(`[generate plugin-glue] skip ${plugin}: missing glue.rs`);
        continue;
      }
      if (dry) {
        console.log(`[generate plugin-glue] dry ${plugin}: ${owned.length} subsets`);
      } else {
        const inv = join(pluginsRoot, plugin, "📦️packages/🦀️rust/🤖️generated-subset-inventory.txt");
        writeFileSync(inv, lines.join("\n") + "\n");
        console.log(`[generate plugin-glue] wrote ${inv}`);
      }
    }
    console.log(`[generate plugin-glue] ${dry ? "dry-run" : "wrote"} inventory for ${emitted} subset refs`);
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
    runCmd("bunx", ["dependency-cruiser", "compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
  }
}
//#endregion 🔖️LintScript

//#region 🔖️VerifyScript
/** 🧪️Aggregates lint + generated-catalog freshness + region/host-contract script lints (`gate`, the cheap pre-`ticket_close` step every refactor session runs), plus the full test suite for the top-level `verify` verb. */
export class VerifyScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "taxonomy") {
      this.runTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-outcome-law") {
      this.runMutationOutcomeLaw();
      return;
    }
    await this.runGate();
    if (segments[0] === "gate") return;
    runCmd("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
  }

  /** 🚦️ Reports findings without failing, or enforces the identical structured result. */
  private runTaxonomy(args: string[]): void {
    const mode = args[0];
    if (mode !== "report" && mode !== "enforce") throw new Error(`[verify taxonomy] expected report or enforce, got ${JSON.stringify(mode)}.`);
    const scope = taxonomyOption(args, "--scope");
    const census = buildSemanticCensus(this.root, { scope });
    process.stdout.write(renderSemanticTaxonomyReport(census, scope));
    if (mode === "enforce" && census.problems.some((problem) => problem.severity === "error")) throw new Error(`[verify taxonomy enforce] ${census.problems.filter((problem) => problem.severity === "error").length} error finding(s).`);
  }

  /**
   * ⚖️Standalone entry point for the `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
   * gate bundle (`policyMutationOutcomeMergePolicyBreaches`'s 7 rules) — the same checks `runGate()`'s
   * "mutation-outcome / merge-policy law" block enforces, runnable in isolation via
   * `bun ./📜️script.ts verify mutation-outcome-law` (the `mutation-outcome-law` nx target in
   * `📋️project.json` wires this ahead of `verify-gate`).
   */
  private runMutationOutcomeLaw(): void {
    const breaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
    if (breaches.length > 0) {
      for (const b of breaches) console.error(`[verify mutation-outcome-law] ${b.kind}: ${b.summary}`);
      throw new Error(`[verify mutation-outcome-law] ${breaches.length} breach(es)`);
    }
    console.log("[verify mutation-outcome-law] passed.");
  }

  private async runGate(): Promise<void> {
    // Deliberately calls dependency-cruiser directly rather than `LintScript`/`nx run-many -t lint --all`:
    // several unrelated projects (repo/client/vscode, compose-js, …) have pre-existing broken eslint configs,
    // and framework-renderer-wgpu:lint has known pending color-literal violations (see spawn_task follow-ups) —
    // this gate must stay a meaningful, currently-green signal for refactor sessions, not inherit that noise.
    console.log("[verify] dependency-cruiser boundaries…");
    runCmd("bunx", ["dependency-cruiser", "compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
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
    console.log("[verify] standards/subsets vocabulary…");
    {
      const subsetVocabBreaches = [
        ...policyStandardsCoverageBreaches(this.root),
        ...policyStandardSubsetVocabularyBreaches(this.root),
      ].filter((b) => b.priority === "high");
      if (subsetVocabBreaches.length > 0) {
        for (const b of subsetVocabBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${subsetVocabBreaches.length} standards/subsets vocabulary policy breach(es)`);
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
    console.log("[verify] package language purity…");
    {
      const packagePurityBreaches = policyPackageLanguagePurityBreaches(this.root).filter((b) => b.priority === "high");
      if (packagePurityBreaches.length > 0) {
        for (const b of packagePurityBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${packagePurityBreaches.length} package language purity breach(es)`);
      }
    }
    console.log("[verify] dissolve-core / plugin-root policies…");
    {
      const dissolveBreaches = [
        ...policyBannedNameStemBreaches(this.root),
        ...policyEmojiPrefixBreaches(this.root),
        ...policyPluginRootShapeBreaches(this.root),
        ...policyPluginBuilderBreaches(this.root, policyDiscoverCrateDirs(this.root)),
        ...policyApaBreaches(this.root),
        ...policyInferenceFamilyBreaches(this.root),
      ].filter((b) => b.priority === "high");
      if (dissolveBreaches.length > 0) {
        for (const b of dissolveBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${dissolveBreaches.length} dissolve-core / plugin-root policy breach(es)`);
      }
    }
    console.log("[verify] window capability taxonomy…");
    {
      const crateDirs = policyDiscoverCrateDirs(this.root);
      const windowBreaches = [...policyWindowCompletenessBreaches(this.root, crateDirs), ...policyModeCompletenessBreaches(this.root, crateDirs)];
      if (windowBreaches.length > 0) {
        for (const breach of windowBreaches) console.error(`[verify] ${breach.kind}: ${breach.summary}`);
        throw new Error(`[verify] ${windowBreaches.length} window/mode capability taxonomy breach(es)`);
      }
    }
    console.log("[verify] mutation-outcome / merge-policy law…");
    {
      const mutationOutcomeBreaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
      if (mutationOutcomeBreaches.length > 0) {
        for (const b of mutationOutcomeBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${mutationOutcomeBreaches.length} mutation-outcome/merge-policy law breach(es)`);
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

  /** 📊️Walks every `*.lcov`/`lcov.info`/`coverage.info`/`*.cover` file under `.🧬semio/🦑️repo/📊️metrics/coverage/`, merges them into one repo-wide LCOV, writes `summary.json`, and hard-fails below the 95% threshold — the exhaustive-level gate. */
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
    writeFileSync(join(getRepoMetaDir(this.root), "📊️metrics", "coverage", "lcov.info"), renderLcov(merged));
    writeFileSync(join(getRepoMetaDir(this.root), "📊️metrics", "coverage", "summary.json"), JSON.stringify(summary, null, 2));
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
      runCmd("bunx", ["playwright", "test", "--config", ".storybook/playwright.config.ts"], {
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

//#region 🔖️StdioLedgerScript
type StdioDialectDefinition = Readonly<{
  id: string;
  artifact: string;
  standard: string;
  subset: string;
  schema: string;
  io: string;
  mutations: string | null;
  inferences: string | null;
}>;

type StdioCodecDefinition = Readonly<{
  id: string;
  status: string;
  from: string;
  to: string;
  executable_registration: boolean;
}>;

type StdioRuntimeCapability = Readonly<{
  id: string;
  category: "schema" | "inference" | "codec" | "representation" | "grammar" | "composer" | "subset-validator";
  descriptor: string;
  claims: readonly Readonly<{ namespace: "schema" | "codec" | "extension" | "mime" | "dialect" | "grammar"; value: string }>[];
}>;

type StdioExecutableLeaf = Readonly<{ id: string; status: "unimplemented" | "implemented" | "verified"; executable_registration: boolean }>;

type StdioArtifactDefinition = Readonly<{
  id: string;
  kind: string;
  directory: string;
  depends: readonly string[];
  component: string;
  dialects: readonly StdioDialectDefinition[];
  representations: StdioSchemaDefinition["representations"];
  codecs: readonly StdioCodecDefinition[];
  support: StdioSupportLedger;
}>;

type StdioSupportState = "unimplemented" | "opaque" | "implemented";

type StdioSupportLedger = Readonly<{
  normative_source: string | null;
  publication_date: string | null;
  source_checksum: string | null;
  redistribution_status: string;
  clauses_or_features: readonly string[];
  profiles: readonly string[];
  registered_code_points: readonly string[];
  read: StdioSupportState;
  write: StdioSupportState;
  lossless: StdioSupportState;
  canonical: StdioSupportState;
  validators: readonly string[];
  mutations: readonly string[];
  inferences: readonly string[];
  fixtures: readonly string[];
}>;

type StdioSchemaDefinition = Readonly<{
  id: string;
  artifact: string;
  directory: string;
  dependencies: readonly string[];
  standards: readonly Readonly<{ id: string; revision: string; status: string }>[];
  profiles: readonly Readonly<{ id: string; standard: string; profile: string; status: string }>[];
  source_dialects: readonly Readonly<{ id: string; standard: string; dialect: string; registered_code_points: readonly string[]; status: string }>[];
  representations: readonly Readonly<{ id: string; standard: string; representation: string; mimes: readonly string[]; extensions: readonly string[]; is_binary: boolean; aliases: readonly string[]; neutral: boolean; status: string }>[];
  codecs: readonly StdioCodecDefinition[];
  mutations: readonly StdioExecutableLeaf[];
  inferences: readonly StdioExecutableLeaf[];
  resources: readonly Readonly<{ id: string; status: string }>[];
  localized_descriptors: readonly Readonly<{ id: string; locale: "en" | "de"; name: string; description: string; status: string }>[];
  conformance_suites: readonly Readonly<{ id: string; status: string; fixtures: readonly string[] }>[];
  runtime_capabilities: readonly StdioRuntimeCapability[];
  support_ledger: StdioSupportLedger;
}>;

type StdioArtifactLedger = Readonly<{
  artifacts: readonly StdioArtifactDefinition[];
  counts: Readonly<{ artifacts: number; registeredMimes: number; standards: number; profiles: number; dialects: number; representations: number; codecs: number; mutations: number; inferences: number; conformanceSuites: number; runtimeCapabilities: number; capabilities: Readonly<{ declared: number; registered: number; implemented: number; verified: number }> }>;
}>;

const STDIO_ROOT_REL = join("✏️s", "🔌️plugins", "🗄️stdio");
const STDIO_ARTIFACTS_DIR = "🗿️artifacts";
const STDIO_CATALOG_REL = join(STDIO_ROOT_REL, "📇️registry", "📇️catalog.json");
const STDIO_COMPONENT_TS = "🟦️component.ts";
const STDIO_COMPONENT_RS = "🦀️component.rs";

function stdioRel(workspaceRoot: string, path: string): string {
  return relative(workspaceRoot, path).split("\\").join("/");
}

function stdioWalkFiles(path: string, files: string[] = []): string[] {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    const full = join(path, entry.name);
    if (entry.isDirectory()) stdioWalkFiles(full, files);
    else files.push(full);
  }
  return files;
}

function stdioAssertUnique(label: string, values: readonly string[]): void {
  const duplicateValues = [...new Set(values.filter((value, index) => values.indexOf(value) !== index))];
  if (duplicateValues.length > 0) throw new Error(`[stdio] duplicate ${label}: ${duplicateValues.sort().join(", ")}`);
}

function stdioDefinitionIdentity(id: string, suffix = ""): void {
  if (!/^s\.stdio\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(id)) throw new Error(`[stdio] non-canonical definition identity ${JSON.stringify(id)}.`);
  if (suffix && !id.includes(suffix)) throw new Error(`[stdio] definition identity ${JSON.stringify(id)} is missing ${suffix}.`);
}

function stdioVersionedLeaf(id: string, prefix: string): void {
  if (!id.startsWith(prefix) || !/^[a-z0-9-]+\.v[1-9][0-9]*$/u.test(id.slice(prefix.length))) throw new Error(`[stdio] ${JSON.stringify(id)} must be a canonical vN leaf below ${JSON.stringify(prefix)}.`);
  stdioDefinitionIdentity(id);
}

function stdioRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`[stdio] ${label} must be an object.`);
  return value as Record<string, unknown>;
}

function stdioExactFields(value: unknown, label: string, fields: readonly string[]): Record<string, unknown> {
  const record = stdioRecord(value, label);
  const missing = fields.filter((field) => !(field in record));
  const unknown = Object.keys(record).filter((field) => !fields.includes(field));
  if (missing.length > 0 || unknown.length > 0) throw new Error(`[stdio] ${label} has ${missing.length ? `missing fields ${missing.join(", ")}` : ""}${missing.length && unknown.length ? "; " : ""}${unknown.length ? `unknown fields ${unknown.join(", ")}` : ""}.`);
  return record;
}

function stdioString(value: unknown, label: string): string {
  if (typeof value !== "string" || !value) throw new Error(`[stdio] ${label} must be a non-empty string.`);
  return value;
}

function stdioStringArray(value: unknown, label: string): readonly string[] {
  if (!Array.isArray(value) || !value.every((item) => typeof item === "string" && item)) throw new Error(`[stdio] ${label} must be a string array.`);
  return value;
}

function stdioNullableString(value: unknown, label: string): string | null {
  if (value === null || typeof value === "string") return value;
  throw new Error(`[stdio] ${label} must be a string or null.`);
}

function stdioArray(value: unknown, label: string): readonly unknown[] {
  if (!Array.isArray(value)) throw new Error(`[stdio] ${label} must be an array.`);
  return value;
}

function stdioDefinitionCatalog(workspaceRoot: string): readonly StdioSchemaDefinition[] {
  const catalogPath = join(workspaceRoot, STDIO_CATALOG_REL);
  const catalog = stdioExactFields(JSON.parse(readFileSync(catalogPath, "utf8")), "definition catalog", ["artifact_definition_paths"]);
  if (!Array.isArray(catalog.artifact_definition_paths)) throw new Error("[stdio] catalog must contain artifact_definition_paths.");
  const paths = catalog.artifact_definition_paths;
  if (!paths.every((path): path is string => typeof path === "string" && path.endsWith("📜️artifact-definition.json"))) throw new Error("[stdio] catalog paths must target artifact definition leaves.");
  stdioAssertUnique("artifact definition path", paths);
  const definitions = paths.map((definitionPath) => {
    const absolute = resolve(dirname(catalogPath), definitionPath);
    const artifactsRoot = resolve(workspaceRoot, STDIO_ROOT_REL, STDIO_ARTIFACTS_DIR);
    if (!absolute.startsWith(`${artifactsRoot}/`)) throw new Error(`[stdio] artifact definition escapes the stdio artifact root: ${definitionPath}.`);
    if (!existsSync(absolute)) throw new Error(`[stdio] artifact definition is missing: ${definitionPath}.`);
    return JSON.parse(readFileSync(absolute, "utf8"));
  });
  const artifactRoot = join(workspaceRoot, STDIO_ROOT_REL, STDIO_ARTIFACTS_DIR);
  const discovered = stdioWalkFiles(artifactRoot)
    .filter((path) => path.endsWith("📜️artifact-definition.json"))
    .map((path) => relative(dirname(catalogPath), path).split("\\").join("/"))
    .sort();
  if (discovered.join("\n") !== [...paths].sort().join("\n")) throw new Error("[stdio] catalog definition paths are not the complete schema-owned artifact definition set.");
  if (definitions.length !== 36) throw new Error(`[stdio] expected 36 artifact definitions, got ${definitions.length}.`);
  definitions.forEach(stdioAssertDefinition);
  return definitions as readonly StdioSchemaDefinition[];
}

function stdioAssertDefinition(value: unknown): asserts value is StdioSchemaDefinition {
  const definition = stdioExactFields(value, "artifact definition", ["definition_version", "id", "artifact", "directory", "dependencies", "standards", "profiles", "source_dialects", "representations", "codecs", "mutations", "inferences", "resources", "localized_descriptors", "conformance_suites", "runtime_capabilities", "support_ledger"]);
  if (definition.definition_version !== 1) throw new Error("[stdio] artifact definition_version must equal 1.");
  const artifact = stdioString(definition.artifact, "artifact slug");
  const id = stdioString(definition.id, "artifact definition id");
  const artifactId = `s.stdio.${definition.artifact}`;
  if (id !== artifactId) throw new Error(`[stdio] definition ${id} does not own ${artifactId}.`);
  stdioDefinitionIdentity(id);
  if (!/^[a-z0-9_-]+$/u.test(artifact)) throw new Error(`[stdio] invalid artifact slug ${JSON.stringify(artifact)}.`);
  stdioString(definition.directory, `${id}.directory`);
  stdioStringArray(definition.dependencies, `${id}.dependencies`);
  const standards = stdioArray(definition.standards, `${id}.standards`).map((standard, index) => {
    const record = stdioExactFields(standard, `${id}.standards[${index}]`, ["id", "revision", "normative_source", "publication_date", "source_checksum", "redistribution_status", "clauses_or_features", "status"]);
    stdioString(record.id, `${id}.standards[${index}].id`);
    const revision = stdioString(record.revision, `${id}.standards[${index}].revision`);
    if (!/^[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(revision)) throw new Error(`[stdio] ${id} standard revision ${JSON.stringify(revision)} is not a canonical identity fragment.`);
    stdioNullableString(record.normative_source, `${id}.standards[${index}].normative_source`);
    stdioNullableString(record.publication_date, `${id}.standards[${index}].publication_date`);
    stdioNullableString(record.source_checksum, `${id}.standards[${index}].source_checksum`);
    stdioString(record.redistribution_status, `${id}.standards[${index}].redistribution_status`);
    stdioStringArray(record.clauses_or_features, `${id}.standards[${index}].clauses_or_features`);
    stdioString(record.status, `${id}.standards[${index}].status`);
    return record as unknown as StdioSchemaDefinition["standards"][number];
  });
  if (standards.length === 0) throw new Error(`[stdio] ${id} has no standards.`);
  for (const standard of standards) {
    const id = `${artifactId}.standard.${standard.revision}`;
    if (standard.id !== id) throw new Error(`[stdio] standard ${standard.id} must be ${id}.`);
    stdioDefinitionIdentity(standard.id, ".standard.");
  }
  const standardIds = new Set(standards.map((standard) => standard.id));
  const profiles = stdioArray(definition.profiles, `${artifactId}.profiles`).map((profile, index) => {
    const record = stdioExactFields(profile, `${artifactId}.profiles[${index}]`, ["id", "standard", "profile", "status"]);
    stdioString(record.id, `${artifactId}.profiles[${index}].id`);
    stdioString(record.standard, `${artifactId}.profiles[${index}].standard`);
    stdioString(record.profile, `${artifactId}.profiles[${index}].profile`);
    stdioString(record.status, `${artifactId}.profiles[${index}].status`);
    return record as unknown as StdioSchemaDefinition["profiles"][number];
  });
  if (profiles.length === 0) throw new Error(`[stdio] ${artifactId} has no profiles.`);
  for (const profile of profiles) {
    if (!standardIds.has(profile.standard) || profile.id !== `${profile.standard}.profile.${profile.profile}`) throw new Error(`[stdio] invalid profile identity ${profile.id}.`);
    stdioDefinitionIdentity(profile.id, ".profile.");
  }
  const dialects = stdioArray(definition.source_dialects, `${artifactId}.source_dialects`).map((dialect, index) => {
    const record = stdioExactFields(dialect, `${artifactId}.source_dialects[${index}]`, ["id", "standard", "dialect", "registered_code_points", "status"]);
    stdioString(record.id, `${artifactId}.source_dialects[${index}].id`);
    stdioString(record.standard, `${artifactId}.source_dialects[${index}].standard`);
    stdioString(record.dialect, `${artifactId}.source_dialects[${index}].dialect`);
    stdioStringArray(record.registered_code_points, `${artifactId}.source_dialects[${index}].registered_code_points`);
    stdioString(record.status, `${artifactId}.source_dialects[${index}].status`);
    return record as unknown as StdioSchemaDefinition["source_dialects"][number];
  });
  if (dialects.length === 0) throw new Error(`[stdio] ${artifactId} has no source dialects.`);
  for (const dialect of dialects) {
    if (!standardIds.has(dialect.standard) || dialect.id !== `${dialect.standard}.dialect.${dialect.dialect}`) throw new Error(`[stdio] invalid source dialect identity ${dialect.id}.`);
    stdioDefinitionIdentity(dialect.id, ".dialect.");
  }
  const representations = stdioArray(definition.representations, `${artifactId}.representations`).map((representation, index) => {
    const record = stdioExactFields(representation, `${artifactId}.representations[${index}]`, ["id", "standard", "representation", "mimes", "extensions", "is_binary", "aliases", "neutral", "status"]);
    stdioString(record.id, `${artifactId}.representations[${index}].id`);
    stdioString(record.standard, `${artifactId}.representations[${index}].standard`);
    stdioString(record.representation, `${artifactId}.representations[${index}].representation`);
    stdioStringArray(record.mimes, `${artifactId}.representations[${index}].mimes`);
    stdioStringArray(record.extensions, `${artifactId}.representations[${index}].extensions`);
    if (typeof record.is_binary !== "boolean") throw new Error(`[stdio] ${artifactId}.representations[${index}].is_binary must be boolean.`);
    stdioStringArray(record.aliases, `${artifactId}.representations[${index}].aliases`);
    if (typeof record.neutral !== "boolean") throw new Error(`[stdio] ${artifactId}.representations[${index}].neutral must be boolean.`);
    stdioString(record.status, `${artifactId}.representations[${index}].status`);
    return record as unknown as StdioSchemaDefinition["representations"][number];
  });
  if (representations.length === 0) throw new Error(`[stdio] ${artifactId} has no representations.`);
  for (const representation of representations) {
    if (!standardIds.has(representation.standard) || representation.id !== `${representation.standard}.representation.${representation.representation}`) throw new Error(`[stdio] invalid representation identity ${representation.id}.`);
    if (representation.extensions.length === 0 || representation.extensions.some((extension) => !extension.startsWith("."))) throw new Error(`[stdio] ${representation.id} has no file extension.`);
    stdioAssertUnique(`MIME claim for ${representation.id}`, representation.mimes);
    stdioAssertUnique(`extension claim for ${representation.id}`, representation.extensions);
    stdioDefinitionIdentity(representation.id, ".representation.");
  }
  if (new Set(representations.map((representation) => representation.standard)).size !== standards.length) throw new Error(`[stdio] ${artifactId} must give each declared standard its own representation.`);
  const codecs = stdioArray(definition.codecs, `${artifactId}.codecs`).map((codec, index) => {
    const record = stdioExactFields(codec, `${artifactId}.codecs[${index}]`, ["id", "status", "from", "to", "executable_registration"]);
    stdioString(record.id, `${artifactId}.codecs[${index}].id`);
    stdioString(record.status, `${artifactId}.codecs[${index}].status`);
    stdioString(record.from, `${artifactId}.codecs[${index}].from`);
    stdioString(record.to, `${artifactId}.codecs[${index}].to`);
    if (typeof record.executable_registration !== "boolean") throw new Error(`[stdio] ${artifactId}.codecs[${index}].executable_registration must be boolean.`);
    return record as unknown as StdioCodecDefinition & { executable_registration: boolean };
  });
  for (const codec of codecs) {
    const standard = standards.find((candidate) => codec.id.startsWith(`${candidate.id}.codec.`));
    if (!standard) throw new Error(`[stdio] codec ${codec.id} is not owned by a declared standard.`);
    stdioVersionedLeaf(codec.id, `${standard.id}.codec.`);
    if (!["unimplemented", "implemented", "verified"].includes(codec.status)) throw new Error(`[stdio] codec ${codec.id} has an invalid implementation status.`);
  }
  const mutations = stdioArray(definition.mutations, `${artifactId}.mutations`).map((mutation, index) => {
    const record = stdioExactFields(mutation, `${artifactId}.mutations[${index}]`, ["id", "status", "executable_registration"]);
    stdioString(record.id, `${artifactId}.mutations[${index}].id`);
    stdioString(record.status, `${artifactId}.mutations[${index}].status`);
    if (typeof record.executable_registration !== "boolean") throw new Error(`[stdio] ${artifactId}.mutations[${index}].executable_registration must be boolean.`);
    return record as unknown as StdioSchemaDefinition["mutations"][number] & { executable_registration: boolean };
  });
  for (const mutation of mutations) {
    stdioVersionedLeaf(mutation.id, `${artifactId}.mutation.`);
    if (artifact === "gltf" && (mutation.id.includes(".no-mutation.") || mutation.id.includes(".set-snapshot.") || mutation.id.includes(".set-"))) throw new Error(`[stdio] GLTF mutation ${mutation.id} is not semantically specific.`);
    if (!["unimplemented", "implemented", "verified"].includes(mutation.status)) throw new Error(`[stdio] mutation ${mutation.id} has an invalid implementation status.`);
  }
  const inferences = stdioArray(definition.inferences, `${artifactId}.inferences`).map((inference, index) => {
    const record = stdioExactFields(inference, `${artifactId}.inferences[${index}]`, ["id", "status", "executable_registration"]);
    stdioString(record.id, `${artifactId}.inferences[${index}].id`);
    stdioString(record.status, `${artifactId}.inferences[${index}].status`);
    if (typeof record.executable_registration !== "boolean") throw new Error(`[stdio] ${artifactId}.inferences[${index}].executable_registration must be boolean.`);
    return record as unknown as StdioSchemaDefinition["inferences"][number] & { executable_registration: boolean };
  });
  for (const inference of inferences) {
    stdioVersionedLeaf(inference.id, `${artifactId}.inference.`);
    if (!["unimplemented", "implemented", "verified"].includes(inference.status)) throw new Error(`[stdio] inference ${inference.id} has an invalid implementation status.`);
  }
  const runtimeCapabilities = stdioArray(definition.runtime_capabilities, `${artifactId}.runtime_capabilities`).map((capability, index) => {
    const record = stdioExactFields(capability, `${artifactId}.runtime_capabilities[${index}]`, ["id", "category", "descriptor", "claims"]);
    const category = stdioString(record.category, `${artifactId}.runtime_capabilities[${index}].category`);
    if (!["schema", "inference", "codec", "representation", "grammar", "composer", "subset-validator"].includes(category)) throw new Error(`[stdio] invalid runtime capability category ${category}.`);
    const capabilityId = stdioString(record.id, `${artifactId}.runtime_capabilities[${index}].id`);
    stdioVersionedLeaf(capabilityId, `${artifactId}.runtime.${category}.`);
    const descriptor = stdioString(record.descriptor, `${artifactId}.runtime_capabilities[${index}].descriptor`);
    const claims = stdioArray(record.claims, `${artifactId}.runtime_capabilities[${index}].claims`).map((claim, claimIndex) => {
      const claimRecord = stdioExactFields(claim, `${artifactId}.runtime_capabilities[${index}].claims[${claimIndex}]`, ["namespace", "value"]);
      const namespace = stdioString(claimRecord.namespace, `${artifactId}.runtime_capabilities[${index}].claims[${claimIndex}].namespace`);
      if (!["schema", "codec", "extension", "mime", "dialect", "grammar"].includes(namespace)) throw new Error(`[stdio] invalid runtime claim namespace ${namespace}.`);
      return { namespace, value: stdioString(claimRecord.value, `${artifactId}.runtime_capabilities[${index}].claims[${claimIndex}].value`) };
    });
    if (claims.length === 0) throw new Error(`[stdio] runtime capability ${capabilityId} has no claims.`);
    stdioAssertUnique(`runtime claim for ${capabilityId}`, claims.map((claim) => `${claim.namespace}:${claim.value}`));
    return { id: capabilityId, category: category as StdioRuntimeCapability["category"], descriptor, claims } satisfies StdioRuntimeCapability;
  });
  stdioAssertUnique(`${artifactId} runtime capability`, runtimeCapabilities.map((capability) => capability.id));
  stdioAssertUnique(`${artifactId} runtime category claims`, runtimeCapabilities.map((capability) => `${capability.category}|${capability.claims.map((claim) => `${claim.namespace}:${claim.value}`).sort().join("|")}`));
  for (const capability of runtimeCapabilities.filter((capability) => capability.category === "representation")) {
    const claims = capability.claims.map((claim) => `${claim.namespace}:${claim.value}`).sort().join("|");
    if (representations.filter((representation) => [...representation.mimes.map((mime) => `mime:${mime}`), ...representation.extensions.map((extension) => `extension:${extension}`)].sort().join("|") === claims).length === 0) throw new Error(`[stdio] runtime representation ${capability.id} does not claim a representation leaf.`);
  }
  const resources = stdioArray(definition.resources, `${artifactId}.resources`).map((resource, index) => {
    const record = stdioExactFields(resource, `${artifactId}.resources[${index}]`, ["id", "external_reference_policy", "status"]);
    stdioString(record.id, `${artifactId}.resources[${index}].id`);
    stdioString(record.external_reference_policy, `${artifactId}.resources[${index}].external_reference_policy`);
    stdioString(record.status, `${artifactId}.resources[${index}].status`);
    return record as unknown as StdioSchemaDefinition["resources"][number];
  });
  const descriptors = stdioArray(definition.localized_descriptors, `${artifactId}.localized_descriptors`).map((descriptor, index) => {
    const record = stdioExactFields(descriptor, `${artifactId}.localized_descriptors[${index}]`, ["id", "locale", "name", "description", "status"]);
    const locale = stdioString(record.locale, `${artifactId}.localized_descriptors[${index}].locale`);
    if (locale !== "en" && locale !== "de") throw new Error(`[stdio] ${artifactId} has unsupported locale ${locale}.`);
    stdioString(record.id, `${artifactId}.localized_descriptors[${index}].id`);
    stdioString(record.name, `${artifactId}.localized_descriptors[${index}].name`);
    stdioString(record.description, `${artifactId}.localized_descriptors[${index}].description`);
    stdioString(record.status, `${artifactId}.localized_descriptors[${index}].status`);
    return record as unknown as StdioSchemaDefinition["localized_descriptors"][number];
  });
  const expectedLocales = ["en", "de"];
  if (descriptors.length !== expectedLocales.length || descriptors.some((descriptor) => descriptor.id !== `${artifactId}.localization.${descriptor.locale}`) || expectedLocales.some((locale) => descriptors.filter((descriptor) => descriptor.locale === locale).length !== 1)) throw new Error(`[stdio] ${artifactId} must own exactly one English and German descriptor.`);
  const suites = stdioArray(definition.conformance_suites, `${artifactId}.conformance_suites`).map((suite, index) => {
    const record = stdioExactFields(suite, `${artifactId}.conformance_suites[${index}]`, ["id", "status", "fixtures"]);
    stdioString(record.id, `${artifactId}.conformance_suites[${index}].id`);
    stdioString(record.status, `${artifactId}.conformance_suites[${index}].status`);
    stdioStringArray(record.fixtures, `${artifactId}.conformance_suites[${index}].fixtures`);
    return record as unknown as StdioSchemaDefinition["conformance_suites"][number];
  });
  if (resources.length === 0 || suites.length === 0) throw new Error(`[stdio] ${artifactId} must own resources and conformance suites.`);
  const ledger = stdioExactFields(definition.support_ledger, `${artifactId}.support_ledger`, ["normative_source", "publication_date", "source_checksum", "redistribution_status", "clauses_or_features", "profiles", "registered_code_points", "read", "write", "lossless", "canonical", "validators", "mutations", "inferences", "fixtures"]);
  stdioNullableString(ledger.normative_source, `${artifactId}.support_ledger.normative_source`);
  stdioNullableString(ledger.publication_date, `${artifactId}.support_ledger.publication_date`);
  stdioNullableString(ledger.source_checksum, `${artifactId}.support_ledger.source_checksum`);
  stdioString(ledger.redistribution_status, `${artifactId}.support_ledger.redistribution_status`);
  for (const field of ["clauses_or_features", "profiles", "registered_code_points", "validators", "mutations", "inferences", "fixtures"] as const) stdioStringArray(ledger[field], `${artifactId}.support_ledger.${field}`);
  if (!ledger || !["unimplemented", "opaque", "implemented"].includes(ledger.read) || !["unimplemented", "opaque", "implemented"].includes(ledger.write) || !["unimplemented", "opaque", "implemented"].includes(ledger.lossless) || !["unimplemented", "opaque", "implemented"].includes(ledger.canonical)) throw new Error(`[stdio] ${definition.id} has an invalid support ledger.`);
  const implemented = [ledger.read, ledger.write, ledger.lossless, ledger.canonical].includes("implemented");
  if (implemented && (!ledger.normative_source || !ledger.publication_date || !ledger.source_checksum || ledger.redistribution_status === "unknown" || ledger.clauses_or_features.length === 0 || ledger.validators.length === 0 || ledger.fixtures.length === 0)) throw new Error(`[stdio] ${artifactId} claims implemented support without normative, validator, and fixture evidence.`);
  const IDs = [
    id, ...standards.map((item) => item.id), ...profiles.map((item) => item.id), ...dialects.map((item) => item.id), ...representations.map((item) => item.id), ...codecs.map((item) => item.id), ...mutations.map((item) => item.id), ...inferences.map((item) => item.id), ...resources.map((item) => item.id), ...descriptors.map((item) => item.id), ...suites.map((item) => item.id), ...runtimeCapabilities.map((item) => item.id),
  ];
  for (const id of IDs) stdioDefinitionIdentity(id);
  stdioAssertUnique(`definition identity for ${artifactId}`, IDs);
}

function stdioArtifactLedger(workspaceRoot: string): StdioArtifactLedger {
  const definitions = stdioDefinitionCatalog(workspaceRoot);
  stdioAssertUnique("artifact identity", definitions.map((definition) => definition.id));
  stdioAssertUnique("artifact directory", definitions.map((definition) => definition.directory));
  const knownArtifacts = new Set(definitions.map((definition) => definition.id));
  const knownDialectIds = new Set(definitions.flatMap((definition) => definition.source_dialects.map((dialect) => dialect.id)));
  const knownIdentityIds = new Set(definitions.flatMap((definition) => [
    definition.id,
    ...definition.standards.map((item) => item.id),
    ...definition.profiles.map((item) => item.id),
    ...definition.source_dialects.map((item) => item.id),
    ...definition.representations.map((item) => item.id),
    ...definition.codecs.map((item) => item.id),
    ...definition.mutations.map((item) => item.id),
    ...definition.inferences.map((item) => item.id),
    ...definition.runtime_capabilities.map((item) => item.id),
    ...definition.resources.map((item) => item.id),
    ...definition.localized_descriptors.map((item) => item.id),
    ...definition.conformance_suites.flatMap((item) => [item.id, ...item.fixtures]),
  ]));
  const equalIdSets = (left: readonly string[], right: readonly string[]): boolean => [...new Set(left)].sort().join("\n") === [...new Set(right)].sort().join("\n");
  for (const definition of definitions) {
    stdioAssertUnique(`dependency for ${definition.id}`, definition.dependencies);
    for (const dependency of definition.dependencies) {
      if (!knownArtifacts.has(dependency) || dependency === definition.id) throw new Error(`[stdio] ${definition.id} has an invalid dependency ${dependency}.`);
    }
    const localIds = new Set([
      definition.id,
      ...definition.standards.map((item) => item.id),
      ...definition.profiles.map((item) => item.id),
      ...definition.source_dialects.map((item) => item.id),
      ...definition.representations.map((item) => item.id),
      ...definition.codecs.map((item) => item.id),
      ...definition.mutations.map((item) => item.id),
      ...definition.inferences.map((item) => item.id),
      ...definition.runtime_capabilities.map((item) => item.id),
      ...definition.resources.map((item) => item.id),
      ...definition.localized_descriptors.map((item) => item.id),
      ...definition.conformance_suites.flatMap((item) => [item.id, ...item.fixtures]),
    ]);
    for (const fixture of definition.conformance_suites.flatMap((suite) => suite.fixtures)) {
      stdioDefinitionIdentity(fixture);
      if (!localIds.has(fixture)) throw new Error(`[stdio] ${definition.id} fixture ${fixture} does not resolve locally.`);
    }
    const ledger = definition.support_ledger;
    const declaredCodePoints = definition.source_dialects.flatMap((dialect) => dialect.registered_code_points);
    if (!equalIdSets(ledger.profiles, definition.profiles.map((profile) => profile.id))) throw new Error(`[stdio] ${definition.id} support ledger profiles diverge from schema profiles.`);
    if (!equalIdSets(ledger.registered_code_points, declaredCodePoints)) throw new Error(`[stdio] ${definition.id} support ledger code points diverge from source dialects.`);
    if (!equalIdSets(ledger.mutations, definition.mutations.map((mutation) => mutation.id))) throw new Error(`[stdio] ${definition.id} support ledger mutations diverge from definitions.`);
    if (!equalIdSets(ledger.inferences, definition.inferences.map((inference) => inference.id))) throw new Error(`[stdio] ${definition.id} support ledger inferences diverge from definitions.`);
    if (!equalIdSets(ledger.fixtures, definition.conformance_suites.flatMap((suite) => suite.fixtures))) throw new Error(`[stdio] ${definition.id} support ledger fixtures diverge from conformance suites.`);
    for (const reference of [...ledger.validators, ...ledger.mutations, ...ledger.inferences, ...ledger.fixtures]) {
      if (!localIds.has(reference) || !knownIdentityIds.has(reference)) throw new Error(`[stdio] ${definition.id} support ledger reference ${reference} does not resolve.`);
    }
    for (const codec of definition.codecs) {
      if (!codec.from || !codec.to || !knownDialectIds.has(codec.from) || !knownDialectIds.has(codec.to)) throw new Error(`[stdio] ${definition.id} codec ${codec.id} references an unknown source dialect.`);
    }
  }
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const definitionsById = new Map(definitions.map((definition) => [definition.id, definition]));
  const visit = (id: string): void => {
    if (visiting.has(id)) throw new Error(`[stdio] cyclic definition dependency at ${id}.`);
    if (visited.has(id)) return;
    visiting.add(id);
    definitionsById.get(id)!.dependencies.forEach(visit);
    visiting.delete(id);
    visited.add(id);
  };
  definitions.forEach((definition) => visit(definition.id));
  const artifacts = definitions.map((definition) => ({
      id: definition.artifact,
      kind: definition.id,
      directory: definition.directory,
      depends: definition.dependencies.map((dependency) => dependency.slice("s.stdio.".length)),
      component: stdioRel(workspaceRoot, join(workspaceRoot, STDIO_ROOT_REL, STDIO_ARTIFACTS_DIR, definition.directory, STDIO_COMPONENT_TS)),
      dialects: definition.source_dialects.map((dialect) => ({ id: dialect.id, artifact: definition.artifact, standard: dialect.standard, subset: dialect.dialect, schema: "", io: "", mutations: null, inferences: null })),
      representations: definition.representations,
      codecs: [...definition.codecs],
      support: definition.support_ledger,
    } satisfies StdioArtifactDefinition));
  const representations = definitions.flatMap((definition) => definition.representations);
  for (const [label, claims] of [["registered MIME", representations.flatMap((representation) => representation.mimes.map((mime) => [mime, definitions.find((definition) => definition.representations.includes(representation))!.id] as const))], ["file extension", representations.flatMap((representation) => representation.extensions.map((extension) => [extension, definitions.find((definition) => definition.representations.includes(representation))!.id] as const))]] as const) {
    const owners = new Map<string, string>();
    for (const [claim, owner] of claims) {
      const existing = owners.get(claim);
      if (existing && existing !== owner) throw new Error(`[stdio] ${label} ${claim} is claimed by both ${existing} and ${owner}.`);
      owners.set(claim, owner);
    }
  }
  stdioAssertUnique("source dialect", artifacts.flatMap((artifact) => artifact.dialects.map((dialect) => dialect.id)));
  stdioAssertUnique("codec", artifacts.flatMap((artifact) => artifact.codecs.map((codec) => codec.id)));
  stdioAssertUnique("runtime capability", definitions.flatMap((definition) => definition.runtime_capabilities.map((capability) => capability.id)));
  const epw = artifacts.find((artifact) => artifact.id === "epw");
  if (!epw || epw.representations.some((representation) => representation.mimes.length !== 0)) throw new Error("[stdio] EPW must remain MIME-unregistered.");
  const txt = artifacts.find((artifact) => artifact.id === "txt");
  if (!txt || !txt.representations.some((representation) => representation.mimes.includes("text/plain"))) throw new Error("[stdio] TXT must own text/plain.");
  return {
    artifacts,
    counts: {
      artifacts: artifacts.length,
      registeredMimes: representations.reduce((count, representation) => count + representation.mimes.length, 0),
      standards: definitions.reduce((count, definition) => count + definition.standards.length, 0),
      profiles: definitions.reduce((count, definition) => count + definition.profiles.length, 0),
      dialects: definitions.reduce((count, definition) => count + definition.source_dialects.length, 0),
      representations: definitions.reduce((count, definition) => count + definition.representations.length, 0),
      codecs: definitions.reduce((count, definition) => count + definition.codecs.length, 0),
      mutations: definitions.reduce((count, definition) => count + definition.mutations.length, 0),
      inferences: definitions.reduce((count, definition) => count + definition.inferences.length, 0),
      conformanceSuites: definitions.reduce((count, definition) => count + definition.conformance_suites.length, 0),
      runtimeCapabilities: definitions.reduce((count, definition) => count + definition.runtime_capabilities.length, 0),
      capabilities: {
        declared: definitions.reduce((count, definition) => count + definition.codecs.length + definition.mutations.length + definition.inferences.length, 0),
        registered: definitions.reduce((count, definition) => count + definition.codecs.filter((item) => item.executable_registration).length + definition.mutations.filter((item) => item.executable_registration).length + definition.inferences.filter((item) => item.executable_registration).length, 0),
        implemented: definitions.reduce((count, definition) => count + definition.codecs.filter((item) => item.status === "implemented").length + definition.mutations.filter((item) => item.status === "implemented").length + definition.inferences.filter((item) => item.status === "implemented").length, 0),
        verified: definitions.reduce((count, definition) => count + definition.codecs.filter((item) => item.status === "verified").length + definition.mutations.filter((item) => item.status === "verified").length + definition.inferences.filter((item) => item.status === "verified").length, 0),
      },
    },
  };
}

/** 🗄️ Runs schema-derived stdio catalog, support-ledger, and runtime gates. */
function stdioAssertTypeScriptExports(workspaceRoot: string, ledger: StdioArtifactLedger): void {
  const barrel = readFileSync(join(workspaceRoot, STDIO_ROOT_REL, "📦️packages", "🟦️typescript", "📦️index.ts"), "utf8");
  const exports = [...barrel.matchAll(/^export \* as (\w+) from /gmu)].map((match) => match[1]!);
  stdioAssertUnique("TypeScript export", exports);
  const expected = ledger.artifacts.map((artifact) => artifact.id).sort();
  if (exports.sort().join("\n") !== expected.join("\n")) throw new Error("[stdio] TypeScript exports are not definition-complete.");
  for (const artifact of ledger.artifacts) {
    const component = readFileSync(join(workspaceRoot, artifact.component), "utf8");
    if (!component.includes("📜️artifact-definition.json") || !component.includes("export { definition }")) throw new Error(`[stdio] ${artifact.id} exports a namespace instead of its schema-owned definition.`);
  }
}

function stdioAssertManifestAssembly(workspaceRoot: string, ledger: StdioArtifactLedger): void {
  const manifest = readFileSync(join(workspaceRoot, STDIO_ROOT_REL, "🛂️manifest", STDIO_COMPONENT_RS), "utf8");
  if (!manifest.includes("crate::registry::format_descriptors()")) throw new Error("[stdio] manifest must derive descriptors from schema-owned definitions.");
  if (!manifest.includes("crate::registry::artifact_definitions()")) throw new Error("[stdio] manifest must expose the schema-owned artifact definition assembly.");
  const epw = ledger.artifacts.find((artifact) => artifact.id === "epw");
  if (!epw || epw.representations.some((representation) => representation.mimes.length !== 0)) throw new Error("[stdio] EPW must remain MIME-unregistered in the schema ledger.");
}

function stdioRequireClosed(ledger: StdioArtifactLedger, gate: string): never {
  const incomplete = ledger.artifacts.filter((artifact) => artifact.support.read !== "implemented" || artifact.support.write !== "implemented" || artifact.support.lossless !== "implemented" || artifact.support.canonical !== "implemented" || artifact.codecs.length === 0);
  throw new Error(`[stdio] ${gate} is not closed: ${incomplete.length} schema-owned support ledgers remain unimplemented; no runtime, fuzz, or cross-platform support is claimed.`);
}

function stdioRunStructuralGate(workspaceRoot: string, gate: "quick" | "schema-parity" | "standards-coverage" | "codec" | "mutation-law" | "inference"): StdioArtifactLedger {
  const ledger = stdioArtifactLedger(workspaceRoot);
  stdioAssertTypeScriptExports(workspaceRoot, ledger);
  stdioAssertManifestAssembly(workspaceRoot, ledger);
  if (gate !== "quick") stdioRequireClosed(ledger, gate);
  return ledger;
}

export class StdioScript extends Script {
  run(segments: string[]): void {
    const gate = segments[0] ?? "quick";
    if (gate === "ledger") {
      process.stdout.write(`${JSON.stringify(stdioArtifactLedger(this.root), null, 2)}\n`);
      return;
    }
    if (gate === "quick" || gate === "schema-parity" || gate === "standards-coverage" || gate === "codec" || gate === "mutation-law" || gate === "inference") {
      const ledger = stdioRunStructuralGate(this.root, gate);
      console.log(`[stdio] ${gate} passed (${ledger.counts.artifacts} artifacts, ${ledger.counts.dialects} dialects, ${ledger.counts.codecs} codecs).`);
      return;
    }
    if (gate === "long") {
      for (const structuralGate of ["quick", "schema-parity", "standards-coverage", "codec", "mutation-law", "inference"] as const) stdioRunStructuralGate(this.root, structuralGate);
      runCmd("bun", ["nx", "run-many", "-t", "test-long", "-p", "@semio-tech/stdio-plugin", "@semio-tech/stdio-js"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    if (gate === "exhaustive") {
      this.run(["long"]);
      runCmd("bun", ["nx", "run-many", "-t", "test-exhaustive", "-p", "@semio-tech/stdio-plugin", "@semio-tech/stdio-js"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    if (gate === "runtime") {
      stdioRequireClosed(stdioRunStructuralGate(this.root, "quick"), "runtime");
      return;
    }
    if (gate === "fuzz") {
      stdioRequireClosed(stdioRunStructuralGate(this.root, "quick"), "fuzz");
      return;
    }
    if (gate === "cross-platform") {
      stdioRequireClosed(stdioRunStructuralGate(this.root, "quick"), "cross-platform");
      return;
    }
    throw new Error(`[stdio] expected ledger|quick|long|exhaustive|schema-parity|standards-coverage|codec|mutation-law|inference|runtime|fuzz|cross-platform, got ${JSON.stringify(gate)}.`);
  }
}
//#endregion 🔖️StdioLedgerScript

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
      mkdirSync(join(getRepoMetaDir(this.root), "⚡️cache"), { recursive: true });
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
    return process.env.VCPKG_ROOT || join(getRepoMetaDir(this.root), "⚡️cache", "vcpkg");
  }

  private ensureWindowsMsvc(): void {
    if (process.platform !== "win32") return;
    if (queryVisualStudio2026InstallPath()) return;
    console.error("[cpp] Visual Studio 2026 with the Desktop development with C++ workload is required.");
    console.error("[cpp] On native Windows run: bun ./📜️script.ts setup native");
    process.exit(1);
  }

  private purgeStaleCmakeCache(preset: string): void {
    const cacheDir = join(getRepoMetaDir(this.root), "⚡️cache", "cmake", preset);
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
  const registryPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json");
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
      const container = join(this.root, owner, taxonomy.artifactsDirName);
      if (existsSync(container)) {
        for (const child of readdirSync(container)) {
          const examplesRel = `${owner}/${taxonomy.artifactsDirName}/${child}/📚️examples`;
          const examplesAbs = join(this.root, examplesRel);
          if (!existsSync(examplesAbs) || !statSync(examplesAbs).isDirectory()) continue;
          for (const slug of readdirSync(examplesAbs)) {
            const slugRel = `${examplesRel}/${slug}`;
            if (!statSync(join(this.root, slugRel)).isDirectory()) continue;
            roots.push(slugRel);
          }
        }
      }
      // 👁️✏️ Surface examples nest under 🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/{👁️viewer,✏️editor}/,
      // deeper than the flat <kind>/<child> shape 🎛️apps used — walked via the shared surface finder.
      for (const surfaceRel of policySurfaceRoots(this.root, owner, taxonomy)) {
        const examplesRel = `${surfaceRel}/📚️examples`;
        const examplesAbs = join(this.root, examplesRel);
        if (!existsSync(examplesAbs) || !statSync(examplesAbs).isDirectory()) continue;
        for (const slug of readdirSync(examplesAbs)) {
          const slugRel = `${examplesRel}/${slug}`;
          if (!statSync(join(this.root, slugRel)).isDirectory()) continue;
          roots.push(slugRel);
        }
      }
    }
    return roots.sort();
  }
}
//#endregion 🔖️ExamplesScript

//#region 🔖️CleanMechanismNewScript
/**
 * 🏗️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, task 3 (corrected by W1-E per the
 * ⚠️ CORRECTION in design.md §1): permanent taxonomy-v6 scaffolders sibling to the framework
 * registry's `new surface` (📇️registry/📜️script.ts, ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET)
 * — `new artifact`/`new standard`/`new subset` generate the v6 skeleton design.md §1 describes: a
 * root component with mounts + declaration-fn stub, plus (subset only) 🧬️schema, 🚪️io with the
 * NATIVE codec dirs directly beneath it (unsplit — `import`/`export` are a FOREIGN-dialect-only
 * concept and are never scaffolded generically, since a generic scaffolder cannot know which foreign
 * dialect a not-yet-written subset will consume), 👁️viewer, ✏️editor, 📚️examples. Every generated
 * leaf carries the same `SCAFFOLD` marker convention `new surface` uses, so
 * `policyOwnerMountsChildrenBreaches`'s "missing-owner-root" check treats a freshly scaffolded root
 * as what it is — a placeholder, not a finished packet. Idempotent — never overwrites an existing
 * leaf, so re-running after hand-authoring never clobbers real content.
 */
const NEW_SCAFFOLD_MARKER = "SCAFFOLD";
const NEW_SCAFFOLD_TICKET_PATH = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM";

function newScaffoldRustLeaf(label: string): string {
  return `//! 🚧️ ${NEW_SCAFFOLD_MARKER}: ${label} — generated by \`bun ./📜️script.ts new …\`, not implemented.\n//! @see ${NEW_SCAFFOLD_TICKET_PATH}\npub const SCAFFOLD: bool = true;\n`;
}

function newScaffoldTsLeaf(label: string): string {
  return `// 🚧️ ${NEW_SCAFFOLD_MARKER}: ${label} — generated by \`bun ./📜️script.ts new …\`, not implemented.\n// @see ${NEW_SCAFFOLD_TICKET_PATH}\nexport const SCAFFOLD = true;\n`;
}

function newScaffoldEmptyFacetMarkdown(facetLabel: string): string {
  return `# Empty ${facetLabel} facet\n\nThis facet currently declares no specific items. Generated by \`bun ./📜️script.ts new …\`.\n`;
}

/** 📝️ Writes `relPath` only when absent; an existing leaf is reported as skipped, never overwritten. */
function newScaffoldWriteIfAbsent(repoRoot: string, relPath: string, content: string, created: string[], skipped: string[], dryRun: boolean): void {
  const abs = join(repoRoot, relPath);
  if (existsSync(abs)) {
    skipped.push(relPath);
    return;
  }
  created.push(relPath);
  if (dryRun) return;
  mkdirSync(dirname(abs), { recursive: true });
  writeFileSync(abs, content);
}

/** 🔎️ Resolves a bare CLI id to the real emoji-prefixed child directory name of `parentAbs`. */
function newResolveChildDir(parentAbs: string, wantStripped: string): string | undefined {
  if (!existsSync(parentAbs)) return undefined;
  for (const name of readdirSync(parentAbs)) {
    if (!statSync(join(parentAbs, name)).isDirectory()) continue;
    if (policyStripEmoji(name) === wantStripped) return name;
  }
  return undefined;
}

/** 🚪️ Scaffolds `${ioRel}` per the corrected design.md §1 shape: root leaves, then one dir per
 * `ioSemanticCollectionDirNames` member DIRECTLY under `${ioRel}` (native codec, unsplit — `import`/
 * `export` express direction and exist only for FOREIGN dialects, which this generic scaffolder
 * cannot know in advance and therefore never creates) — `representationDirs` children for
 * 📸️snapshot/🔺️diff, a wildcard-slug-ready empty facet marker for 🧬️mutations/💡️inferences (their real
 * content is per-mutation/per-inference emoji slugs, which `new subset` also cannot know in advance). */
function newScaffoldIoTree(repoRoot: string, ioRel: string, taxonomy: ReturnType<typeof loadTaxonomy>, created: string[], skipped: string[], dryRun: boolean): void {
  newScaffoldWriteIfAbsent(repoRoot, `${ioRel}/🦀️component.rs`, newScaffoldRustLeaf("io root (io() -> IoDeclaration stub)"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${ioRel}/🟦️component.ts`, newScaffoldTsLeaf("io root (IoEntryDescriptor[] mirror)"), created, skipped, dryRun);
  for (const kind of taxonomy.ioSemanticCollectionDirNames ?? []) {
    const kindRel = `${ioRel}/${kind}`;
    if (kind === "🧬️mutations" || kind === "💡️inferences") {
      newScaffoldWriteIfAbsent(repoRoot, `${kindRel}/${taxonomy.windowEmptyFacetFilename}`, newScaffoldEmptyFacetMarkdown(kind), created, skipped, dryRun);
      continue;
    }
    for (const rep of taxonomy.representationDirs ?? []) {
      newScaffoldWriteIfAbsent(repoRoot, `${kindRel}/${rep}/🦀️component.rs`, newScaffoldRustLeaf(`${kind}/${rep} native codec`), created, skipped, dryRun);
    }
  }
}

function newScaffoldSubsetTree(repoRoot: string, subsetRel: string, taxonomy: ReturnType<typeof loadTaxonomy>, dryRun: boolean): { created: string[]; skipped: string[] } {
  const created: string[] = [];
  const skipped: string[] = [];
  newScaffoldWriteIfAbsent(repoRoot, `${subsetRel}/🦀️component.rs`, newScaffoldRustLeaf("subset root (subset() -> SubsetDeclaration stub; mounts schema/io/viewer/editor/examples)"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${subsetRel}/🟦️component.ts`, newScaffoldTsLeaf("subset root"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${subsetRel}/🧬️schema/🦀️component.rs`, newScaffoldRustLeaf("schema root — own Snapshot/Diff/Mutation types, no codecs"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${subsetRel}/🧬️schema/🟦️component.ts`, newScaffoldTsLeaf("schema root"), created, skipped, dryRun);
  newScaffoldIoTree(repoRoot, `${subsetRel}/🚪️io`, taxonomy, created, skipped, dryRun);
  for (const role of taxonomy.surfaceRoles) {
    const surfaceRel = `${subsetRel}/${taxonomy.surfaceDirNames[role]}`;
    newScaffoldWriteIfAbsent(repoRoot, `${surfaceRel}/🦀️component.rs`, newScaffoldRustLeaf(`${role} surface`), created, skipped, dryRun);
    newScaffoldWriteIfAbsent(repoRoot, `${surfaceRel}/🟦️component.ts`, newScaffoldTsLeaf(`${role} surface`), created, skipped, dryRun);
  }
  newScaffoldWriteIfAbsent(repoRoot, `${subsetRel}/📚️examples/${taxonomy.windowEmptyFacetFilename}`, newScaffoldEmptyFacetMarkdown("examples"), created, skipped, dryRun);
  return { created, skipped };
}

function newScaffoldStandardTree(repoRoot: string, standardRel: string, dryRun: boolean): { created: string[]; skipped: string[] } {
  const created: string[] = [];
  const skipped: string[] = [];
  newScaffoldWriteIfAbsent(repoRoot, `${standardRel}/🦀️component.rs`, newScaffoldRustLeaf("standard root (standard() -> StandardDeclaration stub; mounts subsets)"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${standardRel}/🟦️component.ts`, newScaffoldTsLeaf("standard root"), created, skipped, dryRun);
  const manifest = { standard: policyStripEmoji(standardRel.split("/").pop() ?? ""), subsets: { "*": {} } };
  newScaffoldWriteIfAbsent(repoRoot, `${standardRel}/🪆️subsets/🔣️component.json`, `${JSON.stringify(manifest, null, 2)}\n`, created, skipped, dryRun);
  return { created, skipped };
}

function newScaffoldArtifactTree(repoRoot: string, artRel: string, dryRun: boolean): { created: string[]; skipped: string[] } {
  const created: string[] = [];
  const skipped: string[] = [];
  newScaffoldWriteIfAbsent(repoRoot, `${artRel}/🦀️component.rs`, newScaffoldRustLeaf("artifact root (artifact() -> ArtifactDeclaration stub; mounts standards)"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${artRel}/🟦️component.ts`, newScaffoldTsLeaf("artifact root"), created, skipped, dryRun);
  return { created, skipped };
}

/**
 * 🚪️ `new artifact|standard|subset` CLI — registered as `bun ./📜️script.ts new <kind> …` via
 * `ScriptRouter`. Existing path segments (plugin/artifact-kind/standard) are resolved the same
 * emoji-tolerant way `new surface` resolves them; the final, NEW segment is taken literally (it must
 * already carry the right emoji prefix and, for standard/subset, the taxonomy's dir prefix).
 */
class CleanMechanismNewScript extends Script {
  run(segments: string[]): void {
    const kind = segments[0];
    if (kind !== "subset" && kind !== "standard" && kind !== "artifact") {
      console.error("usage: bun ./📜️script.ts new artifact <plugin> <new-artifact-dir>");
      console.error("   or: bun ./📜️script.ts new standard <plugin> <artifact-kind> <new-standard-dir>");
      console.error("   or: bun ./📜️script.ts new subset <plugin> <artifact-kind> <standard> <new-subset-dir> [--dry-run]");
      process.exit(1);
      return;
    }
    const taxonomy = loadTaxonomy();
    const rest = segments.slice(1);
    const dryRun = rest.includes("--dry-run");
    const positional = rest.filter((a) => a !== "--dry-run");
    const repoRoot = this.root;
    const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");

    const resolveExisting = (parentAbs: string, arg: string, label: string): string => {
      const dir = newResolveChildDir(parentAbs, policyStripEmoji(arg));
      if (!dir) throw new Error(`new ${kind}: no ${label} "${arg}" under ${relative(repoRoot, parentAbs)}`);
      return dir;
    };

    try {
      if (kind === "artifact") {
        if (positional.length !== 2) throw new Error("usage: bun ./📜️script.ts new artifact <plugin> <new-artifact-dir>");
        const [pluginArg, newDir] = positional as [string, string];
        const pluginDir = resolveExisting(pluginsRoot, pluginArg, "plugin");
        const artifactsAbs = join(pluginsRoot, pluginDir, taxonomy.artifactsDirName);
        const artRel = relative(repoRoot, join(artifactsAbs, newDir)).replaceAll("\\", "/");
        const { created, skipped } = newScaffoldArtifactTree(repoRoot, artRel, dryRun);
        this.report(artRel, created, skipped, dryRun);
        return;
      }
      if (kind === "standard") {
        if (positional.length !== 3) throw new Error("usage: bun ./📜️script.ts new standard <plugin> <artifact-kind> <new-standard-dir>");
        const [pluginArg, artArg, newDir] = positional as [string, string, string];
        const pluginDir = resolveExisting(pluginsRoot, pluginArg, "plugin");
        const artifactsAbs = join(pluginsRoot, pluginDir, taxonomy.artifactsDirName);
        const artDir = resolveExisting(artifactsAbs, artArg, "artifact kind");
        const standardsAbs = join(artifactsAbs, artDir, taxonomy.standardsDirName);
        if (taxonomy.standardDirPrefix && !newDir.startsWith(taxonomy.standardDirPrefix)) throw new Error(`new standard: "${newDir}" must start with standardDirPrefix "${taxonomy.standardDirPrefix}"`);
        const standardRel = relative(repoRoot, join(standardsAbs, newDir)).replaceAll("\\", "/");
        const { created, skipped } = newScaffoldStandardTree(repoRoot, standardRel, dryRun);
        this.report(standardRel, created, skipped, dryRun);
        return;
      }
      if (positional.length !== 4) throw new Error("usage: bun ./📜️script.ts new subset <plugin> <artifact-kind> <standard> <new-subset-dir>");
      const [pluginArg, artArg, stdArg, newDir] = positional as [string, string, string, string];
      const pluginDir = resolveExisting(pluginsRoot, pluginArg, "plugin");
      const artifactsAbs = join(pluginsRoot, pluginDir, taxonomy.artifactsDirName);
      const artDir = resolveExisting(artifactsAbs, artArg, "artifact kind");
      const standardsAbs = join(artifactsAbs, artDir, taxonomy.standardsDirName);
      const standardDir = resolveExisting(standardsAbs, stdArg, "standard");
      const subsetsAbs = join(standardsAbs, standardDir, taxonomy.subsetsDirName);
      if (taxonomy.subsetDirPrefix && !newDir.startsWith(taxonomy.subsetDirPrefix)) throw new Error(`new subset: "${newDir}" must start with subsetDirPrefix "${taxonomy.subsetDirPrefix}"`);
      const subsetRel = relative(repoRoot, join(subsetsAbs, newDir)).replaceAll("\\", "/");
      const { created, skipped } = newScaffoldSubsetTree(repoRoot, subsetRel, taxonomy, dryRun);
      this.report(subsetRel, created, skipped, dryRun);
    } catch (error) {
      console.error((error as Error).message);
      process.exit(1);
    }
  }

  private report(rel: string, created: string[], skipped: string[], dryRun: boolean): void {
    const verb = dryRun ? "would create" : "created";
    console.log(`new: ${rel} — ${verb} ${created.length} file(s), ${skipped.length} already present.`);
    for (const p of created) console.log(`  ${dryRun ? "+ (dry-run)" : "+"} ${p}`);
  }
}
//#endregion 🔖️CleanMechanismNewScript

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
  .register(
    "scale-fixture",
    /** 🧫️ 50×50 scale fixture (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet F1) —
     * `check` regenerates the seeded registry/catalog and diffs, the same freshness idiom
     * `plugin-registry:check` uses. */
    class extends Script {
      run(segments: string[]): void {
        const sub = segments[0];
        if (sub === "check") {
          runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:scale-fixture-check"], { cwd: this.root });
          return;
        }
        throw new Error(`unknown scale-fixture subcommand: ${sub} (expected check)`);
      }
    },
  )
  .register("new", CleanMechanismNewScript)
  .register("lint", LintScript)
  .register("verify", VerifyScript)
  .register("format", FormatScript)
  .register("test", TestScript)
  .register("stdio", StdioScript)
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
 * 🛂️ Neo4j → `.🧬semio/🦑️repo/🛂️manifest/<graph>.cypher` export (pure module; invoked from root `script.ts`). Product graphs are fixed specs; extra Bolt graphs use `NEO4J_EXTRA_GRAPH_DATABASES` (comma-separated). Argv segments join with `-` via `joinNeo4jGraphDatabaseName`.
 */
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const NEO4J_VERSION = "5.26.26";

/** 🏗️Product graphs only (compose stack); not arbitrary developer databases. */
export const NEO4J_PRODUCT_GRAPH_DATABASE_SPECS = [["compose"], ["elements"], ["coda"], ["reuse"]] as const;

/** 🗑️Env key: comma-separated extra Bolt graph names for `bun run generate` and native `.🧬semio/🦑️repo/🛂️manifest/*.cypher` stubs. */
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
    const cachedShell = join(getRepoMetaDir(this.repoRoot), "⚡️cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
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

    const queryDir = join(getRepoMetaDir(this.repoRoot), "⚡️cache");
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
    const outDir = join(getRepoMetaDir(this.repoRoot), "🛂️manifest");
    mkdirSync(outDir, { recursive: true });

    const finalAbs = join(outDir, `${technology}.cypher`);
    const cacheDir = join(getRepoMetaDir(this.repoRoot), "⚡️cache");
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
 * `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin.mjs` into the synthetic `breach-script_ts` nx lint target (`bun ./📜️script.ts policy`).
 * Judgment-call findings (a real SDK/primitive gap, e.g. the terminology native/reuse Labels axis, or
 * puzzle's icon-based `tree_item_with_action`) are encoded as explicit low-priority allowlisted/tracked
 * breaches, never as a hard `policy` failure — see `POLICY_SDK_GAP_ALLOWLIST` below.
 */

//#region 🔧️PolicyFsScan
const POLICY_SKIP_DIRS = new Set(["node_modules", ".git", ".🧬semio", "target", "dist", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);

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
  // seeded by W1b scaffold, burn down as W2/W3 land real 🧰️triples-backed sparse diffs (see
  // w1b-scaffold-manifest.md §6 "Diff/Mutation shape" — a full-replace scaffold Diff today, all
  // 8 laws pass, but no real DiffCodec impl yet). Keys computed via policyNormalizeRelPath.
  // W3 closer (26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) removed
  // html/epw/mp3/tsv/wav — each now carries a real hand-rolled `impl protocol::DiffCodec for
  // <X>Diff` in the same file (verified by grep before removal, policy re-run green after). mp4/avi
  // stay: their `MutationDiff`/`DiffAlgebra` impls exist but neither file has a `DiffCodec` impl —
  // a real, still-open gap, not yet satisfied.
  "stdio/mp4/standards#isobmff-subsets-any-schema-diff-component",
  "stdio/avi/standards#1.0-subsets-any-schema-diff-component",
  // W2a's 6 subsets (brep/cad/drawing/mesh/model/object) removed by the W2a closer
  // (26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) — each verified to
  // now carry a real hand-rolled `impl protocol::DiffCodec for Semio<X>Diff` in the same file
  // (grepped before removal, `policyDiffCompletenessBreaches` re-run green after; see
  // `w2a-close-report.md`). W2b/W3 subsets are untouched here.
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
/** 🧩️A COMPOSITE mutation owns its payload plus a plan; its diff and inverse are folded from that plan, so it owns neither `🔺️diff` nor `↩️inverse`. */
const POLICY_MUTATION_COMPOSITE_DIRS = ["🦠️mutation", "🧩️plan"] as const;
const POLICY_MUTATION_PLAN_DIR = "🧩️plan";
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
    const text = policyReadFileSafe(repoRoot, relPath);
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
  const files = paths.map((relPath) => ({ relPath, content: policyReadFileSafe(repoRoot, relPath) }));
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const text = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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

    const content = policyReadFileSafe(repoRoot, relPath);
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

/** 🔎️Repo-wide `script.ts` file paths (repo-relative), skipping node_modules/target/.🧬semio and other policy skip dirs. */
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
 * 📖️Reads a repo-relative file, returning `""` when it has vanished between the directory walk and
 * the read. Sibling of [[policyReaddirSafe]].
 *
 * Every rule here walks a file list and then reads each entry, which races against concurrent
 * sessions deleting files mid-run — a real crash, not a hypothetical: `policyDeclarativeRegistration`
 * died on `ENOENT` while artifact `⚙️engine/` directories were being dissolved beneath it, taking the
 * whole `policy` command down for every session. `""` rather than a skip is deliberate and correct:
 * these rules pattern-match content to find violations, and **a file that no longer exists has no
 * violations in it**, so empty content yields the right answer with no call-site restructuring.
 */
function policyReadFileSafe(repoRoot: string, ...parts: string[]): string {
  try {
    return readFileSync(join(repoRoot, ...parts), "utf8");
  } catch {
    return "";
  }
}

/**
 * 👁️✏️ Every `👁️viewer`/`✏️editor` surface root that actually exists under an owner's
 * `🗿️artifacts/<a>/🏅️standards/🔖️<s>/🪆️subsets/<sub>/` tree — the W3 dissolution replacement for the
 * old single `🎛️apps` root. Shared by every walker that used to start at `taxonomy.appsDirName`.
 */
function policySurfaceRoots(repoRoot: string, ownerRoot: string, taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const roots: string[] = [];
  const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
  for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
    const standardsRoot = `${artifactsRoot}/${artifact.name}/${taxonomy.standardsDirName}`;
    for (const standard of policyReaddirSafe(repoRoot, standardsRoot).filter((e) => e.isDirectory)) {
      const subsetsRoot = `${standardsRoot}/${standard.name}/${taxonomy.subsetsDirName}`;
      for (const subset of policyReaddirSafe(repoRoot, subsetsRoot).filter((e) => e.isDirectory)) {
        for (const role of taxonomy.surfaceRoles) {
          const dirName = taxonomy.surfaceDirNames[role];
          const surfaceRoot = `${subsetsRoot}/${subset.name}/${dirName}`;
          if (existsSync(join(repoRoot, surfaceRoot))) roots.push(surfaceRoot);
        }
      }
    }
  }
  return roots;
}

/**
 * 📏️Taxonomy validator, discovery-contract clause 1: every `🗿️artifacts/<a>/` may only contain the known
 * artifact child vocabulary (`taxonomy.artifactChildDirs`, plus its own leaf file), every `🏅️standards/`
 * descends into `🪆️subsets/<s>/` (`taxonomy.subsetChildDirs`, including the two surfaces), and every
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
    const schemaChildDirs = (taxonomy as { schemaChildDirs?: string[] }).schemaChildDirs ?? ["📸️snapshot", "🔺️diff", "🧬️mutations"];
    const representationDirs = (taxonomy as { representationDirs?: string[] }).representationDirs ?? ["📝️text", "💾️binary"];
    const ioDirectionDirs = (taxonomy as { ioDirectionDirs?: string[] }).ioDirectionDirs ?? ["📥️import", "📤️export"];
    const ioDirectionChildDirs = (taxonomy as { ioDirectionChildDirs?: Record<string, string> }).ioDirectionChildDirs ?? {"📥️import": "🧩️deserializers", "📤️export": "🧵️serializers"};
    const recognizedArtifactChildDirs = [...taxonomy.artifactChildDirs, ...((taxonomy as { newArtifactChildDirs?: string[] }).newArtifactChildDirs ?? [])];

    //#region NestedFacetWalk
    const validateSchemaFacet = (nestedRoot: string, breachScope: string): void => {
      for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
        if (schemaChildDirs.includes(nested.name)) {
          const repRoot = `${nestedRoot}/${nested.name}`;
          for (const rep of policyReaddirSafe(repoRoot, repRoot).filter((e) => e.isDirectory)) {
            if (representationDirs.includes(rep.name)) continue;
            if (nested.name === "🧬️mutations") {
              // wildcard mutation slug under schema/mutations
              continue;
            }
            breaches.push({
              id: `taxonomy-dirs-artifact-${repRoot}-${rep.name}`,
              summary: `"${repRoot}/${rep.name}" is not a recognized representation dir`,
              kind: "taxonomy/dirs",
              scope: breachScope,
              priority: policyNewSurfacePriority(crate, "medium"),
              reason: `Discovery contract: 🧬️schema/<snapshot|diff|mutations> may contain ${representationDirs.join(", ")} (mutations also allow emoji-slug dirs).`,
              solution: `Move "${rep.name}" into a representationDirs member or a mutation slug.`,
            });
          }
          continue;
        }
        breaches.push({
          id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
          summary: `"${nestedRoot}/${nested.name}" is not a recognized schema child dir`,
          kind: "taxonomy/dirs",
          scope: breachScope,
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: `Discovery contract: 🧬️schema may only contain ${schemaChildDirs.join(", ")}.`,
          solution: `Move "${nested.name}" into a schemaChildDirs member.`,
        });
      }
    };
    const validateIoFacet = (nestedRoot: string, breachScope: string): void => {
      for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
        if (!ioDirectionDirs.includes(nested.name)) {
          breaches.push({
            id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
            summary: `"${nestedRoot}/${nested.name}" is not a recognized io direction dir`,
            kind: "taxonomy/dirs",
            scope: breachScope,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: 🚪️io may only contain ${ioDirectionDirs.join(", ")}.`,
            solution: `Rename "${nested.name}" to an ioDirectionDirs member.`,
          });
          continue;
        }
        const expectedChild = ioDirectionChildDirs[nested.name];
        const directionRoot = `${nestedRoot}/${nested.name}`;
        for (const codec of policyReaddirSafe(repoRoot, directionRoot).filter((e) => e.isDirectory)) {
          if (expectedChild && codec.name === expectedChild) {
            const codecRoot = `${directionRoot}/${codec.name}`;
            for (const mid of policyReaddirSafe(repoRoot, codecRoot).filter((e) => e.isDirectory)) {
              if (mid.name !== taxonomy.artifactsDirName) {
                breaches.push({
                  id: `taxonomy-dirs-artifact-${codecRoot}-${mid.name}`,
                  summary: `"${codecRoot}/${mid.name}" is not ${taxonomy.artifactsDirName}`,
                  kind: "taxonomy/dirs",
                  scope: breachScope,
                  priority: policyNewSurfacePriority(crate, "medium"),
                  reason: `Discovery contract: deserializers/serializers nest under ${taxonomy.artifactsDirName}/<stdio-artifact>.`,
                  solution: `Move "${mid.name}" under ${taxonomy.artifactsDirName}/.`,
                });
                continue;
              }
              // wildcard stdio artifact dirs under artifacts/
            }
            continue;
          }
          breaches.push({
            id: `taxonomy-dirs-artifact-${directionRoot}-${codec.name}`,
            summary: `"${directionRoot}/${codec.name}" is not the declared io direction child`,
            kind: "taxonomy/dirs",
            scope: breachScope,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: ${nested.name} must contain ${expectedChild}.`,
            solution: `Rename "${codec.name}" to ${expectedChild}.`,
          });
        }
      }
    };
    //#endregion NestedFacetWalk

    for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
      const artifactDir = `${artifactsRoot}/${artifact.name}`;
      const artifactScope = `${scopeId}/${policyStripEmoji(artifact.name)}`;
      for (const child of policyReaddirSafe(repoRoot, artifactDir).filter((e) => e.isDirectory)) {
        if (recognizedArtifactChildDirs.includes(child.name)) {
          if (child.name === "🧬️schema") {
            validateSchemaFacet(`${artifactDir}/${child.name}`, artifactScope);
          } else if (child.name === "🚪️io") {
            validateIoFacet(`${artifactDir}/${child.name}`, artifactScope);
          } else if (child.name === taxonomy.standardsDirName) {
            //#region SubsetFacetWalk
            const standardsRoot = `${artifactDir}/${child.name}`;
            for (const standard of policyReaddirSafe(repoRoot, standardsRoot).filter((e) => e.isDirectory)) {
              const standardDir = `${standardsRoot}/${standard.name}`;
              for (const standardChild of policyReaddirSafe(repoRoot, standardDir).filter((e) => e.isDirectory)) {
                if (!(taxonomy.standardChildDirs ?? []).includes(standardChild.name)) {
                  breaches.push({
                    id: `taxonomy-dirs-standard-${standardDir}-${standardChild.name}`,
                    summary: `"${standardDir}/${standardChild.name}" is not a recognized standard child dir`,
                    kind: "taxonomy/dirs",
                    scope: artifactScope,
                    priority: policyNewSurfacePriority(crate, "medium"),
                    reason: `Discovery contract: a standard dir may only contain ${(taxonomy.standardChildDirs ?? []).join(", ")}.`,
                    solution: `Move "${standardChild.name}" into a recognized standardChildDirs member.`,
                  });
                  continue;
                }
                if (standardChild.name !== taxonomy.subsetsDirName) continue;
                const subsetsRoot = `${standardDir}/${standardChild.name}`;
                for (const subset of policyReaddirSafe(repoRoot, subsetsRoot).filter((e) => e.isDirectory)) {
                  const subsetDir = `${subsetsRoot}/${subset.name}`;
                  const subsetScope = `${artifactScope}/${policyStripEmoji(subset.name)}`;
                  for (const subsetChild of policyReaddirSafe(repoRoot, subsetDir).filter((e) => e.isDirectory)) {
                    if (!taxonomy.subsetChildDirs.includes(subsetChild.name)) {
                      breaches.push({
                        id: `taxonomy-dirs-subset-${subsetDir}-${subsetChild.name}`,
                        summary: `"${subsetDir}/${subsetChild.name}" is not a recognized subset child dir`,
                        kind: "taxonomy/dirs",
                        scope: subsetScope,
                        priority: policyNewSurfacePriority(crate, "medium"),
                        reason: `Discovery contract: a subset dir may only contain ${taxonomy.subsetChildDirs.join(", ")}.`,
                        solution: `Move "${subsetChild.name}" into a recognized subsetChildDirs member.`,
                      });
                      continue;
                    }
                    if (subsetChild.name === "🧬️schema") {
                      validateSchemaFacet(`${subsetDir}/${subsetChild.name}`, subsetScope);
                    } else if (subsetChild.name === "🚪️io") {
                      validateIoFacet(`${subsetDir}/${subsetChild.name}`, subsetScope);
                    } else if (Object.values(taxonomy.surfaceDirNames ?? {}).includes(subsetChild.name)) {
                      const surfaceDir = `${subsetDir}/${subsetChild.name}`;
                      for (const surfaceChild of policyReaddirSafe(repoRoot, surfaceDir).filter((e) => e.isDirectory)) {
                        if (taxonomy.surfaceChildDirs.includes(surfaceChild.name)) continue;
                        breaches.push({
                          id: `taxonomy-dirs-surface-${surfaceDir}-${surfaceChild.name}`,
                          summary: `"${surfaceDir}/${surfaceChild.name}" is not a recognized surface child dir`,
                          kind: "taxonomy/dirs",
                          scope: subsetScope,
                          priority: policyNewSurfacePriority(crate, "medium"),
                          reason: `Discovery contract: a surface dir may only contain ${taxonomy.surfaceChildDirs.join(", ")}.`,
                          solution: `Move "${surfaceChild.name}" into a recognized surfaceChildDirs member.`,
                        });
                      }
                    }
                  }
                }
              }
            }
            //#endregion SubsetFacetWalk
          }
          continue;
        }
        breaches.push({
          id: `taxonomy-dirs-artifact-${artifactDir}-${child.name}`,
          summary: `"${artifactDir}/${child.name}" is not a recognized artifact component dir`,
          kind: "taxonomy/dirs",
          scope: artifactScope,
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: `Discovery contract: an artifact dir may only contain ${recognizedArtifactChildDirs.join(", ")}.`,
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
    for (const surfaceRoot of policySurfaceRoots(repoRoot, ownerRoot, taxonomy)) walkForWindows(surfaceRoot);
  }
  return breaches;
}

/**
 * 📏️Window completeness: every app window declares every required facet explicitly. Empty facets
 * carry only their tracked marker; components are forbidden at facet level and required on every
 * specific item. Shape, marker, and implementation languages come from shared vocabulary.
 */
export function policyWindowCompletenessBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const scopeId = crate.pluginId || policyStripEmoji(crate.ownerRel.split("/").pop() ?? "");
    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir).filter((candidate) => candidate.isDirectory)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.name === taxonomy.windowsDirName) {
          for (const window of policyReaddirSafe(repoRoot, childRel).filter((candidate) => candidate.isDirectory)) {
            const windowDir = `${childRel}/${window.name}`;
            const actual = new Set(policyReaddirSafe(repoRoot, windowDir).filter((candidate) => candidate.isDirectory).map((candidate) => candidate.name));
            for (const required of taxonomy.windowRequiredChildDirs) {
              const capabilityDir = `${windowDir}/${required}`;
              if (!actual.has(required)) {
                breaches.push({
                  id: `taxonomy-window-completeness-${windowDir}-${required}`,
                  summary: `"${windowDir}" is missing required window capability dir "${required}"`,
                  kind: "taxonomy/window-completeness",
                  scope: `${scopeId}/${policyStripEmoji(window.name)}`,
                  priority: "high",
                  reason: `Every window must explicitly carry ${taxonomy.windowRequiredChildDirs.join(", ")}; an empty capability is valid, an absent capability is not.`,
                  solution: `Add ${capabilityDir}/${taxonomy.windowEmptyFacetFilename}; replace the marker with specific item directories when the facet gains members.`,
                });
                continue;
              }
              const members = policyReaddirSafe(repoRoot, capabilityDir).filter((candidate) => candidate.isDirectory);
              const markerRel = `${capabilityDir}/${taxonomy.windowEmptyFacetFilename}`;
              for (const filename of new Set(Object.values(taxonomy.taxonomyLeafFilenames))) {
                if (!existsSync(join(repoRoot, capabilityDir, filename))) continue;
                breaches.push({
                  id: `taxonomy-window-facet-component-${capabilityDir}-${filename}`,
                  summary: `"${capabilityDir}" contains forbidden facet-level component leaf "${filename}"`,
                  kind: "taxonomy/window-facet-component",
                  scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(required)}`,
                  priority: "high",
                  reason: `Window facets are collections; only specific item directories may contain component leaves.`,
                  solution: `Move ${capabilityDir}/${filename} into ${capabilityDir}/<specific-item>/${filename}, or remove it and add ${markerRel} when the facet is empty.`,
                });
              }
              if (members.length === 0) {
                if (!existsSync(join(repoRoot, markerRel))) {
                  breaches.push({
                    id: `taxonomy-window-empty-facet-${capabilityDir}`,
                    summary: `Empty window facet "${capabilityDir}" is missing marker "${taxonomy.windowEmptyFacetFilename}"`,
                    kind: "taxonomy/window-empty-facet",
                    scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(required)}`,
                    priority: "high",
                    reason: `An empty required facet must remain tracked without pretending that the facet itself is a specific component.`,
                    solution: `Add ${markerRel}.`,
                  });
                }
                continue;
              }
              if (existsSync(join(repoRoot, markerRel))) {
                breaches.push({
                  id: `taxonomy-window-populated-facet-marker-${capabilityDir}`,
                  summary: `Populated window facet "${capabilityDir}" still contains empty marker "${taxonomy.windowEmptyFacetFilename}"`,
                  kind: "taxonomy/window-empty-facet",
                  scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(required)}`,
                  priority: "high",
                  reason: `A facet cannot be both empty and populated with specific items.`,
                  solution: `Remove ${markerRel}.`,
                });
              }
              for (const member of members) {
                const unit = { name: member.name, rel: `${capabilityDir}/${member.name}` };
                for (const lang of taxonomy.windowComponentLangs) {
                  const filename = taxonomy.taxonomyLeafFilenames[lang];
                  if (!filename || existsSync(join(repoRoot, unit.rel, filename))) continue;
                  breaches.push({
                    id: `taxonomy-window-component-${unit.rel}-${lang}`,
                    summary: `"${unit.rel}" is missing required ${lang} component leaf "${filename}"`,
                    kind: "taxonomy/window-component",
                    scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(unit.name)}`,
                    priority: "high",
                    reason: `Every specific window capability item must mirror ${taxonomy.windowComponentLangs.join(", ")}.`,
                    solution: `Add ${unit.rel}/${filename}.`,
                  });
                }
              }
            }
          }
          continue;
        }
        walk(childRel);
      }
    };
    for (const surfaceRoot of policySurfaceRoots(repoRoot, crate.ownerRel, taxonomy)) walk(surfaceRoot);
  }
  return breaches;
}

/**
 * 📏️Mode completeness: the twin of {@link policyWindowCompletenessBreaches} one level up. A mode is a
 * state-owning scope, so it declares every `modeRequiredChildDirs` member explicitly — its `🪟️windows`
 * collection plus its own three state lanes (`🎚️config` persisted-local, `👥️presence`
 * ephemeral-shared, `🫧️transient` ephemeral-local). An empty child is valid and carries only the
 * tracked `windowEmptyFacetFilename` marker; an absent child is not, and a child that is both empty
 * and populated is a contradiction. Ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION
 * wave A0: before it, `modeChildDirs` did not exist at all and a mode declared no children whatsoever.
 * Optional structural capabilities such as `🎮️commands` remain in `modeChildDirs` without forcing
 * an empty marker into every mode.
 */
export function policyModeCompletenessBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const required: readonly string[] = taxonomy.modeRequiredChildDirs ?? taxonomy.modeChildDirs ?? [];
  const breaches: BreachRecord[] = [];
  if (required.length === 0) return breaches;
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const scopeId = crate.pluginId || policyStripEmoji(crate.ownerRel.split("/").pop() ?? "");
    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir).filter((candidate) => candidate.isDirectory)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.name === taxonomy.modesDirName) {
          for (const mode of policyReaddirSafe(repoRoot, childRel).filter((candidate) => candidate.isDirectory)) {
            const modeDir = `${childRel}/${mode.name}`;
            const actual = new Set(policyReaddirSafe(repoRoot, modeDir).filter((candidate) => candidate.isDirectory).map((candidate) => candidate.name));
            for (const child of required) {
              const childDir = `${modeDir}/${child}`;
              const markerRel = `${childDir}/${taxonomy.windowEmptyFacetFilename}`;
              if (!actual.has(child)) {
                breaches.push({
                  id: `taxonomy-mode-completeness-${modeDir}-${child}`,
                  summary: `"${modeDir}" is missing required mode child dir "${child}"`,
                  kind: "taxonomy/mode-completeness",
                  scope: `${scopeId}/${policyStripEmoji(mode.name)}`,
                  priority: "high",
                  reason: `Every mode must explicitly carry ${required.join(", ")}; an empty child is valid, an absent child is not.`,
                  solution: `Add ${childDir}/${taxonomy.windowEmptyFacetFilename}; replace the marker with specific item directories when the child gains members.`,
                });
                continue;
              }
              const members = policyReaddirSafe(repoRoot, childDir).filter((candidate) => candidate.isDirectory);
              if (members.length === 0) {
                if (!existsSync(join(repoRoot, markerRel))) {
                  breaches.push({
                    id: `taxonomy-mode-empty-child-${childDir}`,
                    summary: `Empty mode child "${childDir}" is missing marker "${taxonomy.windowEmptyFacetFilename}"`,
                    kind: "taxonomy/mode-empty-child",
                    scope: `${scopeId}/${policyStripEmoji(mode.name)}/${policyStripEmoji(child)}`,
                    priority: "high",
                    reason: `An empty required child must remain tracked without pretending that the child itself is a specific component.`,
                    solution: `Add ${markerRel}.`,
                  });
                }
                continue;
              }
              if (existsSync(join(repoRoot, markerRel))) {
                breaches.push({
                  id: `taxonomy-mode-populated-child-marker-${childDir}`,
                  summary: `Populated mode child "${childDir}" still contains empty marker "${taxonomy.windowEmptyFacetFilename}"`,
                  kind: "taxonomy/mode-empty-child",
                  scope: `${scopeId}/${policyStripEmoji(mode.name)}/${policyStripEmoji(child)}`,
                  priority: "high",
                  reason: `A child cannot be both empty and populated with specific items.`,
                  solution: `Remove ${markerRel}.`,
                });
              }
            }
          }
          continue;
        }
        walk(childRel);
      }
    };
    for (const surfaceRoot of policySurfaceRoots(repoRoot, crate.ownerRel, taxonomy)) walk(surfaceRoot);
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
  } else {
    for (const entry of policyReaddirSafe(repoRoot, assetsRel)) {
      if (entry.isDirectory) continue;
      const assetAbs = join(repoRoot, assetsRel, entry.name);
      if (statSync(assetAbs).size === 0) {
        breaches.push({
          id: `semio-examples-zero-byte-asset-${exampleRel}-${entry.name}`,
          summary: `"${assetsRel}/${entry.name}" is a 0-byte placeholder asset`,
          kind: "taxonomy/semio-examples",
          scope,
          priority,
          reason: "Example assets must be real, builder-emitted (or genuinely sourced) content — a 0-byte file proves nothing and can't be decoded by any analyzer test.",
          solution: `Replace ${assetsRel}/${entry.name} with a small file the artifact's own builder/analyzer can actually round-trip, or a real fixture.`,
        });
      }
    }
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
        reason: "Examples belong under 🗿️artifacts/<artifact>/📚️examples or a subset's 👁️viewer|✏️editor/📚️examples — never at the plugin root.",
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
    for (const surfaceRoot of policySurfaceRoots(repoRoot, ownerRoot, taxonomy)) {
      // 👁️✏️ Unlike the old 🎛️apps facet, 📚️examples is NOT in surfaceRequiredChildDirs (contract
      // §7.5) — a surface with no examples is legal, so this only validates units that DO exist.
      const [subsetName, roleDirName] = surfaceRoot.split("/").slice(-2);
      const surfaceScope = `${scopeId}/${policyStripEmoji(subsetName ?? "")}/${policyStripEmoji(roleDirName ?? "")}`;
      const surfaceExamples = `${surfaceRoot}/${examplesDir}`;
      if (!existsSync(join(repoRoot, surfaceExamples))) continue;
      const sets = policyReaddirSafe(repoRoot, surfaceExamples).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-surface-empty-${surfaceExamples}`,
          summary: `"${surfaceExamples}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: surfaceScope,
          priority,
          reason: "A declared surface 📚️examples/ must contain at least one emoji-slug example unit.",
          solution: `Add ${surfaceExamples}/<emoji-slug>/ with definition leaves, assets, and tests, or remove the empty ${examplesDir}/ dir.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${surfaceExamples}/${set.name}`, surfaceScope, priority));
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
    // 👁️✏️ Surfaces now nest under 🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/{👁️viewer,✏️editor}/, which
    // this recursive walk already reaches — no separate root needed once 🎛️apps is gone.
    walk(`${ownerRoot}/${taxonomy.artifactsDirName}`);
  }
  return breaches;
}

/**
 * 📏️Anti-inlining tripwire (Single-File-Repo hazard ruling, master ticket): a migrated package's entry
 * `📦️glue.rs` must stay wiring-only (`#[path]` mod declarations + `plugin_exports!(plugin::plugin)`) — no
 * non-trivial `fn`/`impl` body content beyond `taxonomy.libWiringLineBudget`. Catches the exact
 * regression this repo has hit twice before: an agent following the (now-scoped) "single file repo" goal
 * inlining split `#[path]` modules back into `glue.rs`. Plugin identity lives at the plugin root via
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
      reason: "Single-File-Repo hazard ruling: a taxonomy package's 📦️glue.rs is #[path] mod wiring + plugin_exports!(plugin::plugin) only — real logic lives in taxonomy component files at the plugin root, never inlined back.",
      solution: `Move the non-trivial fn/impl bodies out of ${crate.libRelPath} into their owning taxonomy component file(s) at the plugin root; glue.rs should only declare "#[path = \"...\"] mod ...;" and call plugin_exports!(plugin::plugin).`,
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

/** 🧷️Names whose exact spelling is owned by an ecosystem or taxonomy contract. */
function policyEmojiFixedFilenames(): ReadonlySet<string> {
  const taxonomy = loadTaxonomy();
  const names = new Set<string>([
    ...taxonomy.packagingFileNames,
    ...taxonomy.rootDataFileNames,
    ...taxonomy.rootDocFileNames,
    ...Object.values(taxonomy.taxonomyLeafFilenames),
    ...Object.values(taxonomy.entryFilenames),
    ...Object.values(taxonomy.exampleLeafFilenames),
    ...Object.values(taxonomy.exampleTestLeafFilenames),
    "Cargo.lock",
    "bun.lock",
    "bun.lockb",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "clippy.toml",
    "go.work",
    "go.work.sum",
    "next-env.d.ts",
    "vite-env.d.ts",
    "components.json",
    "post-checkout",
    "post-commit",
    "post-merge",
    "post-rewrite",
    "prepare-commit-msg",
  ]);
  const visit = (value: unknown): void => {
    if (typeof value === "string" && /\.[a-z0-9]+$/i.test(value)) names.add(value);
    else if (Array.isArray(value)) value.forEach(visit);
    else if (value && typeof value === "object") Object.values(value).forEach(visit);
  };
  visit(taxonomy);
  return names;
}

/** 🏭️Generated and framework-owned trees whose entry names are not repository taxonomy. */
function policyEmojiEntryIsGeneratedOrFrameworkOwned(relDir: string, name: string): boolean {
  const rel = `${relDir}/${name}`;
  const segments = rel.split("/");
  return name.startsWith(".") || name === "pkg" || name === "coverage" || name === "partial_movie_files" || name === "__pycache__" || name === "client" || name === "client_bin" || segments.includes("app");
}

/** 🪪️Whether an entry is free to adopt the repository's emoji-prefixed identity. */
function policyEmojiEntryIsRenamable(relDir: string, name: string, isDirectory: boolean, fixedFilenames: ReadonlySet<string>): boolean {
  if (!name || name.startsWith(".") || policyEmojiEntryIsGeneratedOrFrameworkOwned(relDir, name)) return false;
  if (isDirectory) return true;
  const taxonomy = loadTaxonomy();
  return !fixedFilenames.has(name) && !taxonomy.packagingFileSuffixes.some((suffix) => name.endsWith(suffix));
}

/** 🎭️Families whose shared leading emoji is a structural kind marker rather than a sibling identity.
 *
 * 🪟️ A window's and a mode's children are entirely taxonomy VOCABULARY (`windowChildDirs` /
 * `modeChildDirs`) — nobody authored those names locally, so they carry no local visual identity to
 * keep unique, and the vocabulary itself is free to spell two related capabilities with one family
 * emoji (`🎚️options` beside the `🎚️config` state lane). Renaming either at a site would break the
 * vocabulary it is a member of, so uniqueness is the wrong instrument here. */
function policyEmojiSiblingIdentityIsStructural(relDir: string, name: string, taxonomy: ReturnType<typeof loadTaxonomy>): boolean {
  const segments = relDir.split("/");
  const parent = segments[segments.length - 1] ?? "";
  const grandparent = segments[segments.length - 2] ?? "";
  if (grandparent === taxonomy.windowsDirName && taxonomy.windowChildDirs.includes(name)) return true;
  if (grandparent === taxonomy.modesDirName && (taxonomy.modeChildDirs ?? []).includes(name)) return true;
  return name === "📝️text" || name === "💾️binary" || parent === "🏅️standards" || parent === "🪆️subsets" || parent === "💡️inferences" || parent === "🗿️artifacts" || parent === "📚️examples" || segments.includes("🖼️assets");
}

/** 🧬️Migration slug families whose bare-symbol presentation is enforced by their dedicated policy. */
function policyEmojiPresentationIsStructural(relDir: string): boolean {
  const segments = relDir.split("/");
  return segments.includes("🧬️mutations") || segments.includes("💡️inferences");
}

/** ✨️Whether a name starts with an actual emoji sequence rather than arbitrary non-ASCII text. */
function policyHasLeadingEmoji(name: string): boolean {
  const prefix = policyLeadingEmojiPrefix(name);
  return prefix.length > 0 && /[^\x00-\x7f]/u.test(prefix);
}

/** 🎨️Whether a Latin-stemmed emoji identity is missing the required emoji presentation selector. */
function policyEmojiPrefixNeedsVs16(name: string): boolean {
  const prefix = policyLeadingEmojiPrefix(name);
  const first = [...prefix][0] ?? "";
  return policyHasLeadingEmoji(name) && /^[A-Za-z]/.test(name.slice(prefix.length)) && !/\p{Emoji_Presentation}/u.test(first) && !prefix.includes(POLICY_VS16);
}

/** ✅️Every renamable entry in a clean taxonomy area has a VS16 emoji identity unique among its siblings. */
export function policyEmojiPrefixBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  if (taxonomy.requireEmojiPrefixWithVs16 !== true) return [];
  const fixedFilenames = policyEmojiFixedFilenames();
  const breaches: BreachRecord[] = [];
  const cleanRoots = Object.entries(taxonomy.areas)
    .filter(([, state]) => state === "clean")
    .map(([root]) => root)
    .filter((root, index, roots) => !roots.some((candidate, candidateIndex) => candidateIndex !== index && root.startsWith(`${candidate}/`)));
  const walk = (relDir: string): void => {
    const seen = new Map<string, string>();
    for (const entry of policyReaddirSafe(repoRoot, relDir)) {
      const childRel = `${relDir}/${entry.name}`;
      const renamable = policyEmojiEntryIsRenamable(relDir, entry.name, entry.isDirectory, fixedFilenames);
      const hasPrefix = policyHasLeadingEmoji(entry.name);
      if (renamable && !hasPrefix) {
        breaches.push({
          id: `emoji-prefix-missing-${childRel}`,
          summary: `"${childRel}" is renamable but has no leading emoji prefix`,
          kind: "taxonomy/emoji-prefix",
          scope: childRel,
          priority: "high",
          reason: "Every renamable file and directory in a clean taxonomy area must start with an emoji identity.",
          solution: `Rename "${entry.name}" with a sibling-unique emoji prefix and update every reference.`,
        });
      } else if (renamable && !policyEmojiPresentationIsStructural(relDir) && policyEmojiPrefixNeedsVs16(entry.name)) {
        breaches.push({
          id: `emoji-vs16-${childRel}`,
          summary: `"${childRel}" is missing U+FE0F on its emoji prefix`,
          kind: "taxonomy/emoji-prefix",
          scope: childRel,
          priority: "high",
          reason: "taxonomy.requireEmojiPrefixWithVs16 requires emoji presentation on Latin-stemmed renamable entries.",
          solution: `Rename the leading emoji so it includes U+FE0F, preserving the stem and references.`,
        });
      }
      if (renamable && hasPrefix && !policyEmojiSiblingIdentityIsStructural(relDir, entry.name, taxonomy)) {
        const prefix = policyLeadingEmojiPrefix(entry.name).replaceAll(POLICY_VS16, "");
        const previous = seen.get(prefix);
        if (previous) {
          breaches.push({
            id: `emoji-prefix-duplicate-${relDir}-${prefix.codePointAt(0)?.toString(16) ?? "unknown"}-${entry.name}`,
            summary: `"${relDir}" reuses emoji prefix "${prefix}" for siblings "${previous}" and "${entry.name}"`,
            kind: "taxonomy/emoji-prefix-uniqueness",
            scope: childRel,
            priority: "high",
            reason: "A leading emoji is the local visual identity of an entry and must be unique among siblings.",
            solution: `Rename either sibling with a distinct emoji prefix and update every reference.`,
          });
        } else seen.set(prefix, entry.name);
      }
      if (entry.isDirectory && !policyEmojiEntryIsGeneratedOrFrameworkOwned(relDir, entry.name)) walk(childRel);
    }
  };
  cleanRoots.forEach(walk);
  return breaches;
}

/**
 * 🔌️ Every plugin owner under `✏️s/🔌️plugins/` must carry its contract leaf and facets directly at its root.
 */
function policyPluginRootShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const children = taxonomy.pluginChildDirs;
  const pluginsRoot = "✏️s/🔌️plugins";
  const breaches: BreachRecord[] = [];
  for (const entry of policyReaddirSafe(repoRoot, pluginsRoot)) {
    if (!entry.isDirectory) continue;
    const ownerRel = `${pluginsRoot}/${entry.name}`;
    const nestedContractRel = `${ownerRel}/🔌️plugin`;
    if (existsSync(join(repoRoot, nestedContractRel))) {
      breaches.push({
        id: `plugin-root-nested-${ownerRel}`,
        summary: `"${ownerRel}" contains a redundant 🔌️plugin/ directory`,
        kind: "taxonomy/plugin-root-shape",
        scope: ownerRel,
        priority: "high",
        reason: "Plugin contracts and facets are direct children of the plugin root.",
        solution: `Move ${nestedContractRel}/🦀️component.rs and its facet leaves directly into ${ownerRel}/, then remove ${nestedContractRel}/.`,
      });
    }
    if (!existsSync(join(repoRoot, ownerRel, "🦀️component.rs"))) {
      breaches.push({
        id: `plugin-root-leaf-${ownerRel}`,
        summary: `"${ownerRel}" is missing root 🦀️component.rs`,
        kind: "taxonomy/plugin-root-shape",
        scope: ownerRel,
        priority: "high",
        reason: "The plugin root must have a leaf component that returns Plugin via Plugin::builder.",
        solution: `Add ${ownerRel}/🦀️component.rs exporting pub fn plugin() -> Plugin.`,
      });
    }
    for (const child of children) {
      const childLeaf = join(repoRoot, ownerRel, child, "🦀️component.rs");
      if (!existsSync(childLeaf)) {
        breaches.push({
          id: `plugin-root-child-${ownerRel}/${child}`,
          summary: `"${ownerRel}" is missing direct facet ${child}/🦀️component.rs`,
          kind: "taxonomy/plugin-root-shape",
          scope: ownerRel,
          priority: "high",
          reason: `${child} is a required direct plugin-root facet (taxonomy.pluginChildDirs).`,
          solution: `Add ${ownerRel}/${child}/🦀️component.rs.`,
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
        summary: `"${crate.libRelPath}" still uses semio_plugin! — migrate to Plugin::builder at the plugin root`,
        kind: "taxonomy/plugin-builder",
        scope: crate.pluginId || crate.ownerRel,
        priority: "high",
        reason: "semio_plugin! is retired; plugin identity lives in the root 🦀️component.rs via typestate PluginBuilder.",
        solution: "Move registration into the plugin-root 🦀️component.rs using Plugin::builder(...).build() and call plugin_exports!(plugin::plugin).",
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
//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture
/**
 * 🏛️ APA (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — five report-mode census rules enforcing
 * the target plugin shape: a plugin is EXACTLY 🎛️apps + 🗿️artifacts + root 🦀️component.rs/AGENTS.md/
 * README.md + 📦️packages wiring; all io, state changes, registration, and side effects belong to
 * artifacts — never apps, never a setup facet. REPORT MODE IS LOAD-BEARING: every breach below carries
 * `priority: "medium"`, never `"high"` — `VerifyScript.runGate`'s `dissolveBreaches` block (where these
 * are also wired in) filters to `priority === "high"` before throwing, so these five rules census the
 * fleet without failing the gate for the ~14 concurrent agents already running against it
 * (📌️important.md). Wave 3 (mass plugin migration) is what fixes what these rules find; Wave 5 flips
 * them to `"high"` once migration lands, the same staging `policyPluginRootShapeBreaches` itself used.
 */
const POLICY_APA_PLUGINS_ROOT = "✏️s/🔌️plugins";

//#region 🔧️PolicyRulePluginClosedShape
/** 🎫️Ticket-cited exceptions to the plugin-root closed-shape rule — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_PLUGIN_CLOSED_SHAPE_ALLOWLIST = new Set<string>();

/** 🗑️Filenames/dirnames that are always stray tooling junk at a plugin root, never legitimate content, regardless of plugin. */
const POLICY_PLUGIN_CLOSED_SHAPE_JUNK = new Set(["node_modules", ".DS_Store"]);

/**
 * 🗺️extra-entry → proposed APA destination, keyed `"<pluginOwnerRel>/<entryName>"` — transcribed from
 * 📓️w0-b-plugin-shape.md §5 / 📓️w0-census.md §3 §6 (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
 * Every plugin-root entry this table doesn't name falls back to a generic relocate-under-artifacts-or-apps
 * solution in `policyPluginClosedShapeBreaches` below.
 */
const POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS: Readonly<Record<string, string>> = {
  "✏️s/🔌️plugins/🌀️procedural/🎮️play": "Fold into 🗿️artifacts/<kind>/📚️examples/ once confirmed non-placeholder (dir currently holds only AGENTS.md) — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🌊️flow/🔨️modules": "Move 🔨️modules/🧮️compute/🟦️component.ts into 🗿️artifacts/<flowcompute-kind>/🏅️standards/🔖️1/⚙️engine/compute/ (artifact-kind name needs an owner decision) — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions": "Extension-crate axis (role=extension, extends=flow, 9 crates) — sanctioned third axis pending the §6 ruling in 📓️w0-census.md; not auto-relocatable.",
  "✏️s/🔌️plugins/🌍️gis/🔨️modules": "Move 🔨️modules/🏔️terrain/🦀️component.rs into 🗿️artifacts/gismap/🏅️standards/🔖️1/⚙️engine/terrain/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/➗️formulation": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/formulation/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🏗️model": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/model/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/📏️elements2d": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/elements2d/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🔢️sparse": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/sparse/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🕸️mesh": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/mesh/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🧊️elements3d": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/elements3d/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🧮️analyses": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/analyses/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🖥️app-surface": "CANNOT CLASSIFY under a current APA slot — shared by fem2d_ui and fem3d_ui, needs an explicit cross-app-shared-code ruling before relocation (bannedNameStems forbids the obvious core/shared/common fallback) — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/🏭️process/🧩️extensions": "Extension-crate axis (role=extension, extends=process, 4 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📐️cad/🔨️modules": "Needs per-subdir inspection (14 files, not enumerated) — likely folds into 🗿️artifacts/<cad-artifact>/🏅️standards/…/⚙️engine/ per the gis/puzzle pattern — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions": "Extension-crate axis (role=extension, extends=cad, 4 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📕️norm/🎚️config": "Shared default for 15 norm apps — needs a cross-app-shared-code ruling (duplicate into each 🎛️apps/<norm-app>/🎚️config/, or a new sanctioned slot) — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/📕️norm/👥️presence": "Same shared-across-15-apps ruling as 🎚️config — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/📕️norm/📄️artifact": "Feeds all 15 norm standard artifacts — needs a cross-artifact-shared-engine ruling (candidate: 🗿️artifacts/norm/🏅️standards/🔖️shared/⚙️engine/core/) — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/📕️norm/🖥️app-surface": "Same cross-app-shared ruling as fem's 🖥️app-surface — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/📖️playbook/🧩️extensions": "Extension-crate axis (role=extension, extends=playbook, 1 crate) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📜️imperative/🧩️extensions": "Extension-crate axis (role=extension, extends=imperative, 5 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/🔋️energy/⚙️engine": "Single largest violation in the census — a 50-submodule headless HVAC engine sitting at plugin root → 🗿️artifacts/energy/🏅️standards/🔖️1/⚙️engine/<module>/ for each submodule — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🌳️ast": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/ast/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🔤️lexer": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/lexer/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🔨️modules": "23 files incl. jack/shell+jack/lsp → 🗿️artifacts/<kind>/🏅️standards/…/⚙️engine/jack/{shell,lsp}/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🗣️language-service": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/language-service/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🧮️executor": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/executor/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🖍️draw/🔄️fsm": "→ 🗿️artifacts/draw/🏅️standards/🔖️1/⚙️engine/fsm/; the nested ✨️macros sub-crate needs a crate-boundary specialist, not a plain directory move — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🧩️puzzle/🔨️modules": "Move 🔨️modules/🎲️board-2d/🦀️component.rs into 🗿️artifacts/puzzle2d/🏅️standards/🔖️1/⚙️engine/board-2d/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions": "Extension-crate axis (role=extension, extends=sourcing, 3 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📐️cad/🔣️machine.json": "210KB root data file — CANNOT CLASSIFY without reading contents (likely a generated/vendored CAD-kernel data file) — 📓️w0-b-plugin-shape.md §5.",
};

/**
 * 📏️The three legacy per-plugin facets `taxonomy.pluginChildDirs` dropped (it now reads `["🎮️commands"]`
 * only, per the W3 dissolution) — every one of the 33 plugins still carries doc-only (or, for
 * gis/lowpoly/norm/stdio, real) content under these directory names; this is the "missing absence
 * check" `policyPluginRootShapeBreaches` never performed (it only ever checks presence of required
 * facets, never flags an extra one).
 */
const POLICY_PLUGIN_CLOSED_SHAPE_LEGACY_FACETS: Readonly<Record<string, string>> = {
  "🎟️capabilities": "Doc-only stub in all 33 plugins (zero real .capability() calls repo-wide) — delete. The one real capability call (🪐️space's .local_backbone_storage()) already lives at plugin root, not this facet.",
  "🔧️setup": "30/33 are 1-line doc-only stubs — delete. 🌍️gis/💠️lowpoly/📕️norm carry real fan-out code (register_gis_exports/register_lowpoly_exports/register_norm_exports) — fold each into the owning artifact's own ⚙️engine registration path (or the M1 declarative ArtifactDeclaration mechanism once it lands), not a standalone plugin-root facet.",
  "🛂️manifest": "32/33 are 1-line doc-only stubs — delete. 🗄️stdio's 344-line stdio_format_descriptors() catalog (region 🔖️FormatCatalog) is real declared data and belongs under 🗿️artifacts/<kind>/…, not this facet.",
};

/**
 * 📏️Plugin-root closed-shape rule: for every owner under `✏️s/🔌️plugins/`, every direct child entry NOT
 * in the taxonomy-derived allowed set (`pluginChildDirs` ∪ `artifactsDirName` ∪ `packagesDirName` ∪ the
 * root component leaf ∪ `rootDocFileNames` ∪ `rootDataDirNames` ∪ `rootDataFileNames` — the last two are
 * legal ONLY at an owner root per taxonomy.json's own `_treePurityComment`, which is why 🗄️stdio's
 * `📇️registry`, 📐️cad's `🖼️assets`/`🧫️fixtures`, and every plugin's root `🛂️manifest.json` are excluded
 * below rather than flagged) is a shape breach. The allowed set is derived from `🔣️taxonomy.json` via
 * `loadTaxonomy()` rather than hardcoded, per the ticket's instruction — a dedicated `pluginRootAllowedEntries`
 * taxonomy key was deliberately NOT added: every field this rule needs already exists on `Taxonomy`
 * (`pluginChildDirs`/`artifactsDirName`/`packagesDirName`/`rootDocFileNames`/`rootDataDirNames`/
 * `rootDataFileNames`), and `🔣️taxonomy.json`/`🔍️discovery/🟦️component.ts`/`🧪️index.test.ts` are outside
 * this session's single-writer boundary on `📜️script.ts` — adding a key there risks colliding with W1's
 * concurrent taxonomy work for no expressive gain.
 */
function policyPluginClosedShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const allowedDirs = new Set<string>([...taxonomy.pluginChildDirs, taxonomy.artifactsDirName, taxonomy.packagesDirName, ...taxonomy.rootDataDirNames]);
  const allowedFiles = new Set<string>(["🦀️component.rs", ...taxonomy.rootDocFileNames, ...taxonomy.rootDataFileNames]);
  const breaches: BreachRecord[] = [];
  let owners: ReturnType<typeof readdirSync>;
  try {
    owners = readdirSync(join(repoRoot, POLICY_APA_PLUGINS_ROOT), { withFileTypes: true });
  } catch {
    return breaches;
  }
  for (const owner of owners) {
    if (!owner.isDirectory()) continue;
    const ownerRel = `${POLICY_APA_PLUGINS_ROOT}/${owner.name}`;
    let children: ReturnType<typeof readdirSync>;
    try {
      children = readdirSync(join(repoRoot, ownerRel), { withFileTypes: true });
    } catch {
      continue;
    }
    for (const child of children) {
      const childRel = `${ownerRel}/${child.name}`;
      if (POLICY_PLUGIN_CLOSED_SHAPE_ALLOWLIST.has(childRel)) continue;
      if (child.isDirectory()) {
        if (allowedDirs.has(child.name)) continue;
        if (POLICY_PLUGIN_CLOSED_SHAPE_JUNK.has(child.name)) {
          breaches.push({
            id: `plugin-closed-shape-junk-${childRel}`,
            summary: `"${childRel}" is stray tooling junk at a plugin root`,
            kind: "taxonomy/plugin-closed-shape",
            scope: ownerRel,
            priority: "medium",
            reason: "APA: a plugin is EXACTLY 🗿️artifacts + root wiring — build caches and OS junk never belong at plugin root.",
            solution: `Delete ${childRel}; add "${child.name}" to the repo .gitignore if it keeps recurring.`,
          });
          continue;
        }
        const legacy = POLICY_PLUGIN_CLOSED_SHAPE_LEGACY_FACETS[child.name];
        const solution =
          legacy ??
          POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS[childRel] ??
          `Relocate ${childRel} under ${ownerRel}/${taxonomy.artifactsDirName}/<kind>/<standard>/🪆️subsets/<subset>/{👁️viewer,✏️editor}/ — see 📓️w0-b-plugin-shape.md §5 for the nearest analogous mapping, or file a "needs ruling" note if none fits.`;
        breaches.push({
          id: `plugin-closed-shape-dir-${childRel}`,
          summary: `"${childRel}" is a plugin-root entry outside the closed artifacts shape`,
          kind: "taxonomy/plugin-closed-shape",
          scope: ownerRel,
          priority: "medium",
          reason: "APA: a plugin is EXACTLY 🗿️artifacts + root 🦀️component.rs/AGENTS.md/README.md + 📦️packages wiring — every other direct child is a shape violation, not merely a missing-facet gap (which is all policyPluginRootShapeBreaches ever checked).",
          solution,
        });
        continue;
      }
      if (allowedFiles.has(child.name)) continue;
      breaches.push({
        id: `plugin-closed-shape-file-${childRel}`,
        summary: `"${childRel}" is a plugin-root file outside the closed apps+artifacts shape`,
        kind: "taxonomy/plugin-closed-shape",
        scope: ownerRel,
        priority: "medium",
        reason: "APA: plugin-root files are limited to the leaf component, taxonomy.rootDocFileNames, and taxonomy.rootDataFileNames — a stray data file at root is a shape violation.",
        solution: POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS[childRel] ?? `Classify and relocate ${childRel} under 🗿️artifacts or 🎛️apps, or add it to taxonomy.rootDataFileNames if it is genuinely owner-root data.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRulePluginClosedShape

//#region 🔧️PolicyRulePluginPurity
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_PLUGIN_PURITY_ALLOWLIST = new Set<string>();

/** 🗂️`fs::`-word-bounded catches both `std::fs::x` and the common `use std::fs; fs::x` idiom this tree actually uses (📓️w0-c-purity.md §1), plus tokio::fs/read_dir/read_to_string/File::. */
const POLICY_PLUGIN_PURITY_FS_RE = /\b(?:std::fs::|tokio::fs::|fs::|read_dir\s*\(|read_to_string\s*\(|File::)/;
const POLICY_PLUGIN_PURITY_ENV_RE = /\b(?:std::env::|std::process::|Command::new\s*\(|temp_dir\s*\()/;
const POLICY_PLUGIN_PURITY_NET_RE = /\b(?:std::net::|reqwest::|ureq::|hyper::|TcpStream\b)/;
const POLICY_PLUGIN_PURITY_THREAD_LOCAL_RE = /\bthread_local!\s*(?:\{|\()/;
const POLICY_PLUGIN_PURITY_STATIC_MUT_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+mut\b/;
/** 🔓️A `static` item's own declaration line is item-scope even when the item is lexically nested inside a `fn` body (the `fn next_id() { static COUNTER: AtomicU64 = …; }` monotonic-id-counter idiom this tree uses 15+ times) — `static` denotes a persistent 'static-duration item in Rust regardless of lexical nesting, so this overrides the `!inFn` gate for that one line. */
const POLICY_PLUGIN_PURITY_STATIC_ITEM_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+(?:mut\s+)?\w+\s*:/;
const POLICY_PLUGIN_PURITY_LAZY_STATIC_RE = /\blazy_static!\s*\{/;
/** 🔒️Item-scope interior-mutability types that are genuinely mutated after construction — deliberately excludes bare OnceLock/OnceCell/LazyLock (the sanctioned write-once lazy-table pattern every artifact io_registry uses); a nested `OnceLock<Mutex<...>>` still matches because `Mutex<` is present in the same line. */
const POLICY_PLUGIN_PURITY_INTERIOR_MUT_RE = /\b(Mutex|RwLock|RefCell|Cell)\s*<|\b(Atomic(?:Bool|I8|I16|I32|I64|Isize|U8|U16|U32|U64|Usize))\b/;
const POLICY_PLUGIN_PURITY_TS_RE = /\b(fetch\s*\(|XMLHttpRequest\b|WebSocket\b|localStorage\b|sessionStorage\b|indexedDB\b)/;
const POLICY_PLUGIN_PURITY_FN_ITEM_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+\w/;

/** 🧪️Brace-span of a `#[test]`/`#[tokio::test]`/`#[async_std::test]`/`#[wasm_bindgen_test]`-attributed fn body — sanctioned test-only IO the purity rule must skip in addition to `#[cfg(test)] mod …` spans (`policyTestModSpans`, reused as-is). */
function policyPluginPurityTestFnSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  let depth = 0;
  let pendingTestAttr = false;
  let recording = false;
  let recordDepth = 0;
  let openLine = -1;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    if (!recording) {
      if (/^\s*#\[(?:test|tokio::test|async_std::test|wasm_bindgen_test)\b/.test(raw)) {
        pendingTestAttr = true;
      } else if (pendingTestAttr && /^\s*#\[/.test(raw)) {
        // stacked attribute — keep waiting for the fn line
      } else if (pendingTestAttr && /\bfn\s+\w/.test(raw)) {
        if (/\{/.test(codeOnly)) {
          recording = true;
          recordDepth = depth;
          openLine = i + 1;
        }
        pendingTestAttr = false;
      } else if (pendingTestAttr) {
        pendingTestAttr = false;
      }
    }
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    if (recording && depth <= recordDepth) {
      spans.push({ name: "test-fn", startLine: openLine, endLine: i + 1 });
      recording = false;
    }
  });
  return spans;
}

/** 🔎️Every `.ts`/`.tsx` file under `rootRel`, excluding the repo's sole TS test convention (`🧪️tests/` dirs, `🟦️test.ts` files) — the same split 📓️w0-c-purity.md's scout used, confirmed a 100%-clean split there. */
function policyPluginPurityTsFiles(repoRoot: string, rootRel: string): string[] {
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
      const childRel = `${relDir}/${ent.name}`;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name) || ent.name === "🧪️tests") continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "🟦️test.ts") continue;
      if (ent.name.endsWith(".ts") || ent.name.endsWith(".tsx")) found.push(childRel);
    }
  };
  walk(rootRel);
  return found.sort();
}

//#region 🔖️StateLaneExhaustiveness
/**
 * 🫧️Files that may touch browser/native storage directly, because they ARE a sanctioned storage owner:
 * - `🖥️platform` is the config lane's persistence adapter (`StoragePort`).
 * - `🟦️backbone-worker.ts` backs `BlobStore` — content-addressed ARTIFACT content (what a
 *   `LinkPin::Snapshot` escrows), which belongs to the artifact mechanism, not to config. Flagging it
 *   would be a category error: it is not local-only UI state routing around a lane.
 */
const POLICY_STATE_LANE_CONFIG_ADAPTER_PREFIXES = ["🧰️framework/🔨️modules/🖥️platform/", "🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts"];

/** 🫧️Trees excluded from the lane rules: the parallel `compose/` stack and product/site code that is not an app. */
const POLICY_STATE_LANE_EXCLUDED_PREFIXES = ["compose/", "♻️mit-bestand/", "🌎️hub/", "🧰️framework/🛍️products/🦑️repo/"];

/** 🫧️Direct browser/native persistence — legal only inside the config-lane adapter. */
const POLICY_STATE_LANE_STORAGE_RE = /\b(?:localStorage|sessionStorage|indexedDB)\b/;

/** 🫧️The TS process-local ephemeral boxes the Transient lane replaces. */
const POLICY_STATE_LANE_EPHEMERAL_BOX_RE = /\bephemeral(?:Box|Map|Set|WeakMap)\b/;

/**
 * 📏️State-lane exhaustiveness: exactly FOUR state mechanisms exist, and nothing may route around them —
 * **artifacts** (persisted + shared), **config** (persisted + local-only), **presence** (ephemeral + shared),
 * **transient** (ephemeral + local-only UI). Ticket `26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION`.
 *
 * 🎯️ The lanes became reachable in wave A1 (`PresenceStore`/`TransientStore`, `EphemeralEmit`,
 * `ArtifactApp::ephemeral`). This rule is the other half of the mandate — "enforced by api AND policies":
 * an available lane that nothing forces you to use just becomes a fifth option beside the ad-hoc ones.
 *
 * Two sub-kinds, both REPORT-MODE at medium priority (they measure a real, large existing surface —
 * ~158 `ephemeralBox*` uses and the shell's direct `localStorage` writes — which wave A4 retires):
 * - `storage-outside-config-lane`: direct `localStorage`/`sessionStorage`/`indexedDB` outside the
 *   `🖥️platform` adapter. Persisted local-only state IS the config lane; the medium is an adapter
 *   detail, so bypassing the lane is what is banned, never the API itself.
 * - `ephemeral-box`: the `ephemeralBox`/`ephemeralMap`/`ephemeralSet`/`ephemeralWeakMap` helpers in
 *   `🎠️kernel/🟦️component.ts`, whose own docstring calls them the "sole lane until the OS draft
 *   snapshot owns these keys". The Transient lane now owns them.
 */
export function policyStateLaneExhaustivenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const roots = ["🧰️framework", "✏️s"];
  const seen = new Set<string>();

  for (const root of roots) {
    for (const relPath of policyPluginPurityTsFiles(repoRoot, root)) {
      if (seen.has(relPath)) continue;
      seen.add(relPath);
      if (POLICY_STATE_LANE_EXCLUDED_PREFIXES.some((prefix) => relPath.startsWith(prefix))) continue;
      // 🧪️ Test files are exempt: a test that drives the storage adapter, or asserts that an
      // ephemeral box was retired, must be able to NAME the thing it is testing.
      if (/\.test\.tsx?$/.test(relPath) || relPath.includes("/🧪️")) continue;
      const isConfigAdapter = POLICY_STATE_LANE_CONFIG_ADAPTER_PREFIXES.some((prefix) => relPath.startsWith(prefix));
      const content = policyReadFileSafe(repoRoot, relPath);
      if (!content) continue;

      content.split(/\r?\n/).forEach((raw, i) => {
        const lineNo = i + 1;
        // 🎯️ Mask literals, then strip BOTH comment forms: this rule greps raw source, and prose
        // explaining that a helper was retired must not itself trip the rule — a hazard this repo's
        // vocabulary policies have already been bitten by. `//` alone is not enough; the block-comment
        // continuation (` * …`) is exactly where a docstring mentioning `localStorage` lives.
        const trimmed = raw.trimStart();
        if (trimmed.startsWith("*") || trimmed.startsWith("/*") || trimmed.startsWith("//")) return;
        const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "").replace(/\/\*.*?\*\//g, "");

        if (!isConfigAdapter && POLICY_STATE_LANE_STORAGE_RE.test(codeOnly)) {
          breaches.push({
            id: `state-lane-storage-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" persists local state outside the config lane`,
            kind: "taxonomy/state-lane-storage-outside-config-lane",
            scope: relPath,
            line: lineNo,
            priority: "medium",
            reason: "Only four state mechanisms exist. Persisted local-only state IS the config lane; a direct localStorage/sessionStorage/indexedDB write is a fifth, untyped, unvalidated one that no schema facet describes and no policy can check the shape of.",
            solution: `Move the state at ${relPath}:${lineNo} into the app's (or the OS shell's) config record and let the 🖥️platform StoragePort adapter persist it. The storage MEDIUM stays legal — bypassing the lane is what does not.`,
          });
        }

        if (POLICY_STATE_LANE_EPHEMERAL_BOX_RE.test(codeOnly)) {
          breaches.push({
            id: `state-lane-ephemeral-box-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" uses a process-local ephemeral box`,
            kind: "taxonomy/state-lane-ephemeral-box",
            scope: relPath,
            line: lineNo,
            priority: "medium",
            reason: "The ephemeralBox/Map/Set/WeakMap helpers are untyped process-local state keyed by string — their own docstring calls them the sole lane 'until the OS draft snapshot owns these keys'. The Transient lane (ephemeral + local-only) now owns exactly that role, typed and dispatched.",
            solution: `Replace the ephemeral box at ${relPath}:${lineNo} with the app's Transient lane (ArtifactApp::Transient + ArtifactApp::ephemeral), or — if the value is genuinely render-internal (no plugin render depends on it, no second surface reads it, losing it on re-render breaks only a visual nicety, and it is never persisted) — keep it as ordinary component-local state instead of a module-level box.`,
          });
        }
      });
    }
  }

  return breaches;
}
//#endregion 🔖️StateLaneExhaustiveness

/**
 * 📏️Plugin purity: every `.rs` file under `✏️s/🔌️plugins/` may not perform filesystem, env/process, or
 * network IO, nor own item-scope ambient mutable state (`thread_local!`/`static mut`/`lazy_static!`/
 * item-scope `Mutex`/`RwLock`/`RefCell`/`Cell`/`Atomic*`) — skips `#[cfg(test)]` modules and `#[test]`
 * functions (test-only IO is currently sanctioned, same skip 📓️w0-c-purity.md's census applied). Also
 * scans `.ts`/`.tsx` under the plugin tree for direct browser-side IO (`fetch(`/`XMLHttpRequest`/
 * `WebSocket`/`localStorage`/`sessionStorage`/`indexedDB`).
 */
function policyPluginPurityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];

  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) continue;
    if (POLICY_PLUGIN_PURITY_ALLOWLIST.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const lines = content.split(/\r?\n/);
    const skipSpans = [...policyTestModSpans(lines), ...policyPluginPurityTestFnSpans(lines)];
    let depth = 0;
    const fnBodyDepths: number[] = [];

    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
      if (policyLineInTestMod(skipSpans, lineNo)) {
        depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
        while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
        return;
      }
      const inFn = fnBodyDepths.some((d) => depth >= d);
      const opensFn = !inFn && POLICY_PLUGIN_PURITY_FN_ITEM_RE.test(raw) && /\{/.test(codeOnly);

      const emit = (subKind: string, summaryVerb: string, reason: string, solution: string) => {
        breaches.push({
          id: `plugin-purity-${subKind}-${relPath}-${lineNo}`,
          summary: `"${relPath}:${lineNo}" ${summaryVerb} inside a plugin tree`,
          kind: `taxonomy/plugin-purity-${subKind}`,
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason,
          solution,
        });
      };

      if (POLICY_PLUGIN_PURITY_FS_RE.test(codeOnly)) {
        emit(
          "filesystem-io",
          "performs filesystem IO",
          "APA: all io/state changes belong to artifacts, never apps/setup facets or non-taxonomy subtrees — filesystem access outside an artifact's own ⚙️engine is a purity violation.",
          `Move the filesystem call at ${relPath}:${lineNo} behind an artifact-owned ⚙️engine, or (for a build.rs/bin.rs sibling) confirm with the ticket owner whether APA's shape rule reaches compile-time/CLI code at all — 📓️w0-c-purity.md §1 flags this as UNVERIFIED.`,
        );
      }
      if (POLICY_PLUGIN_PURITY_ENV_RE.test(codeOnly)) {
        emit(
          "env-process-io",
          "reads env/process state",
          "APA: environment/process access is a side effect that must be routed through the OS host, not read directly by plugin code.",
          `Move the env/process call at ${relPath}:${lineNo} behind an artifact-owned config path or an OS host capability.`,
        );
      }
      if (POLICY_PLUGIN_PURITY_NET_RE.test(codeOnly)) {
        emit(
          "network-io",
          "performs network IO",
          "APA: network IO must be routed through the OS host's capability-gated network-fetch effect, never called directly from plugin code.",
          `Replace the direct network call at ${relPath}:${lineNo} with a capability-gated HostEffect (e.g. OpenExternalUrl/DownloadMediaExport), or route it through an artifact's own io.`,
        );
      }

      if (!inFn || POLICY_PLUGIN_PURITY_STATIC_ITEM_RE.test(raw)) {
        if (POLICY_PLUGIN_PURITY_THREAD_LOCAL_RE.test(codeOnly)) {
          emit(
            "thread-local-state",
            "declares thread_local! ambient state",
            "APA: thread_local! has no framework-sanctioned owner in a plugin — every app that needed session state independently reinvented this because ArtifactApp gave it none. That gap is now closed: the four state mechanisms (artifact/config/presence/transient) are all typed lanes on ArtifactApp.",
            `Replace thread_local! at ${relPath}:${lineNo} with the lane that matches what the state actually IS — ephemeral local UI state → the Transient lane (ArtifactApp::Transient, emitted via ArtifactApp::ephemeral); ephemeral shared state → Presence; uncommitted document content → the Draft lane; a cached view of an owned CHILD's content → do not cache at all, read it through ArtifactView.children, which cannot go stale (see 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION/📓️cw2-child-cache-finding.md for the one case still blocked: codecs take no context, so a child-content cache cannot be deleted until the parent's serialized form drops resolved child content in favour of the handle).`,
          );
        }
        if (POLICY_PLUGIN_PURITY_STATIC_MUT_RE.test(raw)) {
          emit(
            "static-mut-state",
            "declares item-scope static mut",
            "APA: item-scope static mut is forbidden ambient state in a plugin tree.",
            `Delete the static mut at ${relPath}:${lineNo} and route ownership through the artifact's own engine or the OS host.`,
          );
        }
        if (POLICY_PLUGIN_PURITY_LAZY_STATIC_RE.test(codeOnly)) {
          emit(
            "lazy-static-state",
            "declares lazy_static! ambient state",
            "APA: lazy_static! process globals are forbidden outside an artifact-owned write-once table.",
            `Replace lazy_static! at ${relPath}:${lineNo} with std OnceLock/LazyLock if genuinely write-once, or route real mutation through the artifact engine.`,
          );
        }
        const interior = codeOnly.match(POLICY_PLUGIN_PURITY_INTERIOR_MUT_RE);
        if (interior) {
          const ty = (interior[1] ?? interior[2])!;
          const tyLower = ty.toLowerCase();
          emit(
            `interior-mutability-${tyLower}`,
            `declares item-scope ${ty} ambient state`,
            "APA: item-scope Mutex/RwLock/RefCell/Cell/Atomic* is real mutable ambient state, not the sanctioned write-once OnceLock/OnceCell/LazyLock lazy-table pattern (which is exempt on its own — only nested Mutex/RwLock/RefCell/Cell inside one still counts).",
            `Move the ${ty} state at ${relPath}:${lineNo} behind the artifact's own dispatch path (Mutex<Vec<...>> contribution registries) or a typed Draft lane (per-app session/scratch state) — see 📓️w0-c-purity.md §4 for the sanctioned-vs-real classification this rule mirrors.`,
          );
        }
      }

      if (opensFn) fnBodyDepths.push(depth + 1);
      depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
      while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
    });
  }

  for (const relPath of policyPluginPurityTsFiles(repoRoot, POLICY_APA_PLUGINS_ROOT)) {
    if (POLICY_PLUGIN_PURITY_ALLOWLIST.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const lines = content.split(/\r?\n/);
    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      const codeOnly = raw.replace(/\/\/.*$/, "");
      const m = codeOnly.match(POLICY_PLUGIN_PURITY_TS_RE);
      if (!m) return;
      breaches.push({
        id: `plugin-purity-ts-side-effect-${relPath}-${lineNo}`,
        summary: `"${relPath}:${lineNo}" calls ${m[1]} directly from a plugin app-tree file`,
        kind: "taxonomy/plugin-purity-ts-side-effect",
        scope: relPath,
        line: lineNo,
        priority: "medium",
        reason: "APA: browser-side IO/ambient-storage APIs (fetch/XMLHttpRequest/WebSocket/localStorage/sessionStorage/indexedDB) must be routed through an artifact, never called directly from an app-tree component — 📓️w0-c-purity.md §3 calls animate's two fetch() sites the cleanest violation in the whole census.",
        solution: `Route the call at ${relPath}:${lineNo} through an artifact-owned io path instead of calling it directly from the app component.`,
      });
    });
  }

  return breaches;
}
//#endregion 🔧️PolicyRulePluginPurity

//#region 🔧️PolicyRuleDeclarativeRegistration
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_DECLARATIVE_REGISTRATION_ALLOWLIST = new Set<string>();

/**
 * 📇️Exhaustive OS-host global-registry-mutator family — union of the assignment's given list, the
 * `register_*` family from 📓️w0-a-escape-hatch.md §1 (`register_os_fixture_json`, and the
 * `register_artifact_descriptor(s)` pair used by `register_app_io`/hot-swap), per §6 of
 * 📓️w0-d-sdk-surface.md. Deliberately excludes `register_studio_port` (confirmed plugin-local to
 * 🪐️space, not a framework SDK fn — w0-d §6) and plain `register_app` (`&mut self` method on
 * `PluginRegistry`, not a free-function global-static mutator — w0-a §1).
 */
const POLICY_REGISTRATION_FAMILY_FNS = [
  "register_mesh_exporter",
  "register_mesh_importer",
  "register_mesh_dwg_export_handler",
  "register_mesh_dwg_import_handler",
  "register_solid_exporter",
  "register_solid_importer",
  "register_2d_export_handlers",
  "register_dwg_import_handler",
  "register_app_io",
  "register_os_media_export_handler_kind",
  "register_os_media_import_handler_kind",
  "register_composer_entries",
  "set_io_fallback_dispatcher",
  "register_subset_validator",
  "register_format_descriptors",
  "register_artifact_schema_descriptor",
  "register_artifact_inference_descriptor",
  "register_app_schema_descriptor",
  "register_language",
  "register_document_codec",
  "register_document_codec_for_app",
  "register_dialect_migration",
  "register_os_fixture_json",
  "register_artifact_descriptors",
  "register_artifact_descriptor",
] as const;
const POLICY_REGISTRATION_FAMILY_RE = new RegExp(String.raw`\b(${POLICY_REGISTRATION_FAMILY_FNS.join("|")})\b`);
const POLICY_REGISTRATION_OS_PATH_RE = /\bsemio_framework_os::/;

/** 🧭️True when `relPath` sits inside a `🗿️artifacts/<kind>/…/⚙️engine/…` subtree — the sole currently-compliant interim registration site per 📓️w0-a-escape-hatch.md §2a. */
function policyRegistrationIsEngineSite(relPath: string): boolean {
  const artifactsIdx = relPath.indexOf("/🗿️artifacts/");
  if (artifactsIdx === -1) return false;
  return relPath.indexOf("/⚙️engine/", artifactsIdx) !== -1;
}

/** 🏗️Builds the (identical-shape) engine-backlog vs violation BreachRecord for a registration-family/os-path hit. */
function policyRegistrationBreach(relPath: string, lineNo: number, isEngine: boolean, subKind: string, what: string): BreachRecord {
  if (isEngine) {
    return {
      id: `plugin-registration-engine-backlog-${subKind}-${relPath}-${lineNo}`,
      summary: `"${relPath}:${lineNo}" calls ${what} from inside its own artifact ⚙️engine — currently-compliant interim shape, migration backlog`,
      kind: "taxonomy/plugin-registration-engine-backlog",
      scope: relPath,
      line: lineNo,
      priority: "medium",
      reason: "APA (w0-a §2a): a registration call inside the owning artifact's own ⚙️engine is the sanctioned interim shape until the M1/M2 declarative ArtifactDeclaration/Registrar mechanism lands — tracked as migration backlog, not a live architecture violation.",
      solution: `Once M1's ArtifactDeclaration/Registrar mechanism lands, convert ${relPath}:${lineNo} from an imperative ${what} call into a declarative .artifact(...) entry.`,
    };
  }
  return {
    id: `plugin-registration-violation-${subKind}-${relPath}-${lineNo}`,
    summary: `"${relPath}:${lineNo}" calls ${what} outside its owning artifact's ⚙️engine — wrong layer for a registration call`,
    kind: "taxonomy/plugin-registration-violation",
    scope: relPath,
    line: lineNo,
    priority: "medium",
    reason: 'APA: registration/IO for a kind belongs to that kind\'s own artifact ⚙️engine — a 🔧️setup facet, app file, pane, panel, command handler, or plugin root calling the global registration family (or reaching into semio_framework_os::) mutates OS-host state from the wrong layer, and can register a kind the caller doesn\'t even own (💠️lowpoly registering "3d.mesh" — w0-a §2b-§2d; the 🎪️demonstrator half of that exemplar is RESOLVED, ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D2 moved its process/procedural/gis/cad registrations to those owners and dropped its semio-framework-os dependency, so a demonstrator hit here is now a regression, not a known backlog).',
    solution: `Delete the ${what} call at ${relPath}:${lineNo} and move the registration into the owning artifact's own 🗿️artifacts/<kind>/…/⚙️engine/ (or, for register_app_io/register_os_fixture_json, into the app's own declarative registration path once M1 lands).`,
  };
}

/**
 * 📏️Declarative registration: under `✏️s/🔌️plugins/`, flags (a) `.setup(` inside a `Plugin::builder(…)`
 * chain — registration must be declarative data, not an imperative callback (M1 retires `PluginBuilder`'s
 * `setup: Option<fn()>`); (b) any call to the exhaustive global registration family; (c) any
 * `semio_framework_os::` path reference (the OS host crate is forbidden to plugins). (b)/(c) are split into
 * two separable kinds: a call inside the owning artifact's own `⚙️engine` is the currently-compliant
 * interim shape (migration backlog); the same call anywhere else (`🔧️setup/`, `🎛️apps/`, `📌️panels/`,
 * `🎮️commands/`, or the plugin root) is a live architecture violation. (`🎪️panes/` used to head that
 * list; the repo's only pane facet, 🎪️demonstrator's, was dissolved into `🎛️apps` by ticket
 * 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D3.)
 */
function policyDeclarativeRegistrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];

  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) continue;
    if (POLICY_DECLARATIVE_REGISTRATION_ALLOWLIST.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);

    const builderRe = /Plugin::builder\s*\(/g;
    let bm: RegExpExecArray | null;
    while ((bm = builderRe.exec(content))) {
      const buildIdx = content.indexOf(".build()", bm.index);
      const chainEnd = buildIdx === -1 ? Math.min(content.length, bm.index + 4000) : buildIdx;
      const chain = content.slice(bm.index, chainEnd);
      const setupMatch = chain.match(/\.setup\s*\(/);
      if (!setupMatch) continue;
      const idx = bm.index + setupMatch.index!;
      const lineNo = policyLineOfIndex(content, idx);
      breaches.push({
        id: `plugin-registration-setup-callback-${relPath}-${lineNo}`,
        summary: `"${relPath}:${lineNo}" registers via .setup(...) callback inside Plugin::builder — registration must be declarative data, not an imperative hook`,
        kind: "taxonomy/plugin-registration-setup-callback",
        scope: relPath,
        line: lineNo,
        priority: "medium",
        reason: "APA/W1 (📓️w1-mechanism-design.md M1): PluginBuilder's setup: Option<fn()> + .setup(...) + the `if let Some(setup) = self.setup { setup(); }` call are retired in favor of declarative ArtifactDeclaration data walked by build() in a fixed order — an imperative callback can register anything in any order, defeating the ownership check the declarative form enforces structurally.",
        solution: `Replace .setup(...) at ${relPath}:${lineNo} with .artifact(ArtifactDeclaration { .. }) entries once the M1 declarative registration mechanism lands (W1); until then this is tracked, not blocking.`,
      });
    }

    const lines = content.split(/\r?\n/);
    const isEngine = policyRegistrationIsEngineSite(relPath);
    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
      const fnMatch = codeOnly.match(POLICY_REGISTRATION_FAMILY_RE);
      if (fnMatch) breaches.push(policyRegistrationBreach(relPath, lineNo, isEngine, "family-call", `${fnMatch[1]}(...)`));
      if (POLICY_REGISTRATION_OS_PATH_RE.test(codeOnly)) breaches.push(policyRegistrationBreach(relPath, lineNo, isEngine, "os-host-path", "semio_framework_os::"));
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDeclarativeRegistration

//#region 🔧️PolicyRulePluginDependencyAllowlist
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_PLUGIN_DEP_ALLOWLIST = new Set<string>();

/** ✅️What a plugin legitimately needs per 📓️w0-d-sdk-surface.md §3.1: the plugin SDK, the os-kernel crate, 3d where genuinely used, and schema. Everything else pointing into 🧰️framework/ is a bypass of the curated SDK surface. */
const POLICY_PLUGIN_DEP_FRAMEWORK_ALLOWLIST = new Set(["semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-3d", "semio-framework-schema"]);
/** ✅️serde-family third-party crates are always allowed regardless of how they're declared (version-pinned, not path-pointed at 🧰️framework, so they never reach this rule's path filter anyway — kept for defense in depth). */
const POLICY_PLUGIN_DEP_SERDE_RE = /^serde(_json|-wasm-bindgen)?$/;

/**
 * 📇️Exact `semio_framework_os::` symbols each plugin's own `.rs` files use (📓️w0-d-sdk-surface.md §3.2)
 * — keyed by plugin owner dir name, folded into the breach `solution` so the SDK re-export fix is
 * obvious from the breach alone. An empty array means the dependency is declared but 0 uses were found
 * (UNVERIFIED why — w0-d flags ✒️writer/📐️cad/🔱️trinity/🖍️draw as likely stale/unused declarations).
 */
const POLICY_PLUGIN_DEP_OS_SYMBOLS: Readonly<Record<string, readonly string[]>> = {
  "✒️writer": [],
  "🌀️procedural": ["register_mesh_dwg_import_handler"],
  "🌍️gis": ["DwgColor", "DwgEntity", "register_2d_export_handlers", "register_dwg_import_handler"],
  // 🎪️ BANNED, not downgraded (ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION
  // D2): the demonstrator's twelve foreign-kind registrations moved to their owners (📐️cad, 🌍️gis),
  // and `semio-framework-os` is gone from its Cargo.toml. The empty array is the ratchet — if the
  // dependency is ever reintroduced, `policyPluginDependencyAllowlistBreaches` flags it with no
  // carve-out to soften the message.
  "🎪️demonstrator": [],
  "🏭️process": ["register_mesh_dwg_import_handler"],
  "📏️layout": ["DwgColor", "DwgDrawing", "DwgEntity", "DwgGeometry"],
  "📐️cad": ["register_dwg_import_handler", "register_mesh_dwg_export_handler", "register_mesh_exporter", "register_mesh_importer", "register_solid_exporter", "register_solid_importer"],
  "🎥️shooting": ["rasterize_svg_to_png_base64"],
  "🎞️animate": ["dwg_drawing_to_svg", "rasterize_svg_to_png_base64", "title_card_svg"],
  "💠️lowpoly": ["register_mesh_dwg_export_handler", "register_mesh_dwg_import_handler", "register_mesh_exporter", "register_mesh_importer"],
  "📸️remodel": ["OsMediaExportResult"],
  "🗒️note": ["svg_to_dwg_bytes"],
  "🔱️trinity": [],
  "🖍️draw": [],
  "🧩️puzzle": ["register_2d_export_handlers", "register_dwg_import_handler", "register_mesh_dwg_export_handler", "register_mesh_dwg_import_handler", "register_mesh_exporter", "register_mesh_importer"],
  "🪐️space": ["APP_REGISTRATIONS", "DwgDrawing", "OS_HOME_VFS_ROOT_ID", "OS_SPACE_SCHEMA", "OsBackbonePort", "OsMediaCapability", "OsMediaExportResult", "OsParameter", "OsParameterFieldBinding", "OsParameterType", "OsSpaceCatalogEntry", "OsWorkflowCamera", "SpaceKind", "SpaceVisibility", "VcsError", "Workflow", "WorkflowNode", "WorkflowSnapshot", "delete_os_space", "dwg_to_bytes", "empty_space_snapshot", "empty_workflow_snapshot", "export_os_app_instance_media_kind", "host", "import_os_app_instance_media_kind", "import_os_space_from_dsl", "list_os_space_catalog_entries", "media_accept_filter_kinds", "open_file_space_backbone", "open_folder_space_backbone", "os_parameter_types_compatible", "os_workflow_to_flow_fixture", "register_app_io", "register_dwg_import_handler", "validate_workflow", "workflow"],
  "🖨️raster": ["DwgColor", "DwgDrawing", "DwgEntity", "DwgGeometry", "rasterize_svg_to_png_base64"],
};

/** 🔑Owner dir name (with emoji) for a repo-relative path under `✏️s/🔌️plugins/<owner>/…` — `""` if not under the plugins tree. */
function policyPluginOwnerFromApaPath(relPath: string): string {
  if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) return "";
  return relPath.slice(POLICY_APA_PLUGINS_ROOT.length + 1).split("/")[0] ?? "";
}

/** 🔎️Every `[dependencies]` / `[target.'cfg(...)'.dependencies]` section header's byte offset (start of the section body) in `content`, matching the shape 🧩️puzzle's cfg-gated `semio-framework-os` dependency actually uses (📓️w0-d-sdk-surface.md §3.2). Deliberately excludes `[dev-dependencies]` — dev/test-time deps are out of this rule's runtime-purity scope. */
const POLICY_PLUGIN_DEP_SECTION_RE = /^\[(?:dependencies|target\.[^\]]*\.dependencies)\]\s*$/gm;
const POLICY_PLUGIN_DEP_ANY_SECTION_RE = /^\[[^\]]*\]\s*$/gm;
const POLICY_PLUGIN_DEP_LINE_RE = /^([A-Za-z0-9_.-]+)\s*=\s*\{([^}]*)\}\s*$/gm;

/**
 * 📏️Plugin dependency allowlist: parses every `[dependencies]`/`[target.'cfg(...)'.dependencies]` block
 * in every `Cargo.toml` under `✏️s/🔌️plugins/<owner>/.../📦️packages/🦀️rust/` and flags every dependency whose
 * `path =` points into `🧰️framework/` and whose crate name (the `package = "..."` override, or the TOML
 * key) is outside `POLICY_PLUGIN_DEP_FRAMEWORK_ALLOWLIST`. `semio-framework-os` (the HOST crate) is the
 * headline forbidden one and gets its own kind + the exact offending symbol list in `solution`.
 */
function policyPluginDependencyAllowlistBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverCargoTomlFiles(repoRoot)) {
    if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) continue;
    if (!relPath.includes("/📦️packages/🦀️rust/")) continue;
    if (POLICY_PLUGIN_DEP_ALLOWLIST.has(relPath)) continue;
    const owner = policyPluginOwnerFromApaPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);

    const sectionStarts: number[] = [];
    POLICY_PLUGIN_DEP_SECTION_RE.lastIndex = 0;
    let sm: RegExpExecArray | null;
    while ((sm = POLICY_PLUGIN_DEP_SECTION_RE.exec(content))) sectionStarts.push(sm.index + sm[0].length);
    const allStarts: number[] = [];
    POLICY_PLUGIN_DEP_ANY_SECTION_RE.lastIndex = 0;
    let am: RegExpExecArray | null;
    while ((am = POLICY_PLUGIN_DEP_ANY_SECTION_RE.exec(content))) allStarts.push(am.index);

    for (const start of sectionStarts) {
      const end = allStarts.find((s) => s > start) ?? content.length;
      const block = content.slice(start, end);
      POLICY_PLUGIN_DEP_LINE_RE.lastIndex = 0;
      let m: RegExpExecArray | null;
      while ((m = POLICY_PLUGIN_DEP_LINE_RE.exec(block))) {
        const key = m[1]!;
        const body = m[2]!;
        const pathMatch = body.match(/path\s*=\s*"([^"]*)"/);
        if (!pathMatch || !pathMatch[1]!.includes("🧰️framework")) continue;
        const pkgMatch = body.match(/package\s*=\s*"([^"]*)"/);
        const crateName = pkgMatch ? pkgMatch[1]! : key;
        if (POLICY_PLUGIN_DEP_FRAMEWORK_ALLOWLIST.has(crateName) || POLICY_PLUGIN_DEP_SERDE_RE.test(crateName)) continue;
        const lineNo = policyLineOfIndex(content, start + m.index);
        const isOsHost = crateName === "semio-framework-os";
        const symbols = isOsHost ? (POLICY_PLUGIN_DEP_OS_SYMBOLS[owner] ?? []) : [];
        breaches.push({
          id: `plugin-dependency-allowlist-${relPath}-${crateName}`,
          summary: isOsHost
            ? `"${relPath}" depends on the forbidden HOST crate "semio-framework-os"${symbols.length ? ` (uses: ${symbols.join(", ")})` : " (0 semio_framework_os:: uses found — likely a stale/unused dependency)"}`
            : `"${relPath}" depends on framework crate "${crateName}", outside the plugin dependency allowlist`,
          kind: isOsHost ? "taxonomy/plugin-dependency-os-host" : "taxonomy/plugin-dependency-allowlist",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: isOsHost
            ? "APA/M3: semio-framework-os is the OS HOST crate — plugins may depend only on semio-framework-plugin, the os-kernel crate, semio-framework-3d, semio-framework-schema, and serde-family third-party crates; the host crate's types/fns must be re-exported through the SDK instead (📓️w0-d-sdk-surface.md §3.2)."
            : "APA/M3: the plugin dependency allowlist is semio-framework-plugin + the os-kernel crate + semio-framework-3d (where genuinely used) + semio-framework-schema + serde-family — every other framework crate is a bypass of the curated SDK surface (📓️w0-d-sdk-surface.md §3.1).",
          solution: isOsHost
            ? symbols.length > 0
              ? `Remove "${crateName}" from ${relPath} and add explicit re-exports for [${symbols.join(", ")}] to semio_framework_plugin's tail block instead (M3).`
              : `Remove the unused "${crateName}" dependency from ${relPath} — 0 semio_framework_os:: uses found in this crate's own .rs files (double-check build.rs/feature-gated code before deleting).`
            : `Remove "${crateName}" from ${relPath}, or add an explicit re-export for whatever it provides to semio_framework_plugin's SDK surface (M3) instead of depending on the framework crate directly.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRulePluginDependencyAllowlist

//#region 🔧️PolicyRuleEffectCapabilityParity
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_EFFECT_CAPABILITY_ALLOWLIST = new Set<string>();

/**
 * 📇️`HostEffect` variant → required capability `ArtifactKind`, machine-readable form of the effect→
 * capability contract (📓️w0-d-sdk-surface.md §5, `🎠️kernel/🦀️component.rs:247-387`). Rights/Scope are
 * noted per-line for provenance but the parity check below matches at `ArtifactKind` granularity only:
 * no plugin has ever called `.capability(...)` for real yet (0 hits repo-wide) to pin the exact
 * Rights/Scope argument shape the eventual `has_capability(ArtifactKind, Rights)` gate (M5) will take, so
 * matching finer than ArtifactKind would be guessing at an unimplemented signature. `ArtifactKind` itself
 * has no "Shell"/"UI" member yet, so every window-chrome effect below maps onto the closest existing fit
 * (`Window`) per w0-d's own proposal — flagged there as needing W1/W2 confirmation before this becomes
 * policy, not yet resolved by this ticket.
 */
const POLICY_HOST_EFFECT_CAPABILITY: Readonly<Record<string, string>> = {
  OpenWindow: "Window", // Rights::Open, Scope::Instance
  CloseWindow: "Window", // Rights::Write, Scope::Instance
  Notify: "Window", // Rights::Write, Scope::Instance
  ClipboardWrite: "Document", // Rights::Write, Scope::Global
  RequestSync: "Backbone", // Rights::Invoke, Scope::Instance
  Navigate: "Window", // Rights::Write, Scope::Global
  LoadDocument: "Document", // Rights::Write, Scope::Instance
  OpenExternalUrl: "Network", // Rights::Open, Scope::Global
  SetPanel: "Window", // Rights::Write, Scope::Instance
  DownloadMediaExport: "Asset", // Rights::Write, Scope::Instance
  IconRenderExport: "Asset", // Rights::Write, Scope::Instance
  RequestFileOpen: "Asset", // Rights::Open, Scope::Instance
  RequestMediaFrames: "Asset", // Rights::Open, Scope::Instance
  SpawnPluginInstance: "Window", // Rights::Open, Scope::Global
  OpenPluginInstance: "Window", // Rights::Open, Scope::Global
  SetActiveUtility: "Window", // Rights::Write, Scope::Instance
  SetActiveTool: "Window", // Rights::Write, Scope::Instance
  OpenDialog: "Window", // Rights::Open, Scope::Instance
  DispatchAction: "Window", // Rights::Invoke, Scope::Instance
  ReplayShellCommand: "Window", // Rights::Invoke, Scope::Global
  PatchWorld3dChrome: "Window", // Rights::Write, Scope::Instance
  InvokeExtension: "Engine", // Rights::Invoke, Scope::Plugin
};

const POLICY_HOST_EFFECT_CONSTRUCT_RE = /\bEffect::(\w+)\b/g;
const POLICY_CAPABILITY_CALL_RE = /\.capability\s*\(\s*ArtifactKind::(\w+)/g;
const POLICY_BACKBONE_STORAGE_RE = /\.local_backbone_storage\s*\(/;

/**
 * 📏️Effect/capability parity: for each plugin, collects every `HostEffect::<Variant>` constructed
 * anywhere in its tree (skipping `#[cfg(test)]` spans) and the capabilities it declares in its root
 * `🦀️component.rs` (`.capability(ArtifactKind::X, …)` and `.local_backbone_storage()`, which stands in
 * for `ArtifactKind::Backbone`) — flags every constructed variant whose required `ArtifactKind` isn't
 * declared. Expected to fire on nearly every effect-constructing plugin: today only 🪐️space declares any
 * capability at all, and its lone `.local_backbone_storage()` doesn't satisfy any of the `Window`/
 * `Document`/`Asset` requirements its own 6 constructed variants need either.
 */
function policyEffectCapabilityParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyReaddirSafe(repoRoot, POLICY_APA_PLUGINS_ROOT)) {
    if (!owner.isDirectory) continue;
    const ownerRel = `${POLICY_APA_PLUGINS_ROOT}/${owner.name}`;
    if (POLICY_EFFECT_CAPABILITY_ALLOWLIST.has(ownerRel)) continue;

    const declared = new Set<string>();
    const rootLeaf = join(repoRoot, ownerRel, "🦀️component.rs");
    if (existsSync(rootLeaf)) {
      const rootContent = readFileSync(rootLeaf, "utf8");
      POLICY_CAPABILITY_CALL_RE.lastIndex = 0;
      let cm: RegExpExecArray | null;
      while ((cm = POLICY_CAPABILITY_CALL_RE.exec(rootContent))) declared.add(cm[1]!);
      if (POLICY_BACKBONE_STORAGE_RE.test(rootContent)) declared.add("Backbone");
    }

    const constructedByVariant = new Map<string, { relPath: string; line: number }[]>();
    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.isDirectory) {
          walk(childRel);
          continue;
        }
        if (!entry.name.endsWith(".rs")) continue;
        const content = policyReadFileSafe(repoRoot, childRel);
        const lines = content.split(/\r?\n/);
        const testSpans = policyTestModSpans(lines);
        lines.forEach((raw, i) => {
          const lineNo = i + 1;
          if (policyLineInTestMod(testSpans, lineNo)) return;
          const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
          POLICY_HOST_EFFECT_CONSTRUCT_RE.lastIndex = 0;
          let vm: RegExpExecArray | null;
          while ((vm = POLICY_HOST_EFFECT_CONSTRUCT_RE.exec(codeOnly))) {
            const variant = vm[1]!;
            if (!(variant in POLICY_HOST_EFFECT_CAPABILITY)) continue;
            const list = constructedByVariant.get(variant) ?? [];
            list.push({ relPath: childRel, line: lineNo });
            constructedByVariant.set(variant, list);
          }
        });
      }
    };
    walk(ownerRel);

    for (const [variant, sites] of constructedByVariant) {
      const requiredKind = POLICY_HOST_EFFECT_CAPABILITY[variant]!;
      if (declared.has(requiredKind)) continue;
      const first = sites[0]!;
      breaches.push({
        id: `effect-capability-parity-${ownerRel}-${variant}`,
        summary: `"${ownerRel}" constructs HostEffect::${variant} (${sites.length} site${sites.length === 1 ? "" : "s"}, first at ${first.relPath}:${first.line}) without declaring the ${requiredKind} capability it requires`,
        kind: "taxonomy/effect-capability-parity",
        scope: ownerRel,
        line: first.line,
        priority: "medium",
        reason: "APA/M5: every HostEffect variant maps to a CapabilityRequirement (📓️w0-d-sdk-surface.md §5) — a plugin that constructs the effect without declaring the matching capability makes today's capability system decorative; only 🪐️space declares any capability repo-wide, and it doesn't cover any of the effects plugins actually construct.",
        solution: `Add .capability(ArtifactKind::${requiredKind}, ...) to ${ownerRel}/🦀️component.rs's Plugin::builder chain once M5's has_capability(ArtifactKind, Rights) gate lands, or remove the HostEffect::${variant} construction if the plugin shouldn't have this capability.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleEffectCapabilityParity

//#region 🔧️PolicyRuleApaRatchet
/**
 * 🚦W5 shrink-only ceiling table for APA's five report-mode rules. A rule's kind at or under its ceiling
 * stays entirely at `priority: "medium"` (census, non-gating); only breaches ABOVE the ceiling — the
 * measured regression — are promoted to `priority: "high"`, which `VerifyScript.runGate`'s
 * `dissolveBreaches` block already filters on. Ceilings MAY be lowered freely as debt burns down; RAISING
 * one requires a ticket citation in a comment beside the changed entry, or the ratchet is meaningless.
 * Measured `bun ./📜️script.ts policy` against `.🧬semio/🦑️repo/⚡️cache/breaches/compose.json`, three runs
 * 2026-08-13 ~00:12/00:14/00:16 (~90-100s apart) — see `📓️w5-ratchet-report.md`.
 *
 * Four of eight APA ratchet keys are intentionally ABSENT below — no ceiling, permanently `medium` — not
 * from a stale prior estimate but because the three-run measurement itself showed each one moving:
 * - `plugin-registration-engine-backlog` — coordinator-flagged: a concurrent peer wave (9 agents) is
 *   actively dissolving artifact ⚙️engine/ dirs and relocating register*() calls out of them right now; a
 *   move leaves code transiently in both the old and new location, which can legitimately INCREASE this
 *   count for a few minutes. A shrink-only ceiling here would gate the shared tree on the peer's own
 *   in-flight progress.
 * - `plugin-registration-violation` — measured 570 → 580 → 600 across the three runs; the same dual-write
 *   window reaches this kind too (a register*() call briefly lands outside any ⚙️engine mid-move).
 * - `plugin-registration-setup-callback` — measured 14 → 14 → 15, in the same window `📓️status.md`'s
 *   "FINAL STATE" section describes the `.setup()` 33→11 conversion landing in.
 * - `plugin-purity` (all `taxonomy/plugin-purity-*` sub-kinds summed) — measured 116 → 118 → 125
 *   (filesystem-io alone: 35 → 35 → 42, interior-mutability-mutex: 20 → 22 → 22). Despite being
 *   inventory-only by APA's own original design (nothing in this ticket ever tried to reduce it), the
 *   underlying file set is not static tonight, so it gets the same exemption on measured evidence rather
 *   than on the design intent alone.
 */
const POLICY_APA_RATCHET_CEILINGS: Readonly<Record<string, number>> = {
  "plugin-closed-shape": 41, // flat 41/41/41 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
  "plugin-dependency-allowlist": 105, // flat 105/105/105 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
  "plugin-dependency-os-host": 10, // flat 10/10/10 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
  "effect-capability-parity": 47, // flat 47/47/47 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
};

/** 🔑Ratchet grouping key for a breach `kind` — every `taxonomy/plugin-purity-*` sub-kind (filesystem-io, interior-mutability-*, thread-local-state, ts-side-effect, …) collapses onto one shared `"plugin-purity"` key matching the ceiling table's single combined row; every other APA kind keeps a 1:1 key. */
function policyApaRatchetKey(kind: string): string {
  if (kind.startsWith("taxonomy/plugin-purity-")) return "plugin-purity";
  return kind.startsWith("taxonomy/") ? kind.slice("taxonomy/".length) : kind;
}

/**
 * 🚦Applies the shrink-only ratchet: groups `breaches` by `policyApaRatchetKey`, and for any key present in
 * `POLICY_APA_RATCHET_CEILINGS`, keeps the first `ceiling` breaches at their original `medium` priority and
 * promotes only the breaches beyond the ceiling to `high` with a regression message naming the key, the
 * ceiling, and the measured count. Keys absent from the table pass through untouched, always `medium`.
 */
function policyApaRatchetApply(breaches: readonly BreachRecord[]): BreachRecord[] {
  const grouped = new Map<string, BreachRecord[]>();
  for (const b of breaches) {
    const key = policyApaRatchetKey(b.kind);
    const list = grouped.get(key);
    if (list) list.push(b);
    else grouped.set(key, [b]);
  }
  const out: BreachRecord[] = [];
  for (const [key, list] of grouped) {
    const ceiling = POLICY_APA_RATCHET_CEILINGS[key];
    if (ceiling === undefined || list.length <= ceiling) {
      out.push(...list);
      continue;
    }
    list.forEach((b, i) => {
      if (i < ceiling) {
        out.push(b);
        return;
      }
      out.push({
        ...b,
        priority: "high",
        summary: `RATCHET REGRESSION on "${key}": ceiling is ${ceiling}, measured ${list.length} — shrink-only, raising the ceiling requires a ticket citation — ${b.summary}`,
      });
    });
  }
  return out;
}

/**
 * 🎯Combined, ratcheted breach set for all five APA rules — the single call site both `policy`'s
 * aggregator and `VerifyScript.runGate`'s `dissolveBreaches` block use, so census and gate can never
 * disagree on a breach's priority.
 */
function policyApaBreaches(repoRoot: string): BreachRecord[] {
  return policyApaRatchetApply([
    ...policyPluginClosedShapeBreaches(repoRoot),
    ...policyPluginPurityBreaches(repoRoot),
    ...policyStateLaneExhaustivenessBreaches(repoRoot),
    ...policyDeclarativeRegistrationBreaches(repoRoot),
    ...policyPluginDependencyAllowlistBreaches(repoRoot),
    ...policyEffectCapabilityParityBreaches(repoRoot),
  ]);
}
//#endregion 🔧️PolicyRuleApaRatchet

//#endregion 🔧️PolicyRuleArtifactsOnlyPluginArchitecture


//#region 🔧️PolicyRuleSubsetConformance
/** 🪆️ Subset ownership + roundtrip readiness policies (ticket 26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS). Medium until seal. */

function policyListTopLevelSubsetDirs(repoRoot: string): string[] {
  const taxonomy = loadTaxonomy();
  const out: string[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return out;
  for (const plugin of readdirSync(pluginsRoot, { withFileTypes: true })) {
    if (!plugin.isDirectory()) continue;
    const arts = join(pluginsRoot, plugin.name, taxonomy.artifactsDirName ?? "🗿️artifacts");
    if (!existsSync(arts)) continue;
    for (const art of readdirSync(arts, { withFileTypes: true })) {
      if (!art.isDirectory()) continue;
      const standards = join(arts, art.name, taxonomy.standardsDirName ?? "🏅️standards");
      if (!existsSync(standards)) continue;
      for (const std of readdirSync(standards, { withFileTypes: true })) {
        if (!std.isDirectory() || !std.name.startsWith(taxonomy.standardDirPrefix ?? "🔖️")) continue;
        const subsets = join(standards, std.name, taxonomy.subsetsDirName ?? "🪆️subsets");
        if (!existsSync(subsets)) continue;
        for (const sub of readdirSync(subsets, { withFileTypes: true })) {
          if (!sub.isDirectory() || !sub.name.startsWith(taxonomy.subsetDirPrefix ?? "✳️")) continue;
          out.push(join("✏️s/🔌️plugins", plugin.name, taxonomy.artifactsDirName ?? "🗿️artifacts", art.name, taxonomy.standardsDirName ?? "🏅️standards", std.name, taxonomy.subsetsDirName ?? "🪆️subsets", sub.name).replaceAll("\\", "/"));
        }
      }
    }
  }
  return out;
}

/** 🪆️ Every subset must own schema, engine, io, and (eventually) examples. */
export function policySubsetFacetTotalityBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const required = taxonomy.subsetChildDirs ?? ["🧬️schema", "⚙️engine", "🚪️io", "📚️examples"];
  const breaches: BreachRecord[] = [];
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    for (const child of required) {
      // examples are required structurally but many are still relocating — keep medium
      if (!existsSync(join(repoRoot, subRel, child))) {
        breaches.push({
          id: `subset-facet-missing-${subRel}/${child}`,
          summary: `"${subRel}" is missing required child ${child}/`,
          kind: "subset-conformance/facet-totality",
          scope: subRel,
          priority: "medium",
          reason: "Subsets own schema, engine, IO, and examples.",
          solution: `Create ${subRel}/${child}/ as part of the subset ownership migration.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * ⚙️ An artifact is a `🧬️schema` (snapshot, diff, mutations, inferences) plus a `🚪️io` system — never
 * an engine. Behaviour belongs to the app that edits the artifact (`🎛️apps/<app>/⚙️engine`, already a
 * required app component); pure algorithms belong one level up, in a module's `⚙️engine`, which
 * `taxonomyLeafParentDirs` keeps globally legal.
 *
 * Replaces the two rules that previously *mandated* the facet — `policySubsetEnginePresenceBreaches`
 * and `policyArtifactEnginePresenceBreaches` — the latter of which gated at `high` on a missing
 * folder. Both served `ArtifactEngine`, a trait that never shipped (`grep -rn "trait ArtifactEngine"`
 * → 0 hits; see `🏪️store/🦀️component.rs`'s own note that it "never existed as a live trait").
 *
 * Lands at `low` while the 95 existing dirs burn down, then rises to `high` once the count is zero —
 * verified by counting directories on disk, never via the breach cache. Ticket
 * 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES (#2553).
 */
export function policyArtifactEngineFacetForbiddenBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    for (const owner of policyArtifactEngineOwnerDirs(repoRoot, artRel)) {
      if (!existsSync(join(repoRoot, join(owner, POLICY_ENGINE_FACET)))) continue;
      breaches.push({
        id: `artifact-engine-forbidden-${owner}`,
        summary: `"${owner}" declares a forbidden ${POLICY_ENGINE_FACET}/ facet`,
        kind: "subset-conformance/engine-forbidden",
        scope: owner,
        priority: "low",
        reason: "An artifact has a schema (snapshot, diff, mutations, inferences), an io system and examples — never an engine.",
        solution: `Dissolve ${owner}/${POLICY_ENGINE_FACET} per the D0–D6 procedure: derived compute → 🧬️schema/💡️inferences, (de)serialization → 🚪️io, edits → a 🧬️mutations triad, behaviour → the app's ⚙️engine, pure algorithms → a module's ⚙️engine one level up. Then delete the directory.`,
      });
    }
  }
  return breaches;
}

/** ⚙️ Every level of an artifact tree that could carry an `⚙️engine`: the artifact, its standards, and their subsets. */
function policyArtifactEngineOwnerDirs(repoRoot: string, artRel: string): string[] {
  const owners = [artRel];
  const standardsRel = join(artRel, "🏅️standards");
  for (const std of policyReaddirSafe(repoRoot, standardsRel).filter((e) => e.isDirectory)) {
    const stdRel = join(standardsRel, std.name);
    owners.push(stdRel);
    const subsetsRel = join(stdRel, "🪆️subsets");
    for (const subset of policyReaddirSafe(repoRoot, subsetsRel).filter((e) => e.isDirectory)) {
      owners.push(join(subsetsRel, subset.name));
    }
  }
  return owners;
}

/** 📚️ Examples must not remain at artifact/standard level. */
export function policyExampleNotAtArtifactLevelBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot, { withFileTypes: true })) {
    if (!plugin.isDirectory()) continue;
    const arts = join(pluginsRoot, plugin.name, "🗿️artifacts");
    if (!existsSync(arts)) continue;
    for (const art of readdirSync(arts, { withFileTypes: true })) {
      if (!art.isDirectory()) continue;
      const artEx = join(arts, art.name, "📚️examples");
      if (existsSync(artEx)) {
        const rel = join("✏️s/🔌️plugins", plugin.name, "🗿️artifacts", art.name, "📚️examples").replaceAll("\\", "/");
        breaches.push({
          id: `artifact-examples-remaining-${rel}`,
          summary: `artifact-level examples still present at "${rel}"`,
          kind: "subset-conformance/example-placement",
          scope: rel,
          priority: "medium",
          reason: "Examples belong under 🪆️subsets/<id>/📚️examples/.",
          solution: "Move example units into the owning subset then delete the artifact examples dir.",
        });
      }
      const standards = join(arts, art.name, "🏅️standards");
      if (!existsSync(standards)) continue;
      for (const std of readdirSync(standards, { withFileTypes: true })) {
        if (!std.isDirectory()) continue;
        const stdEx = join(standards, std.name, "📚️examples");
        if (existsSync(stdEx)) {
          const rel = join("✏️s/🔌️plugins", plugin.name, "🗿️artifacts", art.name, "🏅️standards", std.name, "📚️examples").replaceAll("\\", "/");
          breaches.push({
            id: `standard-examples-remaining-${rel}`,
            summary: `standard-level examples still present at "${rel}"`,
            kind: "subset-conformance/example-placement",
            scope: rel,
            priority: "medium",
            reason: "Examples belong under 🪆️subsets/<id>/📚️examples/.",
            solution: "Move into the owning subset then delete the standard examples dir.",
          });
        }
      }
    }
  }
  return breaches;
}

/** 🧹 Phantom Chinese 🏅️标准 trees must be deleted. */
export function policyPhantomStandardsBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  const walk = (dir: string, rel: string) => {
    if (!existsSync(dir)) return;
    for (const ent of readdirSync(dir, { withFileTypes: true })) {
      if (!ent.isDirectory()) continue;
      const childRel = `${rel}/${ent.name}`.replaceAll("\\", "/");
      if (ent.name.includes("标准")) {
        breaches.push({
          id: `phantom-standards-${childRel}`,
          summary: `corrupt standards tree "${childRel}"`,
          kind: "subset-conformance/phantom-standards",
          scope: childRel,
          priority: "medium",
          reason: "Chinese-named 🏅️标准 directories are corrupt phantoms.",
          solution: "Delete the entire phantom tree.",
        });
      }
      walk(join(dir, ent.name), childRel);
    }
  };
  walk(pluginsRoot, "✏️s/🔌️plugins");
  return breaches;
}

/** 🪞 Meta-only TypeScript subset schema leaves (≤7 lines) must become real mirrors. */
export function policySubsetTsParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    const ts = join(repoRoot, subRel, "🧬️schema", "🟦️component.ts");
    if (!existsSync(ts)) continue;
    const lines = readFileSync(ts, "utf8").split(/\r?\n/).length;
    if (lines > 0 && lines <= 7) {
      breaches.push({
        id: `subset-ts-meta-stub-${subRel}`,
        summary: `"${subRel}/🧬️schema/🟦️component.ts" is a ${lines}-line meta stub`,
        kind: "subset-conformance/ts-parity",
        scope: `${subRel}/🧬️schema/🟦️component.ts`,
        priority: "medium",
        reason: "Derived/owning subsets require a real TypeScript implementation mirroring Rust.",
        solution: "Replace the meta stub with a full TypeScript mirror of the Rust conformance/schema surface.",
      });
    }
  }
  return breaches;
}

/** 🚫 Glue compatibility shims keeping pre-migration module paths must be deleted. */
export function policyNoGlueShimBlocksBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot, { withFileTypes: true })) {
    if (!plugin.isDirectory()) continue;
    const glue = join(pluginsRoot, plugin.name, "📦️packages/🦀️rust/📦️glue.rs");
    if (!existsSync(glue)) continue;
    const content = readFileSync(glue, "utf8");
    if (/Shims:\s*keep pre-migration|pre-migration module paths/i.test(content)) {
      const rel = join("✏️s/🔌️plugins", plugin.name, "📦️packages/🦀️rust/📦️glue.rs").replaceAll("\\", "/");
      breaches.push({
        id: `glue-shim-${rel}`,
        summary: `"${rel}" still contains pre-migration shim blocks`,
        kind: "subset-conformance/no-glue-shim-blocks",
        scope: rel,
        priority: "medium",
        reason: "Greenfield forbids compatibility shims.",
        solution: "Delete shim blocks after generated subset registration lands.",
      });
    }
  }
  return breaches;
}

/** 📦 Aggregate subset-conformance scanners. */
export function policySubsetConformanceBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policySubsetFacetTotalityBreaches(repoRoot),
    ...policyArtifactEngineFacetForbiddenBreaches(repoRoot),
    ...policyExampleNotAtArtifactLevelBreaches(repoRoot),
    ...policyPhantomStandardsBreaches(repoRoot),
    ...policySubsetTsParityBreaches(repoRoot),
    ...policyNoGlueShimBlocksBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleSubsetConformance


//#region 🔧️PolicyRuleArtifactViewersEditors
/**
 * 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, contract §6 "Policies" (W0/W1,
 * add-only). Four rules over the new `👁️viewer`/`✏️editor` subset-surface axis, driven entirely by
 * `🔣️taxonomy.json` (`viewerDirName`/`editorDirName`/`surfaceRoles`/`surfaceDirNames`/
 * `windowLeafLangs`/`taxonomyLeafFilenames`) — never a hardcoded emoji literal for the axis itself.
 * `policySubsetSurfaceCompletenessBreaches` and `policyViewerPurityBreaches` land at `"medium"`/`"high"`
 * respectively per the contract; `policyOsConfigShapeBreaches` locks a facet that already shipped
 * (C4), so it fails the gate like any other completed-shape lock.
 */

/** 👁️✏️ Every OWNED subset (has `🧬️schema`, independent of `🚪️io` — see the scaffolder's identical
 * predicate in `📇️registry/📜️script.ts`) carries both `👁️viewer`/`✏️editor`, each with ≥1 mode
 * carrying ≥1 window with both `windowLeafLangs` leaves. A surface whose scaffolded leaves still carry
 * the `SCAFFOLD` marker is a separate, non-passing breach kind — see the scaffolder region docstring
 * in `📇️registry/📜️script.ts` for why the marker exists. */
export function policySubsetSurfaceCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const scaffoldLeafNames = new Set([taxonomy.taxonomyLeafFilenames["🦀️rust"], taxonomy.taxonomyLeafFilenames["🟦️typescript"]].filter((name): name is string => !!name));
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    if (!existsSync(join(repoRoot, subRel, "🧬️schema"))) continue;
    for (const role of taxonomy.surfaceRoles) {
      const surfaceDirName = taxonomy.surfaceDirNames[role];
      const surfaceRel = `${subRel}/${surfaceDirName}`;
      if (!existsSync(join(repoRoot, surfaceRel))) {
        breaches.push({
          id: `subset-surface-missing-${surfaceRel}`,
          summary: `"${subRel}" is missing required surface "${surfaceDirName}"`,
          kind: "taxonomy/surface-completeness",
          scope: subRel,
          priority: "medium",
          reason: `Every owned subset carries both ${taxonomy.subsetRequiredSurfaceDirs.join(", ")}.`,
          solution: `cd 📇️registry && bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> ${role} (or "new surface --all" to batch every missing surface).`,
        });
        continue;
      }
      const modesRel = `${surfaceRel}/${taxonomy.modesDirName}`;
      let hasCompleteWindow = false;
      for (const mode of policyReaddirSafe(repoRoot, modesRel).filter((e) => e.isDirectory)) {
        const windowsRel = `${modesRel}/${mode.name}/${taxonomy.windowsDirName}`;
        for (const w of policyReaddirSafe(repoRoot, windowsRel).filter((e) => e.isDirectory)) {
          const windowDir = `${windowsRel}/${w.name}`;
          if (taxonomy.windowLeafLangs.every((lang) => existsSync(join(repoRoot, windowDir, taxonomy.taxonomyLeafFilenames[lang] ?? "")))) hasCompleteWindow = true;
        }
      }
      if (!hasCompleteWindow) {
        breaches.push({
          id: `subset-surface-window-incomplete-${surfaceRel}`,
          summary: `"${surfaceRel}" has no mode with a window carrying both ${taxonomy.windowLeafLangs.join(", ")} leaves`,
          kind: "taxonomy/surface-completeness",
          scope: subRel,
          priority: "medium",
          reason: "Every surface must carry at least one mode with at least one window declaring every windowLeafLangs leaf.",
          solution: `Scaffold with "new surface … ${role}" (📇️registry/), then author a real mode/window.`,
        });
      }
      const scaffoldLeaves = policyWalkRelFiles(repoRoot, [surfaceRel], (_p, name) => scaffoldLeafNames.has(name)).filter((relPath) => policyReadFileSafe(repoRoot, relPath).includes("SCAFFOLD"));
      if (scaffoldLeaves.length > 0) {
        breaches.push({
          id: `subset-surface-scaffold-residue-${surfaceRel}`,
          summary: `"${surfaceRel}" still carries ${scaffoldLeaves.length} SCAFFOLD-marker leaf(ves)`,
          kind: "taxonomy/surface-scaffold-residue",
          scope: subRel,
          priority: "medium",
          reason: "A surface at scaffold content is not a finished W2 packet — the SCAFFOLD marker exists so this can never masquerade as done.",
          solution: `Author real render/view-model/commands for ${scaffoldLeaves.slice(0, 3).join(", ")}${scaffoldLeaves.length > 3 ? ", …" : ""}, removing the SCAFFOLD marker.`,
        });
      }
    }
  }
  return breaches;
}

/** 👁️ No file under a `👁️viewer` dir may reference mutation dispatch, `artifact_mutations`, or an
 * `::editor::` module path — `ViewEmit` is structurally read-only (contract §2.2); this is the
 * filesystem-content half of that guarantee. */
export function policyViewerPurityBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const patterns: readonly { re: RegExp; label: string }[] = [
    { re: /\.mutation\(/, label: ".mutation(" },
    { re: /Emit::mutations/, label: "Emit::mutations" },
    { re: /artifact_mutations/, label: "artifact_mutations" },
    { re: /::editor::/, label: "::editor::" },
  ];
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    const viewerRel = `${subRel}/${taxonomy.viewerDirName}`;
    if (!existsSync(join(repoRoot, viewerRel))) continue;
    for (const relPath of policyWalkRelFiles(repoRoot, [viewerRel], () => true)) {
      const content = policyReadFileSafe(repoRoot, relPath);
      for (const { re, label } of patterns) {
        if (!re.test(content)) continue;
        breaches.push({
          id: `viewer-purity-${relPath}-${label}`,
          summary: `"${relPath}" under a ${taxonomy.viewerDirName} dir matches forbidden pattern "${label}"`,
          kind: "taxonomy/viewer-purity",
          scope: subRel,
          priority: "high",
          reason: "ViewEmit is structurally read-only; no file under a viewer surface may reference mutation dispatch, artifact_mutations, or an editor-only module path.",
          solution: `Remove the "${label}" reference from ${relPath}, or move the mutating logic to the sibling ${taxonomy.editorDirName} surface.`,
        });
      }
    }
  }
  return breaches;
}

/** 🔗️A surface dir under a subset this plugin does NOT own (no `🧬️schema` of its own — a contributed,
 * mirrored surface per `contributedSubsetChildDirs`) must be backed by a `.depends_on(<owner-plugin>)`
 * on the contributing plugin's builder, exactly like `policyContributionTargetBreaches` requires for
 * mutation/inference contributions. Dormant on today's tree (no contributed surface exists yet — every
 * scaffolded surface sits under a subset its own plugin owns), verified structurally sound so it fires
 * correctly once W2 packets start contributing surfaces onto foreign kinds. */
export function policyContributedSurfaceTargetBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const subsetRels = policyListTopLevelSubsetDirs(repoRoot);
  const pluginRootOf = (subRel: string): string => subRel.split("/").slice(0, 3).join("/");
  const ownerRootBySuffix = new Map<string, string>();
  for (const subRel of subsetRels) {
    if (!existsSync(join(repoRoot, subRel, "🧬️schema"))) continue;
    const pluginRoot = pluginRootOf(subRel);
    const suffix = subRel.slice(pluginRoot.length + 1);
    if (!ownerRootBySuffix.has(suffix)) ownerRootBySuffix.set(suffix, pluginRoot);
  }
  for (const subRel of subsetRels) {
    if (existsSync(join(repoRoot, subRel, "🧬️schema"))) continue; // this plugin owns this triple — not a contribution
    const pluginRoot = pluginRootOf(subRel);
    const suffix = subRel.slice(pluginRoot.length + 1);
    const ownerRoot = ownerRootBySuffix.get(suffix);
    for (const role of taxonomy.surfaceRoles) {
      const surfaceRel = `${subRel}/${taxonomy.surfaceDirNames[role]}`;
      if (!existsSync(join(repoRoot, surfaceRel))) continue;
      if (!ownerRoot) {
        breaches.push({
          id: `contributed-surface-unknown-owner-${surfaceRel}`,
          summary: `"${surfaceRel}" contributes a surface but no other plugin owns "${suffix}"`,
          kind: "plugin-dependency/contributed-surface-target",
          scope: pluginRoot,
          priority: "medium",
          reason: "A contributed surface must mirror the path of a subset some OTHER plugin actually owns (has 🧬️schema for).",
          solution: `Verify "${suffix}" is a real owned subset elsewhere, or remove ${surfaceRel} if it is stray.`,
        });
        continue;
      }
      if (ownerRoot === pluginRoot) continue;
      const ownerPluginId = policyStripEmoji(ownerRoot.split("/").pop() ?? "");
      const declared = policyOwnerOwnComponentFiles(repoRoot, pluginRoot).some((relPath) =>
        new RegExp(`\\.depends_on\\s*\\(\\s*"${ownerPluginId}"`).test(policyReadFileSafe(repoRoot, relPath)),
      );
      if (declared) continue;
      breaches.push({
        id: `contributed-surface-target-undeclared-${surfaceRel}`,
        summary: `"${surfaceRel}" contributes a surface onto "${ownerPluginId}"'s subset without declaring .depends_on("${ownerPluginId}")`,
        kind: "plugin-dependency/contributed-surface-target",
        scope: pluginRoot,
        priority: "high",
        reason: "A surface contributed onto a subset this plugin does not own must be backed by a declared dependency on the owning plugin (surface.contribution-not-permitted).",
        solution: `Add .depends_on("${ownerPluginId}", …) to the plugin/extension builder in ${pluginRoot}.`,
      });
    }
  }
  return breaches;
}

const POLICY_OS_CONFIG_ROOT = "🧰️framework/🛍️products/💻️os/🎚️config";
const POLICY_OS_CONFIG_SCHEMA_ID = "os.config.opening";
const POLICY_OS_CONFIG_MUTATION_KINDS = ["set-default-app", "clear-default-app"] as const;

/** 🎚️Locks the shape of the already-shipped C4 OS config facet: five `schemaFormats` leaves, the
 * frozen `os.config.opening` schema id, and both `set-default-app`/`clear-default-app` mutation
 * triads. Content-keyed (schema id / `kind:` string), not folder-name-keyed, so it survives a slug
 * rename without drifting from the real frozen contract. */
export function policyOsConfigShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const schemaRel = `${POLICY_OS_CONFIG_ROOT}/🧬️schema`;
  if (!existsSync(join(repoRoot, schemaRel))) {
    breaches.push({
      id: `os-config-shape-missing-schema`,
      summary: `"${schemaRel}" is missing`,
      kind: "taxonomy/os-config-shape",
      scope: "os/config",
      priority: "high",
      reason: `The OS-level opening-preferences config facet (${POLICY_OS_CONFIG_SCHEMA_ID}) is a frozen C4 facet.`,
      solution: `Restore ${schemaRel}/ per contract §4.`,
    });
    return breaches;
  }
  for (const format of schemaFacetFormatEntries(repoRoot, schemaRel, taxonomy).map(([, f]) => f)) {
    if (existsSync(join(repoRoot, schemaRel, format.leafFilename))) continue;
    breaches.push({
      id: `os-config-shape-schema-leaf-${format.leafFilename}`,
      summary: `"${schemaRel}" is missing required schema leaf "${format.leafFilename}"`,
      kind: "taxonomy/os-config-shape",
      scope: "os/config",
      priority: "high",
      reason: `Every schemaFormats leaf for facet kind 🧬️data is frozen for ${POLICY_OS_CONFIG_SCHEMA_ID}.`,
      solution: `Add ${schemaRel}/${format.leafFilename}.`,
    });
  }
  const rootRust = policyReadFileSafe(repoRoot, schemaRel, "🦀️component.rs");
  if (!rootRust.includes(POLICY_OS_CONFIG_SCHEMA_ID)) {
    breaches.push({
      id: `os-config-shape-schema-id`,
      summary: `"${schemaRel}/🦀️component.rs" no longer declares the frozen schema id "${POLICY_OS_CONFIG_SCHEMA_ID}"`,
      kind: "taxonomy/os-config-shape",
      scope: "os/config",
      priority: "high",
      reason: "C4 froze the schema id os.config.opening; OpeningResolver and both hosts key off this exact string.",
      solution: `Restore the "${POLICY_OS_CONFIG_SCHEMA_ID}" schema id constant in ${schemaRel}/🦀️component.rs.`,
    });
  }
  const mutationsRel = `${schemaRel}/🧬️mutations`;
  const mutationChildDirs = taxonomy.mutationChildDirs ?? ["🦠️mutation", "🔺️diff", "↩️inverse"];
  const mutationDirs = policyReaddirSafe(repoRoot, mutationsRel).filter((e) => e.isDirectory);
  for (const mutationKind of POLICY_OS_CONFIG_MUTATION_KINDS) {
    const owner = mutationDirs.find((d) => policyReadFileSafe(repoRoot, mutationsRel, d.name, "🦠️mutation", "🦀️component.rs").includes(`kind: "${mutationKind}"`));
    if (!owner) {
      breaches.push({
        id: `os-config-shape-mutation-missing-${mutationKind}`,
        summary: `"${mutationsRel}" is missing the frozen "${mutationKind}" mutation triad`,
        kind: "taxonomy/os-config-shape",
        scope: "os/config",
        priority: "high",
        reason: `C4 froze both mutation kinds (set-default-app, clear-default-app) on ${POLICY_OS_CONFIG_SCHEMA_ID}.`,
        solution: `Restore ${mutationsRel}/<slug>/🦠️mutation/🦀️component.rs declaring kind: "${mutationKind}".`,
      });
      continue;
    }
    for (const child of mutationChildDirs) {
      const childRel = `${mutationsRel}/${owner.name}/${child}/🦀️component.rs`;
      if (existsSync(join(repoRoot, childRel))) continue;
      breaches.push({
        id: `os-config-shape-mutation-leaf-${childRel}`,
        summary: `"${childRel}" is missing`,
        kind: "taxonomy/os-config-shape",
        scope: "os/config",
        priority: "high",
        reason: `Every mutation dir carries the full ${mutationChildDirs.join(", ")} triad.`,
        solution: `Add ${childRel}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleArtifactViewersEditors


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

/** 🔎️Walks `relRoots` for files matching `pred`, skipping `POLICY_SKIP_DIRS` / dotted dirs and directories matching `skipDir`. */
function policyWalkRelFiles(
  repoRoot: string,
  relRoots: readonly string[],
  pred: (relPath: string, name: string) => boolean,
  skipDir?: (relDir: string, name: string) => boolean,
): string[] {
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
        if (skipDir && skipDir(childRel, ent.name)) continue;
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
    const raw = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
    if (policyReadFileSafe(repoRoot, rel).includes("register_language")) return true;
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
      const rsBody = policyReadFileSafe(repoRoot, rsRel);
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
  // 💾️binary / 📝️text are the facet's CODEC dirs (OpBinary/OpText), siblings of the mutation
  // triads rather than mutations themselves — they own no 🦠️mutation/🔺️diff/↩️inverse and never
  // should. They were harmless while the callers walked a shallow path that matched nothing on
  // disk; the moment the mutation rules were repointed at the real deep taxonomy they became 672
  // false "missing triad kind" highs. Reserved here, once, so every caller inherits the exclusion.
  const reserved = new Set<string>([...POLICY_MUTATION_TRIAD_DIRS, POLICY_MUTATION_PLAN_DIR, "📚️examples", "💾️binary", "📝️text"]);
  return policyReaddirSafe(repoRoot, mutationsRel)
    .filter((e) => e.isDirectory && !reserved.has(e.name) && !e.name.startsWith("."))
    .map((e) => e.name)
    .sort();
}

/** 🧩️Whether a mutation dir declares itself COMPOSITE by owning a `🧩️plan` child. */
function policyIsCompositeMutationDir(repoRoot: string, mutRel: string): boolean {
  return existsSync(join(repoRoot, `${mutRel}/${POLICY_MUTATION_PLAN_DIR}`));
}

/**
 * 📏️Every artifact must own `🧬️mutations/`; each concrete mutation dir must carry EITHER the leaf
 * triad `🦠️mutation` / `🔺️diff` / `↩️inverse` OR the composite pair `🦠️mutation` / `🧩️plan` — never a
 * mix (leaves optional until fan-out — directory presence is the gate). A composite folds its diff
 * and inverse from its plan, so owning `🔺️diff` or `↩️inverse` next to `🧩️plan` means two competing
 * sources of the same semantics.
 */
function policyMutationTriadCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
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
      const composite = policyIsCompositeMutationDir(repoRoot, mutRel);
      for (const kind of composite ? POLICY_MUTATION_COMPOSITE_DIRS : POLICY_MUTATION_TRIAD_DIRS) {
        const kindRel = `${mutRel}/${kind}`;
        if (existsSync(join(repoRoot, kindRel))) continue;
        breaches.push({
          id: `mutation-triad-missing-${kindRel}`,
          summary: composite ? `"${mutRel}" is a composite but is missing ${kind}/` : `"${mutRel}" is missing triad kind ${kind}/`,
          kind: "mutation-migration/triad-completeness",
          scope: artRel,
          priority: "high",
          reason: composite
            ? "A composite mutation must expose 🦠️mutation + 🧩️plan directories."
            : "Each concrete leaf mutation must expose 🦠️mutation + 🔺️diff + ↩️inverse directories.",
          solution: `Create ${kindRel}/ with 🦀️component.rs (and 🟦️component.ts stub if needed).`,
        });
      }
      if (!composite) continue;
      for (const derived of ["🔺️diff", "↩️inverse"] as const) {
        const derivedRel = `${mutRel}/${derived}`;
        if (!existsSync(join(repoRoot, derivedRel))) continue;
        breaches.push({
          id: `mutation-composite-derived-${derivedRel}`,
          summary: `"${mutRel}" owns both 🧩️plan and ${derived}/`,
          kind: "mutation-migration/triad-completeness",
          scope: artRel,
          priority: "high",
          reason: "A composite folds its diff and inverse from its plan — a handwritten one is a second, competing source of the same semantics.",
          solution: `Delete ${derivedRel}/ (composite) or delete ${mutRel}/${POLICY_MUTATION_PLAN_DIR}/ (leaf triad).`,
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
  const compositeImplPattern = /\bimpl\b[^\n{]*\bCompositeMutationKind\s*</;
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const mutRel = `${mutationsRel}/${mutName}`;
      const rsRel = `${mutRel}/🦠️mutation/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const abs = join(repoRoot, rsRel);
      if (!existsSync(abs)) continue;
      const content = readFileSync(abs, "utf8");
      const composite = policyIsCompositeMutationDir(repoRoot, mutRel);
      if (composite ? compositeImplPattern.test(content) : implPattern.test(content)) continue;
      breaches.push({
        id: `mutation-impl-missing-${rsRel}`,
        summary: composite ? `"${rsRel}" does not yet implement CompositeMutationKind<…>` : `"${rsRel}" does not yet implement Mutation<…>`,
        kind: "mutation-migration/impl-presence",
        scope: artRel,
        priority: "medium",
        reason: composite
          ? "A composite mutation struct must implement CompositeMutationKind<Snapshot, Op> so its diff and inverse fold from its plan."
          : "Each concrete mutation struct must implement Mutation<P> (or a helper the dispatch enum delegates to).",
        solution: composite ? `Add impl CompositeMutationKind<Snapshot, Op> for the mutation struct in ${rsRel}.` : `Add impl Mutation<Snapshot> for the mutation struct in ${rsRel}.`,
      });
    }
  }
  return breaches;
}

/** 🔗️Plugin roots that may declare runtime dependencies: `✏️s/🔌️plugins/<plugin>` and its `🧩️extensions/<ext>`. */
function policyDependencyOwnerRoots(repoRoot: string): readonly string[] {
  const roots: string[] = [];
  for (const plugin of policyReaddirSafe(repoRoot, "✏️s/🔌️plugins").filter((e) => e.isDirectory && !e.name.startsWith("."))) {
    const pluginRel = `✏️s/🔌️plugins/${plugin.name}`;
    roots.push(pluginRel);
    for (const ext of policyReaddirSafe(repoRoot, `${pluginRel}/🧩️extensions`).filter((e) => e.isDirectory && !e.name.startsWith("."))) {
      roots.push(`${pluginRel}/🧩️extensions/${ext.name}`);
    }
  }
  return roots;
}

/**
 * 🔗️Component leaves that belong to `ownerRel` ITSELF, excluding any nested `🧩️extensions/<ext>/`
 * subtree — an extension is its own dependency owner (it appears separately in
 * [[policyDependencyOwnerRoots]]), so folding its files into its parent plugin's walk made the parent
 * inherit the extension's `.depends_on(...)` and demand a Cargo dependency the parent never needs. A
 * plugin that hosts an extension declaring `.depends_on("cad")` is not itself a dependent of `cad`.
 */
function policyOwnerOwnComponentFiles(repoRoot: string, ownerRel: string): string[] {
  return policyWalkRelFiles(
    repoRoot,
    [ownerRel],
    (_relPath, name) => name === POLICY_RS_COMPONENT_LEAF_NAME,
    (_relDir, name) => name === "🧩️extensions",
  );
}

/** 🔗️Every `semio-s-plugin-<id>` entry in an owner's Cargo manifests, mapped to the plugin id it names. */
function policyCargoPluginDependencyIds(repoRoot: string, ownerRel: string): Set<string> {
  const ids = new Set<string>();
  const manifest = policyReadFileSafe(repoRoot, `${ownerRel}/📦️packages/🦀️rust/Cargo.toml`);
  for (const match of manifest.matchAll(/(?:^|\n)\s*(?:[\w-]+\s*=\s*\{[^}]*?)?package\s*=\s*"semio-s-plugin-([a-z0-9-]+)"/g)) {
    ids.add(match[1]!);
  }
  for (const match of manifest.matchAll(/(?:^|\n)\s*semio-s-plugin-([a-z0-9-]+)\s*=/g)) {
    ids.add(match[1]!);
  }
  return ids;
}

/**
 * 📏️A runtime plugin dependency (`.depends_on("x", …)` in the owner's `🦀️component.rs` tree) must be
 * backed by a real Cargo dependency on `semio-s-plugin-x`, and vice versa — a contributor needs the
 * dependency crate's snapshot/mutation types to plan against, and the host refuses to load a plugin
 * whose declared dependency is absent. Both directions are checked so neither half can drift.
 */
export function policyPluginDependencyParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const ownerRel of policyDependencyOwnerRoots(repoRoot)) {
    const declared = new Set<string>();
    for (const relPath of policyOwnerOwnComponentFiles(repoRoot, ownerRel)) {
      for (const match of policyReadFileSafe(repoRoot, relPath).matchAll(/\.depends_on\s*\(\s*"([a-z0-9-]+)"/g)) {
        declared.add(match[1]!);
      }
    }
    const cargo = policyCargoPluginDependencyIds(repoRoot, ownerRel);
    const ownId = policyStripEmoji(ownerRel.split("/").pop() ?? "");
    for (const id of declared) {
      if (cargo.has(id)) continue;
      breaches.push({
        id: `plugin-dependency-missing-cargo-${ownerRel}-${id}`,
        summary: `"${ownerRel}" declares .depends_on("${id}") with no Cargo dependency on semio-s-plugin-${id}`,
        kind: "plugin-dependency/parity",
        scope: ownerRel,
        priority: "high",
        reason: "A runtime dependency must be backed by the crate dependency that supplies the target's snapshot and mutation types.",
        solution: `Add semio-s-plugin-${id} (default-features = false) to ${ownerRel}/📦️packages/🦀️rust/Cargo.toml.`,
      });
    }
    for (const id of cargo) {
      if (id === ownId || declared.has(id)) continue;
      breaches.push({
        id: `plugin-dependency-undeclared-runtime-${ownerRel}-${id}`,
        summary: `"${ownerRel}" Cargo-depends on semio-s-plugin-${id} without declaring .depends_on("${id}")`,
        kind: "plugin-dependency/parity",
        scope: ownerRel,
        // 🎫️ Held at "medium" while the runtime-dependency API rolls out: every plugin that already
        // links a sibling crate (demonstrator, procedural, …) reports here until it adopts
        // `.depends_on`, which is the migration this ticket tracks rather than a defect to gate on.
        priority: "medium",
        reason: "The host builds its load order, version checks and contribution gates from the runtime manifest — a crate dependency the manifest never mentions is invisible to all three.",
        solution: `Add .depends_on("${id}", …) to the plugin/extension builder in ${ownerRel}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Every `ArtifactContribution::builder("s.<owner>.<artifact>")` must target an artifact whose
 * owning plugin the contributor declares as a direct dependency — contributing onto a plugin you do
 * not depend on is exactly the unowned-registration this ticket's contract forbids.
 */
export function policyContributionTargetBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const ownerRel of policyDependencyOwnerRoots(repoRoot)) {
    const declared = new Set<string>();
    const targets = new Map<string, string>();
    for (const relPath of policyOwnerOwnComponentFiles(repoRoot, ownerRel)) {
      const content = policyReadFileSafe(repoRoot, relPath);
      for (const match of content.matchAll(/\.depends_on\s*\(\s*"([a-z0-9-]+)"/g)) declared.add(match[1]!);
      for (const match of content.matchAll(/ArtifactContribution::builder\s*\(\s*"([^"]+)"/g)) targets.set(match[1]!, relPath);
    }
    for (const [kind, relPath] of targets) {
      const owner = kind.split(".")[1];
      if (!owner) {
        breaches.push({
          id: `contribution-target-ungrammatical-${relPath}-${kind}`,
          summary: `"${relPath}" contributes to "${kind}", which is not a canonical s.<plugin>.<artifact> kind`,
          kind: "plugin-dependency/contribution-target",
          scope: ownerRel,
          priority: "high",
          reason: "A contribution target must name its owning plugin so the dependency gate can resolve it.",
          solution: `Use the canonical kind id of the target artifact in ${relPath}.`,
        });
        continue;
      }
      if (declared.has(owner)) continue;
      breaches.push({
        id: `contribution-target-undeclared-${ownerRel}-${kind}`,
        summary: `"${ownerRel}" contributes to "${kind}" without declaring .depends_on("${owner}")`,
        kind: "plugin-dependency/contribution-target",
        scope: ownerRel,
        priority: "high",
        reason: "Mutations and inferences may only be registered onto artifacts of a directly declared dependency.",
        solution: `Add .depends_on("${owner}", …) to the plugin/extension builder in ${ownerRel}.`,
      });
    }
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
    const content = policyReadFileSafe(repoRoot, relPath);
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
 *
 * Held at `"medium"` (advisory) rather than `"high"`: this rule was silently inert until its scan
 * depth was fixed (it walked the nonexistent `<artifact>/🧬️mutations` shape), and activating it at
 * gate strength would immediately red four facets whose collisions are semantically coherent —
 * `🏗️fem/🧊️3d` and `🏗️fem/◻2d` (`🌱`create-*, `🔁`replace-*, `🗑`delete-*), `📐️cad` (7 prefixes) and
 * `🌊️flow` (`🔀️`reorder-*). Deliberately NOT seeded into a shrink-only allowlist: an allowlist is a
 * timestamp too, and this tree moves faster than a constant can track. Graduate to `"high"` once
 * those four dedup — the rule derives its population at run time, so no constant needs updating.
 */
function policyMutationEmojiUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    const seen = new Map<string, string>();
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const emoji = policyLeadingEmojiPrefix(mutName);
      if (!emoji) {
        breaches.push({
          id: `mutation-emoji-missing-${mutationsRel}/${mutName}`,
          summary: `"${mutationsRel}/${mutName}" has no leading emoji prefix`,
          kind: "mutation-migration/emoji-uniqueness",
          scope: artRel,
          priority: "medium",
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
          priority: "medium",
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
 * 🔍️Every `🧬️mutations` facet dir anywhere under `✏️s`. The real taxonomy nests it under
 * `<artifact>/🏅️standards/<slug>/🪆️subsets/<slug>/🧬️schema/🧬️mutations`, which is deeper than the
 * `<artifact>/🧬️mutations` shape `policyListPluginArtifactDirs` assumes — that shallower path matches
 * nothing on disk, so every rule driven by it was either firing bogus breaches or silently inert.
 * All mutation rules now share this walker.
 */
function policyFindAllMutationsDirs(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    for (const ent of policyReaddirSafe(repoRoot, relDir)) {
      if (!ent.isDirectory || ent.name.startsWith(".")) continue;
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.name === POLICY_MUTATIONS_FACET) {
        found.push(childRel);
        continue;
      }
      walk(childRel);
    }
  };
  walk("✏️s");
  return found.sort();
}

/**
 * 🗿️The owning artifact root for a `🧬️mutations` facet dir — everything above
 * `🏅️standards/…`, else the dir's own parent. Used as the `scope` on mutation breaches so they
 * report against `✏️s/🔌️plugins/<p>/🗿️artifacts/<a>` rather than the full nested facet path.
 */
function policyArtifactRootOfMutationsDir(mutationsRel: string): string {
  const marker = mutationsRel.indexOf("/🏅️standards/");
  if (marker > 0) return mutationsRel.slice(0, marker);
  const parts = mutationsRel.split("/");
  parts.pop();
  return parts.join("/");
}

/**
 * 📏️SEMANTIC-MUTATIONS-OVERHAUL rule 1 (`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST`): every `.rs` file under a
 * `🧬️mutations/` facet or a `🎮️commands/` app-command dir must not reference the banned generic mutation
 * vocabulary the new `SemanticDescriptor`/`MutationKind`/`#[derive(dsl_derive::Mutations)]` mechanism
 * (see `🧰️framework/…/📡️spr/🎮️command/🦀️component.rs`'s `🔖️Semantics` region) replaces: `SetSnapshot`
 * (the whole-document escape hatch), `NoMutation` (the sentinel sibling), and `CollectionMutation<`/`::`
 * (the generic collection wrapper, worst-offender count ~70 in one facet). `Set[A-Z]\w*` dispatch-enum
 * variant names are additionally flagged at `"medium"` (unallowlisted advisory — 562 of these exist
 * repo-wide today; only the three HIGH tokens above gate `bun policy`, so only those need seeding).
 * Seeded with the exact repo-wide census at ticket time (342 files) via
 * `grep -rlE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s --include="*.rs" | grep -E "🧬️mutations|🎮️commands"`;
 * later waves' handcrafted `rename`/`change`/`move`/… mutations shrink this file-by-file (see
 * `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`).
 */
const POLICY_SEMANTIC_VOCABULARY_ALLOWLIST = new Set<string>([
  "✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🎮️commands/✍️text/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🕸️node-graph/🦀️component.rs",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🗺️features/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🎮️commands/🖱️canvas/🦀️component.rs",
  "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/⌨️engagement/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/🀄️tile/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/👁️view/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/🖼️source/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/🎥️set-camera/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/📦️asset/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/📷️shot/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎮️commands/🗃️fixture/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥saved-cameras/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥saved-cameras/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦assets/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦assets/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸shots/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸shots/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫙no-mutation/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫙no-mutation/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫙no-mutation/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🎮️commands/📚️example/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🎮️commands/📚️example/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🏗️element/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/📤️exchange/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🔬️analysis/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🕸️graph/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/📄️artifact/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/📤️media/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🔎️inspector/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🛠️workshop/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🪜️step/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/🪵️stock/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋steps/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋steps/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️machines/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️machines/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🎮️commands/📄️fixture/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️objects-move/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️objects-move/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️objects-add/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️objects-add/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️objects-remove/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️objects-remove/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹objects-patch/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹objects-patch/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🧬️example/🦀️component.rs",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎮️commands/📥️io/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎮️commands/🗺️model-definition/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📓️iso16757/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📓️iso16757/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📕️din4108/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📗️din16798/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📗️din16798/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1990/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1991/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1991/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1993/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1993/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1994/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1994/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1995/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1995/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1996/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1996/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1997/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1997/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1998/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1998/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1999/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1999/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📙️din18599/🎮️commands/📤️set-snapshot/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🎛️apps/📙️din18599/🎮️commands/🧮️evaluate/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎮️commands/🔧️step/🦀️component.rs",
  "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎮️commands/🔧️nodes/🦀️component.rs",
  "✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎮️commands/🕸️graph/🦀️component.rs",
  "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎮️commands/📄️artifact/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️set-layer-transform/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-stroke/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-layer/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-layer/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-layer-opacity/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨set-fill/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-layer-name/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️set-layer-visible/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀set-boolean-operation/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-layer/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️set-layer-locked/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-trace-params/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧬️duplicate-layer/↩️inverse/🦀️component.rs",
  "✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎮️commands/🗂️document/🦀️component.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️workflow/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🎮️commands/🗃️fixture/🦀️component.rs",
  "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎮️commands/🎨️example/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎮️commands/📄️artifact/🦀️component.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎮️commands/🧺️curation/🦀️component.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦠️mutation/🦀️component.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
]);

/**
 * 🚫️High-priority banned generic-vocabulary tokens: the whole-document escape hatch, the sentinel, and the
 * generic collection wrapper — every real occurrence must either be handcrafted away or cited in
 * `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST`. Deliberately substring (no `\b` word boundary) matching the
 * seeding grep exactly, so derived type names like `SetSnapshotDiff`/`SetSnapshotMutation` still count —
 * they're just as much banned vocabulary as the bare identifier.
 */
const POLICY_SEMANTIC_VOCABULARY_HIGH_TOKENS: readonly { label: string; re: RegExp }[] = [
  { label: "SetSnapshot", re: /SetSnapshot/ },
  { label: "NoMutation", re: /NoMutation/ },
  { label: "CollectionMutation<...>", re: /CollectionMutation(<|::)/ },
];

/** 🏷️Every `.rs` file under a `🧬️mutations/` facet or a `🎮️commands/` app-command dir, repo-wide. */
function policyListSemanticVocabularyScanFiles(repoRoot: string): string[] {
  return policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => {
    if (!name.endsWith(".rs")) return false;
    const norm = relPath.replaceAll("\\", "/");
    return norm.includes(`/${POLICY_MUTATIONS_FACET}/`) || norm.includes("/🎮️commands/");
  });
}

function policySemanticVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyListSemanticVocabularyScanFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    const hitTokens = POLICY_SEMANTIC_VOCABULARY_HIGH_TOKENS.filter((t) => t.re.test(content)).map((t) => t.label);
    const allowlisted = POLICY_SEMANTIC_VOCABULARY_ALLOWLIST.has(relPath);
    if (hitTokens.length > 0) {
      if (!allowlisted) {
        breaches.push({
          id: `semantic-vocabulary-banned-${relPath}`,
          summary: `"${relPath}" references banned generic mutation vocabulary: ${hitTokens.join(", ")}`,
          kind: "mutation-migration/semantic-vocabulary",
          scope: relPath,
          priority: "high",
          reason: "SetSnapshot/NoMutation/CollectionMutation are the generic escape hatches the SemanticDescriptor/MutationKind/#[derive(dsl_derive::Mutations)] mechanism replaces with handcrafted rename/change/move/… mutations.",
          solution: `Replace ${hitTokens.join(", ")} in ${relPath} with handcrafted semantic MutationKind payload(s), or if this facet hasn't been reached yet, add "${relPath}" to POLICY_SEMANTIC_VOCABULARY_ALLOWLIST citing this ticket.`,
        });
      }
    } else if (allowlisted) {
      breaches.push({
        id: `semantic-vocabulary-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_SEMANTIC_VOCABULARY_ALLOWLIST but no longer references banned vocabulary`,
        kind: "mutation-migration/semantic-vocabulary",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${relPath}" from POLICY_SEMANTIC_VOCABULARY_ALLOWLIST.`,
      });
    }
    const bareSetVariants = [...new Set([...content.matchAll(/\bSet[A-Z]\w*/g)].map((m) => m[0]))].filter((name) => name !== "SetSnapshot");
    if (bareSetVariants.length > 0) {
      breaches.push({
        id: `semantic-vocabulary-bare-set-${relPath}`,
        summary: `"${relPath}" declares ${bareSetVariants.length} bare Set* identifier(s): ${bareSetVariants.slice(0, 6).join(", ")}${bareSetVariants.length > 6 ? ", …" : ""}`,
        kind: "mutation-migration/semantic-vocabulary",
        scope: relPath,
        priority: "medium",
        reason: "Set* is the generic verb this overhaul retires in favor of the closed APPROVED_VERBS vocabulary (rename/change/move/flatten/connect/group/…) — advisory (unallowlisted) until the fan-out wave rewrites dispatch enums.",
        solution: `Rename the Set* variant(s) in ${relPath} to a semantic verb-entity name backed by a MutationKind payload.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️SEMANTIC-MUTATIONS-OVERHAUL rule 2 (`policyMutationDispatchCoverageBreaches`, formerly a Wave-3
 * placeholder — this wave lands the real comparison): for every `🧬️mutations/🦀️component.rs` dispatch
 * file, extracts its `pub enum \w*Mutation\w* { … }` variant names and compares them against the
 * concrete triad-dir stems (`policyListMutationDirs`, kebab-case minus emoji, PascalCased) sitting
 * beside it. Kept at `"medium"` (advisory) rather than `"high"` because zero facets have adopted the
 * `#[derive(dsl_derive::Mutations)]` 1:1 variant-per-triad-dir shape yet (today's dispatch enums are
 * still the generic `CollectionMutation<…>` shape `policySemanticVocabularyBreaches` flags separately) —
 * this rule graduates to `"high"` once the fan-out wave lands real per-mutation triad wiring, mirroring
 * `policyMutationImplPresenceBreaches`'s own "advisory while Wave 3 pilot lands" graduation comment.
 */
function policyMutationEnumVariantNames(content: string): string[] {
  const enumMatch = content.match(/pub\s+enum\s+\w*Mutation\w*\s*\{/);
  if (!enumMatch || enumMatch.index === undefined) return [];
  let depth = 1;
  let i = enumMatch.index + enumMatch[0].length;
  for (; i < content.length && depth > 0; i++) {
    if (content[i] === "{") depth++;
    else if (content[i] === "}") depth--;
  }
  const body = content.slice(enumMatch.index + enumMatch[0].length, i - 1);
  const names = new Set<string>();
  for (const line of body.split("\n")) {
    const variantMatch = line.match(/^\s{4}([A-Z]\w*)/);
    if (variantMatch) names.add(variantMatch[1]!);
  }
  return [...names];
}

/** 🐫Kebab (minus emoji) → PascalCase, for comparing a triad-dir stem against a dispatch-enum variant name. */
function policyKebabToPascal(slug: string): string {
  return slug
    .split("-")
    .filter(Boolean)
    .map((seg) => seg.charAt(0).toUpperCase() + seg.slice(1))
    .join("");
}

function policyMutationDispatchCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const dispatchRel = `${mutationsRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (!existsSync(join(repoRoot, dispatchRel))) continue;
    const variantNames = new Set(policyMutationEnumVariantNames(policyReadFileSafe(repoRoot, dispatchRel)));
    if (variantNames.size === 0) continue;
    const triadPascalNames = new Set(policyListMutationDirs(repoRoot, mutationsRel).map((dir) => policyKebabToPascal(policyStripEmoji(dir))));
    const uncoveredVariants = [...variantNames].filter((v) => !triadPascalNames.has(v));
    const orphanTriads = [...triadPascalNames].filter((t) => !variantNames.has(t));
    if (uncoveredVariants.length === 0 && orphanTriads.length === 0) continue;
    breaches.push({
      id: `mutation-dispatch-coverage-${dispatchRel}`,
      summary: `"${dispatchRel}" dispatch variants and ${mutationsRel} triad dirs disagree: ${uncoveredVariants.length} variant(s) with no triad dir, ${orphanTriads.length} triad dir(s) with no variant`,
      kind: "mutation-migration/dispatch-coverage",
      scope: mutationsRel,
      priority: "medium",
      reason: "Once the fan-out wave adopts #[derive(dsl_derive::Mutations)], every dispatch variant must be a single-field tuple wrapping exactly one triad dir's MutationKind payload — advisory until then.",
      solution: `Reconcile ${dispatchRel}'s enum variants with the triad dirs under ${mutationsRel} (rename to match, or add/remove triad dirs).`,
    });
  }
  return breaches;
}

/**
 * 📏️SEMANTIC-MUTATIONS-OVERHAUL rule 3 (`policyMutationTsMirrorBreaches`): flags any `.ts` leaf anywhere
 * under a `🧬️mutations/` facet whose content is trivially `export {};` (or empty) — near-universal today
 * (triad `.ts` leaves were scaffolded as stubs, see `policyIsConstitutionalTsFacadePath`'s explicit tolerance for this exact shape),
 * so this stays `"low"` advisory with no allowlist rather than seeding ~1000+ file paths for a finding that
 * blocks nothing; it exists so a real TS mirror rollout has a tracked signal to burn down file-by-file.
 */
function policyMutationTsMirrorBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const tsFiles = policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => name === POLICY_TS_COMPONENT_LEAF && relPath.replaceAll("\\", "/").includes(`/${POLICY_MUTATIONS_FACET}/`));
  for (const relPath of tsFiles) {
    const stripped = policyReadFileSafe(repoRoot, relPath)
      .replace(/\/\*[\s\S]*?\*\//g, "")
      .replace(/\/\/.*$/gm, "")
      .trim();
    if (stripped !== "" && stripped !== "export {};") continue;
    breaches.push({
      id: `mutation-ts-mirror-stub-${relPath}`,
      summary: `"${relPath}" is a trivial "export {};" stub with no real TS mirror`,
      kind: "mutation-migration/ts-mirror",
      scope: relPath,
      priority: "low",
      reason: "A triad leaf's TS mirror should eventually re-export the same MutationKind payload shape as its Rust sibling, not stay an empty stub.",
      solution: `Give ${relPath} real content mirroring its 🦀️component.rs sibling once the DSL TS codegen for this triad lands.`,
    });
  }
  for (const rsRel of policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => name === POLICY_RS_COMPONENT_LEAF_NAME && relPath.replaceAll("\\", "/").includes(`/${POLICY_MUTATIONS_FACET}/`))) {
    const tsRel = `${rsRel.slice(0, rsRel.lastIndexOf("/"))}/${POLICY_TS_COMPONENT_LEAF}`;
    if (existsSync(join(repoRoot, tsRel))) continue;
    breaches.push({
      id: `mutation-ts-mirror-absent-${tsRel}`,
      summary: `"${rsRel}" has no 🟦️component.ts mirror beside it at all`,
      kind: "mutation-migration/ts-mirror",
      scope: tsRel,
      priority: "low",
      reason: "An ABSENT mirror is invisible to the stub scan above, so a facet that never scaffolded its .ts leaves looked cleaner than one that did — this half closes that blind spot.",
      solution: `Create ${tsRel} mirroring its 🦀️component.rs sibling's MutationKind payload shape.`,
    });
  }
  return breaches;
}

/**
 * 🎮️App command folders must be one semantically named command (verb-noun kebab), matching
 * `🧬️mutations/<emoji><verb>-<noun>/`. Noun buckets (`✍️text`, `🗂️selection`, `🗣️locale`) are
 * architecture violations. Held at `"medium"` while ticket `26/08/13/SEMANTIC-COMMAND-NAMES`
 * fans out; graduate to `"high"` once every plugin leaf is 1:1.
 */
const POLICY_COMMAND_VERBS = new Set<string>([
  "create", "delete", "insert", "remove", "add", "rename", "change", "update", "move",
  "drag", "resize", "rotate", "scale", "reorder", "edit", "replace", "duplicate",
  "connect", "disconnect", "bind", "unbind", "group", "ungroup", "flatten", "unflatten",
  "split", "merge", "extract", "inline", "clear", "fix", "toggle", "apply", "set",
  "format", "open", "load", "save", "import", "export", "lint", "request", "select",
  "hover", "evaluate", "run", "retry", "calibrate", "place", "commit", "input",
  "submit", "abort", "patch", "paint", "nudge", "reset", "focus", "step", "query",
  "fill", "snap", "ingest", "evaluate",
]);

function policyCommandFolderSemanticBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const walk = (relDir: string): void => {
    for (const ent of policyReaddirSafe(repoRoot, relDir)) {
      if (!ent.isDirectory || ent.name.startsWith(".")) continue;
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.name === "🎮️commands") {
        for (const cmd of policyReaddirSafe(repoRoot, childRel)) {
          if (!cmd.isDirectory) continue;
          const folder = cmd.name;
          const emoji = policyLeadingEmojiPrefix(folder);
          const slug = emoji ? folder.slice(emoji.length) : folder;
          const verb = slug.split("-")[0] ?? slug;
          if (POLICY_COMMAND_VERBS.has(verb)) continue;
          breaches.push({
            id: `command-folder-semantic-${childRel}/${folder}`,
            summary: `"${childRel}/${folder}" is a noun bucket, not a semantic command`,
            kind: "taxonomy/command-folder-semantic",
            scope: childRel,
            priority: "medium",
            reason: "Each 🎮️commands/ leaf must be one command named like a mutation (verb-noun kebab, e.g. ✍️text-edit). Grouping under domain nouns (text, selection, locale, camera) is banned.",
            solution: `Split ${childRel}/${folder} into one folder per command using an approved verb prefix (${[...POLICY_COMMAND_VERBS].slice(0, 8).join(", ")}, …).`,
          });
        }
        continue;
      }
      if (ent.name === "target" || ent.name === "node_modules") continue;
      walk(childRel);
    }
  };
  walk("✏️s");
  return breaches;
}

/** ⚖️Aggregates Wave 2b mutation / ArtifactEngine / op-grammar scanners. */
function policyMutationArtifactEngineBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyMutationTriadCompletenessBreaches(repoRoot),
    ...policyMutationImplPresenceBreaches(repoRoot),
    ...policyOpGrammarStartMutationBreaches(repoRoot),
    ...policyMutationEmojiUniquenessBreaches(repoRoot),
    ...policyMutationDispatchCoverageBreaches(repoRoot),
    ...policySemanticVocabularyBreaches(repoRoot),
    ...policyMutationTsMirrorBreaches(repoRoot),
    ...policyCommandFolderSemanticBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleMutationArtifactEngines

//#region 🔧️PolicyRuleMutationOutcomeMergePolicy
/**
 * 🎫️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` C2/C4/C10 gates: every
 * `🔺️diff` leaf must report through `protocol::MutationOutcome<D>` using only the 7 frozen codes
 * (`📋️contract-freeze.md` §C2), `validate` is deleted everywhere, `Severity::Hint` is gone, the CRDT
 * merge-strategy/conflict-rule vocabulary reaches zero, `MergePolicy`'s 3 variants mirror across all 4
 * surfaces, and the dsl derive macro's two build-shape entry points stay byte-identical.
 */
const POLICY_MUTATION_FROZEN_CODES = ["mutation.target-missing", "mutation.no-op", "mutation.partial", "mutation.clamped", "mutation.duplicate-id", "mutation.invariant", "mutation.cascade"] as const;
const POLICY_MUTATION_FROZEN_CODE_SET = new Set<string>(POLICY_MUTATION_FROZEN_CODES);

/** 🗄️The gltf legacy typed-sparse-operation architecture's owning root — see `POLICY_MUTATION_TOTAL_KIND_ALLOWLIST` entry (b). Exempts this tree from both `policyMutationOutcomeBreaches` (rule 1) and `policyMutationMessageCodeBreaches` (rule 2) — gltf's `GltfTopLevelMutationRejection` codes (e.g. `"mutation.rejected"`) are that separate architecture's own vocabulary, never the 7 frozen `MutationOutcome` codes. */
const POLICY_MUTATION_GLTF_ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf";
/** 🌐️Sentinel toggling allowlist entry (a) — see `POLICY_MUTATION_TOTAL_KIND_ALLOWLIST`'s doc comment. */
const POLICY_MUTATION_TOTAL_KIND_SENTINEL = "🌐️root-scoped-total-kind";

/**
 * 🎫️ `📋️contract-freeze.md` fan-out recipe: "Total kinds (root-scoped `clear-*`, root
 * `change-<artifact>-<field>`) may return message-free outcomes via a shrink-only allowlist." Seeded
 * with exactly the two documented entries this ticket froze:
 * (a) `POLICY_MUTATION_TOTAL_KIND_SENTINEL` — a structural toggle (not a literal path) for root-scoped
 *     total kinds: any mutation slug beginning `clear-` (clearing an artifact-level collection has no
 *     id to address, so it's inherently root-scoped) or `change-<artifactId>-` (the artifact's own root
 *     field, which always exists) may call bare `protocol::MutationOutcome::new(diff)` with no message —
 *     `policyMutationIsRootScopedTotalKind` below is the exact predicate. Remove this entry once total
 *     kinds are required to carry `mutation.no-op`/`mutation.cascade` like every other verb family.
 * (b) `POLICY_MUTATION_GLTF_ROOT` — 116 leaves under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/**` are a
 *     SEPARATE typed-sparse-operation architecture (`derive`/`apply_diff`/`GltfTopLevelMutationRejection`,
 *     `Result<T, GltfTopLevelMutationRejection>` — never `protocol::MutationOutcome<..>`, never on the
 *     `Mutation::diff` path) owned by the live ticket
 *     `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`. This same entry also
 *     exempts the tree from `policyMutationMessageCodeBreaches` (rule 2) — its `Mutation::diff` shim at
 *     `🔨️modules/🧭️mutation-dispatch/🦀️component.rs` legitimately builds a `protocol::MutationOutcome`
 *     to satisfy that trait, but reports the SEPARATE architecture's own rejection reason (a
 *     `GltfTopLevelMutationRejection`'s `Display`) as the message, which is never one of the 7 frozen
 *     codes by design. Remove this entry once that ticket either folds gltf onto `MutationOutcome` or
 *     the split architecture is formally frozen elsewhere.
 */
const POLICY_MUTATION_TOTAL_KIND_ALLOWLIST = new Set<string>([POLICY_MUTATION_TOTAL_KIND_SENTINEL, POLICY_MUTATION_GLTF_ROOT]);

/** 🔎️Root-scoped total kind per the fan-out recipe: bare `clear-*`, or `change-<artifactId>-*` addressing the artifact's own always-present root field. */
function policyMutationIsRootScopedTotalKind(mutName: string, artifactId: string): boolean {
  const stripped = policyStripEmoji(mutName);
  if (stripped.startsWith("clear-")) return true;
  return artifactId !== "" && stripped.startsWith(`change-${artifactId}-`);
}

/**
 * 📏️Rule 1: every `🧬️mutations/<slug>/🔺️diff/🦀️component.rs` must return `protocol::MutationOutcome<`
 * and reference at least one of the 7 frozen codes, unless `POLICY_MUTATION_TOTAL_KIND_ALLOWLIST` exempts
 * it. Composite mutation dirs (own `🧩️plan`, not `🔺️diff`) are out of scope — their outcome folds from
 * the plan. A leaf whose `🔺️diff` doesn't exist yet is tracked by `policyMutationTriadCompletenessBreaches`
 * instead, not here.
 */
export function policyMutationOutcomeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const gltfAllowlisted = POLICY_MUTATION_TOTAL_KIND_ALLOWLIST.has(POLICY_MUTATION_GLTF_ROOT);
  const totalKindAllowlisted = POLICY_MUTATION_TOTAL_KIND_ALLOWLIST.has(POLICY_MUTATION_TOTAL_KIND_SENTINEL);
  for (const mutationsRel of policyFindAllMutationsDirs(repoRoot)) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    const artifactId = policyStripEmoji(artRel.split("/").pop() ?? "");
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const mutRel = `${mutationsRel}/${mutName}`;
      if (policyIsCompositeMutationDir(repoRoot, mutRel)) continue;
      const diffRel = `${mutRel}/🔺️diff/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (!existsSync(join(repoRoot, diffRel))) continue;
      if (gltfAllowlisted && diffRel.startsWith(`${POLICY_MUTATION_GLTF_ROOT}/`)) continue;
      const content = policyReadFileSafe(repoRoot, diffRel);
      const returnsOutcome = /protocol::MutationOutcome\s*</.test(content);
      if (!returnsOutcome) {
        breaches.push({
          id: `mutation-outcome-missing-type-${diffRel}`,
          summary: `"${diffRel}" does not return protocol::MutationOutcome<..>`,
          kind: "mutation-migration/outcome",
          scope: artRel,
          priority: "high",
          reason: "C2/C4: every 🔺️diff leaf must report through the frozen MutationOutcome<D> contract.",
          solution: `Change diff's signature to -> protocol::MutationOutcome<XDiff> and wrap the success path in MutationOutcome::new(..) (see 📋️contract-freeze.md's fan-out recipe, or the already-converted 🕸️dag/📐️cad/💠️lowpoly facets).`,
        });
        continue;
      }
      const isTotalKind = totalKindAllowlisted && policyMutationIsRootScopedTotalKind(mutName, artifactId);
      if (isTotalKind) continue;
      const hasCode = POLICY_MUTATION_FROZEN_CODES.some((code) => content.includes(code));
      if (hasCode) continue;
      breaches.push({
        id: `mutation-outcome-missing-code-${diffRel}`,
        summary: `"${diffRel}" returns protocol::MutationOutcome<..> but never references one of the 7 frozen message codes`,
        kind: "mutation-migration/outcome",
        scope: artRel,
        priority: "high",
        reason: "The verb-family table (📋️contract-freeze.md fan-out recipe) requires real Error/Warning/Fatal/Info detection per verb family — a bare MutationOutcome::new(..) with no message is only legal for a root-scoped total kind.",
        solution: `Add the real detection this verb family requires (target missing ⇒ ::error("mutation.target-missing", ..), idempotent ⇒ .warn("mutation.no-op", ..), duplicate id / invariant ⇒ ::fatal(..), cascade ⇒ .info("mutation.cascade", ..)), or if this genuinely is a root-scoped total kind, confirm POLICY_MUTATION_TOTAL_KIND_ALLOWLIST covers it.`,
      });
    }
  }
  return breaches;
}

/** 🔎️Matches definite `MutationOutcome::(error|fatal)(...)`/`MutationMessage::(info|warn|error|fatal)(...)` builders — always genuine regardless of surrounding context, capturing the first string-literal argument. */
const POLICY_MUTATION_MESSAGE_CODE_BUILDER_RE = /\b(?:MutationOutcome::(?:error|fatal)|MutationMessage::(?:info|warn|error|fatal))\s*\(\s*"([^"]*)"/g;
/** 🔎️Matches the chainable `.info(..)`/`.warn(..)` shorthand — only checked inside a fn body already proven to build a `MutationOutcome` (see `policyMutationMessageCodeBreaches`), never file-wide, so an unrelated `console.warn(..)`/`log::warn!`/`tracing::warn!` can never match. */
const POLICY_MUTATION_MESSAGE_CODE_CHAIN_RE = /\.(?:info|warn)\s*\(\s*"([^"]*)"/g;
/** 🔎️Finds `fn <name>(..` openers so rule 2 can scope the chainable-call check one function body at a time (paired with `policyExtractFnBody`). */
const POLICY_FN_DECL_RE = /\bfn\s+[A-Za-z_]\w*/g;

/**
 * 📏️Rule 2: any message-constructing call's first argument must be exactly one of the 7 frozen codes
 * (📋️contract-freeze.md §C2) — no per-plugin codes, ever. Scoped to `.rs` files (the codes are a Rust
 * vocabulary; this also keeps 📜️script.ts itself, which is a `.ts` file, out of its own scan). Definite
 * builders are checked file-wide; the chainable `.info(..)`/`.warn(..)` shorthand is checked only
 * inside a `fn` body that itself references `MutationOutcome` — a plain `console.warn(..)` embedded as
 * a JS string literal inside a Rust file (never inside such a body) can never false-positive here.
 * Exempts `POLICY_MUTATION_GLTF_ROOT`, same allowlist entry (b) as `policyMutationOutcomeBreaches` —
 * gltf's separate typed-sparse-operation architecture owns its own rejection vocabulary.
 */
export function policyMutationMessageCodeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const gltfAllowlisted = POLICY_MUTATION_TOTAL_KIND_ALLOWLIST.has(POLICY_MUTATION_GLTF_ROOT);
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (gltfAllowlisted && relPath.startsWith(`${POLICY_MUTATION_GLTF_ROOT}/`)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    const reportedLines = new Set<number>();
    const pushIfBad = (code: string, index: number) => {
      if (POLICY_MUTATION_FROZEN_CODE_SET.has(code)) return;
      const line = policyLineOfIndex(content, index);
      if (reportedLines.has(line)) return;
      reportedLines.add(line);
      breaches.push({
        id: `mutation-message-code-${relPath}-${line}`,
        summary: `"${relPath}:${line}" uses message code "${code}" — not one of the 7 frozen codes`,
        kind: "mutation-migration/message-code",
        scope: relPath,
        line,
        priority: "high",
        reason: "C2's frozen code set is exactly 7 generic codes — there are no per-plugin codes. An eighth code must be reported to the coordinator, never invented.",
        solution: `Map this message onto mutation.target-missing/no-op/partial/clamped/duplicate-id/invariant/cascade, or report the gap to the coordinator if none fits.`,
      });
    };

    POLICY_MUTATION_MESSAGE_CODE_BUILDER_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_MUTATION_MESSAGE_CODE_BUILDER_RE.exec(content))) pushIfBad(m[1]!, m.index);

    if (!content.includes("MutationOutcome")) continue;
    POLICY_FN_DECL_RE.lastIndex = 0;
    let fm: RegExpExecArray | null;
    while ((fm = POLICY_FN_DECL_RE.exec(content))) {
      const body = policyExtractFnBody(content, fm.index);
      if (!body || !body.includes("MutationOutcome")) continue;
      const bodyStart = content.indexOf(body, fm.index);
      POLICY_MUTATION_MESSAGE_CODE_CHAIN_RE.lastIndex = 0;
      let cm: RegExpExecArray | null;
      while ((cm = POLICY_MUTATION_MESSAGE_CODE_CHAIN_RE.exec(body))) pushIfBad(cm[1]!, bodyStart + cm.index);
    }
  }
  return breaches;
}

/** 🔎️CRDT-era vocabulary C10 deletes: `merge_strategy`, `MergeStrategyKind`, `merge_concurrent_diffs`, `ConflictRule`, `ResolutionPlan`, `assert_crdt_*`. */
const POLICY_CRDT_VOCABULARY_TOKENS = ["merge_strategy", "MergeStrategyKind", "merge_concurrent_diffs", "ConflictRule", "ResolutionPlan", "assert_crdt_"] as const;
const POLICY_CRDT_VOCABULARY_RE = new RegExp(`\\b(${POLICY_CRDT_VOCABULARY_TOKENS.join("|")})`, "g");

/**
 * 📏️Rule 3: zero repo-wide occurrences of the CRDT merge-strategy/conflict-rule vocabulary C10 deletes
 * (`.🧬semio/`, `node_modules`, `target`, `dist` excluded via `POLICY_SKIP_DIRS`). Scoped to `.rs` files —
 * every token is a Rust-only identifier (no TS mirror was ever specified for the deleted CRDT pair).
 */
export function policyNoCrdtVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    POLICY_CRDT_VOCABULARY_RE.lastIndex = 0;
    const seenLines = new Set<number>();
    let m: RegExpExecArray | null;
    while ((m = POLICY_CRDT_VOCABULARY_RE.exec(content))) {
      const line = policyLineOfIndex(content, m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-crdt-vocabulary-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references CRDT vocabulary "${m[1]}"`,
        kind: "mutation-migration/no-crdt-vocabulary",
        scope: relPath,
        line,
        priority: "high",
        reason: "C10 deletes 📡️spr/🔀️crdt/** and its whole vocabulary in favor of C3's MergePolicy + C5's first-class Conflict — merge_strategy/MergeStrategyKind/merge_concurrent_diffs/ConflictRule/ResolutionPlan/assert_crdt_* must reach zero occurrences repo-wide.",
        solution: `Delete or rewrite this reference — reach for protocol::MergePolicy / the new 📡️spr/⚔️conflict module instead (📋️contract-freeze.md §C3/C5/C10).`,
      });
    }
  }
  return breaches;
}

/** 🔎️A `fn validate(&self, ..)` override — the exact shape C4/C10 delete from `Mutation`/`MutationKind`/`CompositeMutationKind`. */
const POLICY_VALIDATE_OVERRIDE_RE = /\bfn\s+validate\s*\(\s*&self\b/g;
/** 🔎️An `impl (protocol::)?(Composite)?MutationKind<..> for X {` block opener, to scope the second half of rule 4. */
const POLICY_MUTATION_KIND_IMPL_RE = /\bimpl\b[^\n{]*\b(?:CompositeMutationKind|MutationKind)\s*</g;

/**
 * 📏️Rule 4: no `fn validate(&self, ..)` survives — not inside a `🧬️mutations/**` file, and not inside
 * any `impl … MutationKind`/`impl … CompositeMutationKind` block anywhere (hand-written config/presence
 * enums included). Its checks move into the `🔺️diff` leaf as Error/Fatal messages (C4/C10). Note: gltf's
 * `pub fn validate(payload: &P, base: &S) -> Result<(), GltfTopLevelMutationRejection>` is a FREE function
 * with no `&self` receiver — a structurally different shape that this rule never matches.
 */
export function policyNoValidateOverrideBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    const reportedLines = new Set<number>();
    const pushBreach = (line: number, detail: string) => {
      if (reportedLines.has(line)) return;
      reportedLines.add(line);
      breaches.push({
        id: `no-validate-override-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still overrides fn validate(&self, ..) ${detail}`,
        kind: "mutation-migration/no-validate-override",
        scope: relPath,
        line,
        priority: "high",
        reason: "C4/C10: validate no longer exists on Mutation/MutationKind/CompositeMutationKind — every override is deleted, its checks moved into the 🔺️diff leaf as Error/Fatal messages.",
        solution: `Delete this fn validate override; move its check into the sibling 🔺️diff leaf as a protocol::MutationOutcome::error(..)/::fatal(..) message.`,
      });
    };

    const underMutations = relPath.replaceAll("\\", "/").includes(`/${POLICY_MUTATIONS_FACET}/`);
    if (underMutations) {
      POLICY_VALIDATE_OVERRIDE_RE.lastIndex = 0;
      let vm: RegExpExecArray | null;
      while ((vm = POLICY_VALIDATE_OVERRIDE_RE.exec(content))) pushBreach(policyLineOfIndex(content, vm.index), `under a 🧬️mutations/ leaf`);
    }

    POLICY_MUTATION_KIND_IMPL_RE.lastIndex = 0;
    let im: RegExpExecArray | null;
    while ((im = POLICY_MUTATION_KIND_IMPL_RE.exec(content))) {
      const body = policyExtractFnBody(content, im.index);
      POLICY_VALIDATE_OVERRIDE_RE.lastIndex = 0;
      const vm = POLICY_VALIDATE_OVERRIDE_RE.exec(body);
      if (!vm) continue;
      pushBreach(policyLineOfIndex(content, im.index + vm.index), `inside an impl …MutationKind block`);
    }
  }
  return breaches;
}

/** 🔎️Repo-relative paths that resolve to 📜️script.ts's own content — the root `script.ts` symlink (compat alias for tooling that can't glob the emoji filename) reads through to the identical bytes, so both names must be excluded from a self-scan. */
const POLICY_MUTATION_LAW_SELF_PATHS = new Set<string>(["📜️script.ts", "script.ts"]);

/** 🔎️Source files (`.rs`/`.ts`/`.tsx`) repo-wide, excluding 📜️script.ts (and its `script.ts` symlink alias) — this policy region's own code necessarily names the banned tokens as string/regex literals, so it must not scan itself. */
function policyMutationLawSourceFiles(repoRoot: string): string[] {
  return policyWalkRelFiles(repoRoot, [""], (relPath, name) => (name.endsWith(".rs") || name.endsWith(".ts") || name.endsWith(".tsx")) && !POLICY_MUTATION_LAW_SELF_PATHS.has(relPath));
}

const POLICY_SEVERITY_HINT_STRUCT_RE = /\bSeverity(?:::|\.)Hint\b/g;

/**
 * 📏️Rule 5: zero `Severity::Hint`/`Severity.Hint` (C1 declaration order is now `Info, Warning, Error,
 * Fatal` — `derive(Ord)` IS the level order, Hint was folded into Info repo-wide), and zero bare `"hint"`
 * severity-literal on a line that also mentions "severity"/"level" (scoped to avoid flagging the ordinary
 * English word "hint" used for unrelated UI copy/tooltips).
 */
export function policySeverityInfoBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyMutationLawSourceFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    const reportedLines = new Set<number>();
    POLICY_SEVERITY_HINT_STRUCT_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_SEVERITY_HINT_STRUCT_RE.exec(content))) {
      const line = policyLineOfIndex(content, m.index);
      if (reportedLines.has(line)) continue;
      reportedLines.add(line);
      breaches.push({
        id: `severity-hint-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references Severity::Hint/Severity.Hint`,
        kind: "mutation-migration/no-severity-hint",
        scope: relPath,
        line,
        priority: "high",
        reason: "C1 collapsed Hint into Info repo-wide; Severity's declaration order (Info < Warning < Error < Fatal, 0..3) has no Hint member anymore.",
        solution: `Rewrite this to Severity::Info / "Info" — Hint was merged into Info by C1.`,
      });
    }
    const lines = content.split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i]!;
      if (!/["']hint["']/i.test(line) || !/severity|level/i.test(line)) continue;
      const lineNo = i + 1;
      if (reportedLines.has(lineNo)) continue;
      reportedLines.add(lineNo);
      breaches.push({
        id: `severity-hint-literal-${relPath}-${lineNo}`,
        summary: `"${relPath}:${lineNo}" uses a "hint" severity literal`,
        kind: "mutation-migration/no-severity-hint",
        scope: relPath,
        line: lineNo,
        priority: "high",
        reason: "C1 collapsed Hint into Info repo-wide; a serialized/matched severity level literal must never be \"hint\".",
        solution: `Rewrite the literal to "info" — Hint was merged into Info by C1.`,
      });
    }
  }
  return breaches;
}

/** 🔎️The 3 frozen MergePolicy variants (C3) mirrored at exactly these 4 surfaces. */
const POLICY_MERGE_POLICY_VARIANTS = ["LaissezFaire", "Normal", "Vigilant"] as const;
/** 🔡️Idiomatic per-language spelling accepted for each variant — Rust surfaces spell it exactly like the enum (`LaissezFaire`); TS surfaces (host codec, kernel types, i18n) may use either the PascalCase mirror or the camelCase object-key form (`laissezFaire`) that idiomatic TS reaches for. Presence-only, per surface's own convention — this is a mirror-existence check, not a shape check. */
const POLICY_MERGE_POLICY_VARIANT_SPELLINGS: Readonly<Record<(typeof POLICY_MERGE_POLICY_VARIANTS)[number], readonly string[]>> = {
  LaissezFaire: ["LaissezFaire", "laissezFaire"],
  Normal: ["Normal", "normal"],
  Vigilant: ["Vigilant", "vigilant"],
};
const POLICY_MERGE_POLICY_SURFACES: readonly { label: string; relPath: string }[] = [
  { label: "Rust spine (protocol::MergePolicy, 📡️spr/🧾️wire)", relPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs" },
  { label: "TS host codec (💻️os/🟦️component.ts)", relPath: "🧰️framework/🛍️products/💻️os/🟦️component.ts" },
  { label: "TS kernel types (🎠️kernel/🟦️component.ts)", relPath: "🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts" },
  { label: "i18n bundles (de+en, 🖱️ui react index.tsx)", relPath: "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx" },
];

/**
 * 📏️Rule 6: the three `MergePolicy` variants must be present in all 4 frozen surfaces from C3/C9,
 * each under whatever idiomatic spelling that surface's language uses (`POLICY_MERGE_POLICY_VARIANT_
 * SPELLINGS` — as substrings, this is a mirror-existence check, not a shape check); fewer than all
 * four ⇒ one breach per surface that's missing at least one variant under any of its spellings.
 */
export function policyMergePolicyParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const surface of POLICY_MERGE_POLICY_SURFACES) {
    const content = policyReadFileSafe(repoRoot, surface.relPath);
    const missing = POLICY_MERGE_POLICY_VARIANTS.filter((v) => !POLICY_MERGE_POLICY_VARIANT_SPELLINGS[v].some((spelling) => content.includes(spelling)));
    if (missing.length === 0) continue;
    breaches.push({
      id: `merge-policy-parity-${surface.relPath}`,
      summary: `"${surface.relPath}" (${surface.label}) is missing MergePolicy variant(s): ${missing.join(", ")}`,
      kind: "mutation-migration/merge-policy-parity",
      scope: surface.relPath,
      priority: "high",
      reason: "C3/C9: MergePolicy {LaissezFaire, Normal, Vigilant} must mirror across all 4 surfaces (Rust spine, TS host codec, TS kernel types, both i18n bundles) or the merge-policy setting silently desyncs across a surface.",
      solution: `Add the missing MergePolicy variant name(s) to ${surface.relPath} (see 📋️contract-freeze.md §C3/C9 for the exact shape expected at this surface).`,
    });
  }
  return breaches;
}

const POLICY_DERIVE_MIRROR_A = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs";
const POLICY_DERIVE_MIRROR_B = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs";

/** 📏️Rule 7: the dsl derive macro's two build-shape entry points must stay byte-identical — a drift means one shape silently runs stale derive logic. */
export function policyDeriveMirrorBreaches(repoRoot: string): BreachRecord[] {
  const a = policyReadFileSafe(repoRoot, POLICY_DERIVE_MIRROR_A);
  const b = policyReadFileSafe(repoRoot, POLICY_DERIVE_MIRROR_B);
  if (a === b) return [];
  return [
    {
      id: `derive-mirror-drift-${POLICY_DERIVE_MIRROR_A}`,
      summary: `"${POLICY_DERIVE_MIRROR_A}" and "${POLICY_DERIVE_MIRROR_B}" have drifted — they must stay byte-identical`,
      kind: "mutation-migration/derive-mirror",
      scope: POLICY_DERIVE_MIRROR_A,
      priority: "high",
      reason: "The dsl derive macro's component.rs and its glue.rs copy under 📦️packages/🦀️rust are two build-shape entry points for the exact same macro body — any drift means one shape silently runs stale derive logic.",
      solution: `Copy whichever of the two files has the real edit over the other so they stay byte-identical (${POLICY_DERIVE_MIRROR_A} ⇔ ${POLICY_DERIVE_MIRROR_B}).`,
    },
  ];
}

/** ⚖️Aggregates this ticket's 7 mutation-outcome / merge-policy / no-CRDT / no-validate / derive-mirror gates — the bundle both `policy` (below) and `VerifyScript.runGate`/`verify mutation-outcome-law` share. */
function policyMutationOutcomeMergePolicyBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyMutationOutcomeBreaches(repoRoot),
    ...policyMutationMessageCodeBreaches(repoRoot),
    ...policyNoCrdtVocabularyBreaches(repoRoot),
    ...policyNoValidateOverrideBreaches(repoRoot),
    ...policySeverityInfoBreaches(repoRoot),
    ...policyMergePolicyParityBreaches(repoRoot),
    ...policyDeriveMirrorBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleMutationOutcomeMergePolicy

//#region 🔧️PolicyRuleInferenceFamily
/**
 * 💡️ Wave P3 inference-family scanners (INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
 * Mirrors 🔧️PolicyRuleMutationArtifactEngines's structure and idioms one region up: `💡️inferences` is the
 * fourth schema family (alongside `📸️snapshot` / `🔺️diff` / `🧬️mutations`), with codecs owned by the
 * sibling `🚪️io/💡️inferences/` collection, same report-mode discipline. REPORT MODE IS LOAD-BEARING:
 * every breach below carries `priority: "medium"` or `"low"`, never `"high"` — registered only at
 * `VerifyScript.runGate`'s `dissolveBreaches` block (which filters to `priority === "high"` before
 * throwing), never the earlier `osBreaches` block (which throws on ANY breach regardless of priority).
 * All 112 owning subsets carry `💡️inferences/`; per-family completeness (root leaves and emoji hygiene)
 * still varies wave to wave as fan-out continues. None of that can gate:
 * every rule here walks only `💡️inferences` dirs that already exist on disk
 * (`policyFindAllInferencesDirs`), never requires the facet's presence, so an unauthored subset — were
 * one to exist — would produce zero breaches rather than a hard block; real incompleteness reports
 * honestly at `medium`/`low` instead.
 */
const POLICY_INFERENCES_FACET = "💡️inferences";

/** 🔎️Inference-specific slug dirs under `💡️inferences/` (skips leaf files and examples; codecs are I/O components). */
function policyListInferenceDirs(repoRoot: string, inferencesRel: string): string[] {
  const reserved = new Set<string>(["📚️examples"]);
  return policyReaddirSafe(repoRoot, inferencesRel)
    .filter((e) => e.isDirectory && !reserved.has(e.name) && !e.name.startsWith("."))
    .map((e) => e.name)
    .sort();
}

/**
 * 🔍️Every `💡️inferences` facet dir anywhere under `✏️s` — same deep-taxonomy walk
 * `policyFindAllMutationsDirs` uses for `🧬️mutations`, so a subset that has not fanned out yet is simply
 * absent from the result rather than reported missing.
 */
function policyFindAllInferencesDirs(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    for (const ent of policyReaddirSafe(repoRoot, relDir)) {
      if (!ent.isDirectory || ent.name.startsWith(".")) continue;
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.name === POLICY_INFERENCES_FACET) {
        found.push(childRel);
        continue;
      }
      walk(childRel);
    }
  };
  walk("✏️s");
  return found.sort();
}

/** 🗿️Owning artifact root for a `💡️inferences` facet dir — same marker-based derivation `policyArtifactRootOfMutationsDir` uses, reused directly since the logic is generic to any `🧬️schema` child, not mutation-specific. */
function policyArtifactRootOfInferencesDir(inferencesRel: string): string {
  return policyArtifactRootOfMutationsDir(inferencesRel);
}

/**
 * 📏️Family-root leaf completeness: every existing `💡️inferences/` must carry the 5 `schemaFormats`
 * root leaves (`🔣️taxonomy.json`'s `schemaFormats` — same SSOT `policyArtifactSchemaFacetCompletenessBreaches`
 * reads for `🧬️schema`/`📸️snapshot`/`🔺️diff`). Format codecs have their own `🚪️io/💡️inferences/` semantic
 * collection and do not belong below the inference result collection.
 */
function policyInferenceFamilyRootCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  for (const inferencesRel of policyFindAllInferencesDirs(repoRoot)) {
    const artRel = policyArtifactRootOfInferencesDir(inferencesRel);
    const rootLeaves = schemaFacetFormatEntries(repoRoot, inferencesRel, taxonomy).map(([, f]) => f.leafFilename);
    for (const leaf of rootLeaves) {
      const rel = `${inferencesRel}/${leaf}`;
      if (existsSync(join(repoRoot, rel))) continue;
      breaches.push({
        id: `inference-family-root-leaf-missing-${rel}`,
        summary: `"${inferencesRel}" is missing family-root leaf ${leaf}`,
        kind: "inference-migration/family-root-completeness",
        scope: artRel,
        priority: "medium",
        reason: "Every 💡️inferences facet root must carry all five schemaFormats leaves (🔣️taxonomy.json), same as 🧬️schema/📸️snapshot/🔺️diff.",
        solution: `Add handcrafted ${rel}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Slug-dir leaf presence: every concrete inference slug dir must carry a real `🦀️component.rs`, and a
 * `🟦️component.ts` that is present AND real — not a trivial `export {};`/empty stub (same bar
 * `policyMutationTsMirrorBreaches` holds triad `.ts` leaves to).
 */
function policyInferenceSlugLeafPresenceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const inferencesRel of policyFindAllInferencesDirs(repoRoot)) {
    const artRel = policyArtifactRootOfInferencesDir(inferencesRel);
    for (const slug of policyListInferenceDirs(repoRoot, inferencesRel)) {
      const slugRel = `${inferencesRel}/${slug}`;
      const rsRel = `${slugRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (!existsSync(join(repoRoot, rsRel))) {
        breaches.push({
          id: `inference-slug-rs-missing-${slugRel}`,
          summary: `"${slugRel}" has no 🦀️component.rs`,
          kind: "inference-migration/slug-leaf-presence",
          scope: artRel,
          priority: "medium",
          reason: "Every 💡️inferences/<slug> dir must carry a real Rust derivation leaf.",
          solution: `Add ${rsRel} with the derivation (impl …InferredField< or a plain pub fn reading the snapshot).`,
        });
      }
      const tsRel = `${slugRel}/${POLICY_TS_COMPONENT_LEAF}`;
      const tsAbs = join(repoRoot, tsRel);
      if (!existsSync(tsAbs)) {
        breaches.push({
          id: `inference-slug-ts-missing-${slugRel}`,
          summary: `"${slugRel}" has no 🟦️component.ts mirror at all`,
          kind: "inference-migration/slug-leaf-presence",
          scope: artRel,
          priority: "medium",
          reason: "Every 💡️inferences/<slug> dir must carry a 🟦️component.ts mirror beside its 🦀️component.rs.",
          solution: `Create ${tsRel} mirroring its 🦀️component.rs sibling.`,
        });
        continue;
      }
      const stripped = policyReadFileSafe(repoRoot, tsRel)
        .replace(/\/\*[\s\S]*?\*\//g, "")
        .replace(/\/\/.*$/gm, "")
        .trim();
      if (stripped === "" || stripped === "export {};") {
        breaches.push({
          id: `inference-slug-ts-stub-${slugRel}`,
          summary: `"${tsRel}" is a trivial "export {};" stub, not a real mirror`,
          kind: "inference-migration/slug-leaf-presence",
          scope: artRel,
          priority: "medium",
          reason: "A slug leaf's TS mirror must be real, not an empty export {} stub — unlike constitutional facet stubs, 💡️inferences carries no structural stub exemption.",
          solution: `Give ${tsRel} real content mirroring its 🦀️component.rs sibling.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️Impl presence: each slug's `🦀️component.rs` must carry a real derivation — either
 * `impl …InferredField<` or a plain `pub fn` reading the snapshot. **Binding coordinator ruling: only
 * 4 of 112 families use `InferredField`; the other 108 are pure-fn folds and are correct** — a merkle
 * dep-chain over a flat whole-snapshot record costs more than the fold it caches, so `InferredField` is
 * required only where the derivation is genuinely per-entity and DAG-shaped (see the puzzle3d
 * `🎛flat-position/` pilot and trinity `🔌️jack/🎛flat-position/`), while a whole-snapshot scalar (e.g.
 * architect's `🧭topology/`) is the sanctioned pure-fn exemplar. Demanding `InferredField` universally
 * would flag 108 correct families — this rule accepts either shape deliberately.
 */
function policyInferenceImplPresenceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const inferredFieldPattern = /\bimpl\b[^\n{]*\bInferredField\s*</;
  const pubFnPattern = /\bpub\s+fn\s+\w+/;
  for (const inferencesRel of policyFindAllInferencesDirs(repoRoot)) {
    const artRel = policyArtifactRootOfInferencesDir(inferencesRel);
    for (const slug of policyListInferenceDirs(repoRoot, inferencesRel)) {
      const rsRel = `${inferencesRel}/${slug}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const abs = join(repoRoot, rsRel);
      if (!existsSync(abs)) continue; // reported by policyInferenceSlugLeafPresenceBreaches
      const content = policyReadFileSafe(repoRoot, rsRel);
      if (inferredFieldPattern.test(content) || pubFnPattern.test(content)) continue;
      breaches.push({
        id: `inference-impl-missing-${rsRel}`,
        summary: `"${rsRel}" has neither an InferredField impl nor a plain pub fn derivation`,
        kind: "inference-migration/impl-presence",
        scope: artRel,
        priority: "medium",
        reason: "Each concrete inference slug must implement InferredField<…> (per-entity DAG-shaped derivations) or expose a plain pub fn reading the snapshot (whole-snapshot pure-fn folds — the sanctioned shape for 108 of 112 families).",
        solution: `Add impl …InferredField<…> for a per-entity derivation, or a plain pub fn compute_${policyStripEmoji(slug).replaceAll("-", "_")}(&Snapshot) -> … for a whole-snapshot fold, in ${rsRel}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Emoji uniqueness (within one family tree only — inference slugs legitimately repeat the SAME emoji
 * across DIFFERENT families by design, e.g. `⏱duration` on animation/audio/mp3/wav/mp4/avi and
 * `🧭topology` on flow/graph/raster/jack; only a collision inside a single `💡️inferences/` tree is a
 * defect) and bare-emoji shape (no U+FE0F): inference slugs are bare by convention (see
 * `isEmojiPrefixedSlugDir`'s own docstring in 🔍️discovery/🟦️component.ts, which cites `📦bounds`,
 * `🧭topology`, `⏱duration`, `🧾outline` as bare exemplars) — unlike most taxonomy dirs, which
 * `requireEmojiPrefixWithVs16`.
 */
function policyInferenceEmojiUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const inferencesRel of policyFindAllInferencesDirs(repoRoot)) {
    const artRel = policyArtifactRootOfInferencesDir(inferencesRel);
    const seen = new Map<string, string>();
    for (const slug of policyListInferenceDirs(repoRoot, inferencesRel)) {
      const emoji = policyLeadingEmojiPrefix(slug);
      if (!emoji) {
        breaches.push({
          id: `inference-emoji-missing-${inferencesRel}/${slug}`,
          summary: `"${inferencesRel}/${slug}" has no leading emoji prefix`,
          kind: "inference-migration/emoji-uniqueness",
          scope: artRel,
          priority: "medium",
          reason: "Each concrete inference slug directory must pick a unique (within its family) emoji prefix.",
          solution: `Rename ${slug} to include a leading emoji prefix (e.g. 🧭topology).`,
        });
        continue;
      }
      if (emoji.includes("️")) {
        breaches.push({
          id: `inference-emoji-vs16-${inferencesRel}/${slug}`,
          summary: `"${inferencesRel}/${slug}" carries U+FE0F on its emoji prefix — inference slugs are bare by convention`,
          kind: "inference-migration/emoji-uniqueness",
          scope: artRel,
          priority: "low",
          reason: "Inference slug dirs are bare-emoji by established convention (📦bounds, 🧭topology, ⏱duration, 🧾outline all lack U+FE0F) — unlike most taxonomy dirs, which require it.",
          solution: `Rename ${slug} to drop the U+FE0F variation selector after its leading emoji.`,
        });
      }
      const prev = seen.get(emoji);
      if (prev) {
        breaches.push({
          id: `inference-emoji-dup-${artRel}-${emoji}-${slug}`,
          summary: `"${inferencesRel}/${slug}" reuses emoji "${emoji}" already used by "${prev}" within the same family`,
          kind: "inference-migration/emoji-uniqueness",
          scope: artRel,
          priority: "medium",
          reason: "Inference slug emojis must be unique WITHIN one artifact's 💡️inferences/ tree — reuse across different families/artifacts is fine and common by design.",
          solution: `Give ${slug} a different emoji than ${prev} (scoped to this family only).`,
        });
        continue;
      }
      seen.set(emoji, slug);
    }
  }
  return breaches;
}

/**
 * 📏️kebab→camel assembly coverage: every `💡️inferences/<slug>` dir must correspond to a field on the
 * family-root `<Prefix>Inference` struct, and every field on that struct must correspond to a real slug
 * dir. Matching normalizes both the slug stem and the Rust field name/type by stripping separators and
 * casing (so `flat-position` ↔ `flat_position`/`FlatPosition`/`flatPosition` all collapse to the same
 * key) — real families mix snake_case field names, PascalCase field types, and camelCase serde/id output
 * for the same concept (see trinity `🔌️jack`'s `flat_position: JackFlatPosition` field, whose
 * `InferenceFieldSpec` id ends `...flatPosition`), so a single casing convention would false-positive.
 * Structurally mirrors `policyMutationDispatchCoverageBreaches` one region up (orphan/uncovered
 * diffing between a directory set and a Rust declaration), reuses `policyStripEmoji` from the same
 * mutation cluster, and reuses `policyExtractRustSchemaFields` from the artifact-schema cluster to
 * read the struct's real fields instead of re-deriving a Rust field parser.
 */
function policyInferenceNormalizeToken(raw: string): string {
  return raw.toLowerCase().replace(/[^a-z0-9]/g, "");
}

function policyInferenceAssemblyCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const inferencesRel of policyFindAllInferencesDirs(repoRoot)) {
    const artRel = policyArtifactRootOfInferencesDir(inferencesRel);
    const rootRsRel = `${inferencesRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (!existsSync(join(repoRoot, rootRsRel))) continue; // reported by policyInferenceFamilyRootCompletenessBreaches
    const content = policyReadFileSafe(repoRoot, rootRsRel);
    const structMatch = /\bpub\s+struct\s+(\w+Inference)\b/.exec(content);
    if (!structMatch) continue; // no XInference struct yet — family-root completeness already flags the missing/incomplete root
    const structName = structMatch[1]!;
    const extract = policyExtractRustSchemaFields(content, structName);
    if (extract.typeName !== structName) continue; // extractor could not isolate the struct body — avoid a false coverage report
    const slugDirs = policyListInferenceDirs(repoRoot, inferencesRel);
    const slugTokens = new Map(slugDirs.map((s) => [s, policyInferenceNormalizeToken(policyStripEmoji(s))]));
    const fieldTokens = extract.fields.map((f) => ({ field: f, nameToken: policyInferenceNormalizeToken(f.name), scalarToken: policyInferenceNormalizeToken(f.scalar) }));
    for (const slug of slugDirs) {
      const token = slugTokens.get(slug)!;
      const covered = fieldTokens.some((f) => f.nameToken === token || f.scalarToken.endsWith(token));
      if (covered) continue;
      breaches.push({
        id: `inference-orphan-slug-${inferencesRel}/${slug}`,
        summary: `"${inferencesRel}/${slug}" has no matching field on ${structName}`,
        kind: "inference-migration/assembly-coverage",
        scope: artRel,
        priority: "medium",
        reason: `Every 💡️inferences/<slug> dir must be assembled into a #[derived] field of ${structName} — a slug dir the family root never references is dead weight.`,
        solution: `Add a field on ${structName} named or typed after "${policyStripEmoji(slug)}", or delete ${inferencesRel}/${slug} if it is stale.`,
      });
    }
    for (const f of fieldTokens) {
      const hasSlug = [...slugTokens.values()].some((token) => f.nameToken === token || f.scalarToken.endsWith(token));
      if (hasSlug) continue;
      breaches.push({
        id: `inference-uncovered-field-${inferencesRel}-${f.field.name}`,
        summary: `"${structName}.${f.field.name}" has no matching 💡️inferences/<slug> dir`,
        kind: "inference-migration/assembly-coverage",
        scope: artRel,
        priority: "medium",
        reason: `Every ${structName} field should be backed by a real 💡️inferences/<slug>/ derivation dir — a field with no matching dir is unassembled or the naming has drifted.`,
        solution: `Rename a slug dir to match "${f.field.name}", or add the missing 💡️inferences/<slug>/ if the field is new.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_DERIVED_MARKER`: `#[derived]` may appear only inside `💡️inferences/`, never in a
 * `📸️snapshot` facet — a snapshot field is persisted input; marking it derived would blur computed
 * values into stored state, which is exactly the escape hatch the dep-hash cache design closes.
 *
 * Derivation is its OWN axis, orthogonal to the four state lanes (`artifact`/`config`/`presence`/
 * `transient`) — it deliberately no longer rides on a `StateClass` variant, so this rule now watches
 * the `#[derived]` attribute (JSON Schema twin: `x-semio-derived: true`) instead of a state token.
 */
const POLICY_DERIVED_MARKER = "#[derived]";

function policyDerivedMarkerLeakBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, ["✏️s"], (relPath, name) => {
    if (name !== POLICY_RS_COMPONENT_LEAF_NAME) return false;
    const norm = relPath.replaceAll("\\", "/");
    return norm.includes("/📸️snapshot/") && !norm.includes(`/${POLICY_INFERENCES_FACET}/`);
  });
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    const idx = content.indexOf(POLICY_DERIVED_MARKER);
    if (idx < 0) continue;
    breaches.push({
      id: `derived-marker-leak-${relPath}`,
      summary: `"${relPath}" declares ${POLICY_DERIVED_MARKER} inside a 📸️snapshot facet`,
      kind: "inference-migration/state-leak",
      scope: relPath,
      line: policyLineOfIndex(content, idx),
      priority: "medium",
      reason: "#[derived] marks a field as computed-and-cached; that contract belongs exclusively to 💡️inferences/ — a snapshot facet field is persisted input and must never carry it.",
      solution: `Move the derived field out of ${relPath} into a 💡️inferences/<slug>/${POLICY_RS_COMPONENT_LEAF_NAME} sibling.`,
    });
  }
  return breaches;
}

/** ⚖️Aggregates the P3 inference-family scanners. */
function policyInferenceFamilyBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyInferenceFamilyRootCompletenessBreaches(repoRoot),
    ...policyInferenceSlugLeafPresenceBreaches(repoRoot),
    ...policyInferenceImplPresenceBreaches(repoRoot),
    ...policyInferenceEmojiUniquenessBreaches(repoRoot),
    ...policyInferenceAssemblyCoverageBreaches(repoRoot),
    ...policyDerivedMarkerLeakBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleInferenceFamily

//#region 🔧️PolicyRuleArtifactSchemas
/**
 * 🧬️Wave W2 artifact-schema facet scanners (ARTIFACT-SCHEMA-FACETS).
 * Three facets × five `schemaFormats` leaves must agree on canonical camelCase fields; extractors are the
 * compiler this design deliberately does not have. Nested `📸️snapshot` / `🔺️diff` children are recognized
 * via taxonomy `schemaChildDirs` / `representationDirs` / `ioDirectionDirs` in `policyTaxonomyDirsBreaches`.
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
  const expected = policyExpectedSchemaTypeNameForFacetPath(facetRel);
  const out: { formatId: string; leafFilename: string; fieldCasing: string; relPath: string; extract: PolicySchemaLeafExtract | null }[] = [];
  for (const [formatId, format] of schemaFacetFormatEntries(repoRoot, facetRel, taxonomy)) {
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
      case "📜️wit":
        extract = { typeName: "", fields: [] };
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
      for (const [formatId, format] of schemaFacetFormatEntries(repoRoot, facetAbs, taxonomy)) {
        const leafRel = `${facetAbs}/${format.leafFilename}`;
        if (existsSync(join(repoRoot, leafRel))) continue;
        breaches.push({
          id: `artifact-schema-leaf-missing-${leafRel}`,
          summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
          kind: "artifact-schema/facet-completeness",
          scope: artRel,
          priority: "high",
          reason: "Each schema facet must carry every schemaFormats leaf for its facet kind from 🔣️taxonomy.json.",
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
 * 📏️State-class parity: snapshot facet fields equal exactly the `artifact`-lane fields of the artifact facet.
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
    const persistent = artJson.fields.filter((f) => f.state === "artifact");
    const snapMap = new Map(snapJson.fields.map((f) => [f.name, f]));
    const persMap = new Map(persistent.map((f) => [f.name, f]));
    for (const f of persistent) {
      const s = snapMap.get(f.name);
      if (!s) {
        breaches.push({
          id: `artifact-schema-state-parity-missing-${artRel}-${f.name}`,
          summary: `Snapshot facet is missing artifact-lane field "${f.name}"`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: "XSnapshot must equal exactly the artifact-lane fields of XArtifact (equality, not subset).",
          solution: `Add "${f.name}" to ${snapshotFacet}/🔣️component.json (and the other four leaves) matching the artifact facet.`,
        });
        continue;
      }
      if (s.optional !== f.optional || s.cardinality !== f.cardinality) {
        breaches.push({
          id: `artifact-schema-state-parity-shape-${artRel}-${f.name}`,
          summary: `Snapshot field "${f.name}" shape differs from the artifact-lane field`,
          kind: "artifact-schema/state-parity",
          scope: artRel,
          priority: "high",
          reason: `Artifact-lane field "${f.name}" is optional=${f.optional}, cardinality=${f.cardinality}; snapshot has optional=${s.optional}, cardinality=${s.cardinality}.`,
          solution: `Align "${f.name}" in ${snapshotFacet}/🔣️component.json with ${artifactFacet}/🔣️component.json.`,
        });
      }
    }
    for (const name of snapMap.keys()) {
      if (persMap.has(name)) continue;
      breaches.push({
        id: `artifact-schema-state-parity-extra-${artRel}-${name}`,
        summary: `Snapshot facet has non-artifact-lane field "${name}"`,
        kind: "artifact-schema/state-parity",
        scope: artRel,
        priority: "high",
        reason: "XSnapshot may only contain the artifact-lane fields of XArtifact.",
        solution: `Remove "${name}" from ${snapshotFacet}/🔣️component.json, or move it into the artifact lane on the artifact facet if it belongs there.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️Diff coverage: every non-transient artifact field has a diff entry; no transient field does; `artifact` exists.
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
      if (f.state === "transient") {
        if (diffNames.has(f.name)) {
          breaches.push({
            id: `artifact-schema-diff-effect-${artRel}-${f.name}`,
            summary: `Diff facet must not cover transient field "${f.name}"`,
            kind: "artifact-schema/diff-coverage",
            scope: artRel,
            priority: "high",
            reason: "Transient fields are ephemeral local-only UI state — never shared, never diffed — and must not appear in XDiff.",
            solution: `Remove "${f.name}" from ${diffFacet}/🔣️component.json.`,
          });
        }
        continue;
      }
      if (!diffNames.has(f.name)) {
        breaches.push({
          id: `artifact-schema-diff-coverage-${artRel}-${f.name}`,
          summary: `Diff facet is missing entry for non-transient artifact field "${f.name}"`,
          kind: "artifact-schema/diff-coverage",
          scope: artRel,
          priority: "high",
          reason: "Every artifact field whose state lane is not transient must have a same-named diff entry.",
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

/** ⚖️Aggregates artifact-schema facet scanners (completeness, parity, coverage). */
export function policyArtifactSchemaBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyArtifactSchemaFacetCompletenessBreaches(repoRoot),
    ...policyArtifactSchemaFieldParityBreaches(repoRoot),
    ...policyArtifactSchemaStateParityBreaches(repoRoot),
    ...policyArtifactSchemaDiffCoverageBreaches(repoRoot),
    ...policyArtifactSchemaTypeNameParityBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleArtifactSchemas

//#region 🔧️PolicyRuleAppSchemas
/**
 * 🧬️Wave A2 surface-schema facet scanners (APP-SCHEMA-FACETS, retargeted W3 from `🎛️apps` to the two
 * per-subset surfaces). Two facets (config + presence) × five `schemaFormats` leaves; the five
 * per-format extractors from `PolicyRuleArtifactSchemas` are reused unchanged. Owners are derived from
 * each surface's `type Config = …` binding — never a hand-maintained prefix table.
 */

/** 🎚️Canonical surface config dir (level-slider). */
const POLICY_APP_CONFIG_DIR = "🎚️config";
/** 🧮Legacy abacus config dir — forbidden by `app-schema/config-relocation`. */
const POLICY_APP_CONFIG_LEGACY_DIR = "🧮️config";
/** 👥️Surface presence dir, sibling of the config owner. */
const POLICY_APP_PRESENCE_DIR = "👥️presence";
/** 🕸️Legacy wasm dir — forbidden by `app-schema/config-relocation`. */
const POLICY_APP_WASM_LEGACY_DIR = "🕸️wasm";
/** 🧬️Schema facet folder under a config or presence owner. */
const POLICY_APP_SCHEMA_FACET = "🧬️schema";

/** 🪪One discovered surface-schema owner (deduped by owner path). */
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
 * 🗂️Walk every plugin's `👁️viewer`/`✏️editor` surface `🦀️component.rs`, parse `type Config = XConfig;`,
 * and resolve the config owner dir (surface `🎚️config`, else legacy `🧮️config`, else plugin-level
 * `🎚️config` that declares `pub struct XConfig`). Presence owner is the sibling `👥️presence` under the
 * same parent.
 */
export function policyDiscoverAppSchemaOwners(repoRoot: string): PolicyAppSchemaOwner[] {
  const pluginsRoot = "✏️s/🔌️plugins";
  const taxonomy = loadTaxonomy();
  const byOwner = new Map<string, PolicyAppSchemaOwner>();
  for (const plugin of policyReaddirSafe(repoRoot, pluginsRoot)) {
    if (!plugin.isDirectory) continue;
    const pluginRel = `${pluginsRoot}/${plugin.name}`;
    for (const surfaceRel of policySurfaceRoots(repoRoot, pluginRel, taxonomy)) {
      const componentRel = `${surfaceRel}/🦀️component.rs`;
      if (!existsSync(join(repoRoot, componentRel))) continue;
      const text = policyReadFileSafe(repoRoot, componentRel);
      const m = /\btype\s+Config\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;/.exec(text);
      if (!m) continue;
      const configType = m[1]!;
      const sliderRel = `${surfaceRel}/${POLICY_APP_CONFIG_DIR}`;
      const legacyRel = `${surfaceRel}/${POLICY_APP_CONFIG_LEGACY_DIR}`;
      const pluginConfigRel = `${pluginRel}/${POLICY_APP_CONFIG_DIR}`;
      let ownerRel: string | null = null;
      if (existsSync(join(repoRoot, sliderRel))) {
        ownerRel = sliderRel;
      } else if (existsSync(join(repoRoot, legacyRel))) {
        ownerRel = legacyRel;
      } else {
        const pluginCfgRs = `${pluginConfigRel}/🦀️component.rs`;
        if (
          existsSync(join(repoRoot, pluginCfgRs)) &&
          new RegExp(`\\bpub\\s+struct\\s+${configType}\\b`).test(policyReadFileSafe(repoRoot, pluginCfgRs))
        ) {
          ownerRel = pluginConfigRel;
        }
      }
      if (!ownerRel) continue;
      const presenceType = policyAppPresenceTypeName(configType);
      const parentRel = ownerRel.split("/").slice(0, -1).join("/");
      const presenceRel = `${parentRel}/${POLICY_APP_PRESENCE_DIR}`;
      const [subsetName, roleDirName] = surfaceRel.split("/").slice(-2);
      const surfaceId = `${plugin.name}/${subsetName}/${roleDirName}`;
      const existing = byOwner.get(ownerRel);
      if (existing) {
        existing.apps.push(surfaceId);
        continue;
      }
      byOwner.set(ownerRel, { ownerRel, configType, presenceType, presenceRel, apps: [surfaceId] });
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

/** 🧭️Taxonomy `surfaceSchemaSpecFilenames` key for a config or presence facet. */
function policyAppSchemaFacetRole(kind: "config" | "presence"): string {
  return kind === "config"
    ? `${POLICY_APP_CONFIG_DIR}/${POLICY_APP_SCHEMA_FACET}`
    : `${POLICY_APP_PRESENCE_DIR}/${POLICY_APP_SCHEMA_FACET}`;
}

/**
 * 📏️Facet completeness + normative leaf: both config and presence schema facets, each with every
 * schemaFormats leaf and the `surfaceSchemaSpecFilenames` normative JSON Schema leaf.
 */
function policyAppSchemaFacetCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const normativeByFacet = taxonomy.surfaceSchemaSpecFilenames ?? {};
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
      for (const [formatId, format] of schemaFacetFormatEntries(repoRoot, facetAbs, taxonomy)) {
        const leafRel = `${facetAbs}/${format.leafFilename}`;
        if (existsSync(join(repoRoot, leafRel))) continue;
        breaches.push({
          id: `app-schema-leaf-missing-${leafRel}`,
          summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
          kind: "app-schema/facet-completeness",
          scope: owner.ownerRel,
          priority: "high",
          reason: "Each schema facet must carry every schemaFormats leaf for its facet kind from 🔣️taxonomy.json.",
          solution: `Add handcrafted ${leafRel}.`,
        });
      }
      const normative = normativeByFacet[policyAppSchemaFacetRole(kind)] ?? "🔣️component.json";
      const normativeRel = `${facetAbs}/${normative}`;
      if (!existsSync(join(repoRoot, normativeRel))) {
        breaches.push({
          id: `app-schema-normative-missing-${normativeRel}`,
          summary: `"${facetAbs}" is missing normative surfaceSchemaSpecFilenames leaf ${normative}`,
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
    const real = policyExtractRustSchemaFields(policyReadFileSafe(repoRoot, cfgRs), owner.configType);
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
 * 📏️State purity: every config-facet field is `config`; every presence-facet field is `presence`.
 */
function policyAppSchemaStatePurityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyDiscoverAppSchemaOwners(repoRoot)) {
    const checks: { facetAbs: string; expectedState: string; expectedTypeName: string; label: string }[] = [
      {
        facetAbs: `${owner.ownerRel}/${POLICY_APP_SCHEMA_FACET}`,
        expectedState: "config",
        expectedTypeName: owner.configType,
        label: "config",
      },
      {
        facetAbs: `${owner.presenceRel}/${POLICY_APP_SCHEMA_FACET}`,
        expectedState: "presence",
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
/** 🧾 Schema-derived stdio definition view for policy checks. */
const POLICY_STDIO_OWNER_TABLE_REL = "✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json";
const POLICY_STDIO_PLUGIN_REL = "✏️s/🔌️plugins/🗄️stdio";
const POLICY_STDIO_ARTIFACTS_REL = `${POLICY_STDIO_PLUGIN_REL}/🗿️artifacts`;
const POLICY_STDIO_FACET_DECOMPOSER = "🪓️decomposer";
const POLICY_STDIO_FACET_TEXT = "📝️text";
const POLICY_STDIO_FACET_BINARY = "💾️binary";
const POLICY_STDIO_FACET_DESERIALIZERS = "🧩️deserializers";
const POLICY_STDIO_FACET_SERIALIZERS = "🧵️serializers";
const POLICY_STDIO_IO_IMPORT = "📥️import";
const POLICY_STDIO_IO_EXPORT = "📤️export";
const POLICY_STDIO_SCHEMA_CHILD_FALLBACK = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;
const POLICY_STDIO_REPRESENTATION_FALLBACK = [POLICY_STDIO_FACET_TEXT, POLICY_STDIO_FACET_BINARY] as const;
const POLICY_STDIO_ARTIFACT_FACET_FALLBACK = ["🧬️schema", "⚙️engine", "🚪️io", POLICY_STDIO_FACET_DECOMPOSER] as const;
const POLICY_STDIO_LEGACY_ARTIFACT_FACETS = new Set(["🗣️dsl", "🔧️op", "📡️spr", "🔺️diff", "📸️snapshot"]);
const POLICY_STDIO_TEXT_SPEC_LEAVES = [
  "📖️component.grammar.semio",
  "🔤️component.ebnf",
  "🅰️component.g4",
  "🔗️component.graphql",
  "🔣️component.json",
  "🛰️component.proto",
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;
const POLICY_STDIO_BINARY_SPEC_LEAVES = [
  "📡️component.protocol.semio",
  "🔠️component.abnf",
  "🥋️component.ksy",
  "🌶️component.spicy",
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;
const POLICY_STDIO_CODEC_BANNED_MARKERS = ["SRAS", "IFCCARTOONMESH", "b\"minimal\"", "stub codec", "minimal stub codec"] as const;

type PolicyStdioDefinitionTable = {
  artifacts: Record<string, { dir: string; depends: string[] }>;
  dependency_edges: { from: string; to: string }[];
  owners: Array<{
    path: string;
    stdio_artifacts: string[];
    import: string[];
    export: string[];
  }>;
  counts?: { stdio_artifacts?: number };
};

function policyLoadStdioOwnerTable(repoRoot: string): PolicyStdioDefinitionTable | null {
  try {
    const ledger = stdioArtifactLedger(repoRoot);
    const artifacts = Object.fromEntries(ledger.artifacts.map((artifact) => [artifact.id, { dir: artifact.directory, depends: [...artifact.depends] }]));
    return { artifacts, dependency_edges: ledger.artifacts.flatMap((artifact) => artifact.depends.map((to) => ({ from: artifact.id, to }))), owners: [], counts: { stdio_artifacts: ledger.counts.artifacts } };
  } catch {
    return null;
  }
}

function policyStdioArtifactFacets(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const dirs = taxonomy.artifactComponentDirs;
  if (!dirs?.length) return [...POLICY_STDIO_ARTIFACT_FACET_FALLBACK];
  const out = new Set<string>(POLICY_STDIO_ARTIFACT_FACET_FALLBACK);
  for (const d of dirs) {
    if (!POLICY_STDIO_LEGACY_ARTIFACT_FACETS.has(d)) out.add(d);
  }
  return [...out];
}

function policyStdioSchemaChildDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const from = taxonomy.schemaChildDirs as string[] | undefined;
  return from?.length ? [...from] : [...POLICY_STDIO_SCHEMA_CHILD_FALLBACK];
}

function policyStdioRepresentationDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const from = taxonomy.representationDirs as string[] | undefined;
  return from?.length ? [...from] : [...POLICY_STDIO_REPRESENTATION_FALLBACK];
}

function policyStdioFormatDir(artifacts: PolicyStdioDefinitionTable["artifacts"], formatId: string): string | undefined {
  return artifacts[formatId]?.dir;
}

function policyStdioArtifactsDirName(): string {
  return loadTaxonomy().artifactsDirName ?? "🗿️artifacts";
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 1: strips
 * `//` line comments, `/* *​/` block comments (nesting-aware -- Rust block comments nest), and
 * double-quoted string literal bodies (raw strings `r#"..."#` are NOT unescaped -- a false-negative
 * there is far cheaper than the false-positives block/line comments would otherwise cause) so
 * `impl <Trait> for` detection only ever sees real code, never a trait name mentioned in a doc
 * comment or a string. Replaces the old `body.includes(traitName)` substring check, which a doc
 * comment merely NAMING the trait (e.g. "still needs ArtifactBuilder") would satisfy without a
 * single real `impl` block existing.
 */
function policyStripRustCommentsAndStrings(content: string): string {
  let out = "";
  let i = 0;
  const n = content.length;
  while (i < n) {
    const two = content.slice(i, i + 2);
    if (two === "//") {
      while (i < n && content[i] !== "\n") i++;
      continue;
    }
    if (two === "/*") {
      i += 2;
      let depth = 1;
      while (i < n && depth > 0) {
        const pair = content.slice(i, i + 2);
        if (pair === "/*") { depth++; i += 2; continue; }
        if (pair === "*/") { depth--; i += 2; continue; }
        i++;
      }
      continue;
    }
    if (content[i] === '"') {
      out += '"';
      i++;
      while (i < n && content[i] !== '"') {
        if (content[i] === "\\") { i += 2; continue; }
        i++;
      }
      if (i < n) i++;
      out += '"';
      continue;
    }
    if (content[i] === "'") {
      // 🩹 Rust char literal (`'"'`, `'{'`, `'\n'`, `'\''`, `'\u{1F600}'`, …) vs a lifetime/generic
      // tick (`'a`, `'static`) — a char literal always closes with another `'` within a few chars
      // (accounting for escapes); a lifetime tick never does. Without this, a char literal whose
      // content is itself `'"'` gets misread as the START of a double-quoted string by the block
      // above, silently swallowing everything up to the next literal `"` in the file — which can
      // hide real code (e.g. a later `impl Trait for ...`) from every regex-based policy check
      // that runs against the stripped output.
      let j = i + 1;
      if (content[j] === "\\") {
        j += 1;
        if (content[j] === "u" && content[j + 1] === "{") {
          j += 2;
          while (j < n && content[j] !== "}") j++;
          j += 1;
        } else {
          j += content[j] === "x" ? 3 : 1;
        }
      } else if (j < n) {
        j += 1;
      }
      if (content[j] === "'") {
        out += content.slice(i, j + 1);
        i = j + 1;
        continue;
      }
      // Not a char literal -- a lifetime/generic tick. Copy just the tick through; the identifier
      // that follows contains no quote characters, so falling through the loop handles it safely.
      out += content[i];
      i++;
      continue;
    }
    out += content[i];
    i++;
  }
  return out;
}

function policyRustFileHasRealTraitImpl(body: string, traitName: string): boolean {
  const stripped = policyStripRustCommentsAndStrings(body);
  return new RegExp(`\\bimpl\\s*(?:<[^>{]*>\\s*)?${traitName}\\b\\s*(?:<[^>{]*>\\s*)?for\\b`).test(stripped);
}

function policyStdioFacetRsTsBreaches(
  repoRoot: string,
  facetRel: string,
  scope: string,
  kind: string,
  traitName: string,
): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const rsRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF}`;
  const tsRel = `${facetRel}/${POLICY_TS_COMPONENT_LEAF}`;
  if (!existsSync(join(repoRoot, facetRel))) {
    breaches.push({
      id: `${kind}-missing-${facetRel}`,
      summary: `"${scope}" is missing ${facetRel.split("/").pop()}/`,
      kind,
      scope,
      priority: "high",
      reason: `Every artifact must expose ${facetRel.split("/").pop()} with Rust and TypeScript taxonomy leaves.`,
      solution: `Create ${facetRel}/ with ${POLICY_RS_COMPONENT_LEAF} and ${POLICY_TS_COMPONENT_LEAF}.`,
    });
    return breaches;
  }
  if (!existsSync(join(repoRoot, rsRel))) {
    breaches.push({
      id: `${kind}-rs-${facetRel}`,
      summary: `"${rsRel}" is missing`,
      kind,
      scope,
      priority: "high",
      reason: `Facet ${facetRel} must declare ${traitName} in ${POLICY_RS_COMPONENT_LEAF}.`,
      solution: `Add ${rsRel} implementing ${traitName}.`,
    });
  } else {
    const body = policyReadFileSafe(repoRoot, rsRel);
    if (!policyRustFileHasRealTraitImpl(body, traitName)) {
      breaches.push({
        id: `${kind}-trait-${facetRel}`,
        summary: `"${rsRel}" has no real "impl ${traitName} for ..." block (comment/string-stripped)`,
        kind,
        scope,
        priority: "high",
        reason: `${POLICY_RS_COMPONENT_LEAF} must implement the SDK ${traitName} trait with a real impl block, not just mention its name in a comment or string.`,
        solution: `Add a real "impl ${traitName} for <YourType>" block in ${rsRel}.`,
      });
    }
  }
  if (!existsSync(join(repoRoot, tsRel))) {
    breaches.push({
      id: `${kind}-ts-${facetRel}`,
      summary: `"${tsRel}" is missing`,
      kind,
      scope,
      priority: "high",
      reason: `Facet ${facetRel} must re-export ${traitName} from the TypeScript barrel leaf.`,
      solution: `Add ${tsRel} exporting ${traitName}.`,
    });
  }
  return breaches;
}

/** ⚖️Thirty-six stdio codec artifacts exist under 🗄️stdio with required completeness facets. */
export function policyStdioCatalogBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) {
    breaches.push({
      id: "stdio-catalog-owner-table-missing",
      summary: `owner table missing at ${POLICY_STDIO_OWNER_TABLE_REL}`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio roster and DAG are normative in the ticket owner table until taxonomy absorbs them.",
      solution: `Restore ${POLICY_STDIO_OWNER_TABLE_REL}.`,
    });
    return breaches;
  }
  const expectedCount = table.counts?.stdio_artifacts ?? 36;
  const rosterIds = Object.keys(table.artifacts);
  if (rosterIds.length !== expectedCount) {
    breaches.push({
      id: "stdio-catalog-roster-count",
      summary: `schema-derived artifact definitions have ${rosterIds.length} entries but normative count is ${expectedCount}`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "The closed stdio catalog must list exactly 36 format artifacts.",
      solution: `Fix schema-owned definitions under ${POLICY_STDIO_OWNER_TABLE_REL}.`,
    });
  }
  const pluginRoot = join(repoRoot, POLICY_STDIO_PLUGIN_REL);
  if (!existsSync(pluginRoot)) {
    breaches.push({
      id: "stdio-catalog-plugin-missing",
      summary: `${POLICY_STDIO_PLUGIN_REL} plugin root is missing`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codecs live in the dedicated 🗄️stdio plugin (zero apps).",
      solution: `Scaffold ${POLICY_STDIO_PLUGIN_REL} per ticket W2.`,
    });
    return breaches;
  }
  const taxonomy = loadTaxonomy();
  const requiredFacets = policyStdioArtifactFacets(taxonomy);
  for (const formatId of rosterIds) {
    const entry = table.artifacts[formatId]!;
    const artRel = `${POLICY_STDIO_ARTIFACTS_REL}/${entry.dir}`;
    const scope = `🗄️stdio/${entry.dir}`;
    if (!existsSync(join(repoRoot, artRel))) {
      breaches.push({
        id: `stdio-catalog-artifact-${formatId}`,
        summary: `stdio artifact "${formatId}" missing at ${artRel}`,
        kind: "stdio-artifacts/catalog",
        scope,
        priority: "high",
        reason: "Every stdio roster id must materialize as an artifact directory under 🗄️stdio.",
        solution: `Create ${artRel}/ with builder, decomposer, schema, engine, and io facets.`,
      });
      continue;
    }
    if (policyArtifactIsMigrated(repoRoot, artRel)) {
      // Migrated: the flat schema/engine/io/builder/decomposer facets are gone by design --
      // policyStandardsCoverageBreaches (+ the Builder/Analyzer/Composer migrated rules) own
      // the deep check under 🏅️standards/ now.
      continue;
    }
    for (const facet of requiredFacets) {
      const facetRel = `${artRel}/${facet}`;
      if (existsSync(join(repoRoot, facetRel))) continue;
      breaches.push({
        id: `stdio-catalog-facet-${formatId}-${facet}`,
        summary: `"${artRel}" is missing required facet ${facet}/`,
        kind: "stdio-artifacts/catalog",
        scope,
        priority: "high",
        reason: "Stdio codec artifacts carry the same completeness facets as domain artifacts.",
        solution: `Add ${facetRel}/ per normative spec §2.`,
      });
    }
  }
  return breaches;
}

/** 🏅 True once an artifact has grown a 🏅️standards/ child -- see 🏅️PolicyRuleStandardsSubsets below.
 * The seven original rules skip migrated artifacts entirely; the new standards/subsets rules own them. */
function policyArtifactIsMigrated(repoRoot: string, artRel: string): boolean {
  const taxonomy = loadTaxonomy();
  const standardsDirName = (taxonomy as any).standardsDirName ?? POLICY_STANDARDS_DIR;
  return existsSync(join(repoRoot, `${artRel}/${standardsDirName}`));
}

/** ⚖️Legacy artifacts no longer own an explicit builder facet. */
export function policyArtifactBuilderBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    if (policyArtifactIsMigrated(repoRoot, artRel)) continue;
    const facetRel = `${artRel}/🏗️builder`;
    if (!existsSync(join(repoRoot, facetRel))) continue;
    breaches.push({
      id: `artifact-derived-builder-${artRel}`,
      summary: `"${artRel}" has a forbidden explicit builder facet`,
      kind: "stdio-artifacts/derived-facets",
      scope: artRel,
      priority: "high",
      reason: "Artifact lifecycle capabilities are derived from schema and IO hooks.",
      solution: `Remove ${facetRel}/ and derive its builder from the subset schema.`,
    });
  }
  return breaches;
}

/** ⚖️Every not-yet-migrated plugin artifact exposes 🪓️decomposer with rs+ts implementing ArtifactDecomposer. */
export function policyArtifactDecomposerBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    if (policyArtifactIsMigrated(repoRoot, artRel)) continue;
    breaches.push(
      ...policyStdioFacetRsTsBreaches(
        repoRoot,
        `${artRel}/${POLICY_STDIO_FACET_DECOMPOSER}`,
        artRel,
        "stdio-artifacts/decomposer",
        "ArtifactDecomposer",
      ),
    );
  }
  return breaches;
}

//#region 🏅️PolicyRuleStandardsSubsets
/**
 * 🏅 W9+ (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase 2): dual-shape vocabulary.
 * An artifact is "migrated" once it has a 🏅️standards/ child; unmigrated artifacts
 * are validated exclusively by the seven rules above and never touched here. This
 * keeps the whole sweep additive: these rules are vacuous ([]) until the first
 * artifact actually grows a 🏅️standards/ dir.
 */
const POLICY_STANDARDS_DIR = "🏅️standards";
const POLICY_SUBSETS_DIR = "🪆️subsets";
const POLICY_DERIVED_FACETS = [
  { dir: "🏗️builder", hook: "construction:", name: "builder" },
  { dir: "🧐️analyzer", hook: "analysis:", name: "analyzer" },
  { dir: "🎹️composer", hook: "composition:", name: "composer" },
] as const;

type PolicyArtifactDialect = {
  artRel: string;
  standardRel: string;
  standardSlug: string;
  subsetRel: string;
  /** 🪆️ Raw on-disk dir name, e.g. "✳️any", "✳️a", "✳️cc6" — kept for path-building. */
  subsetDirName: string;
  /** 🪆️ Logical subset id: `subsetDirName` with the `✳️` prefix stripped and "any" mapped to "*"
   * (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES) — this is what a standard's
   * `🪆️subsets/🔣️component.json` manifest and Rust `SubsetId` both speak. */
  subsetId: string;
};

/** 🪆 Maps a subset dir name to its logical id via taxonomy's subsetDirPrefix/subsetAnyId/subsetAnyDirName. */
function policySubsetIdFromDirName(taxonomy: ReturnType<typeof loadTaxonomy>, dirName: string): string {
  const anyDirName = (taxonomy as any).subsetAnyDirName ?? "✳️any";
  const anyId = (taxonomy as any).subsetAnyId ?? "*";
  if (dirName === anyDirName) return anyId;
  const prefix = (taxonomy as any).subsetDirPrefix ?? "✳️";
  return dirName.startsWith(prefix) ? dirName.slice(prefix.length) : dirName;
}

/** 🏅 One row per (migrated artifact, standard, subset) triple; empty until any artifact migrates. */
function policyListArtifactDialectDirs(repoRoot: string): PolicyArtifactDialect[] {
  const out: PolicyArtifactDialect[] = [];
  const taxonomy = loadTaxonomy();
  const standardsDirName = (taxonomy as any).standardsDirName ?? POLICY_STANDARDS_DIR;
  const subsetsDirName = (taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR;
  const standardPrefix = (taxonomy as any).standardDirPrefix ?? "🔖️";
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const standardsRel = `${artRel}/${standardsDirName}`;
    if (!existsSync(join(repoRoot, standardsRel))) continue;
    for (const std of policyReaddirSafe(repoRoot, standardsRel)) {
      if (!std.isDirectory || !std.name.startsWith(standardPrefix)) continue;
      const standardRel = `${standardsRel}/${std.name}`;
      const standardSlug = std.name.slice(standardPrefix.length);
      const subsetsRel = `${standardRel}/${subsetsDirName}`;
      if (!existsSync(join(repoRoot, subsetsRel))) continue;
      for (const sub of policyReaddirSafe(repoRoot, subsetsRel)) {
        if (!sub.isDirectory) continue;
        out.push({
          artRel,
          standardRel,
          standardSlug,
          subsetRel: `${subsetsRel}/${sub.name}`,
          subsetDirName: sub.name,
          subsetId: policySubsetIdFromDirName(taxonomy, sub.name),
        });
      }
    }
  }
  return out;
}

/** ⚖️Every migrated standard/subset dir carries its required engine/schema/IO children. */
export function policyStandardsCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const standardChildDirs = ((taxonomy as any).standardChildDirs as string[] | undefined) ?? [];
  const subsetChildDirs = ((taxonomy as any).subsetChildDirs as string[] | undefined) ?? [];
  const slugPattern = new RegExp((taxonomy as any).standardSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  const subsetDirPrefix = (taxonomy as any).subsetDirPrefix ?? "✳️";
  const subsetAnyDirName = (taxonomy as any).subsetAnyDirName ?? "✳️any";
  const subsetSlugPattern = new RegExp((taxonomy as any).subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    if (!slugPattern.test(dialect.standardSlug)) {
      breaches.push({
        id: `standards-slug-${dialect.standardRel}`,
        summary: `"${dialect.standardRel}" standard slug "${dialect.standardSlug}" does not match ${slugPattern}`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Standard directory slugs are normalized lowercase identifiers (🔖️2.0, 🔖️ap214, 🔖️1, …).",
        solution: `Rename ${dialect.standardRel} to match the slug pattern.`,
      });
    }
    if (!dialect.subsetDirName.startsWith(subsetDirPrefix)) {
      breaches.push({
        id: `standards-subset-prefix-${dialect.subsetRel}`,
        summary: `"${dialect.subsetRel}" subset dir "${dialect.subsetDirName}" does not start with taxonomy's subsetDirPrefix "${subsetDirPrefix}"`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every subset dir under 🪆️subsets/ must be emoji-prefixed with subsetDirPrefix, symmetric with standards' 🔖️ prefix.",
        solution: `Rename ${dialect.subsetRel} to start with "${subsetDirPrefix}".`,
      });
    } else if (dialect.subsetDirName !== subsetAnyDirName && !subsetSlugPattern.test(dialect.subsetId)) {
      breaches.push({
        id: `standards-subset-slug-${dialect.subsetRel}`,
        summary: `"${dialect.subsetRel}" subset id "${dialect.subsetId}" does not match ${subsetSlugPattern}`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Real subset ids are normalized lowercase identifiers naming an industry conformance profile/class/view or semantic type (✳️a, ✳️cc6, ✳️rv, ✳️brep, ✳️mesh, …), never a version or a conformance level.",
        solution: `Rename ${dialect.subsetRel} to match the slug pattern.`,
      });
    }
    for (const child of standardChildDirs) {
      if (child === POLICY_SUBSETS_DIR) continue;
      if (existsSync(join(repoRoot, `${dialect.standardRel}/${child}`))) continue;
      breaches.push({
        id: `standards-standard-child-${dialect.standardRel}-${child}`,
        summary: `"${dialect.standardRel}" is missing required child ${child}/`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every standard carries engine/subsets per 🔣️taxonomy.json standardChildDirs.",
        solution: `Add ${dialect.standardRel}/${child}/.`,
      });
    }
    for (const child of subsetChildDirs) {
      if (existsSync(join(repoRoot, `${dialect.subsetRel}/${child}`))) continue;
      breaches.push({
        id: `standards-subset-child-${dialect.subsetRel}-${child}`,
        summary: `"${dialect.subsetRel}" is missing required child ${child}/`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every subset carries schema/io; lifecycle capabilities are derived from their hooks.",
        solution: `Add ${dialect.subsetRel}/${child}/.`,
      });
    }
  }
  return breaches;
}

function policyDerivedArtifactFacetBreaches(repoRoot: string, facet: (typeof POLICY_DERIVED_FACETS)[number]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const checkedOwners = new Set<string>();
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    for (const owner of [dialect.artRel, dialect.standardRel, dialect.subsetRel]) {
      if (checkedOwners.has(owner)) continue;
      checkedOwners.add(owner);
      const explicitRel = `${owner}/${facet.dir}`;
      if (!existsSync(join(repoRoot, explicitRel))) continue;
      breaches.push({
        id: `artifact-derived-${facet.name}-explicit-${owner}`,
        summary: `"${owner}" has forbidden explicit ${facet.name} facet ${facet.dir}/`,
        kind: "stdio-artifacts/derived-facets",
        scope: dialect.artRel,
        priority: "high",
        reason: "Builder, analyzer, and composer are derived types, not artifact taxonomy nodes.",
        solution: `Remove ${explicitRel}/ and keep the ${facet.name} hook in schema or IO.`,
      });
    }
    const schemaRel = `${dialect.subsetRel}/🧬️schema/${POLICY_RS_COMPONENT_LEAF}`;
    const schemaAbs = join(repoRoot, schemaRel);
    const body = existsSync(schemaAbs) ? readFileSync(schemaAbs, "utf8") : "";
    if (body.includes("derive_artifact_facets!") && body.includes(facet.hook)) continue;
    breaches.push({
      id: `artifact-derived-${facet.name}-missing-${dialect.subsetRel}`,
      summary: `"${schemaRel}" does not derive its ${facet.name}`,
      kind: "stdio-artifacts/derived-facets",
      scope: dialect.artRel,
      priority: "high",
      reason: "Every subset schema derives the uniform lifecycle types from construction, analysis, and composition hooks.",
      solution: `Invoke derive_artifact_facets! in ${schemaRel} with a ${facet.hook} hook.`,
    });
  }
  return breaches;
}

/** ⚖️Every migrated subset derives its builder from schema-owned construction. */
export function policyArtifactBuilderMigratedBreaches(repoRoot: string): BreachRecord[] {
  return policyDerivedArtifactFacetBreaches(repoRoot, POLICY_DERIVED_FACETS[0]);
}

/** ⚖️Every migrated subset derives its analyzer from schema-owned analysis. */
export function policyArtifactAnalyzerBreaches(repoRoot: string): BreachRecord[] {
  return policyDerivedArtifactFacetBreaches(repoRoot, POLICY_DERIVED_FACETS[1]);
}

/** ⚖️Every migrated subset derives its composer from IO-owned composition. */
export function policyArtifactComposerBreaches(repoRoot: string): BreachRecord[] {
  return policyDerivedArtifactFacetBreaches(repoRoot, POLICY_DERIVED_FACETS[2]);
}

/** 🔣 One row per (migrated artifact, standard) — the manifest a `policyStandardSubsetVocabularyBreaches` group checks. */
type PolicyStandardManifestGroup = {
  artRel: string;
  standardRel: string;
  standardSlug: string;
  manifestRel: string;
  dialects: PolicyArtifactDialect[];
};

function policyGroupDialectsByStandard(repoRoot: string): PolicyStandardManifestGroup[] {
  const taxonomy = loadTaxonomy();
  const subsetsDirName = (taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR;
  const manifestFilename = (taxonomy as any).subsetsManifestFilename ?? "🔣️component.json";
  const groups = new Map<string, PolicyStandardManifestGroup>();
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    let group = groups.get(dialect.standardRel);
    if (!group) {
      group = {
        artRel: dialect.artRel,
        standardRel: dialect.standardRel,
        standardSlug: dialect.standardSlug,
        manifestRel: `${dialect.standardRel}/${subsetsDirName}/${manifestFilename}`,
        dialects: [],
      };
      groups.set(dialect.standardRel, group);
    }
    group.dialects.push(dialect);
  }
  return [...groups.values()];
}

/**
 * ⚖️ Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: every migrated standard declares
 * its real industry subset vocabulary (or, for non-stdio/"domain" artifacts, explicitly declares
 * none) in `🪆️subsets/🔣️component.json`, and that declaration must always equal what's actually on
 * disk — in both directions, so a manifest can never silently drift ahead of or behind the real
 * dirs. Real (non-`*`) subsets on stdio artifacts additionally need a registered `SubsetValidator`
 * on their subset composer — the static half of "every real subset gets a real validator"; the
 * generic runtime half (`io.subset.validator-missing`) is `run_subset_validation` in
 * `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`.
 */
export function policyStandardSubsetVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const anyId = (taxonomy as any).subsetAnyId ?? "*";
  const anyDirName = (taxonomy as any).subsetAnyDirName ?? "✳️any";
  const subsetDirPrefix = (taxonomy as any).subsetDirPrefix ?? "✳️";
  const subsetSlugPattern = new RegExp((taxonomy as any).subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  const stdioPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const group of policyGroupDialectsByStandard(repoRoot)) {
    const isStdio = group.artRel.startsWith(stdioPrefix);
    const manifestAbs = join(repoRoot, group.manifestRel);
    if (!existsSync(manifestAbs)) {
      breaches.push({
        id: `standards-subset-vocabulary-missing-${group.standardRel}`,
        summary: `"${group.standardRel}" has no ${group.manifestRel.split("/").pop()} subset vocabulary manifest`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Every standard declares its subset vocabulary in 🪆️subsets/🔣️component.json — the single source of truth policy checks on-disk dirs against.",
        solution: `Create ${group.manifestRel} declaring {"${anyId}": {...}${isStdio ? ", plus each real subset this standard actually has" : ""}}.`,
      });
      continue;
    }
    let parsed: { artifact?: unknown; standard?: unknown; subsets?: Record<string, unknown> } | undefined;
    try {
      parsed = JSON.parse(readFileSync(manifestAbs, "utf8"));
    } catch (e) {
      breaches.push({
        id: `standards-subset-vocabulary-invalid-${group.standardRel}`,
        summary: `"${group.manifestRel}" is not valid JSON (${(e as Error).message})`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "The subset vocabulary manifest must parse as JSON.",
        solution: `Fix ${group.manifestRel}.`,
      });
      continue;
    }
    const declaredIds = Object.keys(parsed?.subsets ?? {});
    if (parsed?.standard !== group.standardSlug) {
      breaches.push({
        id: `standards-subset-vocabulary-standard-mismatch-${group.standardRel}`,
        summary: `"${group.manifestRel}" declares standard "${String(parsed?.standard)}" but lives under 🔖️${group.standardSlug}/`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "The manifest's own standard field must match the 🔖️<slug> directory it lives under.",
        solution: `Fix "standard" in ${group.manifestRel} to "${group.standardSlug}".`,
      });
    }
    if (!declaredIds.includes(anyId)) {
      breaches.push({
        id: `standards-subset-vocabulary-missing-any-${group.standardRel}`,
        summary: `"${group.manifestRel}" does not declare the base subset "${anyId}"`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Every standard carries the unconstrained base subset; it must always be declared.",
        solution: `Add "${anyId}" to the "subsets" object in ${group.manifestRel}.`,
      });
    }
    for (const id of declaredIds) {
      if (id === anyId || subsetSlugPattern.test(id)) continue;
      breaches.push({
        id: `standards-subset-vocabulary-bad-id-${group.standardRel}-${id}`,
        summary: `"${group.manifestRel}" declares subset id "${id}" which does not match ${subsetSlugPattern}`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Declared subset ids must be normalized lowercase identifiers.",
        solution: `Fix or remove "${id}" in ${group.manifestRel}.`,
      });
    }
    if (!isStdio) {
      const extra = declaredIds.filter((id) => id !== anyId);
      if (extra.length > 0) {
        breaches.push({
          id: `standards-subset-vocabulary-domain-real-subset-${group.standardRel}`,
          summary: `"${group.manifestRel}" declares real subset(s) [${extra.join(", ")}] on a non-stdio (domain) artifact`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "Domain (non-stdio) artifacts stay at v1 with the \"*\" subset only — real industry subset vocabularies are a stdio-only concept.",
          solution: `Remove [${extra.join(", ")}] from ${group.manifestRel}, or move this artifact under stdio if it genuinely has industry subsets.`,
        });
      }
    }
    const declaredDirs = new Set(declaredIds.map((id) => (id === anyId ? anyDirName : `${subsetDirPrefix}${id}`)));
    const actualDirs = new Set(group.dialects.map((d) => d.subsetDirName));
    for (const dir of actualDirs) {
      if (!declaredDirs.has(dir)) {
        breaches.push({
          id: `standards-subset-vocabulary-undeclared-dir-${group.standardRel}-${dir}`,
          summary: `"${group.standardRel}/${(taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR}/${dir}" exists on disk but is not declared in ${group.manifestRel}`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "On-disk subset dirs and the manifest's declared vocabulary must be exactly equal, in both directions.",
          solution: `Add an entry for this subset to ${group.manifestRel}.`,
        });
      }
    }
    for (const dir of declaredDirs) {
      if (!actualDirs.has(dir)) {
        breaches.push({
          id: `standards-subset-vocabulary-missing-dir-${group.standardRel}-${dir}`,
          summary: `${group.manifestRel} declares a subset whose dir "${dir}" does not exist under ${group.standardRel}/${(taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR}/`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "On-disk subset dirs and the manifest's declared vocabulary must be exactly equal, in both directions.",
          solution: `Create the ${dir}/ subset dir, or remove its entry from ${group.manifestRel} if it was declared ahead of the real implementation.`,
        });
      }
    }
    if (isStdio) {
      for (const dialect of group.dialects) {
        if (dialect.subsetId === anyId) continue;
        const composerRel = `${dialect.subsetRel}/🚪️io/${POLICY_RS_COMPONENT_LEAF}`;
        const composerAbs = join(repoRoot, composerRel);
        const body = existsSync(composerAbs) ? readFileSync(composerAbs, "utf8") : "";
        const hasValidatorImpl = policyRustFileHasRealTraitImpl(body, "SubsetValidator");
        const hasRegisterCall = /register_subset_validator\s*\(/.test(policyStripRustCommentsAndStrings(body));
        if (!hasValidatorImpl || !hasRegisterCall) {
          breaches.push({
            id: `standards-subset-vocabulary-validator-missing-${dialect.subsetRel}`,
            summary: `"${dialect.subsetRel}" is a real subset but its IO hook does not ${!hasValidatorImpl ? "implement SubsetValidator" : "call register_subset_validator"}`,
            kind: "stdio-artifacts/standards-subset-vocabulary",
            scope: dialect.artRel,
            priority: "high",
            reason: "Every real (non-\"*\") stdio subset registers a real SubsetValidator (io.subset.validator-missing otherwise fires at runtime for every compose of this dialect) — see the PDF/A pilot.",
            solution: `Add \`impl SubsetValidator for ...\` and a \`register_subset_validator(...)\` call to ${composerRel}.`,
          });
        }
      }
    }
  }
  return breaches;
}
//#endregion 🏅️PolicyRuleStandardsSubsets

function policySchemaRepresentationLeavesFor(repDir: string): readonly string[] {
  if (repDir === POLICY_STDIO_FACET_TEXT) return POLICY_STDIO_TEXT_SPEC_LEAVES;
  if (repDir === POLICY_STDIO_FACET_BINARY) return POLICY_STDIO_BINARY_SPEC_LEAVES;
  return [];
}

function policySchemaFormatLeafBreaches(
  repoRoot: string,
  facetAbs: string,
  artRel: string,
  taxonomy: ReturnType<typeof loadTaxonomy>,
): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const [formatId, format] of schemaFacetFormatEntries(repoRoot, facetAbs, taxonomy)) {
    const leafRel = `${facetAbs}/${format.leafFilename}`;
    if (existsSync(join(repoRoot, leafRel))) continue;
    breaches.push({
      id: `stdio-schema-format-${leafRel}`,
      summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
      kind: "stdio-artifacts/schema-representation",
      scope: artRel,
      priority: "high",
      reason: "Each schema node carries all schemaFormats leaves for its facet kind from 🔣️taxonomy.json.",
      solution: `Add ${leafRel}.`,
    });
  }
  return breaches;
}

/** 🏅 One (scope, schema-root) pair per artifact: the flat artifact-root root for an unmigrated
 * artifact, or one subset-relative root per (standard, subset) dialect for a migrated one. */
function policyArtifactSchemaRoots(repoRoot: string, artRel: string, schemaFacet: string): { scope: string; schemaRoot: string }[] {
  if (!policyArtifactIsMigrated(repoRoot, artRel)) {
    return [{ scope: artRel, schemaRoot: `${artRel}/${schemaFacet}` }];
  }
  return policyListArtifactDialectDirs(repoRoot)
    .filter((d) => d.artRel === artRel)
    .map((d) => ({ scope: artRel, schemaRoot: `${d.subsetRel}/${schemaFacet}` }));
}

/** 🔎️True when `schemaRoot` is exactly the delegating re-export pair — `🦀️component.rs` (`pub use
 * …::any::schema::*;`) + `🟦️component.ts` (either a literal `export * from ".../✳️any/.../🟦️component"`
 * re-export, or a `meta` stamp with no own `interface`/`type`/`enum` — both shapes are attested on
 * disk, ifc ✳️cv20/✳️sav/✳️cobie use the former, pdf/step/zip's conformance subsets use the latter)
 * and NOTHING else. A subset in this shape is a validation-gated conformance STAMP on top of its
 * standard's schema-owning subset — it never duplicates the schema, so it never carries facet
 * mirrors/grammar leaves/diff/mutations trees. Structural, not name-based: a delegating subset
 * never needs an allowlist/filter edit. */
function policySchemaIsDelegatingPair(repoRoot: string, schemaRoot: string): boolean {
  const entries = policyReaddirSafe(repoRoot, schemaRoot);
  if (entries.length !== 2) return false;
  if (entries.some((e) => e.isDirectory)) return false;
  const names = new Set(entries.map((e) => e.name));
  if (!names.has(POLICY_RS_COMPONENT_LEAF) || !names.has(POLICY_TS_COMPONENT_LEAF)) return false;
  const rs = policyReadFileSafe(repoRoot, schemaRoot, POLICY_RS_COMPONENT_LEAF);
  const ts = policyReadFileSafe(repoRoot, schemaRoot, POLICY_TS_COMPONENT_LEAF);
  const rsReexports = /pub\s+use\s+[\w:]+::any::schema::\*\s*;/.test(policyStripRustCommentsAndStrings(rs));
  const tsHasOwnType = /export\s+(interface|type|enum)\s+\w+/.test(ts);
  if (tsHasOwnType) return false;
  const tsLiteralReexport = /export\s+\*\s+from\s+["'][^"']*✳️any\/🧬️schema\/🟦️component["']\s*;/.test(ts);
  const tsHasMetaStamp = /export\s+const\s+meta\s*=\s*\{[^}]*artifactKind\s*:[^}]*\}/.test(ts);
  return rsReexports && (tsLiteralReexport || tsHasMetaStamp);
}

/** 🏅 True when `schemaRoot` owns its schema (has a real `📸️snapshot/` on disk) — the generalized
 * form of `policyListStdioSchemaOwningEntries`'s filter, inlined here because this rule also covers
 * unmigrated (pre-dialect) artifacts, which are always schema-owning by construction (no subset
 * concept to delegate from). See `policySchemaIsDelegatingPair` for the sibling shape. */
function policySchemaRootIsOwning(repoRoot: string, schemaRoot: string, migrated: boolean): boolean {
  if (!migrated) return true;
  return existsSync(join(repoRoot, schemaRoot, "📸️snapshot"));
}

/** ⚖️Schema tree under 🧬️schema with representation text/binary spec leaves (ticket STDIO-ARTIFACTS-AND-IO).
 * Migrated artifacts are checked at their 🏅️standards/🔖️.../🪆️subsets/✳️.../🧬️schema location instead —
 * and, per ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1, only a
 * SCHEMA-OWNING subset (has its own `📸️snapshot/`) is held to the full tree; a DELEGATING subset (see
 * `policySchemaIsDelegatingPair`) needs only the rs+ts re-export pair. */
export function policySchemaRepresentationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const schemaChildDirs = policyStdioSchemaChildDirs(taxonomy);
  const representationDirs = policyStdioRepresentationDirs(taxonomy);
  const schemaFacet = "🧬️schema";
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
   const migrated = policyArtifactIsMigrated(repoRoot, artRel);
   for (const { schemaRoot } of policyArtifactSchemaRoots(repoRoot, artRel, schemaFacet)) {
    if (!existsSync(join(repoRoot, schemaRoot))) {
      breaches.push({
        id: `stdio-schema-root-${artRel}`,
        summary: `"${artRel}" is missing ${schemaFacet}/`,
        kind: "stdio-artifacts/schema-representation",
        scope: artRel,
        priority: "high",
        reason: "Artifact-level schema facet is required; snapshot/diff/mutations nest beneath it.",
        solution: `Create ${schemaRoot}/ per normative spec §1.`,
      });
      continue;
    }
    if (!policySchemaRootIsOwning(repoRoot, schemaRoot, migrated)) {
      if (policySchemaIsDelegatingPair(repoRoot, schemaRoot)) continue;
      breaches.push({
        id: `stdio-schema-delegating-${schemaRoot}`,
        summary: `"${schemaRoot}" is neither schema-owning (no 📸️snapshot/) nor a valid delegating re-export pair`,
        kind: "stdio-artifacts/schema-representation",
        scope: artRel,
        priority: "high",
        reason: "A delegating subset (validation-gated conformance stamp on an existing schema-owning subset, e.g. step ✳️cc1, pdf ✳️a, zip ✳️iso21320) carries ONLY 🦀️component.rs (`pub use …::any::schema::*;`) + 🟦️component.ts (a meta stamp) — no facet mirrors, grammar leaves, or diff/mutations trees.",
        solution: `Replace ${schemaRoot}'s contents with exactly ${POLICY_RS_COMPONENT_LEAF} (re-exporting the standard's schema-owning subset) + ${POLICY_TS_COMPONENT_LEAF} (a meta stamp), or add its own 📸️snapshot/ to make it schema-owning and satisfy the full tree below instead.`,
      });
      continue;
    }
    breaches.push(...policySchemaFormatLeafBreaches(repoRoot, schemaRoot, artRel, taxonomy));
    for (const child of schemaChildDirs) {
      const childAbs = `${schemaRoot}/${child}`;
      if (!existsSync(join(repoRoot, childAbs))) {
        breaches.push({
          id: `stdio-schema-child-${childAbs}`,
          summary: `"${schemaRoot}" is missing child ${child}/`,
          kind: "stdio-artifacts/schema-representation",
          scope: artRel,
          priority: "high",
          reason: `taxonomy.schemaChildDirs requires ${child} under every 🧬️schema facet.`,
          solution: `Add ${childAbs}/ with representation dirs and schemaFormats leaves.`,
        });
        continue;
      }
      breaches.push(...policySchemaFormatLeafBreaches(repoRoot, childAbs, artRel, taxonomy));
      for (const rep of representationDirs) {
        const repAbs = `${childAbs}/${rep}`;
        if (!existsSync(join(repoRoot, repAbs))) {
          breaches.push({
            id: `stdio-schema-rep-${repAbs}`,
            summary: `"${childAbs}" is missing representation ${rep}/`,
            kind: "stdio-artifacts/schema-representation",
            scope: artRel,
            priority: "high",
            reason: `Each schema child carries ${POLICY_STDIO_FACET_TEXT} and ${POLICY_STDIO_FACET_BINARY} spec trees.`,
            solution: `Add ${repAbs}/ with all normative spec leaves.`,
          });
          continue;
        }
        for (const leaf of policySchemaRepresentationLeavesFor(rep)) {
          const leafRel = `${repAbs}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({
            id: `stdio-schema-leaf-${leafRel}`,
            summary: `"${repAbs}" is missing spec leaf ${leaf}`,
            kind: "stdio-artifacts/schema-representation",
            scope: artRel,
            priority: "high",
            reason: "Text and binary representation nodes own fixed handcrafted spec filenames.",
            solution: `Add ${leafRel}.`,
          });
        }
      }
    }
   }
  }
  return breaches;
}

/** 🔎️True if any `🦀️component.rs` exists anywhere under `rootRel` (any depth) — used to check a
 * migrated io leaf exists without having to guess which of a stdio format's own (standard,
 * subset) pairs a given domain artifact happens to bridge to. */
function policyHasComponentUnder(repoRoot: string, rootRel: string): boolean {
  const abs = join(repoRoot, rootRel);
  if (!existsSync(abs)) return false;
  const stack = [rootRel];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    for (const entry of policyReaddirSafe(repoRoot, dir)) {
      if (entry.isDirectory) {
        stack.push(`${dir}/${entry.name}`);
      } else if (entry.name === POLICY_RS_COMPONENT_LEAF) {
        return true;
      }
    }
  }
  return false;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 3: the
 * REAL replacement for `policyIoSerializerMatrixBreaches`'s dead early-continue (see that
 * function's own comment — every owner is migrated now, so it has been a permanent no-op since
 * this session's Phase 1 completed the migration; confirmed via zero `stdio-artifacts/io-matrix`
 * breaches anywhere in a real `bun ./📜️script.ts policy` run). This checks the MIGRATED shape
 * instead: for each catalog owner's curated import/export format list, at least one of that domain
 * artifact's own (standard, subset) dirs must carry a real io leaf for that format — searched
 * existence-anywhere-under its `🚪️io/<direction>/<facet>/🗿️artifacts/<format-dir>/` tree (not a
 * specific (format-standard, format-subset) pair, since a domain artifact may reasonably bridge to
 * any one of a format's now-multiple standards, e.g. pdf 1.4 vs 1.7 — picking one to require would
 * be guessing, and this rule shouldn't invent a policy the codebase hasn't actually adopted yet).
 */
export function policyIoMatrixMigratedBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const artifactsDir = policyStdioArtifactsDirName();
  const dialectsByArt = new Map<string, PolicyArtifactDialect[]>();
  for (const d of policyListArtifactDialectDirs(repoRoot)) {
    if (!dialectsByArt.has(d.artRel)) dialectsByArt.set(d.artRel, []);
    dialectsByArt.get(d.artRel)!.push(d);
  }
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    const dialects = dialectsByArt.get(scope);
    if (!dialects || dialects.length === 0) continue; // not migrated (yet) -- the legacy flat-path rule below still owns it
    const checkDirection = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) continue; // unknown-format-id is already reported by the legacy rule for unmigrated owners; a migrated owner referencing an unknown id is vanishingly unlikely and not worth a second breach shape here
        const found = dialects.some((d) => policyHasComponentUnder(repoRoot, `${d.subsetRel}/🚪️io/${direction}/${childFacet}/${artifactsDir}/${formatDir}`));
        if (!found) {
          breaches.push({
            id: `io-matrix-migrated-${scope}-${direction}-${formatId}`,
            summary: `"${scope}" has no migrated ${label} leaf for stdio format "${formatId}" in any of its own (standard, subset) dirs`,
            kind: "artifact-io/io-matrix-migrated",
            scope,
            priority: "high",
            reason: `The catalog's curated ${label} list for this artifact names "${formatId}", but no 🚪️io/${direction}/${childFacet}/${artifactsDir}/${formatDir}/ leaf exists under any of this artifact's migrated standard/subset dirs.`,
            solution: `Add a ${direction}/${childFacet} leaf for ${formatId} under one of "${scope}"'s 🏅️standards/🔖️.../🪆️subsets/✳️.../🚪️io/ dirs, or remove "${formatId}" from the ${label} list in ${POLICY_STDIO_OWNER_TABLE_REL} if it's no longer curated.`,
          });
        }
      }
    };
    checkDirection(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkDirection(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** ⚖️Curated import/export deserializer and serializer matrix from 🧪owner-table.json. */
export function policyIoSerializerMatrixBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const artifactsDir = policyStdioArtifactsDirName();
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    if (policyArtifactIsMigrated(repoRoot, scope)) {
      // Migrated owner: its io leaves moved under 🏅️standards/.../🚪️io/ with target-qualified
      // (standard, subset) leaf paths -- a dedicated migrated io-matrix rule lands with the
      // domain fan-out (W14); until then this flat-path rule simply doesn't apply to it.
      continue;
    }
    const ioRoot = `${scope}/🚪️io`;
    if (!existsSync(join(repoRoot, ioRoot))) {
      breaches.push({
        id: `stdio-io-matrix-io-${scope}`,
        summary: `"${scope}" is missing 🚪️io/`,
        kind: "stdio-artifacts/io-matrix",
        scope,
        priority: "high",
        reason: "Curated stdio pairs are wired through the io facet deserializer/serializer tree.",
        solution: `Create ${ioRoot}/ with import/deserializers and export/serializers.`,
      });
      continue;
    }
    const checkLeaves = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) {
          breaches.push({
            id: `stdio-io-matrix-unknown-${scope}-${formatId}`,
            summary: `unknown stdio format id "${formatId}" on ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Matrix format ids must exist in the schema-owned definition set.",
            solution: `Fix ${label} list for ${scope} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
          });
          continue;
        }
        const leafBase = `${ioRoot}/${direction}/${childFacet}/${artifactsDir}/${formatDir}`;
        for (const leaf of [POLICY_RS_COMPONENT_LEAF, POLICY_TS_COMPONENT_LEAF] as const) {
          const leafRel = `${leafBase}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({
            id: `stdio-io-matrix-${leafRel}`,
            summary: `missing ${label} ${leaf} for ${formatId} under ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Each curated pair needs both Rust and TypeScript codec leaves under 🗿️artifacts/<stdio-dir>/.",
            solution: `Add ${leafRel}.`,
          });
        }
      }
    };
    checkLeaves(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkLeaves(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** ⚖️Stdio dependency DAG is acyclic and every format eventually depends on binary. */
export function policyIoTerminalityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const nodes = new Set(Object.keys(roster));
  const adj = new Map<string, Set<string>>();
  for (const id of nodes) adj.set(id, new Set(roster[id]?.depends ?? []));
  for (const edge of table.dependency_edges) {
    if (!nodes.has(edge.from) || !nodes.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-unknown-${edge.from}-${edge.to}`,
        summary: `schema-derived dependency edge references unknown node (${edge.from} → ${edge.to})`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "Dependency edges must only connect schema-defined format ids.",
        solution: `Align schema dependencies in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
      continue;
    }
    adj.get(edge.from)?.add(edge.to);
    const rosterDeps = new Set(roster[edge.from]?.depends ?? []);
    if (!rosterDeps.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-definition-edge-${edge.from}-${edge.to}`,
        summary: `schema-derived dependency edge ${edge.from}→${edge.to} is absent from its definition`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "Derived dependency edges must mirror definition dependencies.",
        solution: `Fix dependencies for ${edge.from} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
    }
  }
  const visitedGlobal = new Set<string>();
  const findCycle = (start: string): string[] | null => {
    const stack: string[] = [];
    const onStack = new Set<string>();
    const dfs = (n: string): string[] | null => {
      if (onStack.has(n)) return [...stack.slice(stack.indexOf(n)), n];
      if (visitedGlobal.has(n)) return null;
      onStack.add(n);
      stack.push(n);
      for (const dep of adj.get(n) ?? []) {
        const cyc = dfs(dep);
        if (cyc) return cyc;
      }
      stack.pop();
      onStack.delete(n);
      visitedGlobal.add(n);
      return null;
    };
    return dfs(start);
  };
  for (const n of nodes) {
    if (visitedGlobal.has(n)) continue;
    const cyc = findCycle(n);
    if (!cyc) continue;
    breaches.push({
      id: `stdio-dag-cycle-${cyc.join("-")}`,
      summary: `stdio DAG cycle: ${cyc.join(" → ")}`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codec dependencies must form a DAG.",
      solution: "Remove cyclic schema definition dependencies.",
    });
    break;
  }
  const memo = new Map<string, boolean>();
  const reachesBinary = (n: string, trail: Set<string>): boolean => {
    if (n === "binary") return true;
    if (memo.has(n)) return memo.get(n)!;
    if (trail.has(n)) return false;
    trail.add(n);
    const deps = adj.get(n);
    if (!deps || deps.size === 0) {
      memo.set(n, false);
      return false;
    }
    let ok = true;
    for (const d of deps) ok = ok && reachesBinary(d, trail);
    memo.set(n, ok);
    return ok;
  };
  for (const n of nodes) {
    if (n === "binary") continue;
    if (reachesBinary(n, new Set())) continue;
    breaches.push({
      id: `stdio-dag-term-${n}`,
      summary: `stdio format "${n}" does not terminate at binary via depends`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Every stdio artifact dependency chain must eventually reach the binary root.",
      solution: `Fix dependencies for "${n}" until binary is reachable.`,
    });
  }
  return breaches;
}

/** ⚖️Banned stub codec markers (SRAS, IFCCARTOONMESH, minimal stubs) must not remain in Rust sources. */
export function policyCodecFidelityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const scanRoots = [
    "🧰️framework/🔨️modules/🔺️mesh",
    "🧰️framework/🛍️products/💻️os",
    "🧰️framework/🛍️products/💻️os/🖥️host",
    "✏️s/🔌️plugins",
  ];
  const rsFiles = policyWalkRelFiles(repoRoot, scanRoots, (_p, name) => name.endsWith(".rs"));
  for (const rel of rsFiles) {
    const body = policyReadFileSafe(repoRoot, rel);
    for (const marker of POLICY_STDIO_CODEC_BANNED_MARKERS) {
      if (!body.includes(marker)) continue;
      breaches.push({
        id: `stdio-codec-ban-${rel}-${marker}`,
        summary: `"${rel}" contains banned stub marker ${JSON.stringify(marker)}`,
        kind: "stdio-artifacts/codec-fidelity",
        scope: rel,
        priority: "high",
        reason: "Framework and plugin codecs must be real round-trip implementations, not SRAS/IFCCARTOONMESH/minimal stubs.",
        solution: `Replace the stub in ${rel} with a stdio-owned codec or delete the dead path.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 2: a
 * SELF-referential `Dialect { artifact_kind: "s.stdio.<X>", standard: StandardId("<S>"), subset:
 * SubsetId("<U>") }` literal (one whose `artifact_kind` names the SAME stdio artifact the file
 * lives under -- e.g. `🎞️gif`'s own `DIALECT`/`WRITES` const, never a cross-artifact `DEP_*`/`READS`
 * dependency literal naming some OTHER artifact) must agree with the `🏅️standards/🔖️<S>/…
 * 🪆️subsets/✳️<U>/` directory the file actually lives under. Cross-artifact references are skipped
 * entirely -- they correctly point at a directory this file doesn't live inside, so there's nothing
 * to verify from here. Subset checking honors the `SubsetId::ANY = SubsetId("*")` sentinel
 * convention (the literal is `"*"`, the directory is named `✳️any`) rather than naive string
 * equality -- moot today (every subset is `any`), meaningful once D5's PDF/A-2b pilot lands.
 */
// 🐛 Scoped to `const DIALECT`/`const WRITES` specifically (the two canonical self-identity const
// names used throughout this codebase's composers) — earlier versions matched ANY `Dialect{...}`
// literal in the file, which also caught legitimate same-artifact-different-subset dependency
// consts (e.g. a subset composer's own `DIALECT_ANY` fallback reference into its standard's `any`
// subset) and flagged them as if they claimed self-identity. Indirection (`const WRITES: Dialect =
// SOME_OTHER_CONST;`) is a false negative here (nothing to check), which is the safe direction —
// never a false positive.
const POLICY_DIALECT_LITERAL_RE = /const\s+(?:DIALECT|WRITES)\s*:\s*Dialect\s*=\s*Dialect\s*\{\s*artifact_kind:\s*"([^"]+)"\s*,\s*standard:\s*StandardId\("([^"]+)"\)\s*,\s*subset:\s*SubsetId\("([^"]+)"\)\s*\}/g;

function policyDialectLiteralPathBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches; // owner-table-missing is already flagged by policyStdioCatalogBreaches
  const taxonomy = loadTaxonomy();
  const subsetAnyId = (taxonomy as any).subsetAnyId ?? "*";
  const subsetAnyDirName = (taxonomy as any).subsetAnyDirName ?? "✳️any";
  const subsetDirPrefix = (taxonomy as any).subsetDirPrefix ?? "✳️";
  const dirToKey = new Map<string, string>();
  for (const [key, entry] of Object.entries(table.artifacts)) {
    dirToKey.set(entry.dir, key);
  }
  const artifactsPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const cut = relPath.indexOf(artifactsPrefix);
    if (cut === -1) continue;
    const afterArtifacts = relPath.slice(cut + artifactsPrefix.length);
    const ownDir = afterArtifacts.split("/")[0];
    const ownKey = ownDir ? dirToKey.get(ownDir) : undefined;
    if (!ownKey) continue;
    const ownArtifactKind = `s.stdio.${ownKey}`;
    const standardMatch = relPath.match(/🏅️standards\/🔖️([^/]+)\//);
    const subsetMatch = relPath.match(/🪆️subsets\/✳️([^/]+)\//);
    const content = policyReadFileSafe(repoRoot, relPath);
    POLICY_DIALECT_LITERAL_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_DIALECT_LITERAL_RE.exec(content))) {
      const [, litKind, litStandard, litSubset] = m!;
      if (litKind !== ownArtifactKind) continue;
      if (standardMatch && litStandard !== standardMatch[1]) {
        breaches.push({
          id: `dialect-literal-standard-${relPath}-${litStandard}`,
          summary: `"${relPath}" declares StandardId("${litStandard}") but lives under 🏅️standards/🔖️${standardMatch[1]}/`,
          kind: "artifact-io/dialect-literal-path",
          scope: relPath,
          priority: "high",
          reason: "A self-referential Dialect literal's standard must match the 🔖️<standard> directory the file physically lives under, or the two will silently drift apart the next time either one changes alone.",
          solution: `Fix StandardId("${litStandard}") in ${relPath} to "${standardMatch[1]}", or move the file to 🏅️standards/🔖️${litStandard}/ if that's the one that's wrong.`,
        });
      }
      const subsetDir = litSubset === subsetAnyId ? subsetAnyDirName.slice(subsetDirPrefix.length) : litSubset;
      if (subsetMatch && subsetDir !== subsetMatch[1]) {
        breaches.push({
          id: `dialect-literal-subset-${relPath}-${litSubset}`,
          summary: `"${relPath}" declares SubsetId("${litSubset}") but lives under 🪆️subsets/${subsetDirPrefix}${subsetMatch[1]}/`,
          kind: "artifact-io/dialect-literal-path",
          scope: relPath,
          priority: "high",
          reason: `A self-referential Dialect literal's subset must match the ${subsetDirPrefix}<subset> directory the file physically lives under (SubsetId("${subsetAnyId}") ⇔ ${subsetAnyDirName} by convention).`,
          solution: `Fix SubsetId("${litSubset}") in ${relPath} to match ${subsetDirPrefix}${subsetMatch[1]}/, or move the file if the directory is what's wrong.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, W1 Task 4
 * blocking finding: `store::register_document_codec` (🧰️framework/…/🏪️store/🦀️component.rs) is a
 * plain `HashMap::insert` keyed by the codec's schema id string — registering two DIFFERENT codecs
 * under the SAME id does not panic, it silently overwrites (last-registered-wins), with zero
 * runtime diagnostic (empirically confirmed by W1's own regression test). This is a real risk once
 * many parallel agents each mint their own schema-id constant (13 semio subsets + 7 new formats +
 * 28 pre-existing artifacts, all sharing one process-wide registry) — a typo'd copy-paste collision
 * would silently load the wrong codec at runtime with zero build or test signal. This rule
 * statically resolves every `register_document_codec(store::ArtifactCodec::of::<…>(<id-expr>))`
 * call site's id expression — a string literal, or a `const _: &str` referenced bare or fully
 * path-qualified (both forms are used across the crate) — across the WHOLE stdio crate, and flags
 * any resolved id string claimed by more than one call site. Mirrors `policyDialectLiteralPathBreaches`
 * above: same two-pass grep-then-cross-reference shape, same `policyAllRustFiles` file source.
 */
function policyStdioCodecIdUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginRoot = join(repoRoot, POLICY_STDIO_PLUGIN_REL);
  if (!existsSync(pluginRoot)) return breaches;
  const artifactsPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  const files = policyAllRustFiles(repoRoot).filter((relPath) => relPath.startsWith(artifactsPrefix));

  // Pass 1: crate-wide `const NAME: &str = "value";` map, keyed by the identifier's *last path
  // segment* — call sites reference a schema-id const either bare or fully `crate::…::`-qualified.
  const constValue = new Map<string, string>();
  const CONST_RE = /(?:pub\s+)?const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]*)"\s*;/g;
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    CONST_RE.lastIndex = 0;
    let cm: RegExpExecArray | null;
    while ((cm = CONST_RE.exec(content))) constValue.set(cm[1], cm[2]);
  }

  // Pass 2: every register_document_codec call site's id expression, resolved to a literal value.
  const CALL_RE = /register_document_codec\(\s*store::ArtifactCodec::of::<[\s\S]*?>\(\s*([\w:]+|"[^"]*")\s*\)\s*\)/g;
  const claims = new Map<string, { relPath: string; line: number }[]>();
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    CALL_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = CALL_RE.exec(content))) {
      const raw = m[1]!;
      const value = raw.startsWith('"') ? raw.slice(1, -1) : constValue.get(raw.split("::").pop()!);
      if (!value) continue; // unresolved id expression — not guessed at, silently skipped
      const line = content.slice(0, m.index).split("\n").length;
      const list = claims.get(value) ?? [];
      list.push({ relPath, line });
      claims.set(value, list);
    }
  }

  for (const [id, sites] of claims) {
    if (sites.length <= 1) continue;
    const locations = sites.map((s) => `${s.relPath}:${s.line}`).join(", ");
    for (const site of sites) {
      breaches.push({
        id: `stdio-codec-id-duplicate-${id}-${site.relPath}-${site.line}`,
        summary: `schema id "${id}" is claimed by ${sites.length} register_document_codec call sites: ${locations}`,
        kind: "stdio-artifacts/codec-id-uniqueness",
        scope: site.relPath,
        priority: "high",
        reason:
          "store::register_document_codec silently overwrites on a duplicate id (plain HashMap::insert, no panic) — two codecs sharing an id means the second-registered one silently and invisibly wins at runtime, with zero build or test signal.",
        solution: `Give each of ${locations} its own distinct schema id constant/value.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 8:
 * replaces the old `policyCodecFidelityBreaches` (a 5-string banned-marker grep — "does this file
 * avoid saying SRAS/IFCCARTOONMESH", a purely negative signal) with a positive one: every
 * standard-level `⚙️engine/🦀️component.rs` must contain a real decode→encode→decode (or
 * equivalent lossless) round-trip test. Detected via a deliberately generous name/body heuristic
 * (`round_trip`/`roundtrip`/`decode_encode`/`encode_decode`/`lossless`, case-insensitive) rather
 * than requiring one exact test name — every codec this session actually delivered uses one of
 * these, under varying names (`codec_round_trip`, `zip_deflate_round_trip`,
 * `real_decode_stays_lossless_on_reencode`, ...). Old rule NOT deleted yet (still catches literal
 * stub markers, a cheap orthogonal check) — the plan calls for deleting it once this one's
 * allowlist is fully burned down; premature while 43 files remain.
 */
const POLICY_ROUND_TRIP_TEST_ALLOWLIST = new Set<string>([
  // 🐛 Same lesson as the other two allowlists in this file: keys are policyNormalizeRelPath's
  // canonical short form, computed programmatically against a real repo scan, never hand-typed —
  // see the sniff-reality allowlist's own comment for the full story of what happens otherwise.
  "trinity/jack/standards#1-engine-component",
  "trinity/rewrite/standards#1-engine-component",
  "raster/standards#1-engine-component",
  "flow/standards#1-engine-component",
  "process/process3d/standards#1-engine-component",
  "norm/din4108/standards#1-engine-component",
  "norm/din18599/standards#1-engine-component",
  "norm/din16798/standards#1-engine-component",
  "norm/en1990/standards#1-engine-component",
  "norm/en1991/standards#1-engine-component",
  "norm/en1992/standards#1-engine-component",
  "norm/en1993/standards#1-engine-component",
  "norm/en1994/standards#1-engine-component",
  "norm/en1995/standards#1-engine-component",
  "norm/en1996/standards#1-engine-component",
  "norm/en1997/standards#1-engine-component",
  "norm/en1998/standards#1-engine-component",
  "norm/en1999/standards#1-engine-component",
  "cad/standards#1-engine-component",
  "demonstrator/playground/standards#1-engine-component",
  "block/2d/standards#1-engine-component",
  "block/3d/standards#1-engine-component",
  "block/5d/standards#1-engine-component",
  "dag/standards#1-engine-component",
  "stdio/pdf/standards#1.4-engine-component", // deliberately untouched; real vocabulary is on 1.7
  "reasoning/wires/standards#1-engine-component",
  "writer/standards#1-engine-component",
  "animate/present/standards#1-engine-component",
  "space/home/standards#1-engine-component",
  "procedural/procedural2d/standards#1-engine-component",
  "vcs/standards#1-engine-component",
  "gis/gismap/standards#1-engine-component",
  "gis/gisterrain/standards#1-engine-component",
  "note/standards#1-engine-component",
  "architect/program/standards#1-engine-component",
  "shooting/standards#1-engine-component",
  "puzzle/2d/standards#1-engine-component",
  "puzzle/3d/standards#1-engine-component",
  "puzzle/5d/standards#1-engine-component",
  "fem/2d/standards#1-engine-component",
  "fem/3d/standards#1-engine-component",
  "playbook/standards#1-engine-component",
  "energy/model/standards#1-engine-component",
  // seeded by W1b scaffold, burn down as W3 rewires each format's ArtifactPack off JSON-passthrough
  // onto its own engine's real encoder (see w1b-scaffold-manifest.md §6 — sniff/minimal-parse is
  // already real+tested for all 7, this is specifically the missing decode→encode→decode test) and
  // as W2 adds semio v1's own round-trip test alongside geometry/triples' existing 9. Keys computed
  // via policyNormalizeRelPath.
  // W3 closer removed epw/mp4/mp3/avi/wav — each `⚙️engine/🦀️component.rs` now has a real
  // `#[cfg(test)]` round-trip test (grep-verified, policy re-run green after). html stays: its
  // real codec_retention_law round-trip test lives in `📸️snapshot/🦀️component.rs`, not
  // `⚙️engine/🦀️component.rs` — this rule is scoped to the engine file specifically, so html's
  // `⚙️engine/🦀️component.rs` (just `sniff_real_bytes` + a small sniff test) genuinely still has
  // no round-trip signal under this rule's own detection method; a real architectural mismatch
  // between where html's parser lives and where this rule looks, not a fixed-and-forgotten entry
  // — documented here as a follow-up rather than moved, since relocating html's parser is a
  // design decision outside this closer's scope.
  "stdio/html/standards#5-engine-component",
  "stdio/semio/standards#v1-engine-component",
]);

const POLICY_ROUND_TRIP_SIGNAL_RE = /round_trip|roundtrip|decode_encode|encode_decode|lossless/i;

function policyRoundTripTestBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith("✏️s/🔌️plugins/")) continue;
    if (!relPath.endsWith(`⚙️engine/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!relPath.includes("🏅️standards/🔖️")) continue;
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_ROUND_TRIP_TEST_ALLOWLIST.has(normalized);
    const content = policyReadFileSafe(repoRoot, relPath);
    const hasSignal = content.includes("#[cfg(test)]") && POLICY_ROUND_TRIP_SIGNAL_RE.test(content);
    if (!hasSignal) {
      if (allowlisted) continue;
      breaches.push({
        id: `round-trip-test-${relPath}`,
        summary: `"${relPath}" has no real decode→encode→decode round-trip test`,
        kind: "artifact-io/round-trip-test",
        scope: relPath,
        priority: "high",
        reason: "Every standard-level engine must prove its codec is lossless with a real round-trip test, not just avoid a list of known-stub markers.",
        solution: `Add a #[test] round-trip (decode→encode→decode) test to ${relPath}, or if this standard hasn't been reached by the codec uplift yet, add "${normalized}" to POLICY_ROUND_TRIP_TEST_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `round-trip-test-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_ROUND_TRIP_TEST_ALLOWLIST but already has a round-trip test`,
        kind: "artifact-io/round-trip-test",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_ROUND_TRIP_TEST_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 6
 * (scoped): every derived composition hook declares its cross-artifact dependencies as `const DEP_<NAME>:
 * Dialect = Dialect { artifact_kind: "s.stdio.<dep>", standard: StandardId("<std>"), subset:
 * SubsetId(...) }` (confirmed the universal pattern across every stdio composer this session
 * touched). This rule verifies each declared dependency's (artifact, standard) pair actually
 * exists on disk — `🗿️artifacts/<dep-dir>/🏅️standards/🔖️<std>/` — catching a phantom dependency
 * (typo'd standard, or a standard that got renamed/deleted out from under a composer that still
 * references the old name, e.g. exactly the `ac1018`→`ac1024` kind of rename this session did
 * several times) at policy-check time instead of a runtime compose failure. This is the "no
 * phantoms" half of the plan's fuller "composer dialect consts ↔ io leaf dirs bijection" — the "no
 * orphans" half (an io leaf dir nobody's composer ever references) needs a second pass once rule 3
 * (io-matrix migrated-leaves) traces the legacy early-continue branch it currently shares
 * plumbing with; left for a future session rather than rushed.
 */
const POLICY_DEP_DIALECT_RE = /const\s+DEP_\w+\s*:\s*Dialect\s*=\s*Dialect\s*\{\s*artifact_kind:\s*"s\.stdio\.([^"]+)"\s*,\s*standard:\s*StandardId\("([^"]+)"\)/g;

function policyComposerDependencyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const dirByKey = new Map<string, string>();
  for (const [key, entry] of Object.entries(table.artifacts)) {
    dirByKey.set(key, entry.dir);
  }
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.endsWith(`🚪️io/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!relPath.startsWith(`${POLICY_STDIO_ARTIFACTS_REL}/`)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    POLICY_DEP_DIALECT_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_DEP_DIALECT_RE.exec(content))) {
      const [, depKey, depStandard] = m!;
      const depDir = dirByKey.get(depKey!);
      if (!depDir) {
        breaches.push({
          id: `composer-dep-unknown-artifact-${relPath}-${depKey}`,
          summary: `"${relPath}" depends on unknown stdio artifact "${depKey}" (not in the catalog roster)`,
          kind: "artifact-io/composer-dependency",
          scope: relPath,
          priority: "high",
          reason: "A composer's DEP_* dialect must name a real, currently-cataloged stdio artifact — an unknown one is either a typo or a stale reference to a deleted/renamed artifact.",
          solution: `Fix the artifact_kind in ${relPath}'s DEP_* constant, or add "${depKey}" to the catalog roster if it's genuinely new.`,
        });
        continue;
      }
      const depStandardDir = join(repoRoot, POLICY_STDIO_ARTIFACTS_REL, depDir, "🏅️standards", `🔖️${depStandard}`);
      if (!existsSync(depStandardDir)) {
        breaches.push({
          id: `composer-dep-unknown-standard-${relPath}-${depKey}-${depStandard}`,
          summary: `"${relPath}" depends on ${depKey}@${depStandard}, but no such standard directory exists`,
          kind: "artifact-io/composer-dependency",
          scope: relPath,
          priority: "high",
          reason: "A composer's DEP_* dialect must name a standard that actually exists on disk — a phantom reference (typo, or a stale reference to a renamed/deleted standard) fails at compose time instead of at policy-check time.",
          solution: `Fix the StandardId in ${relPath}'s DEP_* constant to match an existing 🏅️standards/🔖️<std>/ directory under ${depDir}, or restore that standard if it was deleted in error.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 5:
 * flagship artifacts (svg/gltf/pdf/gif — the ones D2 explicitly names as getting real mutation
 * vocabularies, not just `{NoMutation, SetSnapshot}`) must have a mutation enum with more than
 * those two variants. Shrink-only allowlist for the flagships this hasn't reached yet.
 */
const POLICY_FLAGSHIP_MUTATION_ALLOWLIST = new Set<string>([
  // 🐛 Keys are policyNormalizeRelPath's canonical <pluginId>/<component>#<tail> short form, NOT
  // the raw file path — a full-path entry silently never matches (found the hard way: this
  // allowlist, and the sibling POLICY_SNIFF_REALITY_ALLOWLIST, both shipped with raw-path keys for
  // most of a session, during which `bun ./📜️script.ts verify` (a DIFFERENT, narrower gate
  // pipeline than `bun ./📜️script.ts policy`) was mistakenly used to "confirm" zero breaches —
  // `policy` is the command that actually runs these rules; always verify against `policy`, not
  // `verify`, when touching anything under //#region 🔧️PolicyRule*).
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-component", // gltf-internal's steps 1-2 scope didn't include mutations; not yet reached
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-component", // 1.4 deliberately left untouched; the real vocabulary landed on 1.7 instead
  "stdio/gif/standards#87a-subsets-any-schema-mutations-component", // permanent, not a todo: 87a has no GCE/animation/transparency concept to mutate: 89a carries the real vocabulary
]);

/** 🔎️Top-level variant names of the first `pub enum ...Mutation { ... }` found in `content`, or `null` if none found. Brace-depth-tracked (variants carry struct-style bodies with their own braces). */
function policyRustMutationEnumVariants(content: string): string[] | null {
  const stripped = policyStripRustCommentsAndStrings(content);
  const headerMatch = stripped.match(/pub\s+enum\s+\w*Mutation\w*\s*\{/);
  if (!headerMatch) return null;
  const start = headerMatch.index! + headerMatch[0].length;
  let depth = 1;
  let i = start;
  let segStart = start;
  const variants: string[] = [];
  const pushSeg = (raw: string) => {
    const m = raw.match(/(?:#\[[^\]]*\]\s*)*(\w+)/);
    if (m) variants.push(m[1]!);
  };
  while (i < stripped.length && depth > 0) {
    const ch = stripped[i];
    if (ch === "{" || ch === "(") depth++;
    else if (ch === "}" || ch === ")") {
      depth--;
      if (depth === 0) {
        pushSeg(stripped.slice(segStart, i));
        break;
      }
    } else if (ch === "," && depth === 1) {
      pushSeg(stripped.slice(segStart, i));
      segStart = i + 1;
    }
    i++;
  }
  return variants;
}

function policyMutationVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const flagshipDirs = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif",
  ];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.endsWith(`🧬️mutations/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!flagshipDirs.some((dir) => relPath.startsWith(`${dir}/`))) continue;
    const normalized = policyNormalizeRelPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);
    const variants = policyRustMutationEnumVariants(content);
    const realVariantCount = variants ? variants.filter((v) => v !== "NoMutation" && v !== "SetSnapshot").length : 0;
    const allowlisted = POLICY_FLAGSHIP_MUTATION_ALLOWLIST.has(normalized);
    if (realVariantCount === 0) {
      if (allowlisted) continue;
      breaches.push({
        id: `mutation-vocabulary-${relPath}`,
        summary: `"${relPath}" is a flagship mutation enum with only NoMutation/SetSnapshot`,
        kind: "artifact-io/mutation-vocabulary",
        scope: relPath,
        priority: "high",
        reason: "D2 names this artifact as a flagship that must expose a real mutation vocabulary (targeted insert/remove/set-field variants with real apply+inverse), not just the universal NoMutation/SetSnapshot pair every artifact starts with.",
        solution: `Add real, targeted mutation variants (with real apply+inverse) to ${relPath}, or if not reached yet, add "${normalized}" to POLICY_FLAGSHIP_MUTATION_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `mutation-vocabulary-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_FLAGSHIP_MUTATION_ALLOWLIST but already has ${realVariantCount} real mutation variant(s)`,
        kind: "artifact-io/mutation-vocabulary",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_FLAGSHIP_MUTATION_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** ⚖️Aggregates stdio-artifact policy scanners (catalog, builder, decomposer, schema, io matrix, DAG, codecs,
 * plus the standards/subsets migrated-side rules -- each pair is shape-partitioned: an artifact is checked
 * by exactly one side of every pair, never both, per policyArtifactIsMigrated). */
export function policyStdioArtifactsBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyStdioCatalogBreaches(repoRoot),
    ...policyArtifactBuilderBreaches(repoRoot),
    ...policyArtifactDecomposerBreaches(repoRoot),
    ...policySchemaRepresentationBreaches(repoRoot),
    ...policyIoSerializerMatrixBreaches(repoRoot),
    ...policyIoMatrixMigratedBreaches(repoRoot),
    ...policyIoTerminalityBreaches(repoRoot),
    ...policyCodecFidelityBreaches(repoRoot),
    ...policyStandardsCoverageBreaches(repoRoot),
    ...policyArtifactBuilderMigratedBreaches(repoRoot),
    ...policyArtifactAnalyzerBreaches(repoRoot),
    ...policyArtifactComposerBreaches(repoRoot),
    ...policyDialectLiteralPathBreaches(repoRoot),
    ...policyStdioCodecIdUniquenessBreaches(repoRoot),
    ...policyMutationVocabularyBreaches(repoRoot),
    ...policyRoundTripTestBreaches(repoRoot),
    ...policyComposerDependencyBreaches(repoRoot),
    ...policyStandardSubsetVocabularyBreaches(repoRoot),
  ];
}

//#endregion 🔧️PolicyRuleArtifactIo

//#region 🔧️PolicyRuleSniffReality
/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 4: a
 * `fn sniff(...)` whose parameter is underscore-prefixed (Rust's own "deliberately unused"
 * convention) cannot possibly inspect the bytes/text it was handed — it can only ever return a
 * constant confidence, which defeats the entire point of format sniffing. Seeded at V0 with every
 * file that fails this today (shrink-only allowlist, same pattern as
 * `POLICY_PACK_COMPLETENESS_ALLOWLIST`): as each artifact's sniff is made real (see the codec
 * uplift waves), remove its entry here — an allowlisted entry whose sniff has ALREADY been fixed
 * is flagged separately below as a stale-acceptance breach, so cleanup isn't silently forgotten.
 */
const POLICY_SNIFF_REALITY_ALLOWLIST = new Set<string>([
  "writer/standards#1-subsets-any-analyzer-component",
  "mathematical/standards#1-subsets-any-analyzer-component",
  "procedural/procedural2d/standards#1-subsets-any-analyzer-component",
  "procedural/procedural3d/standards#1-subsets-any-analyzer-component",
  "flow/standards#1-subsets-any-analyzer-component",
  "gis/gisterrain/standards#1-subsets-any-analyzer-component",
  "gis/gismap/standards#1-subsets-any-analyzer-component",
  "vcs/standards#1-subsets-any-analyzer-component",
  "animate/present/standards#1-subsets-any-analyzer-component",
  "shooting/standards#1-subsets-any-analyzer-component",
  "demonstrator/playground/standards#1-subsets-any-analyzer-component",
  "sequence/standards#1-subsets-any-analyzer-component",
  "fem/2d/standards#1-subsets-any-analyzer-component",
  "fem/3d/standards#1-subsets-any-analyzer-component",
  "architect/program/standards#1-subsets-any-analyzer-component",
  "process/process3d/standards#1-subsets-any-analyzer-component",
  "lowpoly/standards#1-subsets-any-analyzer-component",
  "reasoning/wires/standards#1-subsets-any-analyzer-component",
  "forms/standards#1-subsets-any-analyzer-component",
  "layout/standards#1-subsets-any-analyzer-component",
  "cad/standards#1-subsets-any-analyzer-component",
  "norm/iso16757/standards#1-subsets-any-analyzer-component",
  "norm/vdi3805/standards#1-subsets-any-analyzer-component",
  "norm/din4108/standards#1-subsets-any-analyzer-component",
  "norm/din16798/standards#1-subsets-any-analyzer-component",
  "norm/en1990/standards#1-subsets-any-analyzer-component",
  "norm/en1991/standards#1-subsets-any-analyzer-component",
  "norm/en1992/standards#1-subsets-any-analyzer-component",
  "norm/en1993/standards#1-subsets-any-analyzer-component",
  "norm/en1994/standards#1-subsets-any-analyzer-component",
  "norm/en1995/standards#1-subsets-any-analyzer-component",
  "norm/en1996/standards#1-subsets-any-analyzer-component",
  "norm/en1997/standards#1-subsets-any-analyzer-component",
  "norm/en1998/standards#1-subsets-any-analyzer-component",
  "norm/en1999/standards#1-subsets-any-analyzer-component",
  "norm/din18599/standards#1-subsets-any-analyzer-component",
  "playbook/standards#1-subsets-any-analyzer-component",
  "imperative/standards#1-subsets-any-analyzer-component",
  "remodel/standards#1-subsets-any-analyzer-component",
  "energy/model/standards#1-subsets-any-analyzer-component",
  "trinity/rewrite/standards#1-subsets-any-analyzer-component",
  "trinity/jack/standards#1-subsets-any-analyzer-component",
  "dag/standards#1-subsets-any-analyzer-component",
  "draw/standards#1-subsets-any-analyzer-component",
  "raster/standards#1-subsets-any-analyzer-component",
  "stdio/ifc/standards#4-subsets-any-analyzer-component",
  "stdio/binary/standards#raw-subsets-any-analyzer-component",
  "stdio/pdf/standards#1.4-subsets-any-analyzer-component",
  "stdio/step/standards#ap214-subsets-any-analyzer-component",
  "stdio/xml/standards#1.0-subsets-any-analyzer-component",
  "stdio/dwg/standards#ac1018-subsets-any-analyzer-component",
  "stdio/dwg/standards#ac1024-subsets-any-analyzer-component",
  "stdio/deflate/standards#rfc1950-subsets-any-analyzer-component",
  "note/standards#1-subsets-any-analyzer-component",
  "puzzle/2d/standards#1-subsets-any-analyzer-component",
  "puzzle/5d/standards#1-subsets-any-analyzer-component",
  "puzzle/3d/standards#1-subsets-any-analyzer-component",
  "block/2d/standards#1-subsets-any-analyzer-component",
  "block/5d/standards#1-subsets-any-analyzer-component",
  "block/3d/standards#1-subsets-any-analyzer-component",
  "space/home/standards#1-subsets-any-analyzer-component",
  "sourcing/curate/standards#1-subsets-any-analyzer-component",
]);

function policySniffRealityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content.includes("fn sniff(")) continue;
    const hasUnusedParam = /fn\s+sniff\(\s*_[A-Za-z0-9_]*\s*:/.test(content);
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_SNIFF_REALITY_ALLOWLIST.has(normalized);
    if (hasUnusedParam) {
      if (allowlisted) continue;
      breaches.push({
        id: `sniff-reality-${relPath}`,
        summary: `"${relPath}" declares fn sniff(...) with an underscore-prefixed (unused) parameter`,
        kind: "artifact-io/sniff-reality",
        scope: relPath,
        priority: "high",
        reason: "sniff() must inspect the bytes/text it's handed to produce a real confidence signal — an unused parameter means it can only ever return a hardcoded constant, defeating format detection.",
        solution: `Make ${relPath}'s sniff() branch on its argument (magic bytes / structural check), or if this artifact hasn't been reached by the codec uplift yet, add "${normalized}" to POLICY_SNIFF_REALITY_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `sniff-reality-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_SNIFF_REALITY_ALLOWLIST but its sniff() already uses its argument`,
        kind: "artifact-io/sniff-reality",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed, or the allowlist silently drifts into meaninglessness.",
        solution: `Remove "${normalized}" from POLICY_SNIFF_REALITY_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleSniffReality


//#region 🔧️PolicyRuleSchemaOverhaulS2
/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, S2 (glue.rs /
 * script.ts / schema-descriptor spine wave): S-7 (vcs collection-machinery ban) + S-8 (four new
 * drift/honesty/coverage rules). All five allowlists below are shrink-only per the ticket's own
 * convention (`POLICY_SNIFF_REALITY_ALLOWLIST`'s stale-entry sub-check pattern): a "new breach"
 * fires when a file/standard fails the check and isn't allowlisted; a "stale" low-priority breach
 * fires when an allowlisted entry has ALREADY been fixed, so cleanup is never silently forgotten.
 * Every allowlist here was seeded by running the exact detection logic below against the real tree
 * at S2 time (see `s2-artifacts/gen_s8_seeds.ts` in this ticket folder for the generator) — none of
 * these are hand-guessed.
 */

/**
 * 📏️S-7: the vcs plugin's generic `CollectionDiff`/`CollectionMutation`/`Patchable`/`Identified`
 * machinery has real users (🌊️flow's `FlowMutation`, 🌿️vcs itself, 🏪️store's re-export — verified by
 * S1) but ZERO stdio artifact users today (confirmed by grep at S1 and re-confirmed here). This
 * ticket's design has every stdio artifact-standard handcraft its own snapshot/diff/mutation triple
 * per `🧬️schema-design.md` instead of reaching for the generic collection machinery — this rule
 * keeps that true going forward. Seeded EMPTY: any hit is a real regression, not a backlog item.
 */
const POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST = new Set<string>([]);

const POLICY_STDIO_VCS_MACHINERY_BAN_MARKERS = ["CollectionDiff", "CollectionMutation", "Patchable", "Identified"] as const;
const POLICY_STDIO_VCS_MACHINERY_BAN_RE = new RegExp(`\\b(${POLICY_STDIO_VCS_MACHINERY_BAN_MARKERS.join("|")})\\b`);

function policyStdioVcsMachineryBanBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const prefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(prefix)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const stripped = policyStripRustCommentsAndStrings(content);
    const hit = stripped.match(POLICY_STDIO_VCS_MACHINERY_BAN_RE)?.[1];
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST.has(normalized);
    if (hit) {
      if (allowlisted) continue;
      breaches.push({
        id: `stdio-vcs-machinery-ban-${relPath}`,
        summary: `"${relPath}" references vcs collection machinery (${hit}) — banned under stdio artifacts`,
        kind: "stdio-artifacts/vcs-machinery-ban",
        scope: relPath,
        priority: "high",
        reason: "Stdio artifacts handcraft their own snapshot/diff/mutation triples per artifact (see 🧬️schema-design.md); the generic vcs CollectionDiff/CollectionMutation/Patchable/Identified machinery is reserved for flow/vcs/store and must never leak into stdio.",
        solution: `Remove the ${hit} reference from ${relPath} and handcraft this artifact's own diff/mutation type instead, or add "${normalized}" to POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST citing a real, reviewed justification (expected to stay empty forever).`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `stdio-vcs-machinery-ban-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST but no longer references vcs collection machinery`,
        kind: "stdio-artifacts/vcs-machinery-ban",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** 🗂️One (artifact, standard, subset) triple under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, restricted to real on-disk dirs (dynamic — NOT a hardcoded "31/32" count, so a future new standard/subset is picked up automatically). */
type PolicyStdioStandardEntry = { artRel: string; artifactId: string; standardSlug: string; subsetRel: string; subsetId: string };

function policyListStdioStandardEntries(repoRoot: string): PolicyStdioStandardEntry[] {
  const out: PolicyStdioStandardEntry[] = [];
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    if (!dialect.artRel.startsWith(`${POLICY_STDIO_ARTIFACTS_REL}/`)) continue;
    out.push({
      artRel: dialect.artRel,
      artifactId: policyStripEmoji(dialect.artRel.slice(POLICY_STDIO_ARTIFACTS_REL.length + 1)),
      standardSlug: dialect.standardSlug,
      subsetRel: dialect.subsetRel,
      subsetId: dialect.subsetId,
    });
  }
  return out;
}

/**
 * 🔎️Only the SCHEMA-OWNING subset entries (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES,
 * replaces the old exact-string `subsetId === "✳️any"` filter). A subset is schema-owning iff its
 * `🧬️schema/📸️snapshot/` dir exists on disk — a real conformance subset (pdf 1.7's `✳️a`, step's
 * `✳️cc6`, …) is a validation-gated DELEGATING stamp on top of its standard's owning subset (pure
 * `pub use …::subsets::any::schema::*`, per the pilot), never a new snapshot type, so it never has
 * its own `📸️snapshot/` dir. `✳️any` is always schema-owning by construction. This is structural
 * rather than name-based so a future new subset never needs an allowlist/filter edit here — it is
 * automatically excluded from schema-internal rules (DiffAlgebra, field-sweep, grammar-honesty,
 * facet-mirror-drift) the moment it's created as a delegating subset, and automatically INCLUDED in
 * builder/analyzer/composer/io rules (those already iterate the full `policyListStdioStandardEntries`
 * result, unfiltered).
 */
function policyListStdioSchemaOwningEntries(repoRoot: string): PolicyStdioStandardEntry[] {
  return policyListStdioStandardEntries(repoRoot).filter((e) => existsSync(join(repoRoot, e.subsetRel, "🧬️schema", "📸️snapshot")));
}

/**
 * 📏️S-8 rule 3 (`POLICY_DIFF_ALGEBRA`): every stdio artifact-standard's `🔺️diff/🦀️component.rs` must
 * carry a real `impl DiffAlgebra<...> for ...` block (S1 added the trait, deliberately implemented it
 * for nothing — see `s1-spine-report.md`). Seeded with all 31 current standards; F-wave agents shrink
 * this to zero as they land real diffs with handcrafted `inverse`/`between`/`is_empty`.
 */
const POLICY_DIFF_ALGEBRA_ALLOWLIST = new Set<string>([]);

function policyDiffAlgebraBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    const rustRel = `${entry.subsetRel}/🧬️schema/🔺️diff/🦀️component.rs`;
    if (!existsSync(join(repoRoot, rustRel))) continue;
    const content = policyReadFileSafe(repoRoot, rustRel);
    const hasImpl = policyRustFileHasRealTraitImpl(content, "DiffAlgebra");
    const normalized = policyNormalizeRelPath(rustRel);
    const allowlisted = POLICY_DIFF_ALGEBRA_ALLOWLIST.has(normalized);
    if (!hasImpl) {
      if (allowlisted) continue;
      breaches.push({
        id: `diff-algebra-missing-${rustRel}`,
        summary: `"${rustRel}" has no impl DiffAlgebra<...> for ... block`,
        kind: "stdio-artifacts/diff-algebra",
        scope: entry.artRel,
        priority: "medium",
        reason: "Every stdio diff type must implement DiffAlgebra (inverse/between/is_empty) alongside MutationDiff — see 🧬️schema-design.md's Verb set per artifact.",
        solution: `Implement DiffAlgebra<${entry.artifactId}Snapshot> for the diff type in ${rustRel}, or if this standard hasn't been reached yet, add "${normalized}" to POLICY_DIFF_ALGEBRA_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `diff-algebra-stale-${rustRel}`,
        summary: `"${rustRel}" is allowlisted in POLICY_DIFF_ALGEBRA_ALLOWLIST but already implements DiffAlgebra`,
        kind: "stdio-artifacts/diff-algebra",
        scope: entry.artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_DIFF_ALGEBRA_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️S-8 rule 4 (field-sweep-test presence): every stdio artifact-standard must own a test function
 * matching `field_sweep` somewhere in its own tree — the plan's law #6, "THE acceptance criterion for
 * 'diff can change every field'" (see 🧬️schema-design.md's Test laws section). Seeded with all 31
 * current standards (none exist yet, confirmed by grep at S1/S2).
 */
const POLICY_FIELD_SWEEP_ALLOWLIST = new Set<string>([]);

function policyStdioStandardKey(artifactId: string, standardSlug: string): string {
  return `stdio/${artifactId}/standards#${standardSlug}`;
}

/** 🗝️Per-SUBSET field-sweep key (widened from `policyStdioStandardKey`'s per-standard form, ticket
 * 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1): a standard with
 * multiple schema-owning subsets (e.g. semio v1's 13) needs its OWN field_sweep test per subset —
 * one sweep anywhere under the standard must never silently cover sibling subsets. */
function policyStdioSubsetKey(artifactId: string, standardSlug: string, subsetId: string): string {
  return `${policyStdioStandardKey(artifactId, standardSlug)}/subsets#${subsetId}`;
}

function policyFieldSweepPresenceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const fieldSweepRe = /fn\s+\w*field_sweep\w*\s*\(/;
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    const standardRel = entry.subsetRel.split("/🪆️subsets/")[0]!;
    // 🔒 Scoped to the subset's OWN tree, not the whole standard — a standard with several
    // schema-owning subsets (semio v1's 13) must not let one subset's sweep cover its siblings.
    const rsFiles = policyWalkRelFiles(repoRoot, [entry.subsetRel], (_p, name) => name.endsWith(".rs"));
    const found = rsFiles.some((f) => fieldSweepRe.test(policyReadFileSafe(repoRoot, f)));
    const key = policyStdioSubsetKey(entry.artifactId, entry.standardSlug, entry.subsetId);
    const allowlisted = POLICY_FIELD_SWEEP_ALLOWLIST.has(key);
    if (!found) {
      if (allowlisted) continue;
      breaches.push({
        id: `field-sweep-missing-${key}`,
        summary: `"${entry.subsetRel}" has no test function matching field_sweep`,
        kind: "stdio-artifacts/field-sweep-presence",
        scope: entry.artRel,
        priority: "medium",
        reason: "field_sweep is the plan's law #6, the acceptance criterion that a diff can change every field of a snapshot (see 🧬️schema-design.md's Test laws section) — required per schema-owning subset, not once per standard.",
        solution: `Add a field_sweep test under ${entry.subsetRel} (sweep_a()/sweep_b() differing in every mutable field, asserting between(a,b).apply(a)==b), or if this subset hasn't been reached yet, add "${key}" to POLICY_FIELD_SWEEP_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `field-sweep-stale-${key}`,
        summary: `"${entry.subsetRel}" is allowlisted in POLICY_FIELD_SWEEP_ALLOWLIST but already has a field_sweep test`,
        kind: "stdio-artifacts/field-sweep-presence",
        scope: entry.artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${key}" from POLICY_FIELD_SWEEP_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️S-8 rule 2 (`POLICY_GRAMMAR_HONESTY`): flags a grammar leaf (`.g4`/`.ebnf`/`.grammar.semio` under
 * `📝️text/`, `.ksy`/`.spicy`/`.abnf`/`.protocol.semio` under `💾️binary/`) whose content is still the
 * placeholder skeleton every stdio facet was scaffolded with (`payload = *OCTET` / `size-eos: true` /
 * `payload: bytes &eod;` / the fixed `DOCUMENT: 'schema' [ ]+`/`header = 'schema', space,` literal
 * templates) — literal-marker textual heuristic, matching this repo's other grammar/spec policies
 * (`policySpecDistinctnessBreaches` et al.), not a real parser. Scoped to stdio only (this ticket's
 * scope); the repo-wide `.grammar`-file completeness program is `POLICY_GRAMMAR_FILE_ALLOWLIST`
 * (jolly-spindle W1), a different rule for a different file convention. Seeded with the current
 * census — 645 of 651 stdio grammar leaves (nearly all, as expected: this program hasn't rewritten
 * any grammar leaf yet, only json's `.grammar.semio`/`.protocol.semio` got real content from an
 * earlier wave and are correctly NOT in this seed).
 */
const POLICY_GRAMMAR_HONESTY_LEAF_MARKERS: Record<string, string> = {
  "🅰️component.g4": "DOCUMENT: 'schema' [ ]+",
  "🔤️component.ebnf": "header = 'schema', space,",
  "📖️component.grammar.semio": "payload = *OCTET",
  "🔠️component.abnf": "payload = *OCTET",
  "📡️component.protocol.semio": "payload = *OCTET",
  "🥋️component.ksy": "size-eos: true",
  "🌶️component.spicy": "payload: bytes &eod;",
};

const POLICY_GRAMMAR_HONESTY_ALLOWLIST = new Set<string>([
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.abnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.ksy",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.spicy",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-text-component.ebnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-text-component.g4",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-text-component.g4",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-text-component.g4",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.abnf",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.ksy",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.spicy",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/bmp/standards#v3-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.abnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.spicy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-text-component.ebnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-text-component.g4",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-text-component.g4",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-text-component.g4",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.abnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.ksy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.spicy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-text-component.ebnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-text-component.g4",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-text-component.g4",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-text-component.g4",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.abnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.ksy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.spicy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-text-component.ebnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-text-component.g4",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-text-component.g4",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-text-component.g4",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-binary-component.ksy",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-binary-component.spicy",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.abnf",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.ksy",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-diff-text-component.ebnf",
  "stdio/gif/standards#87a-subsets-any-schema-diff-text-component.g4",
  "stdio/gif/standards#87a-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-text-component.g4",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-text-component.g4",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.abnf",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.ksy",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.spicy",
  "stdio/gif/standards#89a-subsets-any-schema-diff-text-component.ebnf",
  "stdio/gif/standards#89a-subsets-any-schema-diff-text-component.g4",
  "stdio/gif/standards#89a-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-text-component.g4",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-text-component.g4",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/gltf/standards#2.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.abnf",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.ksy",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.spicy",
  "stdio/ifc/standards#4-subsets-any-schema-diff-text-component.ebnf",
  "stdio/ifc/standards#4-subsets-any-schema-diff-text-component.g4",
  "stdio/ifc/standards#4-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-text-component.g4",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/ifc/standards#4-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-binary-component.abnf",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-binary-component.ksy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-binary-component.spicy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/las/standards#1.0-subsets-any-schema-diff-text-component.ebnf",
  "stdio/las/standards#1.0-subsets-any-schema-diff-text-component.g4",
  "stdio/las/standards#1.0-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-text-component.g4",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.abnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.ksy",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-text-component.ebnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-text-component.g4",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-text-component.g4",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-text-component.g4",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/pdf/standards#1.7-subsets-any-schema-diff-binary-component.ksy",
  "stdio/pdf/standards#1.7-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-text-component.ebnf",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-text-component.g4",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.abnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.ksy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.spicy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-text-component.ebnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/step/standards#ap214-subsets-any-schema-diff-binary-component.abnf",
  "stdio/step/standards#ap214-subsets-any-schema-diff-binary-component.ksy",
  "stdio/step/standards#ap214-subsets-any-schema-diff-binary-component.spicy",
  "stdio/step/standards#ap214-subsets-any-schema-diff-text-component.ebnf",
  "stdio/step/standards#ap214-subsets-any-schema-diff-text-component.g4",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-text-component.g4",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-text-component.g4",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-binary-component.abnf",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-binary-component.ksy",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-binary-component.spicy",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-text-component.ebnf",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-text-component.g4",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-text-component.g4",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-binary-component.abnf",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-binary-component.ksy",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-binary-component.spicy",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-text-component.g4",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.abnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.ksy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.spicy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-text-component.ebnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-text-component.g4",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-text-component.g4",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-text-component.g4",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.abnf",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.ksy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.spicy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-diff-binary-component.ksy",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-text-component.ebnf",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-text-component.ebnf",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-text-component.g4",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-text-component.g4",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-text-component.g4",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-text-component.grammar.semio",
]);

function policyGrammarHonestyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const walk = (relDir: string): void => {
    for (const ent of policyReaddirSafe(repoRoot, relDir)) {
      const childRel = `${relDir}/${ent.name}`;
      if (ent.isDirectory) {
        walk(childRel);
        continue;
      }
      const marker = POLICY_GRAMMAR_HONESTY_LEAF_MARKERS[ent.name];
      if (!marker) continue;
      const content = policyReadFileSafe(repoRoot, childRel);
      const isPlaceholder = content.includes(marker);
      const normalized = policyNormalizeRelPath(childRel);
      const allowlisted = POLICY_GRAMMAR_HONESTY_ALLOWLIST.has(normalized);
      if (isPlaceholder) {
        if (allowlisted) continue;
        breaches.push({
          id: `grammar-honesty-${childRel}`,
          summary: `"${childRel}" is still the scaffolded placeholder grammar, not a handcrafted spec`,
          kind: "stdio-artifacts/grammar-honesty",
          scope: childRel,
          priority: "low",
          reason: "Every grammar/protocol leaf must be handcrafted honestly per the format's real spec (user decision, see the master plan's Context section) — no *OCTET/size-eos passthrough placeholders may survive an artifact's own wave.",
          solution: `Handcraft ${childRel} to reflect this facet's real structure, or if this artifact/facet hasn't been reached yet, add "${normalized}" to POLICY_GRAMMAR_HONESTY_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `grammar-honesty-stale-${childRel}`,
          summary: `"${childRel}" is allowlisted in POLICY_GRAMMAR_HONESTY_ALLOWLIST but is no longer a placeholder`,
          kind: "stdio-artifacts/grammar-honesty",
          scope: childRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_GRAMMAR_HONESTY_ALLOWLIST.`,
        });
      }
    }
  };
  walk(POLICY_STDIO_ARTIFACTS_REL);
  return breaches;
}

/**
 * 📏️S-8 rule 1 (`POLICY_FACET_MIRROR_DRIFT`): for every stdio standard's snapshot/diff/mutations
 * facet, the camelCased public field identifiers found in the Rust `.rs` leaf should also appear
 * (textually) in the sibling `.ts`/`.graphql`/`.json`/`.proto` leaves — textual heuristic (substring
 * presence), not a real cross-language type check, matching this repo's other facet-parity policies.
 * Bidirectional: this also harvests each sibling leaf's OWN declared field names (interface members,
 * GraphQL type/input fields, JSON Schema `properties` keys, proto message fields) and flags any that
 * have no corresponding Rust field — this is what catches a sibling still shaped like the generic W1b
 * scaffold (`entries: [{key, value}]`) sitting where real fields belong; the forward direction alone
 * would pass that scaffold as long as it also happened to textually contain the real field names.
 * Seeded with the current census: ALL 93 checked (standard,facet) pairs drift today (31 standards × 3
 * facets — none of this program's facet mirrors have been rewritten yet; e.g. gif's TS mirror is
 * still literally zip's, per the master plan's own opening finding). F-wave agents shrink this to
 * zero as they rewrite each artifact's facets for real; this wave does NOT fix any facet itself.
 */
const POLICY_FACET_MIRROR_DRIFT_FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;
const POLICY_FACET_MIRROR_DRIFT_SIBLINGS = ["🟦️component.ts", "🔗️component.graphql", "🔣️component.json", "🛰️component.proto"] as const;
const POLICY_FACET_MIRROR_DRIFT_FIELD_RE = /(?:^|[\s{,(])(?:pub\s+)?([a-z][a-z0-9_]*)\s*:\s*[A-Za-z_&\[<('"]/gm;
const POLICY_FACET_MIRROR_DRIFT_KEYWORDS = new Set(["self", "where", "if", "else", "match", "for", "while", "let", "fn", "return", "in", "as", "dyn", "mut", "ref", "impl", "type"]);

function policySnakeToCamel(name: string): string {
  const parts = name.split("_");
  return parts[0] + parts.slice(1).map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("");
}

/** 🔎️CamelCased public field/variant identifiers referenced in a schema facet's Rust leaf (textual heuristic, not a parser — matches struct field decls AND enum struct-variant field decls). */
function policyFacetRustFieldNames(content: string): string[] {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_FIELD_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_FIELD_RE.exec(stripped))) {
    const n = m[1]!;
    if (POLICY_FACET_MIRROR_DRIFT_KEYWORDS.has(n) || n.startsWith("r#")) continue;
    names.add(n);
  }
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

//#region 🔧️PolicyFacetMirrorDriftReverse
const POLICY_FACET_MIRROR_DRIFT_TS_RE = /(?:^|[\s{;(])([A-Za-z_][A-Za-z0-9_]*)\??\s*:\s*[^;\n=]+[;\n]/gm;
const POLICY_FACET_MIRROR_DRIFT_TS_KEYWORDS = new Set([
  "interface", "type", "export", "import", "from", "extends", "implements", "class", "function", "const", "let", "var",
  "namespace", "module", "declare", "readonly", "public", "private", "protected", "static", "abstract", "new", "this",
  "super", "typeof", "keyof", "case", "default", "switch", "try", "catch", "finally", "throw", "async", "await", "yield",
  "get", "set", "constructor", "void", "never", "unknown", "any", "undefined", "null", "true", "false", "satisfies",
  "asserts", "is", "infer", "of", "in", "as", "if", "else", "for", "while", "return",
]);
const POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE = /^\s*([a-z][A-Za-z0-9_]*)\s*:/gm;
const POLICY_FACET_MIRROR_DRIFT_PROTO_RE = /^\s*(?:optional\s+|repeated\s+)?(?:map\s*<[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/gm;
const POLICY_FACET_MIRROR_DRIFT_PROTO_SKIP_RE = /^\s*(enum|oneof|message|package|syntax|import)\b/;
const POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE = /#\[serde\([^\]]*\btag\s*=\s*"([a-zA-Z_][a-zA-Z0-9_]*)"/g;
/** ⚖️Below-which a sibling field name is too short to trust as a substring match against a real Rust field (avoids "id"/"at"-style accidental hits) — see `policyFacetMirrorDriftBreaches`'s extraFields step. */
const POLICY_FACET_MIRROR_DRIFT_SUBSTRING_MIN_LEN = 4;

/**
 * 🔎️Serde internally-tagged-enum discriminant values (`#[serde(tag = "kind")]`) found in a schema
 * facet's Rust leaf. These are real fields every non-Rust mirror must carry (GraphQL/JSON-Schema/
 * proto have no native tagged-union sugar, so they always spell the discriminant out as an actual
 * field) even though `policyFacetRustFieldNames` never sees them — there is no `kind: SomeType`
 * struct-field declaration in the Rust source, just this attribute. Read raw (not comment/string
 * stripped) since the tag value IS a string literal; the regex is narrow enough not to misfire
 * inside a line or block comment in practice.
 */
function policyFacetRustTagFieldNames(content: string): string[] {
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE.exec(content))) names.add(m[1]!);
  return [...names];
}

const POLICY_FACET_MIRROR_DRIFT_VARIANT_RE = /^\s*([A-Z][A-Za-z0-9]*)\s*[{(]/gm;

/**
 * 🔎️PascalCase enum variant names declared in a schema facet's Rust leaf (e.g. `enum BmpMutation {
 * SetHeaderFields { .. }, SetPixelData { .. }, .. }`, or a plain `Rgb { r, g, b }` color variant).
 * `policyFacetRustFieldNames` only sees the FIELDS a struct/struct-variant declares, never the
 * variant name itself — but externally-tagged (the serde default, no `#[serde(tag = "…")]`) or
 * `oneof`-shaped mirrors spell the variant name out as a real field/message/oneof-arm identifier
 * (proto's `SetHeaderFields set_header_fields = 3;`, a GraphQL union member, a JSON discriminant
 * value, …), same problem `policyFacetRustTagFieldNames` solves for the internally-tagged case.
 * Line-start heuristic (`PascalCaseIdent` immediately followed by `{`/`(`) — true parsing would
 * need brace-tracked enum-body awareness this file's other policy rules don't attempt either; a
 * stray match (an `Ok(`/`Some(`/struct-literal-construction hit) only ever ADDS a permitted name,
 * never suppresses a real breach, so the cost of over-matching here is small.
 */
function policyFacetRustVariantFieldNames(content: string): string[] {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_VARIANT_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_VARIANT_RE.exec(stripped))) {
    const n = m[1]!;
    names.add(n.charAt(0).toLowerCase() + n.slice(1));
  }
  return [...names];
}

/** 🔎️Field names a `.ts` sibling mirror leaf itself declares (interface/type member lines), for the reverse extra-field drift check. */
function policyFacetTsFieldNames(content: string): string[] {
  const stripped = policyStripTsCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_TS_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_TS_RE.exec(stripped))) {
    const n = m[1]!;
    if (POLICY_FACET_MIRROR_DRIFT_TS_KEYWORDS.has(n)) continue;
    names.add(n);
  }
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.graphql` sibling mirror leaf itself declares (type/input field lines), for the reverse extra-field drift check. */
function policyFacetGraphqlFieldNames(content: string): string[] {
  const noComments = content.replace(/#.*$/gm, "");
  const noArgs = noComments.replace(/\([^)]*\)/g, "");
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE.exec(noArgs))) names.add(m[1]!);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.json` sibling mirror leaf itself declares (every key under any `properties` object, including nested under `definitions`/`$defs`), for the reverse extra-field drift check. Throws if the file isn't valid JSON — callers must treat that as its own breach, not a crash. */
function policyFacetJsonFieldNames(content: string): string[] {
  const root: unknown = JSON.parse(content);
  const names = new Set<string>();
  const visit = (node: unknown): void => {
    if (!node || typeof node !== "object") return;
    if (Array.isArray(node)) {
      for (const item of node) visit(item);
      return;
    }
    const obj = node as Record<string, unknown>;
    if (obj.properties && typeof obj.properties === "object" && !Array.isArray(obj.properties)) {
      for (const key of Object.keys(obj.properties as Record<string, unknown>)) names.add(key);
    }
    for (const val of Object.values(obj)) visit(val);
  };
  visit(root);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.proto` sibling mirror leaf itself declares (numbered message fields, skipping enum/oneof/message/package/syntax/import lines), for the reverse extra-field drift check. */
function policyFacetProtoFieldNames(content: string): string[] {
  const bodyLines = content.split(/\r?\n/).filter((line) => !POLICY_FACET_MIRROR_DRIFT_PROTO_SKIP_RE.test(line));
  const body = bodyLines.join("\n");
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_PROTO_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_PROTO_RE.exec(body))) names.add(m[1]!);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Dispatches to the right per-language harvester by the sibling's own filename suffix. Returns `[]` for a suffix none of the harvesters own (defensive; every current entry in `POLICY_FACET_MIRROR_DRIFT_SIBLINGS` is covered). */
function policyFacetSiblingFieldNames(sib: string, content: string): string[] {
  if (sib.endsWith(".ts")) return policyFacetTsFieldNames(content);
  if (sib.endsWith(".graphql")) return policyFacetGraphqlFieldNames(content);
  if (sib.endsWith(".json")) return policyFacetJsonFieldNames(content);
  if (sib.endsWith(".proto")) return policyFacetProtoFieldNames(content);
  return [];
}
//#endregion 🔧️PolicyFacetMirrorDriftReverse

const POLICY_FACET_MIRROR_DRIFT_ALLOWLIST = new Set<string>([
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-component",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-component",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-component",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-component",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-component",
  "stdio/bmp/standards#v3-subsets-any-schema-snapshot-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-component",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-component",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-component",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-component",
  "stdio/gif/standards#87a-subsets-any-schema-diff-component",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-component",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-component",
  "stdio/gif/standards#89a-subsets-any-schema-diff-component",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-component",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-diff-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-snapshot-component",
  "stdio/ifc/standards#4-subsets-any-schema-diff-component",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-component",
  "stdio/ifc/standards#4-subsets-any-schema-snapshot-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-snapshot-component",
  "stdio/las/standards#1.0-subsets-any-schema-diff-component",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/las/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/md/standards#commonmark-subsets-any-schema-diff-component",
  "stdio/md/standards#commonmark-subsets-any-schema-mutations-component",
  "stdio/md/standards#commonmark-subsets-any-schema-snapshot-component",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-component",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-component",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-diff-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-mutations-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-component",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-component",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/ply/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/png/standards#1.2-subsets-any-schema-diff-component",
  "stdio/png/standards#1.2-subsets-any-schema-mutations-component",
  "stdio/png/standards#1.2-subsets-any-schema-snapshot-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/step/standards#ap214-subsets-any-schema-diff-component",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-component",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-component",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-component",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-component",
  "stdio/stl/standards#ascii-subsets-any-schema-snapshot-component",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-component",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-component",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-diff-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-mutations-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-snapshot-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/binary/standards#raw-subsets-any-schema-diff-component",
  "stdio/binary/standards#raw-subsets-any-schema-mutations-component",
  "stdio/binary/standards#raw-subsets-any-schema-snapshot-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-diff-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-mutations-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-snapshot-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-diff-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-mutations-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-snapshot-component",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-component",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-component",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-component",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-component",
]);

function policyFacetMirrorDriftBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_FACET_MIRROR_DRIFT_FACETS) {
      const facetRel = `${entry.subsetRel}/🧬️schema/${facet}`;
      const rustRel = `${facetRel}/🦀️component.rs`;
      if (!existsSync(join(repoRoot, rustRel))) continue;
      const rustContent = policyReadFileSafe(repoRoot, rustRel);
      const camelFields = policyFacetRustFieldNames(rustContent);
      // 🩹 compareFields also folds in serde tag discriminants (`#[serde(tag = "kind")]`) and
      // camelCased enum variant names — real fields every non-Rust mirror must spell out even
      // though no Rust struct literally declares them as a named field (see
      // policyFacetRustTagFieldNames's and policyFacetRustVariantFieldNames's docstrings;
      // confirmed against stdio.json/png/bmp's real, handcrafted mirrors during this rule's own
      // verification).
      const compareFields = [...camelFields, ...policyFacetRustTagFieldNames(rustContent), ...policyFacetRustVariantFieldNames(rustContent)];
      const missingBySibling: string[] = [];
      for (const sib of POLICY_FACET_MIRROR_DRIFT_SIBLINGS) {
        const sibAbs = join(repoRoot, facetRel, sib);
        if (!existsSync(sibAbs)) {
          missingBySibling.push(`${sib}:MISSING_FILE`);
          continue;
        }
        const sibContent = readFileSync(sibAbs, "utf8");
        const missingFields = camelFields.filter((f) => !sibContent.includes(f));
        if (missingFields.length > 0) missingBySibling.push(`${sib}:${missingFields.length}`);
        let siblingFields: string[];
        try {
          siblingFields = policyFacetSiblingFieldNames(sib, sibContent);
        } catch {
          missingBySibling.push(`${sib}:PARSE_ERROR`);
          continue;
        }
        // 🩹 A sibling field counts as "real" if it exactly matches a Rust/tag field, OR visibly
        // CONTAINS one as a substring (e.g. GraphQL/proto disambiguate same-named enum-variant
        // fields with a type prefix — stdio.json's real mirror spells Rust's plain `value` field
        // as `boolValue`/`stringValue` per variant — mirroring the forward check's own substring
        // leniency, just in the other direction). Case-INSENSITIVE on purpose: the disambiguating
        // prefix capitalizes the embedded real field's first letter (`arrayItems`, not
        // `arrayitems`), so a case-sensitive substring test would miss it entirely. The length
        // floor keeps this from rubber-stamping every sibling field against a trivially short real
        // field name.
        const extraFields = siblingFields.filter((f) => {
          if (f === "schema" || compareFields.includes(f)) return false;
          const fLower = f.toLowerCase();
          return !compareFields.some((cf) => cf.length >= POLICY_FACET_MIRROR_DRIFT_SUBSTRING_MIN_LEN && fLower.includes(cf.toLowerCase()));
        });
        if (extraFields.length > 0) missingBySibling.push(`${sib}:extra:${extraFields.length}`);
      }
      const normalized = policyNormalizeRelPath(rustRel);
      const allowlisted = POLICY_FACET_MIRROR_DRIFT_ALLOWLIST.has(normalized);
      if (missingBySibling.length > 0) {
        if (allowlisted) continue;
        breaches.push({
          id: `facet-mirror-drift-${rustRel}`,
          summary: `"${facetRel}" siblings drift from the Rust leaf's real fields — missing and/or extra (${missingBySibling.join(", ")})`,
          kind: "stdio-artifacts/facet-mirror-drift",
          scope: entry.artRel,
          priority: "low",
          reason: "Every camelCased field the Rust leaf declares should also appear in its sibling .ts/.graphql/.json/.proto leaves, AND every field a sibling declares should correspond to a real Rust field — a stale/copy-pasted mirror (missing real fields) or a leftover generic scaffold (extra fields with no Rust counterpart, e.g. entries/key/value) both silently drift from the real shape otherwise (see the master plan's gif-TS-mirror-is-literally-zip's finding).",
          solution: `Rewrite ${facetRel}'s sibling leaves to mirror the Rust leaf's real fields exactly (no missing, no extra), or if this artifact/facet hasn't been reached yet, add "${normalized}" to POLICY_FACET_MIRROR_DRIFT_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `facet-mirror-drift-stale-${rustRel}`,
          summary: `"${facetRel}" is allowlisted in POLICY_FACET_MIRROR_DRIFT_ALLOWLIST but its siblings no longer drift`,
          kind: "stdio-artifacts/facet-mirror-drift",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_FACET_MIRROR_DRIFT_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}

/** ⚖️Aggregates S2's S-7 ban + all four S-8 drift/honesty/coverage rules. */
export function policySchemaOverhaulS2Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyStdioVcsMachineryBanBreaches(repoRoot),
    ...policyFacetMirrorDriftBreaches(repoRoot),
    ...policyGrammarHonestyBreaches(repoRoot),
    ...policyDiffAlgebraBreaches(repoRoot),
    ...policyFieldSweepPresenceBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleSchemaOverhaulS2

//#region 🔧️PolicyRuleSchemaOverhaulPC
/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, Phase 2 PC (pilot
 * closer wave, after M1/M2/M3's mechanism spine + the P1(json/csv)/P2(zip/png)/P3(txt/binary) pilot
 * ladder). Four new shrink-only rules tracking the Phase 2 program's own "real dialect" migration —
 * same allowlist semantics as the S2 rules above (`policyGrammarHonestyBreaches` et al.): a "new
 * breach" fires when a file/standard fails the check and isn't allowlisted (an unexpected regression
 * or an undiscovered gap); a "stale" low-priority breach fires when an allowlisted entry has ALREADY
 * been fixed, so cleanup is never silently forgotten. Every allowlist below was seeded by running the
 * exact detection logic against the real tree at PC time (not hand-guessed) — see `p2-pc-report.md`
 * in the ticket folder for the exact commands/counts. These are heuristic, textual checks (matching
 * every other rule in this file, e.g. `policyGrammarHonestyBreaches`'s own placeholder-marker scan) —
 * the REAL enforcement for grammar/protocol conformance is each artifact's own
 * `committed_facet_files_parse`/`grammar_conformance_law`/`protocol_walk_law` tests plus the
 * framework's `STDIO_CONFORMANCE_GRADUATED` graduation list in `🧪️fixture-sweep/🦀️component.rs`.
 */

/**
 * 📏️`POLICY_GRAMMAR_PARSEABILITY`: every stdio `📖️component.grammar.semio` file should look like the
 * REAL M1/M2-era dialect (`dialect grammar` on its own header line, `grammar <id>`, `start <prod>`)
 * and not still carry an old-dialect/ABNF tell (a `;`-prefixed comment line, `%xHH` character-class
 * syntax, or a bare `*hexdig`/`*OCTET` prefix-repetition placeholder — none of which this dialect's
 * lexer/parser accepts). Textual heuristic only (`looksLikeRealGrammarOrProtocolDialect` below), not a
 * real parse — matching the house style documented above. Seeded with the current census: json/csv/
 * zip/png/txt/binary (the P1-P3 pilot ladder) are OUT; every other stdio standard's grammar leaves are
 * still IN (old fossil headers or unconverted ABNF bodies) — including a few standards this heuristic
 * ALSO already finds header-conformant by accident (`stdio/pdf#1.7` has a contract-correct header
 * but an untouched ABNF body, correctly caught as still-fossil by the ABNF-tell check; `stdio/semio#v1`'s
 * `✳️object` subset — a live, unrelated concurrent ticket's WIP artifact, not part of this program's
 * roster — happens to already pass both checks, so it is honestly NOT in this seed; see the PC report).
 */

/** 🔎️Shared textual heuristic for `POLICY_GRAMMAR_PARSEABILITY`/`POLICY_PROTOCOL_PARSEABILITY`: does this
 * `.grammar.semio`/`.protocol.semio` file look like the real M1/M2 dialect (own-line `dialect grammar`/
 * `dialect protocol` header directive, an `<kind> <id>` line, a `start <production>` line) with no
 * leftover old-dialect/ABNF tell (`#`-comment lines are stripped first, since a real file's own doc
 * comments legitimately quote the old ABNF spec in prose — e.g. csv's grammar cites RFC 4180's own
 * `%x20-21` char-class range inside a `#` comment, which must not trip this check). Not a real parse —
 * see this region's own doc comment for why a heuristic is sufficient here.
 */
function policyLooksLikeRealGrammarOrProtocolDialect(content: string, kind: "grammar" | "protocol"): boolean {
  const lines = content.split(/\r?\n/);
  const codeLines = lines.filter((l) => !l.trim().startsWith("#"));
  const headerWord = kind === "grammar" ? "dialect grammar" : "dialect protocol";
  const idWord = kind === "grammar" ? "grammar" : "protocol";
  const idRe = new RegExp(`^${idWord}\\s+\\S+`);
  const hasDialectLine = lines.some((l) => l.trim() === headerWord);
  const hasIdLine = lines.some((l) => idRe.test(l.trim()));
  const hasStartLine = lines.some((l) => /^start\s+\S+/.test(l.trim()));
  const code = codeLines.join("\n");
  const hasAbnfTell = codeLines.some((l) => l.trim().startsWith(";")) || /%x[0-9A-Fa-f]/.test(code) || code.includes("*hexdig") || code.includes("*OCTET");
  return hasDialectLine && hasIdLine && hasStartLine && !hasAbnfTell;
}

const POLICY_GRAMMAR_PARSEABILITY_FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;

const POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/html/standards#5-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/html/standards#5-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/html/standards#5-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-document-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-document-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-document-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-image-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-image-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-image-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-model-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-model-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-model-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-video-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-video-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-video-schema-snapshot-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-diff-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-mutations-text-component.grammar.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-snapshot-text-component.grammar.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-snapshot-text-component.grammar.semio",
]);

function policyGrammarParseabilityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_GRAMMAR_PARSEABILITY_FACETS) {
      const rel = `${entry.subsetRel}/🧬️schema/${facet}/📝️text/📖️component.grammar.semio`;
      if (!existsSync(join(repoRoot, rel))) continue;
      const content = policyReadFileSafe(repoRoot, rel);
      const looksReal = policyLooksLikeRealGrammarOrProtocolDialect(content, "grammar");
      const normalized = policyNormalizeRelPath(rel);
      const allowlisted = POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST.has(normalized);
      if (!looksReal) {
        if (allowlisted) continue;
        breaches.push({
          id: `grammar-parseability-${rel}`,
          summary: `"${rel}" does not look like the real grammar dialect (dialect grammar / grammar <id> / start <prod>, no leftover ABNF tell)`,
          kind: "stdio-artifacts/grammar-parseability",
          scope: entry.artRel,
          priority: "medium",
          reason: "Phase 2's own mandate: every stdio grammar leaf must be handcrafted in the REAL parse_grammar dialect the repo's Recognizer actually compiles, not an unparseable fossil header or an unconverted ABNF body (see 📖️phase2-design.md and p2-w0-recon-report.md's parseability census).",
          solution: `Rewrite ${rel} in the real dialect per 📖️grammar-recipe.md, or if this standard hasn't reached its FG-wave yet, add "${normalized}" to POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `grammar-parseability-stale-${rel}`,
          summary: `"${rel}" is allowlisted in POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST but already looks like the real dialect`,
          kind: "stdio-artifacts/grammar-parseability",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_PROTOCOL_PARSEABILITY`: same heuristic, for `📡️component.protocol.semio` files. Seeded
 * with the current census: json/csv/zip/png/txt/binary OUT; every other stdio standard IN (same
 * `stdio/semio#v1/subsets#object` accidental-pass caveat as the grammar rule above applies here too).
 */
const POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/avi/standards#1.0-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/epw/standards#energyplus-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/html/standards#5-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/html/standards#5-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/html/standards#5-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/mp4/standards#isobmff-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-animation-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-audio-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-brep-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-cad-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-document-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-document-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-document-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-drawing-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-image-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-image-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-image-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-mesh-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-model-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-model-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-model-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-presentation-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-video-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-video-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-video-schema-snapshot-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-diff-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-mutations-binary-component.protocol.semio",
  "stdio/semio/standards#v1-subsets-workflow-schema-snapshot-binary-component.protocol.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/tsv/standards#iana-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-snapshot-binary-component.protocol.semio",
]);

function policyProtocolParseabilityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_GRAMMAR_PARSEABILITY_FACETS) {
      const rel = `${entry.subsetRel}/🧬️schema/${facet}/💾️binary/📡️component.protocol.semio`;
      if (!existsSync(join(repoRoot, rel))) continue;
      const content = policyReadFileSafe(repoRoot, rel);
      const looksReal = policyLooksLikeRealGrammarOrProtocolDialect(content, "protocol");
      const normalized = policyNormalizeRelPath(rel);
      const allowlisted = POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST.has(normalized);
      if (!looksReal) {
        if (allowlisted) continue;
        breaches.push({
          id: `protocol-parseability-${rel}`,
          summary: `"${rel}" does not look like the real protocol dialect (dialect protocol / protocol <id> / start <block>, no leftover ABNF tell)`,
          kind: "stdio-artifacts/protocol-parseability",
          scope: entry.artRel,
          priority: "medium",
          reason: "Phase 2's own mandate: every stdio protocol leaf must be handcrafted in the REAL parse_protocol dialect walk_protocol actually walks, not an unparseable fossil header or an unconverted ABNF body (see 📖️phase2-design.md and p2-w0-recon-report.md's parseability census).",
          solution: `Rewrite ${rel} in the real dialect per 📖️grammar-recipe.md, or if this standard hasn't reached its FG-wave yet, add "${normalized}" to POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `protocol-parseability-stale-${rel}`,
          summary: `"${rel}" is allowlisted in POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST but already looks like the real dialect`,
          kind: "stdio-artifacts/protocol-parseability",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_FIXTURE_HONESTY`: every stdio ARTIFACT's (not per-standard — the demo fixture pair lives
 * once per artifact dir, shared across a multi-standard artifact like gif 87a/89a) `🗣️example.dsl.semio`
 * must start with a genuine `semio stdio.<artifact>`-prefixed preamble line (not a Phase-1-era fake like
 * `{"hello":"stdio.xml","n":1}` with no preamble at all), AND a sibling `🎒️example.pack.semio` must
 * exist on disk. Seeded with the current census: the 6 piloted artifacts (binary/csv/json/png/txt/zip)
 * are OUT; every other stdio artifact is still IN (including a stray, content-less `🧬️schema` dir
 * directly under `🗿️artifacts/` — not a real artifact, harmless to seed, will self-resolve as a stale
 * entry if that debris is ever cleaned up).
 */
const POLICY_FIXTURE_HONESTY_ALLOWLIST = new Set<string>([
  "stdio/avi",
  "stdio/epw",
  "stdio/html",
  "stdio/mp3",
  "stdio/mp4",
  "stdio/schema",
  "stdio/semio",
  "stdio/tsv",
  "stdio/wav",
]);

function policyStdioArtifactKey(artifactId: string): string {
  return `stdio/${artifactId}`;
}

function policyFixtureHonestyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const art of policyReaddirSafe(repoRoot, POLICY_STDIO_ARTIFACTS_REL)) {
    if (!art.isDirectory) continue;
    const artRel = `${POLICY_STDIO_ARTIFACTS_REL}/${art.name}`;
    const artifactId = policyStripEmoji(art.name);
    const dslRel = `${artRel}/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`;
    const packRel = `${artRel}/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`;
    const dslAbs = join(repoRoot, dslRel);
    const dslOk = existsSync(dslAbs) && readFileSync(dslAbs, "utf8").split(/\r?\n/, 1)[0]!.trim().startsWith(`semio stdio.${artifactId}`);
    const packOk = existsSync(join(repoRoot, packRel));
    const key = policyStdioArtifactKey(artifactId);
    const allowlisted = POLICY_FIXTURE_HONESTY_ALLOWLIST.has(key);
    const honest = dslOk && packOk;
    if (!honest) {
      if (allowlisted) continue;
      breaches.push({
        id: `fixture-honesty-${artRel}`,
        summary: `"${artRel}" fixtures are not genuine (dsl.semio preamble ok=${dslOk}, pack.semio present=${packOk})`,
        kind: "stdio-artifacts/fixture-honesty",
        scope: artRel,
        priority: "medium",
        reason: "Phase 2's own mandate: 🗣️example.dsl.semio must be the genuine print_dsl output (with its mandatory `semio stdio.<artifact>...` preamble line, not a Phase-1-era fake) and a genuine 🎒️example.pack.semio (real encode_pack bytes) must exist alongside it (see 📖️phase2-design.md's per-standard deliverable list).",
        solution: `Regenerate ${dslRel}/${packRel} from the real print_dsl/encode_pack output, or if this artifact hasn't reached its FG-wave yet, add "${key}" to POLICY_FIXTURE_HONESTY_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `fixture-honesty-stale-${artRel}`,
        summary: `"${artRel}" is allowlisted in POLICY_FIXTURE_HONESTY_ALLOWLIST but its fixtures are already genuine`,
        kind: "stdio-artifacts/fixture-honesty",
        scope: artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${key}" from POLICY_FIXTURE_HONESTY_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_LANGUAGE_REGISTRATION`: every stdio (artifact, standard)'s `⚙️engine::register()` must call
 * `dsl::register_language` at least 5 times — the full 5-role `LanguageSpec` registration (Document/
 * Ops/Diff/Pack/Spr) every P1-P3 pilot landed, per note's exemplar pattern (see 📖️grammar-recipe.md).
 * Plain grep count on the standard's own `⚙️engine/🦀️component.rs`, matching this file's other
 * grep-count-based rules. Seeded with the current census: the 6 piloted standards (5 calls each) are
 * OUT; every other stdio standard is still IN (0-1 calls — the pre-Phase-2 single-role registration,
 * or none at all).
 */
const POLICY_LANGUAGE_REGISTRATION_MIN_CALLS = 5;
const POLICY_LANGUAGE_REGISTRATION_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0",
  "stdio/epw/standards#energyplus",
  "stdio/html/standards#5",
  // 🎓️ P2-PW: jpg/jfif-1.01's real gap (0 of 5 register_language calls, flagged by P2-FG2's closer)
  // was fixed by the dedicated FG2-fix wave (p2-fg2-fix-jpg-report.md) — jpg now genuinely registers
  // 5/5 languages, confirmed by direct grep. Removed below along with every other now-real FG-wave
  // standard; kept here only as the historical pointer for why this allowlist once carried jpg.
  "stdio/mp3/standards#mpeg1-layer3",
  "stdio/mp4/standards#isobmff",
  "stdio/semio/standards#v1",
  "stdio/tsv/standards#iana",
  "stdio/wav/standards#riff-pcm",
]);

function policyLanguageRegistrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const seenStandards = new Set<string>();
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    const standardRel = entry.subsetRel.split("/🪆️subsets/")[0]!;
    const key = policyStdioStandardKey(entry.artifactId, entry.standardSlug);
    if (seenStandards.has(key)) continue; // 🔒 one check per (artifact, standard), not per schema-owning subset
    seenStandards.add(key);
    const engineRel = `${standardRel}/⚙️engine/🦀️component.rs`;
    const count = existsSync(join(repoRoot, engineRel)) ? (policyReadFileSafe(repoRoot, engineRel).match(/register_language/g) ?? []).length : 0;
    const allowlisted = POLICY_LANGUAGE_REGISTRATION_ALLOWLIST.has(key);
    const ok = count >= POLICY_LANGUAGE_REGISTRATION_MIN_CALLS;
    if (!ok) {
      if (allowlisted) continue;
      breaches.push({
        id: `language-registration-${key}`,
        summary: `"${engineRel}" calls register_language ${count} time(s), fewer than the required ${POLICY_LANGUAGE_REGISTRATION_MIN_CALLS} (5-role LanguageSpec set)`,
        kind: "stdio-artifacts/language-registration",
        scope: entry.artRel,
        priority: "medium",
        reason: "Phase 2's own mandate: every standard registers the full 5-role LanguageSpec (Document/Ops/Diff/Pack/Spr), per note's exemplar and every P1-P3 pilot's own register_pilot_languages() (see 📖️grammar-recipe.md's registration checklist).",
        solution: `Add the missing dsl::register_language roles to ${engineRel}'s register_pilot_languages(), or if this standard hasn't reached its FG-wave yet, add "${key}" to POLICY_LANGUAGE_REGISTRATION_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `language-registration-stale-${key}`,
        summary: `"${engineRel}" is allowlisted in POLICY_LANGUAGE_REGISTRATION_ALLOWLIST but already registers ${count} >= ${POLICY_LANGUAGE_REGISTRATION_MIN_CALLS} languages`,
        kind: "stdio-artifacts/language-registration",
        scope: entry.artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${key}" from POLICY_LANGUAGE_REGISTRATION_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️`POLICY_STDIO_JSON_TRANSFER_BAN`: no stdio artifact's `ArtifactPack`/`OpBinary`/`DiffCodec` impl
 * block may use `serde_json::to_vec(`/`serde_json::from_slice(` as its transfer mechanism (brace-
 * matched scan of each `impl (...::)?(ArtifactPack|OpBinary|DiffCodec) for ... { ... }` block's own
 * body — deliberately narrower than a whole-file grep, so an artifact's legitimate NATIVE json
 * parsing elsewhere in the same file, e.g. gltf's own `⚙️engine`, is never a false positive). A second,
 * narrower check covers the one real cross-artifact bridge surface W0's census found that isn't
 * literally one of those three impls: any `.rs` file under a `🚪️io/` (import/export bridge) dir using
 * a `serde_json::{to_vec,from_slice,to_string,from_str}(` call (gltf's json-snapshot deserializer
 * bridge). Per-standard `#diff`/op/pack facets legitimately implementing these traits with REAL binary
 * (`dsl::ByteWriter`/`pack_rt`/`dsl::variants_binary`) are unaffected — only the literal serde_json
 * call inside the impl body trips this. Seeded with the current census (run fresh at PC time, per
 * this rule's own mandate to confirm rather than assume): W0's originally-named 4 (ifc/2x3's
 * mutations OpBinary, svg's and xml's snapshot ArtifactPack, gltf's io bridge) are all still real
 * violations, unfixed by the pilot ladder (none of those 4 standards were in the P1-P3 roster) —
 * plus a real, larger set of additional violations from a separate, currently-live concurrent ticket
 * scaffolding new stdio artifact types (avi/mp3/mp4/wav's mutations OpBinary, and most of 🧿️semio
 * v1's many subsets' snapshot ArtifactPack / mutations OpBinary) — included honestly since this
 * policy's detection logic was run for real against the current tree, not limited to the original 4.
 */
const POLICY_STDIO_JSON_TRANSFER_BAN_TRAIT_RE = /impl(?:<[^>]*>)?\s+(?:[\w:]+::)?(ArtifactPack|OpBinary|DiffCodec)[^{]*for/g;

/** 🔎️First of `ArtifactPack`/`OpBinary`/`DiffCodec` whose own `impl ... for ... { ... }` block body contains a literal `serde_json::to_vec(`/`serde_json::from_slice(` call, or `undefined` if none. */
function policyStdioJsonTransferTraitHit(content: string): string | undefined {
  POLICY_STDIO_JSON_TRANSFER_BAN_TRAIT_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_STDIO_JSON_TRANSFER_BAN_TRAIT_RE.exec(content))) {
    const block = policyExtractFnBody(content, m.index);
    if (block.includes("serde_json::to_vec(") || block.includes("serde_json::from_slice(")) return m[1];
  }
  return undefined;
}

const POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST = new Set<string>([
  "stdio/avi/standards#1.0-subsets-any-schema-mutations-component",
  // 🎓️ P2-PW: the live `serde_json::to_vec(&from.value)` transfer call this entry was seeded for was
  // genuinely replaced by FG3 (reusing json's own real text codec, p2-fg3-closer-report.md) — direct
  // read confirms zero live serde_json calls remain in this file. Stays allowlisted only because the
  // checker's substring scan (by design, not stripping comments — see this rule's own doc comment)
  // still matches the SAME string appearing inside this file's own historical doc comment describing
  // what the old code used to do ("was a literal `serde_json::to_vec(&from.value)` JSON..."). A known
  // checker limitation, not a real violation and not a rule-logic change; left in place rather than
  // removed to avoid a false "medium" breach at the next policy run.
  "stdio/gltf/standards#2.0-subsets-any-io-import-deserializers-artifacts-json-rfc8259-any-component",
  "stdio/mp3/standards#mpeg1-layer3-subsets-any-schema-mutations-component",
  "stdio/mp4/standards#isobmff-subsets-any-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-animation-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-any-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-audio-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-brep-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-brep-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-cad-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-document-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-drawing-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-drawing-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-image-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-mesh-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-model-schema-mutations-component",
  "stdio/semio/standards#v1-subsets-model-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-object-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-presentation-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-video-schema-snapshot-component",
  "stdio/semio/standards#v1-subsets-workflow-schema-snapshot-component",
  "stdio/wav/standards#riff-pcm-subsets-any-schema-mutations-component",
]);

function policyStdioJsonTransferBanBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const prefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(prefix)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const traitHit = policyStdioJsonTransferTraitHit(content);
    const ioHit =
      relPath.includes("🚪️io") &&
      (content.includes("serde_json::to_vec(") || content.includes("serde_json::from_slice(") || content.includes("serde_json::to_string(") || content.includes("serde_json::from_str("));
    const hit = traitHit ?? (ioHit ? "io-bridge" : undefined);
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST.has(normalized);
    if (hit) {
      if (allowlisted) continue;
      breaches.push({
        id: `stdio-json-transfer-ban-${relPath}`,
        summary: `"${relPath}" uses serde_json (${hit}) as an ArtifactPack/OpBinary/DiffCodec transfer mechanism`,
        kind: "stdio-artifacts/json-transfer-ban",
        scope: relPath,
        priority: "medium",
        reason: "Phase 2 decision 4 (no JSON / no serde on any transfer path): stdio artifact transfer paths must be real binary (dsl::ByteWriter/pack_rt/dsl::variants_binary) or DSL text, never a literal serde_json round trip disguised as a binary pack.",
        solution: `Replace the serde_json call in ${relPath}'s transfer impl with a real binary/DSL-text encoding, or if this standard hasn't reached its FG-wave yet, add "${normalized}" to POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `stdio-json-transfer-ban-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST but no longer uses serde_json as its transfer mechanism`,
        kind: "stdio-artifacts/json-transfer-ban",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** ⚖️Aggregates PC's four new parseability/fixture-honesty/registration/json-transfer-ban rules. */
export function policySchemaOverhaulPCBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyGrammarParseabilityBreaches(repoRoot),
    ...policyProtocolParseabilityBreaches(repoRoot),
    ...policyFixtureHonestyBreaches(repoRoot),
    ...policyLanguageRegistrationBreaches(repoRoot),
    ...policyStdioJsonTransferBanBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleSchemaOverhaulPC

//#region 🔧️PolicyRuleDissolvedKernels
/** 🧊️ Doctrine tier (d) ephemeral working representations — legal only as locals inside a diff
 * constructor or an `InferredField::{plan,dep_input,compute}` body, never as durable state. */
const POLICY_DISSOLVED_EPHEMERAL_REP_TYPES = ["HalfedgeMesh", "BrepEngineHost", "DrawingStore", "DrawingEngine", "EngineCache"] as const;

/** 🧊️ Modules sanctioned to own an `EngineCache`: the wasm guest↔host boundary only. */
const POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS = ["🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"] as const;

/** 🧊️ Shrink-only. Every entry is a known tier-(d) escape awaiting its dissolution wave. */
const POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST = new Set<string>([
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs",
  "🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs",
]);

/** 🧊️ A durable field or static holding an ephemeral representation — measures REACH, not mutability.
 *
 * A `static HOST: OnceLock<BrepEngineHost>` is write-once, so every mutability-based rule passes it,
 * and it is still a plugin holding a handle to host-owned engine state for the process lifetime.
 * Reach is the property that matters; a separate rule from any `&mut`/`static mut` check. */
function policyDissolvedRepEscapeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const normalized = policyNormalizeRelPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);
    const held: string[] = [];
    for (const repType of POLICY_DISSOLVED_EPHEMERAL_REP_TYPES) {
      const fieldHit = new RegExp(`^\\s*(pub\\s+)?[a-z_][a-z0-9_]*\\s*:\\s*[^,;]*\\b${repType}\\b`, "m").test(content);
      const staticHit = new RegExp(`static\\s+[A-Z0-9_]+\\s*:\\s*[^=;]*\\b${repType}\\b`).test(content);
      if (fieldHit || staticHit) held.push(repType);
    }
    const allowlisted = POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST.has(normalized);
    if (held.length > 0) {
      if (allowlisted) continue;
      breaches.push({
        id: `dissolved-rep-escape-${relPath}`,
        summary: `"${relPath}" holds an ephemeral working representation (${held.join(", ")}) in a durable field or static`,
        kind: "dissolved-kernels/rep-escape",
        scope: relPath,
        priority: "medium",
        reason:
          "Doctrine tier (d): a working representation (halfedge adjacency, BVH, brep arena, tessellation buffer, engine cache) may exist only as a local inside a 🔺️diff constructor or an InferredField::{plan,dep_input,compute} body. Held durably it becomes authoritative state living outside the ArtifactStore — and a write-once wrapper such as OnceLock does not help, because the violation is ambient REACH, not ambient mutability.",
        solution: `Rebuild the representation from the snapshot via EngineRep::build(&base) at each use in ${relPath}, or if this file's dissolution wave has not landed yet, add "${normalized}" to POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST citing ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `dissolved-rep-escape-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST but no longer holds an ephemeral representation`,
        kind: "dissolved-kernels/rep-escape",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is dissolved.",
        solution: `Remove "${normalized}" from POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** 🧊️ `EngineCache` reachable outside the wasm guest↔host boundary.
 *
 * Its general "kernel cache" role is over: derived values belong in a 💡️inference facet keyed by
 * DepHash, ephemeral representations in an EngineRep. No seed — the narrowed scope is the target
 * state going forward, so this fails on new violations rather than burning down a backlog. */
function policyDissolvedEngineCacheScopeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS.some((dir) => relPath.startsWith(dir))) continue;
    const normalized = policyNormalizeRelPath(relPath);
    if (POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST.has(normalized)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!/\bEngineCache::new\b/.test(content)) continue;
    breaches.push({
      id: `dissolved-engine-cache-scope-${relPath}`,
      summary: `"${relPath}" constructs or implements an engine cache outside the sanctioned wasm-boundary modules`,
      kind: "dissolved-kernels/engine-cache-scope",
      scope: relPath,
      priority: "medium",
      reason:
        "EngineCache survives only at the wasm guest↔host boundary, where byte serialization is unavoidable. Elsewhere a cache of derived values is state outside the ArtifactStore: derived values belong in a 💡️inference facet with a real DepHash chain, ephemeral representations in an EngineRep rebuilt from the snapshot.",
      solution: `Move the derived value in ${relPath} into a 💡️inference facet, or rebuild it per call via EngineRep::build(&base).`,
    });
  }
  return breaches;
}

/** 🧊️ A whole-document-replace triad directory — banned vocabulary with no replacement.
 *
 * Whole-document replace is not expressible as an in-history mutation; it goes through
 * `ArtifactStore::reset`, outside history. A directory-level check catches what an identifier grep
 * misses, because the dispatch arm and the triad dir fail independently. */
function policyDissolvedWholeDocumentReplaceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.includes("🧬️mutations/")) continue;
    if (!/\/📄set-snapshot\//.test(relPath)) continue;
    const dir = relPath.slice(0, relPath.indexOf("/📄set-snapshot/") + "/📄set-snapshot".length);
    breaches.push({
      id: `dissolved-whole-document-replace-${dir}`,
      summary: `"${dir}" is a whole-document-replace triad directory`,
      kind: "dissolved-kernels/whole-document-replace",
      scope: dir,
      priority: "medium",
      reason:
        "Whole-document replace is not an in-history mutation at all — it records what the document became, never what the user did, and has no meaningful inverse. It goes through ArtifactStore::reset (a non-undoable rebase used for file-open/import), entirely outside the mutation vocabulary.",
      solution: `Delete ${dir}, its dispatch arm, and its 📦️glue.rs #[path] mount in the SAME change — a mount pointing at a removed directory aborts the build before compilation, for every crate in the workspace.`,
    });
  }
  return Array.from(new Map(breaches.map((b) => [b.id, b])).values());
}

/** ⚖️ Aggregates the dissolved-kernels doctrine rules (ticket 26/08/12/DISSOLVE-KERNELS…). */
export function policyDissolvedKernelsBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyDissolvedRepEscapeBreaches(repoRoot),
    ...policyDissolvedEngineCacheScopeBreaches(repoRoot),
    ...policyDissolvedWholeDocumentReplaceBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleDissolvedKernels

//#region 🔧️PolicyRuleComposition
/** 🪪️ Canonical artifact-kind grammar mirror of `ArtifactKindId::parse`/`is_canonical_artifact_kind`
 * (🧰️framework/🔨️modules/🚪️io/🦀️component.rs:101-153, ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
 * W1): exactly three dot-separated ASCII segments, the first literally `s`, the remaining two
 * lowercase-kebab (`[a-z0-9-]`, no leading/trailing/doubled hyphen). */
function policyIsCanonicalArtifactKind(kind: string): boolean {
  const segments = kind.split(".");
  if (segments.length !== 3 || segments[0] !== "s") return false;
  const isKebabSegment = (segment: string): boolean =>
    segment.length > 0 && !segment.startsWith("-") && !segment.endsWith("-") && !segment.includes("--") && /^[a-z0-9-]+$/.test(segment);
  return isKebabSegment(segments[1]!) && isKebabSegment(segments[2]!);
}

/** 🔎️True when a regex match's own source line (from the preceding newline up to the match) already
 * contains a `//` — i.e. the "hit" is doc-comment prose (`///`/`//!` quoting the pattern as an
 * example), not real code. Cheap same-line guard against the exact false-positive class flagged in
 * this ticket's `📌️important.md` ("the sweep rule… strip comments, check the target"). */
function policyMatchIsCommentedOut(content: string, matchIndex: number): boolean {
  const lineStart = content.lastIndexOf("\n", matchIndex - 1) + 1;
  return content.slice(lineStart, matchIndex).includes("//");
}

const POLICY_ARTIFACT_KIND_SPEC_ID_RE = /ArtifactKindSpec\s*\{\s*id:\s*"((?:[^"\\]|\\.)*)"/g;

/** 🗿️ `ArtifactKindSpec { id: "…", … }` construction sites — the pre-migration legacy-registration
 * shape (`pub fn artifact_kind() -> ArtifactKindSpec`) still declaring an artifact's own kind identity
 * across dozens of plugin `artifact_kind()` functions. Only LITERAL string ids are checked here:
 * `#[child(kind = "…")]` reference values legitimately carry a 4th subset segment
 * (`s.stdio.semio.<subset>`, since stdio's single `semio` artifact hosts all 18 subsets under one
 * 3-segment kind) and are references to an already-declared kind, not declaration sites themselves —
 * out of this rule's scope by design (see design doc: "only actual ArtifactKindSpec/kind-declaration
 * sites… are breaches"). Non-literal ids (`format!(...)`, a delegating helper fn call) are skipped
 * rather than guessed at. Renaming these ids to canonical grammar is explicitly a later/APA
 * registration-macro wave (io/component.rs's own doc comment: "renaming existing artifact ids to this
 * grammar is a later wave") — `medium` priority so this known, deferred debt does not gate the build;
 * the rule's job is to stop the count growing, not to burn it down itself. */
export function policyCanonicalArtifactKindBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content.includes("ArtifactKindSpec")) continue;
    POLICY_ARTIFACT_KIND_SPEC_ID_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_ARTIFACT_KIND_SPEC_ID_RE.exec(content))) {
      if (policyMatchIsCommentedOut(content, m.index)) continue;
      const kind = m[1]!;
      if (policyIsCanonicalArtifactKind(kind)) continue;
      const line = content.slice(0, m.index).split("\n").length;
      breaches.push({
        id: `canonical-artifact-kind-${relPath}-${line}`,
        summary: `"${relPath}:${line}" declares ArtifactKindSpec.id "${kind}", not canonical grammar s.<plugin>.<artifact>`,
        kind: "composition/canonical-artifact-kind",
        scope: relPath,
        line,
        priority: "medium",
        reason:
          "Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's ArtifactKindId::parse (🚪️io/🦀️component.rs) is now the ONLY sanctioned artifact-kind grammar: three dot-separated ASCII segments, s.<plugin>.<artifact>, kebab-case. This is pre-migration legacy debt — renaming existing ids is a later/APA registration-macro wave, not this rule's job — flagged so nothing NEW regresses further from canonical grammar while that wave is pending.",
        solution: `Rename this ArtifactKindSpec.id to s.<plugin>.<artifact> when ${relPath} migrates to the declarative registration macro (ticket ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE), or if this is a genuinely new declaration, use canonical grammar from the start.`,
      });
    }
  }
  return breaches;
}

type PolicyChildSlotOwner = { ownerKey: string; kindCandidates: string[] };

/** 🧩️Derives a composition-graph node identity from a schema file's path: `<plugin>/<artifact>` or,
 * under a multi-subset artifact, `<plugin>/<artifact>/<subset>` (the `🪆️subsets/✳️<subset>/` segment),
 * plus the `s.<plugin>.<artifact>[.<subset>]` kind string(s) a `#[child(kind = "…")]` elsewhere in the
 * repo would use to target this owner. Mirrors the real convention confirmed by grepping the migrated
 * plugins: `s.stdio.semio.<subset>` for stdio's multi-subset `semio` artifact, plain
 * `s.<plugin>.<artifact>` for a single-subset (`✳️any`) artifact. */
function policyChildSlotOwner(relPath: string): PolicyChildSlotOwner | null {
  const segments = relPath.split("/");
  const pluginsIdx = segments.indexOf("🔌️plugins");
  const artifactsIdx = segments.indexOf("🗿️artifacts");
  if (pluginsIdx < 0 || artifactsIdx < 0 || artifactsIdx <= pluginsIdx) return null;
  const pluginSlug = policyStripEmoji(segments[pluginsIdx + 1] ?? "");
  const artifactSlug = policyStripEmoji(segments[artifactsIdx + 1] ?? "");
  if (!pluginSlug || !artifactSlug) return null;
  const subsetsIdx = segments.indexOf("🪆️subsets");
  const subsetSlug = subsetsIdx >= 0 && segments.length > subsetsIdx + 1 ? policyStripEmoji(segments[subsetsIdx + 1] ?? "") : "";
  const ownerKey = subsetSlug ? `${pluginSlug}/${artifactSlug}/${subsetSlug}` : `${pluginSlug}/${artifactSlug}`;
  const kindCandidates: string[] = [];
  if (subsetSlug && subsetSlug !== "any") kindCandidates.push(`s.${pluginSlug}.${artifactSlug}.${subsetSlug}`);
  kindCandidates.push(`s.${pluginSlug}.${artifactSlug}`);
  return { ownerKey, kindCandidates };
}

const POLICY_CHILD_KIND_RE = /#\[child\(kind\s*=\s*"([^"]+)"\)\]/g;

/** 🔁️`#[child(kind = "…")]` composition-slot declarations must form an acyclic ownership graph — no
 * artifact may (transitively) compose itself as a child (design doc: `CompositionGraph{Owns: forest}`;
 * `VcsError::CompositionCycle` is the runtime backstop this rule catches statically, at author time).
 * Builds a directed graph over every schema file's derived owner key (`policyChildSlotOwner`), with an
 * edge for every `#[child(kind = "…")]` attribute whose target resolves to another declared owner, then
 * runs DFS cycle detection. Unresolvable targets (kind strings that don't match any declared owner —
 * a referential-integrity concern, not an acyclicity one) are silently skipped, not guessed at. */
export function policyChildSlotKindDagBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const schemaFiles = policyAllRustFiles(repoRoot).filter((f) => f.includes("🔌️plugins/") && f.includes("🧬️schema"));

  const kindToOwner = new Map<string, string>();
  for (const relPath of schemaFiles) {
    const owner = policyChildSlotOwner(relPath);
    if (!owner) continue;
    for (const kind of owner.kindCandidates) if (!kindToOwner.has(kind)) kindToOwner.set(kind, owner.ownerKey);
  }

  const edges = new Map<string, Set<string>>();
  const edgeSites = new Map<string, string>();
  for (const relPath of schemaFiles) {
    const owner = policyChildSlotOwner(relPath);
    if (!owner) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content.includes("#[child(")) continue;
    POLICY_CHILD_KIND_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_CHILD_KIND_RE.exec(content))) {
      if (policyMatchIsCommentedOut(content, m.index)) continue;
      const targetOwner = kindToOwner.get(m[1]!);
      if (!targetOwner) continue;
      if (!edges.has(owner.ownerKey)) edges.set(owner.ownerKey, new Set());
      edges.get(owner.ownerKey)!.add(targetOwner);
      edgeSites.set(`${owner.ownerKey}->${targetOwner}`, relPath);
    }
  }

  const WHITE = 0,
    GRAY = 1,
    BLACK = 2;
  const color = new Map<string, number>();
  const reportedCycles = new Set<string>();
  const dfs = (node: string, path: string[]): string[] | null => {
    color.set(node, GRAY);
    path.push(node);
    for (const next of edges.get(node) ?? []) {
      const c = color.get(next) ?? WHITE;
      if (c === GRAY) return [...path, next];
      if (c === WHITE) {
        const found = dfs(next, path);
        if (found) return found;
      }
    }
    path.pop();
    color.set(node, BLACK);
    return null;
  };
  for (const node of edges.keys()) {
    if ((color.get(node) ?? WHITE) !== WHITE) continue;
    const cycle = dfs(node, []);
    if (!cycle) continue;
    const key = [...cycle].sort().join(">");
    if (reportedCycles.has(key)) continue;
    reportedCycles.add(key);
    const chain = cycle.join(" -> ");
    breaches.push({
      id: `child-slot-kind-dag-cycle-${key}`,
      summary: `Composition ownership cycle: ${chain}`,
      kind: "composition/child-slot-kind-dag",
      scope: edgeSites.get(`${cycle[0]}->${cycle[1]}`) ?? cycle[0]!,
      priority: "high",
      reason:
        "Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's composition model requires the ownership graph to be a forest (CompositionGraph{Owns: forest}); a #[child(kind = …)] cycle means an artifact transitively composes itself, which CompositionCoordinator would reject at dispatch time (VcsError::CompositionCycle) — better caught here, statically, before it ever reaches a running app.",
      solution: `Break the cycle ${chain} by removing or re-pointing one of the #[child(kind = …)] slots along this chain.`,
    });
  }
  return breaches;
}

/** 🧊️ Shrink-only. Confirmed, justified exceptions to the dissolved-kind redefinition ban below, keyed
 * `<relPath>:<TypeName>` (mirrors `POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST`'s shape). Empty at seed time —
 * the ~25-plugin migration wave that authored this ticket's frozen stdio roster already eliminated
 * every known duplicate of these 18 shapes; this allowlist exists only for a future, deliberately-
 * justified exception, never as a place to launder new debt. */
const POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST = new Set<string>([]);

/** 🧊️ The 18 canonical stdio `🧿️semio` subset snapshot types (ticket
 * 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's frozen roster, `📓️status.md` "ROSTER FROZEN") — the
 * ONE shape each of these content kinds may take repo-wide. */
const POLICY_DISSOLVED_KIND_CANONICAL_TYPES = [
  "SemioAnimationSnapshot",
  "SemioAudioSnapshot",
  "SemioBrepSnapshot",
  "SemioCadSnapshot",
  "SemioDocumentSnapshot",
  "SemioDrawingSnapshot",
  "SemioFlowSnapshot",
  "SemioGraphSnapshot",
  "SemioImageSnapshot",
  "SemioKitSnapshot",
  "SemioMeshSnapshot",
  "SemioModelSnapshot",
  "SemioObjectSnapshot",
  "SemioPresentationSnapshot",
  "SemioTableSnapshot",
  "SemioTextSnapshot",
  "SemioValueSnapshot",
  "SemioVideoSnapshot",
] as const;

/** 🚫️ Dissolved-kind redefinition ban — seeded allowlist ratchet (design doc "W6 ratchet" scope,
 * corrigendum-narrowed to composition-specific predicates only). A plugin outside `🗄️stdio` redeclaring
 * one of the 18 frozen `🧿️semio` subset snapshot types by name is exactly the duplicated-content-type
 * failure this ticket's migration wave exists to eliminate (design doc: "9+ mesh types, 4 brep
 * topologies… duplicate kind ids"). Deliberately narrow — an EXACT canonical struct/enum name
 * collision only, never a fuzzy shape-similarity heuristic — because the ~25-plugin migration already
 * burned this down to zero known duplicates: this rule's job is to catch a NEW one being reintroduced,
 * not to re-litigate settled debt (a broad heuristic here would drown real signal, exactly the trap
 * `📌️important.md` warns against). */
export function policyDissolvedKindRedefinitionBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.includes("🔌️plugins/") || relPath.includes("🔌️plugins/🗄️stdio/")) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    for (const typeName of POLICY_DISSOLVED_KIND_CANONICAL_TYPES) {
      if (!content.includes(typeName)) continue;
      const re = new RegExp(`^[ \\t]*(?:pub(?:\\([^)]*\\))?\\s+)?(?:struct|enum)\\s+${typeName}\\b`, "m");
      const match = re.exec(content);
      if (!match) continue;
      if (policyMatchIsCommentedOut(content, match.index)) continue;
      const line = content.slice(0, match.index).split("\n").length;
      const key = `${relPath}:${typeName}`;
      if (POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST.has(key)) continue;
      breaches.push({
        id: `dissolved-kind-redefinition-${relPath}-${typeName}`,
        summary: `"${relPath}:${line}" redefines "${typeName}", one of the 18 frozen stdio 🧿️semio subset snapshot types, outside 🗄️stdio`,
        kind: "composition/dissolved-kind-redefinition",
        scope: relPath,
        line,
        priority: "high",
        reason:
          "Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM dissolved every plugin-local duplicate of these 18 neutral content shapes into stdio's frozen 🧿️semio roster; a plugin redeclaring the same type name is the exact regression the migration wave eliminated (design doc: '9+ mesh types, 4 brep topologies… duplicate kind ids').",
        solution: `Import store::ArtifactChild<${typeName}> from stdio instead of redefining ${typeName} in ${relPath}, or if this is a deliberate, justified exception, add "${key}" to POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST citing why.`,
      });
    }
  }
  return breaches;
}

/** ⚖️ Aggregates the composition-specific policy rules (ticket
 * 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W6 ratchet, corrigendum-narrowed scope: canonical
 * artifact-kind grammar, child-slot composition-graph acyclicity, dissolved-kind redefinition ban).
 * `declare_artifact!` registration collapsing, `MeshExporter`/`MeshImporter` deletion, and
 * `📇️catalog.json` generation are explicitly CEDED to ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE — not this
 * function's scope; see the design doc's corrigendum. */
export function policyCompositionBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyCanonicalArtifactKindBreaches(repoRoot),
    ...policyChildSlotKindDagBreaches(repoRoot),
    ...policyDissolvedKindRedefinitionBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleComposition

//#region 🔧️PolicyRuleCleanMechanism
/**
 * 🧹 Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, wave W1 (allow-half): seven new
 * report-mode policies sizing the migration backlog the design doc's four rules imply — owner-mounted
 * children, subset isolation, module consumer counts, io exclusivity, io declaration reachability,
 * subset standalone-ness, and the plugin->artifact->standard->subset declaration tree. ALL land at
 * `priority: "medium"` (never `"high"`) — report mode, per the ticket: counted into the cache JSON
 * (`formatBreachReport`'s "full breach set (including non-blocking priorities)"), never failing
 * `runPolicyExit`'s high-priority gate. W6 is the wave that promotes any of these to blocking and
 * deletes the shapes they still tolerate (design.md's debt row D4).
 */

//#region 🔖️ModulePathSlug
/** 🔤️ General module-path slug (design.md §1 "Module path slugs"): strip the emoji prefix, kebab ->
 * snake, a leading digit gets an underscore prefix. Applies to module/artifact/subset dir names. */
function policyModulePathSlug(dirName: string): string {
  const stripped = policyStripEmoji(dirName).replace(/-/g, "_");
  return /^[0-9]/.test(stripped) ? `_${stripped}` : stripped;
}

/** 🔖️ Standard-slug variant (design.md §1): same kebab->snake folding, but prefixed `v` (digit-leading
 * raw slug) or `v_` (letter-leading raw slug) instead of the general rule's bare underscore prefix —
 * `🔖️1`->`v1`, `🔖️1.4`->`v1_4`, `🔖️ap214`->`v_ap214`, `🔖️ecma-376`->`v_ecma_376`. Idempotent: a raw
 * dir already in slug form (confirmed live today: `🧿️semio`'s `🔖️v1`/`🔖️v3`) is returned unchanged
 * instead of double-prefixed (`v_v1`) — verified against the real `standards::v1::` module path. */
function policyStandardModulePathSlug(dirName: string): string {
  const stripped = policyStripEmoji(dirName).replace(/[.\-]/g, "_");
  if (/^v[0-9]/.test(stripped) || /^v_/.test(stripped)) return stripped;
  return /^[0-9]/.test(stripped) ? `v${stripped}` : `v_${stripped}`;
}

/** 🧯️Escapes a literal string for embedding in a `RegExp` constructor. */
function policyEscapeRegExp(literal: string): string {
  return literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
//#endregion 🔖️ModulePathSlug

//#region 🔖️OwnerDiscovery
type PolicySubsetRel = {
  subsetRel: string;
  standardRel: string;
  artRel: string;
  pluginRel: string;
  subsetSlug: string;
  standardSlug: string;
  artSlug: string;
};

/** 🗂️ Splits a `policyListTopLevelSubsetDirs` row into its four owner levels plus the Rust module-path
 * slug (per [[policyModulePathSlug]]/[[policyStandardModulePathSlug]]) each level resolves to. */
function policySplitSubsetRel(subRel: string): PolicySubsetRel {
  const parts = subRel.split("/");
  // ✏️s / 🔌️plugins / <plugin> / 🗿️artifacts / <art> / 🏅️standards / 🔖️<std> / 🪆️subsets / ✳️<sub>
  const artDir = parts[4] ?? "";
  const standardDir = parts[6] ?? "";
  const subsetDir = parts[8] ?? "";
  return {
    subsetRel: subRel,
    standardRel: parts.slice(0, 7).join("/"),
    artRel: parts.slice(0, 5).join("/"),
    pluginRel: parts.slice(0, 3).join("/"),
    subsetSlug: policyModulePathSlug(subsetDir),
    standardSlug: policyStandardModulePathSlug(standardDir),
    artSlug: policyModulePathSlug(artDir),
  };
}

/** 🌳️ Every distinct (plugin, artifact, standard, subset) row discovered under ✏️s/🔌️plugins today. */
function policyAllSubsetSplits(repoRoot: string): PolicySubsetRel[] {
  return policyListTopLevelSubsetDirs(repoRoot).map(policySplitSubsetRel);
}

function policyListArtifactRels(splits: readonly PolicySubsetRel[]): string[] {
  return [...new Set(splits.map((s) => s.artRel))].sort();
}

function policyListStandardRels(splits: readonly PolicySubsetRel[]): string[] {
  return [...new Set(splits.map((s) => s.standardRel))].sort();
}

function policyListPluginRels(splits: readonly PolicySubsetRel[]): string[] {
  return [...new Set(splits.map((s) => s.pluginRel))].sort();
}

/** 🎯️ Nearest ancestor "owner root" of a repo-relative path — subset root, else standard root, else
 * artifact root, else the first three path segments (plugin/product root). Used to count DISTINCT
 * consumer roots for [[policyModuleConsumerCountBreaches]]. */
function policyNearestOwnerRoot(relPath: string): string {
  const subsetMatch = relPath.match(/^(.*\/🪆️subsets\/[^/]+)\//);
  if (subsetMatch) return subsetMatch[1]!;
  const standardMatch = relPath.match(/^(.*\/🏅️standards\/[^/]+)\//);
  if (standardMatch) return standardMatch[1]!;
  const artifactMatch = relPath.match(/^(.*\/🗿️artifacts\/[^/]+)\//);
  if (artifactMatch) return artifactMatch[1]!;
  return relPath.split("/").slice(0, 3).join("/");
}

/** 🔎️ Every directory literally named `dirName` under `roots`, not descending past a match (its own
 * children are enumerated by the caller, e.g. [[policyModuleConsumerCountBreaches]]'s module units). */
function policyFindDirsNamed(repoRoot: string, roots: readonly string[], dirName: string): string[] {
  const found: string[] = [];
  const walk = (rel: string): void => {
    for (const e of policyReaddirSafe(repoRoot, rel)) {
      if (!e.isDirectory) continue;
      const childRel = `${rel}/${e.name}`;
      if (e.name === dirName) {
        found.push(childRel);
        continue;
      }
      walk(childRel);
    }
  };
  for (const root of roots) walk(root);
  return found.sort();
}

const POLICY_CLEAN_MECHANISM_SCAN_ROOTS = ["✏️s/🔌️plugins", "✏️s/🔨️modules", "🧰️framework"];
const POLICY_CLEAN_MECHANISM_KIND = "clean-mechanism";
//#endregion 🔖️OwnerDiscovery

//#region 🔖️Policy1-OwnerMountsChildren
/** 🧭️ `#[path = "…"]` mount targets declared anywhere in `content` (order of first appearance). */
function policyExtractPathMountTargets(content: string): string[] {
  const targets: string[] = [];
  const re = /#\[path\s*=\s*"([^"]+)"\]/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) targets.push(m[1]!);
  return targets;
}

/** 🧭️ Child dir names of `ownerRel` that are BOTH real on disk AND legal per `allowedChildDirs`. */
function policyExistingLegalChildren(repoRoot: string, ownerRel: string, allowedChildDirs: readonly string[]): string[] {
  return policyReaddirSafe(repoRoot, ownerRel)
    .filter((e) => e.isDirectory && allowedChildDirs.includes(e.name))
    .map((e) => e.name);
}

/**
 * 🦀️ For every artifact/standard/subset root that HAS its own `🦀️component.rs`, its own `#[path]`
 * mounts must cover its existing, legal child dirs (design.md §1: "each level one root component.rs
 * that mounts its own children"). Most owners have no root component yet — `missing-owner-root` is
 * the migration backlog counter design.md's recipe (§5 step 6) still has to work through; the ones
 * that DO exist today (confirmed: ~30 artifact roots, one full standard+subset chain under
 * `🗄️stdio/🧊️gltf`) carry zero self-mounts because mounting is still centralized in `📦️glue.rs` — the
 * exact shape this ticket dissolves. Plugin `📦️glue.rs` may only `#[path]`-mount artifact roots,
 * plugin `🔨️modules`, `🎮️commands`, and its own plugin root; any deeper mount (a standard/subset/
 * schema/io leaf reached directly) is that old centralized-glue shape. `pub use …subsets::`/
 * `…standards::` lines in glue.rs are the sibling "shim" pattern — this scans for the pattern itself,
 * independent of the "Shims: keep pre-migration" text marker `policyNoGlueShimBlocksBreaches` already
 * polices.
 */
function policyOwnerMountsChildrenBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const splits = policyAllSubsetSplits(repoRoot);

  const checkOwnerRoot = (rel: string, level: "artifact" | "standard" | "subset", allowedChildDirs: readonly string[]): void => {
    const rootFile = `${rel}/${POLICY_RS_COMPONENT_LEAF}`;
    if (!existsSync(join(repoRoot, rootFile))) {
      breaches.push({
        id: `owner-mounts-children-missing-root-${rel}`,
        summary: `"${rel}" has no root ${POLICY_RS_COMPONENT_LEAF} yet`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: rel,
        priority: "medium",
        reason: `design.md §1: every ${level} is one root component.rs that mounts its own children and exports one declaration function — this is the migration backlog counter.`,
        solution: `bun ./📜️script.ts new ${level} … to scaffold the root skeleton, then hand-author its mounts and declaration fn per the recipe (design.md §5).`,
      });
      return;
    }
    const content = readFileSync(join(repoRoot, rootFile), "utf8");
    const mountCount = policyExtractPathMountTargets(content).length;
    const existingChildren = policyExistingLegalChildren(repoRoot, rel, allowedChildDirs);
    if (existingChildren.length > 0 && mountCount === 0) {
      breaches.push({
        id: `owner-mounts-children-not-self-mounting-${rel}`,
        summary: `"${rootFile}" exists but does not #[path]-mount any of its ${existingChildren.length} child dir(s) (${existingChildren.join(", ")})`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: rel,
        priority: "medium",
        reason: "design.md §1: an owner root mounts its own children — mounting for this owner still lives centralized in the plugin's 📦️glue.rs.",
        solution: `Add #[path = "…"] pub mod … mounts for ${existingChildren.join(", ")} directly in ${rootFile}, and remove the equivalent nested mounts from 📦️glue.rs.`,
      });
    }
  };

  for (const rel of policyListArtifactRels(splits)) checkOwnerRoot(rel, "artifact", taxonomy.artifactChildDirs);
  for (const rel of policyListStandardRels(splits)) checkOwnerRoot(rel, "standard", taxonomy.standardChildDirs);
  for (const s of splits) checkOwnerRoot(s.subsetRel, "subset", taxonomy.subsetChildDirs);

  for (const pluginRel of policyListPluginRels(splits)) {
    const glueRel = `${pluginRel}/📦️packages/🦀️rust/📦️glue.rs`;
    if (!existsSync(join(repoRoot, glueRel))) continue;
    const content = readFileSync(join(repoRoot, glueRel), "utf8");
    const targets = policyExtractPathMountTargets(content);
    const outOfScope = targets.filter((t) => {
      if (t.includes("/🔨️modules/") || t.includes("/🎮️commands/")) return false;
      if (/\/🗿️artifacts\/[^/]+\/🦀️component\.rs$/.test(t)) return false;
      const isDeepMount = ["🗿️artifacts/", "🏅️standards/", "🪆️subsets/", "🧬️schema/", "🚪️io/", "👁️viewer/", "✏️editor/"].some((seg) => t.includes(seg));
      if (!isDeepMount && t.endsWith(POLICY_RS_COMPONENT_LEAF)) return false; // the plugin's own root mount
      return true;
    });
    if (outOfScope.length > 0) {
      breaches.push({
        id: `owner-mounts-children-glue-scope-${pluginRel}`,
        summary: `"${glueRel}" #[path]-mounts ${outOfScope.length} item(s) beyond artifact roots/🔨️modules/🎮️commands/plugin root (still centralized)`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: pluginRel,
        priority: "medium",
        reason: "design.md §1: a plugin's 📦️glue.rs mounts ONLY artifact roots, plugin modules, commands, and the plugin root — everything below an artifact root is that artifact's own job to mount.",
        solution: `Move each deep mount (first example: "${outOfScope[0]}") down into its owning artifact/standard/subset root's own #[path] list.`,
      });
    }
    const shimHits = [...content.matchAll(/pub use\s+[^;]*::(?:subsets|standards)::[^;]*;/g)];
    if (shimHits.length > 0) {
      breaches.push({
        id: `owner-mounts-children-glue-shim-${pluginRel}`,
        summary: `"${glueRel}" carries ${shimHits.length} "pub use …subsets::"/"…standards::" shim re-export(s)`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: pluginRel,
        priority: "medium",
        reason: "design.md §5 step 9: delete every 📦️glue.rs shim that re-exports a sibling subset/standard path — the owning root should export it directly.",
        solution: `Delete the shim re-export(s) (first example: "${shimHits[0]![0]}") once the owning root exports the symbol directly.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔖️Policy1-OwnerMountsChildren

//#region 🔖️Policy2-SubsetIsolation
/**
 * 🪆️ A file under a subset's own tree must not reach into a SIBLING subset or a DIFFERENT standard of
 * its own artifact (design.md rule 2: "a subset never uses a sibling subset or another standard"),
 * and a TS file must never `import` its way above its own subset root. `modules::`, framework crates
 * (`semio_framework*`), and other plugin crates (`semio_s_plugin_*`) stay legal — only same-artifact
 * cross-standard/cross-subset reach-through and a subset-escaping relative import are flagged.
 */
function policySubsetIsolationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const split of policyAllSubsetSplits(repoRoot)) {
    const files = policyWalkRelFiles(repoRoot, [split.subsetRel], (_p, name) => name.endsWith(".rs") || name.endsWith(".ts"));
    const crossStandardRe = new RegExp(`artifacts::${policyEscapeRegExp(split.artSlug)}::standards::(?!${policyEscapeRegExp(split.standardSlug)}\\b)(\\w+)`, "g");
    const crossSubsetRe = new RegExp(`artifacts::${policyEscapeRegExp(split.artSlug)}::standards::${policyEscapeRegExp(split.standardSlug)}::subsets::(?!${policyEscapeRegExp(split.subsetSlug)}\\b)(\\w+)`, "g");
    for (const relPath of files) {
      const content = policyReadFileSafe(repoRoot, relPath);
      if (relPath.endsWith(".rs")) {
        const crossSubsetTargets = new Set<string>();
        for (const m of content.matchAll(crossSubsetRe)) {
          crossSubsetTargets.add(m[1]!);
          const line = content.slice(0, m.index).split("\n").length;
          breaches.push({
            id: `subset-isolation-cross-subset-${relPath}-${m.index}`,
            summary: `"${relPath}:${line}" reaches into sibling subset "${m[1]}" (own subset is "${split.subsetSlug}")`,
            kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-isolation`,
            scope: split.subsetRel,
            line,
            priority: "medium",
            reason: "design.md rule 2: a subset never uses a sibling subset.",
            solution: `Extract the shared code into a 🔨️modules owner at the lowest common level (design.md rule 4) instead of reaching into subsets::${m[1]}.`,
          });
        }
        for (const m of content.matchAll(crossStandardRe)) {
          if (m[1] === split.standardSlug || crossSubsetTargets.has(m[1]!)) continue; // own standard (post-lookahead false match) or already counted as a cross-subset hit at the same span
          const line = content.slice(0, m.index).split("\n").length;
          breaches.push({
            id: `subset-isolation-cross-standard-${relPath}-${m.index}`,
            summary: `"${relPath}:${line}" reaches into standard "${m[1]}" of its own artifact "${split.artSlug}" (own standard is "${split.standardSlug}")`,
            kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-isolation`,
            scope: split.subsetRel,
            line,
            priority: "medium",
            reason: "design.md rule 2: a subset never reaches into another standard of its own artifact.",
            solution: `Extract the shared code into a 🔨️modules owner at the lowest common level (design.md rule 4) instead of reaching across standards::${m[1]}.`,
          });
        }
      } else {
        const depth = relPath.slice(split.subsetRel.length + 1).split("/").length - 1;
        for (const m of content.matchAll(/from\s+["'](\.\.[^"']*)["']/g)) {
          const spec = m[1]!;
          const upCount = (spec.match(/\.\.\//g) ?? []).length;
          if (upCount <= depth) continue;
          // 🔨️ design.md rule 2's "Allowed: modules::, framework crates, other plugin crates" applies to
          // TS the same way it does to Rust — resolve the spec against the file's real dir and allow it
          // through when it lands in a 🔨️modules owner, a different plugin's own tree, or 🧰️framework.
          const resolved = join(dirname(join(repoRoot, relPath)), spec).replaceAll("\\", "/");
          const resolvedRel = relative(repoRoot, resolved).replaceAll("\\", "/");
          if (/(^|\/)🔨️modules(\/|$)/.test(resolvedRel) || resolvedRel.startsWith("🧰️framework/") || (resolvedRel.startsWith("✏️s/🔌️plugins/") && !resolvedRel.startsWith(split.pluginRel + "/"))) continue;
          const line = content.slice(0, m.index).split("\n").length;
          breaches.push({
            id: `subset-isolation-ts-climb-${relPath}-${m.index}`,
            summary: `"${relPath}:${line}" imports "${spec}", which climbs above its own subset root "${split.subsetRel}"`,
            kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-isolation`,
            scope: split.subsetRel,
            line,
            priority: "medium",
            reason: "design.md rule 2 (TS half): no relative import may leave its own subset dir, except into a 🔨️modules owner, 🧰️framework, or another plugin.",
            solution: `Replace the relative import with an absolute import, or move the shared code into a 🔨️modules owner (design.md rule 4).`,
          });
        }
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy2-SubsetIsolation

//#region 🔖️Policy3-ModuleConsumerCount
/**
 * 🔨️ Every `🔨️modules/<m>` needs ≥2 distinct consumer roots (design.md rule 4: "≥2 distinct consumer
 * roots ... One consumer ⇒ inline it"); a subset-level `🔨️modules` is ALWAYS a breach — confirmed live
 * today at `🗄️stdio/🧊️gltf/…/✳️any/🔨️modules` (design.md §4's own "gltf subset 🔨️modules/*" debt row).
 */
function policyModuleConsumerCountBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const modulesDirs = policyFindDirsNamed(repoRoot, POLICY_CLEAN_MECHANISM_SCAN_ROOTS, "🔨️modules");
  for (const modulesDirRel of modulesDirs) {
    const isSubsetLevel = /\/🪆️subsets\/[^/]+\/🔨️modules$/.test(modulesDirRel);
    const units = policyReaddirSafe(repoRoot, modulesDirRel).filter((e) => e.isDirectory);
    for (const unit of units) {
      const unitRel = `${modulesDirRel}/${unit.name}`;
      if (isSubsetLevel) {
        breaches.push({
          id: `module-consumer-subset-level-${unitRel}`,
          summary: `"${unitRel}" is a subset-level 🔨️modules — always forbidden`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/module-consumer-count`,
          scope: unitRel,
          priority: "medium",
          reason: "design.md rule 4: no subset-level 🔨️modules; shared code lives at the lowest common owner at or above standard level.",
          solution: `Lift "${unit.name}" to the owning standard's/artifact's 🔨️modules (or inline it if it truly has one consumer).`,
        });
        continue;
      }
      const slug = policyModulePathSlug(unit.name);
      const token = `modules::${slug}`;
      const searchRoots = modulesDirRel.startsWith("✏️s/🔌️plugins/") ? [modulesDirRel.split("/").slice(0, 3).join("/")] : POLICY_CLEAN_MECHANISM_SCAN_ROOTS;
      const matchFiles = policyWalkRelFiles(repoRoot, searchRoots, (relPath, name) => name.endsWith(".rs") && !relPath.startsWith(`${unitRel}/`) && policyReadFileSafe(repoRoot, relPath).includes(token));
      const consumerRoots = new Set(matchFiles.map((f) => policyNearestOwnerRoot(f)));
      if (consumerRoots.size < 2) {
        breaches.push({
          id: `module-consumer-count-${unitRel}`,
          summary: `"${unitRel}" (module path "${token}") has ${consumerRoots.size} distinct consumer root(s), needs >=2`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/module-consumer-count`,
          scope: unitRel,
          priority: "medium",
          reason: "design.md rule 4: a 🔨️modules member needs ≥2 distinct production consumers or it is inlined.",
          solution:
            consumerRoots.size === 0
              ? `"${unit.name}" has no discovered consumer at all — inline it back into its sole intended caller, or delete it if dead.`
              : `"${unit.name}" has exactly one consumer (${[...consumerRoots][0]}) — inline it there, or find/add its second real consumer.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy3-ModuleConsumerCount

//#region 🔖️Policy4-IoExclusivity
const POLICY_IO_EXCLUSIVITY_PATTERNS: readonly { re: RegExp; label: string }[] = [
  { re: /\bparse_dsl\s*\(/, label: "parse_dsl(" },
  { re: /\bprint_dsl\s*\(/, label: "print_dsl(" },
  { re: /\bencode_pack\s*\(/, label: "encode_pack(" },
  { re: /\bdecode_pack\s*\(/, label: "decode_pack(" },
  { re: /\bArtifactDsl::/, label: "ArtifactDsl::" },
  { re: /\bArtifactPack::/, label: "ArtifactPack::" },
  { re: /include_bytes!/, label: "include_bytes!" },
  { re: /\bstd::fs::/, label: "std::fs::" },
  { re: /\bsemio_s_plugin_\w+::[\w:]*\bio\b/, label: "semio_s_plugin_<other>::…::io" },
];

/** ✂️ Strips top-level `#[cfg(test)] mod … { … }` bodies via brace matching, so test-only code never
 * trips the io-exclusivity scan (design.md §3 "Exclusivity … and #[cfg(test)]"). */
function policyStripCfgTestModules(content: string): string {
  const marker = "#[cfg(test)]";
  let out = "";
  let i = 0;
  while (i < content.length) {
    const idx = content.indexOf(marker, i);
    if (idx === -1) {
      out += content.slice(i);
      break;
    }
    out += content.slice(i, idx);
    const braceStart = content.indexOf("{", idx);
    if (braceStart === -1) {
      out += content.slice(idx);
      break;
    }
    let depth = 0;
    let j = braceStart;
    for (; j < content.length; j++) {
      if (content[j] === "{") depth++;
      else if (content[j] === "}") {
        depth--;
        if (depth === 0) {
          j++;
          break;
        }
      }
    }
    i = j;
  }
  return out;
}

/**
 * 🚪️ Outside `🚪️io/**` and `#[cfg(test)]`/`🧪️tests`, native codec/raw-fs primitives are policy
 * breaches — design.md §3 "Exclusivity". `serde_json` is explicitly NOT banned (it is the UI/command
 * protocol — 741 legitimate call sites per the ticket).
 */
function policyIoExclusivityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, POLICY_CLEAN_MECHANISM_SCAN_ROOTS, (relPath, name) => name.endsWith(".rs") && !relPath.includes("/🚪️io/") && !relPath.includes("/🧪️tests/"));
  for (const relPath of files) {
    const content = policyStripCfgTestModules(policyReadFileSafe(repoRoot, relPath));
    for (const { re, label } of POLICY_IO_EXCLUSIVITY_PATTERNS) {
      const m = re.exec(content);
      if (!m) continue;
      const line = content.slice(0, m.index).split("\n").length;
      breaches.push({
        id: `io-exclusivity-${relPath}-${label}`,
        summary: `"${relPath}:${line}" uses "${label}" outside 🚪️io/**`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/io-exclusivity`,
        scope: relPath,
        line,
        priority: "medium",
        reason: "design.md §3: all IO — including the native DSL grammar and pack binary protocol — goes exclusively through the io system's Serializer/Deserializer entries.",
        solution: `Move this call behind an io_route()/io_run() call, or relocate the codec logic under 🚪️io/.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔖️Policy4-IoExclusivity

//#region 🔖️Policy5-IoDeclaration
/**
 * 🚪️ Every `🚪️io` codec leaf dir (one carrying its own `🦀️component.rs`) must be referenced BY NAME
 * from its subset's `🚪️io/🦀️component.rs` root, and carry a `🟦️component.ts` twin (design.md §1:
 * "io() -> IoDeclaration / IoEntryDescriptor[] mirror").
 */
function policyIoDeclarationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const split of policyAllSubsetSplits(repoRoot)) {
    const ioRel = `${split.subsetRel}/🚪️io`;
    if (!existsSync(join(repoRoot, ioRel))) continue;
    const ioRootFile = `${ioRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const ioRootContent = policyReadFileSafe(repoRoot, ioRootFile);
    const leafFiles = policyWalkRelFiles(repoRoot, [ioRel], (relPath, name) => name === POLICY_RS_COMPONENT_LEAF && relPath !== ioRootFile);
    for (const leafFile of leafFiles) {
      const leafDirRel = leafFile.slice(0, leafFile.length - `/${POLICY_RS_COMPONENT_LEAF}`.length);
      const leafName = leafDirRel.split("/").pop() ?? "";
      if (!ioRootContent.includes(leafName)) {
        breaches.push({
          id: `io-declaration-unreachable-${leafFile}`,
          summary: `"${leafDirRel}" is not referenced by name from "${ioRootFile}"`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/io-declaration`,
          scope: split.subsetRel,
          priority: "medium",
          reason: "design.md §1: io() -> IoDeclaration must reach every codec leaf; an unreferenced leaf is dead weight the io registry never sees.",
          solution: `Mount/reference "${leafName}" from ${ioRootFile}, or delete the leaf if it is genuinely unused.`,
        });
      }
      if (!existsSync(join(repoRoot, leafDirRel, "🟦️component.ts"))) {
        breaches.push({
          id: `io-declaration-missing-ts-twin-${leafDirRel}`,
          summary: `"${leafDirRel}" has no 🟦️component.ts twin`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/io-declaration`,
          scope: split.subsetRel,
          priority: "medium",
          reason: "design.md §1: 🚪️io/🟦️component.ts exports the IoEntryDescriptor[] mirror — every Rust codec leaf needs its TS twin.",
          solution: `Add ${leafDirRel}/🟦️component.ts mirroring the Rust leaf's IoEntryDescriptor row(s).`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy5-IoDeclaration

//#region 🔖️Policy6-SubsetStandalone
/**
 * 🪆️ A subset's `🧬️schema/🦀️component.rs` must not be a bare `pub use …subsets::<other>::*`
 * re-export of a sibling subset, and the subset's `🧬️schema/**` subtree must declare its OWN
 * `…Snapshot` struct somewhere (design.md rule 2: "own snapshot/diff/mutations/inferences types").
 */
function policySubsetStandaloneBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const split of policyAllSubsetSplits(repoRoot)) {
    const schemaRootFile = `${split.subsetRel}/🧬️schema/${POLICY_RS_COMPONENT_LEAF}`;
    if (!existsSync(join(repoRoot, schemaRootFile))) continue;
    const rootContent = readFileSync(join(repoRoot, schemaRootFile), "utf8");
    for (const m of rootContent.matchAll(/pub use\s+[\w:]*subsets::(\w+)[\w:]*::\*/g)) {
      const other = m[1]!;
      if (other === split.subsetSlug) continue;
      const line = rootContent.slice(0, m.index).split("\n").length;
      breaches.push({
        id: `subset-standalone-bare-reexport-${schemaRootFile}-${m.index}`,
        summary: `"${schemaRootFile}:${line}" is a bare re-export of sibling subset "${other}"`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-standalone`,
        scope: split.subsetRel,
        line,
        priority: "medium",
        reason: "design.md rule 2: every subset is a complete standalone implementation, never a bare re-export of another subset's schema.",
        solution: `Give "${split.subsetSlug}" its own snapshot/diff/mutation types (built from shared 🔨️modules types where genuinely shared), instead of re-exporting subsets::${other}::*.`,
      });
    }
    const schemaFiles = policyWalkRelFiles(repoRoot, [`${split.subsetRel}/🧬️schema`], (_p, name) => name.endsWith(".rs"));
    const hasOwnSnapshot = schemaFiles.some((f) => /(?:^|[^\w])struct\s+\w*Snapshot\b/.test(policyReadFileSafe(repoRoot, f)));
    if (!hasOwnSnapshot) {
      breaches.push({
        id: `subset-standalone-no-own-snapshot-${split.subsetRel}`,
        summary: `"${split.subsetRel}/🧬️schema" declares no own "…Snapshot" struct`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-standalone`,
        scope: split.subsetRel,
        priority: "medium",
        reason: "design.md rule 2: every subset owns its own snapshot type.",
        solution: `Define a "…Snapshot" struct somewhere under ${split.subsetRel}/🧬️schema, or if genuinely derived, a Deserializer FROM the base dialect (design.md §5 step 4) rather than a bare re-export.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔖️Policy6-SubsetStandalone

//#region 🔖️Policy7-DeclarationTree
/**
 * 🌳️ Where the NEW-shape declaration functions exist (design.md §1: `pub fn artifact() ->
 * ArtifactDeclaration`, `pub fn standard() -> StandardDeclaration`, `pub fn subset() ->
 * SubsetDeclaration`), the parent level must register every child that owns one, matching
 * `🪆️subsets/🔣️component.json`. Dormant on today's tree — a repo-wide grep confirmed these exact
 * zero-arg signatures do not exist anywhere yet (the SDK's `ArtifactDeclaration` today is still the
 * OLD `ArtifactDeclarationBuilder` shape, per `🔌️plugin/🦀️component.rs`) — verified structurally sound
 * so it fires correctly once a W2+ packet lands the first real declaration tree, the same precedent
 * `policyContributedSurfaceTargetBreaches` already set for a dormant-but-ready rule.
 */
function policyDeclarationTreeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const splits = policyAllSubsetSplits(repoRoot);
  for (const split of splits) {
    const artFile = `${split.artRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const stdFile = `${split.standardRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const subFile = `${split.subsetRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const artContent = policyReadFileSafe(repoRoot, artFile);
    const stdContent = policyReadFileSafe(repoRoot, stdFile);
    const subContent = policyReadFileSafe(repoRoot, subFile);
    if (/pub fn artifact\(\)\s*->\s*ArtifactDeclaration/.test(artContent) && /pub fn standard\(\)\s*->\s*StandardDeclaration/.test(stdContent) && !artContent.includes(split.standardSlug)) {
      breaches.push({
        id: `declaration-tree-artifact-missing-standard-${split.artRel}-${split.standardSlug}`,
        summary: `"${artFile}" declares artifact() but never references standard "${split.standardSlug}"`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/declaration-tree`,
        scope: split.artRel,
        priority: "medium",
        reason: "design.md §1: artifact() registers every standard it owns.",
        solution: `Add "${split.standardSlug}"'s standard() to the artifact()'s .standards vec in ${artFile}.`,
      });
    }
    if (/pub fn standard\(\)\s*->\s*StandardDeclaration/.test(stdContent) && /pub fn subset\(\)\s*->\s*SubsetDeclaration/.test(subContent) && !stdContent.includes(split.subsetSlug)) {
      breaches.push({
        id: `declaration-tree-standard-missing-subset-${split.standardRel}-${split.subsetSlug}`,
        summary: `"${stdFile}" declares standard() but never references subset "${split.subsetSlug}"`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/declaration-tree`,
        scope: split.standardRel,
        priority: "medium",
        reason: "design.md §1: standard() registers every subset it owns, matching 🪆️subsets/🔣️component.json.",
        solution: `Add "${split.subsetSlug}"'s subset() to the standard()'s .subsets vec in ${stdFile}.`,
      });
    }
  }
  for (const pluginRel of policyListPluginRels(splits)) {
    const glueRel = `${pluginRel}/📦️packages/🦀️rust/📦️glue.rs`;
    const glueContent = policyReadFileSafe(repoRoot, glueRel);
    if (!glueContent) continue;
    for (const artRel of policyListArtifactRels(splits.filter((s) => s.pluginRel === pluginRel))) {
      const artContent = policyReadFileSafe(repoRoot, `${artRel}/${POLICY_RS_COMPONENT_LEAF}`);
      if (!/pub fn artifact\(\)\s*->\s*ArtifactDeclaration/.test(artContent)) continue;
      const artSlug = policyModulePathSlug(artRel.split("/").pop() ?? "");
      if (!glueContent.includes(".artifact(") || !glueContent.includes(artSlug)) {
        breaches.push({
          id: `declaration-tree-plugin-missing-artifact-${pluginRel}-${artSlug}`,
          summary: `"${glueRel}" does not register artifact "${artSlug}" via .artifact(…)`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/declaration-tree`,
          scope: pluginRel,
          priority: "medium",
          reason: "design.md §2: PluginBuilder<Ready>.artifact(...) is the ONLY registration channel.",
          solution: `Call .artifact(${artSlug}::artifact()) from ${glueRel}'s Plugin::builder chain.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy7-DeclarationTree

/** ⚖️ Aggregates the seven ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1 report-mode
 * policies (task 2) — ALL land at `"medium"` priority; see the region docstring above. */
export function policyCleanArtifactStandardSubsetMechanismBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyOwnerMountsChildrenBreaches(repoRoot),
    ...policySubsetIsolationBreaches(repoRoot),
    ...policyModuleConsumerCountBreaches(repoRoot),
    ...policyIoExclusivityBreaches(repoRoot),
    ...policyIoDeclarationBreaches(repoRoot),
    ...policySubsetStandaloneBreaches(repoRoot),
    ...policyDeclarationTreeBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleCleanMechanism

/** 📦️ Shape V2 package-folder purity — language-neutral assets must not live inside `📦️packages/<lang>/`. */
const POLICY_PACKAGE_PURITY_PRIORITY = "high" as const;

export function policyPackageLanguagePurityBreaches(repoRoot: string): BreachRecord[] {
  const problems = discoverPackageProblems(repoRoot, loadTaxonomy());
  return problems
    .filter((problem) => problem.kind === "packaging-violation" || problem.kind === "unknown-lang")
    .map((problem) => ({
      id: `package-purity-${problem.path.replaceAll("/", "-")}`,
      summary: problem.message,
      kind: "taxonomy/package-purity",
      scope: problem.path,
      priority: POLICY_PACKAGE_PURITY_PRIORITY,
      reason: "Shape V2: 📦️packages/<lang>/ holds ONLY packaging code; 📦️packages/ children must be declared langs.",
      solution: "Hoist language-neutral assets to the owner root beside 📦️packages/.",
    }));
}

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
  breaches.push(...policyWindowCompletenessBreaches(repoRoot, crateDirs));
  breaches.push(...policyModeCompletenessBreaches(repoRoot, crateDirs));
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
  breaches.push(...policyApaBreaches(repoRoot));
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
  breaches.push(...policyMutationOutcomeMergePolicyBreaches(repoRoot));
  breaches.push(...policyPluginDependencyParityBreaches(repoRoot));
  breaches.push(...policyContributionTargetBreaches(repoRoot));
  breaches.push(...policyInferenceFamilyBreaches(repoRoot));
  breaches.push(...policySchemaOverhaulS2Breaches(repoRoot));
  breaches.push(...policySchemaOverhaulPCBreaches(repoRoot));
  breaches.push(...policyDissolvedKernelsBreaches(repoRoot));
  breaches.push(...policyCompositionBreaches(repoRoot));
  breaches.push(...policyHandcraftedSpecP3Breaches(repoRoot));
  breaches.push(...policyArtifactSchemaBreaches(repoRoot));
  breaches.push(...policyAppSchemaBreaches(repoRoot));
  breaches.push(...policyStdioArtifactsBreaches(repoRoot));
  breaches.push(...policyProtocolMigrationBreaches(repoRoot));
  breaches.push(...policyDbServerOnlyBreaches(repoRoot));
  breaches.push(...policyOsStateAuthorityBreaches(repoRoot));
  breaches.push(...policyDocumentAppShapeBreaches(repoRoot));
  breaches.push(...policyNoPackFilesBreaches(repoRoot));
  breaches.push(...policyRawSpawnBreaches(repoRoot));
  breaches.push(...policyBudgetNullBreaches(repoRoot));
  breaches.push(...policyMcpConfigBreaches(repoRoot));
  breaches.push(...policySniffRealityBreaches(repoRoot));
  breaches.push(...policySubsetConformanceBreaches(repoRoot));
  breaches.push(...policySubsetSurfaceCompletenessBreaches(repoRoot));
  breaches.push(...policyViewerPurityBreaches(repoRoot));
  breaches.push(...policyContributedSurfaceTargetBreaches(repoRoot));
  breaches.push(...policyOsConfigShapeBreaches(repoRoot));
  breaches.push(...policyCleanArtifactStandardSubsetMechanismBreaches(repoRoot));
  breaches.push(...policyPackageLanguagePurityBreaches(repoRoot));
  return breaches;
});
//#endregion 🔖️PolicyExport
//#endregion 🔖️Policy

if (import.meta.main) {
  if (!(await dispatchPolicyArgv(process.argv.slice(2), import.meta.url))) {
    await runWorkspaceScriptMain(router);
  }
}
