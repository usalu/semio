#!/usr/bin/env bun
/** 📦️ tiff Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-tiff", {
  testFeatures: ["component-app-assembly"],
  snapshotSqliteTests: ["../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts", "../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/💰️operation/🟦️.ts", "../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/💰️frontiers/🟦️.ts"],
});
