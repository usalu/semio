import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
/** 🧪️ Source and native test commands: wire, payload, transport, list record, pack schema, retirement, key index, assembly lifetime, jco adapter, artifact kind, returned reads, edit metadata, actor identity, preparation control, config paging, cleanup context, crate boundaries and the DSL architecture proof.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { captureOwnedCommandV1 } from "@semio-tech/process/owned-execution";
import { scriptInvocationEnvironment } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createIssuedResourceControlV1, scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { runAdmittedCargoTestsV1 as runCargoTestsV1, runAdmittedCargoPackageTestsV1, runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { runRepositoryTestCommand } from "@semio-tech/process/test-execution";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runNestedCargoPackageAdapter } from "@semio-tech/repo-lib/authoring";
import { exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";
import { runtimeTypeEnvironmentV1 } from "../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
import { SqliteDatabaseV1 } from "../../../../../../🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts";
import { runtimeResolveV1 } from "../../../../../../🔨️modules/⚙️runtime/🚀️process/🟦️.ts";

//#region 🔎️ScalarWireSource
export class ScalarWireSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testScalarRecordWireFixture } = await import("../../../../🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts");
    testScalarRecordWireFixture();
  }
}
//#endregion 🔎️ScalarWireSource

//#region 🪪️PackSingleOwnerSource
export class PackSingleOwnerSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testPackSingleOwner } = await import("../../../../🔨️modules/🎒️pack/🧪️tests/🪪️single-owner/🟦️.ts");
    testPackSingleOwner();
  }
}
//#endregion 🪪️PackSingleOwnerSource

/** 🏛️ Verifies shared policy and mutation publication authority through exact native laws. */
export class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("canonical-architecture accepts no arguments");
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{ package: "semio-framework-os-store-snapshot-clone", target: { kind: "lib" }, laws: ["law::tests::request_builder_borrows_the_original_snapshot_and_returns_its_exact_registry_lease"] }, { package: "semio-framework-os-list-delta", target: { kind: "lib" }, laws: ["tests::commuting_rows_absorb_to_one_structurally_equal_delta"] }, {
        package: "semio-framework-os-kernel",
        target: { kind: "lib", name: "semio_framework_os_kernel" },
        laws: [
          "plugin_module_schema_exports_match_declared_formats",
          "declared_access_policy_matches_the_language_neutral_truth_table",
          "access_policy_is_closed_by_default_and_deny_overrides_allow",
        ],
      }, {
        package: "semio-framework-os-store-tests-01",
        target: { kind: "lib" },
        laws: [
          "one_item_publication_fixture_matches_the_third_party_json_oracle",
        ],
      }, {
        package: "semio-framework-os-store-tests-04",
        target: { kind: "lib" },
        laws: [
          "artifact_snapshot_root_is_o1_and_generation_stable_until_the_next_event",
          "presence_local_read_is_o1_and_never_clones_the_payload_at_capture",
          "transient_root_is_o1_and_retains_the_exact_pre_reset_value",
        ],
      }, {
        package: "semio-framework-os-store-tests-03",
        target: { kind: "lib" },
        laws: [
          "artifact_store_batch_publication_of_one_mutation_is_the_single_item_case",
          "artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step",
          "artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing",
          "artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history",
          "artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged",
        ],
      }, {
        package: "semio-framework-os-store-tests-02",
        target: { kind: "lib" },
        laws: [
          "retained_member_publication_rejects_wrong_owner_staleness_and_cancels_without_commit",
        ],
      }],progress(event) {
        console.log("canonical-architecture-native " + event.stage + ": " + (event.law ?? ""));
      }});
    console.log("canonical-architecture-native receipts=" + receipts.length);
  }
}

/** 🚦️ Verifies the neutral caller input and transport contract through the independent Source oracle. */
export class CommandTransportSourceTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{if(segments.length)throw Error("test-command-transport-source accepts no arguments");const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🧪️tests/🟦️.ts");await runRepositoryTestCommand(process.execPath,["x","--no-install","tsc","--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot});await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});}
}

