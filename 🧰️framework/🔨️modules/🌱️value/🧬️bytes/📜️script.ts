#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TestScript } from "./🧪️tests/🧬️base64/🟦️.ts";
const router=new ScriptRouter(import.meta.dir).register("test",TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand:"test"}) }));
