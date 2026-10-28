/** 👯️ Tests for cache-gc dedupe. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {chmodSync,copyFileSync,linkSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheRunnerPortsV1} from "../../🏃️runner/🟦️.ts";
import {cloneFileV1,dedupeTreeV1,planDedupeV1,type DedupeFileV1} from "../🟦️.ts";

const HOUR=3600000,DAY=24*HOUR,SIZE=8192;
let root="";const now=Date.now();
const ports=(extra:Partial<Parameters<typeof cacheRunnerPortsV1>[0]>={},controller=new AbortController())=>{let calls=0;return cacheRunnerPortsV1({signal:controller.signal,now:()=>now+(calls++)*300,progress:async()=>{},exclusive:async()=>({release:()=>{}}),...extra});};
function put(path:string,content:string,ageMs:number,mode=0o644):string{const full=join(root,path);mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,content.padEnd(SIZE,"."));chmodSync(full,mode);const time=(now-ageMs)/1000;utimesSync(full,time,time);return full;}
function listing(base:string):string[]{return readdirSync(base,{recursive:true,withFileTypes:true}).filter(entry=>entry.isFile()).map(entry=>join(entry.parentPath,entry.name)).sort();}
const request=(dryRun:boolean,write:(line:string)=>void=()=>{},language:"en"|"de"="en")=>({base:root,minimumBytes:SIZE,quietMs:HOUR,maximumFiles:1000,dryRun,language,write});
beforeAll(()=>{root=mkdtempSync(join(tmpdir(),"ddp-"));});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

function fixture():void{
 rmSync(root,{recursive:true,force:true});mkdirSync(root,{recursive:true});
 put("a/x1","X",3*DAY);put("b/x2","X",4*DAY,0o444);put("c/deep/x3","X",5*DAY);linkSync(join(root,"a/x1"),join(root,"a/x1-link"));
 put("a/y1","Y",3*DAY);put("b/y2","Y",6*DAY);
 put("a/unique","U",3*DAY);put("a/tiny","T",3*DAY);
 put("d/recent1","R",3*DAY);put("d/recent2","R",60000);
 put("e/one.log","L",3*DAY);put("e/two.log","L",3*DAY);
 put("f/z1","Z",3*DAY);put("f/z2","Z",60000);
}

function randomCensus(seed:number,count:number):DedupeFileV1[]{
 let state=seed>>>0;const random=()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};
 return Array.from({length:count},(_,index)=>{const content=Math.floor(random()*12);return{path:"p/"+String(index).padStart(4,"0"),size:1000+content%3,hash:"h"+content,identity:"1:"+Math.floor(random()*(count/2)),links:1+Math.floor(random()*3),protected:random()<0.25};});
}

test("the pure plan equals an independent re-implementation on 300 seeded censuses, and replaces nothing protected",()=>{
 const code=`
import json,sys
def plan(files):
  groups={}
  for f in files: groups.setdefault((f["size"],f["hash"]),[]).append(f)
  out={"groups":0,"files":0,"reclaimable":0,"replacements":[]}
  for key in sorted(groups,key=lambda k:"%d:%s"%k):
    g=groups[key]
    if len({f["identity"] for f in g})<2: continue
    stable=[f for f in g if not f["protected"]]
    if not stable: continue
    c=sorted(stable,key=lambda f:(-f["links"],f["path"]))[0]
    v=sorted([f for f in stable if f["identity"]!=c["identity"]],key=lambda f:f["path"])
    if not v: continue
    out["groups"]+=1; out["files"]+=len(g)
    by={}
    for f in g:
        if f["identity"]!=c["identity"]: by.setdefault(f["identity"],[]).append(f)
    out["reclaimable"]+=sum(key[0] for ident,fs in by.items() if all(not f["protected"] for f in fs))
    out["replacements"]+=[[c["path"],x["path"]] for x in v]
  return out
print(json.dumps([plan(files) for files in json.load(sys.stdin)]))`;
 const censuses=Array.from({length:300},(_,index)=>randomCensus(index+1,40)),run=spawnSync("python3",["-I","-c",code],{input:JSON.stringify(censuses),encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);const expected=JSON.parse(run.stdout) as Array<{groups:number;files:number;reclaimable:number;replacements:string[][]}>;
 censuses.forEach((census,index)=>{
  const seed=index+1,plan=planDedupeV1(census);
  expect({groups:plan.groups,files:plan.files,reclaimable:plan.reclaimableBytes,replacements:plan.replacements.map(row=>[row.canonical,row.path])},"seed "+seed).toEqual(expected[index]);
  for(const row of plan.replacements)expect(census.find(file=>file.path===row.path)!.protected,"seed "+seed).toBe(false);
  expect(planDedupeV1(census.map(file=>({...file,protected:true}))).replacements).toEqual([]);
 });
});

test("a dry run finds exactly what an independent hashlib census finds and changes nothing",async()=>{
 fixture();const before=listing(root).map(path=>[path,statSync(path).ino]);
 const lines:string[]=[],outcome=await dedupeTreeV1(request(true,line=>lines.push(line)),ports());
 const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),root,String(SIZE),String(HOUR),String(now)],{encoding:"utf8"});
 expect(oracle.status,oracle.stderr).toBe(0);const expected=JSON.parse(oracle.stdout);
 expect({groups:outcome.plan.groups,files:outcome.plan.files,replacements:outcome.plan.replacements.map(row=>({canonical:row.canonical,path:row.path}))}).toEqual(expected);
 expect(outcome.plan.replacements.map(row=>row.path.slice(root.length+1)).sort()).toEqual(["b/x2","b/y2","c/deep/x3"]);
 expect(outcome.replaced).toBe(0);expect(lines.length).toBe(1);expect(lines[0]).toContain("would replace 3");
 expect(listing(root).map(path=>[path,statSync(path).ino])).toEqual(before);
 const shasum=spawnSync("sh",["-c","cd '"+root+"' && find . -type f -size +4k -exec shasum -a 256 {} + | awk '{print $1}' | sort | uniq -d | wc -l"],{encoding:"utf8"});
 expect(Number(shasum.stdout.trim())).toBeGreaterThanOrEqual(outcome.plan.groups);
});

test("replacing keeps bytes, mode and times, leaves recent and volatile files alone and distinct inodes untouched in content",async()=>{
 fixture();
 const stats=(path:string)=>{const info=statSync(join(root,path));return{ino:info.ino,mode:info.mode&0o777,mtimeMs:info.mtimeMs,content:readFileSync(join(root,path),"utf8")};};
 const paths=["a/x1","a/x1-link","b/x2","c/deep/x3","a/y1","b/y2","a/unique","d/recent1","d/recent2","e/one.log","e/two.log","f/z1","f/z2"],before=Object.fromEntries(paths.map(path=>[path,stats(path)]));
 const copy=async(source:string,destination:string)=>{copyFileSync(source,destination);return true;};
 const outcome=await dedupeTreeV1(request(false),ports(),copy);
 expect(outcome.replaced).toBe(3);expect(outcome.failed).toEqual([]);expect(outcome.freedBytes).toBe(3*SIZE);
 const after=Object.fromEntries(paths.map(path=>[path,stats(path)]));
 for(const path of paths){expect(after[path]!.content,path).toBe(before[path]!.content);expect(after[path]!.mode,path).toBe(before[path]!.mode);expect(Math.abs(after[path]!.mtimeMs-before[path]!.mtimeMs),path).toBeLessThan(2);}
 for(const path of["a/x1","a/x1-link","a/y1","a/unique","d/recent1","d/recent2","e/one.log","e/two.log","f/z1","f/z2"])expect(after[path]!.ino,path).toBe(before[path]!.ino);
 for(const path of["b/x2","c/deep/x3","b/y2"])expect(after[path]!.ino,path).not.toBe(before[path]!.ino);
 expect(listing(root).filter(path=>path.includes(".dedupe-"))).toEqual([]);
 const again=await dedupeTreeV1(request(true),ports());
 expect(again.plan.replacements.length).toBe(3);
});

test("a volume that cannot clone says so once, before any file is hashed, keeps every file and leaves no probe behind",async()=>{
 fixture();const before=listing(root).map(path=>[path,statSync(path).ino]),lines:string[]=[];let hashing=false;
 const outcome=await dedupeTreeV1(request(false,line=>lines.push(line)),ports({progress:async stage=>{if(stage.startsWith("dedupe:"))hashing=true;}}),async()=>false);
 expect(outcome.clones,"zero replacements are explained").toBe(false);expect(outcome.replaced).toBe(0);expect(outcome.skipped).toBe(0);expect(outcome.plan.groups).toBe(0);expect(lines.length).toBe(1);expect(lines[0]).toContain("cannot make copy-on-write clones");expect(hashing,"the census never started").toBe(false);
 expect((await dedupeTreeV1(request(false,line=>lines.push(line),"de"),ports(),async()=>false)).replaced).toBe(0);expect(lines[1]).toContain("keine Copy-on-Write-Klone");
 expect(listing(root).map(path=>[path,statSync(path).ino])).toEqual(before);
 expect(await cloneFileV1(join(root,"a/x1"),join(root,"a/x1"))).toBe(false);
});

test("a file that changes after it was hashed is kept, and cancellation stops between files with the tree intact",async()=>{
 fixture();let touched=false;
 const outcome=await dedupeTreeV1(request(false),ports({progress:async stage=>{if(stage.startsWith("dedupe:replace")&&!touched){touched=true;chmodSync(join(root,"b/x2"),0o644);writeFileSync(join(root,"b/x2"),"CHANGED".padEnd(SIZE,"!"),{flag:"w"});}}}),async(source,destination)=>{copyFileSync(source,destination);return true;});
 expect(touched).toBe(true);expect(readFileSync(join(root,"b/x2"),"utf8").startsWith("CHANGED")).toBe(true);expect(outcome.skipped).toBeGreaterThanOrEqual(1);
 fixture();const controller=new AbortController();let ticks=0;
 await expect(dedupeTreeV1(request(false),ports({progress:async stage=>{if(stage.startsWith("dedupe:replace")&&++ticks===1)controller.abort(Error("stop"));}},controller),async(source,destination)=>{copyFileSync(source,destination);return true;})).rejects.toThrow("stop");
 expect(listing(root).filter(path=>path.includes(".dedupe-"))).toEqual([]);
 for(const path of["a/x1","b/x2","c/deep/x3","a/y1","b/y2"])expect(readFileSync(join(root,path),"utf8").length).toBe(SIZE);
});

test("a canonical file rewritten with the same size after it was hashed is never cloned over its duplicates",async()=>{
 fixture();let touched=false;const lines:string[]=[];
 const outcome=await dedupeTreeV1(request(false,line=>lines.push(line)),ports({progress:async stage=>{if(stage.startsWith("dedupe:replace")&&!touched){touched=true;writeFileSync(join(root,"a/x1"),"CHANGED".padEnd(SIZE,"!"));}}}),async(source,destination)=>{copyFileSync(source,destination);return true;});
 expect(touched).toBe(true);
 for(const path of["b/x2","c/deep/x3"])expect(readFileSync(join(root,path),"utf8").startsWith("X"),path).toBe(true);
 expect(readFileSync(join(root,"b/y2"),"utf8").startsWith("Y")).toBe(true);
 expect(outcome.replaced).toBe(1);expect(outcome.skipped).toBe(2);expect(lines.filter(line=>line.includes(join(root,"a/x1"))).length).toBe(2);
});

test("on a copy-on-write volume the real clone frees blocks without sharing an inode",async()=>{
 fixture();const probe=join(root,"probe"),source=join(root,"a/x1");
 if(!await cloneFileV1(source,probe)){return;}
 expect(statSync(probe).ino).not.toBe(statSync(source).ino);expect(readFileSync(probe,"utf8")).toBe(readFileSync(source,"utf8"));
 rmSync(probe);
 const outcome=await dedupeTreeV1(request(false),ports());
 expect(outcome.replaced).toBe(3);expect(statSync(join(root,"b/x2")).ino).not.toBe(statSync(join(root,"a/x1")).ino);
 expect(readFileSync(join(root,"b/x2"),"utf8")).toBe(readFileSync(join(root,"a/x1"),"utf8"));
});

test("the dedupe command reports and replaces in the declared places in both languages, skips unmounted places and refuses unknown ones",async()=>{
 const{runCacheLedgerCommandV1,cacheArgumentsV1}=await import("../../⌨️command/🟦️.ts"),{readMachinePolicyV1}=await import("../../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts");
 const purpose={en:"p",de:"q"},partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}},machinePolicy=readMachinePolicyV1({
  version:1,defaultTotalBytes:1<<20,environment:"TEST_BUDGET",file:"m.json",hostFreeFloorBytes:0,alignBytes:1048576,toleranceBytes:0,
  stores:[{id:"alpha",partition:"alpha",weightBasisPoints:10000,floorBytes:0,headroomBasisPoints:0,purpose}],
  dedupe:[{id:"here",partition:"alpha",directory:"out",minimumBytes:SIZE,quietMs:HOUR,maximumFiles:100},{id:"there",partition:"alpha",directory:"missing",minimumBytes:SIZE,quietMs:HOUR,maximumFiles:100}]
 },partitions);
 const workspace=mkdtempSync(join(tmpdir(),"ddw-"));
 try{
  for(const name of["a","b","c"]){const full=join(workspace,".cache/b/a/m/out",name,"blob");mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"S".repeat(SIZE));const time=(now-3*DAY)/1000;utimesSync(full,time,time);}
  const run=async(args:string[])=>{const out:string[]=[];const code=await runCacheLedgerCommandV1(args,{workspace,environment:{},write:line=>out.push(line),partitions,machinePolicy},ports());return{code,out:out.join("\n")};};
  const english=await run(["dedupe","--dry-run","--lang=en"]),german=await run(["dedupe","--dry-run","--lang=de"]);
  expect(english.code).toBe(0);expect(english.out).toContain("1 groups of identical files, 3 files");expect(english.out).toContain("would replace 2 by clones");expect(english.out).toContain("Skipping there");
  expect(german.out).toContain("würde 2 durch Klone ersetzen");expect(german.out).toContain("Überspringe there");
  const json=JSON.parse((await run(["dedupe","--dry-run","--json","--area=here"])).out);
  expect(json.targets.length).toBe(1);expect(json.targets[0].plannedReplacements).toBe(2);expect(json.targets[0].reclaimableBytes).toBe(2*SIZE);expect(json.dryRun).toBe(true);
  expect(statSync(join(workspace,".cache/b/a/m/out/b/blob")).nlink).toBe(1);
  expect(()=>cacheArgumentsV1(["dedupe","--area=nope"])).toThrow("Unknown cache area");
  expect(()=>cacheArgumentsV1(["dedupe","--deep"])).toThrow("Unknown cache option");
  expect(cacheArgumentsV1(["dedupe","--dry-run","--area=bazel-outputs"]).only).toEqual(new Set(["bazel-outputs"]));
 }finally{rmSync(workspace,{recursive:true,force:true});}
});

test("the dedupe command explains zero replacements and an unreadable free space instead of reporting a silent zero, in both languages and in JSON",async()=>{
 const{runCacheLedgerCommandV1}=await import("../../⌨️command/🟦️.ts"),{readMachinePolicyV1}=await import("../../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts");
 const purpose={en:"p",de:"q"},partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}},machinePolicy=readMachinePolicyV1({
  version:1,defaultTotalBytes:1<<20,environment:"TEST_BUDGET",file:"m.json",hostFreeFloorBytes:0,alignBytes:1048576,toleranceBytes:0,
  stores:[{id:"alpha",partition:"alpha",weightBasisPoints:10000,floorBytes:0,headroomBasisPoints:0,purpose}],
  dedupe:[{id:"here",partition:"alpha",directory:"out",minimumBytes:SIZE,quietMs:HOUR,maximumFiles:100}]
 },partitions);
 const workspace=mkdtempSync(join(tmpdir(),"ddx-"));
 try{
  for(const name of["a","b","c"]){const full=join(workspace,".cache/b/a/m/out",name,"blob");mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"S".repeat(SIZE));const time=(now-3*DAY)/1000;utimesSync(full,time,time);}
  const run=async(args:string[],extra:Record<string,unknown>)=>{const out:string[]=[];const code=await runCacheLedgerCommandV1(args,{workspace,environment:{},write:line=>out.push(line),partitions,machinePolicy,...extra},ports());return{code,out:out.join("\n")};};
  const noClone=async()=>false,copy=async(source:string,destination:string)=>{copyFileSync(source,destination);return true;};
  const english=await run(["dedupe","--lang=en"],{clone:noClone,freeBytes:()=>1000}),german=await run(["dedupe","--lang=de"],{clone:noClone,freeBytes:()=>1000});
  expect(english.code).toBe(0);expect(english.out).toContain("cannot make copy-on-write clones");expect(english.out).toContain("Replaced 0 files");expect(german.out).toContain("keine Copy-on-Write-Klone");
  const none=JSON.parse((await run(["dedupe","--json"],{clone:noClone,freeBytes:()=>1000})).out).targets[0];
  expect(none.clones).toBe(false);expect(none.plannedReplacements).toBe(0);expect(none.replaced).toBe(0);
  const unknown=await run(["dedupe","--lang=en"],{clone:copy,freeBytes:()=>undefined}),unknownDe=await run(["dedupe","--lang=de"],{clone:copy,freeBytes:()=>undefined});
  expect(unknown.out).toContain("free space of this volume could not be read");expect(unknown.out).toContain("the volume gain is unknown");expect(unknown.out).not.toContain("the volume gained");
  expect(unknownDe.out).toContain("freie Platz dieses Volumes konnte nicht gelesen werden");expect(unknownDe.out).toContain("Gewinn des Volumes ist unbekannt");
  const reading=JSON.parse((await run(["dedupe","--json"],{clone:copy,freeBytes:()=>undefined})).out).targets[0];
  expect(reading.clones).toBe(true);expect(reading.volumeGainBytes,"unknown is null, never a zero").toBeNull();
  for(const name of["a","b","c"]){const full=join(workspace,".cache/b/a/m/out",name,"blob");writeFileSync(full,"S".repeat(SIZE));const time=(now-3*DAY)/1000;utimesSync(full,time,time);}
  let reads=0;const known=await run(["dedupe","--lang=en"],{clone:copy,freeBytes:()=>(reads++===0?1000:1000+2*SIZE)});
  expect(known.out).toContain("the volume gained 16.00 KiB");expect(known.out).not.toContain("could not be read");
 }finally{rmSync(workspace,{recursive:true,force:true});}
});
