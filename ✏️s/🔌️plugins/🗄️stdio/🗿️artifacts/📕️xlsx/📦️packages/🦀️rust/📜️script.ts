#!/usr/bin/env repo
/** 📦️ xlsx Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";

const packageName = "semio-s-artifact-stdio-xlsx";
await runArtifactRustPackageMain(import.meta.dir, packageName, {
  testPackages: [
    packageName,
    "semio-s-artifact-stdio-xlsx-base",
    "semio-s-artifact-stdio-xlsx-strict",
    "semio-s-artifact-stdio-xlsx-strict-mutations",
    "semio-s-artifact-stdio-xlsx-transitional",
  ],
  snapshotSqliteTests: ["../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],
});
