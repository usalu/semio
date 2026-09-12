#!/usr/bin/env bun
import { ScriptRouter, runBundleScriptMain } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { DescriptorBuildScript, DescriptorTestScript } from "../../🏗️component-build/🟦️.ts";
import { DescribeScript } from "../../🛂️descriptor-emission/🟦️.ts";

if (import.meta.main) await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("build", DescriptorBuildScript).register("test", DescriptorTestScript).register("describe", DescribeScript), import.meta.url);
