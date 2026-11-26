import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../🧶️nx/🟦️.ts";
import{publishWantedV1,readWantedV1}from"../../../🟦️.ts";
import{emptyOwnerCostTableV1,readOwnerCostsV1,recordOwnerCostV1}from"../../../💰️cost/🟦️.ts";
import{readPriorityV1}from"../../../../../🕸️graph/🕰️generations/🧭️steering/🟨️.mjs";
import{runWantedServerV1,type WantedInferV1}from"../../../🛠️server/🟦️.ts";

/** 💰️ Contract: the serving machine infers the heavy asked-for projects first, ordered by cost, and tells the fleet what it measured so that every reader plans with it. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-priority-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLATFORM="darwin-arm64",KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const});
const sample=(name:string,megabytes:number)=>({name,kind:"project",megabytes,seconds:1,rows:1,stopped:false,atMs:1});
const ok=(generation:number):WantedInferV1=>({ok:true,status:0,seconds:12,generation,detail:""});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}

test("the priority file lists the projects the most requests ask for first (b, asked by both), then the heavy projects of the requesters, then the others heaviest first (a recorded cost of 500 MB is lighter than the 820 MB of an unknown owner), equal cost in the order of the older request",async()=>{
 const {server,client}=await world();
 try{
  const first=client("first"),second=client("second"),serving=client("serving"),steer=temp("steer");
  let table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("c",4000));table=recordOwnerCostV1(table,sample("a",500));table=recordOwnerCostV1(table,sample("z",9000));
  await publishWantedV1({client:first,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,projects:["a","b","c","d"],heavy:["d"],now:()=>1_000_000});
  await publishWantedV1({client:second,requester:"g1-m07",platform:PLATFORM,graphKey:KEY,projects:["e","z","b"],now:()=>1_000_001});
  expect((await readWantedV1({client:serving,requester:"g1-m02",now:()=>1_000_002}))?.heavy).toEqual(["d"]);
  let release:()=>void=()=>{},calls=0;const seen:(readonly string[]|undefined)[]=[];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m02","g1-m07"],headroom:roomy,costs:()=>table,quietMilliseconds:async()=>0,say:()=>{},signal:new AbortController().signal,now:()=>1_000_010,rounds:4,
   infer:()=>new Promise<WantedInferV1>(resolve=>{release=()=>resolve(ok(2));}),
   sleep:async()=>{calls++;if(calls>=2){seen.push(readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer})?.projects);release();}}});
  expect(seen.find(list=>list!==undefined)).toEqual(["b","d","z","c","e","a"]);
 }finally{await server.stop();}
},60000);

test("without a table the heavy projects of the requesters still come first and the rest keeps the order of the requests",async()=>{
 const {server,client}=await world();
 try{
  const first=client("first"),serving=client("serving"),steer=temp("steer");
  await publishWantedV1({client:first,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,projects:["a","b","workspace"],heavy:["workspace"],now:()=>1_000_000});
  let release:()=>void=()=>{},calls=0;const seen:(readonly string[]|undefined)[]=[];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m02"],headroom:roomy,quietMilliseconds:async()=>0,say:()=>{},signal:new AbortController().signal,now:()=>1_000_010,rounds:4,
   infer:()=>new Promise<WantedInferV1>(resolve=>{release=()=>resolve(ok(2));}),
   sleep:async()=>{calls++;if(calls>=2){seen.push(readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer})?.projects);release();}}});
  expect(seen.find(list=>list!==undefined)).toEqual(["workspace","a","b"]);
 }finally{await server.stop();}
},60000);

test("the table of the server reaches the fleet, once per change and at most once per interval",async()=>{
 const {server,client}=await world();
 try{
  const serving=client("serving"),reader=client("reader");let time=1_000_000_000,table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("@semio-tech/s-dev-composition",13976)),step=0;
  const steps=[()=>{},()=>{table=recordOwnerCostV1(table,sample("workspace",3100));},()=>{time+=61_000;}];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>[],headroom:async()=>({freeMegabytes:100,pressure:"normal" as const}),infer:()=>{throw Error("no inference without a request");},costs:()=>table,quietMilliseconds:async()=>0,say:()=>{},signal:new AbortController().signal,now:()=>time,rounds:3,sleep:async()=>{steps[step++]?.();}});
  const afterFirst=await readOwnerCostsV1({client:reader});
  expect(Object.keys(afterFirst?.owners??{}).sort()).toEqual(["@semio-tech/s-dev-composition"]);
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>[],headroom:async()=>({freeMegabytes:100,pressure:"normal" as const}),infer:()=>{throw Error("no inference without a request");},costs:()=>table,quietMilliseconds:async()=>0,say:()=>{},signal:new AbortController().signal,now:()=>time,rounds:1,sleep:async()=>{}});
  expect(Object.keys((await readOwnerCostsV1({client:reader}))?.owners??{}).sort()).toEqual(["@semio-tech/s-dev-composition","workspace"]);
 }finally{await server.stop();}
},60000);
