import{test,expect}from"bun:test";
import{admitCargoSourceBoundariesV1,type CargoSourceAdmissionMemoV1}from"../../🟦️.ts";
import{readCargoCompilationGraphV1}from"../../../../🕸️units/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import graphCorpus from"../../../../🕸️units/🧪️tests/🧫️fixtures/🔣️.json"with{type:"json"};
import corpus from"../🔣️.json"with{type:"json"};

/** 🧠️ A unit whose admission was remembered reads no Source, asks for no compiler cfg and takes no durable recheck; its computed fields come back exactly, and only a miss reads again. */
const policy={version:1 as const,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100};
function graphOf(units:number){const raw=graphCorpus.cases[graphCorpus.identicalUnits.case]!.units[graphCorpus.identicalUnits.unit]!;const graph=readCargoCompilationGraphV1({version:1,units:Array.from({length:units},()=>({...structuredClone(raw),dependencies:[]})),roots:Array.from({length:units},(_,index)=>index)},graphCorpus.metadata,"/repo");graph.units.forEach((unit,index)=>{unit.compilerFlags=["--cfg","flag"+index];});return graph;}

test("a remembered unit admission skips reads, cfg probes and durable rechecks and restores the computed fields",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"cargo:memo-source-admission",0),store=new Map<string,Parameters<CargoSourceAdmissionMemoV1["recall"]> extends never?never:ReturnType<CargoSourceAdmissionMemoV1["recall"]>>(),remembered:Array<{probed:readonly string[];startedMs:number}>=[];
 const memo:CargoSourceAdmissionMemoV1={recall:unit=>store.get(unit.id),remember:(unit,_policy,result)=>{remembered.push({probed:result.probed,startedMs:result.startedMs});store.set(unit.id,{inputs:result.inputs.map(input=>({...input})),cfg:[...result.cfg],environment:result.environment});}};
 const counts=()=>({cfg:0,reads:0,units:0,complete:0});
 const run=async(count:ReturnType<typeof counts>,graph:ReturnType<typeof graphOf>)=>admitCargoSourceBoundariesV1(graph,"/repo",policy,control,{async recheck(_control,phase){if(phase==="unit")count.units++;else count.complete++;},async cfg(unit){count.cfg++;return[unit.compilerFlags[1]!];},async read(){count.reads++;return new TextEncoder().encode(corpus.source);}},memo);
 const first=graphOf(3),cold=counts();await run(cold,first);
 expect(cold.cfg).toBe(3);expect(cold.reads).toBeGreaterThanOrEqual(3);expect(cold.units).toBe(3);expect(remembered.length).toBe(3);expect(remembered.every(row=>row.probed.length>0&&row.startedMs>0)).toBe(true);
 const second=graphOf(3),warm=counts();await run(warm,second);
 expect(warm.cfg).toBe(0);expect(warm.reads).toBe(0);expect(warm.units).toBe(0);expect(warm.complete).toBe(1);
 for(const [index,unit] of second.units.entries()){expect(unit.sourceInputs).toEqual(first.units[index]!.sourceInputs);expect(unit.compilerCfg).toEqual(first.units[index]!.compilerCfg);expect(unit.compilerEnvironment).toEqual(first.units[index]!.compilerEnvironment);}
 store.delete(second.units[1]!.id);const partial=counts();await run(partial,graphOf(3));
 expect(partial.cfg).toBe(1);expect(partial.units).toBe(1);
}));

