#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🔺️ Runs the native mesh IO and semantic value laws through the canonical test owner. */
import {resolve} from "node:path";
import {BundleScript,ScriptRouter} from "../../../🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runCargoTestsV1,readCargoTestPolicyV1} from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {resolveTestLevel} from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
class Native extends BundleScript {
 async run(segments:string[]):Promise<void>{const{rest}=resolveTestLevel(segments);await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-mesh-engine"],cwd:this.root,extraArgs:rest},readCargoTestPolicyV1(process.env));}
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test-native",Native), { invocation: original }));
