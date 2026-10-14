#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🌊️ Runs Flow-owned compute source witnesses independently of Cargo preparation. */
import {ComputeOwnershipTestScript} from "../../../../../../🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🟦️.ts";
import {ScriptRouter} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class Test extends ComputeOwnershipTestScript{readonly source="./🧪️tests/🟦️.ts";}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test), { invocation: original }));
