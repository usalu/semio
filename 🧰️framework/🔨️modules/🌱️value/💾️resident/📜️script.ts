#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TestScript } from "./🧪️tests/🧪️resident-oracle/🟦️.ts";

const router = new ScriptRouter(import.meta.dirname).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
