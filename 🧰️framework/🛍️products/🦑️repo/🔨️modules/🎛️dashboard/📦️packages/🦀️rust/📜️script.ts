#!/usr/bin/env bun
import {repositoryCargoNetworkV1} from "./../../../📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {readBuildMode,buildModeEnvironment} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {runOwnedCommand} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationEnvironment,receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {runDeclaredRustBuildV1} from "../../../📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🏃️routing/🟦️.ts";
import {runOwnedNxInvocationV1} from "../../../📚️library/⚡️caching/🚀️bootstrap/📜️script.ts";
/** ⚙️ Builds, installs and tests the `semio-framework-repo-dashboard` crate and execs its `semio` binary (nx bridge for `repo/dashboard/rs`). */
import {join,relative} from "node:path";
import {runRepositoryCargoTestExecutableV1} from "../../../📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { normalizeDashboardLaunchBuildV1, projectDashboardLaunch, ticketLaunchCommands, launchParameterArguments } from "../../🌳️command-tree/🚀️launch/🟦️.ts";
import { captureDashboardSources, installedDashboard, installDashboard, staleDashboard, launchDashboard, withInstalledDashboardV1, type DashboardExecutionOptions } from "../../📦️installation/🟦️.ts";
import {runRepositoryCargoTests} from "../../../📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {devToolingEnv} from "../../../📚️library/🏃️process/🌿️environment/🟦️.ts";
import {runCmd} from "../../../📚️library/🏃️process/🟦️.ts";
import { BundleScript, ScriptRouter, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const crate = "semio-framework-repo-dashboard";
const dashboardOptions=(invocation:ScriptInvocation,workspace:string):DashboardExecutionOptions=>({invocation,workspace,network:repositoryCargoNetworkV1(invocation,workspace),mode:readBuildMode(process.env.SEMIO_BUILD_MODE),control:scriptCompilerAdmissionControlV1(invocation,"dashboard:execution",0)});

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
      process.exitCode=await launchDashboard(this.repoRoot,["run",command.id,...launchParameterArguments(command,process.env)],dashboardOptions(this.invocation,this.repoRoot));return;
    }
    if(action!=="generate"&&action!=="check")throw new Error(`Unknown dashboard launch action ${action}`);
    const output=join(this.repoRoot,".vscode/launch.json"),current=JSON.parse(await readFile(output,"utf8"));
    const projected=projectDashboardLaunch(current,commands,ticket);
    if(action==="check"&&JSON.stringify(current)!==JSON.stringify(projected))throw new Error("Dashboard launch projection is stale");
    if(action==="generate")await writeFile(output,JSON.stringify(normalizeDashboardLaunchBuildV1(projected),null,2)+"\n");
    console.log(`[DEBUG] dashboard launch ${action} owner=${ticket} commands=${commands.length} locales=en,de unrelatedPreserved=true`);
  }
}

class BuildScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(process.env.NX_TASK_TARGET_PROJECT!=="@semio-tech/repo-dashboard-rs"||process.env.NX_TASK_TARGET_TARGET!=="build"){await runOwnedNxInvocationV1(this.repoRoot,["run","@semio-tech/repo-dashboard-rs:build",...segments],this.invocation);return;}
    await buildAndInstallDashboard(this.root,this.repoRoot,dashboardOptions(this.invocation,this.repoRoot));
  }
}

class CompileScript extends BundleScript {
  async run(segments:string[]):Promise<void>{await runDeclaredRustBuildV1(this.repoRoot,"@semio-tech/repo-dashboard-rs",this.root,this.invocation,segments,"compile");}
}

class InstallScript extends BundleScript {
  async run(): Promise<void> { console.log(`[dashboard] Installed ${await installDashboard(this.root, this.repoRoot,dashboardOptions(this.invocation,this.repoRoot))}`); }
}

class PreferencesScript extends BundleScript {
  async run(segments:string[]):Promise<void>{process.exitCode=await launchDashboard(this.repoRoot,["preferences",...segments],dashboardOptions(this.invocation,this.repoRoot));}
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "unit") {
      const { rest } = resolveTestLevel(segments.slice(1));
      await runRepositoryCargoTests([crate], this.repoRoot, this.invocation, rest); return;
    }
    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧭️authority/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
    if (segments[0] === "authority") return;
    if (segments[0] === "execution") {
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/dashboard-execution");
      await mkdir(artifacts, { recursive: true });
      await runOwnedCommand(process.execPath,["test",join(this.root,"../../🧪️tests/🧊️execution/🟦️.ts")],this.repoRoot,"dashboard:execution-test",compilerAdmissionBudgetV1(dashboardOptions(this.invocation,this.repoRoot).control),{env:devToolingEnv({SEMIO_TEST_ARTIFACT_DIR:artifacts}),signal:this.invocation.control.signal});return;
    }
    if (BATTLE_SUITES.includes(segments[0] ?? "")) { await battle(this.root, this.repoRoot, segments[0]!,dashboardOptions(this.invocation,this.repoRoot)); return; }
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_PATH ??= process.env.PATH;
    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🌀️control-plane/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
    const options=dashboardOptions(this.invocation,this.repoRoot);await dashboardExecutable(this.root,this.repoRoot,options);
    await withInstalledDashboardV1(this.repoRoot,options,async executable=>{
      const native = devToolingEnv({ SEMIO_DASHBOARD_BIN: executable });
      runCmd("bun", ["test", join(this.root, "../../🧪️tests/🎮️registry/🟦️.ts")], { cwd: this.repoRoot, env: native });
      runCmd("bun", ["test", "--timeout", "240000", join(this.root, "../../🧪️tests/🧭️cli/🟦️.ts")], { cwd: this.repoRoot, env: native });
    });
    await runRepositoryCargoTests([crate], this.repoRoot, this.invocation, rest);
  }
}

