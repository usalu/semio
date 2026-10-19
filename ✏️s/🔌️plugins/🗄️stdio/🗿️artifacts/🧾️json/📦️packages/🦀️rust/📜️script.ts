#!/usr/bin/env bun
/** 📦️ Original JSON artifact owning package commands. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { OriginalReceivingSource, OriginalReceivingNative } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧬️ Current owning original schema proof without source copies or fresh authority. */
class NativeSchemaScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("native JSON schema accepts no arguments");await runArtifactRustTests("semio-s-artifact-stdio-json",this.repoRoot,["--lib","sqlite_snapshot_json_owned_record_identity_matches_handwritten_definition","--no-fail-fast"],this.invocation);}
}
const router = new ScriptRouter(import.meta.dir).register("native-schema-check", NativeSchemaScript).register("test-original-receiving-source", OriginalReceivingSource).register("test-original-receiving-native", OriginalReceivingNative);
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-stdio-json",{ packages: ["semio-s-artifact-stdio-json", "semio-s-artifact-stdio-json-mutations", "semio-s-artifact-stdio-json-mutations-2", "semio-s-artifact-stdio-json-number", "semio-s-artifact-stdio-json-schema", "semio-s-artifact-stdio-json-schema-2", "semio-s-artifact-stdio-json-tests"],commands: router,snapshotSqliteTests:["../../🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts", "../../🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts","../../🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});