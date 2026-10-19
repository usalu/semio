#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🧮️ Runs this owner's compute source laws independently of native preparation. */
import {runComputeOwnershipTest} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🟦️.ts";
import {BundleScript,ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
class Test extends BundleScript{async run(segments:string[]):Promise<void>{await runComputeOwnershipTest({root:this.root,repoRoot:this.repoRoot},"./🧪️tests/🟦️.ts",segments);}}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",Test), { invocation: original }));