const BATTLE_SUITES = ["coverage", "journeys", "load", "smoke"];

/**
 * ⚔️ Battle tests (`test coverage|journeys|load|smoke`): the installed native `semio` against the real monorepo or the
 * journey fixture workspace. The pseudo-terminal suites are Rust test executables run directly, never under
 * `cargo test`, whose job object forbids the breakaway the daemon needs.
 */
async function battle(root: string, repoRoot: string, suite: string,options:DashboardExecutionOptions): Promise<void> {
  await dashboardExecutable(root,repoRoot,options);
  return withInstalledDashboardV1(repoRoot,options,executable=>battleWithInstalledDashboard(root,repoRoot,suite,options,executable));
}

async function battleWithInstalledDashboard(root:string,repoRoot:string,suite:string,options:DashboardExecutionOptions,dashboard:string):Promise<void>{
  const dashboardRoot = join(root, "../..");
  const env=devToolingEnv({SEMIO_TEST_CLI:dashboard});
  const suites:Record<string,string[]>={coverage:["🗺️coverage/🟦️.ts"],journeys:["🧭️journeys/🟦️.ts"],load:["🧭️journeys/🏋️load/🟦️.ts"],smoke:["🧭️journeys/💨️smoke/🟦️.ts"]};
  for(const file of suites[suite]!)await runOwnedCommand(process.execPath,["test",join(dashboardRoot,"🧪️tests",file),"--timeout","3600000"],repoRoot,"dashboard:battle",compilerAdmissionBudgetV1(options.control),{env:scriptInvocationEnvironment(options.invocation,env),signal:options.control.signal});
  const rust={journeys:"journeys",load:"load"}[suite];if(!rust)return;
  const manifest=join(dashboardRoot,"🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml");
  await runRepositoryCargoTestExecutableV1({version:1,manifest:relative(repoRoot,manifest).replaceAll("\\","/"),package:"semio-dashboard-journeys",test:rust,arguments:["--test-threads=1"]},repoRoot,options.invocation,env);
}

/** ▶️ Runs the installed native dashboard without a build prerequisite. */
class RunScript extends BundleScript {
  async run(segments:string[]):Promise<void>{process.exitCode=await launchDashboard(this.repoRoot,segments,dashboardOptions(this.invocation,this.repoRoot));}
}

/**
 * 🌀 Forwards `semio daemon …` using the installed native executable.
 */
class DaemonScript extends BundleScript {
  async run(segments:string[]):Promise<void>{process.exitCode=await launchDashboard(this.repoRoot,["daemon",...segments],dashboardOptions(this.invocation,this.repoRoot));}
}

/** 🔨️ Builds the selected executable and installs it; the sources are captured first so edits made during the build stay detectable, and Ctrl+C cancels the build. */
export async function buildAndInstallDashboard(packageRoot: string, workspace: string, options:DashboardExecutionOptions): Promise<string> {
  compilerAdmissionBudgetV1(options.control);const mode=readBuildMode(options.mode);
  const sources = await captureDashboardSources(packageRoot, workspace);compilerAdmissionBudgetV1(options.control);
  await runDeclaredRustBuildV1(workspace,"@semio-tech/repo-dashboard-rs",packageRoot,options.invocation,[],"compile");
  return await installDashboard(packageRoot, workspace, options,sources);
}

/** 🚀️ Selects one mode's immutable executable, rebuilding only when that installation's sources changed. */
export async function dashboardExecutable(packageRoot: string, workspace: string, options:DashboardExecutionOptions): Promise<string> {
  compilerAdmissionBudgetV1(options.control);const mode=readBuildMode(options.mode),reason = await staleDashboard(workspace,mode);compilerAdmissionBudgetV1(options.control);
  if (reason) {
    console.log(`[dashboard] Rebuilding ${mode} because ${reason}`);
    return await buildAndInstallDashboard(packageRoot, workspace,options);
  }
  return installedDashboard(workspace,mode,options.workspace);
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("launch", LaunchScript).register("build", BuildScript).register("compile",CompileScript).register("install", InstallScript).register("preferences", PreferencesScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
