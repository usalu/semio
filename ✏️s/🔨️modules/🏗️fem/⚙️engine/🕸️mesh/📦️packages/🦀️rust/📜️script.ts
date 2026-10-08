#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
/** 🦀️ Checks the independent neutral native package with explicit owning manifest and finite process authority. */
class CheckScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Neutral mesh check accepts no implicit arguments");
  await runRepositoryCommand("cargo",["check","--offline","--manifest-path",resolve(this.root,"Cargo.toml")],this.root,"fem-mesh-check",300000);
 }
}
/** 🧪️ Runs the complete original neutral mesh laws under finite owning execution. */
class TestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Neutral mesh laws accept no implicit selection");
  await runRepositoryCommand("cargo",["test","--offline","--manifest-path",resolve(this.root,"Cargo.toml"),"--lib","--","--nocapture"],this.root,"fem-mesh-test",300000);
 }
}
/** 📐️ Runs the complete portable neutral surface corpus without installing foreign artifact providers. */
class SurfaceTestScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Neutral surface corpus accepts no implicit selection");
  if(!process.env.SEMIO_FEM_NEUTRAL_RESULTS)throw Error("Neutral surface corpus requires an explicit output authority");
  await runRepositoryCommand("cargo",["test","--offline","--manifest-path",resolve(this.root,"Cargo.toml"),"--lib","surface_tests::complete_portable_region_surface_corpus","--","--exact","--nocapture"],this.root,"fem-mesh-surface-test",300000);
 }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("check",CheckScript).register("test",TestScript).register("test-surface",SurfaceTestScript));
