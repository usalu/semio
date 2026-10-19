#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { OwnershipScript, NativeScript, CustodyNativeScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


const router = new ScriptRouter(import.meta.dir).register("test-ownership", OwnershipScript).register("test-native", NativeScript).register("test-custody-native",CustodyNativeScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test-ownership" }));