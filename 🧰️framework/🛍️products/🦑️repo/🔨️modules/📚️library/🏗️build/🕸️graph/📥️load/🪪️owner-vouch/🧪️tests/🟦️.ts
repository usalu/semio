/** 🪪️ Contract: a worker killed in the middle of a read repeats no completed owner check: the owners it recorded stay vouched, a path row that the journal reports as touched since the cursor of the ledger is probed again, and nothing is vouched without an exact journal, for another key or across a gap. */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {hash} from "node:crypto";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {spawnSync} from "node:child_process";
import {openOwnerVouchV1,vouchedOwnerV1} from "../🟦️.ts";
import type {JournalFeedV1,JournalRowV1} from "../../🗞️journal-clean/🟦️.ts";
import fixture from "./🔣️.json";

type ChangeV1=readonly [string,0|1,string];
const feedOf=(rows:readonly ChangeV1[]|"refuse",epoch="e1"):JournalFeedV1=>({excluder:()=>()=>false,cursor:async()=>({epoch,seq:10}),since:async()=>rows==="refuse"?{exact:false}:{exact:true,cursor:{epoch,seq:12},rows}});
const changesOf=(rows:readonly(readonly(string|number)[])[]):ChangeV1[]=>rows.map(([kind,flag,path])=>{if(typeof kind!=="string"||(flag!==0&&flag!==1)||typeof path!=="string")throw new Error("malformed change row");return[kind,flag,path] as const;});
const digest=(name:string)=>Bun.hash(name).toString(16).padStart(64,"0");
const rowsOf=(root:string,name:keyof typeof fixture.owners):JournalRowV1[]=>fixture.owners[name].map(([kind,path])=>({kind,path:kind==="env"?path:root+"/"+path,fact:path.length}));
const clock=()=>{let at=0;return{now:()=>at,tick:(milliseconds:number)=>{at+=milliseconds;}};};
const sandbox=()=>{const root=mkdtempSync(join(tmpdir(),"owner-vouch-"));for(const name of ["pkg/a","pkg/b","pkg/c"])mkdirSync(join(root,name),{recursive:true});return{root,file:join(root,"ledger.json"),done:()=>rmSync(root,{recursive:true,force:true})};};
const names=Object.keys(fixture.owners) as (keyof typeof fixture.owners)[];

test("the restarted worker skips exactly the path rows no change touched, equal to an independent CPython model",async()=>{
 const box=sandbox();
 try{
  for(const scenario of fixture.scenarios){
   const first=await openOwnerVouchV1({feed:feedOf([]),root:box.root,file:box.file,key:"k",insensitive:false});
   for(const name of names)first.record(digest(name));
   first.flush();
   const again=await openOwnerVouchV1({feed:feedOf(changesOf(scenario.changes)),root:box.root,file:box.file,key:"k",insensitive:false});
   const actual:Record<string,boolean[]>={};
   for(const name of names){const rows=rowsOf(box.root,name),clean=await again.cleanFor(digest(name),rows);actual[name]=rows.map(row=>clean?.(row)??false);}
   const model=spawnSync("python3",["-I","-c",`
import json,sys
d=json.load(sys.stdin);ch=d["changes"];root=d["root"]+"/"
kinds={"file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames"}
def touched(p):
    for k,dr,c in ch:
        if c==p or c.startswith(p+"/") or p.startswith(c+"/") and not (dr==1 and k=="m"): return True
    return False
print(json.dumps({n:[r["kind"] in kinds and r["path"].startswith(root) and not touched(r["path"][len(root):]) for r in rows] for n,rows in d["rows"].items()}))
`],{input:JSON.stringify({changes:scenario.changes,root:box.root,rows:Object.fromEntries(names.map(name=>[name,rowsOf(box.root,name)]))}),encoding:"utf8"});
   expect(model.status).toBe(0);
   expect(actual,scenario.name).toEqual(JSON.parse(model.stdout));
   expect(actual,scenario.name).toEqual(fixture.scenarios.find(item=>item===scenario)!.expected);
  }
 }finally{box.done();}
});

