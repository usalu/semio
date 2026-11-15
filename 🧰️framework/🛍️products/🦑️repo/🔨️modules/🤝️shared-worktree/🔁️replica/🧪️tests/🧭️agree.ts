import {expect,setDefaultTimeout,test} from "bun:test";
import {mkdirSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {formatHlcV1} from "../../📒️journal/🟦️.ts";
import {gitStateV1,gitViewDeltaV1,gitViewV1,type GitViewV1} from "../../🌿️git/🟦️.ts";
import {bucketOfV1,createAttesterV1,fleetReportV1,type AttesterV1,type FleetReportV1} from "../../🩺️attest/🟦️.ts";
import {storeReachableV1} from "../🟦️.ts";
import {buildFleetV1,disposeFleetV1,gitV1,performV1,rngV1,roundV1,type ChaosFleetV1,type ChaosMachineV1,type RngV1} from "./🌪️harness.ts";

setDefaultTimeout(240_000);

const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const where=(who:ChaosMachineV1,path:string):string=>join(who.options.worktree,...path.split("/"));

type Churn=Readonly<{fleet:ChaosFleetV1;attesters:readonly AttesterV1[];unsettled:readonly Set<string>[];files:readonly Set<string>[];rng:RngV1}>;

async function churning(count:number,seed:number):Promise<Churn>{
 const fleet=await buildFleetV1(count),unsettled=fleet.machines.map(()=>new Set<string>()),files=fleet.machines.map(()=>new Set<string>());
 const attesters=fleet.machines.map((who,index)=>createAttesterV1({
  storeRoot:fleet.real,originId:who.name,groupId:"g1",now:Date.now,heartbeatMilliseconds:20,viewMilliseconds:200,viewDebounceMilliseconds:20,deltaDebounceMilliseconds:10,
  git:signal=>gitStateV1(who.options.worktree,signal),
  view:async(signal,request)=>{
   if(request!==undefined){const next=await gitViewDeltaV1(who.options.worktree,request.previous as GitViewV1,{paths:request.paths,restage:request.restage},signal);if(next!==null)return next;}
   return gitViewV1(who.options.worktree,signal);
  },
  changes:()=>{const dirt=who.replica!.viewDirt,taken={paths:[...dirt.paths],overflow:dirt.overflow};dirt.paths.clear();dirt.overflow=false;return taken;},
  unsettled:()=>[...unsettled[index]!],
  reachable:()=>storeReachableV1(fleet.real)
 }));
 for(const attester of attesters)await attester.start();
 return {fleet,attesters,unsettled,files,rng:rngV1(seed)};
}

async function tickAll(churn:Churn):Promise<void>{
 for(const [index,who] of churn.fleet.machines.entries()){
  const replica=who.replica;
  if(replica===undefined)continue;
  await churn.attesters[index]!.tick({sync:true,files:replica.files.shared,manifest:replica.files.manifestHex(),buckets:replica.files.bucketsHex(),cursors:replica.cursors,incoming:replica.queue.size,busy:replica.applying>0,hlc:formatHlcV1(replica.clock,who.name),activity:replica.activity});
 }
}

function edit(churn:Churn,index:number,step:number):void{
 const who=churn.fleet.machines[index]!,names=[...churn.files[index]!],kind=churn.rng.int(10),note=(path:string):void=>{churn.unsettled[index]!.add(path);who.replica!.viewDirt.paths.add(path);};
 if(kind<5||names.length===0){const path=`churn/${who.name}/n${step}.txt`;mkdirSync(dirname(where(who,path)),{recursive:true});writeFileSync(where(who,path),`x${churn.rng.hex(12)}\n`);churn.files[index]!.add(path);note(path);}
 else if(kind<8){const path=churn.rng.pick(names);writeFileSync(where(who,path),`y${churn.rng.hex(12)}\n`);note(path);}
 else{const path=churn.rng.pick(names);rmSync(where(who,path),{force:true});churn.files[index]!.delete(path);note(path);}
}

async function rounds(churn:Churn,indexes:readonly number[]):Promise<void>{
 for(const index of indexes){await roundV1(churn.fleet.machines[index]!);churn.unsettled[index]!.clear();}
}

const moving=(report:FleetReportV1):boolean=>report.rows.some(row=>(row.behindOrigins??0)>0||(row.incoming??0)>0);

async function drive(churn:Churn,steps:number,observe:(report:FleetReportV1,step:number)=>void,skip:readonly number[]=[],from:number=0):Promise<void>{
 const count=churn.fleet.machines.length;
 for(let step=from;step<from+steps;step++){
  for(let index=0;index<count;index++)if(!skip.includes(index))edit(churn,index,step);
  for(const index of skip)churn.fleet.machines[index]!.replica!.activity++;
  const subset=Array.from({length:count},(_,index)=>index).filter(index=>!skip.includes(index)&&(index===step%count||churn.rng.chance(0.5)));
  await rounds(churn,subset);
  await tickAll(churn);
  await pause(25);
  observe(await fleetReportV1(churn.fleet.real),step);
 }
}

posixTest("four replicas under continuous edits on every machine agree at common checkpoints without any idle moment, and never report a divergence",async()=>{
 const churn=await churning(4,20260513);
 const agreed:number[]=[],diverged:string[]=[];
 try{
  await drive(churn,60,(report,step)=>{
   if(step>=6&&report.ok&&moving(report)&&report.rows.every(row=>row.basis==="checkpoint"))agreed.push(step);
   for(const row of report.rows)if(row.verdict==="diverged")diverged.push(`${step} ${row.originId} ${row.differs.join(",")} ${JSON.stringify(report.differing.filter(item=>item.originId===row.originId)).slice(0,300)}`);
  });
 }finally{await disposeFleetV1(churn.fleet);}
 expect(diverged).toEqual([]);
 expect(agreed.length).toBeGreaterThanOrEqual(3);
});

posixTest("a working-tree file changed on disk behind the daemon is reported as diverged in the view at a common checkpoint while the edits continue",async()=>{
 const churn=await churning(4,777);
 const victim=2;
 try{
  const before:string[]=[];
  await drive(churn,16,(report,step)=>{for(const row of report.rows)if(row.verdict==="diverged")before.push(`${step} ${row.originId}`);});
  expect(before).toEqual([]);
  const who=churn.fleet.machines[victim]!;
  expect(await performV1(churn.fleet,{op:"stale",m:victim,path:"a.txt",text:"ALPHA"})).toBe("ok");
  const named:string[]=[];
  await drive(churn,24,(report)=>{
   const row=report.rows.find(item=>item.originId===who.name)!;
   if(row.verdict==="diverged")named.push(row.differs.join(","));
   for(const other of report.rows)if(other.originId!==who.name&&other.verdict==="diverged"){named.push(`wrong:${other.originId}`);console.log("[DEBUG]",JSON.stringify(report.differing),JSON.stringify(report.rows.map(r=>[r.originId,r.verdict])));}
  },[victim],16);
  expect(named.length).toBeGreaterThan(0);
  expect(named.filter(item=>item.startsWith("wrong:"))).toEqual([]);
  expect(new Set(named)).toEqual(new Set(["view.digest"]));
 }finally{await disposeFleetV1(churn.fleet);}
});

posixTest("one event dropped from the shared files of a replica is reported as diverged in the manifest at a common checkpoint while the edits continue",async()=>{
 const churn=await churning(4,4242);
 const victim=1;
 try{
  const before:string[]=[];
  await drive(churn,16,(report,step)=>{for(const row of report.rows)if(row.verdict==="diverged")before.push(`${step} ${row.originId}`);});
  expect(before).toEqual([]);
  const who=churn.fleet.machines[victim]!,replica=who.replica!,id=replica.book.find("b.txt")!;
  expect(replica.files.clearId(id)).toBe(true);
  const named:string[]=[];
  await drive(churn,24,(report)=>{
   const row=report.rows.find(item=>item.originId===who.name)!;
   if(row.verdict==="diverged")named.push(row.differs.join(","));
   for(const other of report.rows)if(other.originId!==who.name&&other.verdict==="diverged"){named.push(`wrong:${other.originId}`);if(!(globalThis as any).__dbg){(globalThis as any).__dbg=1;for(const m of churn.fleet.machines){const r=m.replica!;const inb=[...r.files.entries()].filter(([path])=>bucketOfV1(path)===35).map(([path,e])=>`${path}:${(e as any).sha256?.slice(0,6)}:${(e as any).hlc}`);console.log("[DEBUG]",m.name,r.files.manifestHex().slice(0,8),JSON.stringify(inb),JSON.stringify(r.cursors),r.queue.size);}}}
  },[victim],16);
  expect(named.length).toBeGreaterThan(0);
  expect(named.filter(item=>item.startsWith("wrong:"))).toEqual([]);
  expect(new Set(named)).toEqual(new Set(["manifest"]));
 }finally{await disposeFleetV1(churn.fleet);}
});

posixTest("commits and stage changes among the edits make the git state undecided for a moment and never a divergence",async()=>{
 const churn=await churning(3,99);
 const diverged:string[]=[];
 try{
  for(let round=0;round<4;round++){
   const who=churn.fleet.machines[round%3]!;
   gitV1(who,["add","--","churn"]);
   churn.unsettled[round%3]!.add(".git/index");
   gitV1(who,["commit","-q","--allow-empty","-m",`c${round}`]);
   churn.unsettled[round%3]!.add(".git/refs/heads/main");
   await drive(churn,10,(report,step)=>{for(const row of report.rows)if(row.verdict==="diverged")diverged.push(`${step} ${row.originId} ${row.differs.join(",")}`);},[],round*10);
  }
 }finally{await disposeFleetV1(churn.fleet);}
 expect(diverged).toEqual([]);
});
