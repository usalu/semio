#!/usr/bin/env bun
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { ScriptRouter, runBundleScriptMain } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { MaterializeScript, SupportScript } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("support", SupportScript).register("materialize", MaterializeScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
