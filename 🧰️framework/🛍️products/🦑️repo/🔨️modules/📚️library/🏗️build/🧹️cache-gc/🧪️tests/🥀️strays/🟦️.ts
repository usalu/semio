/** 🥀️ Law: cache roots that tools left below member directories are found by git and by an independent directory walk alike, and the collector empties them only when nothing holds them. */
import {Database} from "bun:sqlite";
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative} from "node:path";
import {cacheLedgerV1,cacheStrayNamesV1,readCacheLedgerV1,type CacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheMountPointsV1,cacheRunnerPortsV1,cacheStrayRootsV1,collectCacheV1,type CacheRunnerPortsV1} from "../../🏃️runner/🟦️.ts";

const HOUR=3600000,DAY=24*HOUR,reason={en:"x",de:"y"},purpose={en:"p",de:"q"},now=Date.now();
const ledger:CacheLedgerV1=readCacheLedgerV1({
 version:1,
 roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]},rust:{directory:".r",maximumBytes:0,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:HOUR,ignored:[]}},
 areas:[{id:"t1",root:"repository",kind:"tree",directory:"t1",budget:{bytes:100000},collect:{unusedAgeMs:10*DAY,quietMs:HOUR,custody:"quiet"},writers:["w"],purpose}],
 retirements:[],
 strays:{quietMs:HOUR,ignoredSegments:["node_modules",".git"],excludedDirectories:[".semio-tickets"],reason:{en:"Stray cache roots found.",de:"Verirrte Cache-Wurzeln gefunden."}}
},{});
let workspace="";
const file=(path:string,ageMs:number,size=10):string=>{const full=join(workspace,path);mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"x".repeat(size));const time=(now-ageMs)/1000;utimesSync(full,time,time);return full;};
const ports=(extra:Partial<CacheRunnerPortsV1>={}):CacheRunnerPortsV1=>cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>({release:()=>{}}),mountPoints:async()=>[],...extra});
const lines=():{write(line:string):void;language:"en"|"de";out:string[]}=>{const out:string[]=[];return{write:line=>{out.push(line);},language:"en",out};};
const walk=(directory:string,found:string[]=[]):string[]=>{
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  if(!entry.isDirectory()||entry.name===".git")continue;
  const path=join(directory,entry.name);
  if(entry.name===".cache"||entry.name===".r")found.push(relative(workspace,path).split("\\").join("/"));
  if(entry.name!=="node_modules"&&entry.name!==".semio-tickets")walk(path,found);
 }
 return found;
};
const lay=():void=>{
 rmSync(workspace,{recursive:true,force:true});mkdirSync(workspace,{recursive:true});
 writeFileSync(join(workspace,".gitignore"),"*.[Cc]ache\n.r\nnode_modules/\n");
 file(".cache/b/p/root.bin",30*DAY);file("r/cache/t1/u/keep.bin",2*DAY);
 file("m/x/.cache/b/t/old.bin",30*DAY,100);file("m/x/.cache/boot-recovery/a.jsonl",30*DAY);file("m/x/.cache/young.bin",60000);
 mkdirSync(join(workspace,"m/y/.cache"),{recursive:true});
 file("m/.r/q/x.bin",30*DAY);
 file("node_modules/p/.cache/z.bin",30*DAY);
 file(".semio-tickets/t/.cache/w.bin",30*DAY);
 mkdirSync(join(workspace,"m/z/.cache/held"),{recursive:true});file("m/mnt/.cache/vol/a.bin",30*DAY);
 file("m/x/notcache/.cachex/ok.bin",30*DAY);
 mkdirSync(join(workspace,"m/lnk"),{recursive:true});try{symlinkSync(join(workspace,"m/x"),join(workspace,"m/lnk/.cache"));}catch{}
 spawnSync("git",["init","-q"],{cwd:workspace});
};
beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"strays-"));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the stray names are the single-segment root directories of the ledger",()=>{
 expect(cacheStrayNamesV1(ledger)).toEqual([".cache",".r"]);
 expect(()=>readCacheLedgerV1({...ledger,strays:{...ledger.strays,extra:1}},{})).toThrow("Closed cache ledger required");
 const {strays:_,...bare}=ledger;expect(readCacheLedgerV1(bare,{}).strays).toBeUndefined();
});

