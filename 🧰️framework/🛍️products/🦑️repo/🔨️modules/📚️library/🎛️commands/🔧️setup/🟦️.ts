/** 🔧️ `setup` command family: native bootstrap, repo-local git settings, devcontainer lifecycle and postinstall.
 * @see 📜️script.ts
 */
import { NativeDependenciesScript } from "../../⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts";
import { ensureMcpBinary } from "../../../../../💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { dispatchSubcommand, runCmd, installMicroCommitGitHooks } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, linkSync, mkdirSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const NATIVE_BOOTSTRAP_DIR = "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap";

//#region 🔖️NativeOsScript
/** 🖥️Runs the native bootstrap under `repo/native/bootstrap`. */
export class NativeOsScript extends Script {
  run(segments: string[]): void {
    const cmd = segments[0] ?? "setup";
    const env = { ...process.env };
    if (process.platform === "win32") {
      const ps1 = join(this.root, NATIVE_BOOTSTRAP_DIR, "🔵️.ps1");
      if (!existsSync(ps1)) {
        console.error(`[native] missing ${ps1}; expected repo/native/bootstrap/🔵️.ps1.`);
        process.exit(1);
      }
      runCmd("powershell.exe", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", ps1, cmd], { cwd: this.root, env });
      return;
    }
    if (process.platform === "darwin" || process.platform === "linux") {
      const sh = join(this.root, NATIVE_BOOTSTRAP_DIR, "🐚️.sh");
      if (!existsSync(sh)) {
        console.error(`[native] missing ${sh}; expected repo/native/bootstrap/🐚️.sh.`);
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

//#region 🔖️SetupScript
/** 🔗️ Repo-relative instruction files `setup git` recreates as links to `AGENTS.md`, one per
 * client that cannot read `AGENTS.md` itself. `codex`, `cursor-chat`, `windsurf-chat`, `droid` and
 * `kiro-cli` read the canonical file directly and need no alias; `claude-code` reads `CLAUDE.md`,
 * `antigravity-chat` reads `GEMINI.md`, and `copilot-chat` reads only `.github/copilot-instructions.md`.
 * @see AGENTS.md */
export const AGENT_INSTRUCTION_ALIASES = ["CLAUDE.md", "GEMINI.md", ".github/copilot-instructions.md"] as const;

/** 🧷️ The repo-local git settings `setup git` writes into `.git/config`; the user's global git config is never
 * touched. `core.longpaths` lets Git for Windows check out, track and clean paths past `MAX_PATH` (tracked fixture paths
 * reach 210 UTF-16 units below the clone root; git ignores the key elsewhere) and `core.symlinks` materializes the
 * committed symlinks. A first clone on Windows needs `git clone -c core.longpaths=true`, which writes the same key.
 * @see README.md */
export const REPO_LOCAL_GIT_CONFIG = [
  ["core.symlinks", "true"],
  ["core.longpaths", "true"],
] as const;

export class SetupScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (!segments[0]) {
      this.runFull();
      return;
    }
    await dispatchSubcommand(
      segments,
      {
        postinstall: () => this.runPostinstall(),
        git: () => this.runGit(),
        native: (rest) => new NativeOsScript(this.root, this.repoRoot, this.invocation).run(rest),
        deps: (rest) => new NativeDependenciesScript(this.root, this.repoRoot, this.invocation).run(rest),
        prepare: () => console.log("[prepare] Build prerequisites completed"),
        devcontainer: (rest) => this.runDevcontainer(rest),
      },
      "bun ./📜️script.ts setup [postinstall|git|native|deps|prepare|devcontainer <start|attach|gitkraken>]",
    );
  }

  /** 🔁️ The devcontainer lifecycle `.devcontainer/devcontainer.json` runs: `start` on every container start, `attach` on
   * every editor attach, `gitkraken` to open GitKraken Desktop on the checkout. Loaded on demand, so no other verb pays for it.
   * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts */
  private async runDevcontainer(segments: string[]): Promise<void> {
    const lifecycle = await import("../../⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts");
    const host = lifecycle.processLifecycleHost(), context = lifecycle.processLifecycleContext(this.root);
    await dispatchSubcommand(
      segments,
      { start: () => lifecycle.devcontainerStart(host, context), attach: () => lifecycle.devcontainerAttach(host, context), gitkraken: () => lifecycle.launchGitKraken(host, context) },
      "bun ./📜️script.ts setup devcontainer <start|attach|gitkraken>",
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
    for (const [key, value] of REPO_LOCAL_GIT_CONFIG) runCmd("git", ["config", "--local", key, value], { cwd: this.root });
    installMicroCommitGitHooks(this.root);
    const source = "AGENTS.md";
    for (const alias of AGENT_INSTRUCTION_ALIASES) {
      const aliasPath = join(this.root, ...alias.split("/"));
      mkdirSync(dirname(aliasPath), { recursive: true });
      if (existsSync(aliasPath)) rmSync(aliasPath, { force: true });
      const target = relative(dirname(aliasPath), join(this.root, source)).replaceAll("\\", "/");
      try {
        symlinkSync(target, aliasPath, "file");
      } catch (error) {
        if (process.platform !== "win32") throw error;
        linkSync(join(this.root, source), aliasPath);
      }
    }
  }

  private runFull(): void {
    console.log("[setup] locked dependency environments are ready through the repository build graph");
    console.log("[setup] staging semio-os-mcp so `.mcp.json` initialize never waits on a cold build");
    ensureMcpBinary(this.root);
  }
}
//#endregion 🔖️SetupScript
