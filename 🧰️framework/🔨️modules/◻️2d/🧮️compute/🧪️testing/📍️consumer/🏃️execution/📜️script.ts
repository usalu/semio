#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🎚️ Runs the complete compute ownership runner level law. */
import {ComputeOwnershipTestScript} from "./🟦️.ts";
import {ScriptRouter} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test), { invocation: original }));
