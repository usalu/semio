#!/usr/bin/env repo
import {withCargoCompilerInputsV1} from "../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";
import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import {createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {copyFileSync} from "node:fs";
import { receiveScriptProcessInvocation } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { testLevelBudgetMs, resolveTestLevel } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🦀️ `@semio-tech/framework` task router: `repo ./📜️script.ts test|generate|check|lint`. */
import { readCargoTestPolicyV1, runAdmittedCargoCommandV1, runAdmittedCargoTestsV1 as runCargoTestsV1, runAdmittedCargoLintV1 as runCargoLintV1, withAdmittedCompilerStoreV1 } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {runBudgetedTestCommand} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {runVitestV1,readVitestPolicyV1} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
import { captureOwnedProcess, resolveTypescriptCompilerV1 } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {startNativeProgress} from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter, findWorkspaceRoot } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { seedGeneratedFile } from "../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { basename, join, relative, resolve, isAbsolute } from "node:path";
import { runtimeTypeEnvironmentV1 } from "../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

/** 📦️ Every Rust package of the framework facade and its carved producer crates, in workspace order. */
const FRAMEWORK_PACKAGES:string[]=["semio-framework-abi", "semio-framework-action-bus", "semio-framework-interaction", "semio-framework-kernel", "semio-framework-kernel-history", "semio-framework-kernel-identity", "semio-framework-kernel-media-jobs", "semio-framework-kernel-ui", "semio-framework-manifest-actions", "semio-framework-manifest-address", "semio-framework-manifest-app-surface", "semio-framework-manifest-arg-schema", "semio-framework-manifest-contribution", "semio-framework-manifest-media", "semio-framework-manifest-mutation-inputs", "semio-framework-manifest-plugin", "semio-framework-manifest-view-model", "semio-framework-onboarding", "semio-framework-source-projection", "semio-framework-time-travel-actions", "semio-framework-tool-run-actions", "semio-framework"];
/** 🔐️ Selects a policy only for an explicitly declared owning package manifest. */
function cargoPolicy(root:string,manifest:string){
 const path=resolve(root,manifest),policies=JSON.parse(process.env.SEMIO_CARGO_TEST_POLICIES??"[]") as unknown[];
 if(!Array.isArray(policies)||policies.length>64)throw new Error("Invalid exact Cargo policy inventory");
 const policy=policies.map(value=>readCargoTestPolicyV1({...process.env,SEMIO_CARGO_TEST_POLICY:JSON.stringify(value)})).find(value=>value.manifestPath===path)??readCargoTestPolicyV1(process.env);
 if(policy.manifestPath!==path)throw new Error(`Missing exact Cargo policy: ${path}`);
 return policy;
}
/** 🦀️ Runs a declared package under its supplied manifest policy and assertion budgets. */
async function testCargo(original:ScriptInvocation,phaseControl:ResourceLeaseControl,root:string,manifest:string,packages:string[],args:string[]){
 requireIssuedResourceControlInvocationV1(phaseControl,original);checkResourceLeaseControl(phaseControl);
 const policy=cargoPolicy(root,manifest);
 await runCargoTestsV1({manifestPath:policy.manifestPath,packages,cwd:root,extraArgs:args,environment:{...process.env,CARGO_TARGET_DIR:policy.targetDirectory}},policy,findWorkspaceRoot(root),original,phaseControl);
 checkResourceLeaseControl(phaseControl);
}
/** 📥️ Captures one owned compiler operation with bounded evidence, progress and cancellation. */
async function captureCargo(original:ScriptInvocation,workspace:string,root:string,args:string[],environment:Readonly<Record<string,string|undefined>>=process.env,emit=true){
 const policy=readCargoTestPolicyV1(environment),control=scriptCompilerAdmissionControlV1(original,"framework:cargo",policy.buildBudgetMs);
 return withAdmittedCompilerStoreV1(workspace,policy.mode,control,environment,async (store,control)=>{
  const boundary=args.indexOf("--"),arguments_=boundary<0?[...args,"--manifest-path",policy.manifestPath]:[...args.slice(0,boundary),"--manifest-path",policy.manifestPath,...args.slice(boundary)],generated=environment.SEMIO_TYPEGEN_OUT?join(store.captureDirectory,"typegen.ts"):undefined,env={...store.environment,...(generated?{SEMIO_TYPEGEN_OUT:generated}:{})};
  return withCargoCompilerInputsV1({manifest:policy.manifestPath,cwd:root,arguments:arguments_,environment:env,control},store.withCargoInputs,async actual=>{
   const arguments_=[...actual.arguments],env=actual.environment,control=actual.control;
   const graph=await captureCargoCompilationGraphV1({workspaceRoot:workspace,cwd:root,program:"cargo",prefix:[],args:arguments_,environment:env,maximumOutputBytes:134217728},control,actualCargoCompilerPlanningPortV1());await admitCargoSourceBoundariesV1(graph,workspace,store.sourceBoundary,control,actualCargoSourceAdmissionPortV1(env,root,store.storage));await store.storage.recheck();
   return actual.withWriter(async control=>{
    const separator=arguments_.indexOf("--"),head=separator<0?arguments_:arguments_.slice(0,separator),tail=separator<0?[]:arguments_.slice(separator),locked=[...head,...["--locked","--offline"].filter(flag=>!head.includes(flag)),...tail];
   const output=mkdtempSync(join(store.captureDirectory,"general-cargo-"));try{const remaining=control.remainingMilliseconds();if(remaining===null||remaining<1)throw new Error("Finite original compiler capture deadline required");const result=await captureOwnedProcess("cargo",locked,{invocation:original,cwd:root,env,budgetMs:Math.floor(remaining),maxOutputBytes:16777216,stdoutPath:join(output,"stdout.txt"),stderrPath:join(output,"stderr.txt")});if(emit)process.stdout.write(result.stdout);process.stderr.write(result.stderr);if(result.reason!=="exit"||result.signal)throw new Error(`Framework Cargo stopped: ${result.reason} ${result.signal??""}`);if(result.status===0&&generated)copyFileSync(generated,environment.SEMIO_TYPEGEN_OUT!);return result.status??1;}finally{if(!policy.retainArtifacts)rmSync(output,{recursive:true,force:true});}
   });
  });
 });
}

