import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolChildLimitsV1,ownerPoolFitsV1,pullOwnerPoolV1,type OwnerPoolUnitV1}from"../../🟦️.ts";
import{createOwnerPoolPublisherV1}from"../../📣️publisher/🟦️.ts";
import{OWNER_POOL_FLEET_POLICY_V1 as F,ownerPoolTurnMillisecondsV1,type OwnerPoolMachineV1}from"../../🤝️fleet/🟦️.ts";

/** 🧭️ An owner that does not fit the reader is asked for with its need in a wave of its own; of the machines that can hold it the largest free one claims it at once and the others only when it still lies there after their turn; a machine that cannot hold it never claims it. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const fast={...P,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
const now0=Date.now();
const machine=(origin:string,gigabytes:number,rss=0):OwnerPoolMachineV1=>({origin,totalMegabytes:gigabytes*1024,grantMegabytes:ownerPoolChildLimitsV1(gigabytes*1024).hardMegabytes,state:"pulling",rssMegabytes:rss,updatedMs:now0});
const fleet=[machine("m8",8),machine("m16",16,1500),machine("m32",32,6000)];
const turnPolicy={...F,turnMilliseconds:150};
type Unit=OwnerPoolUnitV1;
const unitOf=(name:string,megabytes?:number):Unit=>({name,kind:"k",expectedMilliseconds:60000,...megabytes===undefined?{}:{megabytes},json:{name}});

test("three simulated machines: the 32 GB machine takes the 4842 MB owner, the 16 GB machine never needs to, the 8 GB machine never claims it, each owner is computed once",async()=>{
 const table=new Map<string,string>(),computed=new Map<string,string[]>();
 const claimOf=(who:string)=>async(unit:Unit)=>{await pause(1);const held=table.get(unit.name);if(held==="stored")return{state:"stored" as const};if(held!==undefined)return{state:"held" as const,by:held};table.set(unit.name,who);return{state:"granted" as const,digest:sha(unit.name),why:()=>"",deterministic:()=>false,code:()=>"",publish:async()=>{table.set(unit.name,"stored");return true;},abandon:async()=>{table.delete(unit.name);}};};
 const units=[...Array.from({length:12},(_all,index)=>unitOf("light-"+String(index).padStart(2,"0"),100)),unitOf("norm",4842)];
 const puller=(machineRow:OwnerPoolMachineV1)=>pullOwnerPoolV1<Unit>({units:()=>units,fits:unit=>ownerPoolFitsV1(unit,machineRow.grantMegabytes),turn:unit=>ownerPoolTurnMillisecondsV1(machineRow.origin,fleet,unit.megabytes??0,now0,turnPolicy)??0,puller:machineRow.origin+":1",policy:fast,slots:()=>1,claim:claimOf(machineRow.origin),run:async unit=>{computed.set(unit.name,[...computed.get(unit.name)??[],machineRow.origin]);await pause(2);return true;}});
 await Promise.all(fleet.map(puller));
 for(const unit of units)expect(computed.get(unit.name)?.length,unit.name).toBe(1);
 expect(computed.get("norm")).toEqual(["m32"]);
});

test("when the largest machine is gone the next one claims after its turn and not before",async()=>{
 const table=new Map<string,"held"|"stored">();let claimedAt:number|undefined;
 const started=Date.now();
 const claim=async(unit:Unit)=>{await pause(1);if(table.get(unit.name)==="stored")return{state:"stored" as const};claimedAt??=Date.now()-started;table.set(unit.name,"held");return{state:"granted" as const,digest:sha(unit.name),why:()=>"",deterministic:()=>false,code:()=>"",publish:async()=>{table.set(unit.name,"stored");return true;},abandon:async()=>{table.delete(unit.name);}};};
 const row=fleet.find(each=>each.origin==="m16")!;
 await pullOwnerPoolV1<Unit>({units:()=>[unitOf("norm",4842)],fits:unit=>ownerPoolFitsV1(unit,row.grantMegabytes),turn:unit=>ownerPoolTurnMillisecondsV1(row.origin,fleet,unit.megabytes??0,now0,turnPolicy)??0,puller:"m16:1",policy:fast,slots:()=>1,claim,run:async()=>true});
 expect(table.get("norm")).toBe("stored");
 expect(claimedAt!,"rank 1 waits one turn").toBeGreaterThanOrEqual(turnPolicy.turnMilliseconds-10);
});

test("a heavy owner is asked for in a wave of its own with its need, apart from the wave of the read; asking twice publishes once, forcing publishes again",async()=>{
 const cache=mkdtempSync(join(tmpdir(),"owner-turn-cache-")),root=mkdtempSync(join(tmpdir(),"owner-turn-root-"));scratch.push(cache,root);
 const server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={...process.env,SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},published:{units:Unit[]}[]=[];
  const publisher=createOwnerPoolPublisherV1({workspaceRoot:root,environment,identity:"id",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},session:()=>({}),units:()=>Array.from({length:20},(_all,index)=>unitOf("o"+index)),publish:async input=>{published.push({units:[...input.manifest.units]});return true;}});
  const heavy=[unitOf("norm",4842)];
  expect(await publisher.ask(heavy)).toBe(true);
  expect(published.length).toBe(1);
  expect(published[0]!.units.map(unit=>[unit.name,unit.megabytes])).toEqual([["norm",4842]]);
  expect(await publisher.ask(heavy)).toBe(true);
  expect(published.length,"unchanged ask is not published twice").toBe(1);
  expect(await publisher.ask(heavy,true)).toBe(true);
  expect(published.length).toBe(2);
  publisher.announce(20);await pause(50);
  expect(published.length,"the wave of the read is a separate publication").toBe(3);
  expect(published[2]!.units.length).toBe(20);
  expect(await publisher.ask([])).toBe(false);
 }finally{await server.stop();}
});
