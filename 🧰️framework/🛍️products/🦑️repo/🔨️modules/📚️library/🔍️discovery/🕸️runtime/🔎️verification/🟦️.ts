export type { RuntimeTrunkObservationV1, RuntimeActorCargoSelectionV1, RuntimeCompilerChecksumV1 } from "../🧬️schema/🟦️.ts";
import type { RuntimeCompilerInputV1, RuntimeCargoBuildScriptV1, RuntimeCargoUnitV1, RuntimeCargoObservationV1, RuntimeBuildResourceObservationV1, RuntimeCargoBuildResourcesV1, RuntimeCompilerResourceEntryV1, RuntimeCompilerResourceOperationV1, RuntimeCompilerResourceV1, RuntimeCargoCompilerResourcesV1, RuntimeTrunkObservationV1, RuntimeActorCargoSelectionV1, RuntimeCompilerChecksumV1 } from "../🧬️schema/🟦️.ts";
import { observeSelectedRuntimeActorsV1, acquireSelectedRuntimeActorsV1, type SelectedRuntimeActorPublicationPortV1, type SelectedRuntimeActorsV1, parseSelectedRuntimeGraphPackageRootsV1, type SelectedRuntimeGraphPackageRootsV1 } from "../🎭️selection/🧬️schema/🟦️.ts";
import { existsSync, readFileSync, realpathSync, writeFileSync, mkdirSync, readdirSync, statSync, lstatSync } from "node:fs";
import { createHash } from "node:crypto";
import { cargoInputDigestV1, cargoDirectoryEntriesV1, cargoProvenancePhysicalControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoDepInfoResolvedChecksumsV1, cargoDepInfoResolvedSourcesV1, cargoDepInfoSourcesV1, decodeCargoProvenanceV1 } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { blake3Hex } from "../../../../../../../🔨️modules/🔏️hash/🟦️.ts";
import type {FileObservationControlV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🟦️.ts";
import {CurrentPhysicalOwnerV1,type CurrentPhysicalPortV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import {cmdBudgetMs} from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { homedir } from "node:os";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { inspectRuntimeGraphV1, runtimeEcmaReferencesV1, runtimeRustReferencesV1, runtimeFixturePathV1, type RuntimeResourceReadOwnerV1, type RuntimeGraphEvidenceV1, type RuntimeGraphFindingV1 } from "../🟦️.ts";
import { policyWalkRelFiles } from "../../🚶️file-walk/🟦️.ts";
import { repoCacheDirectory } from "../../../⚡️caching/🟦️.ts";
import { cargoDirectories } from "../../../⚡️caching/🦀️cargo/🟦️.ts";
import { runTool } from "../../../⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts";
import {repositoryCargoPreparationStorageV1, prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";

/** 🧵️ Binds one actual verified-Blob import operation while preserving all other computed callers. */
export function runtimeActorChildImportV1(source:string,child:string,actor:string,sha256:string){
  const calls=runtimeEcmaReferencesV1(source,child,{read:()=>undefined,productionTests:"excluded"}).filter(reference=>reference.path===null&&reference.kind==="import"&&reference.expression==="import(moduleUrl)");
  if(calls.length!==1||!calls[0]!.sourceSha256||calls[0]!.callsiteIndex===undefined||!/^[a-f0-9]{64}$/u.test(sha256))return;
  return{path:actor,sha256,sourceSha256:calls[0]!.sourceSha256,callsiteIndex:calls[0]!.callsiteIndex};
}

interface CargoPackageV1 { readonly id: string; readonly name: string; readonly manifest_path: string; readonly source: string | null; readonly targets: readonly { readonly src_path: string; readonly kind: readonly string[] }[] }
interface CargoNodeV1 { readonly id: string; readonly features: readonly string[]; readonly deps: readonly { readonly pkg: string; readonly dep_kinds: readonly { readonly kind: string | null }[] }[] }
/** 🧭️ Bridges the actual internal read operation to one exact source token and verified original byte/metadata inputs. */
export function runtimeCompilerResourceReadOwnersV1(observation:RuntimeCargoObservationV1,context:Parameters<typeof runtimeCompilerResourceInputsV1>[2]):{readonly owners:Readonly<Record<string,readonly RuntimeResourceReadOwnerV1[]>>;readonly directories:Readonly<Record<string,string>>;readonly witnesses:readonly {readonly receipt:RuntimeCargoCompilerResourcesV1;readonly evidence:ReturnType<typeof runtimeCompilerResourceInputsV1>}[]}{
  const owners:Record<string,RuntimeResourceReadOwnerV1[]>={},directories:Record<string,string>={},unitFindings=new Map<RuntimeCargoUnitV1,readonly RuntimeGraphFindingV1[]>(),witnesses=(observation.compilerResources??[]).map(receipt=>({receipt,evidence:compilerResourceInputs(observation,receipt,context,unitFindings)})),relativePath=(path:string)=>relative(context.root,resolve(path)).replaceAll("\\","/");
  for(const{receipt,evidence}of witnesses.filter(witness=>witness.evidence.verified))for(const row of receipt.resources){
    if(row.input.kind==="directory")directories[relativePath(row.input.path)]=row.sha256!;
    const operations:[RuntimeCompilerResourceOperationV1,string,"file"|"directory",string][]=[[row.input.operation,row.input.path,row.input.kind==="read"?"file":"directory",row.sha256!]];
    for(const[operation,input,kind,sha256]of operations){const bytes=context.read(operation.source),source=bytes===undefined?undefined:String(bytes),path=relativePath(operation.source);if(source===undefined)continue;
      const references=runtimeRustReferencesV1(source,path,{read:()=>undefined,features:receipt.producerUnit?.features,cfg:[]}).filter(reference=>reference.callsiteOffset!==undefined&&reference.expression===`Filesystem ${operation.name} requires actual read input ownership`&&source.slice(0,reference.callsiteOffset).split("\n").length===operation.line);
      if(references.length!==1)continue;const reference=references[0]!,entries=owners[path]??=[],existing=entries.find(owner=>owner.callsiteOffset===reference.callsiteOffset),value={path:relativePath(input),kind,sha256};
      if(existing){if(!existing.inputs.some(input=>JSON.stringify(input)===JSON.stringify(value)))entries[entries.indexOf(existing)]={...existing,inputs:[...existing.inputs,value]};}else entries.push({sourceSha256:createHash("sha256").update(source).digest("hex"),callsiteOffset:reference.callsiteOffset!,inputs:[value]});owners[path]=entries;
    }
  }
  return{owners,directories,witnesses};
}

/** 🧷️ Preserves all compatible exact-operation inputs and refuses contradictory capture evidence. */
export function runtimeMergeResourceReadOwnersV1(groups:readonly {readonly owners:Readonly<Record<string,readonly RuntimeResourceReadOwnerV1[]>>;readonly directories:Readonly<Record<string,string>>}[]):{readonly owners:Readonly<Record<string,readonly RuntimeResourceReadOwnerV1[]>>;readonly directories:Readonly<Record<string,string>>;readonly findings:readonly RuntimeGraphFindingV1[]}{
  const owners:Record<string,RuntimeResourceReadOwnerV1[]>={},directories:Record<string,string>={},findings:RuntimeGraphFindingV1[]=[],blocked=new Set<string>(),fail=(path:string,key:string,detail:string)=>{blocked.add(key);findings.push({code:"runtime-input-mismatch",path,detail});};
  for(const group of groups){for(const[path,sha256]of Object.entries(group.directories)){if(directories[path]&&directories[path]!==sha256)fail(path,path,"Compiler resource observations disagree on original directory metadata");directories[path]=sha256;}
    for(const[path,rows]of Object.entries(group.owners))for(const row of rows){const key=path+":"+row.callsiteOffset,entries=owners[path]??=[],existing=entries.find(owner=>owner.callsiteOffset===row.callsiteOffset);if(existing){if(existing.sourceSha256!==row.sourceSha256){fail(path,key,"Exact resource operation has contradictory original source identities");continue;}const inputs=[...existing.inputs];for(const input of row.inputs){const previous=inputs.find(value=>value.path===input.path);if(previous&&(previous.kind!==input.kind||previous.sha256!==input.sha256))fail(path,key,"Exact resource operation has contradictory original input digests or kinds");else if(!previous)inputs.push(input);}entries[entries.indexOf(existing)]={...existing,inputs};}else entries.push({...row,inputs:[...row.inputs]});owners[path]=entries;}}
  for(const[path,rows]of Object.entries(owners))owners[path]=rows.filter(row=>!blocked.has(path+":"+row.callsiteOffset));for(const path of Object.keys(directories))if(blocked.has(path))delete directories[path];
  return{owners,directories,findings};
}

/** 🧮️ Binds original proc-macro reads to the exact selected producer/caller compiler units and immutable tracked capture. */
export function runtimeCompilerResourceInputsV1(observation:RuntimeCargoObservationV1,resource:RuntimeCargoCompilerResourcesV1,context:{readonly root:string;readonly digest:(path:string)=>string|undefined;readonly read:(path:string)=>string|Uint8Array|undefined;readonly entries:(path:string)=>readonly (readonly [string,string,string|null])[]|undefined;readonly fixtureCollections?:readonly string[]}):{readonly inputs:readonly string[];readonly directories:readonly string[];readonly findings:readonly RuntimeGraphFindingV1[];readonly verified:boolean}{
  return compilerResourceInputs(observation,resource,context,new Map());
}

function compilerResourceInputs(observation:RuntimeCargoObservationV1,resource:RuntimeCargoCompilerResourcesV1,context:Parameters<typeof runtimeCompilerResourceInputsV1>[2],unitFindings:Map<RuntimeCargoUnitV1,readonly RuntimeGraphFindingV1[]>):ReturnType<typeof runtimeCompilerResourceInputsV1>{
  const findings:RuntimeGraphFindingV1[]=[],inputs=new Set<string>(),directories=new Set<string>(),fail=(path:string,detail:string):void=>{findings.push({code:"runtime-input-mismatch",path:relative(context.root,path).replaceAll("\\","/"),detail})},inside=(root:string,path:string):boolean=>{const value=relative(root,path);return value!==""&&!isAbsolute(value)&&value.split(/[\\/]/u)[0]!==".."};
  const captured=observation.compilerResourceRoot&&inside(observation.compilerResourceRoot,resource.path)&&basename(resource.path)==="observation.json"?context.read(resource.path):undefined;
  if(!observation.compilerResourceRoot||!inside(observation.compilerResourceRoot,resource.path)||basename(resource.path)!=="observation.json"||captured===undefined||!resource.sha256||createHash("sha256").update(captured).digest("hex")!==resource.sha256||context.digest(resource.path)!==resource.sha256||!Number.isFinite(resource.observedAtMs)||resource.observedAtMs>observation.observedAtMs){fail(resource.path,"Compiler resource capture lost its actual root, immutable bytes or completed observation time");return{inputs:[],directories:[],findings,verified:false}}
  let capture:any;try{capture=JSON.parse(typeof captured==="string"?captured:new TextDecoder("utf-8",{fatal:true}).decode(captured))}catch{fail(resource.path,"Compiler resource capture is not valid JSON");return{inputs:[],directories:[],findings,verified:false}}
  if(capture.version!==1||capture.kind!=="compiler-resource"||capture.completed!==true||!Number.isFinite(capture.builtAtMs)||!Number.isFinite(capture.observedAtMs)||capture.observedAtMs<capture.builtAtMs||capture.observedAtMs>resource.observedAtMs||!Array.isArray(capture.resources))fail(resource.path,"Actual proc-macro entry has no complete original resource observation");
  const units=(message:RuntimeCompilerInputV1|null)=>message?observation.units.filter(unit=>JSON.stringify(unit.message)===JSON.stringify(message)&&!unit.message.profile.test):[],producers=units(resource.producerUnit),callers=units(resource.callerUnit),producer=producers[0],caller=callers[0];
  for(const[identity,unit,role]of [[capture.producer,producer,"producer"],[capture.caller,caller,"caller"]]as const){if(!identity||![identity.manifest,identity.source].every(value=>typeof value==="string"&&isAbsolute(value))||!unit||resolve(unit.message.manifest_path??"")!==resolve(identity.manifest)||!unit.inputs.some(input=>input.kind==="file"&&resolve(input.path)===resolve(identity.source)))fail(resource.path,`Compiler resource ${role} identity is not the same actual compiler unit`)}
  if(producers.length!==1||callers.length!==1||capture.caller?.crate!==caller?.message.target.name.replaceAll("-","_")||!producer?.message.target.kind.includes("proc-macro")||caller?.message.target.kind.some(kind=>["proc-macro","custom-build"].includes(kind))||!caller?.inputs.some(input=>resolve(input.path)===resolve(resource.path)&&input.sha256===resource.sha256))fail(resource.path,"Compiler resource capture lacks single exact producer/caller and tracked dep-info bindings");
  if(producer&&caller)for(const unit of[producer,caller]){if(!unitFindings.has(unit))unitFindings.set(unit,[...runtimeCargoUnitInputsV1(observation,unit,context.root,path=>context.digest(resolve(context.root,path))).findings,...runtimeCargoConsumedInputsV1(unit,context.read)]);findings.push(...unitFindings.get(unit)!);}
  const normalize=(entries:readonly RuntimeCompilerResourceEntryV1[]):readonly (readonly [string,string,string|null])[]=>entries.map(entry=>[basename(entry.path),entry.kind,entry.symlinkTarget]as const).sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])));
  if(resource.resources.length!==capture.resources?.length)fail(resource.path,"Retained compiler resources do not cover the original complete entry roster");
  for(const[index,row]of(resource.resources??[]).entries()){
    const input=row.input;if(JSON.stringify(input)!==JSON.stringify(capture.resources?.[index])||!input||!isAbsolute(input.path)||!input.callsite||!isAbsolute(input.callsite.source)||!Number.isInteger(input.callsite.line)||input.callsite.line<1||!producer?.inputs.some(value=>value.kind==="file"&&resolve(value.path)===resolve(input.callsite.source))){fail(resource.path,"Compiler read lost its original source callsite and exact producer input identity");continue}
    for(const operation of[input.operation])if(!operation||!isAbsolute(operation.source)||!Number.isInteger(operation.line)||operation.line<1||!["read","read_dir"].includes(operation.name)||!producer?.inputs.some(value=>value.kind==="file"&&resolve(value.path)===resolve(operation.source)))fail(resource.path,"Compiler resource internal operation is not the same actual owned helper input");
    const path=relative(context.root,input.path).replaceAll("\\","/");if(runtimeFixturePathV1(path,context.fixtureCollections))findings.push({code:"runtime-fixture-edge",path,detail:"Actual proc-macro consumed a fixture-owned original input"});
    if(context.digest(input.path)!==row.sha256||!row.sha256||!caller?.inputs.some(value=>resolve(value.path)===resolve(input.path)))fail(input.path,"Original compiler resource has no current tracked input digest");
    if(input.kind==="read"){
      inputs.add(path);const bytes=context.read(input.path);if(!Number.isSafeInteger(input.bytes)||input.bytes<0||!row.sourceExact||row.schemaErrors?.length!==0||row.bytes!==input.bytes||bytes===undefined||byteCount(bytes)!==input.bytes||!validDigest(input.sha256)||row.sha256!==input.sha256||createHash("sha256").update(bytes).digest("hex")!==input.sha256||Object.hasOwn(input,"output")||Object.hasOwn(input,"snapshotOperation"))fail(input.path,"Current original bytes differ from the actual in-place proc-macro consumed-byte witness");
    }else if(input.kind==="directory"){
      directories.add(path);if(!Array.isArray(input.entries)||input.entries.some(entry=>dirname(entry.path)!==input.path||!["file","directory","symlink","other"].includes(entry.kind)||(entry.kind==="symlink"?typeof entry.symlinkTarget!=="string":entry.symlinkTarget!==null))||JSON.stringify(normalize(input.entries))!==JSON.stringify(context.entries(input.path))||!row.observedEntries||JSON.stringify(normalize(input.entries))!==JSON.stringify(normalize(row.observedEntries)))fail(input.path,"Directory metadata differs from the actual original complete enumeration");
    }else fail(input.path,"Compiler resource kind is not owned by the original observation contract");
  }
  return{inputs:[...inputs].sort(),directories:[...directories].sort(),findings,verified:findings.length===0};
}


