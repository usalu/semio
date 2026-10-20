#!/usr/bin/env bun
import { ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SyncScript, RefreshLockScript, ContractCheckScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";


if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("sync", SyncScript).register("lock", RefreshLockScript).register("contract-check", ContractCheckScript)).run(process.argv.slice(2), original));