import{test,expect}from"bun:test";
import*as fs from"node:fs";
import{join}from"node:path";
import type{ScriptInvocation}from"../../../../../../🧭️routing/🟦️.ts";
import{nativeDirectoryProtocolV1}from"../../../📁️directory/🟦️.ts";

const protocol=nativeDirectoryProtocolV1(),phase=process.env.SEMIO_CAPACITY_PHASE,shared=process.env.SEMIO_CAPACITY_BASE;
const session=async<T>(name:string,body:(input:{root:string;acquire:(bytes:number)=>Promise<unknown>;journal:()=>number})=>Promise<T>)=>{
 const{findWorkspaceRoot}=await import("../../../../../../🧭️routing/🟦️.ts"),root=findWorkspaceRoot(import.meta.dir),{withBuildTestInvocationV1}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts")),{scriptCompilerAdmissionControlV1}=await import("../../../../../../🏗️build/🎟️capacity/🟦️.ts"),{openNativeFixedPlatformV1}=await import("../../🟦️.ts"),storage=await import("../../../🟦️.ts"),directory=join(shared!,"p"),leaseDirectory=join(shared!,"leases");fs.mkdirSync(leaseDirectory,{recursive:true});
 return withBuildTestInvocationV1(async(original:ScriptInvocation)=>{
  const control=scriptCompilerAdmissionControlV1(original,"test:fixed-capacity-"+name,0),image=await openNativeFixedPlatformV1({directory,leaseDirectory,filesystemBytes:67108864,maximumPhysicalBytes:68157440,control});
  try{const policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:68157440,maximumWorkspaceBytes:67108864,maximumArtifactBytes:67108864};
   return await body({root:image.root,acquire:bytes=>storage.acquireNativeStorageV1({policy,control,port:image.port,owner:"capacity-test",kind:"compiler-workspace",bytes}),journal:()=>fs.readFileSync(join(image.root,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean).length});
  }finally{await image.close().catch(()=>undefined);}
 });
};
test.skipIf(phase!=="honest")("phase honest",()=>session("honest",async({root,acquire})=>{const reservation=await acquire(1048576)as{commit():Promise<void>};fs.writeFileSync(join(root,"honest"),Buffer.alloc(1048576,1));await reservation.commit();}));
test.skipIf(phase!=="outgrown")("phase outgrown",()=>session("outgrown",async({root,acquire})=>{const reservation=await acquire(1048576)as{commit():Promise<void>};fs.writeFileSync(join(root,"dishonest"),Buffer.alloc(80*1048576,2));await reservation.commit();}));
test.skipIf(phase!=="full")("phase full",()=>session("full",async({acquire,journal})=>{fs.writeFileSync(join(shared!,"journal-before"),String(journal()));await acquire(1048576);}));
test.skipIf(phase!=="freed")("phase freed",()=>session("freed",async({acquire,journal})=>{const before=Number(fs.readFileSync(join(shared!,"journal-before"),"utf8"));expect(journal()).toBe(before);await acquire(1048576);expect(journal()).toBe(before+1);}));

/** 🎚️ Runs the real directory backend end to end across reopened sessions (one process each, because a refusal closes its Script invocation): a writer inside its reservation commits, a writer that outgrows it is refused at commit, and while its bytes remain on disk no new reservation is journaled. */
test.skipIf(process.platform==="darwin"||phase!==undefined)("the opened directory partition refuses a write that outgrows its reservation and keeps the journal unchanged until space is back",()=>{
 if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Explicit capacity test artifact directory required");
 const base=fs.mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"cap-")),run=(name:string)=>Bun.spawnSync([process.execPath,"test","--timeout","60000",import.meta.path,"-t","phase "+name],{env:{...process.env,SEMIO_CAPACITY_PHASE:name,SEMIO_CAPACITY_BASE:base},stdout:"pipe",stderr:"pipe"}),text=(result:ReturnType<typeof run>)=>result.stdout.toString()+result.stderr.toString();
 try{
  const honest=run("honest");expect(honest.exitCode,text(honest)).toBe(0);expect(fs.readdirSync(join(base,"p")).sort()).toEqual([protocol.anchor,"image.json",protocol.root,protocol.volume].sort());
  const outgrown=run("outgrown");expect(outgrown.exitCode).not.toBe(0);expect(text(outgrown)).toContain(protocol.refusals.capacity);
  const full=run("full");expect(full.exitCode).not.toBe(0);expect(text(full)).toContain(protocol.refusals.capacity);
  fs.rmSync(join(base,"p",protocol.root,"dishonest"));
  const freed=run("freed");expect(freed.exitCode,text(freed)).toBe(0);
 }finally{fs.rmSync(base,{recursive:true,force:true});}
},240000);