//#region 🧹️WireRetirement
class WireRetirementSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testWireRetirementFixture } = await import("../../🔨️modules/🎯️action-bus/🧹️wire-retirement/🧪️tests/🔬️wire-retirement/🟦️.ts");
    testWireRetirementFixture();
  }
}
class WireRetirementNativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    const { rest } = resolveTestLevel(segments);
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", ["semio-framework-action-bus"], rest.length ? rest : ["--lib", "tests", "--", "--nocapture"]);
  }
}
/** ♻️ Runs the original typed argument owners under the owning protocol package policy. */
class ArgumentRetirementNativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);const {rest}=resolveTestLevel(segments);await testCargo(this.invocation,phaseControl,this.repoRoot,"📦️packages/🦀️rust/Cargo.toml",["semio-framework-manifest-mutation-inputs"],rest.length?rest:["--lib","argument_retirement_tests::manifest_argument_retirement_original_typed_fields_preserve_denied_pointers_and_physical_receipts","--","--nocapture"]);}
}
//#endregion 🧹️WireRetirement

/** 🧱️ Verifies the portable surface ownership witness against its hierarchy oracle. */
class FixtureOwnershipSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw new Error("test-fixture-ownership-source accepts no arguments");
 await runBudgetedTestCommand(process.execPath,["test",join(this.repoRoot,"🧪️tests/🧱️fixture-ownership/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs()});
 }
}

/** 🪪️ Runs the portable artifact grammar oracle. */
class ArtifactKindSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw new Error("test-artifact-kind-source accepts no arguments");
 await runBudgetedTestCommand(process.execPath,["test",join(this.repoRoot,"🔨️modules/🚪️io/🧬️schema/🧪️tests/🪪️artifact-kind/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs()});
 }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    const { rest } = resolveTestLevel(segments);
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", FRAMEWORK_PACKAGES, rest);
    await runVitestV1(readVitestPolicyV1(process.env,this.root),rest,"../../🧪️tests/🎚️config/🟦️.ts",process.env);
  }
}

/** ⏯️ Runs the shared tool run declaration fixture through ajv plus the TypeScript mirror, then the reserved tool run action and chord-law tests. */
class ToolRunActionsTestScript extends BundleScript {
  async run(): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🔨️modules/⏯️tool-run/🕹️actions/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", ["semio-framework-tool-run-actions"], ["--lib", "tests"]);
  }
}

