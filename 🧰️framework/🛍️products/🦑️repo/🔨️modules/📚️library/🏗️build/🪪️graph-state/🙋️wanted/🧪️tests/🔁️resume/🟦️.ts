/** 🔁️ Contract: the graph server that is restarted for new code continues what it served. The claims, the requests it holds and the projects it already told the requesters about are kept in a state file; the new process renews the same claims, answers at once with what was published (never with less), and its graph read is told which projects are published already. */
import Ajv from "ajv";
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {remoteCacheFetchV1} from "../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {graphFleetClientV1} from "../../../🛰️exchange/🟦️.ts";
import {graphFleetSettingsV1} from "../../../🧶️nx/🟦️.ts";
import {publishWantedV1,readServeStateV1,readServedV1,wantedTextV1} from "../../🟦️.ts";
import {readPriorityV1,writePendingV1} from "../../../../🕸️graph/🕰️generations/🧭️steering/🟨️.mjs";
import {runWantedServerV1,serveStateFileV1,type WantedInferV1} from "../../🛠️server/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-resume-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLATFORM="darwin-arm64",KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const});
const names=Array.from({length:6},(_,index)=>"p"+index);

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}

async function firstProcess(input:Readonly<{client:ReturnType<Awaited<ReturnType<typeof world>>["client"]>;requester:ReturnType<Awaited<ReturnType<typeof world>>["client"]>;steer:string;statePath:string;time:()=>number;lines:string[]}>){
 const controller=new AbortController(),steer={SEMIO_GRAPH_STEER_DIRECTORY:input.steer};let round=0;
 const steps:(()=>Promise<void>|void)[]=[
  async()=>{await publishWantedV1({client:input.requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:input.time});},
  ()=>{writePendingV1(steer,5,["p3","p4","p5"],readPriorityV1(steer));},
  ()=>{},
  ()=>{controller.abort();},
 ];
 await runWantedServerV1({client:input.client,owner:"g1-server",platform:PLATFORM,steerDirectory:input.steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,state:serveStateFileV1(input.statePath),
  infer:(_reason,signal)=>new Promise<WantedInferV1>(resolve=>{signal?.addEventListener("abort",()=>resolve({ok:false,status:130,seconds:1,generation:null,detail:"stopped"}),{once:true});}),
  quietMilliseconds:async()=>10**9,say:line=>input.lines.push(line),signal:controller.signal,now:input.time,sleep:async()=>{await steps[round++]?.();}});
}

test("a restart during a serve continues the claim: the new process renews it, answers at once with what was published, hands its read the published projects and never infers one of them again",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),again=client("serving-again"),steer=temp("steer"),statePath=join(temp("state"),"serve-state.json");let clock=1_000_000_000;const time=()=>clock;
  const first:string[]=[];await firstProcess({client:serving,requester,steer,statePath,time,lines:first});
  expect(first).toContain(wantedTextV1("serving","en",{count:6,requester:"g1-m06"}));
  expect((await readServedV1({client:requester,requester:"g1-m06"}))?.projects,"the first process told the requester about three projects").toEqual(["p0","p1","p2"]);
  const saved=JSON.parse(readFileSync(statePath,"utf8"));
  expect(saved.owner).toBe("g1-server");expect(saved.held.length).toBe(1);expect(saved.held[0].list).toEqual(["p0","p1","p2"]);expect(saved.reading).toBe("none");
  const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/ServeState",definitions:schema.definitions});
  expect(validate(saved),JSON.stringify(validate.errors)).toBe(true);expect(readServeStateV1(saved)).toBeDefined();

  clock+=60_000;
  const lines:string[]=[],reads:{reason:string;published:readonly string[]|undefined;inferred:string[]}[]=[],answers:(readonly string[]|undefined)[]=[];let ticks=0;
  await runWantedServerV1({client:again,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,state:serveStateFileV1(statePath),
   infer:async(reason,_signal,published)=>{const inferred=names.filter(name=>!(published??[]).includes(name));reads.push({reason,published,inferred});return{ok:true,status:0,seconds:3,generation:8,detail:""};},
   quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:new AbortController().signal,now:time,
   sleep:async()=>{answers.push((await readServedV1({client:requester,requester:"g1-m06"}))?.projects);ticks++;},rounds:3});
  expect(lines[0]).toBe(wantedTextV1("resumed","en",{held:1,fresh:3}));
  expect(lines.some(line=>line.includes("graph-server: serving")),"no new claim announced: the held one continues").toBe(false);
  expect(lines.some(line=>line===wantedTextV1("lost","en",{requester:"g1-m06",by:"g1-m09"})),"the claim was renewed, not lost").toBe(false);
  expect(reads.length,"one read").toBe(1);expect(reads[0]!.reason).toBe("request");expect(reads[0]!.published).toEqual(["p0","p1","p2"]);expect(reads[0]!.inferred,"only the projects that were not published are inferred").toEqual(["p3","p4","p5"]);
  for(const [index,answer] of answers.entries())expect(["p0","p1","p2"].every(name=>answer?.includes(name)),`answer ${index} never holds less than what was published`).toBe(true);
  const last=await readServedV1({client:requester,requester:"g1-m06"});
  expect(last?.projects).toEqual(names);expect(last?.generation).toBe(8);
  expect(ticks).toBe(3);
  expect(JSON.parse(readFileSync(statePath,"utf8")).held,"nothing is held after the answer").toEqual([]);
  expect(JSON.parse(readFileSync(statePath,"utf8")).told.length).toBe(1);
 }finally{await server.stop();}
},90000);

