#!/usr/bin/env repo
/** 📦️ wav Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runWavTypedChunkChecks } from "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🎼️typed-chunks/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-wav", { packages: ["semio-s-artifact-stdio-wav", "semio-s-artifact-stdio-wav-codec-tests", "semio-s-artifact-stdio-wav-inferences", "semio-s-artifact-stdio-wav-io-registry", "semio-s-artifact-stdio-wav-mutations", "semio-s-artifact-stdio-wav-schema"], twins: [{ name: "wav-typed-chunks", run: runWavTypedChunkChecks }], snapshotSqliteTests: ["../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
