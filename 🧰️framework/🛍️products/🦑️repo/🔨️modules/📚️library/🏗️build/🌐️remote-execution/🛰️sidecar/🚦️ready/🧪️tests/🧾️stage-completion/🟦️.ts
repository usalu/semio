import {expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {createReadyPortsV1,emptyReadyStateV1,READY_POLICY_V1 as policy,runReadyStageV1,runReadyV1,type ReadyInflightV1,type ReadyStageV1} from "../../🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};
import terminalCases from "./📈️timings.json" with {type:"json"};

const posix=test.skipIf(process.platform==="win32");
const makeStage=(name:string,files:readonly string[]=[]):ReadyStageV1=>({name,kind:"repo",arguments:[name],timeoutMilliseconds:60000,inputs:files,bootstrapInput:false});
const stages=[makeStage("repo-mcp"),makeStage("tools"),makeStage("workspace",["workspace-input"])];
const wait=async(check:()=>boolean)=>{
 const deadline=performance.now()+5000;
 while(!check()&&performance.now()<deadline)await Bun.sleep(25);
 expect(check()).toBe(true);
};

async function fixture(exit:number,mirrored=false){
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"c-")),root=join(base,"r"),home=join(base,"h"),ready=join(root,...policy.directory),running=join(ready,policy.inflightName),state=join(ready,policy.stateName),repo=join(home,".local","bin","repo"),runs=join(root,"runs"),stop=join(root,"exit"),input=join(root,"workspace-input");
 mkdirSync(root);mkdirSync(join(home,".local","bin"),{recursive:true});writeFileSync(input,"original");
 writeFileSync(repo,'#!/bin/sh\necho "$*" >> runs\nif [ "$1" = workspace ]; then while [ ! -f exit ]; do sleep 0.05; done; exit '+exit+'; fi\nexit 0\n');chmodSync(repo,0o755);
 const store=join(base,"s"),ports=(plan:readonly ReadyStageV1[]=stages)=>createReadyPortsV1({root,home,environment:mirrored?{SEMIO_FLEET_STORE_ROOT:store}:{},origin:"fixture",group:undefined,stages:plan,say:()=>{}});
 const original=ports(),digests=(await original.digests()).stages;
 original.state.write({...emptyReadyStateV1(),stages:Object.fromEntries(digests.slice(0,-1).map((row,index)=>[row.name,{digest:row.digest,doneMs:index+1}]))});
 const launch=async(bench:ReturnType<typeof ports>,stage:ReadyStageV1,signal=new AbortController().signal)=>{
  expect(bench.lease.acquire()).toBe("own");
  try{return await bench.run(stage,{signal,jobs:1,onLine:()=>{}});}
  finally{bench.lease.release();}
 };
 const detach=async(legacy=false)=>{
  const controller=new AbortController();
  const pending=legacy?runReadyStageV1({root,home,environment:{},stage:stages[2]!,jobs:1,onLine:()=>{},signal:controller.signal}):launch(original,stages[2]!,controller.signal);
  await wait(()=>existsSync(running)&&existsSync(runs));
  const row=JSON.parse(readFileSync(running,"utf8")) as ReadyInflightV1;
  controller.abort();
  expect((await pending).detached).toBe(true);
  writeFileSync(stop,"");
  await wait(()=>existsSync(row.exitFile));
  return row;
 };
 return {root,state,input,runs,running,stop,ready,store,original,ports,launch,detach,close:()=>rmSync(base,{recursive:true,force:true})};
}

posix("a different requested stage reconciles only proven exits of the detached physical stage",async()=>{
 const observations=[];
 for(const row of cases){
  const f=await fixture(row.exit);
  try{
   const prior=await f.detach(row.change==="legacy");
   const plan=row.change==="recipe"?[...stages.slice(0,-1),{...stages[2]!,arguments:["workspace","--changed"]}]:row.change==="order"?[stages[1]!,stages[0]!,stages[2]!]:stages;
   if(row.change==="prerequisite"){
    const state=f.original.state.read();
    f.original.state.write({...state,stages:{...state.stages,tools:{...state.stages.tools!,doneMs:999}}});
   }
   if(row.change==="digest")writeFileSync(f.input,"changed");
   const result=await f.launch(f.ports(plan),plan[0]!);
   const physical=f.original.state.read().stages.workspace;
   observations.push([row.name,result.attribution??null,physical!==undefined]);
   expect(result.stage).toBe("workspace");expect(result.exit).toBe(row.exit);expect(result.attribution??null).toBe(row.result);
   expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
   expect(existsSync(prior.exitFile)).toBe(false);expect(existsSync(f.running)).toBe(false);
   if(physical!==undefined)expect(physical.digest).toBe((await f.original.digests()).stages[2]!.digest);
  }finally{f.close();}
 }
 const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(oracle.status).toBe(0);
 expect(observations).toEqual(oracle.stdout.trim().split("\n").map(text=>JSON.parse(text)));
},60000);

