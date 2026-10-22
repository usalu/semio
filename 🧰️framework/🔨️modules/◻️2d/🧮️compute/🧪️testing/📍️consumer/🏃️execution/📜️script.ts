#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🎚️ Runs the complete compute ownership runner level law. */
import {runComputeOwnershipTest} from "./🟦️.ts";
import {BundleScript,ScriptRouter} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class Test extends BundleScript{async run(segments:string[]):Promise<void>{await runComputeOwnershipTest({root:this.root,repoRoot:this.repoRoot},"./🧪️tests/🟦️.ts",segments);}}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test",Test), { invocation: original }));
