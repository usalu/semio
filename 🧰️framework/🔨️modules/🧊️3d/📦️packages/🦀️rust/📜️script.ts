#!/usr/bin/env bun
import { runExactCargoLaws } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, runCargoLintV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
/** 🦀️ `semio-framework-3d` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-3d"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("canonical-architecture accepts no arguments");
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-3d": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory,
      cwd: this.repoRoot,
      groups: [{ package: "semio-framework-3d", target: { kind: "lib" }, laws: ["retained_modeling_jobs_slice_work_and_match_synchronous_geometry", "retained_modeling_jobs_execute_geometry_inside_each_slice", "retained_generated_attributes_interpolate_source_domains_and_cancel_finalization", "mirror_remaps_owned_channels_and_reflects_corner_capable_normals", "orientation_remaps_corner_and_directed_edge_attributes_without_copying_samples", "indexed_corner_attributes_tessellate_with_seams_and_transform_once", "retained_face_expansion_preserves_source_channels_and_cancels_remapping", "retained_hole_caps_remap_boundary_channels_and_interpolate_source_faces", "retained_face_deletion_preserves_indexed_domains_and_owned_assets", "retained_decimation_preserves_owned_channels_through_each_accepted_collapse", "retained_bevel_clipping_interpolates_owned_channels_before_each_plane", "retained_coplanar_merging_preserves_owned_domains_and_declared_face_interpolation", "retained_welding_preserves_authored_seams_and_vertex_contributors"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: buildBudgetMs(),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`canonical-mesh-modeling ${event.stage}: ${event.law ?? ""}`); },
    });
    console.log(`canonical-mesh-modeling receipts=${receipts.length}`);
  }
}

/** 📈️ Runs Cargo's benchmark mode for `semio-framework-3d`; this package declares no standalone benchmark target.
 * The dedicated Brep Criterion benchmark belongs to `semio-s-plugin-stdio` and its own `bench` command. */
class BenchScript extends BundleScript {
  async run(): Promise<void> {
    await runOwnedCommand("cargo", ["bench", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-3d"], this.repoRoot, "tool:owner", buildBudgetMs(), {env: process.env});
  }
}


/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-3d --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargoLintV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-3d"],cwd:this.root,extraArgs:segments},readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalArchitectureScript).register("test", TestScript).register("bench", BenchScript).register("lint", LintScript);

await runScriptMain(router, { defaultCommand: "test" });
