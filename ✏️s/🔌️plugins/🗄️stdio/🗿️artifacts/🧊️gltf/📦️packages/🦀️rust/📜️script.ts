#!/usr/bin/env bun
/** 📦️ gltf Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runDefinitionChecks } from "../../🧪️tests/📜️definition/🟦️.ts";
import { runBorrowedCarrierChecks } from "../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/📸️snapshot/📦️pack/🧪️tests/🫳️borrowed-carriers/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {repositoryCargoPreparationStorageV1, prepareCargoWorkspaceInvocation } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { devToolingEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { acquireCargoBuildLeaseV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { lstatSync, mkdirSync, realpathSync } from "node:fs";

/** 🧬️ Verifies the actual native codec graph under a caller-owned compiler cache. */
class NativeSchemaScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifacts || !isAbsolute(artifacts) || segments.length > 1) throw new Error("Native GLTF schema proof requires an absolute caller artifact directory and at most one copied workspace");
    mkdirSync(artifacts, { recursive: true });
    const workspace = segments[0] ? realpathSync(segments[0]) : this.repoRoot;
    if (segments[0]) {
      const local = relative(realpathSync(artifacts), workspace);
      if (!local || isAbsolute(local) || local === ".." || local.startsWith(".." + sep) || workspace === this.repoRoot || lstatSync(segments[0]).isSymbolicLink()) throw new Error("Native GLTF copied workspace must be contained in caller artifacts");
    }
    const manifest = resolve(workspace, relative(this.repoRoot, this.root), "Cargo.toml");
    const target = segments[0] ? join(workspace, "target") : join(artifacts, "gltf-native-target");
    const env = devToolingEnv({ NX_WORKSPACE_ROOT: workspace, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: join(target, "intermediate") });
    const args = ["test", "--offline", "--manifest-path", manifest, "-p", "semio-s-artifact-stdio-gltf", "--lib"];
    prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(workspace),workspace,args,workspace,process.env);
    const controller = new AbortController(), abort = () => controller.abort(), budget = buildBudgetMs() || 1_200_000, timer = setTimeout(abort, budget);
    process.once("SIGINT", abort); process.once("SIGTERM", abort);
    let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
    try {
      lease = await acquireCargoBuildLeaseV1({ directory: repoCacheDirectory(workspace, "agents", "resource-leases"), buildDirectory: target, args, signal: controller.signal });
      for (const law of ["definition_tests::native_codec_identity_matches_structural_graph_and_independent_blake3", "standards::v2_0::subsets::any::io::component::binary::snapshot::owned_pack::borrowed_carrier_tests::borrowed_gltf_carriers_match_the_neutral_owned_metadata_and_serde_oracle", "standards::v2_0::subsets::any::io::component::text::inferences::tests::canonical_json_bytes_matches_the_portable_byte_array_contract", "standards::v2_0::subsets::any::io::component::sqlite::snapshot::tests::sqlite_snapshot_gltf_empty_image_and_texture_entities_survive_logical_native_lists", "standards::v2_0::subsets::any::io::component::sqlite::snapshot::tests::sqlite_snapshot_gltf_controlled_binding_retires_deep_completed_fields_after_failure_and_cancellation", "standards::v2_0::subsets::any::io::component::sqlite::snapshot::tests::sqlite_snapshot_gltf_independent_bad_later_json_owner_retires_deep_completed_sibling"]) {
        let observed = false;
        await runOwnedCommand("cargo", [...args, law, "--", "--exact", "--nocapture"], workspace, "gltf-native-schema", budget, { env, signal: controller.signal, onLine: line => { if (line.includes(`test ${law} ... ok`)) observed = true; } });
        if (!observed) throw new Error(`Native GLTF law did not execute: ${law}`);
      }
    } finally {
      clearTimeout(timer); process.off("SIGINT", abort); process.off("SIGTERM", abort); if (lease) await lease.release();
    }
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-gltf", { commands: { "native-schema-check": NativeSchemaScript }, twins: [{ name: "gltf-definition", run: runDefinitionChecks }, { name: "gltf-borrowed-carriers", run: runBorrowedCarrierChecks }], snapshotSqliteTests: ["../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📜️schema/🟦️.ts", "../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
