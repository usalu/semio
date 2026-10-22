#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes original scheduling authority and all portable native receipts before the independent oracle. */
class TestScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
/** 🔮️ Preserves the complete portable corpus and strict independent schema/output oracle. */
class PortableScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw new Error("Mounted portable corpus has no implicit selections");
  await runRepositoryCommand(this.invocation,process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts"),"--test-name-pattern=portable mounted ownership|all independent physical|runtime policy"],this.repoRoot,"mounted-portable-turn",{version:1,maximumElapsedMilliseconds:60000,output:{maximumBytes:134217728,maximumLines:65536}});
 }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test",TestScript).register("portable",PortableScript), { invocation: original }));
