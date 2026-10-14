#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { nativeHostContractLaws } from "./🧪️tests/🟦️.ts";
class ContractScript extends BundleScript {async run():Promise<void> {await nativeHostContractLaws(this.repoRoot);}}
if(import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("contract-check",ContractScript), { invocation: original, ...({defaultCommand:"contract-check"}) }));