/** 🎞️ Accepts only the actual profile owner, surviving compiler inputs and current server-mounted transformation bytes. */
export async function runtimeTrunkObservationCurrentV1(receipt:RuntimeTrunkObservationV1,context:{readonly root:string;readonly profile:string;readonly manifest:string;readonly outputDirectory:string;readonly owner:string;readonly physical:CurrentPhysicalPortV1;readonly compilerCurrent:(observation:RuntimeCargoObservationV1,physical:CurrentPhysicalPortV1)=>Promise<boolean>}):Promise<RuntimeCargoObservationV1|undefined>{
  if(receipt.version!==1||receipt.kind!=="wgpu-trunk"||receipt.profile!==context.profile||resolve(receipt.manifest)!==resolve(context.manifest)||resolve(receipt.outputDirectory)!==resolve(context.outputDirectory)||resolve(receipt.owner.path)!==resolve(context.owner)||(await context.physical.digest(receipt.owner.path))!==receipt.owner.sha256||(await context.physical.digest(receipt.compiler.path))!==receipt.compiler.sha256||(await context.physical.digest(receipt.raw.path))!==receipt.raw.sha256||!Number.isFinite(receipt.builtAtMs)||!Number.isFinite(receipt.observedAtMs)||receipt.observedAtMs<receipt.builtAtMs)return;
  const source=await context.physical.read(receipt.compiler.path);if(source===undefined)return;
  const[observation]=runtimeCargoProvenanceV1(typeof source==="string"?source:new TextDecoder("utf-8",{fatal:true}).decode(source));if(!observation||resolve(observation.manifest)!==resolve(receipt.manifest)||observation.builtAtMs<receipt.builtAtMs||observation.observedAtMs>receipt.observedAtMs )return;
  const selection=runtimeCargoInvocationV1(observation,context.root),args=["build","--target=wasm32-unknown-unknown","--manifest-path",receipt.manifest,...(context.profile==="release"?["--release"]:[]),"--offline","--frozen","--locked","--message-format=json"];
  if(JSON.stringify(observation.args)!==JSON.stringify(args)||selection.target!=="wasm32-unknown-unknown"||!selection.defaults||selection.features.length||selection.packages.length||selection.allFeatures)return;
  const artifacts=observation.units.filter(unit=>resolve(unit.message.manifest_path??"")===resolve(receipt.manifest)&&!unit.message.profile.test).flatMap(unit=>unit.artifacts).filter(artifact=>artifact.stagedPath===receipt.raw.path&&artifact.sha256===receipt.raw.sha256);
  if(artifacts.length!==1||!receipt.outputs.some(output=>output.path.endsWith(".js"))||!receipt.outputs.some(output=>output.path.endsWith("_bg.wasm"))||receipt.outputs.some(output=>relative(receipt.outputDirectory,output.path).replaceAll("\\","/").split("/").some(part=>part==="..")||isAbsolute(relative(receipt.outputDirectory,output.path))))return;
  const outputs=new Map<string,string|Uint8Array|undefined>();for(const output of receipt.outputs){await context.physical.checkpoint();outputs.set(output.path,await context.physical.read(output.path));}if(runtimeTrunkCompilationV1({rawSha256:receipt.raw.sha256,querySha256:artifacts[0]!.sha256,outputs:receipt.outputs},path=>{if(!outputs.has(path))throw Error("Unadmitted Trunk output");return outputs.get(path);}).length)return;if(!await context.compilerCurrent(observation,context.physical))return;
  return observation;
}

/** 🏗️ Binds the actual Trunk compiler query to the transformed bytes served at their mount. */
export function runtimeTrunkCompilationV1(receipt:{readonly rawSha256:string;readonly querySha256:string;readonly outputs:readonly {readonly path:string;readonly sha256:string}[]},read:(path:string)=>string|Uint8Array|undefined):readonly RuntimeGraphFindingV1[]{
  const findings:RuntimeGraphFindingV1[]=[];
  if(receipt.rawSha256!==receipt.querySha256)findings.push({code:"runtime-input-mismatch",path:receipt.outputs[0]?.path??"",detail:"Trunk raw compiler artifact differs from the unchanged actual Cargo artifact query"});
  for(const output of receipt.outputs){const bytes=read(output.path);if(bytes===undefined||createHash("sha256").update(bytes).digest("hex")!==output.sha256)findings.push({code:"runtime-input-mismatch",path:output.path,detail:"Current server-mounted output differs from actual completed Trunk transformation bytes"});}
  return findings;
}

/** 🖥️ Keeps fresh compiler output distinct from the bytes the actual server mount serves. */
export function runtimeBrowserArtifactV1(path:string,content:string,read:(path:string)=>string|Uint8Array|undefined):{readonly path:string;readonly compiledSha256:string;readonly mountedSha256:string|null;readonly current:boolean}{
  const compiledSha256=createHash("sha256").update(content).digest("hex"),bytes=read(path),mountedSha256=bytes===undefined?null:createHash("sha256").update(bytes).digest("hex");
  return {path,compiledSha256,mountedSha256,current:compiledSha256===mountedSha256};
}

function validDigest(value:unknown):value is string{return typeof value==="string"&&/^[0-9a-f]{64}$/u.test(value);}
function byteCount(value:string|Uint8Array):number{return typeof value==="string"?Buffer.byteLength(value):value.byteLength;}
function resourceWitness(value:any):boolean{
 if(!value||!value.input||typeof value.input!=="object"||!(value.sha256===null||validDigest(value.sha256)))return false;
 if(value.input.kind==="read")return !Object.hasOwn(value,"outputSha256")&&!Object.hasOwn(value.input,"output")&&!Object.hasOwn(value.input,"snapshotOperation")&&Number.isSafeInteger(value.input.bytes)&&value.input.bytes>=0&&validDigest(value.input.sha256)&&(value.bytes===null||Number.isSafeInteger(value.bytes)&&value.bytes>=0)&&typeof value.sourceExact==="boolean"&&Array.isArray(value.schemaErrors)&&value.schemaErrors.every((error:unknown)=>typeof error==="string");
 if(Object.hasOwn(value,"bytes")||Object.hasOwn(value,"sourceExact")||Object.hasOwn(value,"schemaErrors"))return false;
 if(value.input.kind==="copy")return value.outputSha256===null||validDigest(value.outputSha256);
 return !Object.hasOwn(value,"outputSha256")&&["directory","failed"].includes(value.input.kind)&&(value.observedEntries===undefined||Array.isArray(value.observedEntries)&&value.observedEntries.every((entry:any)=>typeof entry.path==="string"&&["file","directory","symlink","other"].includes(entry.kind)&&(entry.kind==="symlink"?typeof entry.symlinkTarget==="string":entry.symlinkTarget===null)));
}

