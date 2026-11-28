/** 🛑️ Contract: while the volume is short, a replica does not write a file that arrives from the store (it stays queued and the daemon says so in both languages), it still applies deletions, which free space, and it writes the file as soon as the volume has room again; the same file never appears on a short disk by a retry. The volume is injected, the files are compared with the bytes of the publisher.
 * @see ../🛑️apply-guard/🟦️.ts */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {closeReplicaV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`apply-guard-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const until=async(condition:()=>boolean,milliseconds=20000):Promise<void>=>{const end=Date.now()+milliseconds;while(!condition()){if(Date.now()>end)throw Error("Timed out");await new Promise(done=>setTimeout(done,20));}};
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};
const fast={pollMilliseconds:10,scanSeconds:3600,walkSeconds:3600,sweepMilliseconds:3_600_000,watch:false,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,viewDebounceMilliseconds:50,logFiles:[] as string[]};
const options=(name:string,store:string,extra:Partial<ReplicaOptionsV1>={}):ReplicaOptionsV1=>{const code=temp(`${name}-code`);put(code,"x/🟦️.ts","export const a=1;\n");return {worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",...fast,codeRoot:code,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),...extra};};
const GIB=2**30;

test("a short volume holds back files from the store but not deletions, and the file arrives when the volume has room",async()=>{
 const store=temp("store"),source=options("src",store);
 put(source.worktree,"seed.txt","seed\n");
 await seedV1(source);
 let free=400*GIB;
 const events:ReplicaEventV1[]=[],receiver=options("dst",store,{space:async()=>({free,total:475*GIB})}),controller=new AbortController();
 await joinV1(receiver);
 const peer=await openReplicaV1(source);
 const done=runV1(receiver,controller.signal,event=>events.push(event));
 try{
  await until(()=>events.some(event=>event.event==="started"));
  put(source.worktree,"base.txt","base\n");
  expect((await publishOnceV1(peer,undefined,["base.txt"])).published).toBe(1);
  await until(()=>existsSync(join(receiver.worktree,"base.txt")));
  free=14*GIB;
  await new Promise(resume=>setTimeout(resume,2200));
  rmSync(join(source.worktree,"base.txt"));
  put(source.worktree,"later.txt","later\n");
  expect((await publishOnceV1(peer,undefined,["base.txt","later.txt"])).published).toBeGreaterThanOrEqual(1);
  await until(()=>!existsSync(join(receiver.worktree,"base.txt")));
  await until(()=>events.some(event=>event.event==="blocked"&&event.reason==="disk-short"));
  const blocked=events.find(event=>event.event==="blocked"&&event.reason==="disk-short")!;
  expect(blocked.message).toContain("Disk short");expect(blocked.message).toContain("Platte knapp");
  await new Promise(resume=>setTimeout(resume,300));
  expect(existsSync(join(receiver.worktree,"later.txt"))).toBe(false);
  free=400*GIB;
  await until(()=>existsSync(join(receiver.worktree,"later.txt")));
  expect(readFileSync(join(receiver.worktree,"later.txt"),"utf8")).toBe("later\n");
 }finally{controller.abort();await done;await closeReplicaV1(peer);}
});
