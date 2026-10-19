import assert from "node:assert/strict";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { EMOJI_FONTCONFIG, PERSISTED_HOME_DIRECTORIES, configureGitKrakenWorkspace, configureRepoHooks, devcontainerAttach, devcontainerStart, gitKrakenWorkspaceState, missingSafeDirectories, normalizeClaudeAuth, releaseAssetUrl, toolArchitectures, withSshAgentShellBlock, type LifecycleAnswer, type LifecycleContext, type LifecycleHost } from "../../🔁️lifecycle/🟦️.ts";

/** 🎙️ A host that records every program call and answers from `answer`, never running anything. */
function recordingHost(answer: (command: string, args: readonly string[]) => Partial<LifecycleAnswer> = () => ({})): LifecycleHost & { calls: string[][]; inputs: Map<string, string>; lines: string[] } {
  const calls: string[][] = [], inputs = new Map<string, string>(), lines: string[] = [];
  return {
    calls,
    inputs,
    lines,
    run: (command, args, options) => {
      calls.push([command, ...args]);
      if (options?.input !== undefined) inputs.set([command, ...args].join(" "), options.input);
      return { status: 0, stdout: "", stderr: "", ...answer(command, args) };
    },
    spawnDetached: (command, args) => void calls.push(["detached", command, ...args]),
    download: async () => undefined,
    fetchJson: async () => ({}),
    sleep: async () => undefined,
    log: (line) => void lines.push(line),
  };
}

/** 🔁️ The devcontainer start/attach lifecycle against its language-agnostic fixture: the fontconfig document (jsdom XML
 * oracle), architecture and release-asset selection (lodash oracle), dashboard attach, GitKraken workspace states and the
 * create / complete / set calls on a checkout with a submodule, duplicate-free `safe.directory`, Claude auth storage on a real temporary home, the idempotent shell hook, a start run
 * that never issues a destructive command, and dashboard-only attach through a recording host. */