test("without the state file the same restart tells the requester nothing is fresh and asks the read to start from zero (control for the test above)",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),again=client("serving-again"),steer=temp("steer"),statePath=join(temp("state"),"serve-state.json");let clock=1_000_000_000;const time=()=>clock;
  await firstProcess({client:serving,requester,steer,statePath,time,lines:[]});
  rmSync(statePath,{force:true});clock+=60_000;
  const published:(readonly string[]|undefined)[]=[],answers:(readonly string[]|undefined)[]=[];
  await runWantedServerV1({client:again,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,state:serveStateFileV1(statePath),
   infer:async(_reason,_signal,list)=>{published.push(list);return{ok:true,status:0,seconds:3,generation:8,detail:""};},
   quietMilliseconds:async()=>10**9,say:()=>{},signal:new AbortController().signal,now:time,
   sleep:async()=>{answers.push((await readServedV1({client:requester,requester:"g1-m06"}))?.projects);},rounds:2});
  expect(published[0]).toEqual([]);expect(answers[0],"the requester is told nothing is fresh any more").toEqual([]);
 }finally{await server.stop();}
},90000);

test("a damaged state file, the state of another owner and an expired request are not resumed, and the file is replaced atomically",async()=>{
 const {server,client}=await world();
 try{
  const path=join(temp("state"),"serve-state.json"),store=serveStateFileV1(path),serving=client("serving"),lines:string[]=[];
  const request={version:1 as const,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,createdMs:1,expiresMs:1000};
  const state={version:1 as const,owner:"g1-server",platform:PLATFORM,savedMs:5,held:[{request,list:["p0"],told:true,toldAt:2,heldAt:1}],told:["k".repeat(64)],reading:"request" as const};
  const run=(owner:string)=>runWantedServerV1({client:serving,owner,platform:PLATFORM,steerDirectory:temp("steer"),language:"en",requesters:async()=>[],headroom:async()=>({freeMegabytes:0,pressure:"normal"}),state:store,infer:async()=>({ok:true,status:0,seconds:1,generation:1,detail:""}),quietMilliseconds:async()=>0,say:line=>lines.push(line),signal:new AbortController().signal,now:()=>5_000,sleep:async()=>{},rounds:1});
  store.save(state);expect(store.load()).toEqual(state);expect(existsSync(path+"."+process.pid+".tmp")).toBe(false);
  await run("g1-other");expect(lines,"another owner's state is not ours").toEqual([]);
  store.save(state);await run("g1-server");expect(lines,"the request expired at 1 000 ms: nothing to resume").toEqual([]);
  store.save({...state,held:[{...state.held[0]!,request:{...request,expiresMs:10_000}}]});await run("g1-server");expect(lines[0],"a live request of our own state is resumed").toBe(wantedTextV1("resumed","en",{held:1,fresh:1}));expect(lines[1],"a claim that nobody holds any more is taken again at once, the request is still wanted").toBe(wantedTextV1("reclaimed","en",{requester:"g1-m06"}));
  writeFileSync(path,"{damaged");expect(store.load()).toBeUndefined();
  writeFileSync(path,JSON.stringify({...state,version:2}));expect(store.load()).toBeUndefined();
  const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/ServeState",definitions:schema.definitions});
  expect(validate({...state,reading:"sometimes"})).toBe(false);expect(validate({...state,held:[{...state.held[0],list:[1]}]})).toBe(false);expect(validate(state)).toBe(true);
 }finally{await server.stop();}
},60000);

test("every text exists in both languages with the same placeholders",()=>{
 const en=schema.definitions.Texts.const.en as Record<string,string>,de=schema.definitions.Texts.const.de as Record<string,string>;
 expect(Object.keys(en).sort()).toEqual(Object.keys(de).sort());
 for(const key of Object.keys(en))expect([...en[key]!.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort(),key).toEqual([...de[key]!.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort());
 expect(wantedTextV1("resumed","de",{held:2,fresh:5})).toContain("2");
});
