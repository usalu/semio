#!/usr/bin/env bun
import { ScriptRouter, runBundleScriptMain } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { StylingDotnetBuildScript, StylingDotnetDepsScript } from "../../🏗️builder/🟦️.ts";

await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("deps", StylingDotnetDepsScript).register("build", StylingDotnetBuildScript), import.meta.url);
