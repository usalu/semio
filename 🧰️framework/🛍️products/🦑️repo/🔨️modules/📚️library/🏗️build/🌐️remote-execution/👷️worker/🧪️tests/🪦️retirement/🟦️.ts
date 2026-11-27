import {afterEach,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../../🟦️.ts";
import {claimRemoteExecutionWorkV1,collectRemoteExecutionForLedgerV1,RemoteExecutionBlobCacheV1,retireRemoteExecutionWorkV1,type RemoteExecutionImageReaderV1} from "../../🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {createNativeImageSharedReaderV1,nativeImageSharedReadResourceV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";

/** 🪦️ Contract: the private action roots of a worker whose process is gone are retired at once by the liveness of their owner record, never those of a live owner or of the caller, a folder without a record only once it was untouched for the declared time; equal to an independent reading of the same rule. */
const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=():string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-retire"),"s-"));scratch.push(path);return path;};
const dead=2147483646,day=86400;
type Row={name:string;owner:"live"|"dead"|"none"|"broken";ageSeconds:number;tasks:number};
function build(now:number,rows:readonly Row[],base:string=temporary()):string{
 const work=join(base,"work");mkdirSync(work,{recursive:true});
 for(const row of rows){
  const folder=join(work,row.name);mkdirSync(folder,{recursive:true});
  for(let index=0;index<row.tasks;index++){mkdirSync(join(folder,"task"+index),{recursive:true});writeFileSync(join(folder,"task"+index,"file"),"x");}
  if(row.owner==="live")writeFileSync(join(folder,".owner"),JSON.stringify({version:1,pid:process.pid,startedMs:now}));
  if(row.owner==="dead")writeFileSync(join(folder,".owner"),JSON.stringify({version:1,pid:dead,startedMs:now}));
  if(row.owner==="broken")writeFileSync(join(folder,".owner"),"{");
  const at=(now-row.ageSeconds*1000)/1000;utimesSync(folder,at,at);
 }
 return work;
}
const rows:Row[]=[
 {name:"live-old",owner:"live",ageSeconds:3*day,tasks:2},
 {name:"dead-new",owner:"dead",ageSeconds:1,tasks:3},
 {name:"dead-old",owner:"dead",ageSeconds:3*day,tasks:1},
 {name:"none-young",owner:"none",ageSeconds:60,tasks:1},
 {name:"none-old",owner:"none",ageSeconds:2*day,tasks:2},
 {name:"broken-old",owner:"broken",ageSeconds:2*day,tasks:1},
 {name:"broken-young",owner:"broken",ageSeconds:5,tasks:1},
 {name:"mine",owner:"dead",ageSeconds:2*day,tasks:4}
];

test("a dead owner's work folder goes at once, a live owner's never, an ownerless one only when old, and the caller's own is kept",async()=>{
 const now=Date.now(),work=build(now,rows);
 const plan=await retireRemoteExecutionWorkV1(work,{keep:"mine",ownerlessSeconds:day,now,dryRun:true});
 expect(plan).toEqual({workers:4,roots:3+1+2+1});
 expect(readdirSync(work).sort()).toEqual(rows.map(row=>row.name).sort());
 const done=await retireRemoteExecutionWorkV1(work,{keep:"mine",ownerlessSeconds:day,now});
 expect(done).toEqual(plan);
 expect(readdirSync(work).sort()).toEqual(["broken-young","live-old","mine","none-young"]);
 expect(await retireRemoteExecutionWorkV1(work,{keep:"mine",ownerlessSeconds:day,now})).toEqual({workers:0,roots:0});
});

test("the survivors equal the independent Python reading of the same rule",async()=>{
 const now=Date.now(),work=build(now,rows);
 const facts=rows.map(row=>({name:row.name,owner:row.owner,age:row.ageSeconds}));
 const reading=spawnSync("python3",["-I","-c","import json,sys\nrows=json.loads(sys.argv[1])\nprint(json.dumps(sorted(r['name'] for r in rows if r['name']=='mine' or r['owner']=='live' or (r['owner'] in ('none','broken') and r['age']<86400))))",JSON.stringify(facts)],{encoding:"utf8"});
 expect(reading.status,reading.stderr).toBe(0);
 await retireRemoteExecutionWorkV1(work,{keep:"mine",ownerlessSeconds:day,now});
 expect(readdirSync(work).sort()).toEqual((JSON.parse(reading.stdout) as string[]).sort());
});

test("an aborted sweep removes nothing it had not reached, a stray file and a missing work folder are not errors, and a worker claims its folder with its own pid",async()=>{
 const now=Date.now(),work=build(now,rows),controller=new AbortController();controller.abort();
 expect(await retireRemoteExecutionWorkV1(work,{ownerlessSeconds:day,now,signal:controller.signal})).toEqual({workers:0,roots:0});
 writeFileSync(join(work,"stray.txt"),"x");
 await retireRemoteExecutionWorkV1(work,{ownerlessSeconds:day,now});
 expect(existsSync(join(work,"stray.txt"))).toBe(true);
 expect(await retireRemoteExecutionWorkV1(join(work,"missing"),{ownerlessSeconds:day,now})).toEqual({workers:0,roots:0});
 const mine=join(temporary(),"mine");await claimRemoteExecutionWorkV1(mine);
 expect(JSON.parse(readFileSync(join(mine,".owner"),"utf8"))).toMatchObject({version:1,pid:process.pid});
});

test("the collection of the ledger retires the dead workers' roots of the policy home, answers absent for a missing home and respects the dry run and the declared policy time",async()=>{
 const now=Date.now(),policy=readRemoteExecutionPolicyV1().worker,workspace=temporary(),home=join(workspace,policy.directory);
 expect(await collectRemoteExecutionForLedgerV1(workspace,false,new AbortController().signal)).toBe("absent");
 const work=build(now,rows,home),before=readdirSync(work).length;
 expect(policy.ownerlessWorkSeconds).toBe(day);
 expect(await collectRemoteExecutionForLedgerV1(workspace,true,new AbortController().signal)).toBe("would remove 0 blobs and 11 input roots of 5 dead workers");
 expect(readdirSync(work).length).toBe(before);
 expect(await collectRemoteExecutionForLedgerV1(workspace,false,new AbortController().signal)).toBe("removed 0 blobs and 11 input roots of 5 dead workers");
 expect(readdirSync(work).sort()).toEqual(["broken-young","live-old","none-young"]);
});

const none=new AbortController().signal;
type Probe={reader:RemoteExecutionImageReaderV1;steps:()=>number;refused:()=>number;done:()=>Promise<void>};
function leased(image:string,leaseDirectory:string):Probe{
 const real=createNativeImageSharedReaderV1({leaseDirectory,imageDirectory:image,signal:none,timeoutMs:2000}),resource=nativeImageSharedReadResourceV1(image);let steps=0,refused=0;
 const writer=()=>acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",signal:none,timeoutMs:0});
 return{reader:{run:body=>real.run(async()=>{steps++;if(await writer().then(lease=>{lease.release();return false;},()=>true))refused++;return body();})},steps:()=>steps,refused:()=>refused,done:async()=>{await real.close();(await writer()).release();}};
}

test("every step of the retirement runs under the shared lease of the image, a long removal is split into short steps, and the outcome equals the unleased one",async()=>{
 const now=Date.now(),image=temporary(),lease=temporary(),work=build(now,rows,image),plain=build(now,rows),probe=leased(image,lease);
 const big=join(work,"dead-old");for(let index=0;index<70;index++)writeFileSync(join(big,"task0","f"+index),"x");
 expect(await retireRemoteExecutionWorkV1(work,{keep:"mine",ownerlessSeconds:day,now,reader:probe.reader})).toEqual({workers:4,roots:3+1+2+1});
 expect(readdirSync(work).sort()).toEqual(["broken-young","live-old","mine","none-young"]);
 await retireRemoteExecutionWorkV1(plain,{keep:"mine",ownerlessSeconds:day,now});
 expect(readdirSync(work).sort()).toEqual(readdirSync(plain).sort());
 expect(probe.steps(),"listing, a look and a count per folder, then the removal steps; the 70 files alone need three batches").toBeGreaterThan(20);
 expect(probe.refused(),"a writer was refused in every single step").toBe(probe.steps());
 await probe.done();
});

test("the blob cache of a home inside an image trims in steps under the shared lease and ends between steps on cancellation",async()=>{
 const image=temporary(),lease=temporary(),directory=join(image,"blobs"),old=Date.now()/1000-86400;
 for(const[shard,name,age]of[["aa","aa01",3],["aa","aa02",2],["bb","bb01",1]] as const){mkdirSync(join(directory,shard),{recursive:true});const path=join(directory,shard,name);writeFileSync(path,"x".repeat(100));utimesSync(path,old-age*3600,old-age*3600);}
 const cache=new RemoteExecutionBlobCacheV1(directory,undefined as never,150),probe=leased(image,lease);
 expect(await cache.prune(true,{reader:probe.reader})).toBe(2);expect(readdirSync(join(directory,"aa")).length).toBe(2);
 const planned=probe.steps();expect(planned,"the listing and one step per shard").toBe(3);
 expect(await cache.prune(false,{reader:probe.reader})).toBe(2);
 expect(readdirSync(join(directory,"aa"))).toEqual([]);expect(readdirSync(join(directory,"bb"))).toEqual(["bb01"]);
 expect(probe.refused()).toBe(probe.steps());
 const stop=new AbortController();stop.abort(Error("stop"));
 await expect(cache.prune(false,{reader:probe.reader,signal:stop.signal})).rejects.toThrow("stop");
 await probe.done();
});

test("the collection of the ledger asks the images for the reader of the home it collects and answers cancelled when the signal ends it",async()=>{
 const workspace=temporary(),policy=readRemoteExecutionPolicyV1().worker,home=join(workspace,policy.directory),asked:string[]=[];
 const images={reader:(path:string):RemoteExecutionImageReaderV1=>{asked.push(path);return{run:async body=>body()};}};
 build(Date.now(),rows,home);
 expect(await collectRemoteExecutionForLedgerV1(workspace,true,none,"remote-execution",images)).toBe("would remove 0 blobs and 11 input roots of 5 dead workers");
 expect(asked).toEqual([home]);
 const stopped=new AbortController();stopped.abort(Error("stop"));
 expect(await collectRemoteExecutionForLedgerV1(workspace,false,stopped.signal,"remote-execution",images)).toBe("cancelled");
});
