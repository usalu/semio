import{rustCompilerExecutionEnvironmentV1}from"../../🕸️units/📦️package/🌱️environment/🟦️.ts";
import {rustCompileEnvironmentReferencesV1} from "../../🕸️units/📦️package/🌱️environment/📖️references/🟦️.ts";
import {dirname,relative,isAbsolute,resolve,sep} from "node:path";
import {captureOwnedCommandV1} from "../../../../🎛️owned-execution/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {createRustSourceAnalysisV1,readRustSourceClosureV1} from "../🟦️.ts";
import {noteScriptProcessActivityV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {refuseProcessV1} from "../../../../📖️texts/🟦️.ts";
import {isRepoErrorV1} from "../../../../🏗️build/🗣️language/🚨️error/🟦️.ts";
import type {CargoCompilationGraphV1,CargoCompilationUnitV1} from "../../🕸️units/🟦️.ts";
import {actualCargoCompilerPlanningPortV1,captureCargoCompilationGraphV1} from "../../🕸️units/📥️capture/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../💾️storage/🟦️.ts";
import {createPhysicalCargoSourceReaderV1,type CargoSourceReadRequestV1} from "./📖️read/🟦️.ts";
import type {CargoSourceAdmissionWitnessV1} from "../../🧾️receipt/📎️source-witness/🟦️.ts";
import type {CargoAdmittedSourceV1} from "../👁️drift/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
export interface CargoSourceBoundaryPolicyV1 {readonly version:1;readonly maxSourceBytes:number;readonly maxSourceFiles:number;readonly maxActiveLines:number}
/** 📐️ Resolves a crate's authored ceiling against the repository compiler admission policy. */
export function effectiveCargoSourceBoundaryV1(policy:CargoSourceBoundaryPolicyV1,boundary:CargoCompilationUnitV1["boundary"]):CargoSourceBoundaryPolicyV1 {
 if(!boundary)return policy;
 return {version:1,maxSourceBytes:Math.min(policy.maxSourceBytes,boundary.maxSourceBytes),maxSourceFiles:Math.min(policy.maxSourceFiles,boundary.maxSourceFiles),maxActiveLines:Math.min(policy.maxActiveLines,boundary.maxActiveLines)};
}
/** 📏️ Reports every exhausted count from one observed Cargo Source closure. */
export function cargoSourceBoundaryExcessV1(limits:CargoSourceBoundaryPolicyV1,observed:Readonly<{files:number;bytes:number;lines:number}>):readonly ("files"|"bytes"|"lines")[] {
 const exceeded:("files"|"bytes"|"lines")[]=[];
 if(observed.files>limits.maxSourceFiles)exceeded.push("files");
 if(observed.bytes>limits.maxSourceBytes)exceeded.push("bytes");
 if(observed.lines>limits.maxActiveLines)exceeded.push("lines");
 return exceeded;
}
export interface CargoSourceAdmissionProgressV1 {readonly version:1;readonly phase:"cfg"|"read"|"complete";readonly unit:number;readonly units:number;readonly packageName:string;readonly readOperations:number;readonly files:number;readonly bytes:number}
export type CargoSourceAdmissionRecheckPhaseV1="unit"|"complete";
/** 🧠️ Remembers whole-unit admissions across invocations. Component provenance obtains bodies separately when it needs retained Source witnesses. */
export interface CargoSourceAdmissionMemoV1 {recall(unit:CargoCompilationUnitV1,policy:unknown):Readonly<{inputs:readonly CargoCompilationUnitV1["sourceInputs"][number][];cfg:readonly string[];environment:CargoCompilationUnitV1["compilerEnvironment"]}>|undefined;remember(unit:CargoCompilationUnitV1,policy:unknown,result:Readonly<{inputs:readonly CargoCompilationUnitV1["sourceInputs"][number][];cfg:readonly string[];environment:CargoCompilationUnitV1["compilerEnvironment"];probed:readonly string[];startedMs:number}>):void}
export interface CargoSourceAdmissionPortV1 {prefetch?(units:readonly CargoCompilationUnitV1[],control:ResourceLeaseControl):void;cfg(unit:CargoCompilationUnitV1,control:ResourceLeaseControl):Promise<readonly string[]>;read(path:string,control:ResourceLeaseControl,request:CargoSourceReadRequestV1):Promise<Uint8Array|null>;recheck(control:ResourceLeaseControl,phase:CargoSourceAdmissionRecheckPhaseV1):Promise<void>}
/** 🧱️ Names the crate whose closure a physical read refused for the remaining budget, so the breach says which crate outgrew its boundary instead of blaming the file that happened to cross it. */
export function sourceBoundaryRefusalV1(error:unknown,crate:string,files:number,bytes:number,limits:Readonly<{maxSourceBytes:number;maxSourceFiles:number}>):unknown{if(!isRepoErrorV1(error))return error;if(error.key==="process.source.read-bytes-exceeded")return refuseProcessV1("breach","source.boundary-exceeded",{crate,files:files+1,maxFiles:limits.maxSourceFiles,bytes:bytes+Number(error.values.observedBytes),maxBytes:limits.maxSourceBytes},{cause:error});if(error.key==="process.source.read-files-exhausted")return refuseProcessV1("breach","source.boundary-exceeded",{crate,files:files+1,maxFiles:limits.maxSourceFiles,bytes,maxBytes:limits.maxSourceBytes},{cause:error});return error;}
/** 🚦️ Admits at most one durable storage recheck per interval and always settles the completion phase. */
export function storageRecheckGateV1(intervalMilliseconds:number,now:()=>number,recheck:()=>Promise<void>):(phase:CargoSourceAdmissionRecheckPhaseV1)=>Promise<void>{let last:number|undefined;return async phase=>{if(phase!=="complete"&&last!==undefined&&now()-last<intervalMilliseconds)return;await recheck();last=now();};}
/** 🧩️ Adds the feature cfg of one unit to the compiler cfg printed without features and unquotes values, so units that differ only in features share one compiler invocation. The compiler prints its cfg ordered by name and then value; a merge keeps that order.
 * @see https://doc.rust-lang.org/rustc/command-line-arguments.html#--print-print-compiler-information */
export function mergeCargoCfgFeaturesV1(lines:readonly string[],features:readonly string[]):readonly string[]{const all=features.length?[...new Set([...lines,...features.map(feature=>"feature="+JSON.stringify(feature))])]:[...lines],parts=(line:string):[string,string|null]=>{const at=line.indexOf("=");return at<0?[line,null]:[line.slice(0,at),line.slice(at+1)];};if(features.length)all.sort((a,b)=>{const[an,av]=parts(a),[bn,bv]=parts(b);return an<bn?-1:an>bn?1:av===bv?0:av===null?-1:bv===null?1:av<bv?-1:1;});return Object.freeze(all.map(value=>value.replace(/=(".*")$/u,(_,quoted)=>"="+JSON.parse(quoted))));}
const cfgConcurrencyV1=4;
const cfgArguments=(unit:CargoCompilationUnitV1):string[]=>{const args=["--print","cfg",...unit.compilerArguments,...unit.compilerFlags];if(unit.platform!==null)args.push("--target",unit.platform);if(unit.kind==="proc-macro")args.push("--crate-type=proc-macro");if(unit.kind==="test")args.push("--test");return args;};
/** 🔌️ Observes actual compiler cfg and no-follow source bytes without executing a compilation unit. */
export function actualCargoSourceAdmissionPortV1(environment:Readonly<Record<string,string|undefined>>,cwd:string,storage:NativeStorageReservationV1,now:()=>number=()=>performance.now()):CargoSourceAdmissionPortV1{const cfgs=new Map<string,Promise<readonly string[]>>(),start=(key:string,args:readonly string[],control:ResourceLeaseControl)=>{const pending=captureOwnedCommandV1("rustc",args,cwd,{control,env:rustCompilerExecutionEnvironmentV1(environment),output:{maximumBytes:1048576,maximumLines:65536}}).then(text=>Object.freeze(text.trim().split("\n")));cfgs.set(key,pending);pending.catch(()=>{if(cfgs.get(key)===pending)cfgs.delete(key);});return pending;},reader=createPhysicalCargoSourceReaderV1(),gate=storageRecheckGateV1(schema.definitions.storageRecheck.const.intervalMilliseconds,now,()=>storage.recheck());return{async recheck(control,phase){requireNativeStorageReservationV1(storage,control);await reader.sweep(control);await gate(phase);},prefetch(units,control){requireNativeStorageReservationV1(storage,control);const wanted=[...new Map(units.map(unit=>{const args=cfgArguments(unit);return[JSON.stringify(args),args] as const;}))];let next=0,active=0;const pump=():void=>{while(active<cfgConcurrencyV1&&next<wanted.length){const[key,args]=wanted[next++]!;if(cfgs.has(key))continue;active++;const settle=():void=>{active--;pump();};start(key,args,control).then(settle,settle);}};pump();},async cfg(unit,control){requireNativeStorageReservationV1(storage,control);const args=cfgArguments(unit),key=JSON.stringify(args);return[...mergeCargoCfgFeaturesV1(await(cfgs.get(key)??start(key,args,control)),unit.features)];},async read(path,control,request){requireNativeStorageReservationV1(storage,control);return reader.read(path,request,control);}};}
/** 📏️ Refuses every selected oversized crate before ordinary Cargo compiler dispatch. Each unit owns its closure: manifest-relative inputs resolve
 * against its manifest directory, and token output it emits as a generator belongs to the crates that expand it.
 * @see ../🟦️.ts */
export async function admitCargoSourceBoundariesV1(graph:CargoCompilationGraphV1,workspaceRoot:string,policy:CargoSourceBoundaryPolicyV1,control:ResourceLeaseControl,port:CargoSourceAdmissionPortV1,memo?:CargoSourceAdmissionMemoV1):Promise<void>{
 checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema,policy).length)throw refuseProcessV1("invalid","source.boundary-policy-invalid");const units=graph.units.filter(unit=>unit.provenance.kind==="workspace"),recalled=units.map(unit=>memo?.recall(unit,policy));port.prefetch?.(units.filter((_,index)=>!recalled[index]),control);let unitIndex=0,readOperations=0,files=0,readBytes=0;const analysis=createRustSourceAnalysisV1(),closures=new Map<string,Readonly<{inputs:CargoCompilationUnitV1["sourceInputs"];environment:CargoCompilationUnitV1["compilerEnvironment"];probed:readonly string[];files:number;bytes:number;lines:number}>>();for(const unit of units){checkResourceLeaseControl(control);unitIndex++;let unitReads=0;const publish=async(phase:CargoSourceAdmissionProgressV1["phase"]):Promise<void>=>{checkResourceLeaseControl(control);const progress:CargoSourceAdmissionProgressV1={version:1,phase,unit:unitIndex,units:units.length,packageName:unit.packageName,readOperations,files,bytes:readBytes};await control.publish({resource:"cargo:source-admission:"+JSON.stringify(progress),mode:"shared",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);};await publish("cfg");const startedMs=Date.now(),remembered=recalled[unitIndex-1];if(remembered){unit.sourceInputs=remembered.inputs.map(input=>({...input}));unit.compilerCfg=[...remembered.cfg];unit.compilerEnvironment=remembered.environment;files+=remembered.inputs.length;readBytes+=remembered.inputs.reduce((sum,input)=>sum+input.bytes,0);await publish("complete");continue;}await port.recheck(control,"unit");const limits=effectiveCargoSourceBoundaryV1(policy,unit.boundary),cfg=await port.cfg(unit,control),distance=relative(resolve(workspaceRoot),unit.manifest),root=distance===".."||distance.startsWith(".."+sep)||isAbsolute(distance)?dirname(unit.manifest):resolve(workspaceRoot),key=JSON.stringify([unit.entry,root,unit.manifest,cfg,unit.support.kind,unit.compilerEnvironment]),hit=closures.get(key);if(hit&&hit.files<=limits.maxSourceFiles&&hit.bytes<=limits.maxSourceBytes&&hit.lines<=limits.maxActiveLines){unit.sourceInputs=hit.inputs.map(input=>({...input}));unit.compilerCfg=[...cfg];unit.compilerEnvironment=hit.environment;files+=hit.inputs.length;readBytes+=hit.bytes;memo?.remember(unit,policy,{inputs:unit.sourceInputs,cfg,environment:unit.compilerEnvironment,probed:hit.probed,startedMs});await publish("complete");continue;}
 const seen=new Map<string,number>(),sourceBytes=new Map<string,Uint8Array>(),probed:string[]=[];let exceeded:Error|undefined,used=0;
 checkResourceLeaseControl(control);const inputs=await readRustSourceClosureV1([unit.entry],root,cfg,async path=>{checkResourceLeaseControl(control);probed.push(path);if(unitReads===0)await publish("read");const previous=seen.get(path);const bytes=await port.read(path,control,{version:1,root,expectedBytes:null,maximumBytes:limits.maxSourceBytes-used+(previous??0),remainingFiles:previous===undefined?limits.maxSourceFiles-seen.size:1}).catch(error=>{const refusal=sourceBoundaryRefusalV1(error,unit.packageName,seen.size-(previous===undefined?0:1),used-(previous??0),limits);if(refusal!==error)exceeded=refusal as Error;throw refusal;});checkResourceLeaseControl(control);readOperations++;unitReads++;noteScriptProcessActivityV1();if(bytes!==null){if(previous===undefined)files++;readBytes+=bytes.byteLength;used+=bytes.byteLength-(previous??0);seen.set(path,bytes.byteLength);sourceBytes.set(path,bytes);if(seen.size>limits.maxSourceFiles||used>limits.maxSourceBytes)throw exceeded=refuseProcessV1("breach","source.boundary-exceeded",{crate:unit.packageName,files:seen.size,maxFiles:limits.maxSourceFiles,bytes:used,maxBytes:limits.maxSourceBytes});}if(unitReads%schema.definitions.readCheckpoint.const===0)await publish("read");return bytes;},{manifestDirectory:dirname(unit.manifest),generated:()=>undefined},analysis).catch(error=>{if(control.signal.aborted&&error===control.signal.reason)throw error;throw error===exceeded||isRepoErrorV1(error)?error:refuseProcessV1("refused","source.crate-refused",{crate:unit.packageName,reason:error instanceof Error?error.message:String(error)},{cause:error});});if(unit.support.kind==="supported"&&unit.compilerEnvironment!==null){const environment={...unit.compilerEnvironment};for(const input of inputs.filter(input=>input.kind==="source")){const bytes=sourceBytes.get(resolve(root,input.path));if(!bytes)throw refuseProcessV1("missing","source.environment-source-missing",{file:input.path});Object.assign(environment,rustCompileEnvironmentReferencesV1({version:1,source:new TextDecoder().decode(bytes),cfg,environment}));}unit.compilerEnvironment=environment;}unit.sourceInputs=inputs.map(input=>({path:resolve(root,input.path),digest:input.digest,bytes:input.bytes}));unit.compilerCfg=[...cfg];const lines=inputs.reduce((sum,input)=>sum+input.activeLines,0);if(cargoSourceBoundaryExcessV1(limits,{files:inputs.length,bytes:inputs.reduce((sum,input)=>sum+input.bytes,0),lines}).length)throw Error("Rust crate Source boundary exceeded before compiler: "+unit.packageName+" (active lines "+lines+"/"+limits.maxActiveLines+")");closures.set(key,{inputs:unit.sourceInputs.map(input=>({...input})),environment:unit.compilerEnvironment,probed,files:seen.size,bytes:used,lines});memo?.remember(unit,policy,{inputs:unit.sourceInputs,cfg,environment:unit.compilerEnvironment,probed,startedMs});await publish("complete");
 }
 await port.recheck(control,"complete");
}
/** 🧾️ Read-only digest rows from the completed workspace Source admission, with the start of that observation. */
export type AdmittedCargoSourceRosterV1=Readonly<{startedMs:number;inputs:Iterable<CargoAdmittedSourceV1>}>;
/** 📍️ The admitted source roster with the workspace root Cargo reported for that selection. */
export type AdmittedCargoSelectionV1=AdmittedCargoSourceRosterV1&Readonly<{cargoWorkspace:string}>;

