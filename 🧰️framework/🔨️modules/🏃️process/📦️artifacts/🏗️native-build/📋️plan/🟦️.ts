import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";
import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {type CargoTargetRecipeV1} from "../🎯️recipe/🟦️.ts";
import type {CargoCompilationGraphV1} from "../🕸️units/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📋️ Descriptive plan DATA carries no compiler, Source or publication authority. */
export type CargoArtifactPlanFactsV1=Readonly<{version:1;root:string;platform:"linux"|"darwin"|"win32";mode:"prototype"|"release";product:string;compiler:string;toolchain:string;configuration:string;source:string}>;
const chunk=262144;
const digest=(value:string):string=>createHash("sha256").update(value).digest("hex");

/** 🔤️ Uses the actual received root verbatim; path normalization belongs to its authenticated owner. */
export function cargoArtifactPlanCanonicalV1(input:unknown):string{
 if(validateJsonSchemaSubset(schema,input).length)throw Error("Closed Cargo artifact plan facts required");const value=input as CargoArtifactPlanFactsV1;
 return JSON.stringify([value.version,value.root,value.platform,value.mode,value.product,value.compiler,value.toolchain,value.configuration,value.source]);
}
/** 🏷️ A content key remains descriptive DATA until an owned acquisition supplies custody. */
export function cargoArtifactPlanIdentityV1(value:unknown):string{return digest(cargoArtifactPlanCanonicalV1(value));}

/** 🧬️ Streams the actual admitted graph under its original bounded control without cloning Source. */
export async function captureCargoArtifactPlanFactsV1(graph:CargoCompilationGraphV1,recipe:CargoTargetRecipeV1,mode:"prototype"|"release",platform:CargoArtifactPlanFactsV1["platform"],control:ResourceLeaseControl,maximumBytes:number):Promise<CargoArtifactPlanFactsV1>{
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1)throw Error("Finite artifact plan observation required");
 let bytes=0;const started=performance.now();
 const checkpoint=async()=>{checkResourceLeaseControl(control);await control.publish({resource:"cargo:artifact-plan:"+JSON.stringify({bytes,units:graph.units.length}),mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkResourceLeaseControl(control);};
 const hash=async(value:unknown):Promise<string>=>{
  const target=createHash("sha256");let pending="";
  const flush=():void=>{if(!pending)return;const extent=Buffer.byteLength(pending);if(extent>maximumBytes-bytes)throw Error("Artifact plan observation capacity exceeded");bytes+=extent;target.update(pending);pending="";};
  const sorted=(row:object):string[]=>Object.keys(row).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
  const put=(row:unknown):void=>{
   if(typeof row==="string"){pending+=JSON.stringify(row);return;}
   if(Array.isArray(row)){checkResourceLeaseControl(control);pending+="[";for(let i=0;i<row.length;i++){if(i)pending+=",";put(row[i]);}pending+="]";if(pending.length>=chunk)flush();return;}
   if(row&&typeof row==="object"){checkResourceLeaseControl(control);pending+="{";const keys=sorted(row);for(let i=0;i<keys.length;i++){if(i)pending+=",";pending+=JSON.stringify(keys[i])+":";put((row as Record<string,unknown>)[keys[i]!]);}pending+="}";if(pending.length>=chunk)flush();return;}
   if(row===null||typeof row==="number"&&Number.isFinite(row)||typeof row==="boolean"){pending+=JSON.stringify(row);return;}throw Error("Closed artifact plan primitive required");
  };
  const walk=async(row:unknown,depth:number):Promise<void>=>{
   if(depth>=2||!row||typeof row!=="object"){put(row);return;}
   checkResourceLeaseControl(control);
   const settle=async():Promise<void>=>{if(pending.length>=chunk){flush();await checkpoint();}};
   if(Array.isArray(row)){pending+="[";for(let i=0;i<row.length;i++){if(i)pending+=",";await walk(row[i],depth+1);await settle();}pending+="]";return;}
   pending+="{";const keys=sorted(row);for(let i=0;i<keys.length;i++){if(i)pending+=",";pending+=JSON.stringify(keys[i])+":";await walk((row as Record<string,unknown>)[keys[i]!],depth+1);await settle();}pending+="}";
  };
  await walk(value,0);flush();return target.digest("hex");
 };
 checkResourceLeaseControl(control);if(!graph.units.length)throw Error("Actually admitted Cargo Source graph required");
 for(const unit of graph.units){checkResourceLeaseControl(control);if(unit.provenance.kind==="workspace"&&(!unit.sourceInputs.length||unit.compilerCfg===null||unit.sourceInputs.some(input=>!/^[a-f0-9]{64}$/u.test(input.digest))))throw Error("Actually admitted Cargo Source graph required");}
 const configuration=await hash(recipe),source=await hash(graph),product=await hash([recipe.manifest,recipe.command,recipe.args]);await checkpoint();
 const facts=Object.freeze({version:1 as const,root:graph.workspaceRoot,platform,mode,product,compiler:recipe.compilerIdentity,toolchain:recipe.toolchainIdentity,configuration,source});cargoArtifactPlanCanonicalV1(facts);return facts;
}

/** 🚦️ Advises the next DATA transition; only real leases and rechecked publications can execute it. */
export function cargoArtifactAcquisitionStepV1(input:unknown):"produce"|"join"|"consume"|"cancel"|"recheck"|"refuse"{
 if(validateJsonSchemaSubset(schema.definitions.AcquisitionObservation,input,schema).length)throw Error("Closed artifact acquisition observation required");const row=input as Readonly<{consumerCancelled:boolean;publication:string;publicationRechecked:boolean;producerPlan:string|null;producerSettled:boolean;custodyReleased:boolean}>;
 if(row.consumerCancelled)return "cancel";if(row.publication==="foreign")return "refuse";if(row.publication==="matching")return row.publicationRechecked?"consume":"recheck";if(row.producerPlan==="same"&&(!row.producerSettled||!row.custodyReleased))return "join";return "produce";
}

/** 🧾️ Distinguishes changed completed Source bytes from an absent observation without printing Source. */
export type CargoSourceDigestMismatchV1=Readonly<{path:string;expected:string;observed:string|null;missingRecord:boolean}>;
/** 🔬️ Reports only a bounded path and SHA-256 DATA while retaining exact lookup identity. */
export function cargoSourceDigestMismatchV1(path:string,expected:string,observed:string|null|undefined):CargoSourceDigestMismatchV1|null{
 if(expected===observed)return null;
 const row={path,expected,observed:observed??null,missingRecord:observed===undefined};
 if(validateJsonSchemaSubset(schema.definitions.SourceDigestMismatch,row).length)throw Error("Closed bounded Cargo Source digest comparison required");
 return Object.freeze(row);
}
