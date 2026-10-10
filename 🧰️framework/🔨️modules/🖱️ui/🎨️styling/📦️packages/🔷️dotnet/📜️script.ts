#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { StylingDotnetBuildScript, StylingDotnetDepsScript } from "../../🏗️builder/🔷️dotnet/📜️script.ts";

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("deps", StylingDotnetDepsScript).register("build", StylingDotnetBuildScript)).run(process.argv.slice(2), original));
