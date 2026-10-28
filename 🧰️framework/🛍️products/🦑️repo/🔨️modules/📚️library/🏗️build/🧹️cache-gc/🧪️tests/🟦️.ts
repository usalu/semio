import {Database} from "bun:sqlite";
import {nativeImageBackingFilenameV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {afterAll,beforeAll,expect,test} from "bun:test";
import {linkSync,mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CACHE_SIZE_FILE_V1,cacheTextV1,formatCacheBytesV1,readCacheLedgerV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,cacheSqliteFreeV1,collectCacheV1,measureCacheTreeV1,probeCacheHeldV1,removeCacheTreeV1,loadCacheSizesV1,recordCacheSizesV1,reportCacheLedgerV1,undeclaredEntriesV1,type CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";
import {cacheArgumentsV1,runCacheLedgerCommandV1} from "../⌨️command/🟦️.ts";

const DAY=86400000,HOUR=3600000,partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30},beta:{directory:".cache/b/b",capacityBytes:1<<30,maximumLogicalBytes:1<<30}};
const reason={en:"x",de:"y"},purpose={en:"p",de:"q"};
const ledger:CacheLedgerV1=readCacheLedgerV1({
 version:1,
 roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[".DS_Store",CACHE_SIZE_FILE_V1]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]}},
 areas:[
  {id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},recreate:"derived",writers:["w"],purpose},
  {id:"beta",root:"native",kind:"partition",directory:"b/b",partition:"beta",managed:{owner:"image"},writers:["w"],purpose},
  {id:"inner",root:"native",kind:"nested",directory:"x/inner",partition:"alpha",budget:{bytes:1<<20},managed:{owner:"nx"},writers:["w"],purpose},
  {id:"rest",root:"native",kind:"nested",partition:"alpha",budget:{remainder:true},managed:{owner:"cargo"},writers:["w"],purpose},
  {id:"t1",root:"repository",kind:"tree",directory:"t1",budget:{bytes:100000},collect:{unusedAgeMs:10*DAY,quietMs:HOUR,custody:"quiet"},writers:["w"],purpose},
  {id:"t2",root:"repository",kind:"tree",directory:"t2",budget:{bytes:150000},collect:{unusedAgeMs:null,quietMs:0,custody:"quiet"},writers:["w"],purpose},
  {id:"leased",root:"repository",kind:"tree",directory:"leased",budget:{bytes:100000},collect:{unusedAgeMs:DAY,quietMs:0,custody:"lease",lease:"L:leased"},writers:["w"],purpose},
  {id:"agents",root:"repository",kind:"tree",directory:"agents",budget:{bytes:100000},collect:{unusedAgeMs:DAY,quietMs:0,custody:"sqlite"},writers:["w"],purpose},
  {id:"kept",root:"repository",kind:"tree",directory:"kept",budget:{bytes:100000},managed:{owner:"test-evidence"},writers:["w"],purpose},
  {id:"hex",root:"repository",kind:"tree",pattern:"^[0-9a-f]{8}$",budget:{bytes:100000},collect:{unusedAgeMs:DAY,quietMs:0,custody:"quiet"},writers:["w"],purpose},
  {id:"play",root:"repository",kind:"tree",directory:"tools/play",budget:{bytes:100000},collect:{unusedAgeMs:DAY,quietMs:0,custody:"quiet"},writers:["w"],purpose}
 ],
 retirements:[
  {root:"repository",directory:"old",quietMs:HOUR,reason},
  {root:"repository",directory:"tools/dead",quietMs:HOUR,reason},
  {root:"native",directory:"gone",quietMs:HOUR,reason}
 ]
},partitions);

let workspace="";
const now=Date.now();
function file(path:string,size:number,ageMs:number):string{const full=join(workspace,path);mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"x".repeat(size));const time=(now-ageMs)/1000;utimesSync(full,time,time);return full;}
function ports(extra:Partial<CacheRunnerPortsV1>={},controller=new AbortController()):CacheRunnerPortsV1&{stages:string[];released:string[]}{
 const stages:string[]=[],released:string[]=[];
 const base=cacheRunnerPortsV1({signal:controller.signal,now:()=>now,progress:async stage=>{stages.push(stage);},exclusive:async resource=>({release:()=>{released.push(resource);}}),...extra});
 return {...base,stages,released};
}
const lines=():{write(line:string):void;language:"en"|"de";out:string[]}=>{const out:string[]=[];return{write:line=>{out.push(line);},language:"en",out};};
const sorted=(path:string)=>readdirSync(path).sort();
const reset=():void=>{rmSync(workspace,{recursive:true,force:true});mkdirSync(workspace,{recursive:true});};

beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"cgc-"));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("measuring counts allocated bytes like the file system does, hardlinks once, links not at all, and takes recency from files only",async()=>{
 reset();
 const a=file("m/a.bin",5000,3*DAY),b=file("m/deep/er/b.bin",100,9*DAY);linkSync(a,join(workspace,"m/hardlink.bin"));
 mkdirSync(join(workspace,"outside"));writeFileSync(join(workspace,"outside/huge.bin"),"y".repeat(100000));symlinkSync(join(workspace,"outside"),join(workspace,"m/link"));
 const fresh=now/1000;utimesSync(join(workspace,"m/deep"),fresh,fresh);
 const measure=await measureCacheTreeV1(join(workspace,"m"),ports(),1000);
 const expected=[a,b].reduce((sum,path)=>sum+(statSync(path).blocks*512||statSync(path).size),0),listed=readdirSync(join(workspace,"m"),{recursive:true,withFileTypes:true}).filter(entry=>entry.isFile()).length;
 expect(measure.bytes).toBe(expected);
 expect(listed).toBeGreaterThanOrEqual(3);
 expect(measure.recencyMs).toBeGreaterThan(now-3*DAY-2000);
 expect(measure.recencyMs).toBeLessThan(now-3*DAY+2000);
 expect(measure.partial).toBe(false);
 const empty=join(workspace,"empty");mkdirSync(empty);
 expect((await measureCacheTreeV1(empty,ports(),10)).recencyMs).toBeGreaterThan(0);
 const missing=await measureCacheTreeV1(join(workspace,"nothing"),ports(),10);
 expect(missing).toEqual({present:false,bytes:0,recencyMs:0,entries:0,partial:false,sqlite:[],queues:[]});
 const limited=await measureCacheTreeV1(join(workspace,"m"),ports(),2);
 expect(limited.partial).toBe(true);
 expect(limited.entries).toBe(3);
 const single=await measureCacheTreeV1(a,ports(),10);
 expect(single.entries).toBe(1);
});

test("a cancelled walk stops at once and reports through progress",async()=>{
 reset();
 for(let index=0;index<300;index++)file("c/f"+index,1,DAY);
 const controller=new AbortController(),seen:ReturnType<typeof ports>=ports({progress:async stage=>{seen.stages.push(stage);controller.abort(Error("stop"));}},controller);
 await expect(measureCacheTreeV1(join(workspace,"c"),seen,1000,"walk")).rejects.toThrow("stop");
 expect(seen.stages).toContain("walk");
});

test("a held SQLite lease file is detected by the kernel lock and released locks free it",async()=>{
 reset();
 const path=join(workspace,"lock.sqlite");mkdirSync(workspace,{recursive:true});
 const holder=new Database(path);holder.exec("PRAGMA journal_mode=DELETE;CREATE TABLE t(x)");
 expect(await cacheSqliteFreeV1(path)).toBe(true);
 holder.exec("BEGIN EXCLUSIVE");
 expect(await cacheSqliteFreeV1(path)).toBe(false);
 holder.exec("ROLLBACK");
 holder.exec("BEGIN");holder.query("SELECT * FROM t").all();
 expect(await cacheSqliteFreeV1(path)).toBe(false);
 holder.exec("ROLLBACK");holder.close();
 expect(await cacheSqliteFreeV1(path)).toBe(true);
 writeFileSync(join(workspace,"junk.sqlite"),"not a database");
 expect(await cacheSqliteFreeV1(join(workspace,"junk.sqlite"))).toBe(true);
 expect(await cacheSqliteFreeV1(join(workspace,"absent.sqlite"))).toBe(true);
});