/** 🧾️ Original consumed-byte witnesses refuse coordinated source/output edits after actual Cargo execution. */
export function runtimeRetainedBuildResourceInputsV1(observation: RuntimeCargoObservationV1, resource: RuntimeCargoBuildResourcesV1, context: Parameters<typeof runtimeBuildResourceInputsV1>[1]): ReturnType<typeof runtimeBuildResourceInputsV1> & { readonly verified: boolean } {
  const path = (value: string): string => relative(context.root, resolve(observation.cwd,value)).replaceAll("\\","/");
  const captured = context.read(path(resource.path)); let source = "";
  try { source = captured === undefined ? "" : typeof captured === "string" ? captured : new TextDecoder("utf-8",{fatal:true}).decode(captured); } catch { source = ""; }
  const evidence = runtimeBuildResourceInputsV1(source, context), findings = [...evidence.findings];
  const mismatch = (value: string, detail: string): void => { findings.push({code:"runtime-input-mismatch",path:path(value),detail}); };
  if (!observation.buildScripts.some(script=>script.package_id===resource.package_id && resolve(script.out_dir)===resolve(resource.out_dir)) || resolve(context.outDirectory)!==resolve(resource.out_dir) || dirname(resolve(resource.path))!==resolve(resource.out_dir)) mismatch(resource.path,"Resource observation is not bound to the same actual Cargo build-script OUT_DIR");
  if (captured === undefined || createHash("sha256").update(captured).digest("hex")!==resource.sha256 || resource.observedAtMs<observation.builtAtMs || resource.observedAtMs>observation.observedAtMs) mismatch(resource.path,"Retained resource JSONL digest/time disagrees with actual Cargo observation");
  if (resource.resources.length!==evidence.observations.length) mismatch(resource.path,"Retained source/copy snapshots do not cover the actual producer observation roster");
  for (const [index,row] of evidence.observations.entries()) {
    const retained = resource.resources[index];
    if (!retained || JSON.stringify(retained.input)!==JSON.stringify(row)) { mismatch(row.path,"Retained resource snapshot identifies a different producer input"); continue; }
    if (row.kind==="directory") {
      const normalized=(entries:readonly RuntimeCompilerResourceEntryV1[])=>entries.map(entry=>[basename(entry.path),entry.kind,entry.symlinkTarget]).sort((a,b)=>Buffer.compare(Buffer.from(a[0]!),Buffer.from(b[0]!))),original=normalized(row.entries);
      if(!retained.sha256||createHash("sha256").update(JSON.stringify(original)).digest("hex")!==retained.sha256||!retained.observedEntries||JSON.stringify(normalized(retained.observedEntries))!==JSON.stringify(original))mismatch(row.path,"Retained directory metadata digest differs from the original actual enumeration");
    } else if(row.kind==="read"){
      const bytes=context.read(path(row.path));if(!resourceWitness(retained)||retained.sourceExact!==true||retained.schemaErrors?.length!==0||bytes===undefined||byteCount(bytes)!==row.bytes||retained.bytes!==row.bytes||retained.sha256!==row.sha256||createHash("sha256").update(bytes).digest("hex")!==row.sha256)mismatch(row.path,"Current original resource differs from its actual consumed-byte witness");
    } else if(row.kind==="copy") {
      const bytes = context.read(path(row.path)), output = context.read(path(row.output));
      if (!retained.sha256 || bytes===undefined || createHash("sha256").update(bytes).digest("hex")!==retained.sha256) mismatch(row.path,"Current original resource differs from retained build-time source digest");
      if (!retained.outputSha256 || output===undefined || createHash("sha256").update(output).digest("hex")!==retained.outputSha256 || retained.sha256!==retained.outputSha256) mismatch(row.output,"Current copied/read bytes differ from retained build-time output digest");
    }
  }
  return {...evidence,findings,verified:findings.length===0 && observation.status===0 && !observation.cancelled};
}

/** 🏗️ Bridges verified original build-script reads to exact host helper operations without treating enumeration as child bytes. */
export function runtimeBuildResourceReadOwnersV1(observation:RuntimeCargoObservationV1,resource:RuntimeCargoBuildResourcesV1,context:Parameters<typeof runtimeBuildResourceInputsV1>[1]):{readonly owners:Readonly<Record<string,readonly RuntimeResourceReadOwnerV1[]>>;readonly directories:Readonly<Record<string,string>>;readonly findings:readonly RuntimeGraphFindingV1[];readonly verified:boolean}{
  const evidence=runtimeRetainedBuildResourceInputsV1(observation,resource,context),findings=[...evidence.findings],owners:Record<string,RuntimeResourceReadOwnerV1[]>={},directories:Record<string,string>={},units=observation.units.filter(unit=>unit.message.package_id===resource.package_id&&unit.message.target.kind.includes("custom-build")&&!unit.message.profile.test),producer=units[0],relativePath=(path:string)=>relative(context.root,resolve(observation.cwd,path)).replaceAll("\\","/"),bytes=(path:string)=>context.read(relativePath(path));
  if(units.length!==1||observation.buildScripts.filter(script=>script.package_id===resource.package_id&&resolve(script.out_dir)===resolve(resource.out_dir)).length!==1)findings.push({code:"runtime-unresolved-edge",path:relativePath(resource.path),detail:"Build resource reader lacks a single exact actual host custom-build unit and OUT_DIR"});
  if(producer){findings.push(...runtimeCargoUnitInputsV1(observation,producer,context.root,path=>{const value=context.read(path);return value===undefined?undefined:createHash("sha256").update(value).digest("hex")}).findings,...runtimeCargoConsumedInputsV1(producer,bytes));for(const artifact of producer.artifacts){const value=bytes(artifact.stagedPath??artifact.path);if(value===undefined||createHash("sha256").update(value).digest("hex")!==artifact.sha256)findings.push({code:"runtime-input-mismatch",path:relativePath(artifact.path),detail:"Actual build resource producer artifact is absent or differs from its selected unit"});}}
  if(findings.length)return{owners,directories,findings,verified:false};
  for(const[index,row]of evidence.observations.entries())if(row.kind==="read"||row.kind==="directory"){
    const operation=row.operation,sourceBytes=bytes(operation.source),source=sourceBytes===undefined?undefined:String(sourceBytes),path=relativePath(operation.source),input=resource.resources[index]!;
    if(!producer?.inputs.some(input=>input.kind==="file"&&resolve(input.path)===resolve(operation.source))||source===undefined){findings.push({code:"runtime-input-mismatch",path,detail:"Original build read operation is absent from the same consumed producer source unit"});continue;}
    const references=runtimeRustReferencesV1(source,path,{read:()=>undefined,features:producer.message.features,cfg:[]}).filter(reference=>reference.callsiteOffset!==undefined&&reference.expression===`Filesystem ${operation.name} requires actual read input ownership`&&source.slice(0,reference.callsiteOffset).split("\n").length===operation.line);
    if(references.length!==1||!input.sha256){findings.push({code:"runtime-unresolved-edge",path,detail:"Original build resource operation has no unique actual owned reader token and digest"});continue;}
    const entries=owners[path]??[],existing=entries.find(owner=>owner.callsiteOffset===references[0]!.callsiteOffset),value={path:relativePath(row.path),kind:row.kind==="read"?"file" as const:"directory" as const,sha256:input.sha256};if(existing)entries[entries.indexOf(existing)]={...existing,inputs:[...existing.inputs,value]};else entries.push({sourceSha256:createHash("sha256").update(source).digest("hex"),callsiteOffset:references[0]!.callsiteOffset!,inputs:[value]});owners[path]=entries;if(row.kind==="directory")directories[value.path]=value.sha256;
  }
  return{owners:findings.length?{}:owners,directories:findings.length?{}:directories,findings,verified:findings.length===0};
}

/** 🖼️ Binds actual build-script reads/copies and complete directory observations to current original bytes. */
export function runtimeBuildResourceInputsV1(source: string, context: { readonly root: string; readonly cwd: string; readonly outDirectory: string; readonly fixtureCollections?: readonly string[]; readonly read: (path: string) => string | Uint8Array | undefined; readonly directoryEntries: (path: string) => readonly RuntimeCompilerResourceEntryV1[] | undefined }): { readonly inputs: readonly string[]; readonly observations: readonly RuntimeBuildResourceObservationV1[]; readonly findings: readonly RuntimeGraphFindingV1[] } {
  const observations = source.split("\n").filter(Boolean).map(line => JSON.parse(line)), findings: RuntimeGraphFindingV1[] = [], inputs = new Set<string>();
  const normalized = (path: string): string => relative(context.root, resolve(context.cwd, path)).replaceAll("\\", "/"), out = normalized(context.outDirectory);
  const fixture = (path: string): void => { if (runtimeFixturePathV1(path, context.fixtureCollections)) findings.push({code:"runtime-fixture-edge",path,detail:"Actual build-producer filesystem input belongs to a fixture collection"}); };
  const digest = (bytes: string | Uint8Array | undefined): string | undefined => bytes === undefined ? undefined : createHash("sha256").update(bytes).digest("hex");
  for (const row of observations) {
    if (typeof row.path !== "string" || !["directory","copy","read","failed"].includes(row.kind)) throw Error("Build resource observation lost its input identity");
    const path = normalized(row.path); inputs.add(path); fixture(path);
    if (row.kind === "failed") { findings.push({code:"runtime-unresolved-edge",path,detail:"Actual original resource reader failed; absence/error has no complete successful input observation"});continue; }
    if(row.kind!=="copy"&&(!row.operation||typeof row.operation.source!=="string"||!Number.isSafeInteger(row.operation.line)||row.operation.line<1||row.operation.name!==(row.kind==="directory"?"read_dir":"read")))throw Error("Build resource reader lost its actual internal source operation");
    if (row.kind === "directory") {
      if (!Array.isArray(row.entries) || row.entries.some((entry:any)=>typeof entry.path!=="string"||!["file","directory","symlink","other"].includes(entry.kind)||(entry.kind==="symlink"?typeof entry.symlinkTarget!=="string":entry.symlinkTarget!==null)) || Object.keys(row).sort().join(",") !== "entries,kind,operation,path") throw Error("Build resource directory observation lost its original typed entry roster");
      const normalizedEntries=(entries:readonly RuntimeCompilerResourceEntryV1[])=>entries.map(entry=>({...entry,path:normalized(resolve(context.cwd,entry.path))})).sort((a,b)=>Buffer.compare(Buffer.from(basename(a.path)),Buffer.from(basename(b.path)))),entries=normalizedEntries(row.entries),actual=context.directoryEntries(path);
      for (const entry of entries) if (dirname(entry.path).replaceAll("\\","/") !== path) findings.push({code:"runtime-input-mismatch",path:entry.path,detail:"Observed entry does not belong to its actual resource directory"});
      if (!actual || JSON.stringify(entries) !== JSON.stringify(normalizedEntries(actual.map(entry=>({...entry,path:resolve(context.root,entry.path)}))))) findings.push({code:"runtime-input-mismatch",path,detail:"Current resource directory metadata differs from the actual original enumeration"});
    } else {
      if(row.kind==="read"){
        if(!Number.isSafeInteger(row.bytes)||row.bytes<0||!validDigest(row.sha256)||Object.keys(row).sort().join(",")!=="bytes,kind,operation,path,sha256")throw Error("Build read lost original consumed-byte identity");
        const bytes=context.read(path);if(bytes===undefined||byteCount(bytes)!==row.bytes||digest(bytes)!==row.sha256)findings.push({code:"runtime-input-mismatch",path,detail:"Current original resource differs from actual in-place consumed bytes"});continue;
      }
      if (typeof row.output !== "string" || Object.keys(row).sort().join(",") !== "kind,output,path") throw Error("Build copy lost original/output identity");
      const output = normalized(row.output), original = digest(context.read(path)), copied = digest(context.read(output));
      if (!output.startsWith(out + "/")) findings.push({code:"runtime-input-mismatch",path:output,detail:"Observed resource output does not belong to the selected actual build-script OUT_DIR"});
      if (!original || original !== copied) findings.push({code:"runtime-input-mismatch",path,detail:"Current original resource differs from the actual copied/read snapshot bytes"});
    }
  }
  return {inputs:[...inputs].sort(),observations,findings};
}

