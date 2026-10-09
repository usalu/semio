#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Executes original scheduling authority and all portable native receipts before the independent oracle. */
class TestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Mounted native corpus has no implicit selections");
  for(const name of ["SEMIO_MOUNTED_OWNER_RESULTS","SEMIO_MOUNTED_OWNER_NESTED_RESULTS","CARGO_TARGET_DIR"]){const path=process.env[name];if(!path||!path.startsWith(this.repoRoot)||path.length>256||[...path].length>256)throw Error(`${name} requires an explicit bounded repository-owned path`);}
  await runRepositoryCommand("cargo",["test","--offline","--manifest-path",resolve(this.root,"Cargo.toml"),"--lib","--","--nocapture"],this.root,"mounted-native-turn",300000);
  await runRepositoryCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts"),"--test-name-pattern=actual native verdicts"],this.repoRoot,"mounted-native-oracle",60000);
 }
}
/** 🔮️ Preserves the complete portable corpus and strict independent schema/output oracle. */
class PortableScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Mounted portable corpus has no implicit selections");
  await runRepositoryCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts"),"--test-name-pattern=portable mounted ownership|all independent physical|runtime policy"],this.repoRoot,"mounted-portable-turn",60000);
 }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("portable",PortableScript));
