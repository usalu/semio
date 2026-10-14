#!/usr/bin/env bun
import {RustUnitScript} from "../../📜️script.ts";
import {ScriptRouter,receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const router=new ScriptRouter(import.meta.dir).register("unit",RustUnitScript);
await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
