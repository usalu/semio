#!/usr/bin/env bun
/** 📦️ deflate Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-stdio-deflate",{ packages: ["semio-s-artifact-stdio-deflate", "semio-s-artifact-stdio-deflate-examples", "semio-s-artifact-stdio-deflate-mutations", "semio-s-artifact-stdio-deflate-schema", "semio-s-artifact-stdio-deflate-snapshot", "semio-s-artifact-stdio-deflate-tests", "semio-s-artifact-stdio-deflate-zlib"],commands: router,snapshotSqliteTests:["../../🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});