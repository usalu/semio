import{afterAll,expect,test}from"bun:test";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_TREE_POLICY_V1 as TREE,readyCodeStopV1}from"../../🌲️process-tree/🟦️.ts";
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,runReadyStageV1}from"../../🟦️.ts";

/** 🪢️ Contract: a landing never costs a running stage its progress. A restart of the supervisor detaches and a new one adopts the same process; new code stops a stage only when it waits for a holder or has gone quiet while it only reads, otherwise the new code applies to the next attempt.
 * @see ../../🌲️process-tree/🟦️.ts */
const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`codestop-${label}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const schemaFile=join(import.meta.dir,"..","..","🌲️process-tree","🧬️schema","🔣️.json");
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stageOf=(name:string,timeoutMilliseconds:number)=>({...STAGES.find(stage=>stage.name===name)!,timeoutMilliseconds});
const moving=(steps:number)=>`#!/bin/sh\ni=0\nwhile [ $i -lt ${steps} ]; do i=$((i+1)); echo "Reading the project graph slice ($i s): taken $i of 4455."; sleep 0.3; done\nexit 0\n`;
const standing=`#!/bin/sh\necho "Reading the project graph slice (1 s): taken 1 of 4455."\nsleep 40\n`;
const policy={...TREE,staleStageMilliseconds:1500,codeQuietMilliseconds:900,codeCheckMilliseconds:300,semanticsSources:[{file:"code/semantics.json",pointer:"/revision"}],holderWaitRestartMilliseconds:600000,stallStopMilliseconds:600000};
const declareSemantics=(root:string,revision:number)=>{mkdirSync(join(root,"code"),{recursive:true});writeFileSync(join(root,"code","semantics.json"),JSON.stringify({revision}));};
const start=(home:string,root:string,signal:AbortSignal,onStage?:(stage:string,adopted:boolean)=>void)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools",600000),signal,jobs:1,onLine:()=>undefined,tree:policy,...onStage===undefined?{}:{onStage}});

test("the new code stops a stage only for a holder wait or a quiet read, and equals an independent Python implementation on a grid",()=>{
 const holds=[0,TREE.holderWaitRestartMilliseconds-1,TREE.holderWaitRestartMilliseconds,10*TREE.holderWaitRestartMilliseconds],ages=[0,TREE.staleStageMilliseconds-1,TREE.staleStageMilliseconds,10*TREE.staleStageMilliseconds],quiets=[0,TREE.codeQuietMilliseconds-1,TREE.codeQuietMilliseconds,10*TREE.codeQuietMilliseconds];
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),schemaFile,JSON.stringify(holds),JSON.stringify(ages),JSON.stringify(quiets)],{encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 const rows=JSON.parse(python.stdout) as [number,"reading"|"executing"|null,number,number,"holder"|"stale"|null][];
 expect(rows.length).toBe(4*3*4*4);
 for(const[holderWaitMs,phase,phaseAgeMs,quietMs,expected]of rows)expect(readyCodeStopV1({holderWaitMs,phase,phaseAgeMs,quietMs}),JSON.stringify([holderWaitMs,phase,phaseAgeMs,quietMs])).toBe(expected);
 expect(TREE.codeQuietMilliseconds).toBe(300000);
 expect(rows.some(row=>row[4]==="holder")&&rows.some(row=>row[4]==="stale")&&rows.some(row=>row[4]===null)).toBe(true);
});

posix("law: a read that keeps moving is not stopped when the code changes after the stale age, it finishes and the new code applies to the next attempt",async()=>{
 const root=temp("moving"),home=fakeHome(moving(22));declareSemantics(root,1);
 const pending=start(home,root,new AbortController().signal);
 setTimeout(()=>declareSemantics(root,2),700);
 const result=await pending;
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.codeChanged).toBeUndefined();expect(result.codeStale).toBeUndefined();
},60000);

posix("a read that went quiet is stopped as stale when the code changed, and not before it was quiet for the window",async()=>{
 const root=temp("quiet"),home=fakeHome(standing);declareSemantics(root,1);
 const began=Date.now(),pending=start(home,root,new AbortController().signal);
 setTimeout(()=>declareSemantics(root,2),400);
 const result=await pending;
 expect(result.stalled).toBe(true);expect(result.codeStale).toBe(true);expect(result.codeChanged).toBe("code/semantics.json");
 expect(Date.now()-began).toBeGreaterThanOrEqual(policy.codeQuietMilliseconds);expect(Date.now()-began).toBeLessThan(25000);
},60000);

posix("law: a supervisor restart in the middle of a read that then sees new code adopts the same stage process, which finishes the attempt",async()=>{
 const root=temp("restart"),home=fakeHome(moving(30));declareSemantics(root,1);
 const first=new AbortController(),running=start(home,root,first.signal),file=join(root,...P.directory,P.inflightName);
 for(let waited=0;!existsSync(file)&&waited<5000;waited+=25)await new Promise<void>(done=>setTimeout(done,25));
 const before=JSON.parse(readFileSync(file,"utf8")) as {pid:number;startedMs:number};
 await new Promise<void>(done=>setTimeout(done,500));
 first.abort();
 expect((await running).detached).toBe(true);
 declareSemantics(root,2);
 const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
 expect(alive(before.pid),"the stage survives the supervisor").toBe(true);
 const transitions:[string,boolean][]=[];
 const second=await start(home,root,new AbortController().signal,(stage,adopted)=>transitions.push([stage,adopted]));
 expect(transitions[0]).toEqual(["tools",true]);
 expect(second.startedMs).toBe(before.startedMs);
 expect(second.exit).toBe(0);expect(second.stalled).toBe(false);expect(second.codeChanged).toBeUndefined();expect(second.detached).toBe(false);
},60000);
