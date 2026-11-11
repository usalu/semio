#!/usr/bin/env repo
/** 📦️ ifc Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-ifc", {
  snapshotSqliteTestBudgetMs: 120000,
  snapshotSqliteTests: [
    "../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
    "../../🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
  ],
});
