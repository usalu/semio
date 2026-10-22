#!/usr/bin/env repo
/** 📦️ block-3d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
class NativeSqliteBaselineScript extends BundleScript {
  async run(): Promise<void> { await runArtifactRustTests("semio-s-artifact-block-3d", this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"], this.invocation); }
}
const router = new ScriptRouter(import.meta.dir).register("test-snapshot-sqlite-native", NativeSqliteBaselineScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-block-3d", {packages:["semio-s-artifact-block-3d-diff","semio-s-artifact-block-3d-editor","semio-s-artifact-block-3d-examples","semio-s-artifact-block-3d-inferences","semio-s-artifact-block-3d-io","semio-s-artifact-block-3d-model","semio-s-artifact-block-3d-mut-meta","semio-s-artifact-block-3d-mut-object-kind","semio-s-artifact-block-3d-mut-representations","semio-s-artifact-block-3d-mut-vortex-kinds","semio-s-artifact-block-3d-mut-vortices","semio-s-artifact-block-3d-mutations","semio-s-artifact-block-3d-schema","semio-s-artifact-block-3d-viewer","semio-s-artifact-block-3d"],snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],commands: router});
