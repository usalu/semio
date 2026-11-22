#!/usr/bin/env repo
import {resolve} from "node:path";
import {BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {verifyProducedWasmPackageV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import {buildRepositoryWasmWebV1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts";
const family=resolve(import.meta.dirname,"../📦️packages/🦀️rust");
/** 🏗️ Produces the s-cad-geometry-browser wasm bindings from authored Rust under its received Original. */
class WasmScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Wasm producer accepts no arguments");
  await buildRepositoryWasmWebV1({producer:"browser-module",invocation:this.invocation,consume:verifyProducedWasmPackageV1,control:scriptCompilerAdmissionControlV1(this.invocation,"wasm:build",0),rsDir:family,componentDir:family,logPrefix:"s/dev/cad",wasmBaseName:"cad_geometry",pkg:{name:"@semio-tech/s-cad-geometry-browser",files:["cad_geometry.js","cad_geometry_bg.wasm","cad_geometry.d.ts"],main:"cad_geometry.js",module:"cad_geometry.js",types:"cad_geometry.d.ts"}});
 }
}
const router=new ScriptRouter(import.meta.dirname).register("wasm",WasmScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"wasm"}));
