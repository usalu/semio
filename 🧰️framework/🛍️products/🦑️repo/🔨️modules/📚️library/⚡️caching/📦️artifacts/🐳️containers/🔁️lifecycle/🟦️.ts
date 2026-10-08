/** 🔁️ The devcontainer's start and attach lifecycle (`.devcontainer/devcontainer.json` `postStartCommand` /
 * `postAttachCommand` → `bun ./📜️script.ts setup devcontainer start|attach`), replacing the retired Bash hooks with one
 * owned, testable implementation. Start: emoji font fallback, ownership of the persisted home volumes and submodules,
 * Claude Code auth storage, git `safe.directory` for the bind-mounted checkout, the SSH commit-signing agent. Attach: the
 * optional GUI tools (GitKraken Desktop + CLI, F3D), the GitKraken workspace, the git hooks, and the
 * workspace VS Code extension packaged through Nx and installed into every reachable editor CLI. Every external program
 * runs through an injected {@link LifecycleHost} (one argv, never a shell), so each step is exercised against recording
 * hosts on any machine; failures of optional steps are reported and never block the container.
 * https://containers.dev/implementors/json_reference/#lifecycle-scripts */
import { spawn, spawnSync } from "node:child_process";
import { closeSync, existsSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, renameSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

/** 📟️ What one external program answered. */
export type LifecycleAnswer = Readonly<{ status: number; stdout: string; stderr: string }>;

/** 🧩️ Everything the lifecycle touches outside plain files: programs, downloads, time and the log. */
export type LifecycleHost = Readonly<{
  run: (command: string, args: readonly string[], options?: Readonly<{ input?: string; cwd?: string; env?: Readonly<Record<string, string>> }>) => LifecycleAnswer;
  spawnDetached: (command: string, args: readonly string[], env: Readonly<Record<string, string>>) => void;
  download: (url: string, destination: string) => Promise<void>;
  fetchJson: (url: string) => Promise<unknown>;
  sleep: (milliseconds: number) => Promise<void>;
  log: (line: string) => void;
}>;

/** 🗂️ Where the lifecycle works: the checkout, the container user's home, its account, the environment and the CPU. */
export type LifecycleContext = Readonly<{ workspace: string; home: string; user: string; env: Readonly<Record<string, string | undefined>>; arch: string }>;

/** 🖥️ The real host: `spawnSync` without a shell, `fetch` for downloads, and stdout for the log. */
export function processLifecycleHost(): LifecycleHost {
  return {
    run: (command, args, options = {}) => {
      const answer = spawnSync(command, [...args], { encoding: "utf8", input: options.input, cwd: options.cwd, env: options.env ? { ...process.env, ...options.env } : process.env, shell: false });
      return { status: answer.error ? -1 : (answer.status ?? -1), stdout: answer.stdout ?? "", stderr: answer.error ? String(answer.error) : (answer.stderr ?? "") };
    },
    spawnDetached: (command, args, env) => spawn(command, [...args], { detached: true, stdio: "ignore", env: { ...process.env, ...env } }).unref(),
    download: async (url, destination) => {
      const response = await fetch(url, { signal: AbortSignal.timeout(300_000) });
      if (!response.ok) throw new Error(`download ${url} answered ${response.status}`);
      writeFileSync(destination, new Uint8Array(await response.arrayBuffer()));
    },
    fetchJson: async (url) => {
      const response = await fetch(url, { headers: { accept: "application/vnd.github+json" }, signal: AbortSignal.timeout(60_000) });
      if (!response.ok) throw new Error(`${url} answered ${response.status}`);
      return response.json();
    },
    sleep: (milliseconds) => new Promise((resolveSleep) => setTimeout(resolveSleep, milliseconds)),
    log: (line) => console.log(line),
  };
}

/** 🗂️ The context of the running container: `containerWorkspaceFolder` (else the repository root), `$HOME`, `$USER`. */
export function processLifecycleContext(repoRoot: string): LifecycleContext {
  return { workspace: process.env.containerWorkspaceFolder ?? repoRoot, home: process.env.HOME ?? "/home/vscode", user: process.env.USER ?? "vscode", env: process.env, arch: process.arch };
}

//#region 🔤️EmojiFonts
/** 🔤️ The fontconfig fallback that puts Noto Color Emoji behind every generic family, written to `/etc/fonts/local.conf`. */
export const EMOJI_FONTCONFIG = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>Noto Sans</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>serif</family>
    <prefer>
      <family>Noto Serif</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Noto Sans Mono</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <match target="pattern">
    <test name="lang">
      <string>en</string>
    </test>
    <test name="family">
      <string>emoji</string>
    </test>
    <edit name="family" mode="prepend">
      <string>Noto Color Emoji</string>
    </edit>
  </match>
  <match target="pattern">
    <test name="family">
      <string>Noto Color Emoji</string>
    </test>
    <edit name="fontformat" mode="assign">
      <string>TrueType</string>
    </edit>
    <edit name="scalable" mode="assign">
      <bool>true</bool>
    </edit>
  </match>
</fontconfig>
`;

/** 🔤️ Installs {@link EMOJI_FONTCONFIG} system-wide and rebuilds the font cache. */
export function configureEmojiFonts(host: LifecycleHost): boolean {
  const written = host.run("sudo", ["mkdir", "-p", "/etc/fonts"]).status === 0 && host.run("sudo", ["tee", "/etc/fonts/local.conf"], { input: EMOJI_FONTCONFIG }).status === 0;
  const cached = written && host.run("sudo", ["fc-cache", "-f"]).status === 0;
  host.log(cached ? "✅️ Emoji font fallback configured." : "⚠️  Emoji font fallback could not be configured.");
  return cached;
}
//#endregion 🔤️EmojiFonts

//#region 🔒️Ownership
/** 🔒️ The home directories the devcontainer persists in named volumes, which Docker creates owned by root. */
export const PERSISTED_HOME_DIRECTORIES = [".cache", ".claude", ".codex", ".config", ".codeium", ".gitkraken", ".local/share/GitKrakenCLI", ".local/share/gk", ".config/F3D", ".cursor-server", ".antigravity-server", ".vscode-server", ".windsurf-server", ".kiro"] as const;

/** 🧭️ The submodule paths `.gitmodules` declares, from git's own configuration parser. */
export function submodulePaths(host: LifecycleHost, workspace: string): string[] {
  if (!existsSync(join(workspace, ".gitmodules"))) return [];
  const answer = host.run("git", ["config", "-f", join(workspace, ".gitmodules"), "--get-regexp", "^submodule\\..*\\.path$"]);
  return answer.stdout.split("\n").map((line) => line.trim().split(/\s+/u).slice(1).join(" ")).filter(Boolean);
}

/** 🔒️ Hands every persisted home volume and every submodule checkout to the container user. */
export function ownPersistedState(host: LifecycleHost, context: LifecycleContext): void {
  for (const directory of PERSISTED_HOME_DIRECTORIES) host.run("sudo", ["chown", "-R", `${context.user}:${context.user}`, join(context.home, directory)]);
  for (const path of submodulePaths(host, context.workspace)) host.run("sudo", ["chown", "-R", `${context.user}:${context.user}`, join(context.workspace, path)]);
  host.log("✅️ Fixed ownership for persisted volume mounts and submodules.");
}
//#endregion 🔒️Ownership

//#region 🤖️ClaudeAuth
/** 🤖️ Keeps Claude Code's `~/.claude.json` (+ backup) inside the persisted `~/.claude` volume and links it back: a
 * plain file left at the home path moves into the volume unless the volume already holds one. */
export function normalizeClaudeAuth(home: string): void {
  const directory = join(home, ".claude");
  mkdirSync(directory, { recursive: true });
  for (const name of [".claude.json", ".claude.json.backup"]) {
    const stored = join(directory, name), link = join(home, name);
    const linkKind = existsSync(link) || isSymlink(link) ? (isSymlink(link) ? "symlink" : "file") : "absent";
    if (linkKind === "file") {
      if (existsSync(stored)) unlinkSync(link);
      else renameSync(link, stored);
    }
    if (existsSync(stored) && !existsSync(link) && !isSymlink(link)) symlinkSync(stored, link);
  }
}

function isSymlink(path: string): boolean {
  try {
    return lstatSync(path).isSymbolicLink();
  } catch {
    return false;
  }
}
//#endregion 🤖️ClaudeAuth

//#region 🛡️GitSafe
/** 🛡️ The checkout and submodule paths git must trust (the bind mount's owner differs from the container user) that
 * the user's global configuration does not list yet — so a restart never appends duplicates. */
export function missingSafeDirectories(listed: readonly string[], required: readonly string[]): string[] {
  const present = new Set(listed.map((path) => path.trim()));
  return [...new Set(required)].filter((path) => !present.has(path));
}

/** 🛡️ Adds the missing `safe.directory` entries to the container user's global git configuration. */
export function markSafeDirectories(host: LifecycleHost, context: LifecycleContext): void {
  const listed = host.run("git", ["config", "--global", "--get-all", "safe.directory"]).stdout.split("\n").filter(Boolean);
  const required = [context.workspace, ...submodulePaths(host, context.workspace).map((path) => join(context.workspace, path))];
  for (const path of missingSafeDirectories(listed, required)) host.run("git", ["config", "--global", "--add", "safe.directory", path]);
  host.log("✅️ Workspace and submodules are git safe.directory entries.");
}
//#endregion 🛡️GitSafe

//#region 🔐️SshSigning
/** 🔐️ The shell hook that makes every interactive shell reuse the signing agent. */
export const SSH_AGENT_SHELL_BLOCK = `
#region 🔐️SemioSshAgent
if [ -f "$HOME/.ssh/semio-ssh-agent.env" ]; then
  . "$HOME/.ssh/semio-ssh-agent.env" >/dev/null 2>&1 || true
fi
#endregion 🔐️SemioSshAgent
`;

/** 🔐️ `rc` with {@link SSH_AGENT_SHELL_BLOCK} appended exactly once. */
export function withSshAgentShellBlock(rc: string): string {
  return rc.includes("#region 🔐️SemioSshAgent") ? rc : `${rc}${SSH_AGENT_SHELL_BLOCK}`;
}

/** 🔐️ With `~/.ssh/id_ed25519_signing.pub` present: SSH commit/tag signing in the global git configuration, one agent on a
 * fixed socket (its environment in an owner-only file) and the shell hook; the key itself is unlocked by the person. */
export function configureSshSigning(host: LifecycleHost, context: LifecycleContext): boolean {
  const ssh = join(context.home, ".ssh"), key = join(ssh, "id_ed25519_signing"), publicKey = `${key}.pub`, socket = join(ssh, "semio-ssh-agent.sock"), environment = join(ssh, "semio-ssh-agent.env");
  if (!existsSync(publicKey)) {
    host.log("⚠️  SSH signing public key not found, skipping git SSH signing setup.");
    return false;
  }
  if (existsSync(key)) host.run("chmod", ["600", key]);
  host.run("chmod", ["644", publicKey]);
  for (const [name, value] of [["gpg.format", "ssh"], ["gpg.ssh.program", "ssh-keygen"], ["user.signingkey", publicKey], ["commit.gpgsign", "true"], ["tag.gpgsign", "true"]] as const) host.run("git", ["config", "--global", name, value]);
  const alive = existsSync(socket) && host.run("ssh-add", ["-l"], { env: { SSH_AUTH_SOCK: socket } }).status !== 2;
  if (!alive) {
    mkdirSync(ssh, { recursive: true, mode: 0o700 });
    rmSync(socket, { force: true });
    const agent = host.run("ssh-agent", ["-a", socket, "-s"]);
    const pid = /SSH_AGENT_PID=(\d+)/u.exec(agent.stdout)?.[1] ?? "";
    writeFileSync(environment, `export SSH_AUTH_SOCK=${socket}\nexport SSH_AGENT_PID=${pid}\n`, { mode: 0o600 });
  }
  const rc = join(context.home, ".bashrc");
  writeFileSync(rc, withSshAgentShellBlock(existsSync(rc) ? readFileSync(rc, "utf8") : ""));
  host.log("✅️ Configured SSH commit signing agent.");
  const fingerprint = host.run("ssh-keygen", ["-lf", publicKey]).stdout.split(/\s+/u)[1] ?? "";
  if (!fingerprint || !host.run("ssh-add", ["-l"], { env: { SSH_AUTH_SOCK: socket } }).stdout.includes(fingerprint)) host.log(`⚠️  Unlock signing once per container session with: ssh-add ${key}`);
  return true;
}
//#endregion 🔐️SshSigning

//#region 📦️Tools
/** 🧭️ The Debian/GitHub architecture names of `arch` (Node's `process.arch`) per tool, or null where the tool has no build. */
export function toolArchitectures(arch: string): Readonly<{ debian: string | null; f3d: string | null; gk: string | null }> {
  if (arch === "x64") return { debian: "amd64", f3d: "x86_64", gk: "amd64" };
  if (arch === "arm64") return { debian: "arm64", f3d: "arm64", gk: "arm64" };
  if (arch === "ia32") return { debian: null, f3d: null, gk: "386" };
  return { debian: null, f3d: null, gk: null };
}

/** 🔗️ The download URL of the first asset of a GitHub release whose name ends with `suffix`. */
export function releaseAssetUrl(release: unknown, suffix: string): string | null {
  const assets = (release as { assets?: readonly { name?: unknown; browser_download_url?: unknown }[] })?.assets ?? [];
  const asset = assets.find((row) => typeof row.name === "string" && row.name.endsWith(suffix) && typeof row.browser_download_url === "string");
  return asset ? String(asset.browser_download_url) : null;
}

function onPath(host: LifecycleHost, command: string): boolean {
  return host.run(command, ["--version"]).status === 0;
}

async function installDebianPackage(host: LifecycleHost, url: string, name: string): Promise<boolean> {
  const directory = mkdtempSync(join(tmpdir(), "semio-devcontainer-"));
  try {
    const file = join(directory, `${name}.deb`);
    await host.download(url, file);
    host.run("sudo", ["apt-get", "update"]);
    return host.run("sudo", ["apt-get", "install", "-y", file]).status === 0;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

/** 📦️ GitKraken Desktop from its Debian package, unless already on PATH. */
export async function installGitKrakenDesktop(host: LifecycleHost, context: LifecycleContext): Promise<void> {
  if (host.run("which", ["gitkraken"]).status === 0) return;
  const arch = toolArchitectures(context.arch).debian;
  if (!arch) return host.log(`⚠️  Skipping GitKraken Desktop install for unsupported architecture: ${context.arch}`);
  mkdirSync(join(context.home, ".gitkraken"), { recursive: true });
  host.log(await installDebianPackage(host, `https://release.gitkraken.com/linux/gitkraken-${arch}.deb`, "gitkraken") ? "GitKraken Desktop installed." : "⚠️  GitKraken Desktop install failed.");
}

