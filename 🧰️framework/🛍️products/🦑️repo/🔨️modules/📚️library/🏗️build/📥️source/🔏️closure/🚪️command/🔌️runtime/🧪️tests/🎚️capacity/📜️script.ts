import*as fs from"node:fs";
import{join}from"node:path";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{withRepositoryRuntimeEntriesV1}from"../../🟦️.ts";
import{openNativeFixedPlatformV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts";
import{acquireNativeStorageV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{
 const phase=process.argv[3]!,base=process.argv[2]!;if(!["honest","outgrown","full","freed"].includes(phase))throw Error("Closed fixed capacity phase required");
 const control=createIssuedResourceControlV1(original,"test:fixed-capacity-"+phase,60000);
 await withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{
  const directory=join(base,"p"),leaseDirectory=join(base,"leases");fs.mkdirSync(leaseDirectory,{recursive:true});
  const image=await openNativeFixedPlatformV1({directory,leaseDirectory,filesystemBytes:67108864,maximumPhysicalBytes:68157440,control});
  try{
   const policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:68157440,maximumWorkspaceBytes:67108864,maximumArtifactBytes:67108864},acquire=(bytes:number)=>acquireNativeStorageV1({policy,control,port:image.port,owner:"capacity-test",kind:"compiler-workspace",bytes}),journal=()=>fs.readFileSync(join(image.root,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean).length;
   if(phase==="honest"||phase==="outgrown"){const reservation=await acquire(1048576);fs.writeFileSync(join(image.root,phase==="honest"?"honest":"dishonest"),Buffer.alloc((phase==="honest"?1:80)*1048576,phase==="honest"?1:2));await reservation.commit();}
   else if(phase==="full"){fs.writeFileSync(join(base,"journal-before"),String(journal()));await acquire(1048576);}
   else{const before=Number(fs.readFileSync(join(base,"journal-before"),"utf8"));if(journal()!==before)throw Error("Refused capacity phase changed the journal");await acquire(1048576);if(journal()!==before+1)throw Error("Freed capacity phase did not journal exactly one reservation");}
   console.log(JSON.stringify({version:1,phase,completed:true}));
  }finally{await image.close().catch(()=>undefined);}
 });
});
