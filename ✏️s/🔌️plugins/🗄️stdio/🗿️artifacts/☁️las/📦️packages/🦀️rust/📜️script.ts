#!/usr/bin/env bun
/** 📦️ las Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-las", { packages: ["semio-s-artifact-stdio-las", "semio-s-artifact-stdio-las-mutations", "semio-s-artifact-stdio-las-schema", "semio-s-artifact-stdio-las-snapshot", "semio-s-artifact-stdio-las-tests", "semio-s-artifact-stdio-las-tests-2"],commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });