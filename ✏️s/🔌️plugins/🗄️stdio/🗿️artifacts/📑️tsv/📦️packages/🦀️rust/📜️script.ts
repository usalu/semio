#!/usr/bin/env bun
/** 📦️ tsv Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-tsv", { packages: ["semio-s-artifact-stdio-tsv", "semio-s-artifact-stdio-tsv-inferences", "semio-s-artifact-stdio-tsv-io-registry", "semio-s-artifact-stdio-tsv-mutations", "semio-s-artifact-stdio-tsv-schema", "semio-s-artifact-stdio-tsv-snapshot"], snapshotSqliteTests: ["../../🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