test("queue tickets of living processes hold their unit and those of dead ones do not",async()=>{
 reset();
 const ticket=(pid:number)=>"0000000abc-"+pid.toString(36).padStart(7,"0")+"-owner";
 const dead=Bun.spawnSync([process.execPath,"-e","0"]).pid;
 file("q/a.queue/"+ticket(process.pid),1,DAY);file("z/a.queue/"+ticket(dead),1,DAY);file("y/a.queue/not-a-ticket",1,DAY);
 const live=await measureCacheTreeV1(join(workspace,"q"),ports(),100),gone=await measureCacheTreeV1(join(workspace,"z"),ports(),100),odd=await measureCacheTreeV1(join(workspace,"y"),ports(),100);
 expect(await probeCacheHeldV1(live,ports())).toBe("pid:"+process.pid);
 expect(await probeCacheHeldV1(gone,ports())).toBeUndefined();
 expect(await probeCacheHeldV1(odd,ports())).toBeUndefined();
});

test("removal is cancellable, never follows links and removes a whole tree",async()=>{
 reset();
 mkdirSync(join(workspace,"outside"));writeFileSync(join(workspace,"outside/keep.bin"),"keep");
 file("tree/a/b/c.bin",10,DAY);file("tree/a/d.bin",10,DAY);file("tree/e.bin",10,DAY);symlinkSync(join(workspace,"outside"),join(workspace,"tree/a/link"));
 await removeCacheTreeV1(join(workspace,"tree"),ports(),"remove");
 expect(existsSync(join(workspace,"tree"))).toBe(false);
 expect(readdirSync(join(workspace,"outside"))).toEqual(["keep.bin"]);
 await removeCacheTreeV1(join(workspace,"tree"),ports(),"remove");
 for(let index=0;index<400;index++)file("big/f"+index,1,DAY);
 const controller=new AbortController(),active=ports({progress:async()=>{controller.abort(Error("stop"));}},controller);
 await expect(removeCacheTreeV1(join(workspace,"big"),active,"remove")).rejects.toThrow("stop");
 expect(existsSync(join(workspace,"big"))).toBe(true);
});

test("a collection removes retired roots, aged and over-budget units and undeclared leftovers, and keeps everything else",async()=>{
 reset();
 file("r/cache/old/a.bin",100,30*DAY);file("r/cache/old/b.bin",100,30*DAY);
 file("r/cache/tools/dead/x.bin",100,30*DAY);
 file("r/cache/tools/play/chromium/x.bin",100,30*DAY);file("r/cache/tools/play/new/x.bin",100,60000);
 file("r/cache/tools/mystery/x.bin",100,30*DAY);file("r/cache/tools/young/x.bin",100,60000);
 file("r/cache/e1/x.bin",100,30*DAY);file("r/cache/e2/x.bin",100,2*DAY);file("r/cache/.DS_Store",1,30*DAY);
 file("r/cache/t1/ancient/x.bin",100,20*DAY);file("r/cache/t1/fine/x.bin",100,2*DAY);file("r/cache/t1/quiet/x.bin",100,60000);
 file("r/cache/t2/one/x.bin",65536,5*DAY);file("r/cache/t2/two/x.bin",65536,4*DAY);file("r/cache/t2/three/x.bin",65536,3*DAY);file("r/cache/t2/four/x.bin",65536,2*DAY);
 file("r/cache/kept/x.bin",100,90*DAY);
 file("r/cache/deadbeef/x.bin",100,5*DAY);file("r/cache/cafebabe/x.bin",100,5*60000);file("r/cache/notahex/x.bin",100,5*DAY);
 file(".cache/gone/x.bin",100,30*DAY);file(".cache/qe2e/x.bin",100,2*DAY);
 const preview=ports(),dry=lines(),planned=await collectCacheV1(workspace,ledger,{dryRun:true},preview,dry,partitions);
 expect(planned.removed.length).toBeGreaterThan(0);
 expect(existsSync(join(workspace,"r/cache/old/a.bin"))).toBe(true);
 const real=ports(),out=lines(),result=await collectCacheV1(workspace,ledger,{dryRun:false},real,out,partitions);
 expect(result.failed).toEqual([]);
 expect(result.removed.map(row=>row.target+"/"+row.name+":"+row.reason).sort()).toEqual(planned.removed.map(row=>row.target+"/"+row.name+":"+row.reason).sort());
 expect(existsSync(join(workspace,"r/cache/old"))).toBe(false);
 expect(existsSync(join(workspace,"r/cache/tools/dead"))).toBe(false);
 expect(sorted(join(workspace,"r/cache/tools"))).toEqual(["play","young"]);
 expect(sorted(join(workspace,"r/cache/tools/play"))).toEqual(["new"]);
 expect(existsSync(join(workspace,"r/cache/e1"))).toBe(false);
 expect(existsSync(join(workspace,"r/cache/e2"))).toBe(true);
 expect(existsSync(join(workspace,"r/cache/.DS_Store"))).toBe(true);
 expect(sorted(join(workspace,"r/cache/t1"))).toEqual(["fine","quiet"]);
 expect(sorted(join(workspace,"r/cache/t2"))).toEqual(["four","three"]);
 expect(existsSync(join(workspace,"r/cache/kept/x.bin"))).toBe(true);
 expect(existsSync(join(workspace,"r/cache/deadbeef"))).toBe(false);
 expect(existsSync(join(workspace,"r/cache/cafebabe"))).toBe(true);
 expect(existsSync(join(workspace,"r/cache/notahex"))).toBe(true);
 expect(existsSync(join(workspace,".cache/gone"))).toBe(false);
 expect(existsSync(join(workspace,".cache/qe2e"))).toBe(true);
 expect(result.freedBytes).toBe(result.removed.reduce((sum,row)=>sum+row.bytes,0));
 expect(out.out[0]).toBe(cacheTextV1("en","gc.start",{count:result.plans.length,mode:""}));
 expect(out.out.at(-1)).toBe(cacheTextV1("en","gc.summary.apply",{count:result.removed.length,bytes:formatCacheBytesV1(result.freedBytes),kept:result.kept}));
 expect(out.out.some(line=>line.startsWith("Removing "))).toBe(true);
 expect(real.stages.some(stage=>stage.startsWith("scan:"))).toBe(true);
 const again=await collectCacheV1(workspace,ledger,{dryRun:false},ports(),lines(),partitions);
 expect(again.removed.filter(row=>row.reason!=="budget")).toEqual([]);
});

