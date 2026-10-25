#!/usr/bin/env repo
import { runRepoScriptMain } from "../../../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BuildScript, DevScript, StartScript, TestScript } from "../../🏗️builder/🛤️task-routes/🟦️.ts";

/** 🧭️ Coordinator Next.js package router: `repo ./📜️script.ts build|dev|start|test|policy`; the routes live in `🏗️builder/🛤️task-routes`. */
const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("dev", DevScript).register("start", StartScript).register("test", TestScript);
await runRepoScriptMain(router, import.meta.url, { defaultCommand: "build", policyModule: "../../🏗️builder/🛤️task-routes/🟦️.ts" });
