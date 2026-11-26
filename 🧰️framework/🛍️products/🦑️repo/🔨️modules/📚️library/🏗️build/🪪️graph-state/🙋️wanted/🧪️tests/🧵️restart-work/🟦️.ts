/** 🧵️ Contract: while the code-change restart of the graph server waits for the serve in flight, the loop reports that it is busy, takes no new request and starts no refresh, keeps the read of the held requests, and reports the end of the read the moment it ended. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {remoteCacheFetchV1} from "../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {graphFleetClientV1} from "../../../🛰️exchange/🟦️.ts";
import {graphFleetSettingsV1} from "../../../🧶️nx/🟦️.ts";
import {publishWantedV1,readServedV1} from "../../🟦️.ts";
import {runWantedServerV1,type WantedInferV1} from "../../🛠️server/🟦️.ts";

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-work-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLATFORM="darwin-arm64",KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const});
const names=["p0","p1","p2"];

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}

test("a restart that waits: busy while the read for held requests runs, no new request, no refresh, the held request is answered, and the end of the read is reported at once",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),other=client("other"),serving=client("serving"),steer=temp("steer");let clock=1_000_000_000;const time=()=>clock;
  const lines:string[]=[],reports:{busy:boolean;round:number}[]=[],reads:string[]=[];let round=0,pending=false,release!:(result:WantedInferV1)=>void;
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:time});
  const steps:(()=>Promise<void>|void)[]=[
   ()=>{pending=true;},
   async()=>{await publishWantedV1({client:other,requester:"g1-m07",platform:PLATFORM,graphKey:KEY,projects:["q0"],now:time});},
   ()=>{},
   ()=>{release({ok:true,status:0,seconds:3,generation:5,detail:""});},
   ()=>{},
  ];
  const controller=new AbortController();
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06","g1-m07"],headroom:roomy,
   infer:(reason)=>{reads.push(reason);return new Promise<WantedInferV1>(resolve=>{release=resolve;});},
   work:{restartPending:()=>pending,report:busy=>{reports.push({busy,round});}},
   quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:controller.signal,now:time,sleep:async()=>{await steps[round++]?.();clock+=1000;},rounds:5});
  expect(reads,"one read, for the held request, no refresh").toEqual(["request"]);
  expect(lines.filter(line=>line.includes("serving")).length,"the request that arrived while the restart waits is not taken").toBe(1);
  expect(lines.some(line=>line.includes("g1-m07")&&line.includes("serving"))).toBe(false);
  expect(reports[0]).toEqual({busy:true,round:0});
  const ended=reports.findIndex(row=>!row.busy);expect(ended,"the end of the read is reported").toBeGreaterThan(0);
  expect(reports.slice(1,ended).every(row=>row.busy)).toBe(true);
  expect((await readServedV1({client:requester,requester:"g1-m06"}))?.generation).toBe(5);
 }finally{await server.stop();}
});

test("a restart that waits and nothing held: no refresh is started however quiet the tree is, and without a waiting restart it is",async()=>{
 const {server,client}=await world();
 try{
  for(const pending of [true,false]){
   const serving=client("serving"+pending),steer=temp("steer"+pending),reads:string[]=[];let clock=2_000_000_000;
   await runWantedServerV1({client:serving,owner:"g1-server"+pending,platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>[],headroom:roomy,
    infer:async reason=>{reads.push(reason);return{ok:true,status:0,seconds:1,generation:2,detail:""};},
    work:{restartPending:()=>pending,report:()=>{}},quietMilliseconds:async()=>10**9,say:()=>{},signal:new AbortController().signal,now:()=>clock,sleep:async()=>{clock+=1000;},rounds:3});
   expect(reads,`pending ${pending}`).toEqual(pending?[]:["refresh"]);
  }
 }finally{await server.stop();}
});

test("every held request expired while the read runs: nobody waits for the serve, so it is no longer reported as busy",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),steer=temp("steer");let clock=3_000_000_000;
  const reports:boolean[]=[],controller=new AbortController();let round=0;
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:()=>clock});
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,
   infer:(_reason,signal)=>new Promise<WantedInferV1>(resolve=>{signal?.addEventListener("abort",()=>resolve({ok:false,status:130,seconds:1,generation:null,detail:"stopped"}),{once:true});}),work:{restartPending:()=>false,report:busy=>{reports.push(busy);}},
   quietMilliseconds:async()=>10**9,say:()=>{},signal:controller.signal,now:()=>clock,sleep:async()=>{round++;if(round===2)clock+=24*3600*1000;if(round===4)controller.abort();},rounds:6});
  expect(reports[0],"busy while the request is held").toBe(true);
  expect(reports.at(-1),"not busy once the request expired, although the read still runs").toBe(false);
 }finally{await server.stop();}
});
