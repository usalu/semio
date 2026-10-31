import{test,expect}from"bun:test";
import{mkdtemp,writeFile,rm}from"node:fs/promises";
import{join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts";
import{captureNxAuthorizerSourcesV1}from"../../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{snapshotNxAuthorizerSourcesV1,compileNxAuthorizerSnapshotV1}from"../../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-native-storage",maximumElapsedMilliseconds:180000,scratch:"checkout"});
test("Native partial terminal continuation uses genuine EX and refuses replacement without losing its intent",()=>withBuildTestInvocationV1(async original=>{
 const root=process.cwd(),control=createIssuedResourceControlV1(original,"test:whole-retirement-integration",120000),entry=relative(root,fileURLToPath(new URL("./📜️script.ts",import.meta.url))).replaceAll("\\","/"),captured=await captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true}),compiled=await compileNxAuthorizerSnapshotV1(root,snapshotNxAuthorizerSourcesV1(captured),control,"runtime","program"),area=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"wh-")),program=join(area,"whole.mjs");
 try{await writeFile(program,compiled.bytes,{flag:"wx",mode:0o444});const result=await captureOwnedCommandResultV1(process.execPath,[program],root,{control,env:scriptResourceProcessEnvironmentV1(control,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}),output:{maximumBytes:1048576,maximumLines:8192}});expect(result.signal,result.stderr).toBeNull();expect(result.status,result.stdout+result.stderr).toBe(0);expect(JSON.parse(result.stdout.trim().split("\n").at(-1)!)).toEqual({version:1,completed:true});}finally{await rm(area,{recursive:true,force:true});}
}),120000);
