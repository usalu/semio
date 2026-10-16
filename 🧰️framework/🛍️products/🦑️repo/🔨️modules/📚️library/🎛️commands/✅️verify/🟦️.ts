/** ✅️ `verify` command family: the gate dispatcher over every verification lane.
 * @see 📜️script.ts
 */
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { runDependencyVerification } from "../../🕸️dependencies/⚖️truth/🟦️.ts";
import { dispatchOwnedScriptRoute, orchestratorBudgetOpts, runCmd, semioBuildParallelFlag } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { canonicalArchitectureEnvironment } from "../../⚡️caching/🦀️cargo/🟦️.ts";
import { taxonomyImplementationFilesystemFindings, loadCatalogTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { policyReadFileSafe, policyReadModuleTreeSource } from "../../🔍️discovery/📖️source-access/🟦️.ts";
import { verifyTaxonomy, verifyTaxonomyScopesClosed } from "../../🧹️normalization/🟦️.ts";
import { policyArtifactOwnershipFieldParity } from "../../🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts";
import { policyArtifactSchemaBreaches } from "../../🧬️schema/🗿️artifact/⚖️laws/📋️aggregate/🟦️.ts";
import { policyAppSchemaBreaches } from "../../🧬️schema/🗺️surface/⚖️laws/📋️aggregate/🟦️.ts";
import { policyInferenceFamilyBreaches } from "../../🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts";
import { abstractionOwnershipChecks } from "../../📏️ownership/🏛️abstraction/✅️verification/🟦️.ts";
import { policyAbstractionOwnershipBreaches } from "../../📏️ownership/🏛️abstraction/⚖️law/🟦️.ts";
import { policyIndexedGeneratedOutputBreaches } from "../../⚖️laws/📇️indexed-generated-output/🟦️.ts";
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { checkStorybookFreshness } from "./🎨️storybook-freshness/🟦️.ts";
import { interactivityAllAppActionDispositionFailures, interactivityAllAppActionProductionFailures, interactivityAllAppDiscovery } from "../../⚖️laws/🔬️interactivity-audit/🧭️all-app/🟦️.ts";
import { interactivityDbIoB1B6Failures, interactivityP1qR4Failures } from "../../⚖️laws/🔬️interactivity-audit/🗄️database/🟦️.ts";
import { interactivityDatabaseCatalogBootstrapFailures, interactivityDatabaseCompactionFailures, interactivityDatabaseCreateCatalogFailures, interactivityDatabaseSyncHelloFailures } from "../../⚖️laws/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle/🟦️.ts";
import { INTERACTIVITY_AUDIT_ASYNC_FILE, INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY, INTERACTIVITY_AUDIT_DB_CLI_FILE, INTERACTIVITY_AUDIT_DB_COMPACT_FILE, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY, INTERACTIVITY_AUDIT_DB_INDEX_FILE, INTERACTIVITY_AUDIT_DB_NEO4J_FILE, INTERACTIVITY_AUDIT_DB_POSTGRES_FILE, INTERACTIVITY_AUDIT_DB_QUERY_FILE, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE, INTERACTIVITY_AUDIT_DB_SQLITE_FILE, INTERACTIVITY_AUDIT_DB_STATE_FILE, INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY, INTERACTIVITY_AUDIT_DB_SYNC_FILE, INTERACTIVITY_AUDIT_DB_WAL_FAMILY, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY, INTERACTIVITY_AUDIT_PACK_FORMAT_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS, INTERACTIVITY_AUDIT_SEVERITY, INTERACTIVITY_AUDIT_SURFACE_LANE_FILE, INTERACTIVITY_AUDIT_UI_ENGINE_FILE, INTERACTIVITY_AUDIT_UI_ROOTS, INTERACTIVITY_AUDIT_WINIT_HOST_FILE, INTERACTIVITY_P1X_CONTRACT_FILE, INTERACTIVITY_P1Y_CONTRACT_FILE, INTERACTIVITY_P1Z_CONTRACT_FILE } from "../../⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts";
import { interactivityMountedEngineSurfaceLifetimeFailures, interactivityMountedPreparedRenderFailures, interactivityMountedSurfaceLaneFailures } from "../../⚖️laws/🔬️interactivity-audit/🖥️mounted/🖼️mounted-surface/🟦️.ts";
import { HISTORY_CLOSURE_RULES, historyClosureFindings, historyClosureSources, policyHistoryClosureBreaches } from "../../⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts";
import { interactivityAuditRun } from "../../⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts";
import { policyStandardsCoverageBreaches, policyStandardSubsetVocabularyBreaches } from "../../⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🟦️.ts";
import { policyDocumentAppShapeBreaches, policyOsStateAuthorityBreaches } from "../../⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts";
import { policyPackageLanguagePurityBreaches } from "../../⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts";
import { policyDiscoverCrateDirs } from "../../⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { policyHandcraftedSpecP3Breaches, policyListSemanticVocabularyScanFiles, policyRepositoryOwnedRoots, policySemanticVocabularyBreaches } from "../../⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🟦️.ts";
import { policyMutationOutcomeMergePolicyBreaches } from "../../⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts";
import { policyApaBreaches } from "../../⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts";
import { policyBannedNameStemBreaches, policyEmojiPrefixBreaches, policyModeCompletenessBreaches, policyPluginBuilderBreaches, policyPluginRootShapeBreaches, policyWindowCompletenessBreaches } from "../../⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🟦️.ts";
import { policyReadOsStoreEvidence, policyReadRustPolicyFamily, policyReadRustPolicySource } from "../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { repoTaxonomy, taxonomyOption } from "../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";
import { toolJobArtifactEnvelopeRejectionTransferExact } from "../../⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
import { toolJobFactoryProofActivationScan } from "../../⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobFixedOperationFixtureRun, toolJobFixedOperationRustFixtureSource, toolJobSharedFrameworkActionFixtureRun } from "../../⚖️laws/🎯️tool-job-coverage/⚙️fixed-operation/🟦️.ts";
import { toolJobLiveFixedReplayExact } from "../../⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts";
import { toolJobCoverageRun } from "../../⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts";
import { POLICY_BREACH_GATES } from "./🚨️policy-breach-gates/🟦️.ts";
import { verifyComposedChildRefAssetCount, verifyComposedChildRefRows } from "./🧷️composed-child-refs/🟦️.ts";
import { censusGeneratedCorruption } from "./🧟️generated-corruption/🟦️.ts";
import { runAgnosticGate, runDebugTagCensusGate, runDocstringCensusGate, runDocstringEmojiGate, runInterfaceImportGate, runMutationCapGate, runSourceCensusGate } from "./🚦️gates/🟦️.ts";
import { rustWarningTargetScope } from "./🦀️rust-warnings/🟦️.ts";
import type { SelfTestLanes } from "../../⚖️laws/🧪️self-test-lanes/🟦️.ts";

//#region 🔖️VerifyScript

export abstract class VerifyScript extends Script {
  protected abstract readonly lanes: SelfTestLanes;

  async run(segments: string[]): Promise<void> {
    if (await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", ...segments])) return;
    if (segments[0] === "canonical-architecture") {
      if (segments.length !== 1) throw new Error("Expected verify canonical-architecture");
      this.runCanonicalArchitecture();
      return;
    }
    if (segments[0] === "taxonomy") {
      await this.runTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-outcome-law") {
      this.runMutationOutcomeLaw();
      return;
    }
    if (segments[0] === "history-closure") {
      this.runHistoryClosure(segments.slice(1));
      return;
    }
    if (segments[0] === "composed-child-refs") {
      this.runComposedChildRefs(segments.slice(1));
      return;
    }
    if (segments[0] === "semantic-vocabulary") {
      this.runSemanticVocabulary();
      return;
    }
    if (segments[0] === "package-purity") {
      this.runPackagePurity();
      return;
    }
    if (segments[0] === "generated-corruption") {
      this.runGeneratedCorruption(segments.slice(1));
      return;
    }
    if (segments[0] === "rust-warnings") {
      await this.runRustWarnings(segments.slice(1));
      return;
    }
    if (segments[0] === "production-placeholders") {
      await runSourceCensusGate(this.root, "placeholders");
      return;
    }
    if (segments[0] === "docstrings") {
      await (segments[1] === "emoji-unique" ? runDocstringEmojiGate(this.root, segments.slice(2)) : runDocstringCensusGate(this.root, segments[1]));
      return;
    }
    if (segments[0] === "agnostic") {
      await runAgnosticGate(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-caps") {
      await runMutationCapGate(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "debug-tags") {
      await runDebugTagCensusGate(this.root);
      return;
    }
    if (segments[0] === "interface-owners") {
      await runInterfaceImportGate(this.root);
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "commands") {
      await runSourceCensusGate(this.root, "commands");
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "tool-jobs") {
      await this.runToolJobCoverage(segments.slice(2));
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "apps") {
      await this.runInteractivityApps(segments.slice(2));
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1q-b1-b6") {
      this.runInteractivityP1qB1B6();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1w") {
      this.runInteractivityP1w();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1x") {
      this.runInteractivityP1x();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1y") {
      this.runInteractivityP1y();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1z") {
      this.runInteractivityP1z();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p5d") {
      this.runInteractivityP5d();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p5e") {
      this.runInteractivityP5e();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p3mn") {
      this.runInteractivityP3mn();
      return;
    }
    if (segments[0] === "interactivity") {
      await this.runInteractivityAudit();
      return;
    }
    if (segments[0] === "dependencies") {
      this.runDependencyFreeze(segments.slice(1));
      return;
    }
    if (segments[0] === "layering") {
      this.runLayering(segments.slice(1));
      return;
    }
    if (segments[0] === "browser-actor-host-context") {
      if (segments[1] === "worker") {
        const result = Bun.spawn(["bun", join(this.root, "node_modules/vitest/vitest.mjs"), "run", "--config", join(this.root, "🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts"), "-t", "browser document actor transfers one verified cold pair", "--maxWorkers=1", "--testTimeout=300000", "--silent=false", "--disableConsoleIntercept"], { cwd: this.root, stdout: "inherit", stderr: "inherit" });
        if (await result.exited !== 0) throw new Error("Browser actor host context worker test failed");
        return;
      }
      const { testBrowserActorHostContext } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧪️tests/🪟️host-opening-context/🟦️.ts");
      testBrowserActorHostContext();
      return;
    }
    if (segments[0] === "resolved-host-context") {
      const { testResolvedHostContext } = await import("../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts");
      testResolvedHostContext();
      return;
    }
    if (segments[0] === "host-ownership") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "--", "--nocapture", "--test-threads=1", ...["concrete_window_instances_round_trip_without_kind_collapse", "context_menu_point_resolves_the_exact_concrete_window_instance", "canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript", "build_os_commands_covers_every_wired_setting"]], this.root);
      return;
    }
    
    if (segments[0] === "shared-dynamic-value") {
      const { testSharedDynamicValueOracle } = await import("../../../../../../🔨️modules/🌱️value/🧪️tests/🔣️json-projection/🟦️.ts");
      testSharedDynamicValueOracle();
      return;
    }
    if (segments[0] === "shared-artifact-addressing") {
      const { testContentIdOracle, testSharedArtifactAddressingOracle } = await import("../../../../../💻️os/🔨️modules/🏪️store/🧪️tests/🪪️artifact-addressing/🟦️.ts");
      testSharedArtifactAddressingOracle();
      testContentIdOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "artifact_addressing", "--", "--nocapture"], this.root);
      return;
    }
    
    if (segments[0] === "retained-window-input") {
      const { testRetainedWindowInputOracle } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🟦️.ts");
      testRetainedWindowInputOracle();
      const { testRecursiveOwnedDocumentReplacementOracle } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🟦️.ts");
      testRecursiveOwnedDocumentReplacementOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", segments[2] ?? "retained_window_input", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "publication-retirement-authority") {
      const { testPublicationRetirementAuthorityOracle } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🟦️.ts");
      testPublicationRetirementAuthorityOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "publication_retirement", "--", "--nocapture"], this.root);
      return;
    }
    
    if (segments[0] === "dag-demo-ownership") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-artifact-infinite-dag", "--lib", "--", "--nocapture"], this.root);
      return;
    }
    
    
    
    if (segments[0] === "snapshot-read-retirement") {
      const { testSnapshotReadRetirement } = await import("../../../../../💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🟦️.ts");
      testSnapshotReadRetirement();
      runCmd("bun", [this.root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", this.root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🟦️.ts"], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "snapshot_read_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "semio-envelope-identity") {
      const { runCmd } = await import("../../🏃️process/🟦️.ts");
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      const ownerRoot = this.root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio";
      const { testSemioEnvelopeIdentity } = await import(ownerRoot + "/🧪️tests/🪪️envelope-identity/🟦️.ts");
      testSemioEnvelopeIdentity();
      runCmd("bun", [this.root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", ownerRoot + "/🧪️tests/🪪️envelope-identity/🟦️.ts"], { cwd: this.root });
      if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "semio_envelope_identity", "--", "--nocapture"], this.root);
      return;
    }
    
    
    if (segments[0] === "reset-document-ownership") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-reasoning-wires", "-p", "semio-s-artifact-trinity-rewriting", "--lib", "reset_document_ownership", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "wires-pointer-move") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-reasoning-wires", "--lib", "wires_pointer_move_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "jack-results-window-transient") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-trinity-jack", "--lib", "results_window_large_output_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "shared-value-canonical-json") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "shared_value_canonical_json", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "shared-value-clone") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "shared_value_clone", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "shared-map-delta") {
      const { testSharedMapDeltaOracle } = await import("../../../../../../🔨️modules/📡️replication/🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source/🟦️.ts");
      testSharedMapDeltaOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "shared_map_delta", "--", "--nocapture"], this.root);
      return;
    }
    
    
    if (segments[0] === "artifact-contract-ownership") {
      abstractionOwnershipChecks(this.root);
      const fixture = JSON.parse(readFileSync(join(this.root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧫️fixtures/🧪️abstraction-ownership/🔣️.json"), "utf8")) as { artifactSchemas: string[] };
      const packages = [...new Set(fixture.artifactSchemas.map((path) => {
        const [owner] = path.split("/🏅️standards/");
        if (owner === undefined) throw new Error("Artifact schema owner required");
        const manifest = Bun.TOML.parse(readFileSync(join(this.root, owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as { package: { name: string } };
        return manifest.package.name;
      }))];
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      const packageArgs = packages.flatMap((name) => ["-p", name]);
      if (segments[1] === "test") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "--no-fail-fast", ...packageArgs, "committed_diff", "--", "--nocapture"], this.root);
        console.log(`artifact-contract-ownership: committed diff generation, canonical encoding and application checks completed for ${packages.length} selected artifact crates.`);
        return;
      }
      await runCargo(["check", "--manifest-path", "Cargo.toml", "--tests", ...packageArgs], this.root);
      console.log(`[verify artifact-contract-ownership] ${packages.length} artifact crates and their tests compile.`);
      return;
    }
    
    if (segments[0] === "store-reload-publication") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "artifact_store_reset_preserves_capacity_for_retained_batch_publication", "--", "--nocapture"], this.root);
      return;
    }
    
    
    if (segments[0] === "window-config-provisioning-lifecycle") {
      if (segments.length !== 1) throw new Error("window-config-provisioning-lifecycle accepts no arguments");
      const testPath = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/🚪️provisioning-lifecycle/🟦️.ts");
      const { testWindowConfigProvisioningLifecycleOracle } = await import(testPath);
      testWindowConfigProvisioningLifecycleOracle();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.root });
      return;
    }
    if (segments[0] === "window-config-retained-pack-load") {
      if (segments.length !== 2 || segments[1] !== "native") throw new Error("window-config-retained-pack-load requires native");
      const testPath = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🟦️.ts");
      const { testWindowConfigRetainedPackLoadFixture } = await import(testPath);
      testWindowConfigRetainedPackLoadFixture();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.root });
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "window_config_retained_pack_load_", "--", "--nocapture", "--test-threads=1"], this.root);
      return;
    }
    
    
    
    
    
    
    
    
    
    
    
    if (segments[0] === "block3d-window-transient") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-block-3d", "--lib", "preview_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "ephemeral-transfer-preparation") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "ephemeral_transfer_preparation", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "owned-document-closure") {
      const { testOwnedDocumentClosureOracle } = await import("../../../../../💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🟦️.ts");
      testOwnedDocumentClosureOracle();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🟦️.ts")], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "owned_document_closure", "--", "--nocapture"], this.root);
      }
      return;
    }
    
    
    
    
    
    if (segments[0] === "framework-viewport-ownership") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪟️poses");
      const { testViewportOwnership } = await import(`${testRoot}/🟦️.ts`);
      testViewportOwnership();
      runCmd("bun", [this.root + "/node_modules/prettier/bin/prettier.cjs", "--check", ...["◻️2d", "🧊️3d"].map((dimension) => this.root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/" + dimension + "/🧬️schema/🔗️.graphql")], { cwd: this.root });
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-viewport", "--lib", "viewport_ownership_", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "viewport_ownership_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "framework-viewport-projection") {
      const validation = join(this.root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts");
      runCmd("bun", [validation, "framework-viewport-projection", ...segments.slice(1)], { cwd: this.root });
      return;
    }
    if (segments[0] === "node-graph-viewport-ownership") {
      const nativePhase = segments[2];
      if (segments.length > 3 || (segments[1] !== undefined && segments[1] !== "native") || (nativePhase !== undefined && nativePhase !== "renderer-host" && nativePhase !== "consumers")) {
        throw new Error("node-graph-viewport-ownership accepts native with the optional renderer-host or consumers phase");
      }
      const flowRoot = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust");
      const reactRoot = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript");
      const surfaceRoot = join(this.root, "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust");
      runCmd("bun", [join(flowRoot, "📜️script.ts"), "declarations"], { cwd: flowRoot });
      const { runCargo, runVitest } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      process.env.SEMIO_TEST_LEVEL = "long";
      await runVitest(reactRoot, ["--silent=false", "--reporter=verbose", "-t", "uses the live session viewport|matches the shared neutral viewport schema"], "../../🧪️tests/🎚️config/🟦️.ts");
      if (segments[1] === "native") {
        if (nativePhase === undefined || nativePhase === "renderer-host") {
          runCmd("bun", [join(surfaceRoot, "📜️script.ts"), "wasm"], { cwd: surfaceRoot, env: { ...process.env, SEMIO_WORKSPACE_ROOT: this.root, REPO_ROOT: this.root } });
          for (const [packageName, filters] of [
            ["semio-framework-ui-scene", ["typed_scene_neutral_catalog_matches_native_serde_contracts"]],
            ["semio-framework-surface-node-graph", ["scene_retained_tests::"]],
            ["semio-framework-os-renderer-wgpu", ["node_graph_attach_tests", "concrete_window_instances_round_trip_without_kind_collapse", "context_menu_point_resolves_the_exact_concrete_window_instance", "canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript", "build_os_commands_covers_every_wired_setting"]],
          ] as const) {
            for (const filter of filters) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", filter, "--", "--nocapture"], this.root);
          }
        }
        if (nativePhase === undefined || nativePhase === "consumers") {
          const consumers = [
          ["semio-s-artifact-architect-program", ["every_command_round_trips_text_and_binary_under_its_declared_wire_keyword"]],
          ["semio-s-artifact-reasoning-wires", ["every_command_round_trips_through_text_and_binary"]],
          ["semio-hub-space", ["node_graph_viewport_writes_typed_workflow_camera_config"]],
          ["semio-s-artifact-dag-dag", ["every_command_round_trips_through_text_and_binary"]],
          ["semio-s-artifact-flow-flow", ["node_graph_viewport_moves_the_camera"]],
          ["semio-s-artifact-mathematical-equation", ["node_graph_viewport_writes_config_not_mutations"]],
          ["semio-s-artifact-trinity-rewriting", ["trinity_rewriting_command_text_and_binary_round_trip"]],
          ["semio-s-artifact-trinity-jack", ["trinity_jack_command_text_and_binary_round_trip"]],
          ] as const;
          const filters = [...new Set(consumers.flatMap(([, filters]) => [...filters]))];
          await runCargo(["test", "--manifest-path", "Cargo.toml", ...consumers.flatMap(([packageName]) => ["-p", packageName]), "--lib", "--", "--nocapture", "--test-threads=1", ...filters], this.root);
          for (const packageName of ["semio-s-artifact-procedural-2d-editor", "semio-s-artifact-procedural-generation3d"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--test", "editor-routes", "--", "--nocapture", "--test-threads=1", "every_command_round_trips_through_text_and_binary"], this.root);
        }
      }
      return;
    }
    
    if (segments[0] === "framework-job-physical-close") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close");
      const { testJobPayloadPhysicalClose } = await import(`${testRoot}/🟦️.ts`);
      testJobPayloadPhysicalClose();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-job", "--lib", "--", "--nocapture", "--test-threads=1"], this.root);
      return;
    }
    if (segments[0] === "framework-retained-pack-ownership") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws");
      const { testRetainedPackPhysicalOwnership } = await import(`${testRoot}/🟦️.ts`);
      testRetainedPackPhysicalOwnership();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "outer-native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-host", "--lib", "retained_pack_outer_", "--", "--nocapture"], this.root);
        return;
      }
      if (segments[1] === "catalog-native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_catalog_", "--", "--nocapture"], this.root);
        return;
      }
      if (segments[1] === "value") return;
      if (segments[1] === "diagnostic") return;
      if (segments[1] === "inflater") return;
      if (segments[1] === "inflater-native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-deflate", "--lib", "retained_inflater_physical_", "--", "--nocapture", "--test-threads=1"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_inflater_physical_", "--", "--nocapture", "--test-threads=1"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_pipeline_diagnostic_deflate_", "--", "--nocapture", "--test-threads=1"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "write_then_read_round_trip_with_compressed_segment_and_chunk", "--", "--nocapture", "--test-threads=1"], this.root);
        for (const packageName of ["semio-s-artifact-procedural-2d-io-binary", "semio-s-artifact-procedural-3d-io-binary"]) {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", "retained_mounted_laws", "--", "--nocapture", "--test-threads=1"], this.root);
        }
        return;
      }
      if (segments[1] === "diagnostic-native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        for (const filter of [
          "retained_pack_pipeline_diagnostic_",
          "retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable",
          "retained_anchor_rejects_hostile_crc_and_requires_explicit_close",
        ]) {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", filter, "--", "--nocapture"], this.root);
        }
        return;
      }
      if (segments[1] === "value-native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        for (const [packageName, filters] of [
          ["semio-framework-replication", ["retained_paged_list_capacity_admission_and_exact_release_grants"]],
          ["semio-framework-pack", ["retained_symbol_table_", "retained_pack_catalog_", "write_then_read_round_trip_with_compressed_segment_and_chunk"]],
          ["semio-framework-os-kernel", ["retained_value_", "retained_record_body_"]],
          ["semio-s-artifact-procedural-2d-host", ["every_fourteen_variant_decodes_through_retained_structural_grants"]],
          ["semio-s-artifact-procedural-2d-io-binary", ["retained_mounted_laws"]],
          ["semio-s-artifact-procedural-generation3d", ["every_fourteen_variant_decodes_through_retained_structural_grants"]],
          ["semio-s-artifact-procedural-3d-io-binary", ["retained_mounted_laws"]],
        ] as const) {
          for (const filter of filters) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", filter, "--", "--nocapture"], this.root);
        }
        return;
      }
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_source_laws", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "write_then_read_round_trip_with_compressed_segment_and_chunk", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-io-binary", "--lib", "retained_mounted_laws", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-host", "--lib", "retained_pack_outer_", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-3d-io-binary", "--lib", "retained_mounted_laws", "--", "--nocapture"], this.root);
      }
      return;
    }
    
    if (segments[0] === "framework-paged-list-ownership") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list");
      const { testPagedListOwnership } = await import(`${testRoot}/🟦️.ts`);
      testPagedListOwnership();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], this.root);
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-contract", "--lib", "fixed_list_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "framework-variant-field-casing") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-value-derive", "--test", "variant_field_casing", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "framework-empty-state-contract") {
      const testRoot = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🚫️empty-state");
      const { testFrameworkEmptyStateContract } = await import(`${testRoot}/🟦️.ts`);
      testFrameworkEmptyStateContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "framework_empty_state_contract_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "window-config-pack-identity") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("window-config-pack-identity accepts only optional native");
      const testPath = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/🪪️pack-identity/🟦️.ts");
      const { testWindowConfigPackIdentityContract } = await import(testPath);
      testWindowConfigPackIdentityContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "window_config_pack_identity_", "--", "--nocapture", "--test-threads=1"], this.root);
      }
      return;
    }
    
    
    
    
    
    
    
    
    if (segments[0] === "media-export-encoding") {
      const { testMediaExportEncodingContract } = await import("../../../../../../🔨️modules/🎠️kernel/🧪️tests/⬇️media-export-encoding/🟦️.ts");
      testMediaExportEncodingContract();
      // 🧬️ `tsc --strict` over the codec twin alone: it is self-contained by construction, whereas the
      // law imports `🎠️kernel/🎬️media-export/🟦️.ts` and would drag the whole renderer graph (and its pre-existing,
      // unrelated Bun-ism diagnostics) into this lane's verdict.
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🔨️modules/🚪️io/🔤️base64")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-io-base64", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "media_export_encoding", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "video-render-export") {
      const { testVideoRenderProgramContract } = await import("../../../../../../🔨️modules/🎠️kernel/🧪️tests/🎞️video-render-program/🟦️.ts");
      const { testVideoRenderJobContract } = await import("../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧵️video-render-job/🟦️.ts");
      const { testRasterVideoAvcPcmContract } = await import("../../../../../../🔨️modules/🖌️raster/🎥️video/🧪️tests/🔬️unit/🟦️.ts");
      const { testVideoRenderHostContract } = await import("../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts");
      testVideoRenderProgramContract();
      testVideoRenderJobContract();
      testRasterVideoAvcPcmContract();
      await testVideoRenderHostContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🔨️modules/🖌️raster/🎥️video")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "video_render", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-raster", "--lib", "video", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "wire_effect_round_trip", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-animate-presentation", "--lib", "export_video", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "file-open-import") {
      const { testFileOpenImportContract } = await import("../../../../../../🔨️modules/🎠️kernel/🧪️tests/📤️file-open-import/🟦️.ts");
      testFileOpenImportContract();
      // 🧬️ `tsc --strict` over the PAGE half alone: it imports nothing, which is the whole point — it is
      // the one module allowed to name `document` on this target, and it must type-check against the DOM
      // lib without dragging the renderer graph (and its unrelated diagnostics) into this lane's verdict.
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚪️host-io")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "file_open_import", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "file_open_import", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "ui-dirty-scope") {
      const { testUiDirtyScopeContract } = await import("../../../../../../🔨️modules/🎠️kernel/🧪️tests/🐢️ui-dirty-scope/🟦️.ts");
      testUiDirtyScopeContract();
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "ui_dirty_scope", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "ui_dirty_scope", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "spawned-job-drive") {
      const { testSpawnedJobDriveContract } = await import("../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts");
      await testSpawnedJobDriveContract();
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "spawned_job_drive", "--", "--nocapture"], this.root);
      }
      return;
    }
    
    
    
    
    
    
    
    if (segments[0] === "window-action-context") {
      const { testWindowActionContextOracle } = await import("../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️window-action-context/🟦️.ts");
      testWindowActionContextOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      for (const filter of ["window_action_context", "apply_drop_committed_queues_the_merged_action_into_input"]) {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", filter, "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "surface-view-context") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", segments[1] === "menu" ? "context_menu_wire" : "surface_context", "--", "--nocapture"], join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"));
      return;
    }
    if (segments[0] === "view-context-integer-carriers") {
      const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "view_context_integer_carrier", "--", "--nocapture"], this.root);
      if (segments[1] === "native") return;
      runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-react:view-state-carriage-check"], { cwd: this.root });
      return;
    }
    if (segments[0] === "window-view-context") {
      if (segments[1] === "native") {
        const { runCargo } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "window_view_context_uses_the_addressed_instance"], join(this.root, "🧰️framework/📦️packages/🦀️rust"));
        return;
      }
      const { testWindowViewContext } = await import("../../../../../../🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts");
      testWindowViewContext();
      return;
    }
    if (segments[0] === "artifact-field-parity") {
      const { testArtifactFieldParityOracle } = await import("../../📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts");
      testArtifactFieldParityOracle();
      if (segments[1] === "test") return;
      const breaches = policyArtifactOwnershipFieldParity(this.root);
      for (const breach of breaches) console.log(`[verify artifact-field-parity] ${breach.path}: missing=${breach.missing.join(",")} extra=${breach.extra.join(",")}`);
      console.log(`[verify artifact-field-parity] ${breaches.length} schema representations disagree with the canonical document field sets.`);
      if (breaches.length && segments[1] !== "report") throw new Error("Artifact schema representations disagree on document fields.");
      return;
    }
    if (segments[0] === "abstraction-ownership") {
      abstractionOwnershipChecks(this.root);
      if (segments[1] === "test") return;
      const breaches = policyAbstractionOwnershipBreaches(this.root);
      for (const breach of breaches) console.log(`[verify abstraction-ownership] ${breach.scope}: ${breach.summary}`);
      console.log(`[verify abstraction-ownership] ${breaches.length} misplaced declarations.`);
      if (breaches.length && segments[1] !== "report") throw new Error("OS-owned preferences or host controls are declared by plugin surfaces.");
      return;
    }
    if (segments[0] === "policy-breach") {
      this.runPolicyBreach(segments[1]);
      return;
    }
    await this.runGate();
    if (segments[0] === "gate") return;
    runCmd("bun", ["repo", "run-many", "-t", "test", "--all", "--exclude", "workspace", ...semioBuildParallelFlag()], { cwd: this.root, ...orchestratorBudgetOpts() });
  }

  /** 🚦️ Reports findings without failing, or enforces the identical structured result. */
  private async runTaxonomy(args: string[]): Promise<void> {
    if (args[0] === "implementation") {
      const mode = args[1];
      if (mode !== "report" && mode !== "enforce" || args.length !== 2) throw new Error(`[verify taxonomy implementation] expected report or enforce, got ${JSON.stringify(args)}.`);
      const controller = new AbortController();
      const cancel = (): void => controller.abort();
      process.once("SIGINT", cancel);
      let previousPhase = "";
      try {
        const findings = await taxonomyImplementationFilesystemFindings(this.root, loadCatalogTaxonomy(), {
          signal: controller.signal,
          onProgress: (progress) => {
            if (progress.phase !== previousPhase || (progress.phase === "walk" ? progress.pathsVisited : progress.pathsClassified) % 4096 === 0) console.error(`[verify taxonomy implementation progress] phase=${progress.phase} visited=${progress.pathsVisited} classified=${progress.pathsClassified} findings=${progress.findings} vanished=${progress.vanishedDirectories}`);
            previousPhase = progress.phase;
          },
        });
        console.log(`[verify taxonomy implementation ${mode}] clean=${findings.length === 0} errors=${findings.length}`);
        for (const finding of findings) console.log(`[verify taxonomy implementation ${mode}] error ${finding.breachId} ${finding.path}: ${finding.actualBasename === null ? "A target boundary must own its language packages." : `Implementation leaf ${JSON.stringify(finding.actualBasename)} must use ${JSON.stringify(finding.expectedBasename)}.`}`);
        if (mode === "enforce" && findings.length > 0) throw new Error(`[verify taxonomy implementation enforce] ${findings.length} error finding(s).`);
      } finally {
        process.removeListener("SIGINT", cancel);
      }
      return;
    }
    const mode = args[0];
    if (mode !== "report" && mode !== "enforce") throw new Error(`[verify taxonomy] expected report or enforce, got ${JSON.stringify(mode)}.`);
    const scopesFrom = taxonomyOption(args, "--scopes-from");
    if (scopesFrom !== undefined) {
      const scopes = readFileSync(resolve(this.root, scopesFrom), "utf8").split("\n").map((line) => line.trim()).filter(Boolean);
      let dirty = 0, done = 0;
      for (const verification of verifyTaxonomyScopesClosed({ repoRoot: this.root, scopes })) {
        const errors = verification.violations.filter((violation) => violation.severity === "error").length;
        dirty += Number(!verification.clean);
        done += 1;
        if (args.includes("--json")) console.log(JSON.stringify(verification));
        else {
          console.log(`[verify taxonomy ${mode}] clean=${verification.clean} errors=${errors} warnings=${verification.violations.length - errors} scope=${verification.scope}${verification.closure === undefined ? "" : ` closure=${verification.closure}`}`);
          for (const violation of verification.violations) console.log(`[verify taxonomy ${mode}] ${violation.severity} ${violation.code} ${violation.path}: ${violation.message}`);
        }
      }
      console.error(`[verify taxonomy ${mode}] scopes=${done} dirty=${dirty} from=${scopesFrom}`);
      if (mode === "enforce" && dirty > 0) throw new Error(`[verify taxonomy enforce] ${dirty} of ${done} scope(s) not clean.`);
      return;
    }
    const scope = taxonomyOption(args, "--scope");
    let previousPhase = "";
    let previousReportAt = 0;
    const verification = verifyTaxonomy({
      repoRoot: this.root,
      ...(scope ? { scope } : {}),
      progress: (progress) => {
        const now = Date.now();
        if (progress.phase === previousPhase && now - previousReportAt < 5000) return;
        previousPhase = progress.phase;
        previousReportAt = now;
        console.error(`[verify taxonomy progress] ${progress.operation}/${progress.phase} ${progress.current}/${progress.total}${progress.path ? ` ${progress.path}` : ""}`);
      },
    });
    const errors = verification.violations.filter((violation) => violation.severity === "error");
    const warnings = verification.violations.filter((violation) => violation.severity === "warning");
    console.log(`[verify taxonomy ${mode}] clean=${verification.clean} errors=${errors.length} warnings=${warnings.length}${scope ? ` scope=${scope}` : ""}`);
    for (const violation of verification.violations) console.log(`[verify taxonomy ${mode}] ${violation.severity} ${violation.code} ${violation.path}: ${violation.message}`);
    if (mode === "enforce" && !verification.clean) throw new Error(`[verify taxonomy enforce] ${errors.length} error finding(s), ${warnings.length} warning finding(s).`);
  }

  /**
   * ⚖️Standalone entry point for the `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
   * gate bundle (`policyMutationOutcomeMergePolicyBreaches`'s 16 rules) — the same checks `runGate()`'s
   * "mutation-outcome / merge-policy law" block enforces, runnable in isolation via
   * `bun ./📜️script.ts verify mutation-outcome-law` (the `mutation-outcome-law` build task in
   * `📋️project.json` wires this ahead of `verify-gate`).
   */
  /**
   * 🪆️Repo-wide law for composed-child rows in committed `*.dsl.semio` documents: a child row's
   * `child_id` and its target `ArtifactRef.artifact_id` must be the SAME string. That is not a style
   * rule — `ChildRestoreProjection::child`
   * (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`) refuses any other pair with
   * `InvalidReference`, so a mismatched asset cannot survive a whole-document `Effect::LoadDocument`
   * and every app that gains an example picker fails at its first boot dispatch with
   * `document archive genesis child projection failed`. Found 2026-09-20 by slice B2c on
   * `📖️playbook`'s demo asset (ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`), where the verb
   * was correct and the ASSET was a month-stale bare-id artefact. `--json` prints the census instead
   * of failing.
   */
  private runComposedChildRefs(segments: string[]): void {
    const rows = verifyComposedChildRefRows(this.root);
    const waived = rows.filter((row) => row.waiver !== undefined);
    const breaches = rows.filter((row) => row.waiver === undefined);
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ assets: verifyComposedChildRefAssetCount(this.root), rows, breaches: breaches.length, waived: waived.length }, null, 2));
      return;
    }
    console.log(`[verify composed-child-refs] assets=${verifyComposedChildRefAssetCount(this.root)} mismatched=${rows.length} waived=${waived.length} breaches=${breaches.length}`);
    for (const row of waived) console.log(`[verify composed-child-refs] waived ${row.path} ${row.slot}: ${row.waiver}`);
    for (const row of breaches) console.error(`[verify composed-child-refs] ${row.path} slot ${row.slot}: child_id ${JSON.stringify(row.childId)} but target artifact_id ${JSON.stringify(row.targetArtifactId)}`);
    if (breaches.length > 0) throw new Error(`[verify composed-child-refs] ${breaches.length} composed-child row(s) whose target artifact_id is not their own child_id`);
    console.log("[verify composed-child-refs] passed.");
  }

  private runMutationOutcomeLaw(): void {
    const breaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
    if (breaches.length > 0) {
      for (const b of breaches) console.error(`[verify mutation-outcome-law] ${b.kind}: ${b.summary}`);
      throw new Error(`[verify mutation-outcome-law] ${breaches.length} breach(es)`);
    }
    console.log("[verify mutation-outcome-law] passed.");
  }

  /**
   * 🚫️ The non-destructive history closure gate (`HISTORY_CLOSURE_RULES`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`
   * §20): the planted-violation self-test first, then one scan of the git-visible `✏️s`, `🧰️framework`, `🌎️hub` sources.
   * `--self-test` runs only the planted cases; `--json` prints the per-rule census instead of failing.
   */
  private runHistoryClosure(segments: string[]): void {
    const cases = this.lanes.historyClosurePolicySelfTests();
    console.log(`[verify history-closure] self-test passed (${cases} cases).`);
    if (segments.includes("--self-test")) return;
    const findings = historyClosureFindings(historyClosureSources(this.root));
    const census = Object.fromEntries(HISTORY_CLOSURE_RULES.map((rule) => [rule.id, findings.filter((finding) => finding.rule === rule.id).length]));
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ census, findings }, null, 2));
      return;
    }
    for (const finding of findings) console.error(`[verify history-closure] ${finding.file}:${finding.line} ${finding.text}`);
    console.log(`[verify history-closure] ${Object.entries(census).map(([rule, count]) => `${rule}=${count}`).join(" ")}`);
    if (findings.length > 0) throw new Error(`[verify history-closure] ${findings.length} closure violation(s)`);
    console.log("[verify history-closure] passed.");
  }

  /**
   * 🚫️Standalone entry point for `policySemanticVocabularyBreaches` — the only consumer of
   * `policyRepositoryOwnedRoots()`, so this is also the runnable proof that the walk roots are the
   * taxonomy's `clean` areas and nothing else. `high` breaches fail; the `Set*` dispatch-variant rule is
   * advisory by its own declaration and is reported without failing.
   */
  private runSemanticVocabulary(): void {
    const breaches = policySemanticVocabularyBreaches(this.root);
    const files = policyListSemanticVocabularyScanFiles(this.root);
    const blocking = breaches.filter((b) => b.priority === "high");
    console.log(`[verify semantic-vocabulary] roots=${policyRepositoryOwnedRoots().join(", ")}`);
    console.log(`[verify semantic-vocabulary] scanned=${files.length} breaches=${breaches.length} high=${blocking.length} advisory=${breaches.length - blocking.length}`);
    for (const b of breaches) console.log(`[verify semantic-vocabulary] ${b.priority} ${b.kind}: ${b.summary}`);
    if (blocking.length > 0) throw new Error(`[verify semantic-vocabulary] ${blocking.length} banned-vocabulary breach(es)`);
    console.log("[verify semantic-vocabulary] passed.");
  }

  /** 📦️Standalone entry point for the Shape V2 package-folder purity rule, as `verify-package-purity` runs it. */
  private runPackagePurity(): void {
    const breaches = policyPackageLanguagePurityBreaches(this.root).filter((b) => b.priority === "high");
    for (const b of breaches) console.error(`[verify package-purity] ${b.kind}: ${b.summary}`);
    if (breaches.length > 0) throw new Error(`[verify package-purity] ${breaches.length} package language purity breach(es)`);
    console.log("[verify package-purity] passed.");
  }

  /**
   * 🔬️Standalone entry point for the generated-code corruption census — the permanent gate against the
   * 2026-09-03 rename-plan codemod incident, where an emoji+VS16 was spliced INTO ASCII identifiers of
   * jco/wit-bindgen output (`WasiCliEnvironmen🔬️t029`). Two independent checks run: every jco world
   * binding's alias is re-derived from the WIT id jco prints beside it and must match byte-for-byte,
   * and every parsed TS/JS file must carry no emoji in code position once comments, string/template
   * literals and regex literals are blanked. `--json` prints the machine-readable census instead.
   */
  private runGeneratedCorruption(segments: string[]): void {
    const census = censusGeneratedCorruption(this.root);
    if (segments.includes("--json")) {
      console.log(JSON.stringify(census, null, 2));
    } else {
      console.log(`[verify generated-corruption] scanned=${census.filesScanned} jco-bindings=${census.jcoFilesScanned} jco-aliases-validated=${census.jcoAliasesValidated}`);
      for (const finding of census.findings) console.error(`[verify generated-corruption] ${finding.kind} ${finding.file}:${finding.line} ${finding.detail}`);
    }
    if (census.findings.length > 0) {
      throw new Error(`[verify generated-corruption] ${census.findings.length} spliced-emoji identifier(s) in ${new Set(census.findings.map((f) => f.file)).size} file(s)`);
    }
    console.log("[verify generated-corruption] passed.");
  }

  /**
   * ⚖️Standalone entry point for one of the 12 [[POLICY_BREACH_GATES]] rules, byte-identical in
   * output to the inline `bun -e` eval it replaces: first line is the raw breach count, then one
   * `kind | scope | summary` line per breach, non-zero exit whenever any breach exists.
   */
  private runPolicyBreach(rule: string | undefined): void {
    const fn = rule ? POLICY_BREACH_GATES[rule] : undefined;
    if (!fn) throw new Error(`[verify policy-breach] unknown rule ${JSON.stringify(rule)}. known: ${Object.keys(POLICY_BREACH_GATES).join(", ")}`);
    const breaches = fn(this.root);
    console.log(breaches.length);
    for (const b of breaches) console.log(b.kind, "|", b.scope, "|", b.summary);
    if (breaches.length > 0) throw new Error(`[verify policy-breach ${rule}] ${breaches.length} breach(es)`);
  }

  /**
   * 🦀️ Zero-warning gate per compilation target (ticket
   * `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet `Z1`; exit checklist item 6) —
   * `verify rust-warnings --target <triple> [-p <crate>…]`. The three targets that must all be
   * clean are native (host), `wasm32-wasip2` (the plugin fleet's real component build) and
   * `wasm32-unknown-unknown` (the pure actor kernel, the one that keeps mobile open). With no
   * explicit `-p`, each target resolves to the crate set that target actually exists to prove.
   *
   * Deny-on-warnings is passed as clippy's trailing `-- -D warnings`, NEVER through `RUSTFLAGS`:
   * `RUSTFLAGS` REPLACES rather than merges with `.cargo/config.toml`'s rustflags (`-Z threads=8`,
   * the wasm32 `getrandom_backend` cfg, mold) and would break every wasm build — the same reason
   * [[runCargoLint]] documents for the identical choice.
   */
  private async runRustWarnings(args: string[]): Promise<void> {
    const { rustWarningScopeChecks } = await import("../../../../../../../🧪️tests/🦀️rust-warnings/🟦️.ts");
    rustWarningScopeChecks(this.root, rustWarningTargetScope);
    if (args.includes("--scope-only")) return;
    const targetIndex = args.indexOf("--target");
    const target = targetIndex >= 0 ? args[targetIndex + 1] : undefined;
    if (targetIndex >= 0 && !target) throw new Error("[verify rust-warnings] --target needs a triple (native | wasm32-wasip2 | wasm32-unknown-unknown).");
    const explicit: string[] = [];
    for (const [index, argument] of args.entries()) {
      const name = args[index + 1];
      if (argument === "-p" && name) explicit.push(name);
    }
    const scope = rustWarningTargetScope(this.root, target);
    const packages = explicit.length > 0 ? explicit : scope.packages;
    if (packages.length === 0) throw new Error(`[verify rust-warnings] no crates resolved for target ${target ?? "native"}.`);
    console.log(`[verify rust-warnings] ${target ?? "native"}: ${packages.length} crate(s)…`);
    for (const pkg of packages) {
      runCmd("cargo", ["clippy", "-p", pkg, ...scope.scopeArgs, ...scope.targetArgs, ...(scope.packageArgs[pkg] ?? []), "--", "-D", "warnings"], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
    console.log(`[verify rust-warnings] ${target ?? "native"} clean.`);
  }

  /**
   * ⏱️ `verify interactivity` — the Phase 0 forbidden-call audit (ticket
   * `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`). Scans [[INTERACTIVITY_AUDIT_UI_ROOTS]] for
   * `block_on`/`run_blocking`/sync-fs/sync-net/sync-clipboard/sync-process/sync-db, and the whole
   * repo (minus `compose/` and [[INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS]]) for
   * direct thread construction/scoped threads/Rayon/Tokio runtime construction. WARN in Phase 0 (report,
   * exit 0); flip [[INTERACTIVITY_AUDIT_SEVERITY]] to `"deny"` at Phase 3 packet P3c.
   */
  private async runInteractivityAudit(): Promise<void> {
    await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", "puzzle-fill-policy-self-tests"]);
    const apps = interactivityAllAppDiscovery(this.root);
    const appSelfTests = await this.lanes.interactivityAllAppDiscoverySelfTests();
    console.log(`[verify interactivity apps] ${apps.descriptors.length} descriptor(s), ${apps.appCount} app declaration(s), ${apps.playgrounds} dashboard playground command(s), ${apps.surfaceCount} dev surface(s) over ${apps.renderers.join("/")}, ${apps.actionCount} action row(s), ${apps.surfaceCoveredAppCount} surface-covered app context(s), ${apps.surfaceMissingAppCount} missing surface context(s), ${apps.gates} dashboard-startable gate(s), ${appSelfTests} hostile/oracle self-test(s).`);
    for (const failure of apps.failures.slice(0, 25)) console.log(`  ${failure}`);
    if (apps.failures.length > 25) console.log(`  ${apps.failures.length - 25} additional all-app discovery failure(s) omitted.`);
    const report = interactivityAuditRun(this.root, this.lanes);
    console.log(`[verify interactivity] severity=${INTERACTIVITY_AUDIT_SEVERITY} scope: ${INTERACTIVITY_AUDIT_UI_ROOTS.length} UI root(s), ${INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS.length} runtime-sanctioned root(s) exempt from the thread-pool rule.`);
    console.log(`[verify interactivity] ${report.findings.length} finding(s) total:`);
    for (const category of Object.keys(report.byCategory).sort()) console.log(`  ${category}: ${report.byCategory[category]}`);
    if (report.blockingBridgeUnlisted.length > 0) {
      console.log(`[verify interactivity] ${report.blockingBridgeUnlisted.length} block_on/run_blocking finding(s) NOT covered by the allowlist:`);
      for (const f of report.blockingBridgeUnlisted) console.log(`  ${f.file}:${f.line}: ${f.text}`);
    }
    for (const category of Object.keys(report.byCategory).sort()) {
      if (category === "blocking-bridge") continue; // already printed above (allowlist-reconciled).
      const inCategory = report.findings.filter((f) => f.category === category);
      console.log(`[verify interactivity] ${inCategory.length} "${category}" finding(s):`);
      for (const f of inCategory) console.log(`  ${f.file}:${f.line}: ${f.text}`);
    }
    if (report.staleAllowlistEntries.length > 0) {
      console.log(`[verify interactivity] ${report.staleAllowlistEntries.length} allowlist entr(y/ies) no longer match any finding (stale — shrink the list):`);
      for (const e of report.staleAllowlistEntries) console.log(`  ${e.file} (${e.pattern}, ${e.phase})`);
    }
    if (report.expectedNeverToMatchEntries.length > 0) {
      console.log(`[verify interactivity] ${report.expectedNeverToMatchEntries.length} allowlist entr(y/ies) kept for the record but structurally invisible to this scanner (e.g. test-mod-only — never counted as stale):`);
      for (const e of report.expectedNeverToMatchEntries) console.log(`  ${e.file} (${e.pattern}, ${e.phase})`);
    }
    if (report.preDeclaredOutOfScope.length > 0) {
      console.log(`[verify interactivity] ${report.preDeclaredOutOfScope.length} allowlist entr(y/ies) pre-declared for future scope widening (not scanned today, not counted as stale):`);
      for (const e of report.preDeclaredOutOfScope) console.log(`  ${e.file} (${e.pattern}, ${e.phase})`);
    }
    const otherCategoryFindings = report.findings.filter((f) => f.category !== "blocking-bridge").length;
    if (INTERACTIVITY_AUDIT_SEVERITY === "warn") {
      console.log("[verify interactivity] WARN mode — reporting only, exit 0.");
      return;
    }
    const failures = apps.failures.length + report.blockingBridgeUnlisted.length + report.staleAllowlistEntries.length + otherCategoryFindings;
    if (failures > 0) throw new Error(`[verify interactivity] DENY mode — ${failures} failure(s) (all-app discovery/dashboard surface coverage, unlisted blocking bridges, stale allowlist entries, or sync-fs/net/clipboard/process/db/thread-pool findings).`);
    console.log("[verify interactivity] DENY mode — clean.");
  }

  /** 🧭️Prints the metadata-derived all-app ledger and enforces its bounded discovery contract. */
  private async runInteractivityApps(args: string[]): Promise<void> {
    const report = interactivityAllAppDiscovery(this.root);
    const selfTests = await this.lanes.interactivityAllAppDiscoverySelfTests();
    if (args.includes("--json")) console.log(JSON.stringify({ ...report, selfTests }, null, 2));
    else console.log(`[verify interactivity apps] descriptors=${report.descriptors.length} extensions=${report.descriptors.filter((descriptor) => descriptor.kind === "extension").length} apps=${report.appCount} playgrounds=${report.playgrounds} renderers=${report.renderers.join("/")} surfaces=${report.surfaceCount} actions=${report.actionCount} migratedActions=${report.migratedActionCount} missingActions=${report.missingActionCount} surfaceCoveredApps=${report.surfaceCoveredAppCount} surfaceMissingApps=${report.surfaceMissingAppCount} gates=${report.gates} failures=${report.failures.length} selfTests=${selfTests}`);
    for (const failure of report.failures.slice(0, 25)) console.error(`[verify interactivity apps] ${failure}`);
    if (report.failures.length > 25) console.error(`[verify interactivity apps] ${report.failures.length - 25} additional discovery or dashboard-surface failure(s) omitted.`);
    if (report.failures.length > 0) throw new Error(`[verify interactivity apps] ${report.failures.length} discovery or dashboard-surface failure(s).`);
    if (args.includes("--actions")) {
      const toolJobs = await toolJobCoverageRun(this.root, this.lanes);
      const dispositionFailures = interactivityAllAppActionDispositionFailures(report.descriptors);
      const productionFailures = interactivityAllAppActionProductionFailures(report.descriptors, toolJobs.acceptedCommandRows, toolJobs.acceptedReservedRoutes);
      const actionFailures = [...dispositionFailures, ...productionFailures];
      console.error(`[verify interactivity apps actions] dispositionFailures=${dispositionFailures.length} productionJoinFailures=${productionFailures.length} acceptedCommands=${toolJobs.acceptedCommandRows.length} acceptedSharedRoutes=${toolJobs.acceptedReservedRoutes.length}`);
      for (const failure of dispositionFailures.slice(0, 12)) console.error(`[verify interactivity apps actions] ${failure}`);
      for (const failure of productionFailures.slice(0, 12)) console.error(`[verify interactivity apps actions] ${failure}`);
      const printedFailures = Math.min(dispositionFailures.length, 12) + Math.min(productionFailures.length, 12);
      if (actionFailures.length > printedFailures) console.error(`[verify interactivity apps actions] ${actionFailures.length - printedFailures} additional action failure(s) omitted from console output.`);
      if (actionFailures.length > 0) throw new Error(`[verify interactivity apps actions] ${actionFailures.length} descriptor action disposition or production-reachability join failure(s).`);
      console.log(`[verify interactivity apps actions] ${report.actionCount} descriptor action row(s) joined to ${toolJobs.acceptedCommandRows.length} accepted production command row(s) and ${toolJobs.acceptedReservedRoutes.length} shared reserved route(s).`);
    }
  }

  /** 🗄️ Runs the isolated P1q B1–B6 retained DB I/O source and hostile-mutation gate. */
  private runInteractivityP1qB1B6(): void {
    this.lanes.interactivityDbIoB1B6SelfTests();
    this.lanes.interactivityP1qR4SelfTests();
    const failures = interactivityDbIoB1B6Failures(
      policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SQLITE_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_POSTGRES_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_NEO4J_FILE),
      policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY),
    );
    failures.push(
      ...interactivityP1qR4Failures(
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PACK_FORMAT_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_INDEX_FILE),
        policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_WAL_FAMILY),
        policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_QUERY_FILE),
        policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_COMPACT_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_CLI_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_STATE_FILE),
      ),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p1q-b1-b6] ${failures.join("; ")}`);
    console.log("[verify interactivity p1q-b1-b6] live-source and hostile mutations clean.");
  }

  /** 🌱️ Runs the isolated P1w retained initial-catalog CAS source and hostile-mutation gate. */
  private runInteractivityP1w(): void {
    this.lanes.interactivityDatabaseCatalogBootstrapSelfTests();
    const failures = interactivityDatabaseCatalogBootstrapFailures(policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY));
    if (failures.length > 0) throw new Error(`[verify interactivity p1w] ${failures.join("; ")}`);
    console.log("[verify interactivity p1w] live-source and hostile mutations clean.");
  }

  /** 🌱️ Runs the isolated P1x retained create-document catalog CAS source and hostile-mutation gate. */
  private runInteractivityP1x(): void {
    this.lanes.interactivityDatabaseCreateCatalogSelfTests();
    const failures = interactivityDatabaseCreateCatalogFailures(
      policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_ASYNC_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_P1X_CONTRACT_FILE),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p1x] ${failures.join("; ")}`);
    console.log("[verify interactivity p1x] live-source and hostile mutations clean.");
  }

  /** 🧹 Runs the isolated P1y retained database-compaction source and hostile-mutation gate. */
  private runInteractivityP1y(): void {
    const compact = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_COMPACT_FILE);
    const engine = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY);
    const snapshot = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE);
    const index = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_INDEX_FILE);
    const contract = policyReadRustPolicySource(this.root, INTERACTIVITY_P1Y_CONTRACT_FILE);
    this.lanes.interactivityDatabaseCompactionSelfTests(compact, engine, snapshot, index, contract);
    const failures = interactivityDatabaseCompactionFailures(compact, engine, snapshot, index, contract);
    if (failures.length > 0) throw new Error(`[verify interactivity p1y] ${failures.join("; ")}`);
    console.log("[verify interactivity p1y] live-source and hostile mutations clean.");
  }

  /** 👋️ Runs the isolated P1z retained sync-hello source and hostile-mutation gate. */
  private runInteractivityP1z(): void {
    const sync = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SYNC_FILE);
    const engine = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY);
    const hub = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY);
    const wal = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_WAL_FAMILY);
    const protocol = policyReadRustPolicySource(this.root, "🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs");
    const contract = policyReadRustPolicySource(this.root, INTERACTIVITY_P1Z_CONTRACT_FILE);
    this.lanes.interactivityDatabaseSyncHelloSelfTests(sync, engine, hub, wal, protocol, contract);
    const failures = interactivityDatabaseSyncHelloFailures(sync, engine, hub, wal, protocol, contract);
    if (failures.length > 0) throw new Error(`[verify interactivity p1z] ${failures.join("; ")}`);
    console.log("[verify interactivity p1z] live-source and hostile mutations clean.");
  }

  /** 🎨️ Runs the isolated P5d mounted prepared-render source and hostile-mutation gate. */
  private runInteractivityP5d(): void {
    this.lanes.interactivityMountedPreparedRenderSelfTests(this.root);
    const failures = interactivityMountedPreparedRenderFailures(
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
      policyReadRustPolicySource(this.root, "🧰️framework/🔨️modules/🖱️ui/🖌️render/🖼️frame/🦀️.rs"),
      policyReadRustPolicySource(this.root, "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs"),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p5d] ${failures.join("; ")}`);
    console.log("[verify interactivity p5d] live-source and hostile mutations clean.");
  }

  /** 📐️ Runs the isolated P5e mounted multi-window resize/surface lane source and mutation gate. */
  private runInteractivityP5e(): void {
    this.lanes.interactivityLiveReconcileSelfTests(this.root);
    this.lanes.interactivityMountedLayoutTextSelfTests(this.root);
    this.lanes.interactivityMountedFrameTransactionSelfTests(this.root);
    this.lanes.interactivityMountedPreparedRenderSelfTests(this.root);
    this.lanes.interactivityMountedSurfaceLaneSelfTests(this.root);
    const failures = interactivityMountedSurfaceLaneFailures(
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_SURFACE_LANE_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_UI_ENGINE_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_WINIT_HOST_FILE),
      policyReadModuleTreeSource(this.root, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p5e] ${failures.join("; ")}`);
    console.log("[verify interactivity p5e] live-source and hostile mutations clean.");
  }

  /** 🖥️ Runs the isolated P3mn paired engine-surface lifetime source and hostile-mutation gate. */
  private runInteractivityP3mn(): void {
    this.lanes.interactivityMountedEngineSurfaceLifetimeSelfTests(this.root);
    const failures = interactivityMountedEngineSurfaceLifetimeFailures(
      policyReadModuleTreeSource(this.root, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_WINIT_HOST_FILE),
      policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"),
      policyReadModuleTreeSource(this.root, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
      policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs"),
      ["🧱️core", "🎞️session"].map((node) => policyReadModuleTreeSource(this.root, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/${node}/🦀️.rs`)).join("\n"),
      policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs"),
      policyReadFileSafe(this.root, "🧰️framework/🔨️modules/✍️editor/🦀️.rs"),
      ["🗃️model", "🕸️graph", "🧭️host"].map((node) => policyReadModuleTreeSource(this.root, `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/${node}/🦀️.rs`)).join("\n"),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p3mn] ${failures.join("; ")}`);
    console.log("[verify interactivity p3mn] live-source and hostile mutations clean.");
  }

  /** 🎯️ Permanent Phase-8 generated inventory, factory-registration, and no-bypass gate. */
  private async runToolJobCoverage(args: string[]): Promise<void> {
    if (args[0] === "mounted-dispatch") {
      if (args.length !== 1) throw new Error("Expected verify interactivity tool-jobs mounted-dispatch");
            console.log(`[verify interactivity tool-jobs mounted-dispatch] laws=${await this.lanes.toolJobLatestWinsSelfTests()} clean.`);
      return;
    }
    if (args.includes("--shared-action-fixture-only")) {
      console.log(JSON.stringify(toolJobSharedFrameworkActionFixtureRun(this.root)));
      return;
    }
    if (args.includes("--fixed-operation-rust-fixture-source-only")) {
      process.stdout.write(toolJobFixedOperationRustFixtureSource(this.root));
      return;
    }
    if (args.includes("--fixed-operation-fixture-only")) {
      const fixture = toolJobFixedOperationFixtureRun(this.root);
      const rustFixture = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs");
      if (rustFixture !== toolJobFixedOperationRustFixtureSource(this.root)) throw new Error("[verify interactivity tool-jobs fixed-operation-fixture] generated Rust fixture is stale.");
      const source = `${policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs")}\n${policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs")}`;
      const mutations = this.lanes.toolJobFixedOperationRegistrySelfTests(source);
      console.log(`[verify interactivity tool-jobs fixed-operation-fixture] laws=${fixture.results.length} mutations=${mutations} rustFixtureFresh=true.`);
      console.log(JSON.stringify(fixture));
      return;
    }
    if (args.includes("--p2c-only")) {
      const actor = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🎭️actor/🦀️.rs");
      const shard = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs");
      const executor = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs");
      const wgpu = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
      const actionBus = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs");
      const pluginApp = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const pluginHost = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs");
      const programBridge = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs");
      const mutations = args.includes("--self-test") ? this.lanes.toolJobLiveFixedReplaySelfTests(actor, shard, executor, wgpu, actionBus, pluginApp, pluginHost, programBridge) : 0;
      if (!toolJobLiveFixedReplayExact(actor, shard, executor, wgpu, actionBus, pluginApp, pluginHost, programBridge)) throw new Error("[verify interactivity tool-jobs p2c] live fixed replay contract failed.");
      console.log(`[verify interactivity tool-jobs p2c] live-source clean; hostile-mutations=${mutations}.`);
      return;
    }
    if (args.includes("--p2a1-only")) {
      const store = policyReadOsStoreEvidence(this.root);
      const mutations = args.includes("--self-test") ? this.lanes.toolJobArtifactEnvelopeRejectionTransferSelfTests(store) : 0;
      if (!toolJobArtifactEnvelopeRejectionTransferExact(store)) throw new Error("[verify interactivity tool-jobs p2a1] Store rejection-transfer contract failed.");
      console.log(`[verify interactivity tool-jobs p2a1] live-source clean; hostile-mutations=${mutations}.`);
      return;
    }
    if (args.includes("--p6i-only")) {
      await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", "fem-live-visual-publication", ...args]);
      return;
    }
    if (args.includes("--p6h-only")) {
      await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", "fem-numerical-microcursor", ...args]);
      return;
    }
    if (args.includes("--self-test")) {
      const jobRuntime = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs");
      const artifactRetainedCommand = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs");
      const fixture = toolJobFixedOperationFixtureRun(this.root);
      const rustFixture = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs");
      if (rustFixture !== toolJobFixedOperationRustFixtureSource(this.root)) throw new Error("[verify interactivity tool-jobs] generated fixed-operation Rust fixture is stale.");
      const plugin = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const activation = toolJobFactoryProofActivationScan(this.root);
      if (activation.failures.length > 0) throw new Error(`[verify interactivity tool-jobs] app activation factory proof scan: ${JSON.stringify(activation)}`);
      console.log(`[verify interactivity tool-jobs] exact-factory-proof-owners=${activation.owners} custom-rows=${activation.customRows} generic-rows=${activation.genericRows} clean.`);
      console.log(`[verify interactivity tool-jobs] self-tests=${(await this.lanes.toolJobCoverageSelfTests()) + this.lanes.toolJobFixedOperationRegistrySelfTests(jobRuntime) + this.lanes.toolJobDrawingGestureOperationOwnerSelfTests() + this.lanes.toolJobArtifactRetainedCommandSelfTests(artifactRetainedCommand, plugin) + fixture.results.length + 1} clean.`);
      return;
    }
    const formatIndex = args.indexOf("--format");
    const format = formatIndex >= 0 ? args[formatIndex + 1] : "text";
    if (format !== "text" && format !== "json") throw new Error(`[verify interactivity tool-jobs] unsupported format ${JSON.stringify(format)}.`);
    const report = await toolJobCoverageRun(this.root, this.lanes);
    const outputIndex = args.indexOf("--output");
    const output = outputIndex >= 0 ? args[outputIndex + 1] : undefined;
    if (output) {
      const path = resolve(this.root, output);
      const relativePath = relative(this.root, path);
      if (relativePath.startsWith("..") || relativePath === "") throw new Error("[verify interactivity tool-jobs] --output must name a file inside the repository.");
      writeFileSync(path, `${JSON.stringify(report, null, 2)}\n`, "utf8");
    }
    if (format === "json" && !output) console.log(JSON.stringify(report, null, 2));
    else if (output) console.log(`[verify interactivity tool-jobs] wrote ${output}`);
    else console.log(`[verify interactivity tool-jobs] production-hosts=${report.macroHostFiles} production-invocations=${report.macroInvocations} production-rows=${report.commandRows} literal-registrations=${report.literalRegistrations} fixture-hosts=${report.fixtureMacroHostFiles} fixture-invocations=${report.fixtureMacroInvocations} fixture-rows=${report.fixtureCommandRows} admitted=${report.boundedRows} batch-only=${report.batchOnlyRows} forbidden=${report.forbiddenRows} deleted=${report.deletedRows} remaining=${report.remainingCommands.length} unique=${report.uniqueCommandRows} factories=${report.productionFactories} registrations=${report.productionRegistrations} dispatches=${report.productionDispatches} aliases=${report.aliases} self-tests=${report.selfTests}`);
    if (report.failures.length > 0) throw new Error(`[verify interactivity tool-jobs] ${report.failures.join("; ")}`);
  }

  /**
   * 🔒️ `verify dependencies` keeps the baseline ratchet and prints the all-ecosystem literal truth;
   * `summary`, `list`, `self-test`, and the red-until-zero `literal-external` mode are source-only.
   */
  private runDependencyFreeze(args: string[]): void {
    runDependencyVerification(this.root, args, { jsLockParity: this.lanes.dependencyJsLockParitySelfTests, truth: this.lanes.dependencyTruthSelfTests });
  }

  /** 🧱️ Enforces dependency direction through the repository library's taxonomy-owned graph rule. */
  private runLayering(segments: string[]): void {
    if (segments.length) throw new Error("Expected verify layering");
    runCmd("bun", ["repo", "run-many", "-p", "@semio-tech/repo-lib", "-t", "lint-dependency-direction", "lint-rust-source-direction", "lint-cargo-dependency-direction", "--skip-cache", ...semioBuildParallelFlag()], { cwd: this.root, ...orchestratorBudgetOpts() });
  }

  /** 🏛️ Executes architecture contract checks contributed by their owning build projects. */
  private runCanonicalArchitecture(): void {
    console.log("[verify canonical-architecture] checking owner-contributed dependency, state and I/O contracts…");
    runCmd("bun", ["repo", "run-many", "-t", "canonical-architecture", "--all", "--exclude", "workspace", "--skip-cache", ...semioBuildParallelFlag()], {
      cwd: this.root,
      ...orchestratorBudgetOpts(canonicalArchitectureEnvironment(this.root, process.env)),
    });
  }

  private async runGate(): Promise<void> {
    this.runCanonicalArchitecture();
    // Deliberately calls dependency-cruiser directly rather than `LintScript`/`bun repo run-many --targets=lint --all`:
    // several unrelated projects have pre-existing broken eslint configs,
    // and framework-renderer-wgpu:lint has known pending color-literal violations (see spawn_task follow-ups) —
    // this gate must stay a meaningful, currently-green signal for refactor sessions, not inherit that noise.
    console.log("[verify] dependency-cruiser boundaries…");
    runCmd(process.execPath, ["x", "dependency-cruiser", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "--config", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs", "--output-type", "err"], { cwd: this.root });
    console.log("[verify] generated catalog freshness…");
    runCmd("bun", ["repo", "run", "@semio-tech/plugin-registry:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] region/host-contract script lints…");
    runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-react:lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/framework-os-dev:plugin", "lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/repo-lib:lint-styling-pixels"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] framework owned-schema binding freshness…");
    runCmd("bun", ["repo", "run", "@semio-tech/framework-rs:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] ui locale/terminology axes freshness…");
    runCmd("bun", ["repo", "run", "@semio-tech/ui-rs:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] chrome i18n literal scan…");
    runCmd("bun", ["repo", "run", "@semio-tech/repo-lib:check-chrome-i18n"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] TypeScript/JavaScript dependency direction…");
    this.runLayering([]);
    console.log("[verify] owner-root test taxonomy and feature contract…");
    {
      const domain = String(repoTaxonomy(this.root).testDomainPath ?? "");
      if (domain !== "") runCmd("bun", [join(this.root, domain, "📜️script.ts"), "contract"], { cwd: join(this.root, domain), ...orchestratorBudgetOpts() });
    }
    console.log("[verify] storybook scope freshness…");
    await checkStorybookFreshness(this.root);
    console.log("[verify] indexed generated output (wasm/jco/wasm-pack)…");
    {
      const generatedBreaches = policyIndexedGeneratedOutputBreaches(this.root);
      if (generatedBreaches.length > 0) {
        for (const b of generatedBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${generatedBreaches.length} indexed generated-output breach(es)`);
      }
    }
    console.log("[verify] OS exclusive state authority policies…");
    {
      const osBreaches = [
        ...policyOsStateAuthorityBreaches(this.root),
        ...policyDocumentAppShapeBreaches(this.root),
      ];
      if (osBreaches.length > 0) {
        for (const b of osBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${osBreaches.length} OS exclusive state authority policy breach(es)`);
      }
    }
    console.log("[verify] standards/subsets vocabulary…");
    {
      const subsetVocabBreaches = [
        ...policyStandardsCoverageBreaches(this.root),
        ...policyStandardSubsetVocabularyBreaches(this.root),
      ].filter((b) => b.priority === "high");
      if (subsetVocabBreaches.length > 0) {
        for (const b of subsetVocabBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${subsetVocabBreaches.length} standards/subsets vocabulary policy breach(es)`);
      }
    }
    console.log("[verify] handcrafted grammar P3/M4 policies…");
    {
      const handcraftedBreaches = policyHandcraftedSpecP3Breaches(this.root);
      if (handcraftedBreaches.length > 0) {
        for (const b of handcraftedBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${handcraftedBreaches.length} handcrafted-grammar P3/M4 policy breach(es)`);
      }
    }
    console.log("[verify] artifact-schema facet policies…");
    {
      const artifactSchemaBreaches = policyArtifactSchemaBreaches(this.root);
      if (artifactSchemaBreaches.length > 0) {
        for (const b of artifactSchemaBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${artifactSchemaBreaches.length} artifact-schema policy breach(es)`);
      }
    }
    console.log("[verify] app-schema facet policies…");
    {
      const appSchemaBreaches = policyAppSchemaBreaches(this.root);
      if (appSchemaBreaches.length > 0) {
        for (const b of appSchemaBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${appSchemaBreaches.length} app-schema policy breach(es)`);
      }
    }
    console.log("[verify] package language purity…");
    {
      const packagePurityBreaches = policyPackageLanguagePurityBreaches(this.root).filter((b) => b.priority === "high");
      if (packagePurityBreaches.length > 0) {
        for (const b of packagePurityBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${packagePurityBreaches.length} package language purity breach(es)`);
      }
    }
    console.log("[verify] dissolve-core / plugin-root policies…");
    {
      const dissolveBreaches = [
        ...policyBannedNameStemBreaches(this.root),
        ...policyEmojiPrefixBreaches(this.root),
        ...policyPluginRootShapeBreaches(this.root),
        ...policyPluginBuilderBreaches(this.root, policyDiscoverCrateDirs(this.root)),
        ...policyApaBreaches(this.root),
        ...policyInferenceFamilyBreaches(this.root),
      ].filter((b) => b.priority === "high");
      if (dissolveBreaches.length > 0) {
        for (const b of dissolveBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${dissolveBreaches.length} dissolve-core / plugin-root policy breach(es)`);
      }
    }
    console.log("[verify] window capability taxonomy…");
    {
      const crateDirs = policyDiscoverCrateDirs(this.root);
      const windowBreaches = [...policyWindowCompletenessBreaches(this.root, crateDirs), ...policyModeCompletenessBreaches(this.root, crateDirs)];
      if (windowBreaches.length > 0) {
        for (const breach of windowBreaches) console.error(`[verify] ${breach.kind}: ${breach.summary}`);
        throw new Error(`[verify] ${windowBreaches.length} window/mode capability taxonomy breach(es)`);
      }
    }
    console.log("[verify] mutation-outcome / merge-policy law…");
    {
      const mutationOutcomeBreaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
      if (mutationOutcomeBreaches.length > 0) {
        for (const b of mutationOutcomeBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${mutationOutcomeBreaches.length} mutation-outcome/merge-policy law breach(es)`);
      }
    }
    console.log("[verify] history closure…");
    {
      if (this.lanes.historyClosurePolicySelfTests() === 0) throw new Error("[verify] history-closure self-test executed no cases");
      const historyClosureBreaches = policyHistoryClosureBreaches(this.root);
      if (historyClosureBreaches.length > 0) {
        for (const breach of historyClosureBreaches) console.error(`[verify] ${breach.kind}: ${breach.summary}`);
        throw new Error(`[verify] ${historyClosureBreaches.length} history-closure breach(es)`);
      }
    }
    console.log("[verify] dsl fixture laws…");
    // Quick level here: the full repo-wide sweep (parse→print→reparse fixpoint, canonicalize
    // idempotence over every real 📚️examples fixture — @semio-tech/s-fixture-sweep-rs) runs at
    // `test dsl`/`test dsl exhaustive`; the gate only needs the engine crates' own quick-level unit tests.
    runCmd("bun", ["repo", "run-many", "-t", "test", "-p", "@semio-tech/dsl-rs", "@semio-tech/dsl-schema-rs", "@semio-tech/dsl-derive-rs", ...semioBuildParallelFlag(), "--", "quick"], {
      cwd: this.root,
      ...orchestratorBudgetOpts(),
    });
    // 🌐️ Cross-target compile coverage — the ONLY thing in this repo that compiles
    // `#[cfg(target_arch = "wasm32")]` code. Native `cargo check`/`cargo test` skip those blocks
    // entirely, so a signature change can leave the wasm bindings broken behind a fully green
    // native build AND a green test suite; that is exactly how `🎭️actor`'s `🦀️.rs` sat with an
    // E0308 until MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME's Z1 ran clippy against the real triple.
    // Scoped to the actor kernel because it is small, fast, purity-critical, and the one crate whose
    // wasm glue every renderer depends on — the fleet-wide wasip2/native sweeps stay opt-in via
    // `verify rust-warnings --target <triple>`, which is far too slow for a pre-close gate.
    console.log("[verify] actor kernel wasm32 bindings…");
    await this.runRustWarnings(["--target", "wasm32-unknown-unknown"]);
    console.log("[verify] gate passed.");
  }
}
//#endregion 🔖️VerifyScript
