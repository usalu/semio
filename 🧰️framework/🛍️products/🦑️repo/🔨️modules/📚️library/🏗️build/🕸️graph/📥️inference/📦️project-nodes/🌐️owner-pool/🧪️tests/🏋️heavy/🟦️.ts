import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1}from"../../../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import{graphFleetClientV1}from"../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{runOwnerPoolV1}from"../../🏃️runner/🟨️.mjs";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolFitsV1,ownerPoolHeavyV1,ownerPoolIdV1,publishOwnerPoolV1,pullOwnerPoolV1,type OwnerPoolManifestV1}from"../../🟦️.ts";
import{hereLimitBytesV1}from"../../../🟨️.mjs";

/** 🏋️ Heavy owners: an owner whose known or learned cost exceeds what a machine can give one inference is never claimed by that machine, every machine that can hold it takes it before anything else, and exactly one claims it. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-heavy-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const fast={...P,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
type Unit={name:string;kind:string;expectedMilliseconds:number;megabytes?:number;json:Record<string,unknown>};
const unitOf=(name:string,megabytes?:number):Unit=>({name,kind:"k",expectedMilliseconds:60000,...megabytes===undefined?{}:{megabytes},json:{name}});
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const table=new Map<string,"held"|"stored">();
const claimOf=async(unit:Unit)=>{
 await pause(1);const held=table.get(unit.name);
 if(held==="stored")return{state:"stored" as const};if(held==="held")return{state:"held" as const,by:"x"};
 table.set(unit.name,"held");
 return{state:"granted" as const,digest:sha(unit.name),why:()=>"",deterministic:()=>false,code:()=>"",publish:async()=>{table.set(unit.name,"stored");return true;},abandon:async()=>{table.delete(unit.name);}};
};

test("the size of a unit decides who may claim it: unknown fits everywhere, a known cost above the grant fits nobody there, the heavy size is declared",()=>{
 expect(ownerPoolFitsV1({},100)).toBe(true);
 expect(ownerPoolFitsV1({megabytes:4915},4915)).toBe(true);
 expect(ownerPoolFitsV1({megabytes:4916},4915)).toBe(false);
 expect(ownerPoolHeavyV1({megabytes:P.heavyMegabytes})).toBe(true);
 expect(ownerPoolHeavyV1({megabytes:P.heavyMegabytes-1})).toBe(false);
 expect(ownerPoolHeavyV1({})).toBe(false);
});

test("a small puller never claims an owner above its grant and still ends, a big puller takes every heavy owner first, and each owner is inferred once",async()=>{
 table.clear();
 const units=[...Array.from({length:30},(_all,index)=>unitOf("light-"+String(index).padStart(2,"0"),100)),unitOf("giant-a",9000),unitOf("giant-b",12000),unitOf("giant-c",5000)];
 const computed=new Map<string,string[]>(),order:string[]=[];
 const puller=(name:string,grant:number)=>pullOwnerPoolV1<Unit>({units:()=>units,fits:unit=>ownerPoolFitsV1(unit,grant),puller:name,policy:fast,slots:()=>1,claim:claimOf,run:async unit=>{computed.set(unit.name,[...computed.get(unit.name)??[],name]);if(name==="big:1")order.push(unit.name);await pause(2);return true;}});
 const [small,big]=await Promise.all([puller("small:1",4915),puller("big:1",20000)]);
 for(const unit of units)expect(computed.get(unit.name)?.length).toBe(1);
 for(const giant of["giant-a","giant-b","giant-c"])expect(computed.get(giant)).toEqual(["big:1"]);
 expect(order.slice(0,3).sort()).toEqual(["giant-a","giant-b","giant-c"]);
 expect(small.published+big.published).toBe(units.length);
 expect(small.published).toBeGreaterThan(0);
});

test("no machine that can hold a heavy owner exists: the small machines finish the rest and end, the heavy owner stays unclaimed for the reader to wait for",async()=>{
 table.clear();
 const units=[...Array.from({length:6},(_all,index)=>unitOf("light-"+index,100)),unitOf("giant",64000)];
 const stats=await pullOwnerPoolV1<Unit>({units:()=>units,fits:unit=>ownerPoolFitsV1(unit,4915),puller:"small:2",policy:fast,slots:()=>1,claim:claimOf,run:async()=>true});
 expect(stats.published).toBe(6);
 expect(table.has("giant")).toBe(false);
});

test("the limit of an owner that runs alone never exceeds what the plan guard allows the plan, so the owner is stopped first and handed to the fleet instead of failing the plan",()=>{
 const mb=1048576;
 expect(hereLimitBytesV1({limitBytes:1756*mb,soloBytes:4214*mb,planBytes:Infinity})).toBe(4214*mb);
 expect(hereLimitBytesV1({limitBytes:1756*mb,soloBytes:9830*mb,planBytes:6553*mb})).toBe(Math.floor(6553*mb*0.9));
 expect(hereLimitBytesV1({limitBytes:5000*mb,soloBytes:9830*mb,planBytes:4915*mb})).toBe(5000*mb);
});

const driverPath=join(import.meta.dir,"..","🧫️fixtures","🧮️driver.mjs");
async function wave(totalMegabytes:number,units:Unit[],tag:string){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 try{
  const root=temp("root"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const partial={version:1 as const,identity:"identity-"+tag,globals:{registryDigest:"r",commandRevision:"c",late:false},publisher:"p:"+tag};
  const manifest:OwnerPoolManifestV1={head:{...partial,id:ownerPoolIdV1(partial,units),createdMs:Date.now()},session:Buffer.from(JSON.stringify({burnMilliseconds:5})),units};
  expect(await publishOwnerPoolV1({client,owner:"o",manifest})).toBe(true);
  const done:string[]=[];
  await runOwnerPoolV1({totalMegabytes,root,environment:{...process.env,...environment},signal:new AbortController().signal,capacity:async()=>({children:1,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,pollMilliseconds:500,observe:(_id:number,line:{t:string;ok?:boolean;name?:string})=>{if(line.t==="done"&&line.ok&&line.name)done.push(line.name);}});
  const reader=createProjectFleetPortV1({root:temp("reader"),environment,owner:"r:1"})!,stored:string[]=[];
  for(const unit of units){const claim=await reader.claim({cachePath:join(root,"np",unit.name+".json"),fleet:`libs/${unit.name}`,keyText:JSON.stringify(["rev",null,[unit.name]]),exportRecord:()=>({digest:sha(unit.name),authority:[],observations:[],image:"{}"}),async import(){return true;}});if(claim.state==="stored")stored.push(unit.name);else if(claim.state==="granted")await claim.abandon();}
  return{done,stored};
 }finally{await server.stop();}
}

test("real child processes: a 16 GB machine's children leave a 20 GB owner unclaimed and infer the rest, a 128 GB machine's child takes the heavy owner first",async()=>{
 const units=[...Array.from({length:8},(_all,index)=>unitOf("light-"+index,100)),unitOf("giant",20000)];
 const small=await wave(16384,units,"small");
 expect(small.done).not.toContain("giant");
 expect(small.stored.filter(name=>name.startsWith("light-")).length).toBe(8);
 expect(small.stored).not.toContain("giant");
 const big=await wave(131072,units,"big");
 expect(big.done[0]).toBe("giant");
 expect(big.stored.length).toBe(9);
},120000);