/** 📦️ The GitKraken CLI `gk` from its latest GitHub release into `~/.local/bin`, unless one answers already. */
export async function installGitKrakenCli(host: LifecycleHost, context: LifecycleContext): Promise<void> {
  const local = join(context.home, ".local", "bin", "gk");
  if (host.run(local, ["--version"]).status === 0 || onPath(host, "gk")) return;
  const arch = toolArchitectures(context.arch).gk;
  if (!arch) return host.log(`⚠️  Skipping GitKraken CLI install for unsupported architecture: ${context.arch}`);
  const url = releaseAssetUrl(await host.fetchJson("https://api.github.com/repos/gitkraken/gk-cli/releases/latest"), `linux_${arch}.zip`);
  if (!url) return host.log(`⚠️  Unable to resolve GitKraken CLI download URL for architecture: ${arch}`);
  for (const directory of [join(context.home, ".local", "bin"), join(context.home, ".local", "share", "GitKrakenCLI"), join(context.home, ".local", "share", "gk")]) mkdirSync(directory, { recursive: true });
  const directory = mkdtempSync(join(tmpdir(), "semio-devcontainer-"));
  try {
    await host.download(url, join(directory, "gk.zip"));
    host.run("unzip", ["-q", join(directory, "gk.zip"), "-d", directory]);
    host.run("install", ["-m", "0755", join(directory, "gk"), local]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
  host.log(host.run(local, ["--version"]).status === 0 ? "GitKraken CLI installed." : "⚠️  GitKraken CLI install failed.");
}

/** 📦️ F3D from its latest GitHub release's Debian package (a pinned 3.4.1 package when the release names none). */
export async function installF3d(host: LifecycleHost, context: LifecycleContext): Promise<void> {
  if (onPath(host, "f3d")) return;
  const arch = toolArchitectures(context.arch).f3d;
  if (!arch) return host.log(`⚠️  Skipping F3D install for unsupported architecture: ${context.arch}`);
  const release = await host.fetchJson("https://api.github.com/repos/f3d-app/f3d/releases/latest").catch(() => null);
  const url = releaseAssetUrl(release, `Linux-${arch}.deb`) ?? `https://github.com/f3d-app/f3d/releases/download/v3.4.1/F3D-3.4.1-Linux-${arch}.deb`;
  host.log(await installDebianPackage(host, url, "f3d") ? "F3D installed." : "⚠️  F3D install failed.");
}
//#endregion 📦️Tools

//#region 🐙️GitKrakenWorkspace
/** 🐙️ What `gk ws info <name>` says about an existing workspace: absent, or present with the repositories it prints. */
export function gitKrakenWorkspaceState(info: string): "absent" | "present" {
  return !info.trim() || info.includes("you do not have any workspaces") || info.includes("no workspace with name") ? "absent" : "present";
}

/** 🐙️ Creates or completes the local GitKraken workspace (default `semio`, `SEMIO_GITKRAKEN_WORKSPACE_NAME`) from the
 * checkout and its submodules and makes it the default; skipped without an authenticated `gk`. */
export function configureGitKrakenWorkspace(host: LifecycleHost, context: LifecycleContext): void {
  if (!onPath(host, "gk")) return host.log("⚠️  GitKraken CLI not available, skipping GitKraken workspace bootstrap.");
  if (host.run("gk", ["auth", "status"]).status !== 0) return host.log("⚠️  GitKraken CLI not authenticated, skipping workspace bootstrap.");
  const name = context.env.SEMIO_GITKRAKEN_WORKSPACE_NAME || "semio";
  const repos = [context.workspace, ...submodulePaths(host, context.workspace).map((path) => join(context.workspace, path))].filter((path) => host.run("git", ["-C", path, "rev-parse", "--is-inside-work-tree"]).status === 0);
  if (!repos.length) return host.log("⚠️  No git repositories found for GitKraken workspace bootstrap.");
  const info = host.run("gk", ["ws", "info", name]).stdout;
  const missing = gitKrakenWorkspaceState(info) === "present" ? repos.filter((repo) => !info.includes(repo)) : repos;
  if (gitKrakenWorkspaceState(info) === "absent") host.run("gk", ["ws", "create", name, "--add-repos", repos.join(",")]);
  else if (missing.length) host.run("gk", ["ws", "update", name, "--add-repos", missing.join(",")]);
  if (missing.length) host.run("gk", ["ws", "refresh", name]);
  host.run("gk", ["ws", "set", name]);
  host.log(`✅️ GitKraken workspace ${missing.length ? "created or completed" : "already current"} and set as default: ${name}`);
}

/** 🐙️ Starts GitKraken Desktop on the checkout (a virtual display `:99` when none is set) unless it already runs. */
export function launchGitKraken(host: LifecycleHost, context: LifecycleContext): void {
  if (host.run("pgrep", ["-f", "gitkraken"]).stdout.trim()) return host.log("GitKraken is already running.");
  const display = context.env.DISPLAY || ":99";
  if (display === ":99" && host.run("pgrep", ["-f", "Xvfb.*:99"]).status !== 0) host.spawnDetached("Xvfb", [":99", "-screen", "0", "1920x1080x24", "-ac", "+extension", "GLX", "+render", "-noreset"], {});
  host.spawnDetached("gitkraken", ["--no-sandbox", "--no-debug", "--disable-gpu", "--disable-dev-shm-usage", "--path", context.workspace], { DISPLAY: display });
  host.log("GitKraken launched.");
}
//#endregion 🐙️GitKrakenWorkspace

//#region 🧩️Extension
/** 🧩️ The workspace extension's Nx package (`@semio-tech/repo-vscode`) and its packaged VSIX. */
export const WORKSPACE_EXTENSION = { target: "@semio-tech/repo-vscode:build-vsix", packageRoot: "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript", vsix: "🧩️repo.vsix" } as const;

/** 🧩️ The editor CLIs in preference order: the editor this attach came from first (its IPC hook or `TERM_PROGRAM`), then
 * every other known one. */
export function editorCliOrder(env: Readonly<Record<string, string | undefined>>): string[] {
  const order: string[] = [];
  const add = (cli: string): void => void (order.includes(cli) || order.push(cli));
  if (env.ANTIGRAVITY_IPC_HOOK_CLI || env.TERM_PROGRAM === "antigravity") add("antigravity");
  if (env.WINDSURF_IPC_HOOK_CLI || env.TERM_PROGRAM === "windsurf") add("windsurf");
  if (env.CURSOR_IPC_HOOK_CLI || env.TERM_PROGRAM === "cursor") add("cursor");
  if (env.VSCODE_IPC_HOOK_CLI || env.TERM_PROGRAM === "vscode") for (const cli of ["code", "code-insiders"]) add(cli);
  for (const cli of ["antigravity", "windsurf", "cursor", "code-insiders", "code"]) add(cli);
  return order;
}

/** 🧩️ The remote CLI binaries each editor's server ships under the home directory, newest server layout first. */
export function editorServerClis(home: string, cli: string): string[] {
  const servers: Readonly<Record<string, readonly string[]>> = { antigravity: [".antigravity-server"], windsurf: [".windsurf-server"], cursor: [".cursor-server"], code: [".vscode-server"], "code-insiders": [".vscode-server-insiders"] };
  return (servers[cli] ?? []).flatMap((server) => {
    const bin = join(home, server, "bin");
    return existsSync(bin) ? readdirSync(bin).map((build) => join(bin, build, "bin", "remote-cli", cli)).filter((path) => existsSync(path)) : [];
  });
}

/** 🧩️ Every editor CLI that answers `--version`: PATH entries in {@link editorCliOrder}, then the servers' remote CLIs. */
export function workingEditorClis(host: LifecycleHost, context: LifecycleContext): string[] {
  const clis: string[] = [];
  for (const cli of editorCliOrder(context.env)) {
    for (const path of host.run("which", ["-a", cli]).stdout.split("\n").filter(Boolean)) if (!clis.includes(path) && host.run(path, ["--version"]).status === 0) clis.push(path);
    for (const path of editorServerClis(context.home, cli)) if (!clis.includes(path) && host.run(path, ["--version"]).status === 0) clis.push(path);
  }
  return clis;
}

const EDITOR_UNREACHABLE = "Command is only available in WSL or inside a Visual Studio Code terminal.";

/** 🧩️ Packages the workspace extension through its Nx target and installs it into the first editor CLI that confirms it
 * in `--list-extensions`; one attach at a time (an exclusive lock file, waited on for up to 180 s). Answers the CLI. */
export async function installWorkspaceExtension(host: LifecycleHost, context: LifecycleContext, clis: readonly string[], lockPath = join(tmpdir(), "semio-devcontainer-extension.lock")): Promise<string | null> {
  if (!clis.length) {
    host.log("ℹ  No IDE CLIs detected, skipping extension installation.");
    return null;
  }
  let lock = -1;
  for (let waited = 0; lock < 0 && waited <= 180_000; waited += 1_000) {
    try {
      lock = openSync(lockPath, "wx");
    } catch {
      await host.sleep(1_000);
    }
  }
  if (lock < 0) {
    host.log("⚠️  Timed out waiting for the extension install lock, skipping extension install for this attach.");
    return null;
  }
  try {
    host.log("📦️ Resolving the extension package through Nx...");
    if (host.run("bun", ["nx", "run", WORKSPACE_EXTENSION.target], { cwd: context.workspace }).status !== 0) {
      host.log("⚠️  Extension packaging failed; skipping installation for this attach.");
      return null;
    }
    const root = join(context.workspace, WORKSPACE_EXTENSION.packageRoot), vsix = join(root, WORKSPACE_EXTENSION.vsix);
    if (!existsSync(vsix)) {
      host.log(`⚠️  Extension file not found at ${vsix}`);
      return null;
    }
    const manifest = JSON.parse(readFileSync(join(root, "package.json"), "utf8")) as { publisher?: string; name?: string };
    const id = manifest.publisher && manifest.name ? `${manifest.publisher}.${manifest.name}` : "";
    for (const cli of clis) {
      host.log(`📦️ Trying to install extension via ${cli}...`);
      const installed = host.run(cli, ["--install-extension", vsix, "--force"]);
      if (`${installed.stdout}${installed.stderr}`.includes(EDITOR_UNREACHABLE)) continue;
      const listed = id ? host.run(cli, ["--list-extensions"]) : installed;
      if (id && `${listed.stdout}${listed.stderr}`.includes(EDITOR_UNREACHABLE)) continue;
      if (!id || listed.stdout.split(/\r?\n/u).map((line) => line.trim()).includes(id)) {
        host.log(`✅️ Extension installed via ${cli}`);
        return cli;
      }
    }
    host.log(`⚠️  No IDE CLI could install the extension from this attach session; install ${vsix} manually.`);
    return null;
  } finally {
    closeSync(lock);
    rmSync(lockPath, { force: true });
  }
}
//#endregion 🧩️Extension

//#region 🔁️Lifecycle
/** 🔧️ Reinstalls the git hooks (micro-commit hooks; blocking pre-commit hooks are dropped) through the workspace script, which needs no built binary. */
export function configureRepoHooks(host: LifecycleHost, context: LifecycleContext): void {
  const installed = host.run("bun", ["./📜️script.ts", "micro-commit", "install-hooks"], { cwd: context.workspace });
  host.log(installed.status === 0 ? "✅️ Git hooks installed." : "⚠️  Git hook installation failed, continuing without blocking attach.");
}

/** 🔁️ `setup devcontainer start`: fonts, ownership, Claude auth storage, git trust, commit signing. */
export function devcontainerStart(host: LifecycleHost, context: LifecycleContext): void {
  ownPersistedState(host, context);
  configureEmojiFonts(host);
  normalizeClaudeAuth(context.home);
  host.log("✅️ Normalized Claude Code auth storage.");
  markSafeDirectories(host, context);
  configureSshSigning(host, context);
  host.log("✅️ Environment ready.");
}

/** 🔁️ `setup devcontainer attach`: optional GUI tools (`SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL`), the GitKraken workspace,
 * git hooks and the workspace extension (`SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL`). */
export async function devcontainerAttach(host: LifecycleHost, context: LifecycleContext): Promise<void> {
  const skipTools = Boolean(context.env.SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL);
  if (skipTools) host.log("ℹ  Tool installation skipped.");
  else for (const install of [installGitKrakenDesktop, installGitKrakenCli]) await install(host, context).catch((error: unknown) => host.log(`⚠️  ${String(error)}`));
  configureGitKrakenWorkspace(host, context);
  if (!skipTools) await installF3d(host, context).catch((error: unknown) => host.log(`⚠️  ${String(error)}`));
  configureRepoHooks(host, context);
  await installWorkspaceExtension(host, context, context.env.SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL ? [] : workingEditorClis(host, context));
  host.log("✅️ Post-attach setup complete.");
}
//#endregion 🔁️Lifecycle