test("the dry run says what it would do in German and removes nothing",async()=>{
 reset();
 file("r/cache/old/a.bin",100,30*DAY);
 const out=lines();out.language="de";
 const result=await collectCacheV1(workspace,ledger,{dryRun:true},ports(),out,partitions);
 expect(result.dryRun).toBe(true);
 expect(out.out.some(line=>line.startsWith("Würde entfernen "))).toBe(true);
 expect(out.out[0]).toBe(cacheTextV1("de","gc.start",{count:result.plans.length,mode:cacheTextV1("de","gc.mode.plan")}));
 expect(existsSync(join(workspace,"r/cache/old/a.bin"))).toBe(true);
});

test("a unit that changes after it was scanned is kept",async()=>{
 reset();
 file("r/cache/old/a.bin",100,30*DAY);
 const target=join(workspace,"r/cache/old/a.bin"),active=ports({progress:async stage=>{if(stage.startsWith("scan:undeclared")||stage.includes("e9")){const time=now/1000;utimesSync(target,time,time);}}}),out=lines();
 file("r/cache/e9/x.bin",1,30*DAY);
 const result=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["retired","undeclared"])},active,out,partitions);
 expect(existsSync(target)).toBe(true);
 expect(out.out.some(line=>line.startsWith("Keeping ")&&line.includes("old/a.bin"))).toBe(true);
 expect(result.removed.some(row=>row.name==="a.bin")).toBe(false);
});

test("a lease that is held keeps the whole area and a granted one is released",async()=>{
 reset();
 file("r/cache/leased/old/x.bin",100,30*DAY);
 const denied=ports({exclusive:async()=>undefined}),out=lines(),result=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["leased"])},denied,out,partitions);
 expect(result.removed).toEqual([]);
 expect(existsSync(join(workspace,"r/cache/leased/old/x.bin"))).toBe(true);
 expect(out.out.some(line=>line==="Keeping leased: in use (L:leased)")).toBe(true);
 const granted=ports(),second=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["leased"])},granted,lines(),partitions);
 expect(second.removed.map(row=>row.name)).toEqual(["old"]);
 expect(granted.released).toEqual(["L:leased"]);
 const failing=ports({exclusive:async()=>{throw Error("broken");}});
 await expect(collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["leased"])},failing,lines(),partitions)).rejects.toThrow("broken");
});

