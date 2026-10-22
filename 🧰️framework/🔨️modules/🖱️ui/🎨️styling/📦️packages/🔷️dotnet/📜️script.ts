#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { StylingDotnetBuildScript, StylingDotnetDepsScript } from "../../🏗️builder/🔷️dotnet/📜️script.ts";
import { isMainModuleV1 } from "../../../../⚙️runtime/🏁️main/🟦️.ts";

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("deps", StylingDotnetDepsScript).register("build", StylingDotnetBuildScript)).run(process.argv.slice(2), original));
