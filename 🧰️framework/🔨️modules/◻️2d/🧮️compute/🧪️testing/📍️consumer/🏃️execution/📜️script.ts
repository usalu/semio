#!/usr/bin/env bun
/** 🎚️ Runs the complete compute ownership runner level law. */
import {ComputeOwnershipTestScript} from "./🟦️.ts";
import {ScriptRouter} from "../../../../../🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test));
