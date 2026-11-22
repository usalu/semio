import{test,expect}from"bun:test";
import{execFileSync}from"node:child_process";
import{existsSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{createHash}from"node:crypto";
import{createIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{observeNativeStorageStateV1}from"@semio-tech/process/artifacts/native-build/storage";
import{repositoryCacheBudgetV1}from"../../../../📦️budget/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import*as owner from"../../🟦️.ts";
import{nxGraphNativeFundingDigestV1}from"../../🪙️funding/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();
/** 🏦️ Proves a namespace native image is a parent-funded child that passes parent rechecks and retires together with its charge. */
test("a namespace native image is funded by the nx partition journal and retires with its charge",()=>repository.invoke(async original=>{
 const root=repository.root,control=createIssuedResourceControlV1(original,"nx:funded-namespace",0),value={version:1 as const,toolingIdentity:"f".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"e".repeat(64)},native=schema.definitions.NativePolicy.const,nx=repositoryCacheBudgetV1(root).partitions.nx;
 await owner.prepareNxGraphCacheStorageV1(root,original,control,0);
 const shard=(root:string)=>owner.nxGraphNativeImageDirectoryV1(root,value),children=async(store:owner.NxGraphCacheStoreV1)=>(await observeNativeStorageStateV1({policy:{version:1 as const,root:store.root,leaseDirectory:join(root,".cache/b/l/m/data"),capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes},control,port:store.image.port})).allocations.filter(row=>row.child?.directory===shard(store.root));
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  expect(await children(store)).toHaveLength(0);
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{expect(lease.nativeDirectory).toBe(join(shard(store.root),"m"));await lease.recheck();await store.recheck();
   const rows=await children(store);expect(rows).toHaveLength(1);expect(rows[0]!.kind).toBe("compiler-state");expect(rows[0]!.owner).toBe(native.fundingOwner);expect(rows[0]!.phase).toBe("committed");expect(rows[0]!.bytes).toBe(native.maximumPhysicalBytes+native.fundingMetadataBytes);
  }finally{await lease.release();}
 });
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{await lease.recheck();expect(await children(store)).toHaveLength(1);}finally{await lease.release();}
  let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}expect(retired).toBe(true);
  expect(await children(store)).toHaveLength(0);expect(existsSync(shard(store.root))).toBe(false);expect(existsSync(owner.nxGraphNamespaceDirectoryV1(store.root,value))).toBe(false);
 });
},{maximumElapsedMilliseconds:170000}),180000);

/** 🪦️ Proves a namespace whose birth died between its reservation and its commit keeps its charge and its image, across a device renumbering remount: namespace retirement protects every uncommitted child (the contract of `♻️retirement`), it never retires it. */
test("a namespace native image left reserved by an aborted birth is protected from namespace retirement and keeps its charge",()=>repository.invoke(async original=>{
 const root=repository.root,control=createIssuedResourceControlV1(original,"nx:funded-reserved-namespace",0),value={version:1 as const,toolingIdentity:"a".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"b".repeat(64)},nx=repositoryCacheBudgetV1(root).partitions.nx;
 await owner.prepareNxGraphCacheStorageV1(root,original,control,0);
 const shard=(root:string)=>owner.nxGraphNativeImageDirectoryV1(root,value),children=async(store:owner.NxGraphCacheStoreV1)=>(await observeNativeStorageStateV1({policy:{version:1 as const,root:store.root,leaseDirectory:join(root,".cache/b/l/m/data"),capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes},control,port:store.image.port})).allocations.filter(row=>row.child?.directory===shard(store.root));
 let journal="",id="";
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{await lease.recheck();const rows=await children(store);expect(rows).toHaveLength(1);id=rows[0]!.id;journal=join(store.root,"storage-events.jsonl");}finally{await lease.release();}
 });
 const text=readFileSync(journal,"utf8"),line=JSON.stringify({type:"committed",id})+"\n";expect(text.split(line)).toHaveLength(2);const rebound=(row:string)=>row.includes('"type":"namespace-created","id":"'+id+'"')?row.replace(/"dev":"(\d+)"/g,(_,dev)=>'"dev":"'+(Number(dev)+16)+'"'):row;writeFileSync(journal,text.replace(line,"").split("\n").map(rebound).join("\n"));
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  expect((await children(store))[0]!.phase).toBe("reserved");
  expect(await owner.retireNxGraphNamespaceV1(store,value)).toBe(false);
  const rows=await children(store);expect(rows).toHaveLength(1);expect(rows[0]!.phase).toBe("reserved");expect(existsSync(shard(store.root))).toBe(true);
 });
 writeFileSync(journal,readFileSync(journal,"utf8")+line);
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  expect((await children(store))[0]!.phase).toBe("committed");
  let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}expect(retired).toBe(true);
  expect(await children(store)).toHaveLength(0);
 });
},{maximumElapsedMilliseconds:170000}),180000);

/** 🧟️ Proves a reservation whose reserver died before any physical effect (no namespace receipt, no directory) is retired by an appended journal event when the same namespace is born again, never by editing the journal. */
test("a funded namespace birth retires the orphan reservation of a dead reserver through journal events",()=>repository.invoke(async original=>{
 const root=repository.root,control=createIssuedResourceControlV1(original,"nx:funded-orphan-namespace",0),value={version:1 as const,toolingIdentity:"1".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"2".repeat(64)},nx=repositoryCacheBudgetV1(root).partitions.nx,orphan="0123456789abcdef0123456789abcdef";
 await owner.prepareNxGraphCacheStorageV1(root,original,control,0);
 const shard=(root:string)=>owner.nxGraphNativeImageDirectoryV1(root,value),children=async(store:owner.NxGraphCacheStoreV1)=>(await observeNativeStorageStateV1({policy:{version:1 as const,root:store.root,leaseDirectory:join(root,".cache/b/l/m/data"),capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes},control,port:store.image.port})).allocations.filter(row=>row.child?.directory===shard(store.root));
 let journal="",row:Awaited<ReturnType<typeof children>>[number]|undefined;
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{await lease.recheck();row=(await children(store))[0];journal=join(store.root,"storage-events.jsonl");}finally{await lease.release();}
  let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}expect(retired).toBe(true);
  expect(await children(store)).toHaveLength(0);expect(existsSync(dirname(shard(store.root)))).toBe(false);
 });
 const reserved={type:"reserved",allocation:{id:orphan,owner:row!.owner,kind:row!.kind,bytes:row!.bytes,phase:"reserved",child:{...row!.child!,allocation:orphan}}},before=readFileSync(journal,"utf8");
 writeFileSync(journal,before+JSON.stringify(reserved)+"\n");
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  expect((await children(store)).map(item=>item.id)).toEqual([orphan]);
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{await lease.recheck();const rows=await children(store);expect(rows).toHaveLength(1);expect(rows[0]!.id).not.toBe(orphan);expect(rows[0]!.phase).toBe("committed");}finally{await lease.release();}
 });
 const after=readFileSync(journal,"utf8"),events=after.split("\n").filter(Boolean).map(line=>JSON.parse(line)as{type:string;id?:string});expect(after.startsWith(before)).toBe(true);expect(events.filter(event=>event.id===orphan).map(event=>event.type)).toEqual(["retiring","retired"]);
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}expect(retired).toBe(true);expect(await children(store)).toHaveLength(0);
 });
},{maximumElapsedMilliseconds:170000}),180000);

/** 🕳️ Proves a funded namespace whose physical child an interrupted retirement already removed, whole or down to its creator sidecar, settles its remaining charge instead of failing on the missing directory. */
for(const[title,letter,removal,retiring]of[["with its namespace","c",(shard:string)=>dirname(shard),false],["leaving only its creator sidecar","e",(shard:string)=>shard,false],["after its retirement began (protected, not retired)","f",(shard:string)=>dirname(shard),true]]as const)test("a funded namespace removed by an interrupted retirement settles its remaining charge, "+title,()=>repository.invoke(async original=>{
 const root=repository.root,control=createIssuedResourceControlV1(original,"nx:funded-removed-namespace",0),value={version:1 as const,toolingIdentity:letter.repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"d".repeat(64)},nx=repositoryCacheBudgetV1(root).partitions.nx;
 await owner.prepareNxGraphCacheStorageV1(root,original,control,0);
 const shard=(root:string)=>owner.nxGraphNativeImageDirectoryV1(root,value),children=async(store:owner.NxGraphCacheStoreV1)=>(await observeNativeStorageStateV1({policy:{version:1 as const,root:store.root,leaseDirectory:join(root,".cache/b/l/m/data"),capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes},control,port:store.image.port})).allocations.filter(row=>row.child?.directory===shard(store.root));
 let journal="",id="";
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{await lease.recheck();const rows=await children(store);expect(rows).toHaveLength(1);id=rows[0]!.id;journal=join(store.root,"storage-events.jsonl");}finally{await lease.release();}
 });
 if(retiring)writeFileSync(journal,readFileSync(journal,"utf8")+JSON.stringify({type:"retiring",id})+"\n");
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  expect((await children(store))[0]!.phase).toBe(retiring?"retiring":"committed");
  if(process.platform==="darwin")execFileSync("/usr/bin/hdiutil",["detach","-force",join(shard(store.root),"m")],{stdio:"ignore"});
  rmSync(removal(shard(store.root)),{recursive:true,force:true});
  expect(await children(store)).toHaveLength(1);
  if(retiring){expect(await owner.retireNxGraphNamespaceV1(store,value)).toBe(false);expect(await children(store)).toHaveLength(1);return;}
  let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}expect(retired).toBe(true);
  expect(await children(store)).toHaveLength(0);expect(existsSync(dirname(shard(store.root)))).toBe(false);
 });
},{maximumElapsedMilliseconds:170000}),180000);

/** 🧓️ Proves a committed shard born by the previous format (64 MiB funding digest and size) is retired and re-born when its namespace is opened by the current code, never refused. */
test("an old-format committed shard is retired and re-born by the current opening path",()=>repository.invoke(async original=>{
 const root=repository.root,control=createIssuedResourceControlV1(original,"nx:funded-old-format",0),value={version:1 as const,toolingIdentity:"7".repeat(64),workspaceRoot:root,graphIdentity:null,implementationIdentity:"6".repeat(64)},native=schema.definitions.NativePolicy.const,nx=repositoryCacheBudgetV1(root).partitions.nx;
 await owner.prepareNxGraphCacheStorageV1(root,original,control,0);
 const shard=(root:string)=>owner.nxGraphNativeImageDirectoryV1(root,value),children=async(store:owner.NxGraphCacheStoreV1)=>(await observeNativeStorageStateV1({policy:{version:1 as const,root:store.root,leaseDirectory:join(root,".cache/b/l/m/data"),capacityBytes:nx.capacityBytes,maximumWorkspaceBytes:nx.filesystemBytes,maximumArtifactBytes:nx.maximumArtifactBytes},control,port:store.image.port})).allocations.filter(row=>row.child?.directory===shard(store.root));
 const current={maximumPhysicalBytes:native.maximumPhysicalBytes,filesystemBytes:native.filesystemBytes},previous={maximumPhysicalBytes:67108864,filesystemBytes:66060288,charge:67239936,digest:createHash("sha256").update(JSON.stringify({version:1,owner:native.fundingOwner,filesystemBytes:66060288,maximumPhysicalBytes:67108864,metadataBytes:native.fundingMetadataBytes})).digest("hex")};
 let journal="",id="",partition="";

 Object.assign(native,{maximumPhysicalBytes:previous.maximumPhysicalBytes,filesystemBytes:previous.filesystemBytes});
 try{
  await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
   const lease=await owner.acquireNxGraphNamespaceV1(store,value);
   try{await lease.recheck();const rows=await children(store);expect(rows).toHaveLength(1);id=rows[0]!.id;partition=store.root;journal=join(store.root,"storage-events.jsonl");}finally{await lease.release();}
  });
 }finally{Object.assign(native,current);}
 const born=readFileSync(journal,"utf8"),oldDigest=previous.digest,newDigest=nxGraphNativeFundingDigestV1;
 writeFileSync(journal,born.split("\n").map(row=>row.includes('"'+id+'"')?row.replaceAll(newDigest,oldDigest).replaceAll('"bytes":'+(native.maximumPhysicalBytes+native.fundingMetadataBytes),'"bytes":'+previous.charge).replaceAll('"filesystemBytes":'+native.filesystemBytes,'"filesystemBytes":'+previous.filesystemBytes).replaceAll('"maximumPhysicalBytes":'+native.maximumPhysicalBytes,'"maximumPhysicalBytes":'+previous.maximumPhysicalBytes):row).join("\n"));
 const sidecar=readdirSync(dirname(shard(partition))).filter(name=>name.startsWith(".native-birth-"));expect(sidecar).toHaveLength(1);
 const claim=join(dirname(shard(partition)),sidecar[0]!);writeFileSync(claim,readFileSync(claim,"utf8").replaceAll(newDigest,oldDigest));
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  const before=(await children(store))[0]!;expect(before.child!.policySha256).toBe(oldDigest);expect(before.bytes).toBe(previous.charge);
  const lease=await owner.acquireNxGraphNamespaceV1(store,value);
  try{await lease.recheck();const rows=await children(store);expect(rows).toHaveLength(1);expect(rows[0]!.id).not.toBe(id);expect(rows[0]!.phase).toBe("committed");expect(rows[0]!.bytes).toBe(native.maximumPhysicalBytes+native.fundingMetadataBytes);expect(rows[0]!.child!.policySha256).toBe(newDigest);expect(rows[0]!.child!.maximumPhysicalBytes).toBe(native.maximumPhysicalBytes);}finally{await lease.release();}
 });
 await owner.withNxGraphCacheStoreV1(root,original,control,async store=>{
  let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}expect(retired).toBe(true);expect(await children(store)).toHaveLength(0);
 });
},{maximumElapsedMilliseconds:170000}),180000);

/** 🔓️ Proves a retirement that fails after taking the parent mapping exclusively releases it, so the next retirement of the process neither waits on its own lease nor leaves the pool wedged. */
test("a failed funded child retirement releases its parent mapping for the next retirement",async()=>{
 const value={version:1 as const,toolingIdentity:"9".repeat(64),workspaceRoot:repository.root,graphIdentity:null,implementationIdentity:"8".repeat(64)};
 let sidecar="";
 const retire=()=>repository.invoke(async original=>{
  const control=createIssuedResourceControlV1(original,"nx:funded-failed-retirement",0);
  await owner.prepareNxGraphCacheStorageV1(repository.root,original,control,0);
  return owner.withNxGraphCacheStoreV1(repository.root,original,control,async store=>{
   if(!sidecar){const lease=await owner.acquireNxGraphNamespaceV1(store,value);try{await lease.recheck();}finally{await lease.release();}sidecar=join(dirname(owner.nxGraphNativeImageDirectoryV1(store.root,value)),readdirSync(dirname(owner.nxGraphNativeImageDirectoryV1(store.root,value))).find(name=>name.startsWith(".native-birth-"))!);return false;}
   let retired=false;for(let attempt=0;attempt<40&&!retired;attempt++){retired=await owner.retireNxGraphNamespaceV1(store,value);if(!retired)await new Promise(done=>setTimeout(done,500));}return retired;
  });
 },{maximumElapsedMilliseconds:170000});
 await retire();
 const stray=join(dirname(sidecar),"native","stray");writeFileSync(stray,"x");
 await expect(retire()).rejects.toThrow();
 rmSync(stray);
 expect(await retire()).toBe(true);
},180000);

/** 🔓️ Proves every success-tail mapping writer site of the mounted and fixed child backends also releases on failure: a leaked parent mapping EX wedges every later mapping read of the process. */
test("every parent mapping writer site releases its mapping on failure too",()=>{
 const storage=join(import.meta.dir,"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage");
 for(const file of["🧹️cleanup/🎛️command/🟦️.ts","🧱️fixed/🟦️.ts"]){
  const source=readFileSync(join(storage,file),"utf8"),sites=source.split("await acquireNativeStorageMappingWriterV1(").length-1,guarded=source.split(/catch\(error\)\{(?:failed=true;failure=error;\}[^]{0,400}?|const held=)[^]{0,300}?releaseNativeStorageMappingWriterV1\(held\)/u).length-1;
  expect(sites).toBe(3);
  expect(guarded).toBeGreaterThanOrEqual(2);
 }
});
