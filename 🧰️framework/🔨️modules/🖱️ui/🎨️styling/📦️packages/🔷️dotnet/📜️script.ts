#!/usr/bin/env bun
import { ScriptRouter } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { StylingDotnetBuildScript, StylingDotnetDepsScript } from "../../🏗️builder/🔷️dotnet/📜️script.ts";

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("deps", StylingDotnetDepsScript).register("build", StylingDotnetBuildScript).run(process.argv.slice(2));
