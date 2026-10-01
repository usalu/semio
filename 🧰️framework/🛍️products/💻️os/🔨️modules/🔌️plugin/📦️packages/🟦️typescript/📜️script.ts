#!/usr/bin/env bun
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { MaterializeScript, SupportScript } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("support", SupportScript).register("materialize", MaterializeScript);
if (import.meta.main) await runScriptMain(router);