/** 🎯️ Plans one exact Cargo selection without compiling and admits its Source keyed by the repository root, so every crate a path dependency reaches is
 * admitted whatever directory the selection runs in. Compiler owners outside a build driver (wasm-bindgen engines, the trunk renderer) enter here. */
export async function admitCargoSelectionV1(selection:Readonly<{workspaceRoot:string;cwd:string;args:readonly string[];environment:Readonly<Record<string,string|undefined>>;sourceBoundary:CargoSourceBoundaryPolicyV1;storage:NativeStorageReservationV1;sourceMemo?:CargoSourceAdmissionMemoV1;sourceWitness?:CargoSourceAdmissionWitnessV1}>,control:ResourceLeaseControl):Promise<AdmittedCargoSelectionV1>{
 requireNativeStorageReservationV1(selection.storage,control);await selection.storage.recheck();const graph=await captureCargoCompilationGraphV1({workspaceRoot:selection.workspaceRoot,cwd:selection.cwd,program:"cargo",prefix:[],args:selection.args,environment:selection.environment,maximumOutputBytes:134217728},control,actualCargoCompilerPlanningPortV1());
 const startedMs=Date.now();
 const port=actualCargoSourceAdmissionPortV1(selection.environment,selection.cwd,selection.storage),witness=selection.sourceWitness;
 const reading:CargoSourceAdmissionPortV1=witness?{...port,async read(path,control,request){
  const bytes=await port.read(path,control,request);
  if(bytes!==null)await witness.remember(path,bytes,control);
  return bytes;
 }}:port;
 await admitCargoSourceBoundariesV1(graph,selection.workspaceRoot,selection.sourceBoundary,control,reading,selection.sourceMemo);
 if(witness&&selection.sourceMemo){
  const hydrated=new Set<string>(),root=resolve(selection.workspaceRoot);
  for(const unit of graph.units)if(unit.provenance.kind==="workspace")for(const row of unit.sourceInputs){
   if(hydrated.has(row.path))continue;
   hydrated.add(row.path);
   witness.expect(row.path);
   if(!witness.has(row.path,row.digest)){
    const path=relative(root,row.path);
    if(path===".."||path.startsWith(".."+sep)||isAbsolute(path))continue;
    await reading.read(row.path,control,{version:1,root,expectedBytes:null,maximumBytes:selection.sourceBoundary.maxSourceBytes,remainingFiles:1});
   }
  }
 }
 const inputs=Object.freeze({*[Symbol.iterator](){for(const unit of graph.units)if(unit.provenance.kind==="workspace")for(const {path,digest} of unit.sourceInputs)yield Object.freeze({path,digest});}});
 return Object.freeze({startedMs,cargoWorkspace:graph.workspaceRoot,inputs});
}
