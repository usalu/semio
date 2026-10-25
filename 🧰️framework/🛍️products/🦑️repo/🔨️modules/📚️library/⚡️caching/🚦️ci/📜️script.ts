#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BaselineScript } from "./🟦️.ts";
import { isMainModuleV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";


if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("baseline", BaselineScript)).run(process.argv.slice(2), original));