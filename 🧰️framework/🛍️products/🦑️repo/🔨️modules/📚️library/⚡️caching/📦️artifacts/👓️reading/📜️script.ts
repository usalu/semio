#!/usr/bin/env repo
import { ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TestScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";

const router=new ScriptRouter(import.meta.dirname).register("test",TestScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"test"}));