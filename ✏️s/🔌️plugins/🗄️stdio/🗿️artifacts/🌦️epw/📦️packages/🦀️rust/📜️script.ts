#!/usr/bin/env bun
/** 📦️ epw Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runDefinitionChecks } from "../../🧪️tests/📜️definition/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-epw", { twins: [{ name: "epw-definition", run: runDefinitionChecks }],snapshotSqliteTests:["../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"] });
