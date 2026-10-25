/** 🌐️ Contract: the cache area projection asks each scanner exactly once for the right directory, reports progress under the right area names, hands the authored budgets, ages and guard ages to the five areas and resolves the root of a scanned unit, exactly like an independent Python derivation over the layouts of `./🔣️.json`; a cancelled control stops it before and after scanning. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {CACHE_POLICY} from "../../../../🔍️discovery/📂️source/🟦️.ts";
import {areaUnitRoot,scanCacheAreas} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(request:object):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const live=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:async()=>undefined});

function operations(layout:(typeof corpus.layouts)[number],calls:unknown[][]){
 return{
  cargoDirectories:()=>({build:layout.build,target:layout.target}),
  cacheDirectory:(root:string,area:string)=>root+"/cache/"+area,
  scanCargoBuild:async(path:string,_control:unknown,onUnit?:(unit:never)=>void)=>{calls.push(["build",path]);for(const unit of layout.scans.build)onUnit?.(unit as never);return layout.scans.build as never;},
  scanCargoTarget:async(path:string,_control:unknown,onUnit?:(unit:never)=>void)=>{calls.push(["target",path]);for(const unit of layout.scans.target)onUnit?.(unit as never);return layout.scans.target as never;},
  scanCargoIncremental:async(path:string,_control:unknown,onUnit?:(unit:never)=>void)=>{calls.push(["incremental",path]);for(const unit of [...layout.scans.incremental.units,...layout.scans.incremental.staleSessions])onUnit?.(unit as never);return layout.scans.incremental as never;},
  scanDirectories:async(path:string,_control:unknown,excluded?:ReadonlySet<string>,onUnit?:(unit:never)=>void)=>{const area=path.endsWith("/vite")?"vite":"agents";calls.push([area,path,[...(excluded??[])]]);for(const unit of layout.scans[area])onUnit?.(unit as never);return layout.scans[area] as never;},
 };
}

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,roots:[{area:"x",kind:"file",expected:"y"}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,layouts:[{...corpus.layouts[0],scans:{}}]})).toBe(false);
});

test("every layout is projected into the five authored areas with the right scanner calls and progress, like the Python derivation",async()=>{
 const expected=ask({op:"project",storage:CACHE_POLICY.storage,repoRoot:corpus.repoRoot,layouts:corpus.layouts});
 for(const[index,layout]of corpus.layouts.entries()){
  const calls:unknown[][]=[],progress:unknown[][]=[],areas=await scanCacheAreas(corpus.repoRoot,live(),(area,unit)=>{progress.push([area,unit.path]);},operations(layout,calls) as never);
  expect(JSON.parse(JSON.stringify(areas)),layout.name).toEqual(expected[index].areas);expect(calls,layout.name).toEqual(expected[index].calls);expect(progress,layout.name).toEqual(expected[index].progress);
  expect(areas.map(area=>area.name),layout.name).toEqual(["cargo","cargo-incremental","cargo-incremental-sessions","vite","agents"]);
 }
});

test("a cancelled control stops the projection before any scanner runs, and a control cancelled during scanning stops it afterwards",async()=>{
 const layout=corpus.layouts[0]!,calls:unknown[][]=[],stopped=new AbortController();stopped.abort(Error("stop first"));
 await expect(scanCacheAreas(corpus.repoRoot,{...live(),signal:stopped.signal} as never,undefined,operations(layout,calls) as never)).rejects.toThrow("stop first");expect(calls).toEqual([]);
 const late=new AbortController(),control={...live(),signal:late.signal} as never,ops=operations(layout,calls),original=ops.scanDirectories;
 ops.scanDirectories=async(...args:Parameters<typeof original>)=>{const result=await original(...args);if(String(args[0]).endsWith("/agents"))late.abort(Error("stop after"));return result;};
 await expect(scanCacheAreas(corpus.repoRoot,control,undefined,ops as never)).rejects.toThrow("stop after");
});

test("the root of a scanned unit follows its area and kind, equal to the Python derivation",()=>{
 const answers=ask({op:"root",repoRoot:corpus.repoRoot,rows:corpus.roots}),ops={cargoDirectories:()=>({build:"/repo/b",target:"/repo/t"}),cacheDirectory:(root:string,area:string)=>root+"/cache/"+area};
 for(const[index,row]of corpus.roots.entries()){
  const root=areaUnitRoot(corpus.repoRoot,row.area,{path:"x",bytes:1,recencyMs:1,lockHeld:false,kind:row.kind} as never,ops as never),expected=row.expected==="build"?"/repo/b":row.expected==="target"?"/repo/t":corpus.repoRoot+"/cache/"+row.expected.slice(6);
  expect(root,JSON.stringify(row)).toBe(expected);expect(answers[index],JSON.stringify(row)).toBe(expected);
 }
});