/** 📦️ Retains actual completed Cargo unit observations without requiring deleted capture directories. */
export function runtimeCargoProvenanceV1(source: string): readonly RuntimeCargoObservationV1[] {
  const observations: any[] = [decodeCargoProvenanceV1(source)];
  const digest = (value: unknown): boolean => typeof value === "string" && /^[0-9a-f]{64}$/u.test(value);
  for (const row of observations) {
    if (row.version !== 1 || row.command !== "cargo" || typeof row.manifest !== "string" || !isAbsolute(row.manifest) || typeof row.cwd !== "string" || !isAbsolute(row.cwd) || !Array.isArray(row.args) || !row.args.every((value: unknown) => typeof value === "string") || row.status !== 0 || row.cancelled !== false || !Number.isFinite(row.builtAtMs) || !Number.isFinite(row.observedAtMs) || row.observedAtMs < row.builtAtMs || !Array.isArray(row.units) || !Array.isArray(row.buildScripts)) throw Error("Cargo observation lost completed invocation identity");
    if (!(row.compilerResourceRoot===null || typeof row.compilerResourceRoot==="string"&&isAbsolute(row.compilerResourceRoot)) || !Array.isArray(row.compilerResources)) throw Error("Cargo observation lost explicit compiler resource ownership");
    for(const resource of row.compilerResources) if(typeof resource.path!=="string"||!isAbsolute(resource.path)||(resource.sha256!==null&&!digest(resource.sha256))||!Number.isFinite(resource.observedAtMs)||resource.observedAtMs>row.observedAtMs||!Array.isArray(resource.resources)||!resource.resources.every(resourceWitness)) throw Error("Cargo observation lost retained compiler resource capture");
    if (row.invocationInputs !== undefined && (!Array.isArray(row.invocationInputs) || !row.invocationInputs.every((input:any)=>typeof input.path==="string" && isAbsolute(input.path) && (input.sha256===null || digest(input.sha256))))) throw Error("Cargo observation lost invocation input digests");
    for (const unit of row.units) {
      const message = unit.message;
      if (message?.reason !== "compiler-artifact" || typeof message.package_id !== "string" || !Array.isArray(message.features) || !message.features.every((value: unknown) => typeof value === "string") || !Array.isArray(message.filenames) || !message.filenames.every((value: unknown) => typeof value === "string") || typeof message.target?.name!=="string" || typeof message.target?.src_path !== "string" || !Array.isArray(message.target.kind) || !message.target.kind.every((value: unknown) => typeof value === "string") || typeof message.profile?.test !== "boolean" || !Number.isFinite(unit.observedAtMs) || unit.observedAtMs < row.builtAtMs || unit.observedAtMs > row.observedAtMs || !Array.isArray(unit.depInfo) || !Array.isArray(unit.inputs) || !Array.isArray(unit.artifacts)) throw Error("Cargo observation lost production unit identity");
      for (const dep of unit.depInfo) if (typeof dep.path !== "string" || !digest(dep.sha256) || (typeof dep.baseDirectory!=="string" || !isAbsolute(dep.baseDirectory)) || !Array.isArray(dep.sources) || !dep.sources.every((value: unknown) => typeof value === "string") || !Array.isArray(dep.checksums)) throw Error("Cargo observation lost dep-info content");
      for (const input of unit.inputs) if (typeof input.path !== "string" || !["file","directory"].includes(input.kind) || !digest(input.sha256)) throw Error("Cargo observation lost input digest");
      for (const artifact of unit.artifacts) if (!message.filenames.includes(artifact.path) || !digest(artifact.sha256) || (artifact.stagedPath !== undefined && (typeof artifact.stagedPath !== "string" || artifact.stagedSha256 !== artifact.sha256))) throw Error("Cargo observation lost original/staged artifact binding");
    }
    for (const script of row.buildScripts) if (script.reason !== "build-script-executed" || typeof script.package_id !== "string" || typeof script.out_dir !== "string" || !Array.isArray(script.cfgs) || !script.cfgs.every((value: unknown) => typeof value === "string") || !Array.isArray(script.env) || !script.env.every((pair: unknown) => Array.isArray(pair) && pair.length === 2 && pair.every(value => typeof value === "string"))) throw Error("Cargo observation lost build-script cfg/env/OUT_DIR identity");
    if (row.buildResources !== undefined) {
      if (!Array.isArray(row.buildResources)) throw Error("Cargo observation lost build resource snapshots");
      for (const resource of row.buildResources) if (typeof resource.package_id !== "string" || typeof resource.out_dir !== "string" || typeof resource.path !== "string" || !digest(resource.sha256) || !Number.isFinite(resource.observedAtMs) || resource.observedAtMs < row.builtAtMs || resource.observedAtMs > row.observedAtMs || !Array.isArray(resource.resources) || !resource.resources.every(resourceWitness)) throw Error("Cargo observation lost build-time resource identity/digests");
    }
  }
  return observations;
}

/** 🔐️ Refuses current files that differ from bytes consumed by the actual selected compiler unit. */
export function runtimeCargoConsumedInputsV1(unit:Pick<RuntimeCargoUnitV1,"depInfo">&{readonly inputs:readonly {readonly path:string;readonly kind:"file"|"directory"}[]},read:(path:string)=>string|Uint8Array|undefined):readonly RuntimeGraphFindingV1[]{
  const checksums=unit.depInfo.flatMap(dep=>cargoDepInfoResolvedChecksumsV1(dep));
  return unit.inputs.filter(input=>input.kind!=="directory").flatMap(input=>{const rows=checksums.filter(row=>row.path===resolve(input.path)),value=read(input.path),bytes=typeof value==="string"?Buffer.from(value):value;
    if(!rows.length)return[{code:"runtime-unresolved-edge" as const,path:input.path,detail:"Actual compiler input lacks a consumed-byte checksum"}];
    return bytes===undefined||rows.some(row=>bytes.length!==row.length||blake3Hex(bytes)!==row.blake3)?[{code:"runtime-input-mismatch" as const,path:input.path,detail:"Current input differs from bytes consumed by actual rustc"}]:[];
  });
}

/** 🧭️ Derives target and requested feature selection from the original completed Cargo invocation. */
export function runtimeCargoInvocationV1(observation: RuntimeCargoObservationV1, root: string): { readonly target: string | null; readonly workspace: string; readonly features: readonly string[]; readonly packages: readonly string[]; readonly defaults: boolean; readonly allFeatures: boolean } {
  const values = (name: string, short?: string): string[] => observation.args.flatMap((value, index) => value === name || value === short ? [observation.args[index + 1] ?? ""] : value.startsWith(name + "=") ? [value.slice(name.length + 1)] : []);
  const manifests = values("--manifest-path");
  if (manifests.length > 1 || (manifests.length && resolve(observation.cwd, manifests[0]!) !== resolve(observation.manifest))) throw Error("Cargo observation manifest disagrees with original invocation");
  const targets = values("--target");
  if (targets.length > 1) throw Error("Cargo observation has ambiguous target selection");
  return { target: targets[0] ?? null, workspace: relative(root, observation.manifest).replaceAll("\\", "/"), features: [...new Set(values("--features").flatMap(value => value.split(/[\s,]+/u).filter(Boolean)))].sort(), packages: values("--package", "-p").sort(), defaults: !observation.args.includes("--no-default-features"), allFeatures: observation.args.includes("--all-features") };
}

/** ♻️ Reuses only the newest current observation of each exact declared runtime invocation. */
export async function runtimeCargoObservationsForChecksV1(observations:readonly RuntimeCargoObservationV1[],checks:readonly {workspace:string;target:string;packages:readonly string[];features:readonly string[]}[],root:string,current:(observation:RuntimeCargoObservationV1)=>Promise<boolean>):Promise<readonly RuntimeCargoObservationV1[]>{
  const selected:RuntimeCargoObservationV1[]=[];
  for(const check of checks){const candidates:RuntimeCargoObservationV1[]=[];for(const observation of observations){const selection=runtimeCargoInvocationV1(observation,root);if(selection.workspace===check.workspace&&selection.target===check.target&&selection.defaults&&!selection.allFeatures&&JSON.stringify(selection.packages)===JSON.stringify([...check.packages].sort())&&JSON.stringify(selection.features)===JSON.stringify([...check.features].sort())&&await current(observation))candidates.push(observation);}candidates.sort((a,b)=>b.observedAtMs-a.observedAtMs);if(candidates[0])selected.push(candidates[0]);}return selected;
}


/** 🏅️ Binds current package cdylib selection and original-to-final component custody to one production compiler unit. */
export function runtimeActorCargoUnitV1(observation:RuntimeCargoObservationV1,selection:RuntimeActorCargoSelectionV1,context:{root:string;arguments:readonly string[];read:(path:string)=>string|Uint8Array|undefined}):RuntimeCargoUnitV1|undefined{
  const args=context.arguments,expected=["rustc","-p",selection.cargoPackage,"--lib","--crate-type","cdylib","--target","wasm32-wasip2","--profile","wasm-release","--message-format=json"];
  if(!Array.isArray(args)||args.length!==expected.length+2||args[1]!=="--manifest-path"||typeof args[2]!=="string"||!isAbsolute(args[2])||JSON.stringify([args[0],...args.slice(3)])!==JSON.stringify(expected))return;
  const manifest=args[2]!;
  if(observation.status!==0||observation.cancelled!==false||resolve(observation.cwd)!==resolve(context.root)||resolve(observation.manifest)!==resolve(manifest)||JSON.stringify(observation.args)!==JSON.stringify(args))return;
  const units=observation.units.filter(unit=>{try{return resolve(unit.message.manifest_path!)===resolve(manifest)&&unit.message.profile.test===false&&!unit.message.features.some(feature=>["mutation-testing","artifact-app-testing"].includes(feature))&&unit.message.target.kind.some(kind=>["lib","cdylib"].includes(kind))&&(Bun.TOML.parse(String(context.read(unit.message.manifest_path!)))as{package:{name:string}}).package.name===selection.cargoPackage&&unit.artifacts.some(artifact=>artifact.stagedPath===selection.componentPath&&artifact.sha256===selection.componentSha256&&artifact.stagedSha256===selection.componentSha256&&artifact.path.endsWith(".wasm"));}catch{return false;}});
  return units.length===1?units[0]:undefined;
}

