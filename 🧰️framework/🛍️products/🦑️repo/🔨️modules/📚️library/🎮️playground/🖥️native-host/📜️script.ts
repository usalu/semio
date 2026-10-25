#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { nativeHostContractLaws } from "./🧪️tests/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
class ContractScript extends BundleScript {async run():Promise<void> {await nativeHostContractLaws(this.repoRoot);}}
if(isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("contract-check",ContractScript), { invocation: original, ...({defaultCommand:"contract-check"}) }));
