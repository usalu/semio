import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{claimWantedV1,fleetAdmittedMegabytesV1,publishWantedV1,readServedV1,readWantedV1,wantedPolicyV1,wantedTextV1}from"../../🟦️.ts";
import{fleetReadV1}from"../../📖️read/🟦️.ts";
import{estimateOwnerCostV1,mergeOwnerCostTablesV1,ownerCostPolicyV1,publishOwnerCostsV1,readOwnerCostsV1,recordOwnerCostV1,emptyOwnerCostTableV1}from"../../💰️cost/🟦️.ts";
import{runWantedServerV1,wantedServerPolicyV1,type WantedInferV1}from"../../🛠️server/🟦️.ts";

/** 🌍️ Contract: the request of a Linux worker is read, claimed and answered by a macOS server (a slot is per requester), the request of a Windows worker is served across platform classes except the projects bound to its host, which are named and left to a machine of its class, said once and counted in the heartbeat; a request that holds nothing but bound projects is left unclaimed, a worker that can give 0 MB per project at a small booking still infers the unknown projects, and the costs the server measured reach a worker of another platform. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-two-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const}),ok=(generation:number):WantedInferV1=>({ok:true,status:0,seconds:12,generation,detail:""});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
const base=(serving:ReturnType<Awaited<ReturnType<typeof world>>["client"]>,lines:string[],requesters:string[],reads:string[])=>({client:serving,owner:"g1-server",platform:"darwin-arm64",steerDirectory:temp("steer"),language:"en" as const,requesters:async()=>requesters,headroom:roomy,infer:async(reason:string)=>{reads.push(reason);return ok(9);},quietMilliseconds:async()=>0,say:(line:string)=>lines.push(line),signal:new AbortController().signal,sleep:async()=>{},rounds:1});

test("the request of the Linux worker is read without its platform, claimed by the macOS server, served and answered; the answer is read back by the worker",async()=>{
 const {server,client}=await world();
 try{
  const linux=client("linux"),mac=client("mac"),lines:string[]=[],reads:string[]=[];
  expect(await publishWantedV1({client:linux,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,projects:["p2","p1","p3"]})).toBe(true);
  const seen=await readWantedV1({client:mac,requester:"g1-m04"});
  expect(seen?.platform).toBe("linux-x64");expect(seen?.projects).toEqual(["p1","p2","p3"]);
  await runWantedServerV1(base(mac,lines,["g1-m04"],reads));
  expect(reads).toEqual(["request"]);
  const answer=await readServedV1({client:linux,requester:"g1-m04"});
  expect(answer?.projects).toEqual(["p1","p2","p3"]);expect(answer?.platform).toBe("linux-x64");expect(answer?.server).toBe("g1-server");
  expect(lines).toEqual([wantedTextV1("serving","en",{count:3,requester:"g1-m04"}),wantedTextV1("served","en",{count:3,requester:"g1-m04",seconds:12})]);
 }finally{await server.stop(true);}
},60000);

const BOUND="@semio-tech/os-plugin-describe-rs";
test("the request of a Windows worker is served across classes by the macOS server except the bound owner, which is named, left and counted; a request of bound owners only is left unclaimed",async()=>{
 const {server,client}=await world();
 try{
  const windows=client("windows"),linux=client("linux"),mac=client("mac"),other=client("other"),lines:string[]=[],reads:string[]=[];
  await publishWantedV1({client:windows,requester:"g1-w1",platform:"win32-x64",graphKey:KEY,projects:["w1","w2",BOUND]});
  await publishWantedV1({client:linux,requester:"g1-w2",platform:"win32-arm64",graphKey:KEY,projects:[BOUND]});
  const policy={...wantedServerPolicyV1,aliveRounds:1};
  await runWantedServerV1({...base(mac,lines,["g1-w1","g1-w2"],reads),server:policy,rounds:2});
  expect(reads).toEqual(["request"]);
  expect(lines.filter(line=>line.includes("g1-w1")&&line.includes("across platform classes"))).toEqual([wantedTextV1("cross","en",{requester:"g1-w1",platform:"win32-x64",class:"windows-shared",own:"posix-shared",served:2,left:1,named:": "+BOUND})]);
  expect(lines.filter(line=>line.includes("g1-w2")&&line.includes("bound to the host"))).toHaveLength(1);
  expect(lines.some(line=>line===wantedTextV1("alive","en",{requesters:2,held:1,cross:2,left:2,reading:"request"}))).toBe(true);
  expect(lines.some(line=>line.startsWith("graph-server: serving 2 stale projects for g1-w1"))).toBe(true);
  const answer=await readServedV1({client:windows,requester:"g1-w1"});
  expect(answer?.projects).toEqual(["w1","w2"]);expect(answer?.left).toEqual([BOUND]);
  expect((await readServedV1({client:linux,requester:"g1-w2"}))?.projects??[]).toEqual([]);
  expect((await claimWantedV1({client:other,owner:"g1-w9",platform:"win32-x64",requesters:["g1-w1","g1-w2"],known:()=>false})).map(request=>[request.requester,request.projects.length,request.left??[]])).toEqual([["g1-w2",1,[]]]);
 }finally{await server.stop(true);}
},60000);

test("a Linux worker names only the measured heavy projects even when its early booking is small, and with its real capacity it infers a wave of unknown projects itself",async()=>{
 const small=fleetAdmittedMegabytesV1({hostMegabytes:15848});
 expect(small).toBe(7924);
 expect(fleetAdmittedMegabytesV1({hostMegabytes:15848})).toBe(7924);
 expect(fleetAdmittedMegabytesV1({hostMegabytes:1000})).toBe(0);
 const names=[...Array.from({length:517},(_,at)=>`@semio-tech/p${at}`),"@semio-tech/s-dev-composition","workspace"],lines:string[]=[];
 const result=await fleetReadV1({stale:names,admittedMegabytes:2048,client:undefined,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,name==="workspace"?"root:language:cargo":"project"),say:line=>lines.push(line)}).catch(error=>error);
 expect(String((result as Error).message??result)).toContain("graph.wanted-expired");
 const text=lines.join("\n");expect(text).toContain("@semio-tech/s-dev-composition");expect(text).toContain("workspace");expect(text).not.toContain("@semio-tech/p0 ");
 expect(text).toContain("linux-x64");expect(text).toContain("posix-shared");
 const local=await fleetReadV1({stale:names.slice(0,517),admittedMegabytes:small,client:undefined,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,"project"),say:line=>lines.push(line)}).catch(error=>error);
 expect((local as {outcome:string}).outcome).toBe("served");expect((local as {missing:string[]}).missing).toHaveLength(517);
 expect(lines.some(line=>line.includes("it can infer 192 of the others (48 per wave)")||line.includes("this machine can infer 192 (48 per wave)"))).toBe(true);
},60000);

test("the costs the macOS server measured reach a worker of another platform and decide only by name",async()=>{
 const {server,client}=await world();
 try{
  const mac=client("mac"),linux=client("linux"),table=recordOwnerCostV1(emptyOwnerCostTableV1(),{name:"@semio-tech/os-kernel",kind:"project",megabytes:3900,seconds:70,rows:423,stopped:false,atMs:5});
  expect(await publishOwnerCostsV1({client:mac,server:"g1-server",table})).toBe(true);
  const read=await readOwnerCostsV1({client:linux});expect(read).toEqual(table);
  expect(estimateOwnerCostV1(mergeOwnerCostTablesV1([read]),"@semio-tech/os-kernel")).toEqual({megabytes:3900,source:"recorded"});
  expect(ownerCostPolicyV1.heavySources).toEqual(["recorded","seed"]);
 }finally{await server.stop(true);}
},60000);

test("the whole path: the Linux worker (16 GB, 300 stale projects, 192 inferable) asks, the macOS server answers, the worker continues with what it can infer itself left over",async()=>{
 const {server,client}=await world();
 try{
  const linux=client("linux"),mac=client("mac"),lines:string[]=[],serverLines:string[]=[],reads:string[]=[];let served=false;
  const stale=Array.from({length:300},(_,at)=>`@semio-tech/p${String(at).padStart(3,"0")}`);
  const result=await fleetReadV1({stale,admittedMegabytes:7924,client:linux,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,"project"),say:line=>lines.push(line),
   async sleep(){if(served)return;served=true;await runWantedServerV1(base(mac,serverLines,["g1-m04"],reads));}});
  expect(result.outcome).toBe("served");expect(result.arrived).toBeGreaterThanOrEqual(300-192);expect(result.missing.length).toBeLessThanOrEqual(192);
  expect((await readServedV1({client:linux,requester:"g1-m04"}))?.projects).toHaveLength(300);
  expect(reads).toEqual(["request"]);
  expect(lines[0]).toBe(wantedTextV1("plan","en",{stale:300,taken:0,local:192,wave:48,wanted:300}));
  expect(serverLines[0]).toBe(wantedTextV1("serving","en",{count:300,requester:"g1-m04"}));
 }finally{await server.stop(true);}
},60000);

test("the whole path of a Windows worker: 300 stale projects of which one is bound to its host, the macOS server serves the 299 others, the worker keeps the bound one for itself and says so",async()=>{
 const {server,client}=await world();
 try{
  const windows=client("windows"),mac=client("mac"),lines:string[]=[],serverLines:string[]=[],reads:string[]=[];let served=false;
  const stale=[BOUND,...Array.from({length:299},(_,at)=>`@semio-tech/p${String(at).padStart(3,"0")}`)];
  const result=await fleetReadV1({stale,admittedMegabytes:7924,client:windows,requester:"g1-w1",platform:"win32-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,"project"),say:line=>lines.push(line),
   async sleep(){if(served)return;served=true;await runWantedServerV1({...base(mac,serverLines,["g1-w1"],reads),server:{...wantedServerPolicyV1,aliveRounds:1}});}});
  expect(result.outcome).toBe("served");expect(result.missing).toContain(BOUND);expect(result.missing.length).toBeLessThanOrEqual(193);expect(result.arrived).toBeGreaterThanOrEqual(300-193);
  expect((await readServedV1({client:windows,requester:"g1-w1"}))?.projects).toHaveLength(299);
  expect(reads).toEqual(["request"]);
  expect(lines).toContain(wantedTextV1("leftHere","en",{count:1,named:BOUND}));
  expect(serverLines[0]).toBe(wantedTextV1("serving","en",{count:299,requester:"g1-w1"}));
  expect(serverLines).toContain(wantedTextV1("cross","en",{requester:"g1-w1",platform:"win32-x64",class:"windows-shared",own:"posix-shared",served:299,left:1,named:": "+BOUND}));
 }finally{await server.stop(true);}
},60000);
