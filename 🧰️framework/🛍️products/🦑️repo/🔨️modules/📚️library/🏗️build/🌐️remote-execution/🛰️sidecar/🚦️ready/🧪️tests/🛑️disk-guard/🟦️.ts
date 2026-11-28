/** 🛑️ Contract: a volume that is nearly full stops the stage that writes to it. The floor is the larger of an absolute number and a share of the volume, for bytes and for inodes; an unknown volume never stops anything; the stopped stage says so bilingually. Cross-checked against an independent Python model of the table. */
import{afterAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{chmodSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,runReadyStageV1,readyDiskGuardLineV1,readyDiskGuardV1,readyRefusalV1,type ReadyFilesystemFactsV1}from"../../🟦️.ts";

const MB=1048576,GB=1024*MB;
const facts=(freeBytes:number|null,totalBytes:number,freeInodes:number|null,totalInodes:number):ReadyFilesystemFactsV1=>({freeBytes,freeInodes,totalBytes,totalInodes});
const cases=[
 {name:"plenty",row:facts(100*GB,475*GB,3_000_000,5_966_400)},
 {name:"bytes below share floor",row:facts(4*GB,475*GB,3_000_000,5_966_400)},
 {name:"bytes just above share floor",row:facts(5*GB,475*GB,3_000_000,5_966_400)},
 {name:"small volume absolute floor",row:facts(1*GB,20*GB,100_000,1_000_000)},
 {name:"small volume above absolute floor",row:facts(3*GB,20*GB,100_000,1_000_000)},
 {name:"inodes below share floor",row:facts(100*GB,475*GB,14_000,5_966_400)},
 {name:"inodes above floor",row:facts(100*GB,475*GB,20_000,5_966_400)},
 {name:"windows without inodes",row:facts(100*GB,475*GB,null,0)},
 {name:"unknown bytes",row:facts(null,0,null,0)},
];

test("the floor table equals an independent Python model",()=>{
 const rows=cases.map(row=>[row.row.freeBytes,row.row.totalBytes,row.row.freeInodes,row.row.totalInodes]);
 const script=`
import json,sys
p=json.loads(sys.argv[1]);rows=json.loads(sys.argv[2]);out=[]
for fb,tb,fi,ti in rows:
    fbytes=max(p["floorMegabytes"]*1048576,tb*p["floorBasisPoints"]//10000)
    finodes=max(p["floorInodes"],(ti or 0)*p["floorInodeBasisPoints"]//10000)
    r=None
    if fb is not None and fb<fbytes: r="space"
    elif fi is not None and fi<finodes: r="inodes"
    out.append(r)
print(json.dumps(out))`;
 const run=spawnSync("python3",["-I","-c",script,JSON.stringify(P.diskGuard),JSON.stringify(rows)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout) as (string|null)[];
 expect<unknown>(cases.map(row=>readyDiskGuardV1("/w/.cache",row.row)?.resource??null)).toEqual(model);
 expect(model.filter(Boolean).length).toBeGreaterThan(2);
});

test("an unknown volume stops nothing and the hit carries the numbers",()=>{
 expect(readyDiskGuardV1("/w/.cache",null)).toBeNull();
 const hit=readyDiskGuardV1("/home/deck/semio/.cache",facts(4*GB,475*GB,170_000,5_966_400))!;
 expect(hit.resource).toBe("space");
 expect(hit.available).toBe(4*1024);
 expect(readyDiskGuardV1("/home/deck/semio/.cache",facts(13*GB,475*GB,170_000,5_966_400))).toBeNull();
 expect(readyDiskGuardV1("/w/.cache",facts(100*GB,475*GB,14_000,5_966_400))!.floor).toBe(15_000);
});

test("the stopped stage is told in English and German, and the code keeps the short back-off",()=>{
 const line=readyDiskGuardLineV1({resource:"space",path:"/home/deck/semio/.cache",available:13312,floor:14253});
 expect(line).toContain("disk guard stopped the stage");
 expect(line).toContain("Plattenwächter");
 const refusal=readyRefusalV1("disk-guard",{stage:"dependencies"});
 expect(refusal.en).toContain("dependencies");
 expect(refusal.de).toContain("dependencies");
 expect(refusal.de).toContain("Plattenwächter");
 expect(P.refreshKeepingFailureCodes).toContain("disk-guard");
});

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posix=process.platform==="win32"?test.skip:test;

posix("law: a running stage on a nearly full volume is stopped by the guard within its check interval and says why; on a volume with room it finishes",async()=>{
 const make=(script:string)=>{const home=mkdtempSync(join(tmpdir(),"diskguard-home-")),root=mkdtempSync(join(tmpdir(),"diskguard-root-")),bin=join(home,".local","bin");scratch.push(home,root);mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return{home,root};};
 const stage={...STAGES.find(row=>row.name==="tools")!,timeoutMilliseconds:600000};
 const run=(world:{home:string;root:string},row:ReadyFilesystemFactsV1)=>runReadyStageV1({...world,environment:{},stage,signal:new AbortController().signal,jobs:1,onLine:()=>undefined,facts:()=>row});
 const writer=make("#!/bin/sh\nwhile true; do echo \"Reading the project graph slice (1 s): taken 1 of 4455.\"; sleep 0.3; done\n"),quick=make("#!/bin/sh\nsleep 1\nexit 0\n");
 const began=Date.now(),stopped=await run(writer,facts(3*GB,475*GB,170_000,5_966_400));
 expect(stopped.diskGuard?.resource).toBe("space");
 expect(stopped.tail.some(line=>line.includes("Plattenwächter"))).toBe(true);
 expect(Date.now()-began).toBeLessThan(P.diskGuard.checkMilliseconds+25000);
 const fine=await run(quick,facts(100*GB,475*GB,3_000_000,5_966_400));
 expect(fine.exit).toBe(0);expect(fine.diskGuard).toBeUndefined();
},90000);
