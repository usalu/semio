#!/usr/bin/env bun
import {ComputeOwnershipTestScript} from "../../../../🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🟦️.ts";
import {ScriptRouter} from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
/** 🏛️ Runs the declared OS compute ownership laws. */
class Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test));