test("SQLite lease files and queues in use are never removed, finished ones are",async()=>{
 reset();
 const busy=join(workspace,"r/cache/agents/busy.sqlite"),idle=join(workspace,"r/cache/agents/idle.sqlite");mkdirSync(join(workspace,"r/cache/agents"),{recursive:true});
 const holder=new Database(busy);holder.exec("PRAGMA journal_mode=DELETE;CREATE TABLE t(x)");const other=new Database(idle);other.exec("PRAGMA journal_mode=DELETE;CREATE TABLE t(x)");other.close();
 for(const path of[busy,idle]){const time=(now-30*DAY)/1000;utimesSync(path,time,time);}
 file("r/cache/agents/live.queue/0000000abc-"+process.pid.toString(36).padStart(7,"0")+"-owner",1,30*DAY);
 holder.exec("BEGIN EXCLUSIVE");
 const out=lines(),result=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["agents"])},ports(),out,partitions);
 holder.exec("ROLLBACK");holder.close();
 expect(sorted(join(workspace,"r/cache/agents"))).toEqual(["busy.sqlite","live.queue"]);
 expect(result.removed.map(row=>row.name)).toEqual(["idle.sqlite"]);
 expect(out.out.some(line=>line.includes("busy.sqlite")&&line.startsWith("Keeping "))).toBe(true);
 expect(out.out.some(line=>line.includes("live.queue")&&line.includes("pid:"+process.pid))).toBe(true);
 const final=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["agents"])},ports({alive:()=>false}),lines(),partitions);
 expect(final.removed.map(row=>row.name).sort()).toEqual(["busy.sqlite","live.queue"]);
});

test("managed areas are left to their owner and a delegate may report for them",async()=>{
 reset();
 const out=lines(),result=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["kept","inner"])},ports(),out,partitions);
 expect(out.out).toContain("Leaving kept to test-evidence");
 expect(out.out).toContain("Leaving inner to nx");
 expect(result.plans).toEqual([]);
 const asked:string[]=[],delegated=ports({delegate:async(owner,id,dry)=>{asked.push(owner+":"+id+":"+dry);return owner==="test-evidence"?"pruned 3":undefined;}}),second=lines(),again=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["kept","inner"])},delegated,second,partitions);
 expect(asked).toEqual(["nx:inner:true","test-evidence:kept:true"]);
 expect(second.out).toContain("test-evidence reports: pruned 3");
 expect(again.delegated).toEqual([{owner:"test-evidence",text:"pruned 3"}]);
});

test("cancelling a collection stops it between units and the command reports it",async()=>{
 reset();
 for(const name of["a","b","c"])file("r/cache/old/"+name+"/x.bin",100,30*DAY);
 const controller=new AbortController(),active=ports({progress:async stage=>{if(stage.startsWith("remove:"))controller.abort(Error("stop"));}},controller);
 await expect(collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["retired"])},active,lines(),partitions)).rejects.toThrow("stop");
 expect(readdirSync(join(workspace,"r/cache/old")).length).toBeGreaterThanOrEqual(2);
 const second=new AbortController(),output:string[]=[],code=await runCacheLedgerCommandV1(["gc","--area=retired"],{workspace,environment:{SEMIO_LANGUAGE:"en"},write:line=>output.push(line),ledger,partitions},ports({progress:async stage=>{if(stage.startsWith("remove:"))second.abort(Error("stop"));}},second));
 expect(code).toBe(130);
 expect(output.at(-1)).toBe("Cache collection cancelled");
});