/** 🌱️ Requires surviving artifacts and complete current Cargo configuration before automatic reuse. */
export async function runtimeCargoObservationCurrentV1(observation:RuntimeCargoObservationV1,context:{root:string;cwd:string;outDirectory:string;buildDirectory:string;cargoHome:string;fixtureCollections?:readonly string[];physical:CurrentPhysicalPortV1}):Promise<boolean>{
 const physical=context.physical;if(!physical||typeof physical.checkpoint!=="function"||typeof physical.read!=="function"||typeof physical.digest!=="function"||typeof physical.directoryEntries!=="function"||typeof physical.recheck!=="function")throw Error("Runtime currentness requires complete asynchronous physical ownership");await physical.checkpoint();
 if(!observation.buildDirectory||resolve(observation.buildDirectory)!==resolve(context.buildDirectory)||!observation.invocationInputs?.length)return false;
 const units=observation.units.filter(unit=>!unit.message.profile.test),manifests=[observation.manifest,...units.map(unit=>unit.message.manifest_path)];if(!units.length||manifests.some(path=>typeof path!=="string"))return false;
 const captured=new Map(observation.invocationInputs.map(input=>[resolve(input.path),input.sha256]));for(const input of observation.invocationInputs){await physical.checkpoint();if((await physical.digest(input.path)??null)!==input.sha256)return false;}
 for(let path of[observation.cwd,...(manifests as string[]).map(dirname)])for(;;){await physical.checkpoint();for(const name of["Cargo.toml","Cargo.lock",".cargo/config.toml",".cargo/config","rust-toolchain.toml","rust-toolchain"])if(!captured.has(join(path,name)))return false;const parent=dirname(path);if(parent===path)break;path=parent;}
 if(["config.toml","config"].some(name=>!captured.has(join(context.cargoHome,name)))||(manifests as string[]).some(path=>!captured.has(resolve(path))))return false;
 const packageNames=new Set<string>();for(const unit of units){await physical.checkpoint();if(!unit.message.target.kind.some(kind=>["lib","rlib","cdylib","bin","proc-macro"].includes(kind)))continue;const value=await physical.read(unit.message.manifest_path!);try{const name=(Bun.TOML.parse(value===undefined?"":typeof value==="string"?value:new TextDecoder("utf-8",{fatal:true}).decode(value))as{package?:{name?:string}}).package?.name;if(name)packageNames.add(name);}catch{}}
 if(runtimeCargoInvocationV1(observation,context.root).packages.some(name=>!packageNames.has(name)))return false;
 const artifactDigests=new Map<string,string|undefined>();
 const views=async(paths:readonly string[],directories:readonly string[]=[],digestOnly:readonly string[]=[])=>{const bytes=new Map<string,string|Uint8Array|undefined>(),digests=new Map<string,string|undefined>(),entries=new Map<string,readonly RuntimeCompilerResourceEntryV1[]|undefined>();for(const path of new Set(paths.map(path=>resolve(context.root,path)))){await physical.checkpoint();const value=await physical.read(path);bytes.set(path,typeof value==="string"||value===undefined?value:Buffer.from(value.buffer,value.byteOffset,value.byteLength));digests.set(path,value===undefined?undefined:createHash("sha256").update(value).digest("hex"));}for(const path of new Set(digestOnly.map(path=>resolve(context.root,path)))){await physical.checkpoint();if(!digests.has(path)){if(!artifactDigests.has(path))artifactDigests.set(path,await physical.digest(path));digests.set(path,artifactDigests.get(path));}}for(const path of new Set(directories.map(path=>resolve(context.root,path)))){await physical.checkpoint();const rows=await physical.directoryEntries(path);entries.set(path,rows?.map(row=>Object.freeze({...row})));digests.set(path,rows===undefined?undefined:createHash("sha256").update(JSON.stringify(rows.map(row=>[basename(row.path),row.kind,row.symlinkTarget]))).digest("hex"));}const required=<T>(rows:Map<string,T>,path:string):T=>{const absolute=resolve(context.root,path);if(!rows.has(absolute))throw Error("Runtime currentness has an unadmitted physical view: "+absolute);return rows.get(absolute)!;};return{...context,read:(path:string)=>required(bytes,path),digest:(path:string)=>required(digests,path),directoryEntries:(path:string)=>required(entries,path),entries:(path:string)=>required(entries,path)?.map(entry=>[basename(entry.path),entry.kind,entry.symlinkTarget]as const)};};
 for(const unit of units){await physical.checkpoint();const paths=[...unit.inputs.filter(input=>input.kind!=="directory").map(input=>input.path),...unit.depInfo.flatMap(dep=>cargoDepInfoResolvedSourcesV1(dep))],artifacts=unit.artifacts.map(artifact=>artifact.stagedPath??artifact.path),admitted=await views(paths,unit.inputs.filter(input=>input.kind==="directory").map(input=>input.path),artifacts);for(const artifact of unit.artifacts)if(admitted.digest(artifact.stagedPath??artifact.path)!==artifact.sha256)return false;if(!runtimeCargoUnitInputsV1(observation,unit,context.root,path=>admitted.digest(resolve(context.root,path))).verified||runtimeCargoConsumedInputsV1(unit,admitted.read).length)return false;}
 for(const resource of observation.buildResources??[]){await physical.checkpoint();const paths=[resource.path,...resource.resources.flatMap(row=>[row.input.path,...(row.input.kind==="copy"?[row.input.output]:[])])],directories=resource.resources.filter(row=>row.input.kind==="directory").map(row=>row.input.path),admitted=await views(paths.filter(path=>!directories.includes(path)),directories);if(!runtimeRetainedBuildResourceInputsV1(observation,resource,{...admitted,outDirectory:resource.out_dir}).verified)return false;}
 for(const resource of observation.compilerResources??[]){await physical.checkpoint();const directories=[...resource.resources.filter(row=>row.input.kind==="directory").map(row=>row.input.path),...observation.units.flatMap(unit=>unit.inputs.filter(input=>input.kind==="directory").map(input=>input.path))],paths=[resource.path,...resource.resources.flatMap(row=>[row.input.path,row.input.operation.source,row.input.callsite.source]),...observation.units.flatMap(unit=>[...unit.inputs.filter(input=>input.kind!=="directory").map(input=>input.path),...unit.depInfo.flatMap(dep=>cargoDepInfoResolvedSourcesV1(dep))])],artifacts=observation.units.flatMap(unit=>unit.artifacts.map(artifact=>artifact.stagedPath??artifact.path)),admitted=await views(paths.filter(path=>!directories.includes(path)),directories,artifacts);if(!runtimeCompilerResourceInputsV1(observation,resource,admitted).verified)return false;}
 if(!units.flatMap(unit=>unit.inputs.filter(input=>input.kind==="directory")).every(input=>(observation.compilerResources??[]).some(resource=>resource.resources.some(row=>row.input.kind==="directory"&&resolve(row.input.path)===resolve(input.path)))))return false;
 await physical.recheck();return true;
}

/** 🏭️ Preserves actual build-script cfg/environment records in their selected guest invocation. */
export function runtimeCargoBuildScriptsV1(source: string): readonly (RuntimeCargoBuildScriptV1 & { readonly runtimeTarget: string | null; readonly runtimeWorkspace: string | null })[] {
  let runtimeTarget: string | null = null, runtimeWorkspace: string | null = null;
  return source.split("\n").flatMap(line => {
    const selection = /guest-framework-check (wasm32-[a-z0-9-]+) \(([^)]+)\):/u.exec(line);
    if (selection) { runtimeTarget = selection[1]!; runtimeWorkspace = selection[2]!; }
    const start = line.indexOf('{"reason":"build-script-executed"');
    if (start < 0) return [];
    const row = JSON.parse(line.slice(start));
    if (typeof row.package_id !== "string" || typeof row.out_dir !== "string" || !Array.isArray(row.cfgs) || !Array.isArray(row.env)) throw Error("Build-script record lost cfg/env/OUT_DIR identity");
    return [{ ...row, runtimeTarget, runtimeWorkspace }];
  });
}

