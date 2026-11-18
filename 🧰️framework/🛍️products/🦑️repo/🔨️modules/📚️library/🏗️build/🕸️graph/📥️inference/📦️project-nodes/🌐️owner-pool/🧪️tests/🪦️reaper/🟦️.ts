/** 🪦️ Contract: a child of the pool never outlives its pool or its wave, and a starting pool ends the children of the pools before it. Oracle: real child processes, probed by the operating system (`kill -0`). */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{OWNER_POOL_POLICY_V1 as P}from"../../🟦️.ts";
import{ownerPoolAliveV1,ownerPoolChildFateV1,ownerPoolChildVerdictV1,ownerPoolProcessStartV1,reapOwnerPoolOrphansV1,writeOwnerPoolLeaseV1}from"../../🪦️reaper/🟦️.ts";

const scratch:string[]=[],started:number[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-pool-reaper-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const pid of started)try{process.kill(pid,"SIGKILL");}catch{}for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const until=async(test:()=>boolean,limit:number)=>{const began=Date.now();while(!test()&&Date.now()-began<limit)await pause(50);return Date.now()-began;};
const entry=join(import.meta.dir,"..","..","🧒️child","🟨️.mjs"),driver=join(import.meta.dir,"..","🧫️fixtures","🛌️blocking-driver.mjs"),wave="a".repeat(64),other="b".repeat(64);
const lease=(over:Record<string,unknown>={})=>({version:1 as const,pid:42,beatMs:1000,waves:[wave],...over});
const verdict=(over:Record<string,unknown>={})=>ownerPoolChildVerdictV1({parentPid:42,currentParent:42,alive:()=>true,lease:lease(),wave,now:2000,staleMilliseconds:20000,platform:"linux",...over} as Parameters<typeof ownerPoolChildVerdictV1>[0]);

test("a child holds only while its parent lives and is its parent, the lease names that parent, is fresh and lists the wave",()=>{
 expect(verdict()).toBe("holds");
 expect(verdict({alive:()=>false})).toBe("parent-gone");
 expect(verdict({currentParent:1})).toBe("parent-gone");
 expect(verdict({currentParent:1,platform:"win32"}),"windows does not hand a child to pid 1").toBe("holds");
 expect(verdict({lease:undefined})).toBe("lease-gone");
 expect(verdict({lease:lease({pid:43})})).toBe("lease-other");
 expect(verdict({now:1000+20001})).toBe("lease-stale");
 expect(verdict({lease:lease({waves:[other]})})).toBe("wave-superseded");
});

test("a dead parent ends a child after the short grace; a lease that is stale, other or without the wave only after the long one and never when a verdict in between holds",()=>{
 const fate=(verdict:Parameters<typeof ownerPoolChildFateV1>[0]["verdict"],since:number|undefined,now:number)=>ownerPoolChildFateV1({verdict,since,now,graceMilliseconds:3000,lostGraceMilliseconds:120000});
 expect(fate("holds",500,9e9)).toEqual({since:undefined,end:false});
 expect(fate("parent-gone",undefined,100)).toEqual({since:100,end:false});
 expect(fate("parent-gone",100,3100).end).toBe(true);
 expect(fate("lease-gone",100,3100).end,"a removed lease is a pool that ended").toBe(true);
 for(const verdict of["lease-stale","lease-other","wave-superseded"] as const){expect(fate(verdict,100,3100).end,verdict).toBe(false);expect(fate(verdict,100,60000).end,verdict).toBe(false);expect(fate(verdict,100,120100).end,verdict).toBe(true);}
 expect(fate("lease-stale",100,3100).since).toBe(100);
 expect(P.leaseLostGraceMilliseconds).toBeGreaterThan(P.leaseStaleMilliseconds);expect(P.leaseStaleMilliseconds).toBeGreaterThanOrEqual(10*P.leaseHeartbeatMilliseconds);
});

