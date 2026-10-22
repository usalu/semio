#!/usr/bin/env repo
/** 🔋️ Publishes the complete typed Energy model and its semantic SQLite laws. */
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runRepositoryCommand} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
/** 🪟️ Runs only the neutral optical semantic intent oracle through the registered owner. */
class OpticalFieldIntentScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-optical-field-intent accepts no arguments");await runRepositoryCommand(this.invocation,process.execPath,["test","./✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🪟️optics/🟦️.ts"],this.repoRoot,"energy-optical-field-intent",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:134217728,maximumLines:65536}});}}
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts";
const router = new ScriptRouter(import.meta.dir).register("test-optical-field-intent", OpticalFieldIntentScript);
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/energy-model",{commands: router,suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🪟️optics/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🪪️owned-target/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪪️capability/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
