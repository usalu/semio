#!/usr/bin/env bun
/** 📦️ wav Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runWavTypedChunkChecks } from "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🎼️typed-chunks/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-wav", { twins: [{ name: "wav-typed-chunks", run: runWavTypedChunkChecks }], snapshotSqliteTests: ["../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
