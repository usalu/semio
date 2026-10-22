#!/usr/bin/env repo
/** 📦️ EPW Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runDefinitionChecks } from "../../🧪️tests/📜️definition/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-stdio-epw",{ packages: ["semio-s-artifact-stdio-epw", "semio-s-artifact-stdio-epw-editor", "semio-s-artifact-stdio-epw-io-registry", "semio-s-artifact-stdio-epw-mutations", "semio-s-artifact-stdio-epw-schema", "semio-s-artifact-stdio-epw-snapshot", "semio-s-artifact-stdio-epw-tests"],commands: router,twins:[{name:"epw-definition",run:runDefinitionChecks}],snapshotSqliteTests:["../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});