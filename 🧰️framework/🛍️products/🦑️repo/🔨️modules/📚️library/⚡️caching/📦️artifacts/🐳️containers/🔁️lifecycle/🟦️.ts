/** 🔁️ The devcontainer's start and attach lifecycle (`.devcontainer/devcontainer.json` `postStartCommand` /
 * `postAttachCommand` → `repo run workspace:setup-devcontainer-start|attach`), with one
 * owned, testable implementation. Start: emoji font fallback, ownership of the persisted home volumes and submodules,
 * Claude Code auth storage, git `safe.directory` for the bind-mounted checkout, the SSH commit-signing agent. Attach: the
 * optional F3D viewer. Every external program
 * runs through an injected {@link LifecycleHost} (one argv, never a shell), so each step is exercised against recording
 * hosts on any machine; failures of optional steps are reported and never block the container.
 * https://containers.dev/implementors/json_reference/#lifecycle-scripts */
import { spawnSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

/** 📟️ What one external program answered. */
export type LifecycleAnswer = Readonly<{ status: number; stdout: string; stderr: string }>;

/** 🧩️ Everything the lifecycle touches outside plain files: programs, downloads, time and the log. */
export type LifecycleHost = Readonly<{
  run: (command: string, args: readonly string[], options?: Readonly<{ input?: string; cwd?: string; env?: Readonly<Record<string, string>> }>) => LifecycleAnswer;
  download: (url: string, destination: string) => Promise<void>;
  fetchJson: (url: string) => Promise<unknown>;
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
export const PERSISTED_HOME_DIRECTORIES = [".cache", ".claude", ".codex", ".config", ".config/F3D"] as const;

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
export function toolArchitectures(arch: string): Readonly<{ debian: string | null; f3d: string | null }> {
  if (arch === "x64") return { debian: "amd64", f3d: "x86_64" };
  if (arch === "arm64") return { debian: "arm64", f3d: "arm64" };
  return { debian: null, f3d: null };
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

//#region 🔁️Lifecycle
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

/** 🔁️ Prepares the optional F3D viewer while developer control remains in the dashboard. */
export async function devcontainerAttach(host: LifecycleHost, context: LifecycleContext): Promise<void> {
  const skipTools = Boolean(context.env.SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL);
  if (skipTools) host.log("ℹ  Tool installation skipped.");
  if (!skipTools) await installF3d(host, context).catch((error: unknown) => host.log(`⚠️  ${String(error)}`));
  host.log("✅️ Post-attach setup complete.");
}
//#endregion 🔁️Lifecycle
