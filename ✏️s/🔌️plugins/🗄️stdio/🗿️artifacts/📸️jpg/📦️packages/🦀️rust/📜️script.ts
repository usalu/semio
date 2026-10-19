#!/usr/bin/env bun
/** 📦️ jpg Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-jpg", { packages: ["semio-s-artifact-stdio-jpg", "semio-s-artifact-stdio-jpg-inferences", "semio-s-artifact-stdio-jpg-io-registry", "semio-s-artifact-stdio-jpg-mutations", "semio-s-artifact-stdio-jpg-operations", "semio-s-artifact-stdio-jpg-schema", "semio-s-artifact-stdio-jpg-schema-2"],commands: router,snapshotSqliteTests: ["../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts", "../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });