#!/usr/bin/env repo
/** 🧭️ repo-schema-commands command router: each command runs against the workspace root. */
import { ScriptRouter, findWorkspaceRoot, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SchemaScript } from "./🟦️.ts";

const root = findWorkspaceRoot(import.meta.dirname);
const router = new ScriptRouter(root, root).register("schema", SchemaScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
