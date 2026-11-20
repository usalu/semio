import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import Ajv from"ajv";
import{admitCargoSourceBoundariesV1,mergeCargoCfgFeaturesV1}from"../../🟦️.ts";
import{createRustSourceAnalysisV1,readRustSourceClosureV1}from"../../../🟦️.ts";
import{readCargoCompilationGraphV1}from"../../../../🕸️units/🟦️.ts";
import graphCorpus from"../../../../🕸️units/🧪️tests/🧫️fixtures/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import type{ResourceLeaseControl}from"../../../../../../🔒️leases/🟦️.ts";

/** 🏎️ Compiler cfg is captured once per feature-free invocation and ahead of the reads, and analysed Source is shared per cfg; the answers stay those of one compiler invocation per unit. */
const unquote=(text:string)=>text.trim().split("\n").map(value=>value.replace(/=(".*")$/u,(_,quoted)=>"="+JSON.parse(quoted)));
const control:ResourceLeaseControl={signal:new AbortController().signal,remainingMilliseconds:()=>60000,publish(){},async yieldContinuation(){}};

test("merging features into the feature-free compiler cfg equals the compiler invocation with those features",()=>{
 expect(new Ajv({strict:true}).validate({type:"array",items:{type:"array",items:{type:"string"}}},corpus.featureSets)).toBe(true);
 const base=spawnSync("rustc",["--print","cfg"],{encoding:"utf8"}).stdout.trim().split("\n");
 for(const features of corpus.featureSets){const unique=[...new Set(features)],actual=spawnSync("rustc",["--print","cfg",...unique.flatMap(feature=>["--cfg","feature="+JSON.stringify(feature)])],{encoding:"utf8"});expect(actual.status).toBe(0);expect([...mergeCargoCfgFeaturesV1(base,features)],JSON.stringify(features)).toEqual(unquote(actual.stdout));}
});

test("admission hands the compiler cfg of exactly the not remembered units to the port before any read",async()=>{
 const{units,remembered}=corpus.prefetch,raw=graphCorpus.cases[graphCorpus.identicalUnits.case]!.units[graphCorpus.identicalUnits.unit]!,graph=readCargoCompilationGraphV1({version:1,units:Array.from({length:units},()=>({...structuredClone(raw),dependencies:[]})),roots:Array.from({length:units},(_,index)=>index)},graphCorpus.metadata,"/repo");graph.units.forEach((unit,index)=>{unit.compilerFlags=["--cfg","flag"+index];});
 const events:string[]=[],handed:string[]=[],known=new Set(remembered.map(index=>graph.units[index]!.id)),memo={recall:(unit:{id:string})=>known.has(unit.id)?{inputs:[],cfg:[],environment:null}:undefined,remember(){}};
 await admitCargoSourceBoundariesV1(graph,"/repo",{version:1,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100},control,{prefetch(cold){events.push("prefetch");handed.push(...cold.map(unit=>unit.compilerFlags[1]!));},async recheck(){},async cfg(unit){events.push("cfg");return[unit.compilerFlags[1]!];},async read(){events.push("read");return new TextEncoder().encode("pub const VALUE:u8=1;");}},memo as never);
 expect(events[0]).toBe("prefetch");expect(events.filter(event=>event==="prefetch").length).toBe(1);expect(handed).toEqual(graph.units.flatMap((_,index)=>remembered.includes(index)?[]:["flag"+index]));expect(events.filter(event=>event==="cfg").length).toBe(units-remembered.length);
});

test("analysed Source is shared per cfg and never across cfg",async()=>{
 const{source,inner,withFlag,withoutFlag}=corpus.analysis,files=new Map([["src/lib.rs",source],["src/inner.rs",inner]]),read=async(path:string)=>{const text=[...files].find(([name])=>path.replaceAll("\\","/").endsWith(name))?.[1];return text===undefined?null:new TextEncoder().encode(text);},analysis=createRustSourceAnalysisV1(),run=(cfg:string[],memo?:ReturnType<typeof createRustSourceAnalysisV1>)=>readRustSourceClosureV1(["/repo/src/lib.rs"],"/repo",cfg,read,undefined,memo);
 expect((await run(["flag"],analysis)).length).toBe(withFlag);expect((await run([],analysis)).length).toBe(withoutFlag);expect((await run(["flag"],analysis)).length).toBe(withFlag);expect((await run([],analysis)).length).toBe(withoutFlag);
 expect(await run(["flag"],analysis)).toEqual(await run(["flag"]));
});