test("git and an independent directory walk find the same stray roots, never the root itself, ignored segments, excluded directories or links",async()=>{
 lay();
 const found=await cacheStrayRootsV1(workspace,ledger,new AbortController().signal),oracle=walk(workspace).filter(path=>path.includes("/")).sort();
 expect(found).toEqual(oracle);
 expect(found).toEqual(["m/.r","m/mnt/.cache","m/x/.cache","m/y/.cache","m/z/.cache"]);
 expect(await cacheStrayRootsV1(workspace,{...ledger,strays:undefined},new AbortController().signal)).toEqual([]);
 const stopped=new AbortController();stopped.abort();expect(await cacheStrayRootsV1(workspace,ledger,stopped.signal)).toEqual([]);
});

test("the dry run lists what a stray root holds and removes nothing; the real run removes quiet and unheld entries, empties the root and keeps held, mounted and young ones",async()=>{
 lay();
 const held=join(workspace,"m/z/.cache/held/a.sqlite"),lock=new Database(held);lock.exec("PRAGMA journal_mode=DELETE;CREATE TABLE t(x)");utimesSync(held,(now-30*DAY)/1000,(now-30*DAY)/1000);lock.exec("BEGIN EXCLUSIVE");
 const mounts=[join(workspace,"m/mnt/.cache/vol")];
 try{
  const preview=lines(),planned=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["retired"])},ports({mountPoints:async()=>mounts}),preview);
  expect(planned.removed.map(row=>row.target+"/"+row.name).sort()).toEqual(["stray:m/.r/q","stray:m/x/.cache/b","stray:m/x/.cache/boot-recovery"]);
  expect(preview.out).toContain("Stray cache roots found.");
  expect(existsSync(join(workspace,"m/x/.cache/b/t/old.bin"))).toBe(true);
  const out=lines(),real=await collectCacheV1(workspace,ledger,{dryRun:false},ports({mountPoints:async()=>mounts}),out);
  expect(real.failed).toEqual([]);
  expect(existsSync(join(workspace,"m/x/.cache/b"))).toBe(false);expect(existsSync(join(workspace,"m/x/.cache/boot-recovery"))).toBe(false);
  expect(existsSync(join(workspace,"m/x/.cache/young.bin"))).toBe(true);
  expect(existsSync(join(workspace,"m/.r"))).toBe(false);expect(existsSync(join(workspace,"m/y/.cache"))).toBe(false);
  expect(existsSync(join(workspace,"m/z/.cache/held/a.sqlite"))).toBe(true);
  expect(existsSync(join(workspace,"m/mnt/.cache/vol/a.bin"))).toBe(true);
  expect(out.out.some(line=>line.includes("mount:"))).toBe(true);
  expect(existsSync(join(workspace,"r/cache/t1/u/keep.bin"))).toBe(true);
  expect(existsSync(join(workspace,"node_modules/p/.cache/z.bin"))).toBe(true);expect(existsSync(join(workspace,".semio-tickets/t/.cache/w.bin"))).toBe(true);
  expect(out.out.some(line=>line.includes("sqlite:"))).toBe(true);
 }finally{lock.exec("ROLLBACK");lock.close();}
});

test("a collection that does not name retired leaves stray roots alone and the mount table lists this machine's mounts",async()=>{
 lay();
 const result=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["t1"])},ports(),lines());
 expect(result.removed).toEqual([]);expect(existsSync(join(workspace,"m/x/.cache/b/t/old.bin"))).toBe(true);
 const mounts=await cacheMountPointsV1();
 if(process.platform==="win32")expect(mounts).toEqual([]);else expect(mounts.length).toBeGreaterThan(0);
});

test("the declared ledger carries the stray rule in both languages and never looks into the ticket tree",()=>{
 const rule=cacheLedgerV1().strays!;
 expect(rule.excludedDirectories).toContain(".🧬semio");expect(rule.ignoredSegments).toContain("node_modules");
 expect(rule.reason.en.length).toBeGreaterThan(0);expect(rule.reason.de.length).toBeGreaterThan(0);
 expect(cacheStrayNamesV1(cacheLedgerV1())).toEqual([".cache",".r"]);
});
