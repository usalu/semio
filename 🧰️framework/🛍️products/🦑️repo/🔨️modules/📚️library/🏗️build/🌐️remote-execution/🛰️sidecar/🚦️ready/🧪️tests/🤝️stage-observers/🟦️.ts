import {expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {READY_POLICY_V1 as policy,READY_STAGES_V1 as stages,runReadyStageV1} from "../../🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

const posix=test.skipIf(process.platform==="win32");
const pause=(ms:number)=>new Promise<void>(done=>setTimeout(done,ms));
const waitFor=async(ready:()=>boolean)=>{
 const stop=performance.now()+5000;
 while(!ready()&&performance.now()<stop)await pause(25);
 expect(ready()).toBe(true);
};
const command=(file:string,arg:string)=>spawnSync("ps",["-p",arg,"-o",file+"="],{encoding:"utf8"}).stdout.trim();
const runningChild=(pid:number,script:string)=>command("command",String(pid)).includes(script)&&!command("stat",String(pid)).startsWith("Z");
const fixture=()=>{
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"s-")),root=join(base,"r"),home=join(base,"h"),ready=join(root,...policy.directory),starts=join(root,"started"),repo=join(home,".local","bin","repo");
 mkdirSync(root);mkdirSync(join(home,".local","bin"),{recursive:true});
 const write=(body:string)=>{writeFileSync(repo,"#!/bin/sh\n"+body);chmodSync(repo,0o755);};
 const launch=(stage:string,signal=new AbortController().signal,lines:string[]=[])=>runReadyStageV1({root,home,environment:{},stage:{...stages.find(row=>row.name===stage)!,timeoutMilliseconds:12000},signal,jobs:1,onLine:line=>lines.push(line)});
 const seen=(expected:readonly string[])=>{
  const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),starts],{encoding:"utf8"});
  expect(oracle.status).toBe(0);expect<readonly string[]>(readFileSync(starts,"utf8").trimEnd().split("\n")).toEqual(expected);expect(JSON.parse(oracle.stdout)).toEqual(expected);
 };
 return{base,root,ready,starts,write,launch,seen,close:()=>rmSync(base,{recursive:true,force:true})};
};

posix("two concurrent launches of one stage get the same exit and run one command",async()=>{
 const f=fixture();
 try{
  f.write('echo tools >> started\nsleep 1\nexit 0\n');
  const [first,second]=await Promise.all([f.launch("tools"),f.launch("tools")]);
  f.seen(corpus.simultaneous);expect(first.exit).toBe(0);expect(second).toEqual(first);
 }finally{f.close();}
},30000);

posix("a detached observer leaves the stage running for the other observers and the next stage runs once",async()=>{
 const f=fixture(),owner=new AbortController(),withdraw=new AbortController(),lines:string[]=[];
 try{
  f.write('if [ "$1" = run-many ]; then echo workspace >> started; exit 0; fi\necho tools >> started\necho shared-progress\nsleep 4\n');
  const origin=f.launch("tools",owner.signal);
  await waitFor(()=>existsSync(f.starts));
  const surviving=f.launch("tools",new AbortController().signal,lines),next=f.launch("workspace"),cancelled=f.launch("workspace",withdraw.signal);
  owner.abort();withdraw.abort();
  expect((await origin).detached).toBe(true);expect((await cancelled).detached).toBe(true);
  const [tools,workspace]=await Promise.all([surviving,next]);
  expect(tools.exit).toBe(0);expect(workspace.exit).toBe(0);expect(lines).toContain("shared-progress");f.seen(corpus.turnover);
 }finally{f.close();}
},30000);

posix("a cancellation receipt settles a SIGTERM-resistant child before the stage returns",async()=>{
 const f=fixture(),child=join(f.base,"resistant.js"),pidFile=join(f.root,"pid");let pid:number|undefined;
 try{
  writeFileSync(child,'process.on("SIGTERM",()=>{});require("fs").writeFileSync(process.argv[2],String(process.pid));setInterval(()=>{},1000);');
  f.write('"' + process.execPath + '" "' + child + '" "' + pidFile + '" &\nwait\n');
  const stage=f.launch("tools");
  await waitFor(()=>existsSync(pidFile));pid=Number(readFileSync(pidFile,"utf8"));
  expect(runningChild(pid,child)).toBe(true);
  writeFileSync(join(f.ready,policy.cancelName),"");
  const result=await stage;
  expect(result.cancelled).toBe(true);expect(runningChild(pid,child)).toBe(false);
 }finally{
  if(pid!==undefined&&runningChild(pid,child))try{process.kill(pid,"SIGKILL");}catch{}
  f.close();
 }
},35000);

posix("settling an older receipt leaves a replacement generation's inflight record and tree",async()=>{
 const f=fixture();
 try{
  f.write('echo tools > started\nsleep 2\nexit 0\n');
  const stage=f.launch("tools"),file=join(f.ready,policy.inflightName);
  await waitFor(()=>existsSync(f.starts)&&existsSync(file));
  const prior=JSON.parse(readFileSync(file,"utf8")),replacement={...prior,pid:prior.pid+1000,startedMs:prior.startedMs+1};
  const tree=join(f.ready,"tools.tree.json");
  writeFileSync(file,JSON.stringify(replacement));writeFileSync(tree,"replacement");
  expect((await stage).exit).toBe(0);
  expect(JSON.parse(readFileSync(file,"utf8"))).toEqual(replacement);expect(readFileSync(tree,"utf8")).toBe("replacement");
 }finally{f.close();}
},30000);