posix("a failed state write preserves the raw receipt and replaying an applied receipt never applies it twice",async()=>{
 const f=await fixture(0);
 try{
  const prior=await f.detach(),launchText=readFileSync(f.running,"utf8"),exitText=readFileSync(prior.exitFile,"utf8"),before=readFileSync(f.state,"utf8");
  const temporary=f.state+"."+process.pid+".tmp";
  mkdirSync(temporary);
  await expect(f.launch(f.ports(),stages[0]!)).rejects.toThrow();
  expect(JSON.parse(readFileSync(f.running,"utf8"))).toMatchObject({launch:prior.launch,pid:prior.pid,startedMs:prior.startedMs,exitFile:prior.exitFile});
  expect(readFileSync(prior.exitFile,"utf8")).toBe(exitText);
  expect(readFileSync(f.state,"utf8")).toBe(before);
  rmSync(temporary,{recursive:true});
  writeFileSync(temporary,'{"version":1,"stages":');
  expect((await f.launch(f.ports(),stages[0]!)).attribution).toBe("applied");
  const after=readFileSync(f.state,"utf8");
  writeFileSync(f.running,launchText);writeFileSync(prior.exitFile,exitText);
  expect((await f.launch(f.ports(),stages[0]!)).attribution).toBe("known");
  expect(readFileSync(f.state,"utf8")).toBe(after);
  expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
 }finally{f.close();}
},30000);

posix("a zero exit cannot credit a stage after a restart from durable cancellation",async()=>{
 const f=await fixture(0);
 try{
  await f.detach();
  const row=JSON.parse(readFileSync(f.running,"utf8")) as ReadyInflightV1;
  writeFileSync(f.running,JSON.stringify({...row,stop:"cancel"}));
  const result=await f.launch(f.ports(),stages[0]!);
  expect(result).toMatchObject({stage:"workspace",exit:0,cancelled:true});
  expect(f.original.state.read().stages.workspace).toBeUndefined();
  expect(existsSync(f.running)).toBe(false);
  expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
 }finally{f.close();}
},30000);

posix("the normal readiness loop consumes a fresh physical completion and becomes ready",async()=>{
 const f=await fixture(0),abort=new AbortController(),last=f.ports();
 try{
  writeFileSync(f.stop,"");
  await runReadyV1({...last,disk:()=>null,diskUnknown:()=>[],memory:()=>({ok:true,availableMegabytes:4096}),
   status:{update(patch){if(patch.ready===true)abort.abort();}},
   sleep:async()=>{if(!abort.signal.aborted)throw Error("fixture never waits");}},abort.signal);
  const state=last.state.read();
  expect(abort.signal.aborted).toBe(true);expect(last.marker.exists()).toBe(true);
  expect(state.stages.workspace?.digest).toBe((await last.digests()).stages[2]!.digest);
  expect(state.appliedCompletions?.workspace).toMatch(/^[a-f0-9]{64}$/u);
  expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
 }finally{f.close();}
},30000);

posix("a launch whose proof cannot be published never starts the physical command",async()=>{
 const f=await fixture(0);
 try{
  const temporary=f.running+"."+process.pid+".tmp";
  mkdirSync(temporary);writeFileSync(f.stop,"");
  await expect(f.launch(f.original,stages[2]!)).rejects.toThrow();
  expect(existsSync(f.runs)).toBe(false);expect(existsSync(f.running)).toBe(false);
  rmSync(temporary,{recursive:true});
  expect((await f.launch(f.original,stages[2]!)).attribution).toBe("applied");
  expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
 }finally{f.close();}
},30000);


