#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { RegistryCatalogInputsScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";


const router = new ScriptRouter(import.meta.dirname).register("registry-catalog", RegistryCatalogInputsScript);
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));