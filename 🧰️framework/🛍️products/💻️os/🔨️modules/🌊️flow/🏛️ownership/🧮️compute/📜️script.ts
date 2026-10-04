#!/usr/bin/env bun
/** 🌊️ Runs Flow-owned compute source witnesses independently of Cargo preparation. */
import {ComputeOwnershipTestScript} from "../../../../../../🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🟦️.ts";
import {ScriptRouter} from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test));
