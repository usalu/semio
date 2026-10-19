#!/usr/bin/env bun
/** 🗂️ Framework Space Collection Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir,"semio-framework-artifact-space-collection",{commands: router,snapshotSqliteTests:["../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});