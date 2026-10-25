#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BuildScript, CompileScript, DaemonScript, InstallScript, PreferencesScript, RunScript, TestScript } from "../../🛤️task-routes/🟦️.ts";

/** 🛤️ `@semio-tech/repo-dashboard-rs` task router: `repo ./📜️script.ts <build|compile|install|preferences|test|run|daemon>`; the routes live in `🛤️task-routes`. */
const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("compile",CompileScript).register("install", InstallScript).register("preferences", PreferencesScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
