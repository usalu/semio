#!/usr/bin/env repo
/** 📦️ obj Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-obj", { packages: ["semio-s-artifact-stdio-obj", "semio-s-artifact-stdio-obj-diff", "semio-s-artifact-stdio-obj-inferences", "semio-s-artifact-stdio-obj-io-registry", "semio-s-artifact-stdio-obj-mutations", "semio-s-artifact-stdio-obj-schema"], commands: router,snapshotSqliteTests: ["../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });