import{fileURLToPath}from"node:url";
import{lstat}from"node:fs/promises";
import{join,dirname,relative,sep,isAbsolute}from"node:path";
import{types}from"node:util";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{requireIssuedResourceControlV1,withRequestedIssuedResourceControlV1,withIssuedResourceOperationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{captureRepositorySourceRootV1,captureExactSourcePathV1,captureSourceInputInventoryV1}from"../../../../../../🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs";
import{withExactSourceReadBatchV1,readExactSourceFileV1}from"../../../../../../🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs";
import { type RustUnitInputV1 } from "../../../🧬️definition/🟦️.ts";
import type schemaContract from"./🧬️schema/🔣️.json";
import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1,type PhysicalSourceModuleMetaV1}from"../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
const sourcePath=fileURLToPath(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),policyPath=fileURLToPath(new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)));
function revision(path:string,maximumBytes:number,retainText=false){try{return captureBoundedSourceRevisionV1(path,maximumBytes,retainText);}catch(error){if(error instanceof Error&&error.message==="Bounded physical Source revision required")throw Error("Bounded physical Cargo reader Source required");throw error;}}
const sourceRevision=revision(sourcePath,8388608),policyRevision=revision(policyPath,65536,true),schema=JSON.parse(policyRevision.text!) as typeof schemaContract,policy=Object.freeze({...schema.definitions.Policy.const});
function requireCurrent():void{if(implementationPinnedV1())return;if(revision(sourcePath,sourceRevision.byteLength).sha256!==sourceRevision.sha256||revision(policyPath,policyRevision.byteLength).sha256!==policyRevision.sha256)throw Error("Stale loaded Cargo configuration reader; restart required");}
export interface SelectedCargoConfigurationV1 {readonly planningInputs:readonly Readonly<{path:string;digest:string;bytes:number}>[];readonly compilerInputs:readonly RustUnitInputV1[]}
/** 🧭️ Binds a finite configuration snapshot to one workspace, Cargo root, cwd and sealed pair. */
export interface SelectedCargoConfigurationScopeV1{readonly workspace:string;readonly cargoWorkspace:string;readonly cwd:string;readonly definingInputs:readonly string[]}
interface Request extends SelectedCargoConfigurationScopeV1{readonly manifest:string}
/** 📄️ Reads a package manifest only within its real live configuration callback. */
export type SelectedCargoConfigurationCaptureV1=(manifest:string)=>Promise<SelectedCargoConfigurationV1>;
function capture(options:unknown,scope=false):Request{
 if(!options||typeof options!=="object"||types.isProxy(options)||Object.getPrototypeOf(options)!==Object.prototype)throw Error("Own Cargo configuration request required");
 const keys=scope?["cargoWorkspace","cwd","definingInputs","workspace"]:["cargoWorkspace","cwd","definingInputs","manifest","workspace"];if(Reflect.ownKeys(options).length!==keys.length)throw Error("Closed own Cargo configuration request required");const rows=Object.getOwnPropertyDescriptors(options);
 if(Object.keys(rows).sort().join()!==keys.join()||Object.values(rows).some(row=>!("value"in row)||!row.enumerable))throw Error("Closed own Cargo configuration request required");
 const workspace=captureRepositorySourceRootV1(rows.workspace!.value),files:unknown=rows.definingInputs!.value;
 if(!Array.isArray(files)||types.isProxy(files)||files.length!==policy.maximumDefiningInputs)throw Error("Exact sealed Cargo defining pair required");
 const definingInputs=captureSourceInputInventoryV1({version:1,files},workspace).value.files;
 const coordinate=(value:unknown,allowRoot:boolean):string=>{const path=captureRepositorySourceRootV1(value),distance=relative(workspace,path);if(allowRoot&&path===workspace)return path;if(!distance||distance===".."||distance.startsWith(".."+sep)||isAbsolute(distance))throw Error("Owned Cargo coordinate required");return captureExactSourcePathV1(workspace,distance.replaceAll("\\","/"));};
 const cargoWorkspace=coordinate(rows.cargoWorkspace!.value,true),result=Object.freeze({workspace,cargoWorkspace,cwd:coordinate(rows.cwd!.value,true),manifest:coordinate(scope?join(cargoWorkspace,"Cargo.toml"):rows.manifest!.value,false),definingInputs});
 if(validateJsonSchemaSubset(schema.definitions.Request,result).length)throw Error("Closed Cargo request DATA required");if(scope){const{manifest,...value}=result;if(validateJsonSchemaSubset(schema.definitions.Scope,value).length)throw Error("Closed Cargo scope DATA required");}return result;
}
/** 🔎️ Streams bounded Cargo planning freshness separately from actual sealed compiler inputs. */
export async function captureSelectedCargoConfigurationV1(options:Request,control:ResourceLeaseControl):Promise<SelectedCargoConfigurationV1>{
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrent();const request=capture(options);
 const result=await withRequestedIssuedResourceControlV1(control,0,async actual=>{
 const{workspace,cargoWorkspace,cwd,manifest,definingInputs}=request,paths=new Map<string,number>();
 const add=(path:string,maximum:number)=>{const name=relative(workspace,path).replaceAll("\\","/");captureExactSourcePathV1(workspace,name);if(!paths.has(name)&&paths.size>=policy.maximumPlanningInputs)throw Error("Cargo planning input capacity exceeded");paths.set(name,Math.min(paths.get(name)??maximum,maximum));};
 add(manifest,policy.maximumPlanningFileBytes);add(join(cargoWorkspace,"Cargo.toml"),policy.maximumPlanningFileBytes);add(join(cargoWorkspace,"Cargo.lock"),policy.maximumPlanningFileBytes);for(const name of definingInputs)add(join(workspace,name),policy.maximumDefiningFileBytes);
 let ancestors=0;
 for(let current=cwd;;current=dirname(current)){
 checkResourceLeaseControl(actual);if(++ancestors>policy.maximumAncestors)throw Error("Cargo planning ancestry capacity exceeded");
 for(const name of["rust-toolchain.toml",".cargo/config",".cargo/config.toml"]){const path=join(current,name);captureExactSourcePathV1(workspace,relative(workspace,path).replaceAll("\\","/"));await withIssuedResourceOperationV1(actual,async()=>{checkResourceLeaseControl(actual);try{const facts=await lstat(path);checkResourceLeaseControl(actual);if(!facts.isFile()||facts.isSymbolicLink()||facts.size>policy.maximumPlanningFileBytes)throw Error("Bounded physical Cargo configuration required");add(path,policy.maximumPlanningFileBytes);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;checkResourceLeaseControl(actual);}});}
 if(current===workspace)break;await actual.yieldContinuation();checkResourceLeaseControl(actual);
 }
 return withExactSourceReadBatchV1(workspace,actual,async batch=>{
 const planningInputs:Readonly<{path:string;digest:string;bytes:number}>[]=[],compilerInputs:RustUnitInputV1[]=[];let used=0;
 for(const[path,maximum]of[...paths].sort(([left],[right])=>left<right?-1:left>right?1:0)){
 checkResourceLeaseControl(actual);const value=await readExactSourceFileV1(batch,path,Math.min(maximum,policy.maximumAggregateBytes-used));if(!value)throw Error("Required Cargo planning input missing");used+=value.byteLength;const record=Object.freeze({path,digest:value.sha256,bytes:value.byteLength});planningInputs.push(record);if(definingInputs.includes(path))compilerInputs.push(Object.freeze({...record,kind:"configuration"}));
 }
 checkResourceLeaseControl(actual);const result=Object.freeze({planningInputs:Object.freeze(planningInputs),compilerInputs:Object.freeze(compilerInputs)});if(validateJsonSchemaSubset(schema,result).length)throw Error("Closed Cargo planning/compiler inputs required");return result;
 });
 });requireCurrent();checkResourceLeaseControl(control);return result;
}

/** 🧺️ Reuses bounded common DATA, drains accepted manifest reads and freshly revalidates the common closure before delivery. */
export async function withSelectedCargoConfigurationScopeV1<T>(options:SelectedCargoConfigurationScopeV1,control:ResourceLeaseControl,body:(capture:SelectedCargoConfigurationCaptureV1,actual:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrent();const request=capture(options,true);if(typeof body!=="function")throw Error("Actual scoped Cargo configuration consumer required");
 return withRequestedIssuedResourceControlV1(control,0,async actual=>{
 const index=new Map<string,SelectedCargoConfigurationV1["planningInputs"][number]>();let common:SelectedCargoConfigurationV1|undefined,used=0,live=true,busy=false;
 try{const value=await withRequestedIssuedResourceControlV1(actual,0,async phase=>{
 const read:SelectedCargoConfigurationCaptureV1=manifest=>{if(!live)throw Error("Live scoped Cargo configuration required");if(busy)throw Error("Serialized scoped Cargo configuration required");busy=true;
 try{return withRequestedIssuedResourceControlV1(phase,0,async current=>{
 requireCurrent();const path=captureRepositorySourceRootV1(manifest),distance=relative(request.workspace,path);if(!distance||distance===".."||distance.startsWith(".."+sep)||isAbsolute(distance))throw Error("Owned Cargo coordinate required");const name=distance.replaceAll("\\","/");captureExactSourcePathV1(request.workspace,name);if(!common){common=await captureSelectedCargoConfigurationV1(request,current);for(const row of common.planningInputs){index.set(row.path,row);used+=row.bytes;}}const captured=common;let row=index.get(name);
 if(row){if(row.bytes>policy.maximumPlanningFileBytes)throw Error("Source reducing extent exceeded before read");}else{if(index.size>=policy.maximumPlanningInputs)throw Error("Cargo planning input capacity exceeded");row=await withExactSourceReadBatchV1(request.workspace,current,async batch=>{const observed=await readExactSourceFileV1(batch,name,Math.min(policy.maximumPlanningFileBytes,policy.maximumAggregateBytes-used));if(!observed)throw Error("Required Cargo planning input missing");return Object.freeze({path:name,digest:observed.sha256,bytes:observed.byteLength});});}
 const planningInputs=index.has(name)?captured.planningInputs:Object.freeze([...captured.planningInputs,row].sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0)),result=Object.freeze({planningInputs,compilerInputs:captured.compilerInputs});if(validateJsonSchemaSubset(schema,result).length)throw Error("Closed Cargo planning/compiler inputs required");requireCurrent();checkResourceLeaseControl(current);return result;
 }).finally(()=>{busy=false;});}catch(error){busy=false;throw error;}};
 return body(read,phase);
 });live=false;if(common){const fresh=await captureSelectedCargoConfigurationV1(request,actual);if(JSON.stringify(fresh.planningInputs)!==JSON.stringify(common.planningInputs))throw Error("Scoped Cargo common Source changed");}requireCurrent();checkResourceLeaseControl(actual);return value;
 }finally{live=false;}
 });
}
