#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, scriptInvocationBudget } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

/** 🧬️ Checks canonical record model and binding ownership without product infrastructure. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    const test = resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🪆️ Runs every canonical generic record, notation and native construction law. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const {rest} = resolveTestLevel(segments);
    await runCargoTestsV1({manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-dsl-record"], cwd: this.root, extraArgs: rest}, readCargoTestPolicyV1(process.env));
  }
}

/** 🫴️ Qualifies the neutral original typed receiving contract under one exact invocation. */
class OriginalReceivingScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  const mode=segments[0];if(segments.length!==1||(mode!=="source"&&mode!=="native"))throw Error("test-original-receiving requires source or native");
  {
   const source=resolve(this.root,"../../🫴️receiving/🧪️tests/🟦️.ts");
   await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,15000),signal:this.invocation.control.signal,throwOnFailure:true});
   await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,15000),signal:this.invocation.control.signal,throwOnFailure:true});
  }if(mode==="native")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-dsl-record"],cwd:this.root,extraArgs:["--offline","--lib","original_typed_receiving_","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:this.invocation.control.remainingMilliseconds},readCargoTestPolicyV1(process.env));
 }
}
const router = new ScriptRouter(import.meta.dir).register("test-original-receiving",OriginalReceivingScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand: "test-ownership"}) }));
