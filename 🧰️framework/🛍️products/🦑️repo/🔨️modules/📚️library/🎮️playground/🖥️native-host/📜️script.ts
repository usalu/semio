#!/usr/bin/env bun
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { nativeHostContractLaws } from "./🧪️tests/🟦️.ts";
class ContractScript extends BundleScript {async run():Promise<void> {nativeHostContractLaws(this.repoRoot);}}
if(import.meta.main) await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("contract-check",ContractScript),import.meta.url,{defaultCommand:"contract-check"});