export async function testDevcontainerLifecycle(workspace: string, generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🔁️lifecycle/🔣️.json"), "utf8"));
  await testDevcontainerDashboardOwnership(workspace);
  const { JSDOM } = require("jsdom"), lodash = require("lodash");
  const document = new JSDOM(EMOJI_FONTCONFIG, { contentType: "application/xml" }).window.document as Document;
  const aliases = Object.fromEntries([...document.querySelectorAll("fontconfig > alias")].map(alias => [alias.querySelector("family")!.textContent, [...alias.querySelectorAll("prefer > family")].map(family => family.textContent)]));
  assert.deepEqual(aliases, fixture.fontconfig.aliases);
  assert.ok([...document.querySelectorAll("fontconfig > match")].every(match => match.textContent?.includes(fixture.fontconfig.emojiFamily)));
  for (const row of fixture.architectures) assert.deepEqual(toolArchitectures(row.arch), { debian: row.debian, f3d: row.f3d, gk: row.gk }, row.arch);
  for (const row of fixture.releaseCases) {
    assert.equal(releaseAssetUrl(fixture.release, row.suffix), row.url, row.suffix);
    assert.equal(lodash.find(fixture.release.assets, (asset: { name: string }) => lodash.endsWith(asset.name, row.suffix))?.browser_download_url ?? null, row.url);
  }
  for (const row of fixture.gitKrakenInfo) assert.equal(gitKrakenWorkspaceState(row.info), row.state, row.info);
  for (const row of fixture.gitKrakenWorkspace.cases) {
    const checkout = mkdtempSync(join(generated, "gk-")), fill = (text: string) => text.replaceAll(`{workspace}/${fixture.gitKrakenWorkspace.submodule}`, join(checkout, fixture.gitKrakenWorkspace.submodule)).replaceAll("{workspace}", checkout);
    writeFileSync(join(checkout, ".gitmodules"), `[submodule "${fixture.gitKrakenWorkspace.submodule}"]\n\tpath = ${fixture.gitKrakenWorkspace.submodule}\n`);
    const host = recordingHost((command, args) => (command === "gk" && args[0] === "auth" ? { status: row.authenticated ? 0 : 1 } : command === "gk" && args[1] === "info" ? { stdout: fill(row.info) } : command === "git" && args.includes("--get-regexp") ? { stdout: `submodule.${fixture.gitKrakenWorkspace.submodule}.path ${fixture.gitKrakenWorkspace.submodule}\n` } : {}));
    configureGitKrakenWorkspace(host, { workspace: checkout, home: checkout, user: "vscode", env: { SEMIO_GITKRAKEN_WORKSPACE_NAME: row.name }, arch: "x64" });
    assert.deepEqual(host.calls.filter((call) => call[0] === "gk" && call[1] === "ws" && call[2] !== "info").map((call) => call.slice(1).join(" ")), row.expected.map(fill), row.id);
  }
  assert.deepEqual(missingSafeDirectories(fixture.safeDirectories.listed, fixture.safeDirectories.required), fixture.safeDirectories.missing);
  const once = withSshAgentShellBlock("export PATH=$PATH\n");
  assert.equal(withSshAgentShellBlock(once), once, "the signing-agent shell hook is appended once");

  for (const row of fixture.claudeAuth) {
    const home = mkdtempSync(join(generated, "auth-")), volume = join(home, ".claude");
    mkdirSync(volume, { recursive: true });
    for (const [name, content] of Object.entries(row.home)) writeFileSync(join(home, name), String(content));
    for (const [name, content] of Object.entries(row.volume)) writeFileSync(join(volume, name), String(content));
    normalizeClaudeAuth(home);
    normalizeClaudeAuth(home);
    for (const [name, content] of Object.entries(row.expected)) {
      assert.equal(readFileSync(join(volume, name), "utf8"), content, `${row.id}: ${name} lives in the volume`);
      assert.ok(lstatSync(join(home, name)).isSymbolicLink() && readlinkSync(join(home, name)) === join(volume, name), `${row.id}: ${name} is linked back`);
    }
    if (!Object.keys(row.expected).length) assert.equal(existsSync(join(home, ".claude.json")), false, row.id);
  }

  const home = mkdtempSync(join(generated, "home-")), checkout = mkdtempSync(join(generated, "ws-"));
  writeFileSync(join(checkout, ".gitmodules"), "");
  const start = recordingHost((command, args) => (command === "git" && args.includes("--get-regexp") ? { stdout: "submodule.recherche.path ♻️mit-bestand/🔎️recherche\n" } : command === "git" && args.includes("--get-all") ? { stdout: `${checkout}\n` } : {}));
  const context: LifecycleContext = { workspace: checkout, home, user: "vscode", env: {}, arch: "x64" };
  devcontainerStart(start, context);
  const flat = start.calls.map((call) => call.join(" "));
  for (const directory of PERSISTED_HOME_DIRECTORIES) assert.ok(flat.includes(`sudo chown -R vscode:vscode ${join(home, directory)}`), directory);
  const devcontainer = Bun.JSONC.parse(readFileSync(join(workspace, ".devcontainer/devcontainer.json"), "utf8")) as { remoteUser: string; mounts: string[] };
  const homeVolumes = devcontainer.mounts.map((mount) => /(?:^|,)target=([^,]+)/u.exec(mount)?.[1] ?? "").filter((target) => target.startsWith(`/home/${devcontainer.remoteUser}/`));
  for (const target of homeVolumes) assert.ok(PERSISTED_HOME_DIRECTORIES.some((directory) => `${target}/`.startsWith(`/home/${devcontainer.remoteUser}/${directory}/`)), `the persisted home volume ${target} is handed to the container user on start`);
  assert.ok(flat.includes(`sudo chown -R vscode:vscode ${join(checkout, "♻️mit-bestand/🔎️recherche")}`));
  assert.equal(start.inputs.get("sudo tee /etc/fonts/local.conf"), EMOJI_FONTCONFIG);
  assert.deepEqual(flat.filter((call) => call.startsWith("git config --global --add safe.directory")), [`git config --global --add safe.directory ${join(checkout, "♻️mit-bestand/🔎️recherche")}`]);
  assert.deepEqual(flat.filter((call) => fixture.forbiddenCommands.some((forbidden: string) => call.includes(forbidden))), [], "a container start never discards source-control or database state");

  for (const [status, message] of [[0, "✅️ Git hooks installed."], [1, "⚠️  Git hook installation failed, continuing without blocking attach."]] as const) {
    const hooks = recordingHost(() => ({ status }));
    configureRepoHooks(hooks, context);
    assert.deepEqual(hooks.calls, [["bun", "./📜️script.ts", "micro-commit", "install-hooks"]], "hooks are installed by the workspace script, never by an unbuilt binary");
    assert.deepEqual(hooks.lines, [message]);
  }

  console.log("✅️ Devcontainer lifecycle: fontconfig (jsdom XML), tool selection (lodash), dashboard-only attach, GitKraken states + workspace calls, safe.directory, Claude auth storage, start without destructive commands, IDE controls retired PASS");
}

