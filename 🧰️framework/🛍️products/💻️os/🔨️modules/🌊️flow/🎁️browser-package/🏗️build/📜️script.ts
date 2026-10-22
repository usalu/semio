#!/usr/bin/env repo
import {resolve} from "node:path";
import {BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {buildAdmittedWasmWebV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import {publishFlowBrowserPackage} from "../../../../📦️packages/🟦️typescript/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts";
const family=resolve(import.meta.dirname,"../../📦️packages/🦀️rust");
/** 🏗️ Produces the browser package from authored Rust and publication sources under its received Original. */
class WasmScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Wasm producer accepts no arguments");
  await buildAdmittedWasmWebV1({producer:"browser-module",invocation:this.invocation,consume:capture => publishFlowBrowserPackage(capture, this.invocation),control:scriptCompilerAdmissionControlV1(this.invocation,"wasm:build",0),rsDir:family,componentDir:resolve(family,"../../🧩️component/📦️packages/🦀️rust"),logPrefix:"os/flow/browser-package",wasmBaseName:"flow_core",pkg:{name:"@semio-tech/flow-core",files:["flow_core_bg.wasm","flow_core.js","flow_core.d.ts","flow_core_bg.wasm.d.ts"],main:"flow_core.js",module:"flow_core.js",types:"flow_core.d.ts"}});
 }
}
const router=new ScriptRouter(import.meta.dirname).register("wasm",WasmScript);
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original,defaultCommand:"wasm"}));