test("status reads partitions, areas, retired roots and undeclared entries without any lease",async()=>{
 reset();
 mkdirSync(join(workspace,".cache/b/a/m"),{recursive:true});writeFileSync(join(workspace,".cache/b/a",nativeImageBackingFilenameV1()),"");
 mkdirSync(join(workspace,".cache/b/b/m"),{recursive:true});file(".cache/b/b/m/x.bin",5000,DAY);
 file("r/cache/t1/x/y.bin",5000,DAY);file("r/cache/old/z.bin",5000,DAY);file("r/cache/e1/q.bin",5000,DAY);file("r/cache/deadbeef/x.bin",10,DAY);
 const held=ports({exclusive:async()=>{throw Error("status never takes a lease");}}),report=await reportCacheLedgerV1(workspace,ledger,held,partitions,undefined,{deep:true});
 const state=(id:string)=>report.partitions.find(row=>row.id===id)!;
 expect(state("alpha").state).toBe("detached");
 expect(state("alpha").usedBytes).toBeNull();
 expect(state("beta").state).toBe("directory");
 expect(state("beta").usedBytes).toBe((await measureCacheTreeV1(join(workspace,".cache/b/b/m"),held,100)).bytes);
 const area=(id:string)=>report.areas.find(row=>row.id===id)!;
 expect(area("t1").usedBytes).toBe((await measureCacheTreeV1(join(workspace,"r/cache/t1"),held,100)).bytes);
 expect(area("t1").budgetBytes).toBe(100000);
 expect(area("inner").measurable).toBe(false);
 expect(area("rest").usedBytes).toBeNull();
 expect(area("hex").usedBytes).toBe((await measureCacheTreeV1(join(workspace,"r/cache/deadbeef"),held,100)).bytes);
 expect(report.retired.map(row=>row.path)).toEqual(["repository:old"]);
 expect(report.undeclared.map(row=>row.path)).toEqual(["repository:e1"]);
 const sum=report.areas.filter(row=>row.root==="repository"&&row.partition===null).reduce((total,row)=>total+(row.usedBytes??0),0)+report.retired.concat(report.undeclared).filter(row=>row.root==="repository").reduce((total,row)=>total+(row.usedBytes??0),0);
 expect(report.roots.find(row=>row.id==="repository")!.usedBytes).toBe(sum);
 const narrow=await reportCacheLedgerV1(workspace,ledger,held,partitions,new Set(["t1"]),{deep:true});
 expect(narrow.areas.map(row=>row.id)).toEqual(["t1"]);
 expect(narrow.retired).toEqual([]);
 expect(await undeclaredEntriesV1(workspace,ledger,"repository")).toEqual(["e1"]);
});

test("a mounted-looking nested partition is measured through its root and the remainder is what the siblings leave",async()=>{
 reset();
 mkdirSync(join(workspace,".cache/b/a/m/x/inner"),{recursive:true});file(".cache/b/a/m/x/inner/n.bin",5000,DAY);file(".cache/b/a/m/other/o.bin",5000,DAY);
 const held=ports(),report=await reportCacheLedgerV1(workspace,ledger,held,partitions,new Set(["inner","rest"]),{deep:true});
 expect(report.areas.map(row=>row.id)).toEqual(["inner","rest"]);
 expect(report.areas[0]!.usedBytes).toBe((await measureCacheTreeV1(join(workspace,".cache/b/a/m/x/inner"),held,100)).bytes);
 expect(report.areas[1]!.measurable).toBe(true);
});

test("the command refuses unknown, repeated and misplaced options in both languages and honors the language",async()=>{
 for(const[args,english]of[
  [["clear"],"Expected repo cache status [--deep] [--budget=<size>]|gc"],
  [["gc","--deep"],"Unknown cache option: --deep"],
  [["status","--bogus"],"Unknown cache option: --bogus"],
  [["status","--json","--json"],"Repeated cache option: --json"],
  [["status","--dry-run"],"Unknown cache option: --dry-run"],
  [["gc","--area"],"Option --area needs a value"],
  [["gc","--area=nowhere"],"Unknown cache area: nowhere"],
  [["status","--json=1"],"Unknown cache option: --json=1"]
 ] as [string[],string][]){
  expect(()=>cacheArgumentsV1(args,ledger),args.join(" ")).toThrow(english);
  expect(()=>cacheArgumentsV1(args,ledger),args.join(" ")).toThrow("\n");
 }
 const parsed=cacheArgumentsV1(["gc","--dry-run","--area=t1,retired","--lang","de","--json"],ledger);
 expect(parsed.dryRun&&parsed.json&&parsed.language==="de").toBe(true);
 expect([...parsed.only!]).toEqual(["t1","retired"]);
 reset();file("r/cache/old/a.bin",100,30*DAY);
 const german:string[]=[],code=await runCacheLedgerCommandV1(["status","--lang=de"],{workspace,environment:{LANG:"en_US.UTF-8"},write:line=>german.push(line),ledger,partitions},ports());
 expect(code).toBe(0);
 expect(german[0]).toBe("Cache-Bilanz von "+workspace);
 const json:string[]=[];
 await runCacheLedgerCommandV1(["gc","--dry-run","--json"],{workspace,environment:{SEMIO_LANGUAGE:"en"},write:line=>json.push(line),ledger,partitions},ports());
 const document=JSON.parse(json.join("\n"));
 expect(document.version).toBe(1);
 expect(document.dryRun).toBe(true);
 expect(document.removed.length).toBeGreaterThan(0);
 expect(existsSync(join(workspace,"r/cache/old/a.bin"))).toBe(true);
});

