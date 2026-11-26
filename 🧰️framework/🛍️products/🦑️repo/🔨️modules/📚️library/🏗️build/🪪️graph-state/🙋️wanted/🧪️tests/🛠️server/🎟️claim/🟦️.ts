import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{CLAIM_POLICY_V1,createClaimTableV1,type ClaimTableV1}from"../../../../../🌐️remote-cache/🎟️claims/🟦️.ts";
import{graphFleetClientV1}from"../../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../🧶️nx/🟦️.ts";
import{publishWantedV1,readWantedV1,wantedClaimKeyV1,wantedPolicyV1,wantedTextV1}from"../../../🟦️.ts";
import{runWantedServerV1,wantedServerPolicyV1,type WantedInferV1}from"../../../🛠️server/🟦️.ts";
import memory from"../../../../../🧠️memory/🧬️schema/🔣️.json"with{type:"json"};
import type{GraphFleetClientV1}from"../../../../🛰️exchange/🟦️.ts";

/** 🎟️ Contract: the claim of a request served by this machine is kept alive for as long as the read runs: its time to live stays below the clamp of the cache server, it is renewed three times per time to live, and it asks for the longest lifetime; a claim that is lost is told with the machine that holds it, never as a guess. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`claim-lifetime-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLATFORM="darwin-arm64",KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
const viaTable=(base:GraphFleetClientV1,table:ClaimTableV1):GraphFleetClientV1=>({...base,
 async claim(key,owner,ttl,expected){
  const out=table.claim("main","ac",key,{owner,ttlMilliseconds:ttl,...expected===undefined?{}:{expectedMilliseconds:expected}});
  return out.state==="granted"?{state:"granted",untilMilliseconds:out.untilMilliseconds}:out.state==="held"?{state:"held",by:out.by,untilMilliseconds:out.untilMilliseconds}:out.state==="stored"?{state:"stored"}:{state:"full"};
 },
 async renew(key,owner,ttl){return table.renew("main","ac",key,{owner,ttlMilliseconds:ttl}).state==="granted"?"granted":"lost";},
});
const pending=(reason:"request"|"refresh",signal:AbortSignal|undefined)=>new Promise<WantedInferV1>(done=>signal?.addEventListener("abort",()=>done({ok:true,status:0,seconds:1,generation:1,detail:""}),{once:true}));

test("the policy of the serving claim fits the cache server: the time to live is below its clamp, renewed three times per time to live, and the expected time asks for the longest lifetime",()=>{
 const p=wantedPolicyV1,c=CLAIM_POLICY_V1;
 expect(p.claimTtlMilliseconds).toBeLessThanOrEqual(c.maximumTtlMilliseconds);
 expect(p.claimTtlMilliseconds).toBeGreaterThanOrEqual(c.minimumTtlMilliseconds);
 expect(p.claimRenewMilliseconds*3).toBeLessThanOrEqual(p.claimTtlMilliseconds);
 expect(Math.min(c.maximumLifetimeMilliseconds,Math.max(c.minimumLifetimeMilliseconds,p.claimTtlMilliseconds,p.claimExpectedMilliseconds*c.lifetimeMultiple))).toBe(c.maximumLifetimeMilliseconds);
});

test("only a machine with the memory of a serving host claims: the gate equals the memory plan of the graph and a 16 GB worker offers no headroom",()=>{
 expect(wantedServerPolicyV1.serveTotalMegabytes).toBe(memory.definitions.Policy.const.serveHostMegabytes);
 expect(16384).toBeLessThan(wantedServerPolicyV1.serveTotalMegabytes);
 expect(32768).toBeGreaterThanOrEqual(wantedServerPolicyV1.serveTotalMegabytes);
});

test("a claim held by the server outlives the clamp of the cache server: renewed for fifteen minutes of a running read, never lost, served once",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),lines:string[]=[],controller=new AbortController(),table=createClaimTableV1({now:()=>time,sweepMilliseconds:0});
  let time=Date.now(),ticks=0;
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["a","b"],now:()=>time});
  const request=(await readWantedV1({client:requester,requester:"g1-m06",now:()=>time}))!;
  await runWantedServerV1({client:viaTable(client("serving"),table),owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m06","g1-server"],headroom:roomy,infer:pending,quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:controller.signal,now:()=>time,sleep:async milliseconds=>{time+=milliseconds;if(++ticks>=180)controller.abort();}});
  expect(lines.filter(line=>line===wantedTextV1("serving","en",{count:2,requester:"g1-m06"}))).toHaveLength(1);
  expect(lines.filter(line=>/claim for/u.test(line))).toEqual([]);
  expect(table.claimed("main","ac",wantedClaimKeyV1(request))).toBe(true);
 }finally{await server.stop();}
},60000);

test("a claim another machine took is told with that machine and left to it; the request is not served twice",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),lines:string[]=[],controller=new AbortController(),table=createClaimTableV1({now:()=>time,sweepMilliseconds:0});
  let time=Date.now(),ticks=0,key="";
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["a","b"],now:()=>time});
  key=wantedClaimKeyV1((await readWantedV1({client:requester,requester:"g1-m06",now:()=>time}))!);
  await runWantedServerV1({client:viaTable(client("serving"),table),owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m06","g1-server"],headroom:roomy,infer:pending,quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:controller.signal,now:()=>time,sleep:async milliseconds=>{time+=milliseconds;if(++ticks===4){table.release("main","ac",key,"g1-server");table.claim("main","ac",key,{owner:"g1-m09",ttlMilliseconds:180000});}if(ticks>=40)controller.abort();}});
  expect(lines.filter(line=>/claim for/u.test(line))).toEqual([wantedTextV1("lost","en",{requester:"g1-m06",by:"g1-m09"})]);
  expect(lines.filter(line=>line.includes("serving"))).toHaveLength(1);
  expect(wantedTextV1("lost","de",{requester:"g1-m06",by:"g1-m09"})).toContain("g1-m09");
 }finally{await server.stop();}
},60000);

test("a claim that lapsed without a thief is taken again at once and said so: the machine never claims to have lost it to another machine",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),lines:string[]=[],controller=new AbortController(),table=createClaimTableV1({now:()=>time,sweepMilliseconds:0});
  let time=Date.now(),ticks=0,key="";
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["a","b"],now:()=>time});
  key=wantedClaimKeyV1((await readWantedV1({client:requester,requester:"g1-m06",now:()=>time}))!);
  await runWantedServerV1({client:viaTable(client("serving"),table),owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m06","g1-server"],headroom:roomy,infer:pending,quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:controller.signal,now:()=>time,sleep:async milliseconds=>{time+=milliseconds;if(ticks===3)table.release("main","ac",key,"g1-server");if(++ticks>=40)controller.abort();}});
  expect(lines.filter(line=>/claim for/u.test(line))).toEqual([wantedTextV1("reclaimed","en",{requester:"g1-m06"})]);
  expect(lines.filter(line=>line.includes("serving"))).toHaveLength(1);
  expect(table.claimed("main","ac",key)).toBe(true);
 }finally{await server.stop();}
},60000);
