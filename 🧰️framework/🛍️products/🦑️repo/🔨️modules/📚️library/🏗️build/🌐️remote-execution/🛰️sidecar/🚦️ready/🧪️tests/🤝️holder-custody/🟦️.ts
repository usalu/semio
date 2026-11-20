import{afterAll,expect,test}from"bun:test";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{spawn,type ChildProcess}from"node:child_process";
import{machinePathV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{READY_TREE_POLICY_V1 as TREE}from"../../🌲️process-tree/🟦️.ts";
import{READY_STAGES_V1 as STAGES,readyForegroundBusyOfStoreV1,readyRefusalV1,runReadyStageV1}from"../../🟦️.ts";

/** 🤝️ A stage that waits behind a named holder of the dependency store, with real processes on a scratch state root: a live holder that renews its registry entry is no stall, a silent one is named, a dead one counts as reclaimed, and a background graph read of this machine gives way to the stage while a holder of anyone else never does. */
const scratch:string[]=[],holders:ChildProcess[]=[];
afterAll(async()=>{
 await Promise.all(holders.map(child=>child.exitCode!==null||child.signalCode!==null?Promise.resolve():new Promise<void>(done=>{child.once("exit",()=>done());child.kill("SIGKILL");})));
 for(const path of scratch)rmSync(path,{recursive:true,force:true});
});
const temp=(label:string)=>{const path=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),`${label[0]}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stageOf=(name:string,timeoutMilliseconds:number)=>({...STAGES.find(stage=>stage.name===name)!,timeoutMilliseconds});
const HOLDER=`const fs=require("node:fs");const [folder,record,stale]=process.argv.slice(2);const startedAtMs=Math.round(performance.timeOrigin);const entry={version:1,pid:process.pid,startedAtMs,updatedAtMs:Date.now()-(stale==="stale"?1000000:0),command:"741d306890f3eaea75560cfde56699ec57f6b657c06db92f1e0d550e400c18ef show projects",directory:folder+"/l",holds:[{resource:"native-fixed-image:x",mode:"shared",sinceMs:Date.now(),task:1}],waits:[]};fs.mkdirSync(folder,{recursive:true});const file=folder+"/"+process.pid+"-"+startedAtMs+".json",save=()=>{fs.writeFileSync(file+".tmp",JSON.stringify(entry));fs.renameSync(file+".tmp",file);};save();if(record&&record!=="none"){fs.mkdirSync(require("node:path").dirname(record),{recursive:true});fs.writeFileSync(record,JSON.stringify({version:1,reason:"refresh",runner:process.pid,read:process.pid,startedMs:startedAtMs,beatMs:Date.now(),log:"x",status:null,endedMs:null}));}setInterval(()=>{if(stale!=="stale"){entry.updatedAtMs=Date.now();save();}},250);`;
const holderStart=async(registry:string,record:string,stale=false):Promise<number>=>{
 const file=join(temp("holderjs"),"holder.js");writeFileSync(file,HOLDER);
 const child=spawn(process.execPath,[file,join(registry,"aaaa"),record,stale?"stale":"fresh"],{detached:true,stdio:"ignore"});holders.push(child);
 for(let waited=0;waited<5000;waited+=50){
  const folder=join(registry,"aaaa");
  if(existsSync(folder)&&readdirSync(folder).some(name=>name.startsWith(child.pid+"-")&&name.endsWith(".json"))&&(record==="none"||existsSync(record)))return child.pid!;
  if(child.exitCode!==null||child.signalCode!==null)break;
  await new Promise<void>(done=>setTimeout(done,50));
 }
 throw Error("Holder did not publish its fixture registry and read record");
};
const lives=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch{return false;}};
const goneWithin=async(pid:number,milliseconds:number):Promise<boolean>=>{for(let waited=0;waited<milliseconds;waited+=50){if(!lives(pid))return true;await new Promise<void>(done=>setTimeout(done,50));}return false;};
const waiting=(pid:number,iterations:number,reclaimAfter?:number)=>`#!/bin/sh\ni=0\nwhile [ $i -lt ${iterations} ]; do i=$((i+1)); echo "Process ${pid} (741d306890f3eaea75560cfde56699ec57f6b657c06db92f1e0d550e400c18ef show projects) holds the dependency store since 2026-11-18T03:28:01.557Z; waiting for it ($i s of at most 21600 s)"; ${reclaimAfter===undefined?"":`if [ "$i" -ge ${reclaimAfter} ] && ! kill -0 ${pid} 2>/dev/null; then echo Lease reclaimed; exit 0; fi; `}sleep 0.3; done\nexit 0\n`;
const policy={...TREE,stallStopMilliseconds:2500,holderCheckMilliseconds:300,holderWaitRestartMilliseconds:600000};
const run=(home:string,root:string,registry:string,said:string[]=[],tree=policy)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:line=>said.push(line),tree,orphans:{registryRoot:registry,windows:()=>[],human:()=>null}});

posix("a stage that waits behind a live holder that renews its entry is not stalled although nothing new is printed for longer than the stall window",async()=>{
 const registry=temp("registry"),root=temp("root"),pid=await holderStart(registry,"none");
 const began=Date.now(),result=await run(fakeHome(waiting(pid,40)),root,registry);
 expect(result.stalled,"the old rule stops this stage after 2.5 s").toBe(false);expect(result.exit).toBe(0);expect(Date.now()-began).toBeGreaterThan(9000);expect(lives(pid),"a holder of someone else is never touched").toBe(true);
},60000);

