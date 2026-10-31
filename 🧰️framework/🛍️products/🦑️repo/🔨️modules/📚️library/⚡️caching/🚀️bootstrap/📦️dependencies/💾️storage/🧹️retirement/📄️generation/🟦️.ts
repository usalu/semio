import fundingSchema from "./🧬️schema/💰️funding.json" with {type:"json"};
import {types} from "node:util";
import referencesSchema from "./🧬️schema/🔗️references.json" with {type:"json"};
import {posix,win32} from "node:path";
import {readJavascriptDependencySourceOwnerReadyV1,type JavascriptDependencySourceOwnerReadyV1} from "../../🟦️.ts";
import {javascriptInstallSchemaV1 as installSchema} from "../../🧱️schemas/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🗃️ Retains bounded historical wire metadata as DATA; current readiness, references and actual custody remain separate proofs. */
export interface JavascriptDependencyRetainedGenerationDataV1{
 readonly version:1;readonly purpose:"retained-dependency-generation-data";readonly authority:false;readonly currentReady:false;
 readonly generation:string;readonly prefix:string;readonly scope:string;readonly recipeSha256:string;readonly receiptSha256:string;readonly members:number;
 readonly sourceBindings:JavascriptDependencySourceOwnerReadyV1["bindings"];readonly ownerReferences:JavascriptDependencySourceOwnerReadyV1["ownerReferences"];
}

/** 📜️ Anchors a retained closed receipt without admitting its recipe or removed Source owner for installation, execution or retirement. */
export function readJavascriptDependencyRetainedGenerationDataV1(workspace:string,root:string,value:unknown):JavascriptDependencyRetainedGenerationDataV1{
 if(arguments.length!==3||typeof workspace!=="string"||typeof root!=="string"||workspace.length>256||root.length>256||workspace.includes("\0")||root.includes("\0"))throw Error("Bounded retained generation anchors required");
 const path=win32.isAbsolute(workspace)&&!posix.isAbsolute(workspace)?win32:posix;
 if(!path.isAbsolute(workspace)||!path.isAbsolute(root)||path.normalize(workspace)!==workspace||path.normalize(root)!==root)throw Error("Canonical retained generation anchors required");
 const receipt=readJavascriptDependencySourceOwnerReadyV1(value),row=receipt.value,prefix=row.generation.slice(0,installSchema.$defs.ProducerStageProtocol.const.generationPrefixUnits),binding=row.bindings[0]!;
 if(binding.sourceDirectory!==path.join(workspace,...(row.scope==="."?[]:row.scope.split("/")),"node_modules")||binding.targetDirectory!==path.join(root,installSchema.$defs.ProducerStageProtocol.const.rootRole,prefix,"node_modules"))throw Error("Exact retained generation workspace and store required");
 const data=Object.freeze({version:1 as const,purpose:"retained-dependency-generation-data"as const,authority:false as const,currentReady:false as const,generation:row.generation,prefix,scope:row.scope,recipeSha256:row.recipeSha256,receiptSha256:receipt.sha256,members:row.members.length,sourceBindings:row.bindings,ownerReferences:row.ownerReferences});
 if(validateJsonSchemaSubset(schema as never,data).length)throw Error("Closed retained generation DATA required");return data;
}

/** 🔗️ Observes metadata reachability only; actual pins, physical Sources and native custody must still be held by the private maintenance owner. */
export function inspectJavascriptDependencyRetirementReferencesV1(value:unknown):Readonly<{authority:false;selected:string|null;generations:number}>{
 const input=captureRetainedReferenceDataV1(value)as{generations:JavascriptDependencyRetainedGenerationDataV1[];current:string[];aliases:{kind:string;resolvedTarget:string|null}[];selected:string|null;absent?:string[]};
 if(validateJsonSchemaSubset(referencesSchema as never,input).length)throw Error("Complete closed retained reference DATA required");
 const all=new Map(input.generations.map(row=>[row.generation,row])),prefixes=new Set(input.generations.map(row=>row.prefix)),current=new Set(input.current),absent=new Set(input.absent??[]);
 if(absent.size!==(input.absent??[]).length||[...absent].some(generation=>all.has(generation)||prefixes.has(generation.slice(0,20))||current.has(generation))||all.size!==input.generations.length||prefixes.size!==all.size||current.size!==input.current.length||input.selected!==null&&(!all.has(input.selected)||current.has(input.selected)))throw Error("Unique inactive retained generation required");
 for(const row of input.generations){if(row.prefix!==row.generation.slice(0,20)||row.ownerReferences.some(reference=>!all.has(reference.generation)&&(current.has(row.generation)||!absent.has(reference.generation))))throw Error("Complete retained artifact reference graph required");}
 if(input.selected!==null&&input.generations.some(row=>row.generation!==input.selected&&row.ownerReferences.some(reference=>reference.generation===input.selected)))throw Error("Retained dependency artifact still references selected generation");
 const targets=new Set<string>();for(const generation of current){const row=all.get(generation);if(!row||row.ownerReferences.some(reference=>!current.has(reference.generation)))throw Error("Complete qualified current generation closure required");for(const binding of row.sourceBindings)targets.add(binding.targetDirectory);}
 if(input.aliases.some(row=>row.kind!=="absent"&&(row.kind!=="link"||row.resolvedTarget===null||!targets.has(row.resolvedTarget))))throw Error("Historical Source alias remains old or unknown");
 return Object.freeze({authority:false as const,selected:input.selected,generations:all.size});
}

