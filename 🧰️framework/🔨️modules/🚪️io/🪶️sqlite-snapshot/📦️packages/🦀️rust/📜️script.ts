#!/usr/bin/env bun
import{createIssuedResourceControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { scriptInvocationBudget, BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { cmdBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

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
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    const { rest } = resolveTestLevel(segments);
    if (rest.length) throw Error("test-native accepts only an execution level");
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-io-sqlite-snapshot"], cwd: this.root, extraArgs: ["--lib", "--no-fail-fast", "--", "--nocapture"] }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

/** 📑️ Proves original paid borrowed schema validation against independent SQLite. */
class ReceivingSchemaScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(segments.length)throw Error('test-receiving-schema accepts no arguments');
  const source=resolve(this.root,'../../🧩️artifact/🫴️receiving/🧪️tests/🟦️.ts');
  await runBudgetedTestCommand(process.execPath,['test',source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
  await runCargoTestsV1({manifestPath:resolve(this.root,'Cargo.toml'),packages:['semio-framework-io-sqlite-snapshot'],cwd:this.root,extraArgs:['--lib','sqlite_snapshot_receiving_borrowed_schema_uses_original_work_without_owned_tokens','--no-fail-fast','--success-output','immediate'],signal:this.invocation.control.signal,remainingMilliseconds:this.invocation.control.remainingMilliseconds},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
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

/** 🏛️ Checks the single neutral SQLite owner and complete public transfer contract. */
class OwnershipScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-ownership accepts no arguments");
  const source=resolve(this.root,"../../🧪️tests/🏛️ownership/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 💰️ Runs the closed original SQLite fixed ceiling contract and selected allocator laws. */
class OriginalFixedLimitsScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  const mode=segments[0];if(segments.length!==1||(mode!=="source"&&mode!=="native"))throw Error("test-original-fixed-limits requires source or native");
  const source=resolve(this.root,"../../⚠️refusal/💰️limits/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
  if(mode==="native")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-io-sqlite-snapshot"],cwd:this.root,extraArgs:["--lib","sqlite_snapshot_original_fixed_limits_","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:this.invocation.control.remainingMilliseconds},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}

/** 🫴️ Executes original physical file custody demands before the whole typed file producer. */
class OriginalPhysicalScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  const mode=segments[0];if(segments.length!==1||(mode!=="source"&&mode!=="native"))throw Error("test-original-physical requires source or native");
  if(mode==="source"){
   const source=resolve(this.root,"../../🔁️transfer/🫴️receiving/🧪️tests/🟦️.ts");
   await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,15000),signal:this.invocation.control.signal,throwOnFailure:true});
   await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,15000),signal:this.invocation.control.signal,throwOnFailure:true});
  }else await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-io-sqlite-snapshot"],cwd:this.root,extraArgs:["--lib","sqlite_snapshot_original_physical_","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:this.invocation.control.remainingMilliseconds},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}


const router = new ScriptRouter(import.meta.dir).register("test-original-fixed-limits",OriginalFixedLimitsScript).register("test-original-physical",OriginalPhysicalScript).register("test-ownership",OwnershipScript).register("test-operation",OperationScript).register("test-refusal", RefusalScript).register("test-native", NativeScript).register("test-receiving-schema",ReceivingSchemaScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test-refusal" }) }));

