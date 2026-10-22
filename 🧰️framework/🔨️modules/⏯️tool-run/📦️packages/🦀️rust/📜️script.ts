#!/usr/bin/env repo
import { scriptCompilerAdmissionControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { buildBudgetMs } from "../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoCommandV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { join } from "node:path";
import { BundleScript, ScriptRouter, scriptInvocationBudget } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { advanceScriptInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TestScript, SourceScript, COMPANION, PRESENTATION, ENTITIES } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
import { runtimeTypeEnvironmentV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";


const PACKAGE = "semio-framework-tool-run";

/** 📸️ Verifies original progress metadata against canonical tick/schema oracles and physical native receipts. */
class OriginalProgressTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length>1||(args.length===1&&args[0]!=="source"))throw new Error("Expected test-original-progress [source]");
  await runOwnedCommand(process.execPath,["test","--test-name-pattern","schema oracle|tick codec and step ring",join(this.root,"../../🧪️tests/🧪️conformance/🟦️.ts")],this.repoRoot,"tool-run:original-source",scriptInvocationBudget(this.invocation,testLevelBudgetMs()),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"original-source","running")});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types",runtimeTypeEnvironmentV1(),join(this.root,"../../📊️progress/📸️clone/🟦️.ts"),join(this.root,"../../📊️progress/💍️steps/➕️insert/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🧬️update/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🌱️seed/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🗃️slot/🟦️.ts"),join(this.root,"../../📽️tick/🔐️source/🟦️.ts")],this.repoRoot,"tool-run:original-progress-clone:types",120000);
  if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION,PRESENTATION],cwd:this.root,extraArgs:["--lib","original_tool_run_progress_metadata","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 👥️ Verifies bounded original entity edit custody against shared independent append/retract laws. */
class OriginalEntitiesTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length>1||(args.length===1&&args[0]!=="source"))throw new Error("Expected test-original-entities [source]");
  const file=join(this.root,"../../👥️entities/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,["test",file],this.repoRoot,"tool-run:original-source",scriptInvocationBudget(this.invocation,testLevelBudgetMs()),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"original-source","running")});
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types",runtimeTypeEnvironmentV1(),file],this.repoRoot,"tool-run:entity-edit:types",120000);
  if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[ENTITIES],cwd:this.root,extraArgs:["--lib","original_tool_run_entity_edit","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 📏️ Verifies required captured receiving bounds against the neutral oracle and exact native law. */
class OriginalReceivingLimitsTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length)throw new Error("Expected test-original-receiving-limits");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION],cwd:this.root,extraArgs:["--lib","original_tool_run_receiving_limits_preserve_every_required_captured_bound","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 🪪️ Executes only the original typed identity source/token/native interruption law. */
class OriginalIdentityReceivingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length)throw new Error("Expected test-original-identity-receiving");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION],cwd:this.root,extraArgs:["--lib","original_typed_identity_receiving_preserves_paid_tokens_and_every_native_interruption","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 📝️ Executes the original typed step source/token/native argument interruption law. */
class OriginalStepReceivingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length)throw new Error("Expected test-original-step-receiving");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION],cwd:this.root,extraArgs:["--lib","-E","test(original_typed_identity_receiving_preserves_paid_tokens_and_every_native_interruption) | test(original_typed_step_receiving_preserves_final_native_arguments_and_every_interruption)","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 🏗️ Executes direct final native Tick field preparation and every paid interruption prefix. */
class OriginalNativeTickTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length)throw new Error("Expected test-original-native-tick");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION],cwd:this.root,extraArgs:["--lib","-E","test(original_native_tick_preparation_preserves_final_frame_and_every_initialized_prefix) | test(original_native_trace_column_reservation_keeps_all4096_ops_without_payload_moves)","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 🔢️ Executes the original source-bound final entity-column token and closure law. */
class OriginalEntityColumnTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length)throw new Error("Expected test-original-entity-column");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION],cwd:this.root,extraArgs:["--lib","-E","test(original_native_tick_entity_column_) | test(original_native_tick_preparation_preserves_final_frame_and_every_initialized_prefix)","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
/** 📊️ Executes original Progress final-cell custody at every paid interruption frontier. */
class OriginalProgressReceivingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(args.length)throw new Error("Expected test-original-progress-receiving");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION],cwd:this.root,extraArgs:["--lib","-E","test(original_progress_receiving_keeps_final_slot_counter_and_argument_pointers_at_every_paid_frontier)","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}
class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", PACKAGE],this.repoRoot,process.env,buildBudgetMs());
    await runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", PACKAGE, "--target", "wasm32-wasip2"],this.repoRoot,process.env,buildBudgetMs());
    await runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", COMPANION, "--tests"],this.repoRoot,process.env,buildBudgetMs());
  }
}



/** 🌊️ Runs original native directory custody and independent Buffer range laws through the owner. */
class OriginalPayloadTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length>1||(args.length===1&&args[0]!=="source"))throw new Error("Expected test-original-payload [source]");
  const files=[join(this.root,"../../📨️payload/🌊️stream/🧪️tests/🟦️.ts"),join(this.root,"../../📨️payload/🌊️stream/📡️wire/🧪️tests/🟦️.ts"),join(this.root,"../../🎞️tick/🛬️receive/🧪️tests/🟦️.ts")];
  const options={signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"test-original-payload","running")};
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types",runtimeTypeEnvironmentV1(),...files],this.repoRoot,"tool-run:original-payload:types",scriptInvocationBudget(this.invocation,120000),options);
  await runOwnedCommand(process.execPath,["test",...files],this.repoRoot,"tool-run:original-payload:source",scriptInvocationBudget(this.invocation,30000),options);
  if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[PACKAGE],cwd:this.root,extraArgs:["--lib","original_payload_stream","--","--nocapture"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0));
 }
}
const router = new ScriptRouter(import.meta.dirname).register("test", TestScript).register("test-original-progress", OriginalProgressTestScript).register("test-original-entities",OriginalEntitiesTestScript).register("test-original-receiving-limits",OriginalReceivingLimitsTestScript).register("test-original-identity-receiving",OriginalIdentityReceivingTestScript).register("test-original-step-receiving",OriginalStepReceivingTestScript).register("test-original-native-tick",OriginalNativeTickTestScript).register("test-original-entity-column",OriginalEntityColumnTestScript).register("test-original-progress-receiving",OriginalProgressReceivingTestScript).register("test-source", SourceScript).register("check", CheckScript).register("test-original-payload",OriginalPayloadTestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));