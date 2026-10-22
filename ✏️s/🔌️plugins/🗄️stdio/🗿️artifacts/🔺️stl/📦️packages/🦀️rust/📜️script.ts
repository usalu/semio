#!/usr/bin/env repo
/** 📦️ stl Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-stl", { packages: ["semio-s-artifact-stdio-stl", "semio-s-artifact-stdio-stl-examples", "semio-s-artifact-stdio-stl-inferences", "semio-s-artifact-stdio-stl-mutations", "semio-s-artifact-stdio-stl-schema", "semio-s-artifact-stdio-stl-viewer"], commands: router,snapshotSqliteTests: ["../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });