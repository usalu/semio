/** 🛟️ Contract: a graph writer under optional-cache pressure sheds the accelerator packs of finished owners first and keeps their records, so the next read takes every finished owner without inferring it. Real project records and sparse packs fill one inference dataset of a private repository beyond the low-water mark; a second writer of the same dataset retires down to it; every record is still taken by a third read, the oldest packs are gone.
 * @see ../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts */
import{expect,test}from"bun:test";
import{existsSync,readdirSync,utimesSync,writeFileSync}from"node:fs";
import{join,relative}from"node:path";
import{createHash}from"node:crypto";
import{createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import storageSchema from"../../../📥️inputs/💾️storage/🧬️schema/🔣️.json"with{type:"json"};
import{privateGraphWorkspaceV1}from"../🧰️support/🟦️.ts";

const policy=storageSchema.definitions.OptionalQuota.definitions.DefaultPolicy.const,units=10,values=6,valueBytes=10*1024*1024,sha=(text:string)=>createHash("sha256").update(text).digest("hex");
test("pressure sheds packs, never the record of a finished owner",async()=>{
 await((await import("../../../🕸️graph/📥️inference/🟨️.mjs")) as any).libraryBootstrap;
 const api=await import("../../../📥️inputs/💾️storage/🟨️.mjs") as any,observation=await import("../../../👁️observation/🟨️.mjs") as any,ws=await privateGraphWorkspaceV1(),root=ws.root,source=await ws.scratch("rf-"),identity=sha(source);
 const dataset={version:1,identity,sourceRoot:relative(root,source).replaceAll("\\","/")},cachePathOf=(directory:string,index:number)=>join(directory,"np",sha("owner-"+index).slice(0,24)+".json"),recordsOf=(directory:string,index:number)=>cachePathOf(directory,index)+".records";
 for(let index=0;index<units;index++)writeFileSync(join(source,"in"+index+".txt"),"input "+index);
 await Bun.sleep(2300);
 const read=(body:(directory:string,infer:(index:number)=>Promise<number>)=>Promise<void>)=>ws.invoke((original:any)=>api.withRepositoryBuildInferenceDatasetV1(root,original,createIssuedResourceControlV1(original,"graph:records-first",0),dataset,async(set:any)=>{
  let computes=0;const infer=async(index:number)=>{await observation.observeInferenceSnapshotV1(()=>observation.observeInference({cachePath:join(set.root,"inference","emoji-project-json.json"),key:["outer",Math.random()]},()=>observation.observeProjectInferenceV1({cachePath:cachePathOf(set.root,index),key:["rf",index],owner:"owner-"+index},async()=>{computes++;return{index,text:observation.memoizedFileRecord("rf",join(source,"in"+index+".txt"),(bytes:Buffer)=>bytes.toString())};})));return computes;};
  await body(set.root,infer);
 }),{owner:"records-first"});
 let old=new Date(Date.now()-48*3600000);
 await read(async(directory,infer)=>{
  for(let index=0;index<units;index++){
   await infer(index);
   const{openRecordStore}=await import("../../../../⚡️caching/🗃️records/🟦️.ts") as any,store=openRecordStore(recordsOf(directory,index),{publication:(plan:any,write:()=>void)=>api.withBuildGraphOptionalPublicationV1(join(recordsOf(directory,index),"0-0-0000000000000000.pack"),plan,write)});
   try{for(let value=0;value<values;value++)store.put(sha("value-"+index+"-"+value),Buffer.alloc(valueBytes,value+1));store.flush();}finally{store.close();}
   const stamp=new Date(old.getTime()+index*1000);for(const name of readdirSync(recordsOf(directory,index)))utimesSync(join(recordsOf(directory,index),name),stamp,stamp);utimesSync(cachePathOf(directory,index),stamp,stamp);
  }
  expect(api.buildGraphWriterOptionalQuotaFactsV1(root).usedBytes).toBeGreaterThan(policy.lowWaterBytes);
 });
 let after:string[]=[],packs:boolean[]=[],used=0;
 await read(async(directory)=>{used=api.buildGraphWriterOptionalQuotaFactsV1(root).usedBytes;after=readdirSync(join(directory,"np")).filter(name=>name.endsWith(".json"));packs=Array.from({length:units},(_,index)=>existsSync(recordsOf(directory,index))&&readdirSync(recordsOf(directory,index)).some(name=>name.endsWith(".pack")));});
 expect(used).toBeLessThanOrEqual(policy.lowWaterBytes);
 expect(after.length).toBe(units);
 expect(packs[0]).toBe(false);expect(packs[units-1]).toBe(true);expect(packs.filter(Boolean).length).toBeLessThan(units);
 await read(async(_directory,infer)=>{expect(await infer(0)).toBe(0);expect(await infer(units-1)).toBe(0);});
},600000);