test("the start time of a process is its own and the same on every look, a pid that is gone has none, both platforms ask their own tool",()=>{
 const mine=ownerPoolProcessStartV1(process.pid);
 expect(mine).toBeTruthy();expect(ownerPoolProcessStartV1(process.pid)).toBe(mine);
 expect(ownerPoolProcessStartV1(2**30)).toBeUndefined();
 const asked:string[]=[];
 expect(ownerPoolProcessStartV1(7,"win32",(command,args)=>{asked.push(command+" "+args.join(" "));return{status:0,stdout:"2026-11-17T01:44:03.0000000Z\r\n"};})).toBe("2026-11-17T01:44:03.0000000Z");
 expect(ownerPoolProcessStartV1(7,"linux",(command,args)=>{asked.push(command+" "+args.join(" "));return{status:0,stdout:"Tue Nov 17  01:44:03 2026\n"};})).toBe("Tue Nov 17 01:44:03 2026");
 expect(asked[0]).toContain("powershell.exe");expect(asked[0]).toContain("Get-Process -Id 7");expect(asked[1]).toBe("ps -o lstart= -p 7");
});

type Spawned={pid:number;file:string;dir:string;leaseFile:string};
const childOf=(label:string,parentPid:number,over:Record<string,unknown>={},launch=true):Spawned=>{
 const dir=temp(label),leaseFile=join(dir,P.leaseFileName);
 writeFileSync(join(dir,"head.json"),JSON.stringify({version:1,id:wave,createdMs:Date.now(),globals:{}}));
 writeFileSync(join(dir,"units.json"),JSON.stringify([{name:"owner-0",kind:"k",expectedMilliseconds:60000,json:{}}]));
 writeFileSync(join(dir,"session.bin"),"{}");
 const file=join(dir,"child-1.json");
 writeFileSync(file,JSON.stringify({dir,root:dir,fleetRoot:dir,puller:"t:1",lifetimeMilliseconds:600000,driver,waveId:wave,parentPid,lease:leaseFile,watchMilliseconds:100,graceMilliseconds:400,lostGraceMilliseconds:1500,staleMilliseconds:1500,...over}));
 if(!launch)return{pid:0,file,dir,leaseFile};
 const proc=Bun.spawn([process.execPath,entry,file],{stdin:"pipe",stdout:"pipe",stderr:"inherit",env:{...process.env,SEMIO_OWNER_POOL_CHILD:"1"}});
 started.push(proc.pid);
 return{pid:proc.pid,file,dir,leaseFile};
};
const beat=(file:string,pid:number,waves:string[]=[wave])=>writeOwnerPoolLeaseV1(file,{version:1,pid,beatMs:Date.now(),waves});
const ready=async(pid:number)=>{await pause(1500);expect(ownerPoolAliveV1(pid)).toBe(true);};

test("real child: a parent that is killed without any chance to clean up takes its child with it, even while the child's event loop is held by an owner",async()=>{
 const dir=temp("orphan"),leaseFile=join(dir,"lease.json");
 const code=`const {spawn}=require("node:child_process");const c=spawn(process.execPath,[${JSON.stringify(entry)},process.argv[1]],{detached:true,stdio:"ignore",env:{...process.env,SEMIO_OWNER_POOL_CHILD:"1"}});c.unref();console.log(c.pid);setInterval(()=>{},1000);`;
 const sample=childOf("orphan-args",0,{},false);
 const args=JSON.parse(readFileSync(sample.file,"utf8"));
 const stub=Bun.spawn([process.execPath,"-e",code,sample.file],{stdout:"pipe",stderr:"inherit"});started.push(stub.pid);
 args.parentPid=stub.pid;args.lease=leaseFile;writeFileSync(sample.file,JSON.stringify(args));
 beat(leaseFile,stub.pid);
 const reader=stub.stdout.getReader(),text=new TextDecoder().decode((await reader.read()).value),pid=Number(text.trim());
 started.push(pid);
 const keep=setInterval(()=>{try{beat(leaseFile,stub.pid);}catch{}},200);
 try{await ready(pid);}finally{}
 stub.kill("SIGKILL");await stub.exited;clearInterval(keep);const killed=Date.now();
 const took=await until(()=>!ownerPoolAliveV1(pid),15000);
 expect(ownerPoolAliveV1(pid),"the orphan is still alive").toBe(false);expect(Date.now()-killed).toBeLessThan(8000);expect(took).toBeLessThan(8000);
},60000);

