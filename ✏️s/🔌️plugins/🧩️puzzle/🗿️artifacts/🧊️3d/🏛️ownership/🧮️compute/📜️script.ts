#!/usr/bin/env bun
/** 🧮️ Runs this owner's compute source laws independently of native preparation. */
import {ComputeOwnershipTestScript} from "../../../../../../../🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🟦️.ts";
import {ScriptRouter} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test));
