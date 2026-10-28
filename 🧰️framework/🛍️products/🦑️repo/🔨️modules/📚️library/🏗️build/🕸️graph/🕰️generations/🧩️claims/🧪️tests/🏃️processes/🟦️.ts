import{expect,test}from"bun:test";
import{appendFileSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{acquireQueuedResourceLease}from"@semio-tech/process/leases";
import{fileURLToPath}from"node:url";

const worker=join(dirname(fileURLToPath(import.meta.url)),"📥️worker/🟦️.ts");
type Row={name:string;pid:number;event:string;owners?:string[];computed?:number;served?:boolean};
const rows=(log:string):Row[]=>readFileSync(log,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line));
const launch=(dir:string,log:string,name:string,wanted:string[],extra:Record<string,string>={})=>Bun.spawn([process.execPath,worker],{env:{...process.env,CLAIMS_DIR:dir,CLAIMS_LOG:log,CLAIMS_NAME:name,CLAIMS_WANTED:JSON.stringify(wanted),...extra},stdout:"ignore",stderr:"pipe"});
const until=async(check:()=>boolean,milliseconds:number)=>{const end=Date.now()+milliseconds;while(Date.now()<end){if(check())return true;await new Promise(resolve=>setTimeout(resolve,25));}return false;};

for(const lease of["file","queued"])test("three real processes with overlapping stale owners compute every owner exactly once ("+lease+" leases)",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"claims-proc-")),log=join(dir,"log.jsonl"),names=Array.from({length:60},(_,at)=>"p"+String(at).padStart(2,"0"));
 writeFileSync(log,"");
 try{
  const children=[launch(dir,log,"A",names.slice(0,40),{CLAIMS_LEASE:lease,CLAIMS_RUN_MS:"60"}),launch(dir,log,"B",names.slice(20),{CLAIMS_LEASE:lease,CLAIMS_RUN_MS:"60"}),launch(dir,log,"C",names,{CLAIMS_LEASE:lease,CLAIMS_RUN_MS:"60"})];
  for(const child of children)expect(await child.exited,await new Response(child.stderr).text()).toBe(0);
  const all=rows(log),runs=all.filter(row=>row.event==="run").flatMap(row=>row.owners!);
  expect(runs.sort()).toEqual(names);expect(new Set(runs).size).toBe(runs.length);
  expect(all.filter(row=>row.event==="end")).toHaveLength(3);
  expect(all.filter(row=>row.event==="end").reduce((sum,row)=>sum+row.computed!,0)).toBe(60);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

for(const lease of["file","queued"])test("a joiner takes over the chunk of a process killed with SIGKILL ("+lease+" leases)",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"claims-kill-")),log=join(dir,"log.jsonl"),names=["k1","k2","k3","k4","k5"];
 writeFileSync(log,"");
 try{
  const victim=launch(dir,log,"X",names,{CLAIMS_LEASE:lease,CLAIMS_MODE:"hang"});
  expect(await until(()=>rows(log).some(row=>row.event==="hang"),30000)).toBe(true);
  const joiner=launch(dir,log,"Y",names,{CLAIMS_LEASE:lease,CLAIMS_RUN_MS:"20"});
  await new Promise(resolve=>setTimeout(resolve,400));
  expect(rows(log).filter(row=>row.name==="Y"&&row.event==="run")).toHaveLength(0);
  victim.kill("SIGKILL");await victim.exited;
  expect(await joiner.exited,await new Response(joiner.stderr).text()).toBe(0);
  const mine=rows(log).filter(row=>row.name==="Y"&&row.event==="run").flatMap(row=>row.owners!);
  expect(mine.sort()).toEqual(names);
  const end=rows(log).find(row=>row.name==="Y"&&row.event==="end")!;expect(end.computed).toBe(5);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

for(const lease of["file","queued"])test("a process with stale owners nobody else needs does not wait for a long running peer ("+lease+" leases)",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"claims-disjoint-")),log=join(dir,"log.jsonl");
 writeFileSync(log,"");
 try{
  const slow=launch(dir,log,"S",["s1","s2"],{CLAIMS_LEASE:lease,CLAIMS_MODE:"hang"});
  expect(await until(()=>rows(log).some(row=>row.event==="hang"),30000)).toBe(true);
  const quick=launch(dir,log,"Q",["q1","q2"],{CLAIMS_LEASE:lease,CLAIMS_RUN_MS:"20"});
  expect(await quick.exited,await new Response(quick.stderr).text()).toBe(0);
  expect(rows(log).some(row=>row.name==="Q"&&row.event==="end")).toBe(true);
  slow.kill("SIGKILL");await slow.exited;
  appendFileSync(log,"");
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

const holder=join(dirname(fileURLToPath(import.meta.url)),"📥️holder/🟦️.ts");
const hold=(dir:string,log:string,name:string,cohort:boolean)=>Bun.spawn([process.execPath,holder],{env:{...process.env,HOLD_DIR:dir,HOLD_LOG:log,HOLD_NAME:name,HOLD_RESOURCE:"repository-graph-facts:/w:shard:7",HOLD_COHORT:cohort?"1":"0",HOLD_MS:"1200"},stdout:"ignore",stderr:"pipe"});
const events=(log:string)=>readFileSync(log,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as{name:string;event:string;at:number});

test("shared cohort holders of the namespace writer coexist and keep an exclusive sweep away, plain shared tickets queue behind each other",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"claims-cohort-")),resource="repository-graph-facts:/w:shard:7";
 try{
  for(const cohort of[true,false]){
   const log=join(dir,cohort?"c.jsonl":"p.jsonl");writeFileSync(log,"");
   const first=hold(dir,log,"one",cohort);expect(await until(()=>events(log).some(row=>row.name==="one"&&row.event==="held"),20000)).toBe(true);
   const second=hold(dir,log,"two",cohort);
   await new Promise(resolve=>setTimeout(resolve,500));
   const overlapping=events(log).some(row=>row.name==="two"&&row.event==="held");
   expect(overlapping,cohort?"cohort tickets coexist":"a plain shared ticket waits for the held ticket before it").toBe(cohort);
   await expect(acquireQueuedResourceLease({directory:dir,resource,mode:"exclusive",owner:"sweep",timeoutMs:0,signal:new AbortController().signal})).rejects.toThrow("timed out");
   expect(await first.exited).toBe(0);expect(await second.exited).toBe(0);
   const rows=events(log),held=rows.filter(row=>row.event==="held").map(row=>row.at),released=rows.filter(row=>row.event==="released").map(row=>row.at);
   expect(Math.max(...held)<Math.min(...released)).toBe(cohort);
  }
  const sweep=await acquireQueuedResourceLease({directory:dir,resource,mode:"exclusive",owner:"sweep",timeoutMs:0,signal:new AbortController().signal});sweep.release();
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);
