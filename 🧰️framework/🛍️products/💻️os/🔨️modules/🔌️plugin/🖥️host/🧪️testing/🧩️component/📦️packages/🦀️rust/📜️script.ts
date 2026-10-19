#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {join} from "node:path";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { runCheck } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class SqliteSource extends BundleScript {async run(segments:string[]){if(segments.length)throw new Error("test-snapshot-sqlite-source accepts no arguments");await runBudgetedTestCommand(process.execPath,["test","--timeout","60000",join(import.meta.dir,"../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:300000,throwOnFailure:true});}}
class SqliteNative extends BundleScript {async run(segments:string[]){if(segments.length)throw new Error("test-snapshot-sqlite-native accepts no arguments");await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-host-test-component"], this.repoRoot, this.invocation, ["--lib","sqlite_snapshot_host_count_","--no-fail-fast"]);}}
class SqliteReleaseNative extends BundleScript {async run(segments:string[]){if(segments.length)throw new Error("test-snapshot-sqlite-release-native accepts no arguments");await runAdmittedCargoPackageTestsV1(["semio-framework-plugin-host-test-component"], this.repoRoot, this.invocation, ["--lib","sqlite_retained_host_count_erased_requests_diagnostics_and_full_release","--no-fail-fast"]);}}
class Check extends BundleScript {
  async run(): Promise<void> {
    await runCheck({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("check",Check).register("test-snapshot-sqlite-source",SqliteSource).register("test-snapshot-sqlite-native",SqliteNative).register("test-snapshot-sqlite-release-native",SqliteReleaseNative), { invocation: original, defaultCommand: "check" }));
