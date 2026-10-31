import assert from"node:assert/strict";
import{mkdtempSync,mkdirSync,rmSync}from"node:fs";
import{join}from"node:path";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{runOwnedCommand,OwnedCommandFailure}from"../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import{captureRuntimeEntryV1,capturedRuntimeEntryCommandV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts";
import{withRepositoryRuntimeEntriesV1}from"../../🟦️.ts";
import{nativeDirectoryRuntimeEntryV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📜️script.ts";
import{nativeDirectoryProtocolV1,probeNativeDirectoryCaseSensitivityV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
if(import.meta.main){await receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"test:captured-directory-actions",0);return withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Original test artifact directory required");const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR,"dir-")),protocol=nativeDirectoryProtocolV1();
  try{const directory=join(base,"cli");mkdirSync(join(directory,"m"),{recursive:true});const entry=await captureRuntimeEntryV1(nativeDirectoryRuntimeEntryV1(),control),command=capturedRuntimeEntryCommandV1(entry,control),run=async(...args:string[])=>{const output:{stdout:Buffer[];stderr:Buffer[]}={stdout:[],stderr:[]};let status:number|null=0,signal:string|null=null;try{await runOwnedCommand(command.program,[...command.arguments,...args],process.cwd(),"test:directory-action:"+args[0],0,{control,signal:control.signal,input:command.input,env:scriptResourceProcessEnvironmentV1(control,{...process.env,...command.environment}),output:{maximumBytes:1048576,maximumLines:4096},outputStyle:"static",outputConsumer:(bytes,stream)=>{output[stream].push(Buffer.from(bytes));}});}catch(error){if(!(error instanceof OwnedCommandFailure))throw error;status=error.outcome.status;signal=error.outcome.signal;}return{status,signal,stdout:Buffer.concat(output.stdout).toString("utf8"),stderr:Buffer.concat(output.stderr).toString("utf8")};};
   const allocate=await run("allocate",directory,"33554432","34603008");assert.equal(allocate.signal,null,allocate.stderr);
   if(!probeNativeDirectoryCaseSensitivityV1(base)){assert.equal(allocate.status,1);assert.ok(allocate.stderr.includes(protocol.refusals.caseInsensitive));return;}
   assert.equal(allocate.status,0,allocate.stderr);assert.equal((await run("inactive",directory)).stdout,"");assert.equal((await run("activate",directory)).status,0);assert.equal((await run("inactive",directory)).stdout,"attached\n");
   const again=await run("activate",directory);assert.equal(again.status,1);assert.ok(again.stderr.includes("Inactive partition required"));assert.equal((await run("deactivate",directory)).status,0);assert.equal((await run("inactive",directory)).stdout,"");assert.equal((await run("nonsense",directory)).status,1);
  }finally{rmSync(base,{recursive:true,force:true});}
 });
});console.log(JSON.stringify({version:1,completed:true}));}