/** 🧊️ Captures finite plain DATA without invoking getters, Proxy traps or serialization hooks. */
function captureRetainedReferenceDataV1(value:unknown):unknown{
 let nodes=0,bytes=0;const active=new Set<object>();
 const capture=(row:unknown,depth:number):unknown=>{
  if(++nodes>1048576||depth>16)throw Error("Retained reference DATA capacity exceeded");
  if(row===null||typeof row==="boolean"||typeof row==="number"&&Number.isFinite(row))return row;
  if(typeof row==="string"){bytes+=Buffer.byteLength(row);if(bytes>67108864)throw Error("Retained reference byte capacity exceeded");return row;}
  if(typeof row!=="object"||types.isProxy(row)||active.has(row))throw Error("Plain retained reference DATA required");
  const array=Array.isArray(row);if(!array&&Object.getPrototypeOf(row)!==Object.prototype&&Object.getPrototypeOf(row)!==null)throw Error("Ordinary retained reference DATA required");
  const descriptors=Object.getOwnPropertyDescriptors(row),keys=Reflect.ownKeys(descriptors);if(keys.some(key=>typeof key!=="string")||keys.some(key=>!("value"in descriptors[key as string]!)))throw Error("Own retained reference data fields required");
  active.add(row);try{if(array){const length=descriptors.length?.value;if(!Number.isSafeInteger(length)||length<0||length>1048576||keys.length!==length+1||keys.some(key=>key!=="length"&&!/^(0|[1-9][0-9]*)$/u.test(key as string)))throw Error("Closed retained reference array required");return Array.from({length},(_,index)=>{const field=descriptors[String(index)];if(!field)throw Error("Dense retained reference array required");return capture(field.value,depth+1);});}return Object.fromEntries(keys.map(key=>[key,capture(descriptors[key as string]!.value,depth+1)]));}finally{active.delete(row);}
 };return capture(value,0);
}

/** 💰️ Describes a bounded whole-allocation policy without carrying native parent or allocation authority. */
export interface JavascriptDependencyRetirementPolicyV1{readonly version:1;readonly root:string;readonly leaseDirectory:string;readonly capacityBytes:number;readonly maximumWorkspaceBytes:number;readonly maximumArtifactBytes:number}

/** 🏦️ Accepts the actual retained charge under the current ceiling; only genuine native custody can authenticate its backing and journal. */
export function readJavascriptDependencyRetirementFundingPolicyV1(value:unknown):Readonly<{authority:false;policy:JavascriptDependencyRetirementPolicyV1}>{
 const row=captureRetainedReferenceDataV1(value)as{ceiling:JavascriptDependencyRetirementPolicyV1;retained:JavascriptDependencyRetirementPolicyV1;allocationBytes:number;backingMaximumPhysicalBytes:number};
 if(validateJsonSchemaSubset(fundingSchema as never,row).length)throw Error("Closed actual dependency retirement funding DATA required");
 const actual=row.retained,ceiling=row.ceiling;
 if(actual.root!==ceiling.root||actual.leaseDirectory!==ceiling.leaseDirectory||actual.capacityBytes!==row.backingMaximumPhysicalBytes||actual.maximumWorkspaceBytes!==row.allocationBytes||actual.maximumArtifactBytes!==row.allocationBytes||actual.capacityBytes<row.allocationBytes||actual.capacityBytes>ceiling.capacityBytes||actual.maximumWorkspaceBytes>ceiling.maximumWorkspaceBytes||actual.maximumArtifactBytes>ceiling.maximumArtifactBytes)throw Error("Exact retained dependency funding within current ceiling required");
 return Object.freeze({authority:false as const,policy:Object.freeze({...actual})});
}
