#!/usr/bin/env repo
/** ⚖️ repo-policy command router: the workspace-wide verification gates run against the workspace root. */
import { ScriptRouter, findWorkspaceRoot, receiveScriptProcessInvocation, runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { RootVerifyScript } from "../🎛️commands/✅️verify/🌳️root/🟦️.ts";

const root = findWorkspaceRoot(import.meta.dirname);
const router = new ScriptRouter(root, root).register("verify", RootVerifyScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
