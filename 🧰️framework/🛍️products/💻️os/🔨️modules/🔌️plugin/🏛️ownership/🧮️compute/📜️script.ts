#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🧮️ Runs this owner's compute source laws independently of native preparation. */
import {runComputeOwnershipTest} from "../../../../../../🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🟦️.ts";
import {BundleScript,ScriptRouter} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class Test extends BundleScript{async run(segments:string[]):Promise<void>{await runComputeOwnershipTest({root:this.root,repoRoot:this.repoRoot},"./🧪️tests/🟦️.ts",segments);}}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test",Test), { invocation: original }));
