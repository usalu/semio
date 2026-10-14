#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Executes original scheduling authority and all portable native receipts before the independent oracle. */
class TestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Mounted native corpus has no implicit selections");
  for(const name of ["SEMIO_MOUNTED_OWNER_RESULTS","SEMIO_MOUNTED_OWNER_NESTED_RESULTS","CARGO_TARGET_DIR"]){const path=process.env[name];if(!path||!path.startsWith(this.repoRoot)||path.length>256||[...path].length>256)throw Error(`${name} requires an explicit bounded repository-owned path`);}
  await runRepositoryCommand(this.invocation,"cargo",["test","--offline","--manifest-path",resolve(this.root,"Cargo.toml"),"--lib","--","--nocapture"],this.root,"mounted-native-turn",{version:1,maximumElapsedMilliseconds:300000,output:{maximumBytes:134217728,maximumLines:65536}});
  await runRepositoryCommand(this.invocation,process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts"),"--test-name-pattern=actual native verdicts"],this.repoRoot,"mounted-native-oracle",{version:1,maximumElapsedMilliseconds:60000,output:{maximumBytes:134217728,maximumLines:65536}});
 }
}
/** 🔮️ Preserves the complete portable corpus and strict independent schema/output oracle. */
class PortableScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Mounted portable corpus has no implicit selections");
  await runRepositoryCommand(this.invocation,process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts"),"--test-name-pattern=portable mounted ownership|all independent physical|runtime policy"],this.repoRoot,"mounted-portable-turn",{version:1,maximumElapsedMilliseconds:60000,output:{maximumBytes:134217728,maximumLines:65536}});
 }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("portable",PortableScript), { invocation: original }));