/** ✏️ Runs the reserved history-edit verb fixture through the TypeScript mirror, then the Rust history-edit law. */
class HistoryEditActionsTestScript extends BundleScript {
  async run(): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🔨️modules/⏪️time-travel/🕹️actions/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", ["semio-framework-time-travel-actions"], ["--lib", "tests"]);
  }
}

/** 📈️ Verifies optional semantic-work wire preservation and neutral EN/DE status copy. */
class HistoryProgressTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    if (segments.length) throw new Error("test-history-progress accepts no arguments");
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", ["semio-framework-kernel-history"], ["--lib", "-E", "test(history_patch_tests::every_valid_patch_decodes_identically_through_serde_and_value_and_round_trips) | test(history_reprojection_tests::)", "--status-level", "pass", "--final-status-level", "all", "--", "--nocapture"]);
  }
}

/** 🧬️ Runs the mutation-input corpus (`🧫️fixtures/🧫️mutation-inputs`) through the TypeScript reader with the npm `jsonschema` and strict Ajv oracles, the Python `jsonschema` oracle, then the Rust reader. */
class MutationInputsTestScript extends BundleScript {
  async run(): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    const testCase = join(this.root, "../../🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs");
    await runBudgetedTestCommand(process.execPath, ["test", join(testCase, "🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    await runBudgetedTestCommand(process.env.SEMIO_PYTHON!, [join(testCase, "🐍️.py")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", ["semio-framework-manifest-mutation-inputs"], ["--lib", "mutation_inputs_tests"]);
  }
}

/** 🔁️ Runs the shared host-effect invocation fixture: which channel a guest's `dispatchAction` re-enters, the ONE rule both renderer targets read. */
class HostEffectInvocationTestScript extends BundleScript {
  async run(): Promise<void> {
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🔨️modules/🛂️manifest/🧪️tests/🔁️host-effect-invocation/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

/** 🔽️ Runs the shared neutral closed-choice fixture through the native implementation. */
class ActionChoicesTestScript extends BundleScript {
  async run(): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    await testCargo(this.invocation,phaseControl,this.repoRoot, "📦️packages/🦀️rust/Cargo.toml", ["semio-framework-manifest-app-surface"], ["--lib", "unresolved_action_choices_follow_neutral_catalog_contract", "--", "--nocapture"]);
  }
}


/** ⛽️ Proves original shared staged payload source, page birth and commit funding against Buffer and the system allocator. */
class StagedPayloadFundingTestScript extends BundleScript {
 async run(segments: string[]): Promise<void> {
  const { rest } = resolveTestLevel(segments);
  await runBudgetedTestCommand(process.execPath, ["test", resolve(this.repoRoot, "🔨️modules/🧵️job/🧪️tests/⛽️staged-projection/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true });
  await testCargo(this.invocation,createIssuedResourceControlV1(this.invocation,"framework:tests",0),this.repoRoot, "🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml", ["semio-framework-job"], rest.length ? rest : ["--lib", "-E", "test(original_staged_projection_admits_source_and_writer_frontiers_independently) | test(original_staged_projection_reuses_its_retained_refused_backing_without_allocation) | test(original_boxed_staged_payload_initializes_and_projects_under_fixed_small_copy_grants) | test(original_in_place_payload_builder_preserves_fixed_grants_source_pages_and_all_cancellation_frontiers) | test(original_payload_authority_admits_once_and_borrowed_contexts_preserve_original_ledger_and_receipts) | test(original_payload_pages_preserve_narrow_capacity_exact_extents_and_native_conservation) | test(original_session_frame_return_has_exact_custody) | test(payload_retirement::tests)", "--status-level", "pass", "--final-status-level", "all", "--", "--nocapture"]);
  await runBudgetedTestCommand(process.execPath, ["test", resolve(this.repoRoot, "🔨️modules/⏯️tool-run/🎞️tick/🛫️encode/🧪️tests/🟦️.ts"), resolve(this.repoRoot, "🔨️modules/⏯️tool-run/🎞️tick/⏭️successor/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🔨️modules/⏯️tool-run/📨️payload/🌊️stream/📡️wire/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true });
  await testCargo(this.invocation,createIssuedResourceControlV1(this.invocation,"framework:tests",0),this.repoRoot, "🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml", ["semio-framework-tool-run"], ["--lib", "-E", "test(original_tick_wire_preserves_writer_columns_and_fixed_grants) | test(original_tick_successor_preserves_original_native_columns_and_funded_metadata_receipt) | test(original_payload_stream)", "--status-level", "pass", "--final-status-level", "all", "--", "--nocapture"]);
 }
}

class CoreModulesTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    const { rest } = resolveTestLevel(segments);
    for(const [manifest,name] of [["🔨️modules/🔏️hash/📦️packages/🦀️rust/Cargo.toml","semio-framework-hash"],["🔨️modules/🔲️pixels/📦️packages/🦀️rust/Cargo.toml","semio-framework-pixels"],["🔨️modules/📏️intrinsic-size/📦️packages/🦀️rust/Cargo.toml","semio-framework-intrinsic-size"],["🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/Cargo.toml","semio-framework-mesh-engine"]])await testCargo(this.invocation,phaseControl,this.repoRoot,manifest,[name],rest.length?rest:["--lib"]);
  }
}

/** 🗜️ Verifies admitted physical compression against the independent zlib and miniz oracles. */
class RetainedPackEncodingTestScript extends BundleScript{
 async run(segments:string[]):Promise<void>{const{rest}=resolveTestLevel(segments);await runBudgetedTestCommand(process.execPath,["test",resolve(this.repoRoot,"🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});await testCargo(this.invocation,createIssuedResourceControlV1(this.invocation,"framework:tests",0),this.repoRoot, "🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml",["semio-framework-pack"],rest.length?rest:["--lib","-E","test(borrowed_projected_pack_section_replay_keeps_original_symbols_and_text_views) | test(borrowed_projected_pack_original_expression_ast_matches_independent_corpus_and_cancellation) | test(borrowed_projected_pack_standard_map_statements_wire_match_portable_oracle) | test(borrowed_projected_pack_native_paged_key_and_text_cross_unicode_pages) | test(borrowed_pack_document_default_codec_exact_original_output_and_cancellation) | test(borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec) | test(borrowed_pack_document_multiple_original_frames_match_canonical_oracle) | test(borrowed_pack_document_raw_body_limit_retains_refusal_receipt_and_output_custody) | test(borrowed_pack_document_source_depth_refuses_explicitly_without_grant_stall) | test(borrowed_pack_document_standard_chunks_preserve_original_and_independent_hash) | test(borrowed_pack_document_chunk_frontiers_cancel_with_exact_original_retirement) | test(required_chunked_forgery_with_valid_header_and_footer_crc_is_refused) | test(record_borrowed_schema_hash_retains_exact_paged_work_and_cancellation) | test(retained_pack_required_chunked_matches_actual_custody_and_independent_crc)","--status-level","pass","--final-status-level","all","--","--nocapture"]);}
}
class DeflateEncodingTestScript extends BundleScript{
  async run(segments:string[]):Promise<void>{
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    const {rest}=resolveTestLevel(segments);
    await testCargo(this.invocation,phaseControl,this.repoRoot,"🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml",["semio-framework-deflate"],rest.length?rest:["--lib","deflate_controlled_"]);
  }
}

/** 🪶️ Verifies both snapshot codecs and native interoperability with an independent SQLite engine. */
class SnapshotSqliteTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
    if (segments[0] === "source") {
      const tests = join(this.root, "../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🟦️.ts");
      await runBudgetedTestCommand(process.execPath, ["test", tests, join(this.root, "../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🧪️tests/🟦️.ts"), join(this.root,"../../🔨️modules/🌱️value/🛬️decode/🧪️tests/🟦️.ts"), ...segments.slice(1)], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
      return;
    }
    if (segments[0] !== undefined && segments[0] !== "native") throw new Error("Unknown neutral SQLite snapshot command");
    const nativeOnly = segments[0] === "native";
    const tests = join(this.root, "../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests");
    if (!nativeOnly) await runBudgetedTestCommand(process.execPath, ["test", join(tests, "🔬️unit/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    await testCargo(this.invocation,phaseControl,this.repoRoot,"🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/Cargo.toml",["semio-framework-io-sqlite-snapshot"],["--lib","--no-fail-fast"]);
    await runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["build", "--manifest-path",cargoPolicy(this.repoRoot,"🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/Cargo.toml").manifestPath,"-p", "semio-framework-io-sqlite-snapshot", "--bin", "semio-io-sqlite-snapshot-oracle"],this.repoRoot,process.env,buildBudgetMs());
    if (!nativeOnly) await runBudgetedTestCommand(process.execPath, ["test", join(tests, "🤝️interoperability/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

/** 🧾️ Checks receiving child work and original IO results with independent neutral oracles. */
class SnapshotReceivingSourceScript extends BundleScript{
 private childBudget():number{const remaining=this.invocation.control.remainingMilliseconds();if(remaining!==null&&remaining<1)throw new Error("Original receiving source deadline exhausted");return remaining===null?0:Math.floor(remaining);}
 async run(args:string[]):Promise<void>{
  if(args.length)throw new Error("Expected test-snapshot-receiving-source");
  const {runOwnedCommand}=await import("../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");const tests=["🛫️snapshot/🧪️tests/🟦️.ts","🧪️tests/nested-io/🟦️.ts"].map(test=>resolve(this.root,"../../🔨️modules/🚪️io/⏱️control",test));
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),...tests],this.repoRoot,"framework:snapshot:receiving:types",this.childBudget(),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",...tests],this.repoRoot,"framework:snapshot:receiving:source",this.childBudget(),{signal:this.invocation.control.signal});
 }
}

class PackageDescriptorValueCodecTestScript extends BundleScript {
  async run(): Promise<void> {
    const status = await captureCargo(this.invocation,this.repoRoot,this.repoRoot,["test","-p","semio-framework-manifest-plugin","--lib","package_descriptor_value_codec_tests::package_descriptor_first_party_codec_preserves_serde_wire_and_required_fields","--","--exact"]);
    if (status !== 0) process.exit(status);
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p <each framework package> --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:lint",0);
    const policy=cargoPolicy(this.repoRoot,"📦️packages/🦀️rust/Cargo.toml");
    await runCargoLintV1({manifestPath:policy.manifestPath,packages:FRAMEWORK_PACKAGES,cwd:this.root,extraArgs:segments,environment:{...process.env,CARGO_TARGET_DIR:policy.targetDirectory}},policy,this.repoRoot,this.invocation,phaseControl);
  }
}

//#region 🔖️Typegen
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

function generatedManifestPath(root: string): string {
  return join(root, "..", "..", "🔨️modules", "🛂️manifest", "🤖️generated", "🪪️manifest", "🟦️.ts");
}

/** 🧬️ Runs the owned framework schema export test, optionally writing its stable projection. */
async function runTypegenExportTest(original:ScriptInvocation,workspace:string,root: string, outPath?: string): Promise<void> {
  const env = outPath === undefined ? process.env : { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = await captureCargo(original,workspace,root,["test",TYPEGEN_TEST_FILTER],env);
  if (status !== 0) {
    console.error("framework typegen: owned schema export failed — see output above.");
    process.exit(status);
  }
}

class GenerateScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const outPath = generatedManifestPath(this.root);
    seedGeneratedFile(outPath);
    await runTypegenExportTest(this.invocation,this.repoRoot,this.root, outPath);
    console.log(`framework typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact schema exporter outside the workspace and emits its canonical output bytes. */
class PreviewGeneratedScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const targetPath = generatedManifestPath(this.root);
    const artifacts=readCargoTestPolicyV1(process.env).artifactDirectory;mkdirSync(artifacts,{recursive:true});
    const temp = mkdtempSync(join(artifacts, "semio-framework-typegen-"));
    let content: Buffer;
    try {
      const outPath = join(temp, basename(targetPath));
      const policy=readCargoTestPolicyV1(process.env),status=await captureCargo(this.invocation,this.repoRoot,this.root,["test","--locked",TYPEGEN_TEST_FILTER],{...process.env,SEMIO_CARGO_TEST_POLICY:JSON.stringify({...policy,targetDirectory:join(temp,"target")}),SEMIO_TYPEGEN_OUT:outPath},false);
      if(status!==0)throw new Error(`framework preview export failed: ${status}`);
      content = readFileSync(outPath);
    } finally {
      rmSync(temp, { recursive: true, force: true });
    }
    const nodes = [{ bytesBase64: content.toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(this.repoRoot, targetPath).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: "framework-manifest", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

/** 🔎️ Validates metadata and byte-compares the owned projection with the committed mirror. */
class CheckScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await runTypegenExportTest(this.invocation,this.repoRoot,this.root);
    console.log("framework typescript mirror is fresh.");
  }
}
//#endregion 🔖️Typegen

/** 🧩️ Proves this owner's complete canonical command ingress consumer contract. */
class CommandIngressConsumerScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-command-ingress-consumer accepts no arguments");
  const {runBudgetedTestCommand}=await import("../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts"),{testLevelBudgetMs}=await import("../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts"),source=resolve(this.root,"../../🔨️modules/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 🌲️ Runs the shared physical directory discovery laws and independent graph oracle. */
class DirectoryDiscoveryScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-directory-discovery accepts no arguments");
  const source=resolve(this.root,"../../🔨️modules/📁️filesystem/🔎️discovery/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 🧫️ Runs the complete shared owner corpus and independent oracle. */
class SourceProjectionScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-source-projection accepts no arguments");
  const {runBudgetedTestCommand}=await import("../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts"),{testLevelBudgetMs}=await import("../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts");
  const source=resolve(this.root,"../../🔨️modules/📁️filesystem/📷️snapshot/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 🪟️ Proves original complete view contexts and their exact physical dictionary retirement. */
class ViewModelOriginalScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
    const phaseControl=createIssuedResourceControlV1(this.invocation,"framework:tests",0);
  if(segments.length!==1||!['source','native'].includes(segments[0]!))throw new Error("test-view-model-original requires source or native");
  if(segments[0]==="source"){
   const source=resolve(this.root,"../../🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/♻️original/🟦️.ts");
   await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});
   await runBudgetedTestCommand(process.execPath,[resolve(this.repoRoot,"../node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:120000,throwOnFailure:true});return;
  }
  await testCargo(this.invocation,phaseControl,this.repoRoot,"📦️packages/🦀️rust/Cargo.toml",["semio-framework-action-bus"],["--lib","tests","--","--nocapture"]);
    await testCargo(this.invocation,phaseControl,this.repoRoot,"📦️packages/🦀️rust/Cargo.toml",["semio-framework-manifest-view-model"],["--lib","resolved_host_context_tests::view_model_original_retirement","--","--nocapture"]);
 }
}
/** 🧰️ Runs the shared exact-instance utility laws through the original framework owner. */
class WindowUtilityNativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-window-utility-native accepts no arguments");
  await testCargo(this.invocation,createIssuedResourceControlV1(this.invocation,"framework:tests",0),this.repoRoot, "📦️packages/🦀️rust/Cargo.toml",["semio-framework"],["--lib","manifest::window_view_context_tests","--","--nocapture"]);
 }
}
const workspace=process.env.SEMIO_COMMAND_WORKSPACE;
if(!workspace||!isAbsolute(workspace))throw new Error("Explicit General command workspace required");

/** 🎟️ Exercises portable admission decisions and actual original-source factory capabilities. */
class OriginalFactoryAdmissionScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length>1||(args.length===1&&args[0]!=="source"))throw new Error("test-original-factory-admission [source]");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🔨️modules/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework"],cwd:this.root,extraArgs:["--lib","action_bus::","--","--nocapture"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0));}}

const router = new ScriptRouter(import.meta.dirname,workspace).register("test-view-model-original",ViewModelOriginalScript).register("test-source-projection",SourceProjectionScript).register("test-directory-discovery",DirectoryDiscoveryScript).register("test-command-ingress-consumer",CommandIngressConsumerScript).register("test-artifact-kind-source", ArtifactKindSourceScript).register("test", TestScript).register("test-fixture-ownership-source", FixtureOwnershipSourceScript).register("test-action-choices", ActionChoicesTestScript).register("test-tool-run-actions", ToolRunActionsTestScript).register("test-history-edit-actions", HistoryEditActionsTestScript).register("test-history-progress", HistoryProgressTestScript).register("test-mutation-inputs", MutationInputsTestScript).register("test-host-effect-invocation", HostEffectInvocationTestScript).register("test-snapshot-sqlite", SnapshotSqliteTestScript).register("test-core-modules", CoreModulesTestScript).register("test-deflate-encoding",DeflateEncodingTestScript).register("test-retained-pack-encoding",RetainedPackEncodingTestScript).register("test-staged-payload-funding",StagedPayloadFundingTestScript).register("test-package-descriptor-value-codec", PackageDescriptorValueCodecTestScript).register("test-wire-retirement-source", WireRetirementSourceScript).register("test-wire-retirement-native", WireRetirementNativeScript).register("test-argument-retirement-native",ArgumentRetirementNativeScript).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript).register("lint", LintScript);

router.register("test-snapshot-receiving-source",SnapshotReceivingSourceScript);
router.register("test-window-utility-native",WindowUtilityNativeScript);
router.register("test-original-factory-admission",OriginalFactoryAdmissionScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
