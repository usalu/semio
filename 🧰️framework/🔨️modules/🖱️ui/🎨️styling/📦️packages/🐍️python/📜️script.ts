#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { StylingPythonGenerateScript, StylingPythonTestScript } from "../../🏗️builder/🟦️.ts";

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("generate", StylingPythonGenerateScript).register("test", StylingPythonTestScript)).run(process.argv.slice(2), original));
