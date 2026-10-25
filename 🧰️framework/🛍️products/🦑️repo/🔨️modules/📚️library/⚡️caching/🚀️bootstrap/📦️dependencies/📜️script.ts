#!/usr/bin/env repo
import { ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {SyncScript,RefreshLockScript} from "./⌨️commands/🟦️.ts";
import {ContractCheckScript} from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";


if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("sync", SyncScript).register("lock", RefreshLockScript).register("contract-check", ContractCheckScript)).run(process.argv.slice(2), original));