test("a worker killed after a flush keeps the owners flushed and loses at most the unflushed ones",async()=>{
 const box=sandbox(),time=clock();
 try{
  const first=await openOwnerVouchV1({feed:feedOf([]),root:box.root,file:box.file,key:"k",insensitive:false,now:time.now});
  first.record(digest("a"));time.tick(15000);first.record(digest("b"));time.tick(1);first.record(digest("c"));
  const second=await openOwnerVouchV1({feed:feedOf([]),root:box.root,file:box.file,key:"k",insensitive:false});
  expect(await second.cleanFor(digest("a"),rowsOf(box.root,"a"))).toBeDefined();
  expect(await second.cleanFor(digest("b"),rowsOf(box.root,"b"))).toBeDefined();
  expect(await second.cleanFor(digest("c"),rowsOf(box.root,"c"))).toBeUndefined();
  expect(second.counts().vouched).toBe(2);
 }finally{box.done();}
});

test("nothing is vouched for another key, without an exact journal or across an epoch gap, and clear removes the ledger",async()=>{
 const box=sandbox();
 try{
  const first=await openOwnerVouchV1({feed:feedOf([]),root:box.root,file:box.file,key:"k",insensitive:false});
  first.record(digest("a"));first.flush();
  for(const reopened of [{feed:feedOf([]),key:"other"},{feed:feedOf("refuse"),key:"k"}]){
   const again=await openOwnerVouchV1({root:box.root,file:box.file,insensitive:false,...reopened});
   expect(await again.cleanFor(digest("a"),rowsOf(box.root,"a")),JSON.stringify(reopened.key)).toBeUndefined();
  }
  first.clear();expect(existsSync(box.file)).toBe(false);
  const empty=await openOwnerVouchV1({feed:feedOf([]),root:box.root,file:box.file,key:"k",insensitive:false});
  expect(empty.counts().vouched).toBe(0);
 }finally{box.done();}
});

test("a real worker killed after its first completed owners leaves the ledger and the checkpoint, and the restarted worker serves them from the checkpoint with no row probed",async()=>{
 const box=sandbox(),directory=join(box.root,"oc"),{observeOwnerFactsMemoizedV1}=await import("../../🟦️.ts"),{openOwnerCheckpointV1}=await import("../../💾️owner-checkpoint/🟦️.ts"),support=await import("./🧰️support/🟦️.ts");
 try{
  for(let index=0;index<3;index++)mkdirSync(join(box.root,"pkg"+index),{recursive:true});
  mkdirSync(directory,{recursive:true});
  const child=Bun.spawn([process.execPath,fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),directory,box.root,box.file],{stdout:"pipe",stderr:"inherit"});
  try{const stream=child.stdout.getReader();try{expect(new TextDecoder().decode((await stream.read()).value)).toBe("ready\n");}finally{stream.releaseLock();}}finally{child.kill("SIGKILL");await child.exited;}
  expect(existsSync(box.file)).toBe(true);
  const checkpoint=await openOwnerCheckpointV1(directory,support.CHECKPOINT_SCOPE,support.CHECKPOINT_FORMAT,{check:()=>{},yieldContinuation:async()=>{},publish:(_paths,_plan,write)=>{write();return true;}});
  const vouch=await openOwnerVouchV1({feed:support.feedOf([]),root:box.root,file:box.file,key:"k",insensitive:false}),outcomes:string[]=[];let probed=0;
  for(const [index,question] of support.ownerQuestions.slice(0,3).entries()){
   const suffix=question.owner+"\0"+JSON.stringify([question.targets,question.inputs]),vouched=vouchedOwnerV1(vouch,hash("sha256",suffix,"hex"),async(rows,clean)=>{probed+=rows.filter(row=>clean?.(row)!==true).length;return true;});
   await observeOwnerFactsMemoizedV1({key:"scope2\0"+suffix,hold:rows=>vouched.hold(rows as never),observation:{captureObservedSourceInputsV1:async()=>{throw new Error("must not rediscover");},retainObservedRowsV1:()=>{}} as never,observe:async()=>{throw new Error("must not rediscover");},recover:()=>checkpoint.read(suffix),persist:()=>undefined,outcome:kind=>{outcomes.push(kind);}});
   void index;
  }
  expect(outcomes).toEqual(["checkpoint","checkpoint","checkpoint"]);
  expect(vouch.counts().vouched).toBeGreaterThanOrEqual(2);
  expect(probed).toBeLessThan(6);
 }finally{box.done();}
});