test("the default status walks nothing: unmeasured sizes are unknown, recorded ones carry their age, and only deep walks and records",async()=>{
 reset();
 mkdirSync(join(workspace,".cache/b/a/m"),{recursive:true});
 file("r/cache/t1/x/y.bin",5000,DAY);file("r/cache/old/z.bin",5000,DAY);file("r/cache/e1/q.bin",5000,DAY);
 const quick=ports(),first=await reportCacheLedgerV1(workspace,ledger,quick,partitions);
 expect(quick.stages.every(stage=>stage.startsWith("status:"))).toBe(true);
 const area=(report:typeof first,id:string)=>report.areas.find(row=>row.id===id)!;
 expect(area(first,"t1")).toMatchObject({present:true,usedBytes:null,source:"unknown",recordedAtMs:null});
 expect(area(first,"t2").present).toBe(false);
 expect(first.retired).toMatchObject([{path:"repository:old",usedBytes:null,source:"unknown"}]);
 expect(first.undeclared).toMatchObject([{path:"repository:e1",usedBytes:null,source:"unknown"}]);
 expect(first.partitions.find(row=>row.id==="beta")).toMatchObject({state:"absent",usedBytes:null});
 expect(first.partitions.find(row=>row.id==="alpha")).toMatchObject({state:"directory",usedBytes:null,source:"unknown"});
 expect(first.roots.find(row=>row.id==="repository")).toMatchObject({usedBytes:0,partial:true});
 expect(existsSync(join(workspace,"r/cache",CACHE_SIZE_FILE_V1))).toBe(false);
 const deepPorts=ports(),deep=await reportCacheLedgerV1(workspace,ledger,deepPorts,partitions,undefined,{deep:true});
 expect(deepPorts.stages.some(stage=>!stage.startsWith("status:"))).toBe(true);
 expect(area(deep,"t1")).toMatchObject({source:"walked",recordedAtMs:null});
 const size=(await measureCacheTreeV1(join(workspace,"r/cache/t1"),quick,100)).bytes;
 expect((await loadCacheSizesV1(workspace,ledger)).records.t1).toEqual({usedBytes:size,partial:false,atMs:now});
 const later=ports({now:()=>now+3*HOUR}),second=await reportCacheLedgerV1(workspace,ledger,later,partitions);
 expect(later.stages.every(stage=>stage.startsWith("status:"))).toBe(true);
 expect(area(second,"t1")).toMatchObject({usedBytes:size,source:"recorded",recordedAtMs:now});
 expect(second.retired[0]).toMatchObject({source:"recorded"});
 expect(second.undeclared[0]).toMatchObject({source:"recorded"});
 expect(second.roots.find(row=>row.id==="repository")!.usedBytes).toBe(deep.roots.find(row=>row.id==="repository")!.usedBytes);
 const lines:string[]=[],code=await runCacheLedgerCommandV1(["status","--lang=de"],{workspace,environment:{SEMIO_LANGUAGE:"en"},write:line=>lines.push(line),ledger,partitions},later);
 expect(code).toBe(0);
 expect(lines.some(line=>line.includes("(vor 3 h erfasst)"))).toBe(true);
 expect(lines.some(line=>line.startsWith("Als erfasst oder nicht gemessen"))).toBe(true);
});