posix("a stage that waits behind a holder silent for more than the heartbeat bound is stopped by the stall rule and the holder is named in the result",async()=>{
 const registry=temp("registry"),root=temp("root"),pid=await holderStart(registry,"none",true);
 const result=await run(fakeHome(waiting(pid,60)),root,registry);
 expect(result.stalled).toBe(true);expect(result.holderSilent).toMatchObject({pid});expect(result.holderSilent!.seconds).toBeGreaterThanOrEqual(900);expect(lives(pid)).toBe(true);
 const refusal=readyRefusalV1("stage-holder-silent",{stage:"tools",minutes:10,pid,command:result.holderSilent!.command,silent:result.holderSilent!.seconds,seconds:30,count:1,maximum:5});
 expect(refusal.en).toContain(`process ${pid}`);expect(refusal.de).toContain(`Prozess ${pid}`);for(const text of[refusal.en,refusal.de])expect(text).not.toContain("{");
},60000);

posix("a stage that waits for a pid whose process is gone is reclaimed by the waiter, no stall",async()=>{
 const registry=temp("registry"),root=temp("root"),pid=await holderStart(registry,"none");process.kill(pid,"SIGKILL");expect(await goneWithin(pid,3000)).toBe(true);
 const result=await run(fakeHome(waiting(pid,40,10)),root,registry);
 expect(result.stalled).toBe(false);expect(result.exit).toBe(0);expect(result.tail).toContain("Lease reclaimed");
},60000);

posix("a background graph read of this machine gives way to the stage that waits behind it (said in both languages, recorded with its reason), and the same holder without a read record of the graph server is left alone",async()=>{
 const registry=temp("registry"),root=temp("root"),record=machinePathV1(root,...TREE.readRecord),pid=await holderStart(registry,record),said:string[]=[];
 const result=await run(fakeHome(waiting(pid,60,10)),root,registry,said,{...policy,holderYieldMilliseconds:1500});
 expect(await goneWithin(pid,6000),"the read gave way").toBe(true);expect(result.stalled).toBe(false);expect(result.tail).toContain("Lease reclaimed");
 const line=said.find(text=>text.includes("gives way to the stage"))!;expect(line,said.join("\n")).toContain(`process ${pid}`);expect(line).toContain(" / Stufe tools wartete ");
 const kills=readFileSync(join(root,".cache","f","ready","kills.log"),"utf8");expect(kills).toContain(`Process ${pid}`);expect(kills).toContain("background graph read");expect(kills).toContain("Hintergrund-Graphlesen");
 const registry2=temp("registry"),root2=temp("root"),other=await holderStart(registry2,"none");
 const second=await run(fakeHome(waiting(other,40)),root2,registry2,[],{...policy,holderYieldMilliseconds:1500});
 expect(second.stalled).toBe(false);expect(lives(other),"no record of the graph server: a holder of a person or another agent is no victim").toBe(true);
},90000);

posix("an eligible holder can yield on the first recognized named wait",async()=>{
 const registry=temp("registry"),root=temp("root"),record=machinePathV1(root,...TREE.readRecord),pid=await holderStart(registry,record);
 const observed:{wait?:number;yielded?:number}={};
 const result=await runReadyStageV1({home:fakeHome(waiting(pid,40,10)),root,environment:{},stage:stageOf("tools",60000),signal:new AbortController().signal,jobs:1,
  onLine:line=>{if(line.includes("holds the dependency store since"))observed.wait??=performance.now();if(line.includes("gives way to the stage"))observed.yielded??=performance.now();},
  tree:{...policy,holderYieldMilliseconds:0},orphans:{registryRoot:registry,windows:()=>[],human:()=>null}});
 expect(result.stalled).toBe(false);expect(result.tail).toContain("Lease reclaimed");
 expect(observed.wait).toBeDefined();expect(observed.yielded).toBeDefined();
 expect(observed.yielded!-observed.wait!).toBeLessThan(1800);
},60000);

test("a ready.json of a busy loop holds the graph server of the machine back, a ready one, a missing one and a stale one do not",()=>{
 const store=temp("store"),file=join(store,"v1","origins","g1-m05","ready.json"),now=1794975700000;mkdirSync(dirname(file),{recursive:true});
 const write=(row:object)=>writeFileSync(file,JSON.stringify(row));
 expect(readyForegroundBusyOfStoreV1(store,"g1-m05",now)).toBe(false);
 write({ready:false,state:"running",updatedMs:now-1000});expect(readyForegroundBusyOfStoreV1(store,"g1-m05",now)).toBe(true);expect(readyForegroundBusyOfStoreV1(store,"g1-m06",now)).toBe(false);expect(readyForegroundBusyOfStoreV1(undefined,"g1-m05",now)).toBe(false);
 write({ready:true,state:"ready",updatedMs:now-1000});expect(readyForegroundBusyOfStoreV1(store,"g1-m05",now)).toBe(false);
 write({ready:false,state:"waiting",updatedMs:now-TREE.foregroundFreshMilliseconds-1});expect(readyForegroundBusyOfStoreV1(store,"g1-m05",now)).toBe(false);
});

test("the graph server asks the readiness loop before it serves or refreshes (source shape)",()=>{
 const source=readFileSync(join(import.meta.dir,"..","..","..","🕸️graph","🟦️.ts"),"utf8");
 expect(source).toContain("async mayServe(){return(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes&&!readyForegroundBusyOfStoreV1(store,origin);}");
 expect(source).toContain("async mayRefresh(){if(readyForegroundBusyOfStoreV1(store,origin))return false;");
});
