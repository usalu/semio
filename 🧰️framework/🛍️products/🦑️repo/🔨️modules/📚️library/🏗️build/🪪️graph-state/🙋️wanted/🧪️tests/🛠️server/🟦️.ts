import Ajv from"ajv";
import{readFileSync}from"node:fs";
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{publishWantedV1,readServedV1,wantedPolicyV1,wantedTextV1}from"../../🟦️.ts";
import{emptyOwnerCostTableV1,recordOwnerCostV1}from"../../💰️cost/🟦️.ts";
import type{GraphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{readPriorityV1,writePendingV1}from"../../../../🕸️graph/🕰️generations/🧭️steering/🟨️.mjs";
import{createGraphReadWatchV1,graphReadEndTextV1,graphReadStopRowV1,runWantedServerV1,wantedServerPolicyV1,yieldToInteractiveV1,type GraphYieldViewV1,type WantedInferV1}from"../../🛠️server/🟦️.ts";
import refreshCases from "./refresh.json" with {type:"json"};

/** 🛠️ Contract: a machine with headroom serves the live requests of the others and tells the requester which projects are done; when nobody asks it refreshes the fleet generation after the worktree went quiet (or the generation got old), never without headroom, and a failing read answers nobody and is not retried at once. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-server-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLATFORM="darwin-arm64",KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
const ok=(generation:number):WantedInferV1=>({ok:true,status:0,seconds:12,generation,detail:""});

test("held local custody defers only automatic refresh and release keeps the original cooldown",async()=>{
 const attempted:number[]=[],started:number[]=[];
 let round=0;
 const times=refreshCases.rounds;
 await runWantedServerV1({
  client:{} as GraphFleetClientV1,owner:"g1-server",platform:PLATFORM,language:"en",
  steerDirectory:temp("refresh"),requesters:async()=>[],headroom:roomy,mayServe:async()=>false,
  quietMilliseconds:async()=>10**9,signal:new AbortController().signal,say:()=>{},
  sleep:async()=>{round++;},rounds:times.length,now:()=>times[Math.min(round,times.length-1)]!.at,
  refreshDeferred:()=>times[round]!.holding,
  mayRefresh:async()=>{attempted.push(times[round]!.at);return attempted.length===2;},
  infer:async reason=>{expect(reason).toBe("refresh");started.push(times[round]!.at);return ok(1);},
 });
 expect(attempted).toEqual(refreshCases.attempted);
 expect(started).toEqual(refreshCases.started);
});

test("a request is served by one graph read and answered; the next round finds the claim done and the answer in place",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),reads:string[]=[],lines:string[]=[],controller=new AbortController();
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["b","a","c"]});
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m06","g1-server"],headroom:roomy,infer:async reason=>{reads.push(reason);return ok(9);},quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:controller.signal,sleep:async()=>{},rounds:1,refreshDeferred:()=>{throw Error("Requested reads are independent of refresh custody");}});
  expect(reads).toEqual(["request"]);
  const answer=await readServedV1({client:requester,requester:"g1-m06"});
  expect(answer?.projects).toEqual(["a","b","c"]);expect(answer?.generation).toBe(9);expect(answer?.server).toBe("g1-server");
  expect(lines).toEqual([wantedTextV1("serving","en",{count:3,requester:"g1-m06"}),wantedTextV1("served","en",{count:3,requester:"g1-m06",seconds:12})]);
 }finally{await server.stop(true);}
},60000);

test("no headroom or a critical pressure serves nobody and refreshes nothing; a failing read answers nothing complete and is held off for the quiet time",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),reads:string[]=[],lines:string[]=[];let time=1_000_000_000;
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["a"]});
  const base={client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"de" as const,requesters:async()=>["g1-m06"],quietMilliseconds:async()=>10**9,say:(line:string)=>lines.push(line),signal:new AbortController().signal,now:()=>time,sleep:async()=>{},rounds:3};
  await runWantedServerV1({...base,headroom:async()=>({freeMegabytes:3000,pressure:"normal"}),infer:async reason=>{reads.push(reason);return ok(1);}});
  await runWantedServerV1({...base,headroom:async()=>({freeMegabytes:20000,pressure:"critical"}),infer:async reason=>{reads.push(reason);return ok(1);}});
  expect(reads).toEqual([]);
  await runWantedServerV1({...base,headroom:roomy,infer:async reason=>{reads.push(reason);return{ok:false,status:1,seconds:3,generation:null,detail:"boom"};}});
  expect(reads).toEqual(["request"]);
  expect((await readServedV1({client:requester,requester:"g1-m06"}))?.projects).toEqual([]);
  expect(lines.some(line=>line.startsWith("Graph-Server: das Lesen des Graphen endete nach 3 s mit Status 1: boom"))).toBe(true);
 }finally{await server.stop(true);}
},60000);

test("a 16 GB machine with 6144 MB free serves a request that a refresh would still wait on",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),reads:string[]=[];
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["a"]});
  const base={client:client("serving"),owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en" as const,requesters:async()=>["g1-m06"],quietMilliseconds:async()=>10**9,say:(_line:string)=>{},signal:new AbortController().signal,sleep:async()=>{},rounds:1,infer:async(reason:"request"|"refresh")=>{reads.push(reason);return ok(4);}};
  await runWantedServerV1({...base,headroom:async()=>({freeMegabytes:wantedServerPolicyV1.requestHeadroomMegabytes-1,pressure:"normal"})});
  expect(reads).toEqual([]);
  await runWantedServerV1({...base,headroom:async()=>({freeMegabytes:wantedServerPolicyV1.requestHeadroomMegabytes,pressure:"normal"})});
  expect(reads).toEqual(["request"]);
 }finally{await server.stop(true);}
},60000);

test("the background refresh waits for a quiet worktree, has a maximum age against constant edits, needs headroom and is announced",async()=>{
 const {server,client}=await world();
 try{
  const serving=client("serving"),reads:string[]=[],lines:string[]=[],p=wantedServerPolicyV1;let time=1_000_000_000,quiet=1000,step=0;
  const steps:(()=>void)[]=[()=>{},()=>{time+=p.refreshQuietMilliseconds;},()=>{quiet=p.refreshQuietMilliseconds;},()=>{quiet=1000;time+=p.refreshIntervalMilliseconds;}],seen:number[]=[];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>[],headroom:roomy,infer:async reason=>{reads.push(reason);return ok(4);},quietMilliseconds:async()=>quiet,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>time,sleep:async()=>{seen.push(reads.length);await Bun.sleep(0);steps[step++]?.();},rounds:5});
  expect(seen).toEqual([1,1,1,2,3]);
  expect(lines.filter(line=>line.includes("refreshed in 12 s"))).toHaveLength(3);
 }finally{await server.stop(true);}
},60000);

test("a machine that does not get the refresh slot of the interval does not refresh",async()=>{
 const {server,client}=await world();
 try{
  const serving=client("serving"),reads:string[]=[];let slots=0;
  await runWantedServerV1({client:serving,owner:"g1-m02",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>[],headroom:roomy,infer:async reason=>{reads.push(reason);return ok(4);},quietMilliseconds:async()=>10**9,mayRefresh:async()=>++slots>1,say:()=>{},signal:new AbortController().signal,now:()=>1_000_000_000,sleep:async()=>{},rounds:3});
  expect(slots).toBe(1);expect(reads).toEqual([]);
 }finally{await server.stop(true);}
},60000);

test("Law: a read is watched by its progress, not by a budget: waves, in English or German and however the chunks are cut, keep it alive for hours; silence for the stall limit or the absolute maximum stops it",()=>{
 const policy={...wantedServerPolicyV1,inferStallMilliseconds:1000,inferMaximumMilliseconds:10000};let time=0;
 const watch=createGraphReadWatchV1(policy,()=>time);
 expect(watch.verdict()).toBe("go");expect(watch.generation()).toBeNull();
 watch.feed("graph: gener");time=500;watch.feed("ation 5 published, 48 projects refreshed in 80 s (0 kept their edges)\n");expect(watch.generation()).toBe(5);
 time=1400;expect(watch.verdict(),"900 ms since the wave").toBe("go");
 watch.feed("graph: waiting for the graph build of a peer (3 s in the queue, generation 5 was the newest when this command queued)\n");time=1600;expect(watch.verdict(),"the same generation again is no progress").toBe("stalled");
 watch.feed("Graph: Generation 6 veröffentlicht, 48 Projekte in 90 s erneuert (0 behielten ihre Kanten)\n");expect(watch.verdict()).toBe("go");expect(watch.generation()).toBe(6);
 time=2500;watch.feed("graph: paused after generation 6: 1 command(s) wait for memory on this machine (60 s)\n");time=3400;expect(watch.verdict(),"a pause does not extend progress").toBe("stalled");
 time=4500;expect(watch.verdict()).toBe("stalled");expect(watch.silentMilliseconds()).toBe(2900);
 for(let generation=7;time<9500;generation++){time+=900;watch.feed(`graph: generation ${generation} published\n`);expect(watch.verdict()).toBe("go");}
 time=10000;watch.feed("graph: generation 99 published\n");expect(watch.verdict(),"the maximum ends even a read that makes progress").toBe("too-long");
 expect(graphReadEndTextV1("stalled","en")).toContain("nothing advanced for 25 min");expect(graphReadEndTextV1("stalled","de")).toContain("seit 25 min");
 expect(graphReadEndTextV1("too-long","en")).toContain("12 h");expect(graphReadEndTextV1("too-long","de")).toContain("12 h");
});

test("Law: the policy no longer holds a fixed budget for a read: 25 minutes without a wave, 12 hours at most",()=>{
 expect("inferTimeoutMilliseconds" in wantedServerPolicyV1).toBe(false);
 expect(wantedServerPolicyV1.inferStallMilliseconds).toBe(1_500_000);expect(wantedServerPolicyV1.inferMaximumMilliseconds).toBe(43_200_000);
 expect(wantedServerPolicyV1.inferStallMilliseconds,"a cold first wave (about 5 minutes here) fits several times").toBeGreaterThan(4*300_000);
});

test("A background refresh gives way between waves: it pauses while another command waits for memory or the pressure is raised, tells so in English and German, ignores small forecasts, workers and itself, advances one wave after the maximum and is never held by a gate it cannot read",async()=>{
 const policy={...wantedServerPolicyV1,yieldPollMilliseconds:10,yieldNoticeMilliseconds:25,yieldMaximumMilliseconds:200};let time=0;
 const row=(id:string,over:object={})=>({request:{id,...over}});
 const sleep=async(milliseconds:number)=>{time+=milliseconds;};
 const run=async(views:GraphYieldViewV1[],language:"en"|"de"="en",own:string|undefined="mine")=>{const lines:string[]=[];let at=0;const paused=await yieldToInteractiveV1({view:async()=>views[Math.min(at++,views.length-1)]!,own,generation:80,language,say:line=>lines.push(line),signal:undefined,sleep,now:()=>time,policy});return{paused,lines,looks:at};};
 time=0;expect((await run([{pressure:"normal",waiters:[]}])).paused,"nothing waits").toBe(0);
 time=0;expect((await run([{pressure:"normal",waiters:[row("mine",{elastic:false}),row("w1",{elastic:true}),row("q1",{quiet:true})]}])).paused,"itself, workers and small forecasts do not count").toBe(0);
 time=0;const busy=await run([{pressure:"normal",waiters:[row("other")]},{pressure:"normal",waiters:[row("other")]},{pressure:"normal",waiters:[row("other")]},{pressure:"normal",waiters:[]}]);
 expect(busy.paused).toBe(30);expect(busy.lines).toEqual([wantedTextV1("paused","en",{generation:80,waiters:1,seconds:0})]);expect(busy.lines[0]).toStartWith("graph: paused after generation 80: 1 command(s)");
 time=0;const german=await run([{pressure:"warn",waiters:[]},{pressure:"normal",waiters:[]}],"de");expect(german.paused).toBe(10);expect(german.lines[0]).toStartWith("Graph: pausiert nach Generation 80");
 time=0;expect((await run([{pressure:"normal",waiters:[row("other")]}])).paused,"after the maximum one wave goes on").toBeGreaterThanOrEqual(200);
 time=0;const unreadable=await yieldToInteractiveV1({view:async()=>{throw new Error("no gate");},own:undefined,generation:1,language:"en",say:()=>{throw new Error("silent");},signal:undefined,sleep,now:()=>time,policy});expect(unreadable).toBe(0);
 const controller=new AbortController();controller.abort();expect(await yieldToInteractiveV1({view:async()=>({pressure:"normal",waiters:[row("other")]}),own:undefined,generation:1,language:"en",say:()=>{},signal:controller.signal,sleep,now:()=>time,policy})).toBe(0);
});

test("a request that arrives while a read runs is seen in the next round, steers the running read, is answered from what is fresh and again as waves publish, and the read is not started twice",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),lines:string[]=[],reads:string[]=[],steer=temp("steer-live"),p=wantedServerPolicyV1;
  let time=1_000_000_000,release!:(value:WantedInferV1)=>void,round=0,requested=false;
  const names=Array.from({length:6},(_,index)=>"p"+index),answers:{round:number;projects:readonly string[]|undefined;generation:number|null|undefined}[]=[];
  const steps:(()=>Promise<void>|void)[]=[
   ()=>{},
   async()=>{await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:()=>time});requested=true;},
   ()=>{expect(readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer})?.projects).toEqual(names);},
   ()=>{writePendingV1({SEMIO_GRAPH_STEER_DIRECTORY:steer},5,["p3","p4","p5"],readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer}));},
   ()=>{time+=p.heartbeatMilliseconds;},
   ()=>{writePendingV1({SEMIO_GRAPH_STEER_DIRECTORY:steer},6,["p5"],readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer}));},
   ()=>{release(ok(7));},
  ];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,
   infer:reason=>{reads.push(reason);return new Promise<WantedInferV1>(resolve=>{release=resolve;});},
   quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>time,
   sleep:async()=>{
    const served=await readServedV1({client:requester,requester:"g1-m06"});answers.push({round,projects:served?.projects,generation:served?.generation});
    await steps[round++]?.();
   },rounds:7});
  expect(requested).toBe(true);
  expect(reads).toEqual(["refresh"]);
  expect(answers[0]!.projects).toBeUndefined();
  expect(answers[1]!.projects).toBeUndefined();
  expect(answers[2]!.projects).toEqual([]);
  expect(answers[3]!.projects).toEqual([]);
  expect(answers[4]!.projects).toEqual(["p0","p1","p2"]);
  expect(answers[5]!.projects).toEqual(["p0","p1","p2"]);
  expect(answers[6]!.projects).toEqual(["p0","p1","p2","p3","p4"]);
  const last=await readServedV1({client:requester,requester:"g1-m06"});
  expect(last?.projects).toEqual(names);expect(last?.generation).toBe(7);
  expect(lines).toContain(wantedTextV1("answering","en",{arrived:3,count:6,requester:"g1-m06"}));
  expect(lines.at(-1)).toBe(wantedTextV1("refreshed","en",{seconds:12}));
  expect(lines).toContain(wantedTextV1("served","en",{count:6,requester:"g1-m06",seconds:12}));
 }finally{await server.stop(true);}
},60000);

test("a refresh that cannot even start (an unset steering directory) is told once and backed off, never retried every round",async()=>{
 const {server,client}=await world();
 try{
  const lines:string[]=[],reads:string[]=[];let time=1_000_000_000;
  await runWantedServerV1({client:client("serving"),owner:"g1-server",platform:PLATFORM,steerDirectory:undefined as unknown as string,language:"en",requesters:async()=>[],headroom:roomy,infer:async reason=>{reads.push(reason);return ok(1);},quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>time,sleep:async()=>{time+=5000;},rounds:6});
  expect(reads).toEqual([]);expect(lines.filter(line=>line.includes("ended with status 1"))).toHaveLength(1);
 }finally{await server.stop(true);}
},60000);

test("a machine that may not serve claims no request: a capable machine still finds it and serves it",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),small=client("small"),serving=client("serving"),reads:string[]=[],lines:string[]=[],controller=new AbortController();
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:["a","b"]});
  await runWantedServerV1({client:small,owner:"g1-m08",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m06"],headroom:async()=>({freeMegabytes:0,pressure:"normal"}),mayServe:async()=>false,infer:async reason=>{reads.push(reason);return ok(1);},quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:controller.signal,sleep:async()=>{},rounds:2});
  expect(reads).toEqual([]);expect(lines).toEqual([]);
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m06"],headroom:roomy,mayServe:async()=>true,infer:async reason=>{reads.push(reason);return ok(9);},quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:controller.signal,sleep:async()=>{},rounds:1});
  expect(reads).toEqual(["request"]);
  expect((await readServedV1({client:requester,requester:"g1-m06"}))?.server).toBe("g1-server");
 }finally{await server.stop(true);}
},60000);

test("a server that finds nothing says that it is alive every aliveRounds rounds, with what it sees",async()=>{
 const {server,client}=await world();
 try{
  const lines:string[]=[],p=wantedServerPolicyV1;let time=1_000_000_000;
  await runWantedServerV1({client:client("serving"),owner:"g1-server",platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>["g1-m03","g1-m06"],headroom:async()=>({freeMegabytes:0,pressure:"normal"}),infer:async()=>ok(1),quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>time,sleep:async()=>{time+=p.roundMilliseconds;},rounds:p.aliveRounds*2});
  expect(lines).toEqual([wantedTextV1("alive","en",{requesters:2,held:0,cross:0,left:0,reading:"none"}),wantedTextV1("alive","en",{requesters:2,held:0,cross:0,left:0,reading:"none"})]);
 }finally{await server.stop(true);}
},60000);

const range=(from:number,to:number)=>Array.from({length:to-from},(_,at)=>"p"+String(from+at).padStart(4,"0"));

test("Law: two requests overlapping by 400 projects are served by one read: the union is inferred once, the 400 both ask for come first in the priority, the read starts once, both requesters get the progress record and the complete answer",async()=>{
 const {server,client}=await world();
 try{
  const a=client("a"),b=client("b"),serving=client("serving"),steer=temp("steer"),reads:string[]=[],lines:string[]=[],A=range(0,600),B=range(200,800),environment={SEMIO_GRAPH_STEER_DIRECTORY:steer};
  let time=1_000_000_000,release!:(value:WantedInferV1)=>void,round=0,priority:readonly string[]|undefined;
  await publishWantedV1({client:a,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:A,now:()=>time});
  await publishWantedV1({client:b,requester:"g1-m07",platform:PLATFORM,graphKey:KEY,projects:B,now:()=>time});
  const steps:(()=>void)[]=[()=>{},()=>{priority=readPriorityV1(environment)?.projects;},()=>{release(ok(4));}];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06","g1-m07"],headroom:roomy,
   infer:reason=>{reads.push(reason);return new Promise<WantedInferV1>(resolve=>{release=resolve;});},quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>time,sleep:async()=>{steps[round++]?.();},rounds:5});
  expect(reads).toEqual(["request"]);
  expect(priority).toHaveLength(800);expect(new Set(priority).size).toBe(800);
  expect(new Set(priority!.slice(0,400))).toEqual(new Set(range(200,600)));
  for(const [requester,list] of [["g1-m06",A],["g1-m07",B]] as const){
   const answer=await readServedV1({client:a,requester});
   expect(answer?.projects).toEqual(list);expect(answer?.generation).toBe(4);expect(answer?.total).toBe(600);expect(answer?.remainingMilliseconds).toBe(0);expect(answer?.progressAtMs).toBeGreaterThan(0);
  }
  expect(lines.filter(line=>line.startsWith("graph-server: serving"))).toEqual([wantedTextV1("serving","en",{count:600,requester:"g1-m06"}),wantedTextV1("serving","en",{count:600,requester:"g1-m07"})]);
 }finally{await server.stop(true);}
},60000);

test("Law: a request is idempotent: the requester renews it (same creation time, later expiry, fewer projects) and it outlives its first expiry with its claim renewed; asking again as new work joins the held request instead of serving again, and the read is never started twice",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),steer=temp("steer"),reads:string[]=[],lines:string[]=[],renews:string[]=[],p=wantedPolicyV1,names=range(0,6),environment={SEMIO_GRAPH_STEER_DIRECTORY:steer};
  const spy:GraphFleetClientV1={...serving,renew:async(key,owner,ttl)=>{renews.push(key);return serving.renew(key,owner,ttl);}};
  let time=1_000_000_000,release!:(value:WantedInferV1)=>void,round=0,first=0;
  const MINUTE=60_000,ask=(projects:readonly string[],createdMs?:number)=>publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects,...createdMs===undefined?{}:{createdMs},now:()=>time});
  await ask(names);first=time;
  const steps:(()=>Promise<unknown>|void)[]=[
   ()=>{},
   ()=>{writePendingV1(environment,5,["p0003","p0004","p0005"],readPriorityV1(environment));},
   async()=>{time+=12*MINUTE;await ask(["p0003","p0004","p0005"],first);},
   async()=>{time+=12*MINUTE;await ask(["p0003","p0004","p0005"],first);},
   async()=>{time+=MINUTE;await ask(["p0004","p0005"]);},
   ()=>{},
   ()=>{release(ok(7));},
  ];
  await runWantedServerV1({client:spy,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,
   infer:reason=>{reads.push(reason);return new Promise<WantedInferV1>(resolve=>{release=resolve;});},quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>time,sleep:async()=>{await steps[round++]?.();},rounds:9});
  expect(time-first).toBeGreaterThan(p.ttlMilliseconds);
  expect(reads).toEqual(["request"]);
  expect(renews.length).toBeGreaterThanOrEqual(2);
  expect(lines.filter(line=>line.startsWith("graph-server: serving"))).toHaveLength(1);
  expect(lines).toContain(wantedTextV1("joined","en",{count:2,arrived:3,requester:"g1-m06"}));
  const last=await readServedV1({client:requester,requester:"g1-m06"});
  expect(last?.generation).toBe(7);expect(last?.total).toBe(2);expect(last?.projects).toEqual(["p0004","p0005"]);
 }finally{await server.stop(true);}
},60000);

test("Law: the progress record is honest: it advances when a read starts or a wave publishes, stands still while the server only repeats its sign of life, and the estimate comes from the measured table until the read has finished projects",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),steer=temp("steer"),names=range(0,6),environment={SEMIO_GRAPH_STEER_DIRECTORY:steer},MINUTE=60_000,rows:{round:number;atMs:number;progressAtMs:number|undefined;remaining:number|null|undefined}[]=[];
  let table=emptyOwnerCostTableV1();for(const name of names)table=recordOwnerCostV1(table,{name,kind:"project",megabytes:500,seconds:96,rows:1,stopped:false,atMs:1});
  let time=1_000_000_000,release!:(value:WantedInferV1)=>void,round=0;
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:()=>time});
  const started=time;
  const steps:(()=>void)[]=[()=>{},()=>{time+=MINUTE;},()=>{time+=MINUTE;},()=>{time+=MINUTE;writePendingV1(environment,5,["p0003","p0004","p0005"],readPriorityV1(environment));},()=>{time+=MINUTE;},()=>{release(ok(7));}];
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,costs:()=>table,
   infer:()=>new Promise<WantedInferV1>(resolve=>{release=resolve;}),quietMilliseconds:async()=>10**9,say:()=>{},signal:new AbortController().signal,now:()=>time,
   sleep:async()=>{const served=await readServedV1({client:requester,requester:"g1-m06"});rows.push({round,atMs:served!.atMs,progressAtMs:served?.progressAtMs,remaining:served?.remainingMilliseconds});steps[round++]?.();},rounds:7});
  expect(rows[0]).toMatchObject({atMs:started,progressAtMs:started,remaining:12_000});
  expect(rows[1]!.progressAtMs).toBe(started);expect(rows[2]!.progressAtMs).toBe(started);expect(rows[2]!.atMs).toBeGreaterThan(started);
  const moved=rows.find(row=>row.progressAtMs!==started)!;
  expect(moved.progressAtMs).toBe(started+3*MINUTE);expect(moved.remaining).toBe(Math.round(3*(3*MINUTE)/3));
  const done=await readServedV1({client:requester,requester:"g1-m06"});
  expect(done?.generation).toBe(7);expect(done?.remainingMilliseconds).toBe(0);expect(done?.total).toBe(6);
 }finally{await server.stop(true);}
},60000);

const ownersLine=(running:number,done:number,elapsed:number)=>`[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:owners:${done}:5101:${running}:${elapsed}:framework-os-kernel|language%3Acargo|${elapsed}|62,repo|project|${elapsed}|45:waiting|5608|1","mode":"shared","elapsedMs":${elapsed}}}\n`;

test("R57 law: a read that finishes owners but publishes no wave for longer than the stall limit is alive; a read whose counters stand still is stopped",()=>{
 const policy={...wantedServerPolicyV1,inferStallMilliseconds:25*60000,inferMaximumMilliseconds:12*3600000};let time=0;
 const watch=createGraphReadWatchV1(policy,()=>time);
 for(let minute=1;minute<=60;minute++){time=minute*60000;watch.feed(ownersLine(15-(minute%3),minute*60,time));expect(watch.verdict(),"minute "+minute).toBe("go");}
 expect(watch.generation()).toBeNull();expect(watch.counters()["build:graph-phase:owners"]).toEqual([3600,5101]);
 const frozen=time;
 for(let minute=1;minute<=24;minute++){time=frozen+minute*60000;watch.feed(ownersLine(9,3600,time));expect(watch.verdict(),"a counter that went back or stood still is no progress, minute "+minute).toBe("go");}
 time=frozen+25*60000;watch.feed(ownersLine(15,3600,time));expect(watch.verdict()).toBe("stalled");
 watch.feed(ownersLine(15,3601,time));expect(watch.verdict(),"one more owner finished").toBe("go");
 watch.feed(`[script] {"activity":{"resource":"build:graph-phase:files:10:20"}}\n`);expect(watch.counters()["build:graph-phase:files"]).toEqual([10,20]);
});

test("R57 law: the line a stop writes names the verdict, the silence and the counters, and equals the schema",()=>{
 const policy={...wantedServerPolicyV1,inferStallMilliseconds:1000};let time=0;const watch=createGraphReadWatchV1(policy,()=>time);
 watch.feed(ownersLine(15,885,1));time=1500;
 const row=graphReadStopRowV1(watch,"stalled",11,12,7);
 expect(row).toEqual({version:1,verdict:"stalled",atMs:7,silentMilliseconds:1500,generation:null,counters:{"build:graph-phase:owners":[885,5101]},runner:11,read:12});
 const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:false}).compile(schema.definitions.ReadStop)(row)).toBe(true);
});

const sharedLine=(phase:string,shared:number,elapsed:number)=>`[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:${phase}:0:5189:1:${elapsed}:shared%20closures%20${shared}%2F49","mode":"shared","elapsedMs":0}}\n`;

test("Law: the shared closures a first wave builds in process one by one keep the read alive; the same closure count repeated, or going back, does not, and a long first wave is not stopped",()=>{
 const policy={...wantedServerPolicyV1,inferStallMilliseconds:25*60000,inferMaximumMilliseconds:12*3600000};let time=0;
 const watch=createGraphReadWatchV1(policy,()=>time);
 for(const phase of["inference","owners"]){
  for(let closure=1;closure<=49;closure++){time+=90_000;watch.feed(sharedLine(phase,closure,closure*90));expect(watch.verdict(),phase+" closure "+closure).toBe("go");}
  time+=24*60000;watch.feed(sharedLine(phase,49,4500));expect(watch.verdict()).toBe("go");
  time+=60000;watch.feed(sharedLine(phase,49,4600));expect(watch.verdict(),"no further closure for the stall limit").toBe("stalled");
  watch.feed(sharedLine(phase,3,4700));expect(watch.verdict(),"a count that went back is no progress").toBe("stalled");
  const fresh=createGraphReadWatchV1(policy,()=>time);time+=0;fresh.feed(sharedLine(phase,1,10));
  expect(fresh.verdict()).toBe("go");
  time+=25*60000;expect(fresh.verdict(),"stalled after one closure and the limit").toBe("stalled");
  time+=0;fresh.feed(sharedLine(phase,2,20));expect(fresh.verdict()).toBe("go");
 }
});