/** 🎛️ Checks the language-neutral retirement fixture and records attach behavior with independent lodash predicates. */
export async function testDevcontainerDashboardOwnership(workspace: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🔁️lifecycle/🔣️.json"), "utf8")), lodash = createRequire(import.meta.url)("lodash");
  for (const path of fixture.dashboardOnly.removedPaths) assert.equal(existsSync(join(workspace, path)), false, path);
  const container = Bun.JSONC.parse(readFileSync(join(workspace, ".devcontainer/devcontainer.json"), "utf8"));
  assert.ok(container!==null&&typeof container==="object"&&!Array.isArray(container));
  assert.ok("containerEnv"in container&&container.containerEnv!==null&&typeof container.containerEnv==="object"&&!Array.isArray(container.containerEnv));
  assert.ok("mounts"in container&&Array.isArray(container.mounts)&&container.mounts.every((mount:unknown)=>typeof mount==="string"));
  assert.equal("customizations" in container, false);
  assert.equal("EDITOR" in container.containerEnv, false);
  const native = readFileSync(join(workspace, fixture.dashboardOnly.nativeBootstrap.path), "utf8");
  for (const token of fixture.dashboardOnly.nativeBootstrap.forbiddenTokens) assert.equal(native.includes(token), false, token);
  assert.equal(container.mounts.some((mount: string) => /(?:cursor|antigravity|vscode|windsurf)-server/u.test(mount)), false);
  for (const cli of fixture.dashboardOnly.forbiddenPrograms) {
    const attach = recordingHost((command) => command === "which" ? { stdout: `/usr/bin/${cli}\n` } : {});
    await devcontainerAttach(attach, { workspace, home: workspace, user: "vscode", env: { SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL: "1", TERM_PROGRAM: cli, VSCODE_IPC_HOOK_CLI: "/tmp/editor.sock" }, arch: "x64" });
    const forbidden = (call: string[]) => fixture.dashboardOnly.forbiddenPrograms.includes(call[0]) || call.some((argument) => fixture.dashboardOnly.forbiddenArgs.includes(argument));
    assert.deepEqual(attach.calls.filter(forbidden), []);
    assert.deepEqual(lodash.filter(attach.calls, (call: string[]) => lodash.includes(fixture.dashboardOnly.forbiddenPrograms, call[0]) || lodash.some(call, (argument: string) => lodash.includes(fixture.dashboardOnly.forbiddenArgs, argument))), []);
    assert.ok(attach.lines.includes("✅️ Post-attach setup complete."));
  }
  console.log("✅️ Devcontainer dashboard ownership: retired controls absent; attach runs repository hooks without editor commands (lodash oracle) PASS");
}
