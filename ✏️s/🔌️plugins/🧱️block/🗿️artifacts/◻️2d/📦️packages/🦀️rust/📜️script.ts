#!/usr/bin/env bun
/** 📦️ block-2d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
class NativeSqliteBaselineScript extends BundleScript {
  async run(): Promise<void> { await runArtifactRustTests("semio-s-artifact-block-2d", this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"], this.invocation); }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-block-2d", {packages:["semio-s-artifact-block-2d","semio-s-artifact-block-2d-diff","semio-s-artifact-block-2d-editor","semio-s-artifact-block-2d-editor-tests","semio-s-artifact-block-2d-examples","semio-s-artifact-block-2d-inferences","semio-s-artifact-block-2d-io","semio-s-artifact-block-2d-model","semio-s-artifact-block-2d-mut-handles","semio-s-artifact-block-2d-mut-meta","semio-s-artifact-block-2d-mut-node-kind","semio-s-artifact-block-2d-mutations","semio-s-artifact-block-2d-schema","semio-s-artifact-block-2d-viewer"],snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],commands:{"test-snapshot-sqlite-native":NativeSqliteBaselineScript}});
