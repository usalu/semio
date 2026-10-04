#!/usr/bin/env bun
/** 📦️ block-2d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
class NativeSqliteBaselineScript extends BundleScript {
  async run(): Promise<void> { await runArtifactRustTests("semio-s-artifact-block-2d", this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"]); }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-block-2d", {snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"],commands:{"test-snapshot-sqlite-native":NativeSqliteBaselineScript}});