/** 🪪️ Keeps all original CLI and SPR I/O laws beside actual retained caller-context laws. */
export class CommandTransportNativeTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{if(segments.length)throw Error("test-command-transport-native accepts no arguments");await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--features","sync,ureq","-E","test(command_transport_neutral_)","--no-fail-fast"]);await runAdmittedCargoPackageTestsV1(["semio-framework-os-pack-tools"], this.repoRoot, this.invocation, ["--lib","-E","test(tests::)","--no-fail-fast"]);await runAdmittedCargoPackageTestsV1(["semio-framework-os-spr-cli"], this.repoRoot, this.invocation, ["--lib","-E","test(tests::)","--no-fail-fast"]);await runAdmittedCargoPackageTestsV1(["semio-framework-os-spr"], this.repoRoot, this.invocation, ["--lib","-E","test(io::native::tests::)","--no-fail-fast"]);}
}

export class ListRecordNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-list-record-boundaries accepts no arguments");await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--no-fail-fast","list_record_"]);}
}

export class ComposedPackSchemaTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-composed-pack-schema accepts no arguments");await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","composed_pack_schema_tests::"]);}
}

export class IeeePayloadNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-native accepts no arguments");await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","ieee_payload_"]);}
}

export class IeeePayloadSourceTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-source accepts no arguments");const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔢️ieee754");await runRepositoryTestCommand(process.execPath,["x","--no-install","tsc","--project",join(root,"🧪️tests/📋️tsconfig.json")],{cwd:this.repoRoot});await runRepositoryTestCommand(process.execPath,["test",join(root,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot});}
}

/** ♻️ Proves canonical intrinsic cleanup has no requests and releases every owned capacity. */
export class IntrinsicRetirementNativeTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-intrinsic-retirement-native accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--features","sync,ureq","value_intrinsic_retirement_full_system_requests_and_release","--no-fail-fast"]);
 }
}

/** 🔗️ Measures actual borrowed UTF8 key index backing and typed refusal ownership. */
export class BorrowedKeyIndexNativeTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-borrowed-key-index-native accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--features","sync,ureq","value_borrowed_key_index_full_requests_and_same_caller_are_admitted","--no-fail-fast"]);
 }
}

/** 🧷️ Enforces actual Store assembly guard lifetimes with closed native client laws. */
export class AssemblyLifetimeNativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-assembly-lifetime-native accepts no arguments");
    const source = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧷️assembly/⏳️lifetime/🧪️tests/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [runtimeResolveV1("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", runtimeTypeEnvironmentV1(), source], { cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true });
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "60000", source], { cwd: this.repoRoot });
  }
}

//#region 🧩️JCO Package Adapter
export class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "generate");
  }
}

export class PreviewGeneratedScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "preview");
  }
}

export class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "check");
  }
}
//#endregion 🧩️JCO Package Adapter

/** 🧩️ Proves this owner's complete canonical command ingress consumer contract. */
export class CommandIngressConsumerScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-command-ingress-consumer accepts no arguments");
  const {runBudgetedTestCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts"),{testLevelBudgetMs}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts"),source=resolve(this.root,"../../🔨️modules/📡️spr/🧵️channel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}

/** 🧱️ Verifies actual OS composition and full original DSL/diagnostic source-law preservation. */
export class DslArchitectureProofScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-dsl-architecture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧱️architecture/🟦️.ts"), join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧬️record-floor/🟦️.ts")], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
  }
}

/** 🪪️ Verifies the OS native artifact-kind admission against the neutral grammar. */
export class ArtifactKindNativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
 if(segments.length)throw Error("test-artifact-kind accepts no arguments");
     const artifactDir = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactDir) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned native output");
    
    
     
    
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.repoRoot,progress: event => console.log(`[artifact-kind] ${event.stage}`),groups: [{ package: "semio-framework-os-store-tests-02", target: { kind: "lib" }, laws: ["artifact_kind_id_follows_owner_neutral_corpus"] }]});
    

 }
}

