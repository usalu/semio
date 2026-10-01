#!/usr/bin/env bun
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { nativeHostContractLaws } from "./🧪️tests/🟦️.ts";
class ContractScript extends BundleScript {async run():Promise<void> {nativeHostContractLaws(this.repoRoot);}}
if(import.meta.main) await runScriptMain(new ScriptRouter(import.meta.dir).register("contract-check",ContractScript),{defaultCommand:"contract-check"});
