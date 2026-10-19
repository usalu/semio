#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TestScript, TestEnduserScript, TestReleaseScript, TestReleaseRoutingScript } from "./🟦️.ts";


const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-enduser",TestEnduserScript).register("test-release", TestReleaseScript).register("test-release-routing", TestReleaseRoutingScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));