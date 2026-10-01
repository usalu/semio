#!/usr/bin/env bun
import { ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { StylingPythonGenerateScript, StylingPythonTestScript } from "../../🏗️builder/🟦️.ts";

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("generate", StylingPythonGenerateScript).register("test", StylingPythonTestScript).run(process.argv.slice(2));