/** 🔒️ Binds retained dep-info to every current compiler input and any surviving staged artifact. */
export function runtimeCargoUnitInputsV1(observation: RuntimeCargoObservationV1, unit: RuntimeCargoUnitV1, root: string, digest: (path: string) => string | undefined): { readonly inputs: readonly string[]; readonly findings: readonly RuntimeGraphFindingV1[]; readonly verified: boolean } {
  const findings: RuntimeGraphFindingV1[] = [], path = (value: string): string => relative(root, resolve(observation.cwd, value)).replaceAll("\\", "/");
  const inputs = [...new Set(unit.depInfo.flatMap(dep => cargoDepInfoResolvedSourcesV1(dep).map(value => relative(root,value).replaceAll("\\","/"))))].sort();
  const hashes = new Map(unit.inputs.map(input => [path(input.path), input.sha256]));
  if (!inputs.length || !inputs.includes(path(unit.message.target.src_path)) || !unit.artifacts.length) findings.push({ code: "runtime-unresolved-edge", path: path(unit.message.target.src_path), detail: "Retained production unit lacks complete root/dep-info/artifact observation" });
  for (const input of inputs) if (!hashes.has(input) || digest(input) !== hashes.get(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Current compiler input differs from retained actual unit digest" });
  for (const input of hashes.keys()) if (!inputs.includes(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Retained input digest is absent from actual dep-info roster" });
  for (const artifact of unit.artifacts) {
    const actual = digest(path(artifact.stagedPath ?? artifact.path));
    if ((artifact.stagedPath !== undefined && actual === undefined) || (actual !== undefined && actual !== artifact.sha256)) findings.push({ code: "runtime-input-mismatch", path: path(artifact.stagedPath ?? artifact.path), detail: "Current artifact differs from observed original/staged artifact digest" });
  }
  return { inputs, findings, verified: findings.length === 0 && unit.message.profile.test === false };
}

/** 🏗️ Tracks host build/proc-macro units separately from target libraries and their dependency closure. */
export function runtimeCargoUnitsV1(roots: readonly string[], packages: readonly CargoPackageV1[], nodes: readonly CargoNodeV1[]): readonly { readonly id: string; readonly host: boolean }[] {
  const pending = roots.map(id => ({ id, host: false })), units = new Map<string, { id: string; host: boolean }>();
  for (const unit of pending) {
    if (packages.find(pkg => pkg.id === unit.id)?.targets.some(target => target.kind.includes("proc-macro"))) unit.host = true;
    const key = unit.id + "|" + unit.host;
    if (units.has(key)) continue;
    units.set(key, unit);
    for (const dependency of nodes.find(node => node.id === unit.id)?.deps ?? []) for (const kind of dependency.dep_kinds) if (kind.kind !== "dev") pending.push({ id: dependency.pkg, host: unit.host || kind.kind === "build" });
  }
  return [...units.values()];
}

/** 📎️ Decodes actual rustc dep-info input paths without splitting escaped filesystem spaces. */
export function runtimeDepInfoInputsV1(source: string, cwd: string): readonly string[] {
  if (!source.split(/\r?\n/u).some(row => row.includes(": "))) throw Error("Actual rustc dep-info has no input record");
  return [...new Set(cargoDepInfoSourcesV1(source).map(path => relative(cwd, resolve(cwd, path)).replaceAll("\\", "/")))].sort();
}

/** 🧾️ Normal and build dependency edges exclude test-only dependencies without guessing from crate names. */
export function runtimeCargoPackagesV1(roots: readonly string[], nodes: readonly CargoNodeV1[]): readonly string[] {
  const index = new Map(nodes.map(node => [node.id, node])), reached = new Set<string>(), pending = [...roots];
  for (const id of pending) { if (reached.has(id)) continue; reached.add(id); for (const dependency of index.get(id)?.deps ?? []) if (dependency.dep_kinds.some(kind => kind.kind !== "dev")) pending.push(dependency.pkg); }
  return [...reached].sort();
}

/** 🧊️ Reads complete compiler-artifact records, preserving resolved feature and target identities. */
export function runtimeCompilerArtifactsV1(source: string): readonly RuntimeCompilerInputV1[] {
  let runtimeTarget: string | null = null, runtimeWorkspace: string | null = null;
  return source.split("\n").flatMap(line => {
    const selection = /guest-framework-check (wasm32-[a-z0-9-]+) \(([^)]+)\):/u.exec(line);
    if (selection) { runtimeTarget = selection[1]!; runtimeWorkspace = selection[2]!; }
    const start = line.indexOf('{"reason":"compiler-artifact"');
    if (start < 0) return [];
    let row; try { row = JSON.parse(line.slice(start)); } catch { throw Error("Incomplete compiler-artifact JSON record"); }
    if (!Array.isArray(row.features) || typeof row.package_id !== "string" || typeof row.target?.src_path !== "string" || typeof row.profile?.test !== "boolean") throw Error("Compiler artifact lost feature/target/profile identity");
    return [{ ...row, runtimeTarget, runtimeWorkspace }];
  });
}

/** 🧩️ Reconciles only unknown macro expansion against the complete selected production unit input roster. */
export function reconcileRuntimeMacroInputsV1(evidence: RuntimeGraphEvidenceV1, inputs: readonly string[]): RuntimeGraphEvidenceV1 {
  const compiled = new Set(inputs), root = evidence.roots[0];
  if (!root || !evidence.roots.every(path => compiled.has(path))) return evidence;
  return { ...evidence, nodes: [...new Set([...evidence.nodes, ...inputs])].sort(), edges: [...evidence.edges, ...inputs.filter(path => !evidence.nodes.includes(path)).map(to => ({ from: root, to, kind: "dependency" as const }))], findings: evidence.findings.filter(finding => !(finding.code === "runtime-unresolved-edge" && compiled.has(finding.path) && finding.detail.endsWith("! requires actual expansion/input provenance"))) };
}

/** 🛡️ Executes actual Cargo target metadata and runtime source graphs, retaining qualified evidence rather than inferring compilation. */
/** 🎭️ Resolves the actual Dev-selected closed actor generation and its original package/factory compiler custody. */
export async function verifyRuntimeFixtureGraphV1(root: string, args: readonly string[], actorPort:SelectedRuntimeActorPublicationPortV1, packageRoots:SelectedRuntimeGraphPackageRootsV1): Promise<void> {
  packageRoots=parseSelectedRuntimeGraphPackageRootsV1(packageRoots);
  for (let index = 0; index < args.length; index += 2) if (!["--report", "--compiler-artifacts", "--compiler-provenance", "--asset-owners", "--bundle-inputs", "--dynamic-imports"].includes(args[index]!) || !args[index + 1]) throw Error("runtime-graph accepts --report, --compiler-artifacts, --compiler-provenance, --asset-owners, --dynamic-imports and --bundle-inputs paths");
  const option = (name: string): string | undefined => { const index = args.indexOf(name); if (index < 0) return; if (!args[index + 1]) throw Error(`${name} needs a path`); return resolve(root, args[index + 1]!); };
  const report = option("--report") ?? join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoCacheDirectory(root,"runtime-fixture-graph","reports"),"runtime-fixture-graph.json");
  const artifacts = option("--compiler-artifacts"), artifactText = artifacts ? readFileSync(artifacts, "utf8") : "", compiler = [...runtimeCompilerArtifactsV1(artifactText)], buildScripts = [...runtimeCargoBuildScriptsV1(artifactText)];
  const provenance = option("--compiler-provenance"), receiptText = provenance ? readFileSync(provenance, "utf8") : undefined;
  const receiptPaths = receiptText && Array.isArray(JSON.parse(receiptText)) && JSON.parse(receiptText).every((value: unknown) => typeof value === "string") ? JSON.parse(receiptText).map((path: string) => resolve(dirname(provenance!), path)) : provenance ? [provenance] : [];
  const observations = receiptPaths.flatMap((path: string) => runtimeCargoProvenanceV1(readFileSync(path, "utf8")));
  const retainedUnits = new Map<RuntimeCompilerInputV1, { readonly observation: RuntimeCargoObservationV1; readonly unit: RuntimeCargoUnitV1; readonly selection: ReturnType<typeof runtimeCargoInvocationV1> }>();
  const retainedBuildScripts = new Map<(typeof buildScripts)[number], RuntimeCargoObservationV1>();
  const assets = option("--asset-owners"), assetOwners = assets ? JSON.parse(readFileSync(assets, "utf8")) : {};
  const bundle = option("--bundle-inputs"), bundledInputs = bundle ? JSON.parse(readFileSync(bundle, "utf8")) : undefined;
  const dynamic = option("--dynamic-imports");let dynamicImports = dynamic ? JSON.parse(readFileSync(dynamic, "utf8")) : undefined;let actorSelection:SelectedRuntimeActorsV1|undefined;
  const config = JSON.parse(readFileSync(join(import.meta.dir, "../🔣️.json"), "utf8"));
  const declaration = JSON.parse(readFileSync(join(root, config.guestChecks), "utf8"));
  const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
  const fixtureCollections = ["fixtures", "test-tube-fixtures"].flatMap(id => ["fixtures", "examples"].filter(slug => new RegExp(taxonomy.semanticDirectoryKinds[id].slugPattern, "u").test(slug)).map(slug => taxonomy.semanticDirectoryKinds[id].emoji + slug));
  const checks = declaration.guestFrameworkChecks as readonly { readonly target: string; readonly workspace: string; readonly packages: readonly string[]; readonly features: readonly string[] }[];
  const runtimeChecks:{target:string;workspace:string;packages:readonly string[];features:readonly string[];requestedPackages?:readonly string[];witnessTarget?:string|null;binary?:boolean;observation?:RuntimeCargoObservationV1}[]=[...checks];
  const durableLedger=join(cargoDirectories(root).build,"semio-cargo-provenance");
  const findings: RuntimeGraphFindingV1[] = [], graphs: { readonly selection: unknown; readonly evidence: RuntimeGraphEvidenceV1; readonly resolvedFeatures?: readonly string[] }[] = [], identities = new Map<string, string>();
  const signal = new AbortController(), cancel = (): void => signal.abort();
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  const checkCancellation = (): void => { if (signal.signal.aborted) throw Error("Runtime graph canceled"); };
  const observationStarted=Date.now(),observationBudget=cmdBudgetMs(),observation:FileObservationControlV1={maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>signal.signal.aborted,remainingMs:()=>(observationBudget>0?observationBudget:86_400_000)-(Date.now()-observationStarted),onProgress:step=>console.log(`[runtime graph] physical file ${step.phase} ${step.bytes}/${step.totalBytes}`)};
  const cargoObservation = cargoProvenancePhysicalControlV1({ cancelled: () => signal.signal.aborted, deadlineMs: 86_400_000, remainingMs: () => (observationBudget > 0 ? observationBudget : 86_400_000) - (Date.now() - observationStarted), onProgress: step => console.log(`[runtime graph] physical cargo ${step.phase} ${step.bytes}/${step.totalBytes}`) });
  const read = (path: string): string | undefined => {
    checkCancellation(); const absolute = resolve(root, path);
    if (!existsSync(absolute)) return;
    const actual = realpathSync(absolute), rel = relative(root, actual).replaceAll("\\", "/");
    if (runtimeFixturePathV1(rel, fixtureCollections) && !runtimeFixturePathV1(path, fixtureCollections)) findings.push({ code: "runtime-fixture-edge", path, detail: `Physical source/resource resolves to fixture owner ${rel}` });
    try { const bytes = readFileSync(actual),sha256=createHash("sha256").update(bytes).digest("hex"),previous=identities.get(rel); if(previous && previous!==sha256)findings.push({code:"runtime-input-mismatch",path:rel,detail:"Source/resource changed during actual runtime graph observation"});else identities.set(rel,sha256); return bytes.toString("utf8"); } catch { return; }
  };
  const execute = async (command: string[]): Promise<string> => {
    checkCancellation(); if(command[0]==="cargo") prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(root),root,command.slice(1),root,process.env); const child = Bun.spawn(command, { cwd: root, stdout: "pipe", stderr: "pipe", signal: signal.signal });
    const [stdout, stderr, exit] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    if (exit !== 0) throw Error(`${command.join(" ")} failed ${exit}: ${stderr}`); return stdout;
  };
  try {
    if (!provenance && !artifacts) {
      const ledger = durableLedger;
      const load=():readonly RuntimeCargoObservationV1[]=>existsSync(ledger)?readdirSync(ledger).filter(name=>name.startsWith("cargo-unit-provenance-")&&name.endsWith(".json")).flatMap(name=>{try{return runtimeCargoProvenanceV1(readFileSync(join(ledger,name),"utf8"))}catch{return []}}):[];
      const physical=new CurrentPhysicalOwnerV1(root,cargoObservation),current=(receipt:RuntimeCargoObservationV1):Promise<boolean>=>runtimeCargoObservationCurrentV1(receipt,{root,cwd:receipt.cwd,outDirectory:"",buildDirectory:cargoDirectories(root).build,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),fixtureCollections,physical});
      let completed=await runtimeCargoObservationsForChecksV1(load(),checks,root,current);
      if (completed.length!==checks.length) {
        const producer=declaration.steps.find((step:any)=>step.command?.[0]==="nx"&&step.command?.[1]==="run"&&step.command?.[2]?.endsWith(":guest-framework-check"));
        if (!producer) throw Error("Runtime graph has no registered actual guest compiler producer");
        console.log(`[runtime graph] acquiring current exact guest compiler/resource witnesses through ${producer.command[2]}`);
        mkdirSync(ledger,{recursive:true});
        await runTool(process.execPath,[join(root,"node_modules/nx/dist/bin/nx.js"),...producer.command.slice(1),"--skip-nx-cache","--outputStyle=static"],root,signal.signal,false,{...process.env,CARGO_TARGET_DIR:cargoDirectories(root).target,CARGO_BUILD_BUILD_DIR:cargoDirectories(root).build,SEMIO_TEST_ARTIFACT_DIR:ledger},repositoryCargoPreparationStorageV1(process.cwd()));
        completed=await runtimeCargoObservationsForChecksV1(load(),checks,root,current);
        if (completed.length!==checks.length) throw Error("Registered guest compiler producer did not retain all current exact invocation/input/resource witnesses");
      }
      observations.push(...completed);
    }
    const loadActors=()=>observeSelectedRuntimeActorsV1(actorPort,observation);
    try{try{actorSelection=await loadActors();}catch(error){
      if(provenance||artifacts||dynamic)throw error;
      await acquireSelectedRuntimeActorsV1(actorPort,observation);actorSelection=await loadActors();
    }}catch(error){findings.push({code:"runtime-unresolved-edge",path:actorPort.diagnosticOwner,detail:`Actual selected actor producer proof failed: ${String(error)}`});}
    if(actorSelection){dynamicImports={...dynamicImports,...actorSelection.dynamicImports};observations.push(...actorSelection.observations);
      const version=await execute(["rustc","-vV"]),hostTarget=version.split("\n").find(line=>line.startsWith("host: "))?.slice(6);if(!hostTarget)throw Error("Actual Rust host target unavailable for selected actor descriptor producer");
      for(const observation of actorSelection.observations){const selected=runtimeCargoInvocationV1(observation,root);runtimeChecks.push({target:selected.target??hostTarget,workspace:selected.workspace,packages:selected.packages,features:selected.features,requestedPackages:selected.packages,witnessTarget:selected.target,binary:selected.target===null,observation});}
      for(const witness of actorSelection.witnesses)graphs.push({selection:{language:"typescript",kind:"selected-closed-actor",dataRoot:actorSelection.dataRoot,profile:actorSelection.profile,pointer:actorSelection.published.record.pointer,package:witness.packageId,actor:witness.actor,producer:witness.producer,importOwner:witness.owner},evidence:witness.closed});
    }
  for (const observation of observations) {
    const selection = runtimeCargoInvocationV1(observation, root);
    for (const unit of observation.units) { const message = { ...unit.message, runtimeTarget: selection.target, runtimeWorkspace: selection.workspace }; compiler.push(message); retainedUnits.set(message, { observation, unit, selection }); }
    for (const script of observation.buildScripts) { const record = { ...script, runtimeTarget: selection.target, runtimeWorkspace: selection.workspace }; buildScripts.push(record); retainedBuildScripts.set(record, observation); }
  }
    const workspaceFiles = policyWalkRelFiles(root, packageRoots, (_, name) => name === "package.json");
    const aliases: Record<string, string> = {};
    for (const path of workspaceFiles) {
      const pkg = JSON.parse(readFileSync(join(root, path), "utf8"));
      const exported = (value: unknown): string | undefined => {
        if (typeof value === "string") return value;
        if (!value || typeof value !== "object") return;
        for (const [condition, target] of Object.entries(value)) if (["browser", "import", "default", "module"].includes(condition)) { const path = exported(target); if (path) return path; }
      };
      for (const [key, value] of Object.entries(typeof pkg.exports === "string" ? { ".": pkg.exports } : pkg.exports ?? {})) { const entry = exported(value); if (entry && pkg.name) aliases[pkg.name + (key === "." ? "" : key.slice(1))] = relative(root, resolve(dirname(join(root, path)), entry)).replaceAll("\\", "/"); }
    }
    const productionBoundary = read(config.productionTestBoundaryOwner);
    if (!productionBoundary?.includes('"import.meta.vitest": "undefined"')) throw Error("Actual browser production test boundary is unavailable");
    const ts = inspectRuntimeGraphV1(config.typescriptEntries, { read, fixtureCollections, aliases, assetOwners, dynamicImports, compiledInputs: bundledInputs, workingDirectory: ".", productionTests: "excluded", checkCancellation });
    graphs.push({ selection: { language: "typescript", entries: config.typescriptEntries }, evidence: ts }); findings.push(...ts.findings);
    try {
      const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
      const { renderWgpuBrowserBundles } = await import(join(root, config.browserProducer));
      const browser = await renderWgpuBrowserBundles(root, profile, { taxonomy, isCancelled: () => signal.signal.aborted });
      const browserAliases = Object.fromEntries(Object.entries(profile.workspaceImports).map(([name, value]: [string, any]) => [name, value.entryPath]));
      const browserRoots = profile.entries.map((entry: any) => profile.ownerPath + "/" + entry.sourceRelativePath);
      const outputOwners = Object.fromEntries(profile.entries.map((entry: any)=>[profile.ownerPath+"/"+entry.outputRelativePath,profile.ownerPath+"/"+entry.sourceRelativePath]));
      const { wgpuBrowserMounts } = await import(join(root,config.browserMountSource));
      const { wgpuCompletedFrameworkRootsV1, wgpuCompletedProfileV1 } = await import(join(root,config.browserMountConfig));
      read(config.browserMountSource); read(config.browserMountConfig);
      const completedProfile = wgpuCompletedProfileV1(), roots = wgpuCompletedFrameworkRootsV1(completedProfile);
      const nativeOwner=join(root,config.nativeBrowserProducer),nativeManifest=join(dirname(dirname(roots.compilerRoot)),"Cargo.toml"),nativeLedger=join(cargoDirectories(root).build,"semio-trunk-provenance");
      const physicalOwner=new CurrentPhysicalOwnerV1(root,cargoObservation),currentCompiler=(receipt:RuntimeCargoObservationV1,physical:CurrentPhysicalPortV1):Promise<boolean>=>runtimeCargoObservationCurrentV1(receipt,{root,cwd:receipt.cwd,outDirectory:"",buildDirectory:cargoDirectories(root).build,cargoHome:resolve(process.env.CARGO_HOME??join(homedir(),".cargo")),fixtureCollections,physical});
      const loadNative=async():Promise<{receipt:RuntimeTrunkObservationV1;compiler:RuntimeCargoObservationV1}|undefined>=>{const candidates:{receipt:RuntimeTrunkObservationV1;compiler:RuntimeCargoObservationV1}[]=[];if(existsSync(nativeLedger))for(const name of readdirSync(nativeLedger)){let receipt:RuntimeTrunkObservationV1;try{receipt=JSON.parse(readFileSync(join(nativeLedger,name,"trunk.json"),"utf8"));}catch{continue;}const compiler=await runtimeTrunkObservationCurrentV1(receipt,{root,profile:completedProfile,manifest:nativeManifest,outputDirectory:roots.compilerRoot,owner:nativeOwner,physical:physicalOwner,compilerCurrent:currentCompiler});if(compiler)candidates.push({receipt,compiler});}candidates.sort((a,b)=>b.receipt.observedAtMs-a.receipt.observedAtMs);return candidates[0];};
      let native=await loadNative();
      if(!native&&!provenance&&!artifacts){console.log(`[runtime graph] acquiring actual native browser mount through ${config.nativeBrowserProject}:${completedProfile==="dev"?"wasm":"wasm-release"}`);await runTool(process.execPath,[join(root,"node_modules/nx/dist/bin/nx.js"),"run",config.nativeBrowserProject+":"+(completedProfile==="dev"?"wasm":"wasm-release"),"--skip-nx-cache","--outputStyle=static"],root,signal.signal,false,{...process.env,CARGO_TARGET_DIR:cargoDirectories(root).target,CARGO_BUILD_BUILD_DIR:cargoDirectories(root).build},repositoryCargoPreparationStorageV1(process.cwd()));native=await loadNative();if(!native)throw Error("Registered Trunk producer did not retain a current exact native mount/compiler/resource witness");}
      const nativePaths=new Set<string>(native?.receipt.outputs.map(output=>relative(root,output.path).replaceAll("\\","/"))??[]);
      if(native){const selection=runtimeCargoInvocationV1(native.compiler,root),name=(Bun.TOML.parse(readFileSync(nativeManifest,"utf8")) as {package:{name:string}}).package.name;for(const unit of native.compiler.units){const message={...unit.message,runtimeTarget:selection.target,runtimeWorkspace:selection.workspace};compiler.push(message);retainedUnits.set(message,{observation:native.compiler,unit,selection});}for(const script of native.compiler.buildScripts){const record={...script,runtimeTarget:selection.target,runtimeWorkspace:selection.workspace};buildScripts.push(record);retainedBuildScripts.set(record,native.compiler);}observations.push(native.compiler);runtimeChecks.push({target:selection.target!,workspace:selection.workspace,packages:[name],requestedPackages:selection.packages,features:selection.features});}
      const resourceMounts = Object.fromEntries(wgpuBrowserMounts(roots).filter((row: unknown[])=>typeof row[1]==="string").map(([route,path]:[string,string])=>[route,relative(root,path).replaceAll("\\","/")]));
      const outputs = new Map<string,string>(browser.nodes.map((node:any)=>[node.path,node.content]));
      const physical=(path:string):Uint8Array|undefined=>{try{return readFileSync(resolve(root,path))}catch{return undefined}};
      let mountedOutputs=browser.nodes.map((node:any)=>runtimeBrowserArtifactV1(node.path,node.content,physical));
      if(mountedOutputs.some(artifact=>!artifact.current)&&!provenance&&!artifacts){console.log(`[runtime graph] acquiring current browser mount through ${config.nativeBrowserProject}:generate-frame-worker`);await runTool(process.execPath,[join(root,"node_modules/nx/dist/bin/nx.js"),"run",config.nativeBrowserProject+":generate-frame-worker","--skip-nx-cache","--outputStyle=static"],root,signal.signal,false,{...process.env,CARGO_TARGET_DIR:cargoDirectories(root).target,CARGO_BUILD_BUILD_DIR:cargoDirectories(root).build},repositoryCargoPreparationStorageV1(process.cwd()));mountedOutputs=browser.nodes.map((node:any)=>runtimeBrowserArtifactV1(node.path,node.content,physical));}
      browser.nodes.forEach((node:any)=>read(node.path));
      for(const artifact of mountedOutputs)if(!artifact.current)findings.push({code:"runtime-unresolved-edge",path:artifact.path,detail:artifact.mountedSha256===null?"Fresh browser compiler output is not present at its actual server mount":"Actual server-mounted browser bytes differ from fresh compiler output"});
      const outputUrls = Object.fromEntries(browser.entryInputs.map((entry:any)=>{
        const mount = Object.entries(resourceMounts).find(([,path])=>path===dirname(entry.outputPath).replaceAll("\\","/"));
        if (!mount) throw Error("Actual browser output has no server mount owner: "+entry.outputPath);
        return [entry.outputPath,"https://runtime.invalid"+mount[0]+"/"+entry.outputPath.slice(entry.outputPath.lastIndexOf("/")+1)];
      }));
      const browserRead = (path:string):string|undefined=>outputs.get(path)??read(path);
      const browserNodes = new Set<string>(), browserEdges: {from:string;to:string;kind:"import"|"module"|"resource"|"dependency"|"generated"}[] = [];
      for (const entry of browser.entryInputs) {
        const source = outputOwners[entry.outputPath], moduleUrls = {...outputUrls,...Object.fromEntries(entry.inputs.map((path:string)=>[path,outputUrls[entry.outputPath]]))};
        const evidence = inspectRuntimeGraphV1([source], {read:browserRead,fixtureCollections,aliases:browserAliases,assetOwners,dynamicImports,moduleUrls,resourceMounts,workingDirectory:".",productionTests:"excluded",checkCancellation});
        evidence.nodes.forEach(path=>browserNodes.add(path)); browserEdges.push(...evidence.edges); findings.push(...evidence.findings);
        graphs.push({selection:{language:"typescript",producer:config.browserProducer,entry,source,realm:outputUrls[entry.outputPath],completedProfile,mountOwners:{source:config.browserMountSource,config:config.browserMountConfig,resourceMounts},outputs:mountedOutputs},evidence});
      }
      for (const binding of Object.values(profile.workspaceImports) as { readonly entryPath: string; readonly manifestPath: string }[]) if (browserNodes.has(binding.entryPath)) { browserNodes.add(binding.manifestPath); browserEdges.push({ from: binding.entryPath, to: binding.manifestPath, kind: "dependency" }); read(binding.manifestPath); }
      for (const path of browser.inputs) { read(path); if (!browserNodes.has(path)) findings.push({ code: "runtime-input-mismatch", path, detail: "Actual WGPU browser compiler input has no resolved source/resource graph edge" }); }
      const mountedResources = new Set(browserEdges.filter(edge=>edge.kind==="resource").map(edge=>edge.to));
      for (const path of browserNodes) if (!browser.inputs.includes(path) && !outputs.has(path) && !nativePaths.has(path)) findings.push({ code: mountedResources.has(path)?"runtime-unresolved-edge":"runtime-input-mismatch", path, detail: mountedResources.has(path)?"Actual browser runtime mounted resource needs current native producer/artifact ownership evidence":"WGPU source graph input is absent from actual browser compiler input roster" });
    } catch (error) { findings.push({ code: "runtime-unresolved-edge", path: config.browserProducer, detail: `Actual WGPU browser producer failed: ${String(error)}${error instanceof AggregateError ? "\n" + error.errors.map(String).join("\n") : ""}` }); }
    const hostCfg = (await execute(["rustc", "--print", "cfg"])).trim().split("\n");
    const compilerResourceCache=new Map<RuntimeCargoObservationV1,Map<string,ReturnType<typeof runtimeCompilerResourceReadOwnersV1>>>();
    for (const [index, check] of runtimeChecks.entries()) {
      console.log(`[runtime graph] actual Cargo metadata ${index + 1}/${checks.length} ${check.target}: ${check.packages.join(",")}`);
      const arguments_ = ["cargo", "metadata", "--locked", "--manifest-path", check.workspace, "--format-version", "1", "--filter-platform", check.target, ...(check.features.length ? ["--features", check.features.join(",")] : [])];
      const metadataText = await execute(arguments_), metadata = JSON.parse(metadataText) as { workspace_root:string; packages: CargoPackageV1[]; resolve: { nodes: CargoNodeV1[] } };
      mkdirSync(dirname(report), { recursive: true }); writeFileSync(join(dirname(report), `runtime-cargo-metadata-${index}.json`), metadataText);
      const cfg = (await execute(["rustc", "--print", "cfg", "--target", check.target])).trim().split("\n");
      const selected = check.packages.map(name => { const packages = metadata.packages.filter(pkg => pkg.name === name); if (packages.length !== 1) throw Error(`Selected Cargo package ${name} is not unique`); return packages[0]!.id; });
      for (const { id, host: dependencyHost } of runtimeCargoUnitsV1(selected, metadata.packages, metadata.resolve.nodes)) {
        const host=check.witnessTarget===null||dependencyHost,unitKinds=["lib","rlib","cdylib","proc-macro",...(check.binary&&selected.includes(id)?["bin"]:[])];
        const pkg = metadata.packages.find(pkg => pkg.id === id)!;
        if (pkg.source !== null) continue;
        const manifest = relative(root, pkg.manifest_path).replaceAll("\\", "/"); read(manifest);
        const messages = compiler.filter(row => {
          const retained = retainedUnits.get(row), selection = retained?.selection;
          return row.package_id === id && row.profile.test === false && row.runtimeTarget === (check.witnessTarget!==undefined?check.witnessTarget:check.target) && row.runtimeWorkspace === check.workspace && (!check.observation||retained?.observation===check.observation) && (!selection || selection.defaults && !selection.allFeatures && JSON.stringify(selection.features) === JSON.stringify([...check.features].sort()) && JSON.stringify(selection.packages) === JSON.stringify([...(check.requestedPackages??check.packages)].sort())) && pkg.targets.some(target => target.src_path === row.target.src_path) && row.target.kind.some(kind => unitKinds.includes(kind)) && row.filenames.some(path => host !== path.replaceAll("\\", "/").includes(`/${check.target}/`));
        });
        if (!messages.length) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: `No actual compiler-artifact feature witness for ${check.target} ${pkg.name}; metadata workspace/dev feature union is not runtime compilation proof` });
        const features = messages.length ? [...new Set(messages.flatMap(row => row.features))].sort() : metadata.resolve.nodes.find(node => node.id === id)?.features ?? [];
        const variants = new Set(messages.map(message => JSON.stringify([...message.features].sort())));
        if (variants.size > 1) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: "Production compiler witnesses have different resolved feature units; a package feature union cannot prove runtime ownership" });
        const roots = pkg.targets.filter(target => target.kind.some(kind => unitKinds.includes(kind))).map(target => relative(root, target.src_path).replaceAll("\\", "/"));
        const boundObservations = new Set(messages.flatMap(message => retainedUnits.has(message) ? [retainedUnits.get(message)!.observation] : []));
        const scripts = [...new Map(buildScripts.filter(script => script.package_id === id && script.runtimeTarget === (check.witnessTarget!==undefined?check.witnessTarget:check.target) && script.runtimeWorkspace === check.workspace && (!retainedBuildScripts.has(script) || boundObservations.has(retainedBuildScripts.get(script)!))).map(script => [JSON.stringify([script.out_dir, script.cfgs, script.env]), script])).values()];
        if (scripts.length > 1) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: "Actual build-script records have multiple cfg/env/OUT_DIR units; production units cannot share an aggregate environment" });
        const script = scripts.length === 1 ? scripts[0] : undefined;
        const digest = (path: string): string | undefined => { try { return cargoInputDigestV1(resolve(root,path)).sha256??undefined; } catch { return; } };
        const resourceWitnesses = [...boundObservations].flatMap(observation=>(observation.buildResources ?? []).filter(resource=>resource.package_id===id && script && resolve(resource.out_dir)===resolve(script.out_dir)).map(resource=>{
          const producers = observation.units.filter(unit=>unit.message.package_id===id && unit.message.target.kind.includes("custom-build") && !unit.message.profile.test);
          const producerInputs = producers.map(unit=>runtimeCargoUnitInputsV1(observation,unit,root,digest));
          const evidence = runtimeRetainedBuildResourceInputsV1(observation,resource,{root,cwd:observation.cwd,outDirectory:resource.out_dir,fixtureCollections,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directoryEntries:path=>cargoDirectoryEntriesV1(resolve(root,path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(root,path),name),kind:kind as RuntimeCompilerResourceEntryV1["kind"],symlinkTarget}))});
          findings.push(...evidence.findings,...producerInputs.flatMap(unit=>unit.findings));
          const verified = evidence.verified && producers.length===1 && producerInputs.every(unit=>unit.verified);
          if (!verified) findings.push({code:"runtime-unresolved-edge",path:manifest,detail:"Original build resource snapshots lack the same current verified custom-build production unit"});
          const reads=runtimeBuildResourceReadOwnersV1(observation,resource,{root,cwd:observation.cwd,outDirectory:resource.out_dir,fixtureCollections,read:path=>{try{return readFileSync(resolve(root,path))}catch{return undefined}},directoryEntries:path=>cargoDirectoryEntriesV1(resolve(root,path))?.map(([name,kind,symlinkTarget])=>({path:join(resolve(root,path),name),kind:kind as RuntimeCompilerResourceEntryV1["kind"],symlinkTarget}))});findings.push(...reads.findings);
          return {receipt:{path:resource.path,sha256:resource.sha256,observedAtMs:resource.observedAtMs,resources:resource.resources.length},producerUnits:producers.map(unit=>({source:unit.message.target.src_path,features:unit.message.features,inputs:unit.inputs,artifacts:unit.artifacts})),evidence,reads,verified:verified&&reads.verified};
        }));
        const compilerResourceGroups=[...boundObservations].map(observation=>{const cache=compilerResourceCache.get(observation)??new Map();compilerResourceCache.set(observation,cache);if(!cache.has(id))cache.set(id,runtimeCompilerResourceReadOwnersV1({...observation,compilerResources:observation.compilerResources.filter(resource=>resource.producerUnit?.package_id===id||resource.callerUnit?.package_id===id)},{root,digest:path=>cargoInputDigestV1(path).sha256??undefined,fixtureCollections,read:path=>{try{return readFileSync(path)}catch{return undefined}},entries:path=>cargoDirectoryEntriesV1(path)??undefined}));return cache.get(id)!;});
        const compilerResourceWitnesses=compilerResourceGroups.flatMap(group=>group.witnesses);findings.push(...compilerResourceWitnesses.flatMap(witness=>witness.evidence.findings));
        const mergedResourceReads=runtimeMergeResourceReadOwnersV1([...compilerResourceGroups,...resourceWitnesses.filter(witness=>witness.verified).map(witness=>witness.reads)]),resourceReads=mergedResourceReads.owners,directoryInputs=mergedResourceReads.directories;findings.push(...mergedResourceReads.findings);
        const generated = Object.fromEntries(resourceWitnesses.filter(witness=>witness.verified).flatMap(witness=>witness.evidence.observations.flatMap(row=>(row.kind!=="copy")?[]:[[relative(root,resolve(row.output)).replaceAll("\\","/"),[relative(root,resolve(row.path)).replaceAll("\\","/")]]])));
        const resourceInputs = new Set([...resourceWitnesses.filter(witness=>witness.verified).flatMap(witness=>witness.evidence.inputs),...compilerResourceWitnesses.filter(witness=>witness.evidence.verified).flatMap(witness=>[...witness.evidence.inputs,...witness.evidence.directories])]);
        const context = { read, fixtureCollections, features, cfg: [...(host ? hostCfg : cfg), ...(script?.cfgs ?? [])], environment: script ? { ...Object.fromEntries(script.env), OUT_DIR: script.out_dir.replaceAll("\\", "/") } : undefined, generated, resourceReads, directoryInputs, resourceDigest:(path:string,kind:"file"|"directory")=>{const input=cargoInputDigestV1(resolve(root,path));return input.kind===kind?input.sha256??undefined:undefined}, rootDirectory: root.replaceAll("\\", "/"), manifestDirectory: dirname(pkg.manifest_path).replaceAll("\\", "/"), checkCancellation };
        let evidence = inspectRuntimeGraphV1(roots, context);
        const inputs = new Set<string>();
        const witnesses: { readonly artifact: string; readonly artifactSha256: string; readonly depInfo: string; readonly depInfoSha256: string }[] = [];
        let fresh = variants.size <= 1 && scripts.length <= 1;
        const retainedWitnesses: unknown[] = [];
        for (const message of messages) {
          const retained = retainedUnits.get(message);
          if (retained) {
            const observed = runtimeCargoUnitInputsV1(retained.observation, retained.unit, root, digest);
            const consumed=runtimeCargoConsumedInputsV1(retained.unit,path=>{try{return readFileSync(path)}catch{return undefined}}); findings.push(...observed.findings,...consumed); fresh &&= observed.verified && consumed.length===0;
            retainedWitnesses.push({ manifest: retained.observation.manifest, cwd: retained.observation.cwd, args: retained.observation.args, builtAtMs: retained.observation.builtAtMs, observedAtMs: retained.unit.observedAtMs, depInfo: retained.unit.depInfo.map(dep => ({ path: dep.path, sha256: dep.sha256, baseDirectory: dep.baseDirectory, sources: dep.sources.length, checksums: dep.checksums.length })), artifacts: retained.unit.artifacts, inputDigests: retained.unit.inputs, verified: observed.verified });
            for (const input of observed.inputs) { inputs.add(input); if(retained.unit.inputs.find(value=>relative(root,resolve(value.path)).replaceAll("\\","/")===input)?.kind!=="directory")read(input); if (runtimeFixturePathV1(input, fixtureCollections)) findings.push({ code: "runtime-fixture-edge", path: input, detail: "Actual retained compiler input belongs to a fixture collection" }); }
            continue;
          }
          for (const file of message.filenames) {
          const depInfo = join(dirname(file), basename(file).replace(/^lib/u, "").replace(/\.[^.]+$/u, ".d"));
          if (!existsSync(depInfo) || !existsSync(file)) continue;
          witnesses.push({ artifact: file, artifactSha256: createHash("sha256").update(readFileSync(file)).digest("hex"), depInfo, depInfoSha256: createHash("sha256").update(readFileSync(depInfo)).digest("hex") });
          for (const input of runtimeDepInfoInputsV1(readFileSync(depInfo, "utf8"), metadata.workspace_root).map(path=>relative(root,resolve(metadata.workspace_root,path)).replaceAll("\\","/"))) {
            inputs.add(input); read(input);
            if (runtimeFixturePathV1(input, fixtureCollections)) findings.push({ code: "runtime-fixture-edge", path: input, detail: `Actual compiled ${host ? "host" : check.target} input belongs to a fixture collection` });
            if (!existsSync(join(root, input)) || statSync(join(root, input)).mtimeMs > statSync(file).mtimeMs) { fresh = false; findings.push({ code: "runtime-input-mismatch", path: input, detail: "Current input is missing or changed after actual compiler artifact" }); }
          }
          }
        }
        if (messages.length && !inputs.size) findings.push({ code: "runtime-unresolved-edge", path: manifest, detail: "Actual compiler artifact has no readable dep-info source/resource roster" });
        const macroInputWitnesses = fresh && inputs.size && roots.every(path => inputs.has(path)) ? evidence.findings.filter(finding => finding.code === "runtime-unresolved-edge" && inputs.has(finding.path) && finding.detail.endsWith("! requires actual expansion/input provenance")) : [];
        if (macroInputWitnesses.length) {
          const expanded = inspectRuntimeGraphV1([...roots, ...inputs], context);
          evidence = reconcileRuntimeMacroInputsV1({ ...expanded, roots }, [...inputs]);
        }
        if (inputs.size) {
          for (const input of evidence.nodes) if (!inputs.has(input) && !resourceInputs.has(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Source graph input is absent from actual compiler dep-info or verified original build-resource roster" });
          for (const input of inputs) if (!evidence.nodes.includes(input)) findings.push({ code: "runtime-input-mismatch", path: input, detail: "Actual compiler dep-info input has no resolved source/generated graph edge" });
        }
        graphs.push({ selection: { target: check.target, host, packages: check.packages, features: check.features, manifest, package: pkg.name, featureWitness: messages.length ? "compiler-artifact" : "metadata-broad", compilerUnits: messages.map(message => ({ srcPath: message.target.src_path, profile: message.profile, features: message.features, filenames: message.filenames })), compilerWitnesses: witnesses, retainedWitnesses, buildScript: script ?? null, resourceWitnesses, compilerResourceWitnesses: compilerResourceWitnesses.map(witness => ({ receipt: { path: witness.receipt.path, sha256: witness.receipt.sha256, observedAtMs: witness.receipt.observedAtMs, resources: witness.receipt.resources.length }, evidence: witness.evidence })), resourceReads, directoryInputs, macroInputWitnesses, compiledInputs: [...inputs] }, evidence, resolvedFeatures: features });
        findings.push(...evidence.findings.map(finding => messages.length && variants.size <= 1 || finding.code !== "runtime-fixture-edge" ? finding : { ...finding, code: "runtime-unresolved-edge" as const, detail: "Potential fixture resource under broad/ambiguous features; a single actual production feature unit is unavailable" }));
        for (const target of pkg.targets.filter(target => target.kind.includes("custom-build"))) {
          const source = relative(root, target.src_path).replaceAll("\\", "/"), build = inspectRuntimeGraphV1([source], { ...context, cfg:[...hostCfg,...(script?.cfgs??[])] });
          graphs.push({ selection: { target: "host", package: pkg.name, manifest, kind: "custom-build" }, evidence: build, resolvedFeatures: features });
          findings.push(...build.findings);
        }
      }
    }
  } catch(error) {
    findings.push({code:"runtime-unresolved-edge",path:config.guestChecks,detail:`Actual runtime witness acquisition/verification failed: ${String(error)}`});
  } finally {
    try{await actorSelection?.recheck(observation);}catch(error){findings.push({code:"runtime-input-mismatch",path:actorPort.diagnosticOwner,detail:String(error)});}
    for(const observation of observations)for(const resource of observation.compilerResources)findings.push(...runtimeCompilerResourceInputsV1(observation,resource,{root,digest:path=>cargoInputDigestV1(path).sha256??undefined,fixtureCollections,read:path=>{try{return readFileSync(path)}catch{return undefined}},entries:path=>cargoDirectoryEntriesV1(path)??undefined}).findings);
    for(const[path,sha256]of identities)try{if(createHash("sha256").update(readFileSync(resolve(root,path))).digest("hex")!==sha256)findings.push({code:"runtime-input-mismatch",path,detail:"Source/resource changed before actual runtime graph completion"});}catch{findings.push({code:"runtime-input-mismatch",path,detail:"Source/resource disappeared before actual runtime graph completion"});}
    process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
    mkdirSync(dirname(report), { recursive: true }); writeFileSync(report, JSON.stringify({ generatedAt: new Date().toISOString(), declarations: config, compilerArtifacts: artifacts ?? null, compilerProvenance: receiptPaths, durableLedger, selectedActors:actorSelection?{dataRoot:actorSelection.dataRoot,requested:actorSelection.requested,profile:actorSelection.profile,publication:actorSelection.published.record,witnesses:actorSelection.witnesses}:null, retainedInvocations:observations.map(observation=>({manifest:observation.manifest,cwd:observation.cwd,args:observation.args,builtAtMs:observation.builtAtMs,observedAtMs:observation.observedAtMs,buildDirectory:observation.buildDirectory,invocationInputs:observation.invocationInputs})), graphs, sourceIdentities: Object.fromEntries(identities), findings }, null, 2) + "\n");
  }
  console.log(`[runtime graph] graphs=${graphs.length} sources/resources=${identities.size} findings=${findings.length} report=${report}`);
  if (findings.length) { for (const finding of findings) console.error(JSON.stringify(finding)); throw Error(`Runtime source/resource graph refused ${findings.length} findings`); }
}
