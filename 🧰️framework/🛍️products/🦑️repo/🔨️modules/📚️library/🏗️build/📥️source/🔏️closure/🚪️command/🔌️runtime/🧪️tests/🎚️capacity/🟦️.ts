import{test,expect}from"bun:test";
import*as fs from"node:fs";
import{join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts";
import{nativeDirectoryProtocolV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import{captureNxAuthorizerSourcesV1}from"../../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{snapshotNxAuthorizerSourcesV1,compileNxAuthorizerSnapshotV1}from"../../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-native-storage",maximumElapsedMilliseconds:300000,scratch:"checkout"});
/** 🎚️ Reopens the real directory partition through independent captured receivers for every capacity phase. */
test.skipIf(process.platform==="darwin")("the opened directory partition refuses a write that outgrows its reservation and keeps the journal unchanged until space is back",()=>withBuildTestInvocationV1(async original=>{
 if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Explicit capacity test artifact directory required");
 const root=process.cwd(),control=createIssuedResourceControlV1(original,"test:fixed-capacity-integration",240000),entry=relative(root,fileURLToPath(new URL("./📜️script.ts",import.meta.url))).replaceAll("\\","/"),captured=await captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true}),compiled=await compileNxAuthorizerSnapshotV1(root,snapshotNxAuthorizerSourcesV1(captured),control,"runtime","program"),base=fs.mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR,"cap-")),program=join(base,"capacity.mjs"),protocol=nativeDirectoryProtocolV1();
 fs.writeFileSync(program,compiled.bytes,{flag:"wx",mode:0o444});
 const run=(phase:string)=>captureOwnedCommandResultV1(process.execPath,[program,base,phase],root,{control,env:scriptResourceProcessEnvironmentV1(control,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}),output:{maximumBytes:1048576,maximumLines:4096}}),text=(result:{stdout:string;stderr:string})=>result.stdout+result.stderr;
 try{
  const honest=await run("honest");expect(honest.signal,text(honest)).toBeNull();expect(honest.status,text(honest)).toBe(0);expect(JSON.parse(honest.stdout.trim().split("\n").at(-1)!)).toEqual({version:1,phase:"honest",completed:true});expect(fs.readdirSync(join(base,"p")).sort()).toEqual([protocol.anchor,"image.json",protocol.root,protocol.volume].sort());
  const outgrown=await run("outgrown");expect(outgrown.signal,text(outgrown)).toBeNull();expect(outgrown.status).not.toBe(0);expect(text(outgrown)).toContain(protocol.refusals.capacity);
  const full=await run("full");expect(full.signal,text(full)).toBeNull();expect(full.status).not.toBe(0);expect(text(full)).toContain(protocol.refusals.capacity);
  fs.rmSync(join(base,"p",protocol.root,"dishonest"));
  const freed=await run("freed");expect(freed.signal,text(freed)).toBeNull();expect(freed.status,text(freed)).toBe(0);expect(JSON.parse(freed.stdout.trim().split("\n").at(-1)!)).toEqual({version:1,phase:"freed",completed:true});
 }finally{fs.rmSync(base,{recursive:true,force:true});}
}),240000);
