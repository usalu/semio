#!/usr/bin/env bun
import { ScriptRouter, runBundleScriptMain } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { StylingPythonBuildScript, StylingPythonDepsScript, StylingPythonGenerateScript, StylingPythonTestScript } from "../../🏗️builder/🟦️.ts";

await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("deps", StylingPythonDepsScript).register("generate", StylingPythonGenerateScript).register("build", StylingPythonBuildScript).register("test", StylingPythonTestScript), import.meta.url);
