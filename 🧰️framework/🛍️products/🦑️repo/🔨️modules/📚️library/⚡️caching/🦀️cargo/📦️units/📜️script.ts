#!/usr/bin/env repo
import { ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { RustUnitScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

const router=new ScriptRouter(import.meta.dirname).register("unit",RustUnitScript);if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));