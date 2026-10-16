/** 🧪️ Source and native test commands: wire, payload, transport, list record, pack schema, retirement, key index, assembly lifetime, jco adapter, artifact kind, returned reads, edit metadata, actor identity, preparation control, config paging, cleanup context, crate boundaries and the DSL architecture proof.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { captureOwnedCommandV1 } from "@semio-tech/process/owned-execution";
import { scriptInvocationEnvironment } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { runRepositoryCargoTests, runRepositoryTestCommand, runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runNestedCargoPackageAdapter } from "../../../../../🦑️repo/🔨️modules/📚️library/📽️projection/🧩️package-adapter/📦️publication/🟦️.ts";
import { exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";

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
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{
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
 async run(segments:string[]):Promise<void>{if(segments.length)throw Error("test-command-transport-source accepts no arguments");const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🧪️tests/🟦️.ts");await runRepositoryTestCommand(process.execPath,["x","--no-install","tsc","--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot});await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});}
}

/** 🪪️ Keeps all original CLI and SPR I/O laws beside actual retained caller-context laws. */
export class CommandTransportNativeTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{if(segments.length)throw Error("test-command-transport-native accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--features","sync,ureq","-E","test(command_transport_neutral_)","--no-fail-fast"]);await runRepositoryCargoTests(["semio-framework-os-pack-tools"], this.repoRoot, this.invocation, ["--lib","-E","test(tests::)","--no-fail-fast"]);await runRepositoryCargoTests(["semio-framework-os-spr-cli"], this.repoRoot, this.invocation, ["--lib","-E","test(tests::)","--no-fail-fast"]);await runRepositoryCargoTests(["semio-framework-os-spr"], this.repoRoot, this.invocation, ["--lib","-E","test(io::native::tests::)","--no-fail-fast"]);}
}

export class ListRecordNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-list-record-boundaries accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--no-fail-fast","list_record_"]);}
}

export class ComposedPackSchemaTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-composed-pack-schema accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","composed_pack_schema_tests::"]);}
}

export class IeeePayloadNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-native accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","ieee_payload_"]);}
}

export class IeeePayloadSourceTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-source accepts no arguments");const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔢️ieee754");await runRepositoryTestCommand(process.execPath,["x","--no-install","tsc","--project",join(root,"🧪️tests/📋️tsconfig.json")],{cwd:this.repoRoot});await runRepositoryTestCommand(process.execPath,["test",join(root,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot});}
}

/** ♻️ Proves canonical intrinsic cleanup has no requests and releases every owned capacity. */
export class IntrinsicRetirementNativeTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-intrinsic-retirement-native accepts no arguments");
  await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--features","sync,ureq","value_intrinsic_retirement_full_system_requests_and_release","--no-fail-fast"]);
 }
}

/** 🔗️ Measures actual borrowed UTF8 key index backing and typed refusal ownership. */
export class BorrowedKeyIndexNativeTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("test-borrowed-key-index-native accepts no arguments");
  await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","--features","sync,ureq","value_borrowed_key_index_full_requests_and_same_caller_are_admitted","--no-fail-fast"]);
 }
}

/** 🧷️ Enforces actual Store assembly guard lifetimes with closed native client laws. */
export class AssemblyLifetimeNativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-assembly-lifetime-native accepts no arguments");
    const source = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧷️assembly/⏳️lifetime/🧪️tests/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", source], { cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true });
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
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
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
    
    
     
    
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.repoRoot,progress: event => console.log(`[artifact-kind] ${event.stage}`),groups: [{ package: "semio-framework-os-store-tests-02", target: { kind: "lib" }, laws: ["artifact_kind_id_follows_owner_neutral_corpus"] }]});
      console.log(`[DEBUG] artifact-kind native laws=${receipts.reduce((count, receipt) => count + receipt.assertions, 0)}`);
    

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
  
  const receipts=await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60000, lawMilliseconds: 120000 },cwd:this.repoRoot,...exactCargoStageEnvironments(),progress:event=>console.log("[DEBUG] returned-read native "+event.stage),groups:[{package:"semio-framework-os-kernel",target:{kind:"lib"},laws:["os_store::component::retained_read_return_retirement::tests::original_returned_read_registry_alias_has_exact_physical_custody","os_store::component::retained_read_return_retirement::tests::original_returned_read_registry_parallel_aliases_credit_exactly_one_frame"]}]});console.log("[DEBUG] returned-read custody receipts "+JSON.stringify(receipts));
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
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
 }
}

/** 🪪️ Runs the original actor identity schema and independent SQLite semantic oracle. */
export class SyncActorIdentitySourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-sync-actor-identity-source accepts no arguments");
  const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🪪️actor-identity/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
  await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
 }
}

/** 🎟️ Verifies original canonical native metadata admission with the existing custody source law. */
export class CanonicalPreparationControlSourceScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-canonical-preparation-control-source accepts no arguments");
  const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source");
  const sources=[join(root,"🧪️tests/🟦️.ts"),join(root,"🎟️control/🧪️tests/🟦️.ts")];
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...sources],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
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
   process.stdout.write(await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
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
  process.stdout.write(await captureOwnedCommandV1(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
  process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",source],this.repoRoot,{control:cargoPhase,env:scriptInvocationEnvironment(this.invocation,process.env),output:{maximumBytes:1048576,maximumLines:8192}}));
 }
}