test("a collection records the sizes it saw and forgets what it removed, and the size ledger survives damage",async()=>{
 reset();
 file("r/cache/old/a.bin",100,30*DAY);file("r/cache/t1/ancient/x.bin",100,20*DAY);file("r/cache/t1/fine/x.bin",65536,2*DAY);file("r/cache/e1/x.bin",100,30*DAY);file("r/cache/e2/x.bin",100,2*DAY);
 const result=await collectCacheV1(workspace,ledger,{dryRun:false},ports(),lines(),partitions),records=(await loadCacheSizesV1(workspace,ledger)).records,fine=(await measureCacheTreeV1(join(workspace,"r/cache/t1"),ports(),100)).bytes;
 expect(result.failed).toEqual([]);
 expect(records.t1).toEqual({usedBytes:fine,partial:false,atMs:now});
 expect(records["retired:repository:old"]).toBeUndefined();
 expect(records["undeclared:repository:e1"]).toBeUndefined();
 expect(records["undeclared:repository:e2"]).toMatchObject({atMs:now});
 expect(readdirSync(join(workspace,"r/cache")).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 await recordCacheSizesV1(workspace,ledger,{t1:null,"x:y":{usedBytes:1,partial:true,atMs:5}});
 const keys=Object.keys((await loadCacheSizesV1(workspace,ledger)).records);
 expect(keys).toContain("x:y");
 expect(keys).toContain("undeclared:repository:e2");
 expect(keys).not.toContain("t1");
 for(const damaged of["not json","{}",'{"version":1,"records":{"a":{"usedBytes":-1,"partial":false,"atMs":1}}}','{"version":1,"records":{"a":1}}']){
  writeFileSync(join(workspace,"r/cache",CACHE_SIZE_FILE_V1),damaged);
  expect(await loadCacheSizesV1(workspace,ledger)).toEqual({version:1,records:{}});
 }
 const shallow=await reportCacheLedgerV1(workspace,ledger,ports(),partitions);
 expect(shallow.areas.find(row=>row.id==="t1")).toMatchObject({usedBytes:null,source:"unknown"});
});

test("status --deep is a flag of status only and walks with progress and cancellation",async()=>{
 reset();
 for(let index=0;index<300;index++)file("r/cache/t1/u"+index+"/f",1,DAY);
 expect(cacheArgumentsV1(["status","--deep"],ledger).deep).toBe(true);
 expect(cacheArgumentsV1(["status"],ledger).deep).toBe(false);
 const controller=new AbortController(),active=ports({progress:async stage=>{if(!stage.startsWith("status:"))controller.abort(Error("stop"));}},controller),output:string[]=[];
 const code=await runCacheLedgerCommandV1(["status","--deep"],{workspace,environment:{SEMIO_LANGUAGE:"en"},write:line=>output.push(line),ledger,partitions},active);
 expect(code).toBe(130);
 expect(output.at(-1)).toBe("Cache collection cancelled");
 expect(existsSync(join(workspace,"r/cache",CACHE_SIZE_FILE_V1))).toBe(false);
 const json:string[]=[];
 await runCacheLedgerCommandV1(["status","--deep","--json","--area=t1"],{workspace,environment:{SEMIO_LANGUAGE:"en"},write:line=>json.push(line),ledger,partitions},ports());
 expect(JSON.parse(json.join("\n")).areas[0]).toMatchObject({id:"t1",source:"walked"});
});

test("a derived partition whose image was born smaller than declared is reported as recreate pending, and a matching one is not",async()=>{
 reset();
 mkdirSync(join(workspace,".cache/b/a/m"),{recursive:true});mkdirSync(join(workspace,".cache/b/b/m"),{recursive:true});
 writeFileSync(join(workspace,".cache/b/a/image.json"),JSON.stringify({version:1,maximumPhysicalBytes:1<<29}));
 writeFileSync(join(workspace,".cache/b/b/image.json"),JSON.stringify({version:1,maximumPhysicalBytes:1<<30}));
 const report=await reportCacheLedgerV1(workspace,ledger,ports(),partitions),alpha=report.partitions.find(row=>row.id==="alpha")!,beta=report.partitions.find(row=>row.id==="beta")!;
 expect(alpha).toMatchObject({envelopeBytes:1<<29,declaredBytes:1<<30,recreate:true});
 expect(beta).toMatchObject({envelopeBytes:1<<30,declaredBytes:1<<30,recreate:false});
 const lines:string[]=[],code=await runCacheLedgerCommandV1(["status"],{workspace,environment:{SEMIO_LANGUAGE:"en"},write:line=>lines.push(line),ledger,partitions},ports());
 expect(code).toBe(0);
 expect(lines.some(line=>line.startsWith("  alpha:")&&line.includes("image is 512.00 MiB but 1.00 GiB are declared: recreate pending"))).toBe(true);
 expect(lines.some(line=>line.startsWith("  beta:")&&line.includes("recreate pending"))).toBe(false);
});
