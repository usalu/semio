#!/usr/bin/env repo
/** 🧭️ repo-storybook command router: each command runs against the workspace root. */
import { ScriptRouter, findWorkspaceRoot, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { DevScript } from "../../🎛️commands/🛠️dev/🟦️.ts";
import { RootTestScript } from "../../📦️packages/🟦️typescript/🎛️commands/🧪️test/🚦️root/🟦️.ts";
import { SetupScript } from "../../🎛️commands/🔧️setup/🟦️.ts";

const root = findWorkspaceRoot(import.meta.dirname);
const router = new ScriptRouter(root, root).register("dev", DevScript).register("test", RootTestScript).register("setup", SetupScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