test("real child: a lease that goes stale, one that names another pool, and one that no longer lists the wave each end the child within seconds; a beaten lease that lists it keeps it",async()=>{
 const mine=process.pid,kept=childOf("kept",mine);beat(kept.leaseFile,mine);
 const keep=setInterval(()=>{beat(kept.leaseFile,mine);},200);
 const stale=childOf("stale",mine);beat(stale.leaseFile,mine);
 const another=childOf("another",mine);beat(another.leaseFile,mine+1);
 const superseded=childOf("superseded",mine);const keepSuperseded=setInterval(()=>{beat(superseded.leaseFile,mine,[other]);},200);
 try{
  await pause(1500);
  for(const child of[kept,stale,another,superseded])expect(ownerPoolAliveV1(child.pid)||child===stale||child===another||child===superseded).toBe(true);
  await until(()=>[stale,another,superseded].every(child=>!ownerPoolAliveV1(child.pid)),9000);
  expect(ownerPoolAliveV1(stale.pid),"stale").toBe(false);expect(ownerPoolAliveV1(another.pid),"another pool").toBe(false);expect(ownerPoolAliveV1(superseded.pid),"superseded wave").toBe(false);
  expect(ownerPoolAliveV1(kept.pid),"a held child stays").toBe(true);
 }finally{clearInterval(keep);clearInterval(keepSuperseded);}
},60000);

test("real child: a lease that is not beaten for a while but comes back keeps the child, one that stays lost writes its verdict before the child ends",async()=>{
 const mine=process.pid,starved=childOf("starved",mine,{lostGraceMilliseconds:3000,exitFile:join(temp("exit"),"x.exit.json")});beat(starved.leaseFile,mine);
 const exitFile=JSON.parse(readFileSync(starved.file,"utf8")).exitFile as string;
 await pause(2200);expect(ownerPoolAliveV1(starved.pid),"stale for 0.7 s of 3 s").toBe(true);
 beat(starved.leaseFile,mine);await pause(2200);expect(ownerPoolAliveV1(starved.pid),"beaten again: the clock starts over").toBe(true);
 await until(()=>!ownerPoolAliveV1(starved.pid),9000);
 expect(ownerPoolAliveV1(starved.pid),"lost for good").toBe(false);
 expect(JSON.parse(readFileSync(exitFile,"utf8")).verdict).toBe("lease-stale");
},60000);

test("real processes: a starting pool ends the live children of a pool that is gone by the record of their wave, and leaves a process whose start time differs, a child of a living pool and its own children",async()=>{
 const root=temp("reap"),kept=join(root,"waves"),registry=join(root,"children");mkdirSync(join(kept,"w1"),{recursive:true});mkdirSync(registry,{recursive:true});
 const gone=Bun.spawn(["sleep","600"]),stranger=Bun.spawn(["sleep","600"]),guarded=Bun.spawn(["sleep","600"]),mine=Bun.spawn(["sleep","600"]),parent=Bun.spawn(["sleep","600"]);
 for(const proc of[gone,stranger,guarded,mine,parent])started.push(proc.pid);
 const start=(pid:number)=>ownerPoolProcessStartV1(pid)!;
 writeFileSync(join(kept,"w1","child-1.json"),JSON.stringify({pid:gone.pid,startTime:start(gone.pid),parentPid:2**30}));
 writeFileSync(join(kept,"w1","child-2.json"),JSON.stringify({pid:stranger.pid,startTime:"Mon Jan  1 00:00:00 2001",parentPid:2**30}));
 writeFileSync(join(registry,guarded.pid+".json"),JSON.stringify({pid:guarded.pid,startTime:start(guarded.pid),parentPid:parent.pid,parentStart:start(parent.pid)}));
 writeFileSync(join(registry,mine.pid+".json"),JSON.stringify({pid:mine.pid,startTime:start(mine.pid),parentPid:2**30}));
 const ended=await reapOwnerPoolOrphansV1({kept,registry,own:new Set([mine.pid]),self:process.pid});
 expect(ended).toEqual([gone.pid]);
 expect(ownerPoolAliveV1(gone.pid),"orphan ended").toBe(false);
 for(const survivor of[stranger,guarded,mine])expect(ownerPoolAliveV1(survivor.pid),"left alone").toBe(true);
},30000);
