#!/usr/bin/env bun
/** 📦️ md Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-md", { packages: ["semio-s-artifact-stdio-md", "semio-s-artifact-stdio-md-diff", "semio-s-artifact-stdio-md-mutations", "semio-s-artifact-stdio-md-schema", "semio-s-artifact-stdio-md-snapshot", "semio-s-artifact-stdio-md-tests"], snapshotSqliteTests: ["../../🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
