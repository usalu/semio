#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { cmdBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";

/** ⚠️ Checks owned SQLite refusal producers against the closed schema and independent SQLite. */
class RefusalScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-refusal accepts no arguments");
    const source = resolve(this.root, "../../⚠️refusal/🧪️tests/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", source], { cwd: this.repoRoot, budgetMs: cmdBudgetMs(), throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true });
  }
}

/** 🪶️ Executes the complete owned SQLite native law roster without selected filters. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest.length) throw Error("test-native accepts only an execution level");
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-io-sqlite-snapshot"], cwd: this.root, extraArgs: ["--lib", "--no-fail-fast"] }, readCargoTestPolicyV1(process.env));
  }
}

/** 💰️ Checks the complete neutral SQLite operation and frontier laws. */
class OperationScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-operation accepts no arguments");
  const sources=["💰️operation","💰️frontiers","🗂️typed-index","🗂️row-index"].map(kind=>resolve(this.root,"../../🧪️tests/"+kind+"/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...sources],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",...sources],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

const router = new ScriptRouter(import.meta.dir).register("test-operation",OperationScript).register("test-refusal", RefusalScript).register("test-native", NativeScript);
await runScriptMain(router, { defaultCommand: "test-refusal" });