/** 🧾️ Checks original returned-read registry custody with neutral Ajv and native allocation laws. */
export class ReturnedReadCustodyScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-returned-read-custody accepts no arguments");
  const owner=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧾️return");
  const fixture=JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
  for(const row of fixture.cases)assert.equal(row.expectLastCustody,row.otherAliases===0||row.dropOtherBeforeRelease,row.id);
  const artifactDir=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifactDir)throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned native output");
  
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60000, lawMilliseconds: 120000 },cwd:this.repoRoot,...exactCargoStageEnvironments(),progress:event=>{},groups:[{package:"semio-framework-os-kernel",target:{kind:"lib"},laws:["os_store::component::retained_read_return_retirement::tests::original_returned_read_registry_alias_has_exact_physical_custody","os_store::component::retained_read_return_retirement::tests::original_returned_read_registry_parallel_aliases_credit_exactly_one_frame"]}]});
 }
}

/** 🏭️ Checks the actual returned read owner mount without native compilation. */
export class ReturnedReadFactoryMountScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-returned-read-factory-mount accepts no arguments");
  const remaining=this.invocation.control.remainingMilliseconds();
  if(remaining!==null&&remaining<1)throw Error("Original returned read mount deadline exhausted");
  await runRepositoryTestCommand(process.execPath,["test",join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧾️return/🧪️tests/🏭️mount/🟦️.ts")],{cwd:this.repoRoot,env:process.env,budgetMs:remaining===null?0:Math.floor(remaining),signal:this.invocation.control.signal});
 }
}

/** 🪪️ Runs the original metadata schema and independent SQLite semantic oracle. */
export class NativeEditMetadataSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-native-edit-metadata-source accepts no arguments");
  const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🪪️metadata/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
 }
}

/** 🪪️ Runs the original actor identity schema and independent SQLite semantic oracle. */
export class SyncActorIdentitySourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-sync-actor-identity-source accepts no arguments");
  const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🪪️actor-identity/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
 }
}

/** 🎟️ Verifies original canonical native metadata admission with the existing custody source law. */
export class CanonicalPreparationControlSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-canonical-preparation-control-source accepts no arguments");
  const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source");
  const sources=[join(root,"🧪️tests/🟦️.ts"),join(root,"🎟️control/🧪️tests/🟦️.ts")];
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),...sources],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",...sources],{cwd:this.repoRoot});
 }
}

/** 📋️ Qualifies real Config paged source custody under the original finite caller. */
export class ConfigPagedSourceScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("test-config-paged-source requires source or native");
  if(segments[0]==="source"){
   const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🟦️.ts");
   process.stdout.write(await captureOwnedCommandV1(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
   process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
  }else await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","artifact_store_resident_paged_source_preserves_original_pages_order_and_funded_cleanup","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}

/** 🪙️ Qualifies original Store and Presence cleanup callers under their independently supplied phase authority. */
export class OriginalCleanupContextNativeScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
  if(segments.length)throw Error("test-original-cleanup-context-native accepts no arguments");
  const source=join(this.repoRoot,"🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts");
  process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel"],cwd:this.repoRoot,extraArgs:["--lib","-E","test(interactive_envelope_close_reclaims_every_owner_through_the_job_protocol) | test(field_close_failure_preserves_real_receipts_through_the_original_decode_job) | test(normal_decode_release_records_original_wallet_even_after_child_fault) | test(retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open) | test(registered_rejected_pages_obey_zero_short_and_exact_grants) | test(unadmitted_rejected_pages_obey_zero_short_and_exact_grants) | test(original_field_release_survives_registered_and_unadmitted_refusal)","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
 }
}

/** 📦️ Checks actual independent Rust production boundaries against the portable ownership corpus. */
export class CrateBoundariesSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const cargoPhase=createIssuedResourceControlV1(this.invocation,"source:crate-boundaries",0);
  if(segments.length)throw Error("test-crate-boundaries accepts no arguments");
  const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts");
  process.stdout.write(await captureOwnedCommandV1(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
  process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
 }
}

export class DiagnosticAdmissionCheckScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-diagnostic-admission accepts no arguments");
  const {Database}={ Database: SqliteDatabaseV1 };
  const {replayReportBlocksFinalize,outcomeCodeLevel}=await import("@semio-tech/framework-replication");
  const law:any=JSON.parse(readFileSync(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🧫️fixtures/🚦️admission/🔣️.json"),"utf8"));
  const schema=JSON.parse(readFileSync(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🧫️fixtures/🚦️admission/🧬️schema/🔣️.json"),"utf8"));
  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);assert(Boolean(validate(law)),JSON.stringify(validate.errors));
  const replaySource=readFileSync(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs"),"utf8");
  const retainedSource=readFileSync(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs"),"utf8");
  assert(replaySource.includes("operation.diff(base).stamp_op_index(index as u32).admit()"));
  assert(retainedSource.includes("Stage::Decide"));assert(retainedSource.includes("Option<M::Diff>"));assert(!retainedSource.includes("MutationDiff::retire_cold"));
  assert(!/Mutation::(?:diff|inverse)\(|apply_diff_into\(|encode_op\(/.test(retainedSource));
  for(const port of ["RetainedMutation<S>","SnapshotProjection","decide_retained_mutation","preflight_retained_diff","inverse_retained_mutation","commit_retained_diff","MutationMessageConcat"]){assert(retainedSource.includes(port),port);}
  const database=new Database(":memory:");
  try{
   database.exec("CREATE TABLE severity(level TEXT PRIMARY KEY, rank INTEGER);INSERT INTO severity VALUES('info',0),('warning',1),('error',2),('fatal',3);CREATE TABLE diagnostics(level TEXT, code TEXT, message TEXT, target TEXT, operation INTEGER);CREATE TABLE projection(value INTEGER, apply_calls INTEGER, inverse_calls INTEGER)");
   for(const entry of law.cases){
    const{applied,post,applyCalls,inverseCalls,...message}=entry;
    database.exec("DELETE FROM diagnostics");
    database.query("INSERT INTO diagnostics VALUES(?,?,?,?,?)").run(message.level,message.code,message.message,JSON.stringify(message.target),message.opIndex);
    const reference=database.query("SELECT rank<2 AS applied FROM diagnostics JOIN severity USING(level)").get()as{applied:number};
    const report={fromPosition:0,worst:message.level,outcomes:[{mutationId:"m",editId:"e",opIndex:0,worst:message.level,messages:[message],superseded:false,withdrawn:false}]};
    assert.equal(!replayReportBlocksFinalize(report),Boolean(reference.applied));assert.equal(Boolean(reference.applied),applied);assert.equal(outcomeCodeLevel(message.code),message.level);
    for(const route of ["retained","prefix","history","merge"]){
     database.exec("DELETE FROM projection");database.query("INSERT INTO projection VALUES(?,0,0)").run(law.base);
     database.query("UPDATE projection SET value=value+?,apply_calls=1,inverse_calls=1 WHERE EXISTS(SELECT 1 FROM diagnostics JOIN severity USING(level) WHERE rank<2)").run(law.diff);
     const projected=database.query("SELECT value,apply_calls,inverse_calls FROM projection").get()as{value:number;apply_calls:number;inverse_calls:number};
     assert.deepEqual(projected,{value:post,apply_calls:applyCalls,inverse_calls:inverseCalls},route);
    }
    const row=database.query("SELECT code,message,target,operation FROM diagnostics").get()as{code:string;message:string;target:string;operation:number};
    assert.deepEqual({code:row.code,message:row.message,target:JSON.parse(row.target),opIndex:row.operation},{code:message.code,message:message.message,target:message.target,opIndex:message.opIndex});
   }
   const cursorRoot=join(this.repoRoot,"🧰️framework/🔨️modules/📡️replication/🎮️mutation/📬️preparation");
   const cursorLaw:any=JSON.parse(readFileSync(join(cursorRoot,"🧫️fixtures/🔣️.json"),"utf8"));
   const cursorSchema=JSON.parse(readFileSync(join(cursorRoot,"🧬️schema/🔣️.json"),"utf8"));
   const validateCursor=semioSchemaAjvV1({strict:true,allErrors:true}).compile(cursorSchema);assert(Boolean(validateCursor(cursorLaw)),JSON.stringify(validateCursor.errors));
   assert(cursorLaw.base.rowCount*cursorLaw.base.rowBytes>cursorLaw.grant.maximumCapacityBytes);
   for(const entry of cursorLaw.cases){
    assert(entry.targetRow<cursorLaw.base.rowCount);
    const reference=database.query("SELECT CASE WHEN rank<2 AND ?=0 AND ? IS NULL THEN ?+? ELSE ? END AS publishedScalar, rank<2 AND ?=0 AND ? IS NULL AS published, CASE WHEN ?='decision' THEN 0 ELSE 1 END AS diffDecisions, rank<2 AND COALESCE(?,'') NOT IN('decision','inverse') AS inverseCalls, rank<2 AND COALESCE(?,'') NOT IN('decision','inverse') AS applyCalls FROM severity WHERE level=?").get(Number(entry.lateRefusal),entry.cancelStage,cursorLaw.base.scalar,entry.delta,cursorLaw.base.scalar,Number(entry.lateRefusal),entry.cancelStage,entry.cancelStage,entry.cancelStage,entry.cancelStage,entry.level)as{publishedScalar:number;published:number;diffDecisions:number;inverseCalls:number;applyCalls:number};
    const expected={...entry.expected};delete expected.wholeSnapshotClones;delete expected.terminalEmpty;
    assert.deepEqual({...reference,published:Boolean(reference.published)},expected,entry.name);
   }
  }finally{database.close();}
 }
}

export class OwnedReplayCheckScript extends BundleScript {
 async run(args:string[]):Promise<void> {
  if(args.length)throw Error("test-owned-replay accepts no arguments");
  const {Database}={ Database: SqliteDatabaseV1 };
  const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
  const law=JSON.parse(readFileSync(join(root,"🧪️tests/🧫️fixtures/🧫️replay-retirement/🔣️.json"),"utf8")).ownedReplay as Array<{name:string;base:number;operations:Array<{value:number;fault:string}>;expected:number;coldClones:number;ownedClones:number;terminalOwners:number}>;
  const schema=JSON.parse(readFileSync(join(root,"🧪️tests/🧫️fixtures/🧫️replay-retirement/🧬️schema/📥️owned-replay/🔣️.json"),"utf8"));
  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);assert(Boolean(validate(law)),JSON.stringify(validate.errors));
  const database=new Database(":memory:");
  try {
   database.exec("CREATE TABLE operations(position INTEGER PRIMARY KEY,value INTEGER,fault TEXT);CREATE TABLE original(value INTEGER);CREATE TABLE projection(value INTEGER)");
   for(const row of law) {
    database.exec("DELETE FROM operations;DELETE FROM original;DELETE FROM projection");database.query("INSERT INTO original VALUES(?)").run(row.base);database.query("INSERT INTO projection VALUES(?)").run(row.base);
    for(const [index,operation]of row.operations.entries()) {
     this.invocation.control.signal.throwIfAborted();
     database.query("INSERT INTO operations VALUES(?,?,?)").run(index,operation.value,operation.fault);
     database.query("UPDATE projection SET value=? WHERE ?='none'").run(operation.value,operation.fault);
    }
    const actual=database.query("SELECT (SELECT value FROM projection) AS expected,COUNT(CASE WHEN fault!='message' THEN 1 END) AS coldClones,0 AS ownedClones,2+MAX(CASE WHEN fault='none' THEN 1 ELSE 0 END) AS terminalOwners FROM operations").get();
    assert.deepEqual(actual,{expected:row.expected,coldClones:row.coldClones,ownedClones:row.ownedClones,terminalOwners:row.terminalOwners},row.name);
    assert.deepEqual(database.query("SELECT value FROM original").get(),{value:row.base});
   }
   const inverseRoot=join(root,"📬️paged-one-item/🧬️mutation-apply/🧫️fixtures/♻️inverse-issuer");
   const inverse=JSON.parse(readFileSync(join(inverseRoot,"🔣️.json"),"utf8")) as {cases:Array<{name:string;inverseRows:number;expectedIssuerCalls:number}>};
   const inverseSchema=JSON.parse(readFileSync(join(inverseRoot,"🧬️schema/🔣️.json"),"utf8"));const validateInverse=semioSchemaAjvV1({strict:true,allErrors:true}).compile(inverseSchema);assert(validateInverse(inverse),JSON.stringify(validateInverse.errors));
   database.exec("CREATE TABLE inverse_custody(kind TEXT,issuer TEXT,retained INTEGER)");
   for(const row of inverse.cases) {
    database.exec("DELETE FROM inverse_custody");database.query("INSERT INTO inverse_custody VALUES('original',?,1)").run(row.name);
    for(let index=0;index<row.inverseRows;index++)database.query("INSERT INTO inverse_custody VALUES('inverse',?,1)").run(row.name);
    const actual=database.query("SELECT COUNT(*) AS calls FROM inverse_custody WHERE issuer=? AND retained=1").get(row.name) as {calls:number};assert.equal(actual.calls,row.expectedIssuerCalls);
    database.query("UPDATE inverse_custody SET retained=0 WHERE issuer=?").run(row.name);assert.deepEqual(database.query("SELECT COUNT(*) AS retained FROM inverse_custody WHERE retained=1").get(),{retained:0});
   }
   const cancellationRoot=join(root,"📬️paged-one-item/🧬️mutation-apply/🧫️fixtures/♻️inverse-cancellation");
   const cancellation=JSON.parse(readFileSync(join(cancellationRoot,"🔣️.json"),"utf8")) as {base:{nodes:Array<{text:string}>};cases:Array<{boundary:"decision"|"inverse";inverseRows:number;issuerCalls:number;postText:string}>};
   const cancellationSchema=JSON.parse(readFileSync(join(cancellationRoot,"🧬️schema/🔣️.json"),"utf8")),validateCancellation=semioSchemaAjvV1({strict:true,allErrors:true}).compile(cancellationSchema);assert(validateCancellation(cancellation),JSON.stringify(validateCancellation.errors));
   database.exec("CREATE TABLE cancellation(boundary TEXT,post TEXT);CREATE TABLE canceled_inverse(issuer TEXT)");
   for(const row of cancellation.cases) {
    this.invocation.control.signal.throwIfAborted();
    database.exec("DELETE FROM cancellation;DELETE FROM canceled_inverse");
    database.query("INSERT INTO cancellation VALUES(?,?)").run(row.boundary,cancellation.base.nodes[0]!.text);
    database.query("INSERT INTO canceled_inverse SELECT 'original' FROM cancellation WHERE boundary='inverse'").run();
    assert.deepEqual(database.query("SELECT COUNT(*) AS rows FROM canceled_inverse").get(),{rows:row.inverseRows});
    assert.deepEqual(database.query("SELECT COUNT(*) AS calls FROM canceled_inverse WHERE issuer='original'").get(),{calls:row.issuerCalls});
    assert.deepEqual(database.query("SELECT post FROM cancellation").get(),{post:row.postText});
   }
  } finally { database.close(); }
 }
}

export class CanonicalSourceIdentityCheckScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("Canonical source identity requires source or native");
  const oracle=await import("../../../../🔨️modules/🏪️store/🧵️canonical-edit/🪪️source-identity/🧪️tests/🟦️.ts");oracle.runCanonicalSourceIdentityChecks();
  if(segments[0]==="source")return;
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,...exactCargoStageEnvironments(),groups:[{package:"semio-framework-os-kernel",target:{kind:"lib",name:"semio_framework_os_kernel"},laws:["os_store::canonical_edit::source_identity::tests::canonical_source_identity_streams_original_full_fields_and_closes_every_cancelled_prefix","os_store::canonical_edit::source_identity::tests::canonical_source_identity_refuses_a_foreign_live_root_before_reading_it","os_store::canonical_edit::source_identity::tests::canonical_source_identity_staged64_birth_deep_source_and_cancellation_fit4096","os_store::canonical_edit::source_identity::tests::canonical_source_identity_staged64_refuses_original_source_depth65_and_closes_paid_backing"]}]});
 }
}

export class OriginalVcsCommandScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-original-vcs-command source|native");
  const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
  if(args[0]==="source")return;
  const native=readCargoTestPolicyV1(process.env);await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,0)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:native.rustMinStack},manifestPaths:{"semio-framework-os-kernel":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-os-kernel",target:{kind:"lib",name:"semio_framework_os_kernel"},cargoArgs:["--locked"],laws:["os_store::component::native_vcs_command::tests::original_vcs_request_custody_closes_owned_unicode_and_authors_with_physical_receipts"]}]});
 }
}