import Ajv from "ajv";
import {Database} from "bun:sqlite";
import rows from "./🔣️.json" with {type:"json"};
import type {ResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";

test("neutral memo reuse preserves source-change refusal, completion custody and cancellation",async()=>{
 const validate=new Ajv({strict:true}).compile({"type":"object","required":["version","cases"],"additionalProperties":false,"properties":{"version":{"const":1},"cases":{"type":"array","items":{"type":"object","additionalProperties":false,"required":["id","change","abort","oversize","reads","refuses"],"properties":{"id":{"type":"string"},"change":{"type":"boolean"},"abort":{"type":"boolean"},"oversize":{"type":"boolean"},"reads":{"type":"integer","minimum":0},"refuses":{"type":"boolean"}}}}}});
 expect(validate(rows),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");try{db.exec("create table expected(id text,reads integer,refuses integer)");
 for(const row of rows.cases){db.query("insert into expected values(?,?,?)").run(row.id,row.reads,Number(row.refuses));let revision=0,savedRevision=-1,saved:ReturnType<CargoSourceAdmissionMemoV1["recall"]>,reads=0,complete=0;
 const abort=new AbortController(),control:ResourceLeaseControl={signal:abort.signal,remainingMilliseconds:()=>60000,publish(){},async yieldContinuation(){}},memo:CargoSourceAdmissionMemoV1={recall:()=>revision===savedRevision?saved:undefined,remember(_unit,_policy,result){savedRevision=revision;saved={inputs:result.inputs,cfg:result.cfg,environment:result.environment};}},port={async recheck(_control:ResourceLeaseControl,phase:string){if(phase==="complete")complete++;},async cfg(){return[];},async read(){reads++;return new TextEncoder().encode(row.oversize&&revision?"x".repeat(policy.maxSourceBytes+1):corpus.source);}};
 await admitCargoSourceBoundariesV1(graphOf(1),"/repo",policy,control,port,memo);reads=0;complete=0;if(row.change)revision++;const reason=Error("client EOF");if(row.abort)abort.abort(reason);
 let failure:unknown;try{await admitCargoSourceBoundariesV1(graphOf(1),"/repo",policy,control,port,memo);}catch(error){failure=error;}
 const expected=db.query<{reads:number;refuses:number},[string]>("select reads,refuses from expected where id=?").get(row.id)!;
 expect(reads,row.id).toBe(expected.reads);expect(failure!==undefined,row.id).toBe(Boolean(expected.refuses));if(row.abort)expect(failure).toBe(reason);if(!row.refuses)expect(complete).toBe(1);
 }}finally{db.close();}
});

test("source reads preserve an exact cancellation reason and contextualize genuine read failures",async()=>{
 for(const cancelled of ["same","none","other"]){const abort=new AbortController(),reason=Error("reader failed"),control:ResourceLeaseControl={signal:abort.signal,remainingMilliseconds:()=>60000,publish(){},async yieldContinuation(){}};
 let failure:unknown;try{await admitCargoSourceBoundariesV1(graphOf(1),"/repo",policy,control,{async recheck(){},async cfg(){return[];},async read(){if(cancelled!=="none")abort.abort(cancelled==="same"?reason:Error("separate EOF"));throw reason;}});}catch(error){failure=error;}
 if(cancelled==="same")expect(failure).toBe(reason);else{expect(failure).toBeInstanceOf(Error);expect((failure as Error).cause).toBe(reason);expect((failure as Error).message).toContain("Source refused");}
 }
});

import ts from "typescript";
import {readFileSync} from "node:fs";

test("the actual artifact plan forwards one memo through initial admission and every guarded recheck",async()=>{
 const text=readFileSync(new URL("../../../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("artifact.ts",text,ts.ScriptTarget.Latest,true),definition=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="withCargoArtifactPlanV1")!;
 expect(definition).toBeDefined();
 const abort=new AbortController(),control={signal:abort.signal},memo={},calls:unknown[]=[],checks={storage:0,lease:0,recipe:0,facts:0},storage={allocation:{kind:"compiler-workspace"},backing:{root:"/owned"},async recheck(){checks.storage++;}},environment={CARGO_HOME:"/owned/cargo"},recipe={recipe:{manifest:"/owned/Cargo.toml",buildDirectory:"/owned/build"},async recheck(){checks.recipe++;}};
 const ports={requireNativeStorageReservationV1:()=>storage,requireIssuedResourceControlV1(){},CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1:1024,admitArtifactPolicy:(value:unknown)=>value,cargoArtifactArgumentsV1:(value:unknown)=>value,isAbsolute:()=>true,requestedCompilerAdmissionControlV1:()=>control,requireCargoCompilerPathEnvelopeV1(){},requireNativeStoragePathV1(_root:string,path:string){return path;},resolve:(...parts:string[])=>parts.join("/"),dirname:()=>"/owned",join:(...parts:string[])=>parts.join("/"),mkdirSync(){},cargoBuildOwnerAliveV1:()=>true,withCargoArtifactPlanInputsV1:async(input:any,_port:any,body:any)=>body({...input,requireProduction(){}}),retryOnConcurrentEditV1:async(_kind:string,body:any)=>body(),captureCargoCompilationGraphV1:async()=>({units:[]}),actualCargoCompilerPlanningPortV1:()=>({}),admitCargoSourceBoundariesV1:async(...args:unknown[])=>{calls.push(args[5]);},actualCargoSourceAdmissionPortV1:()=>({}),cargoArtifactTargetRecipeV1:async()=>recipe,captureCargoArtifactPlanFactsV1:async()=>{checks.facts++;return{platform:"linux"};},cargoArtifactPlanIdentityV1:()=>"same",checkResourceLeaseControl(){checks.lease++;},withIssuedResourceOperationV1:async(_control:any,body:any)=>body(),captureSourceSchema:{properties:{maximumPendingOperations:{const:32}}}};
 const javascript=ts.transpileModule(definition.getText(tree).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText,run=new Function(...Object.keys(ports),javascript+";return withCargoArtifactPlanV1;")(...Object.values(ports));
 await run("/owned/Cargo.toml",[],{budgetMs:0,cwd:"/owned",buildDirectory:"/owned/build",captureDirectory:"/owned/capture",mode:"prototype"},{storage,control,environment,consume:async()=>{},maximumObservationBytes:1024,sourceBoundary:policy,sourceMemo:memo,observationOutput:{directory:"/owned/observe"}},async(plan:any)=>{await plan.recheck();await plan.recheck();});
 expect(calls).toEqual([memo,memo,memo]);expect(checks.storage).toBeGreaterThan(1);expect(checks.lease).toBeGreaterThan(1);expect(checks.recipe).toBe(2);expect(checks.facts).toBe(3);
});
