import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {encodeProtobufV1,REAPI_V1} from "../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {parseRemoteCacheStableResultV1} from "../../🔬️action-result/🟦️.ts";
import {openProvenanceV1,PROVENANCE_ADVISORY_OUTPUTS_V1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceClassV1,provenanceDifferingV1,provenanceFingerprintV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🧾️ The fingerprint ignores the manifest names that carry timestamps, and the consumption record counts who read whose results. The fingerprint cases are one shared fixture checked by an independent Python implementation. */
const here=fileURLToPath(new URL(".",import.meta.url)),scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const directory=()=>{const path=mkdtempSync(join(tmpdir(),"prov-"));scratch.push(path);return path;};

test("the fingerprint of an action result leaves out the volatile manifest and equals the independent Python computation on every fixture case",()=>{
 expect(PROVENANCE_VOLATILE_OUTPUTS_V1).toEqual(fixture.volatile);
 for(const row of fixture.cases){
  const bytes=encodeProtobufV1(REAPI_V1.actionResult,{output_files:row.outputs.map(output=>({path:output.path,digest:{hash:output.hash,size_bytes:3}})),exit_code:0});
  const parsed=parseRemoteCacheStableResultV1(bytes,PROVENANCE_VOLATILE_OUTPUTS_V1);
  expect(provenanceFingerprintV1([...parsed.stable,...parsed.trees].map(digest=>digest.hash)),row.name).toBe(row.fingerprint);
  expect(parsed.manifest,row.name).toBe(row.manifest);
 }
 const python=spawnSync("python3",["-I",resolve(here,"🐍️.py"),resolve(here,"🧫️fixtures/🔣️.json")],{encoding:"utf8"});
 if(python.error===undefined)expect(python.stdout.trim(),python.stderr).toBe(`ok ${fixture.cases.length}\nok classes ${fixture.classes.length} differing ${fixture.differing.length}`);
});

test("the class of a result and the paths that differ between two results equal the independent Python computation on every fixture case",()=>{
 expect(PROVENANCE_ADVISORY_OUTPUTS_V1).toEqual(fixture.advisory);
 for(const row of fixture.classes)expect(provenanceClassV1(row.paths,row.manifest),row.name).toEqual({sort:row.sort,advisory:row.advisory});
 for(const row of fixture.differing)expect(provenanceDifferingV1(row.kept,row.refused),row.name).toEqual(row.differing);
});

test("the divergence count is the lifetime total beyond the 1000 kept rows, the count per class and the first time survive a restart, and a note counts as advice, not as divergence",async()=>{
 const root=directory();let now=5000;
 const first=await openProvenanceV1(root,{now:()=>now}),row=(index:number,sort:string)=>({namespace:"main",kind:"ac" as const,key:String(index).padStart(64,"0"),keptOrigin:"g1-m01",keptSha256:"a".repeat(64),refusedOrigin:"g1-m02",refusedSha256:"b".repeat(64),sort,differing:["out/x.rlib"]});
 for(let index=0;index<1100;index++)first.diverged(row(index,index%4===0?"unit":"project-inference"));
 first.advised("failure-note");first.advised("failure-note");first.advised("costs");
 expect(first.summary().count).toBe(1100);expect(first.summary().byClass).toEqual({"project-inference":825,unit:275});expect(first.summary().advisory).toEqual({costs:1,"failure-note":2});
 await first.flush();
 const file=JSON.parse(readFileSync(join(root,"fleet","divergences.json"),"utf8")) as {rows:unknown[];total:number};expect(file.rows.length).toBe(1000);expect(file.total).toBe(1100);
 now=9000;const second=await openProvenanceV1(root,{now:()=>now});
 expect(second.summary().count).toBe(1100);expect(second.summary().byClass).toEqual({"project-inference":825,unit:275});expect(second.summary().since).toBe(5000);expect(second.summary().advisory).toEqual({});
 expect(second.summary().recent.at(-1)!.differing).toEqual(["out/x.rlib"]);
 second.diverged(row(5000,"unit"));await second.flush();expect(second.summary().count).toBe(1101);expect(second.summary().byClass.unit).toBe(276);
});

test("a restart starts a new current window: the lifetime count keeps the history of fixed code, current counts only the divergences of this run, and the last hour slides over the kept rows (checked against a naive filter of the file)",async()=>{
 const root=directory();let now=10_000_000;
 const row=(index:number,sort:string)=>({namespace:"main",kind:"ac" as const,key:String(index).padStart(64,"0"),keptOrigin:"g1-m01",keptSha256:"a".repeat(64),refusedOrigin:"g1-m02",refusedSha256:"b".repeat(64),sort,differing:["x"]});
 const first=await openProvenanceV1(root,{now:()=>now});
 for(let index=0;index<6;index++){first.diverged(row(index,index<4?"unit":"owner-pool"));now+=1000;}
 expect(first.summary().current).toEqual({since:10_000_000,count:6,byClass:{"owner-pool":2,unit:4}});
 await first.flush();
 now+=7_200_000;const second=await openProvenanceV1(root,{now:()=>now});
 expect(second.summary().count).toBe(6);expect(second.summary().current).toEqual({since:now,count:0,byClass:{}});expect(second.summary().lastHour).toEqual({since:0,count:0,byClass:{}});
 now+=500;second.diverged(row(9,"closure"));await second.flush();
 expect(second.summary().current).toEqual({since:now-500,count:1,byClass:{closure:1}});expect(second.summary().count).toBe(7);
 const file=JSON.parse(readFileSync(join(root,"fleet","divergences.json"),"utf8")) as {rows:{atMs:number;sort:string}[]};
 const naive:Record<string,number>={};for(const entry of file.rows)if(entry.atMs>=now-3_600_000)naive[entry.sort]=(naive[entry.sort]??0)+1;
 expect(second.summary().lastHour.byClass).toEqual(naive);expect(second.summary().lastHour.count).toBe(1);
 now+=3_600_001;expect(second.summary().lastHour.count).toBe(0);expect(second.summary().count).toBe(7);
});

test("consumption counts producer>consumer pairs with the compiled units among them, survives a restart and stays bounded",async()=>{
 const root=directory();let now=1000;
 const first=await openProvenanceV1(root,{now:()=>now});
 first.record("main","ac","unit","g1-m01","f1",true);first.record("main","ac","record","g1-m02","f2");
 first.consume("main","ac","unit","g1-m02");first.consume("main","ac","unit","g1-m02");first.consume("main","ac","unit","g1-m01");
 first.consume("main","ac","record","g1-m01");first.consume("main","ac","gone","static");
 expect(first.consumption()).toEqual({since:1000,total:5,pairs:{"g1-m01>g1-m02":{hits:2,units:2},"g1-m01>g1-m01":{hits:1,units:1},"g1-m02>g1-m01":{hits:1,units:0},"unknown>static":{hits:1,units:0}}});
 await first.flush();
 expect(existsSync(join(root,"fleet","consumption.json"))).toBe(true);expect(JSON.parse(readFileSync(join(root,"fleet","consumption.json"),"utf8")).total).toBe(5);
 now=2000;const second=await openProvenanceV1(root,{now:()=>now});
 expect(second.consumption()).toEqual(first.consumption());
 second.consume("main","ac","unit","g1-m02");expect(second.consumption().pairs["g1-m01>g1-m02"]).toEqual({hits:3,units:3});expect(second.consumption().since).toBe(1000);
 for(let index=0;index<1100;index++)second.consume("main","ac","unit","c"+index);
 const pairs=Object.keys(second.consumption().pairs);expect(pairs.length).toBeLessThanOrEqual(1025);expect(second.consumption().pairs["*>*"]!.hits).toBeGreaterThan(0);
 expect(second.consumption().total).toBe(5+1+1100);
 await second.flush();
});

test("evidence keeps both sides of a divergence, replaces one per key and origin, keeps the newest 64 per class and survives a restart",async()=>{
 const root=directory();let now=100;
 const first=await openProvenanceV1(root,{now:()=>now}),blobs=new Map<string,Uint8Array>([["h1",new Uint8Array([1,2,3])],["h2",new Uint8Array([4,5,6])]]);
 const input=(key:string,sort:string)=>({outcome:"refused" as const,sort,namespace:"main",key,keptOrigin:"g1-m01",refusedOrigin:"g1-m02",differing:["out/x.rlib"],kept:new Uint8Array([9]),refused:new Uint8Array([8]),keptRows:[{path:"out/x.rlib",hash:"h1"}],refusedRows:[{path:"out/x.rlib",hash:"h2"}],blob:async(hash:string)=>blobs.get(hash)});
 await first.evidence.keep(input("a".repeat(64),"unit"));
 const entry=first.evidence.list()[0]!;
 expect(entry.differing).toEqual(["out/x.rlib"]);expect(entry.files.map(file=>file.name).sort()).toEqual(["kept.1","kept.ac","refused.1","refused.ac"]);
 expect(Array.from(await first.evidence.read(entry.id,"kept.1")!)).toEqual([1,2,3]);expect(Array.from(await first.evidence.read(entry.id,"refused.1")!)).toEqual([4,5,6]);
 expect(await first.evidence.read(entry.id,"../meta.json")).toBeUndefined();expect(await first.evidence.read("nothing","kept.ac")).toBeUndefined();
 now=200;await first.evidence.keep(input("a".repeat(64),"unit"));expect(first.evidence.list().length).toBe(1);
 for(let index=0;index<70;index++){now=300+index;await first.evidence.keep(input(String(index).padStart(64,"0"),"project-inference"));}
 expect(first.evidence.summary().byClass).toEqual({"project-inference":64,unit:1});
 const second=await openProvenanceV1(root,{now:()=>now});expect(second.evidence.summary().count).toBe(65);expect(second.summary().evidence.count).toBe(65);
});