posix("the readiness loop records the adopted physical terminal timing before considering other work",async()=>{
 for(const row of terminalCases){
  const f=await fixture(row.exit,true),abort=new AbortController(),last=f.ports();
  try{
   const prior=await f.detach(row.legacy),launch=JSON.parse(readFileSync(f.running,"utf8")) as ReadyInflightV1;
   const ended=JSON.parse(readFileSync(prior.exitFile,"utf8")).endedMs as number;
   if(!row.receipt){
    await wait(()=>{try{process.kill(prior.pid,0);return false;}catch{return true;}});
    rmSync(prior.exitFile);
   }
   expect(launch.watchdog).toBeDefined();
   writeFileSync(f.running,JSON.stringify({...launch,watchdog:{...launch.watchdog,peakMegabytes:row.peak},...row.stop===null?{}:{stop:row.stop}}));
   last.state.write(emptyReadyStateV1());
   let requested=0;
   await runReadyV1({...last,disk:()=>null,diskUnknown:()=>[],memory:()=>({ok:true,availableMegabytes:4096}),
    run(stage,options){expect(stage.name).toBe("repo-mcp");if(++requested===2)abort.abort();return last.run(stage,options);},
    sleep:async()=>{if(!abort.signal.aborted)throw Error("fixture never waits");}},abort.signal);
   expect(requested).toBe(2);
   expect(last.state.read().stages).toEqual({});
   expect(last.state.read().failure).toBeNull();
   const local=join(f.ready,policy.timingsName),remote=join(f.store,"v1","origins","fixture",policy.timingsName);
   expect(existsSync(local)).toBe(true);
   expect(existsSync(remote)).toBe(true);
   const document=JSON.parse(readFileSync(local,"utf8")),digest=prior.launch?.digest.slice(0,12)??null;
   expect(JSON.parse(readFileSync(remote,"utf8"))).toEqual(document);
   const timedEnd=row.receipt?ended:document.runs[0].endedMs;
   if(!row.receipt){expect(timedEnd).toBeGreaterThanOrEqual(ended);expect(timedEnd).toBeLessThanOrEqual(Date.now());}
   const oracle=spawnSync("python3",["-I","-c","import json,sys; r,s,e,d=json.load(sys.stdin); print(json.dumps(dict(stage='workspace',startedMs=s,endedMs=e,seconds=(e-s+500)//1000,exit=r['exit'] if r['receipt'] else None,outcome=r['outcome'],peakMegabytes=r['peak'],growthMegabytes=None,attempts=None,digest=d,endEvidence='exit-receipt' if r['receipt'] else 'observed',signal=None)))"],{input:JSON.stringify([row,prior.startedMs,timedEnd,digest]),encoding:"utf8"});
   expect(oracle.status).toBe(0);
   expect(document.runs).toEqual([JSON.parse(oracle.stdout)]);
   expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
  }finally{f.close();}
 }
},60000);

posix("same-stage failure timing uses the adopted launch identity rather than newly planned inputs",async()=>{
 for(const legacy of [false,true]){
  const f=await fixture(14,true),abort=new AbortController(),ports=f.ports();
  try{
   const prior=await f.detach(legacy),ended=JSON.parse(readFileSync(prior.exitFile,"utf8")).endedMs as number;
   writeFileSync(f.input,"planned replacement");
   const proposed=(await ports.digests()).stages[2]!.digest.slice(0,12),physical=prior.launch?.digest.slice(0,12)??null;
   expect(proposed).not.toBe(physical);
   let runs=0;
   await runReadyV1({...ports,disk:()=>null,diskUnknown:()=>[],memory:()=>({ok:true,availableMegabytes:4096}),
    run(stage,options){expect(stage.name).toBe("workspace");expect(++runs).toBe(1);return ports.run(stage,options);},
    sleep:async()=>{abort.abort();}},abort.signal);
   expect(runs).toBe(1);
   const document=JSON.parse(readFileSync(join(f.ready,policy.timingsName),"utf8"));
   const oracle=spawnSync("python3",["-I","-c","import json,sys; s,e,d=json.load(sys.stdin); print(json.dumps(dict(stage='workspace',startedMs=s,endedMs=e,seconds=(e-s+500)//1000,exit=14,outcome='stage-failed',digest=d,endEvidence='exit-receipt',signal=None)))"],{input:JSON.stringify([prior.startedMs,ended,physical]),encoding:"utf8"});
   expect(oracle.status).toBe(0);
   expect(document.runs).toHaveLength(1);
   expect(document.runs[0]).toMatchObject(JSON.parse(oracle.stdout));
   expect(readFileSync(f.runs,"utf8")).toBe("workspace\n");
  }finally{f.close();}
 }
},30000);
