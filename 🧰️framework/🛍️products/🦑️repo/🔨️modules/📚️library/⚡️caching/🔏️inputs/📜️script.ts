#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { RegistryCatalogInputsScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";


const router = new ScriptRouter(import.meta.dir).register("registry-catalog", RegistryCatalogInputsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));