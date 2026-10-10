#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
/** ⚙️ Builds, installs and tests the `semio-framework-repo-dashboard` crate and execs its `semio` binary (nx bridge for `repo/dashboard/rs`). */
import { join } from "node:path";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { projectDashboardLaunch, ticketLaunchCommands, launchParameterArguments } from "../../🌳️command-tree/🚀️launch/🟦️.ts";
import { captureDashboardSources, installedDashboard, installDashboard, staleDashboard } from "../../📦️installation/🟦️.ts";
import { devToolingEnv, runRepositoryCargoTests, runCmd, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const crate = "semio-framework-repo-dashboard";

/** 🧭️ Publishes scoped owner controls and runs their native registry identities. */
class LaunchScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    const [action,selection,...rest]=segments;
    if(action==="test"){runCmd("bun",["test",join(this.root,"../../🧪️tests/🚀️launch/🟦️.ts")],{cwd:this.repoRoot,env:devToolingEnv()});return;}
    const ticket=action==="run"?selection?.slice(7).split("/").slice(0,4).join("/"):selection;
    if(!ticket||!/^\d{2}\/\d{2}\/\d{2}\/[A-Z0-9]+(?:-[A-Z0-9]+)*$/.test(ticket)||rest.length)throw new Error("launch generate|check <YY/MM/DD/TICKET> or launch run <ticket-command-id>");
    const [year,month,day,name]=ticket.split("/");
    const file=join(this.repoRoot,`.🧬semio/🦑️repo/🎫️tickets/🎆️${year}/🌙️${month}/☀️${day}/${name}/🎮️commands.json`);
    const declaration=JSON.parse(await readFile(file,"utf8"));
    const commands=ticketLaunchCommands(ticket,declaration);
    if(action==="run"){
      const command=commands.find(command=>command.id===selection);
      if(!command)throw new Error(`Dashboard command is not declared: ${selection}`);
      const status=runCmdStatus(await dashboardExecutable(this.root,this.repoRoot),["run",command.id,...launchParameterArguments(command,process.env)],{cwd:this.repoRoot,env:devToolingEnv()});
      process.exit(status);
    }
    if(action!=="generate"&&action!=="check")throw new Error(`Unknown dashboard launch action ${action}`);
    const output=join(this.repoRoot,".vscode/launch.json"),current=JSON.parse(await readFile(output,"utf8"));
    const projected=projectDashboardLaunch(current,commands,ticket);
    if(action==="check"&&JSON.stringify(current)!==JSON.stringify(projected))throw new Error("Dashboard launch projection is stale");
    if(action==="generate")await writeFile(output,JSON.stringify(projected,null,2)+"\n");
    console.log(`[DEBUG] dashboard launch ${action} owner=${ticket} commands=${commands.length} locales=en,de unrelatedPreserved=true`);
  }
}

class BuildScript extends BundleScript {
  async run(): Promise<void> { await buildAndInstallDashboard(this.root, this.repoRoot); }
}

class InstallScript extends BundleScript {
  async run(): Promise<void> { console.log(`[dashboard] Installed ${await installDashboard(this.root, this.repoRoot)}`); }
}

class PreferencesScript extends BundleScript {
  async run(segments: string[]): Promise<void> { process.exit(runCmdStatus(await dashboardExecutable(this.root, this.repoRoot), ["preferences", ...segments], { cwd: this.repoRoot, env: devToolingEnv() })); }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "unit") {
      const { rest } = resolveTestLevel(segments.slice(1));
      await runRepositoryCargoTests([crate], this.repoRoot, this.invocation.control, rest); return;
    }
    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧭️authority/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
    if (segments[0] === "authority") return;
    if (segments[0] === "execution") {
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/dashboard-execution");
      await mkdir(artifacts, { recursive: true });
      runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧊️execution/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_TEST_ARTIFACT_DIR: artifacts }) }); return;
    }
    if (BATTLE_SUITES.includes(segments[0] ?? "")) { await battle(this.root, this.repoRoot, segments[0]!); return; }
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_PATH ??= process.env.PATH;
    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🌀️control-plane/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
    const native = devToolingEnv({ SEMIO_DASHBOARD_BIN: await dashboardExecutable(this.root, this.repoRoot) });
    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🎮️registry/🟦️.ts")], { cwd: this.repoRoot, env: native });
    runCmd("bun", ["test", "--timeout", "240000", join(this.root, "../../🧪️tests/🧭️cli/🟦️.ts")], { cwd: this.repoRoot, env: native });
    await runRepositoryCargoTests([crate], this.repoRoot, this.invocation.control, rest);
  }
}

