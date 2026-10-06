#!/usr/bin/env bun
/** 📦️ block-3d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
class NativeSqliteBaselineScript extends BundleScript {
  async run(): Promise<void> { await runArtifactRustTests("semio-s-artifact-block-3d", this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"], ["component-app-assembly"]); }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-block-3d", {snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],commands:{"test-snapshot-sqlite-native":NativeSqliteBaselineScript}});
