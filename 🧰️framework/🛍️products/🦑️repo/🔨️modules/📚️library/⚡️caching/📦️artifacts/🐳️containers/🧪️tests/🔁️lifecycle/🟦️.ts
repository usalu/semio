import assert from "node:assert/strict";
import { chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readlinkSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { homedir, userInfo } from "node:os";
import { EMOJI_FONTCONFIG, PERSISTED_HOME_DIRECTORIES, devcontainerAttach, devcontainerStart, installF3d, missingSafeDirectories, normalizeClaudeAuth, processLifecycleContext, processLifecycleHost, releaseAssetUrl, toolArchitectures, withSshAgentShellBlock, type LifecycleAnswer, type LifecycleContext, type LifecycleHost } from "../../🔁️lifecycle/🟦️.ts";

/** 🎙️ A host that records every program call and answers from `answer`, never running anything. */
function recordingHost(answer: (command: string, args: readonly string[]) => Partial<LifecycleAnswer> = () => ({})): LifecycleHost & { calls: string[][]; inputs: Map<string, string>; lines: string[]; backgrounds: string[] } {
  const calls: string[][] = [], inputs = new Map<string, string>(), lines: string[] = [], backgrounds: string[] = [];
  return {
    calls,
    inputs,
    lines,
    backgrounds,
    run: (command, args, options) => {
      calls.push([command, ...args]);
      if (options?.background) backgrounds.push(command);
      if (options?.input !== undefined) inputs.set([command, ...args].join(" "), options.input);
      return { status: 0, stdout: "", stderr: "", ...answer(command, args) };
    },
    download: async () => undefined,
    fetchJson: async () => ({}),
    log: (line) => void lines.push(line),
  };
}

/** 🔁️ The devcontainer start/attach lifecycle against its language-agnostic fixture: the fontconfig document (jsdom XML
 * oracle), architecture and release-asset selection (lodash oracle), dashboard attach without alternate control planes, duplicate-free `safe.directory`, Claude auth storage on a real temporary home, the idempotent shell hook, a start run
 * that never issues a destructive command, and dashboard-only attach through a recording host. */
export async function testDevcontainerLifecycle(workspace: string, generated: string): Promise<void> {
  const previousHome = process.env.HOME, previousUser = process.env.USER;
  const restore = () => {
    if (previousHome === undefined) delete process.env.HOME;
    else process.env.HOME = previousHome;
    if (previousUser === undefined) delete process.env.USER;
    else process.env.USER = previousUser;
  };
  delete process.env.HOME;
  delete process.env.USER;
  try {
    const missing = processLifecycleContext(workspace);
    assert.equal(missing.home, homedir());
    assert.equal(missing.user, userInfo().username);
  } finally {
    restore();
  }
  process.env.HOME = "/tmp/semio-lifecycle-home";
  process.env.USER = "ada";
  try {
    const present = processLifecycleContext(workspace);
    assert.equal(present.home, "/tmp/semio-lifecycle-home");
    assert.equal(present.user, "ada");
  } finally {
    restore();
  }
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🔁️lifecycle/🔣️.json"), "utf8"));
  await testDevcontainerDashboardOwnership(workspace);
  const { JSDOM } = require("jsdom"), lodash = require("lodash");
  const document = new JSDOM(EMOJI_FONTCONFIG, { contentType: "application/xml" }).window.document as Document;
  const aliases = Object.fromEntries([...document.querySelectorAll("fontconfig > alias")].map(alias => [alias.querySelector("family")!.textContent, [...alias.querySelectorAll("prefer > family")].map(family => family.textContent)]));
  assert.deepEqual(aliases, fixture.fontconfig.aliases);
  assert.ok([...document.querySelectorAll("fontconfig > match")].every(match => match.textContent?.includes(fixture.fontconfig.emojiFamily)));
  for (const row of fixture.architectures) assert.deepEqual(toolArchitectures(row.arch), { debian: row.debian, f3d: row.f3d }, row.arch);
  for (const row of fixture.releaseCases) {
    assert.equal(releaseAssetUrl(fixture.release, row.suffix), row.url, row.suffix);
    assert.equal(lodash.find(fixture.release.assets, (asset: { name: string }) => lodash.endsWith(asset.name, row.suffix))?.browser_download_url ?? null, row.url);
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
  for (const directory of PERSISTED_HOME_DIRECTORIES) assert.ok(flat.includes(`sudo -n chown -R vscode:vscode ${join(home, directory)}`), directory);
  const devcontainer = Bun.JSONC.parse(readFileSync(join(workspace, ".devcontainer/devcontainer.json"), "utf8")) as { remoteUser: string; mounts: string[] };
  const homeVolumes = devcontainer.mounts.map((mount) => /(?:^|,)target=([^,]+)/u.exec(mount)?.[1] ?? "").filter((target) => target.startsWith(`/home/${devcontainer.remoteUser}/`));
  for (const target of homeVolumes) assert.ok(PERSISTED_HOME_DIRECTORIES.some((directory) => `${target}/`.startsWith(`/home/${devcontainer.remoteUser}/${directory}/`)), `the persisted home volume ${target} is handed to the container user on start`);
  assert.ok(flat.includes(`sudo -n chown -R vscode:vscode ${join(checkout, "♻️mit-bestand/🔎️recherche")}`));
  assert.equal(start.inputs.get("sudo -n tee /etc/fonts/local.conf"), EMOJI_FONTCONFIG);
  assert.deepEqual(flat.filter((call) => call.startsWith("git config --global --add safe.directory")), [`git config --global --add safe.directory ${join(checkout, "♻️mit-bestand/🔎️recherche")}`]);
  assert.deepEqual(flat.filter((call) => fixture.forbiddenCommands.some((forbidden: string) => call.includes(forbidden))), [], "a container start never discards source-control or database state");
  assert.equal(flat.some((call) => call.startsWith("Xvfb")), false, "a start without DISPLAY does not open a headless server");
  const display = recordingHost((command) => command === "test" ? { status: 1 } : {});
  devcontainerStart(display, { ...context, env: { DISPLAY: fixture.headlessDisplay.display } });
  assert.deepEqual(lodash.find(display.calls, (call: string[]) => call[0] === "Xvfb"), fixture.headlessDisplay.argv);
  assert.deepEqual(display.backgrounds, ["Xvfb"]);
  const marker = join(generated, "xvfb-background.pid"), script = join(generated, "xvfb-background.sh");
  rmSync(marker, {force: true});
  writeFileSync(script, `#!/bin/sh\necho $$ > ${JSON.stringify(marker)}\nexec sleep 30\n`);
  chmodSync(script, 0o755);
  const began = Date.now(), background = processLifecycleHost().run(script, [], { background: true });
  assert.equal(background.status, 0);
  assert.ok(Date.now() - began < 1500, "a headless display server must not be waited on");
  assert.equal(processLifecycleHost().run("semio-missing-xvfb", [], { background: true }).status, -1);
  let pid = 0;
  for (let attempt = 0; attempt < 50 && !pid; attempt += 1) { try { pid = Number(readFileSync(marker, "utf8")); } catch { Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 20); } }
  assert.ok(pid > 0);
  process.kill(pid, "SIGTERM");
  const warm = recordingHost((command) => command === "test" ? { status: 0 } : {});
  devcontainerStart(warm, { ...context, env: { DISPLAY: fixture.headlessDisplay.display } });
  assert.equal(warm.calls.some((call) => call[0] === "Xvfb"), false);
  const missingF3d = recordingHost((command) => command === "f3d" ? { status: 1 } : {});
  await installF3d(missingF3d, context);
  const sudo = missingF3d.calls.filter((call) => call[0] === "sudo");
  assert.ok(sudo.length >= 2 && sudo.every((call) => call[1] === "-n"), "F3D installation cannot wait for a sudo password");
  assert.ok(sudo.some((call) => call.includes("install") && call.includes("-y")));


  console.log("✅️ Devcontainer lifecycle: fontconfig (jsdom XML), tool selection (lodash), dashboard-only attach, safe.directory, Claude auth storage, start without destructive commands, IDE controls retired PASS");
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
  for (const state of fixture.dashboardOnly.forbiddenState) {
    assert.equal(PERSISTED_HOME_DIRECTORIES.includes(state as typeof PERSISTED_HOME_DIRECTORIES[number]), false, state);
    assert.equal(container.mounts.some((mount: string) => mount.includes(state)), false, state);
  }
  assert.equal("SEMIO_GITKRAKEN_WORKSPACE_NAME" in container.containerEnv, false);
  const policy=fixture.dashboardOnly.automaticHooks,require=createRequire(import.meta.url),validate=new(require("ajv").default)({strict:true}).compile(JSON.parse(readFileSync(join(import.meta.dir,"../../🔁️lifecycle/🧬️schema/🔣️.json"),"utf8")));
  assert.equal(validate(policy),true,JSON.stringify(validate.errors));
  const {Database}=await import("bun:sqlite"),database=new Database(":memory:");
  try{
  const rejected=(calls:readonly (readonly string[])[])=>Number((database.query("SELECT count(*) AS rejected FROM json_each(?1) AS call WHERE EXISTS (SELECT 1 FROM json_each(call.value) AS arg JOIN json_each(?2) AS denied ON arg.value=denied.value)").get(JSON.stringify(calls),JSON.stringify(policy.forbiddenArguments)) as {rejected:number}).rejected);
  for(const row of policy.cases){assert.equal(rejected([row.argv])===0,row.allowed,row.id);assert.equal(!lodash.some(row.argv,(argument:string)=>lodash.includes(policy.forbiddenArguments,argument)),row.allowed,row.id);}
  for (const mode of fixture.dashboardOnly.attachModes) for (const cli of fixture.dashboardOnly.forbiddenPrograms) {
    const attach = recordingHost((command) => command === "which" ? { stdout: `/usr/bin/${cli}\n` } : {});
    await devcontainerAttach(attach, { workspace, home: workspace, user: "vscode", env: { ...(mode.skipTools ? { SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL: "1" } : {}), TERM_PROGRAM: cli, VSCODE_IPC_HOOK_CLI: "/tmp/editor.sock" }, arch: "x64" });
    const forbidden = (call: string[]) => fixture.dashboardOnly.forbiddenPrograms.includes(call[0]) || call.some((argument) => fixture.dashboardOnly.forbiddenArgs.includes(argument));
    assert.deepEqual(attach.calls.filter(forbidden), []);
    assert.equal(rejected(attach.calls),0,"attach never installs automatic Git hooks");
    assert.deepEqual(lodash.filter(attach.calls, (call: string[]) => lodash.includes(fixture.dashboardOnly.forbiddenPrograms, call[0]) || lodash.some(call, (argument: string) => lodash.includes(fixture.dashboardOnly.forbiddenArgs, argument))), []);
    assert.ok(attach.lines.includes("✅️ Post-attach setup complete."));
  }
  }finally{database.close();}
  console.log("✅️ Devcontainer dashboard ownership: retired controls absent; attach installs no automatic Git hooks or editor controls (lodash/SQLite oracles) PASS");
}