const BATTLE_SUITES = ["coverage", "journeys", "load", "smoke"];

/**
 * ⚔️ Battle tests (`test coverage|journeys|load|smoke`): the installed native `semio` against the real monorepo or the
 * journey fixture workspace. The pseudo-terminal suites are Rust test executables run directly, never under
 * `cargo test`, whose job object forbids the breakaway the daemon needs.
 */
async function battle(root: string, repoRoot: string, suite: string): Promise<void> {
  const dashboardRoot = join(root, "../..");
  const target = process.env.CARGO_TARGET_DIR ?? join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/cargo/dashboard-journeys");
  const env = devToolingEnv({ CARGO_TARGET_DIR: target, SEMIO_TEST_CLI: process.env.SEMIO_TEST_CLI ?? await dashboardExecutable(root, repoRoot) });
  const suites: Record<string, string[]> = { coverage: ["🗺️coverage/🟦️.ts"], journeys: ["🧭️journeys/🟦️.ts"], load: ["🧭️journeys/🏋️load/🟦️.ts"], smoke: ["🧭️journeys/💨️smoke/🟦️.ts"] };
  for (const file of suites[suite]!) runCmd("bun", ["test", join(dashboardRoot, "🧪️tests", file), "--timeout", "3600000"], { cwd: repoRoot, env });
  const rust = { journeys: "journeys", load: "load" }[suite];
  if (!rust) return;
  const manifest = join(dashboardRoot, "🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml");
  const built = Bun.spawnSync(["cargo", "test", "--manifest-path", manifest, "--test", rust, "--no-run", "--locked", "--message-format=json"], { cwd: repoRoot, env, stdout: "pipe", stderr: "inherit" });
  if (built.exitCode !== 0) throw new Error(`${rust} compilation exited ${built.exitCode}`);
  const executable = built.stdout.toString().split("\n").flatMap((line) => { try { const row = JSON.parse(line); return row.target?.name === rust && row.target?.kind?.includes("test") ? [row.executable as string | null] : []; } catch { return []; } }).filter(Boolean).at(-1);
  if (!executable) throw new Error(`no ${rust} test executable was built`);
  runCmd(executable, ["--test-threads=1"], { cwd: repoRoot, env });
}

/** ▶️ Runs the installed native dashboard without a build prerequisite. */
class RunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const bin = await dashboardExecutable(this.root, this.repoRoot);
    const status = runCmdStatus(bin, segments, { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/**
 * 🌀 Forwards `semio daemon …` using the installed native executable.
 */
class DaemonScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const bin = await dashboardExecutable(this.root, this.repoRoot);
    const status = runCmdStatus(bin, ["daemon", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/** 🔨️ Builds the release executable and installs it; the sources are captured first so edits made during the build stay detectable, and Ctrl+C cancels the build. */
export async function buildAndInstallDashboard(packageRoot: string, workspace: string): Promise<string> {
  const controller = new AbortController(), cancel = () => controller.abort();
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  try {
    const sources = await captureDashboardSources(packageRoot, workspace);
    await buildRepositoryCargoArtifacts(join(packageRoot, "Cargo.toml"), ["--release", "--bin", "semio"], workspace, { signal: controller.signal });
    return await installDashboard(packageRoot, workspace, sources);
  } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
}

/** 🚀️ Resolves the installed immutable executable, rebuilding and reinstalling it when none is recorded or its sources changed. */
export async function dashboardExecutable(packageRoot: string, workspace: string): Promise<string> {
  const reason = await staleDashboard(workspace);
  if (reason) {
    console.log(`[dashboard] Rebuilding because ${reason}; Ctrl+C cancels`);
    return await buildAndInstallDashboard(packageRoot, workspace);
  }
  return installedDashboard(workspace);
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("launch", LaunchScript).register("build", BuildScript).register("install", InstallScript).register("preferences", PreferencesScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
