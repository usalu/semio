import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";
import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {type CargoTargetRecipeV1} from "../🎯️recipe/🟦️.ts";
import type {CargoCompilationGraphV1} from "../🕸️units/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📋️ Descriptive plan DATA carries no compiler, Source or publication authority. */
export type CargoArtifactPlanFactsV1=Readonly<{version:1;root:string;platform:"linux"|"darwin"|"win32";mode:"prototype"|"release";product:string;compiler:string;toolchain:string;configuration:string;source:string}>;
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
 let bytes=0,turn=0;const started=performance.now();
 const checkpoint=async()=>{checkResourceLeaseControl(control);await control.publish({resource:"cargo:artifact-plan:"+JSON.stringify({bytes,units:graph.units.length}),mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkResourceLeaseControl(control);};
 const hash=async(value:unknown):Promise<string>=>{
  const target=createHash("sha256");
  const write=async(text:string)=>{const extent=Buffer.byteLength(text);if(extent>maximumBytes-bytes)throw Error("Artifact plan observation capacity exceeded");bytes+=extent;target.update(text);if(++turn%64===0)await checkpoint();else checkResourceLeaseControl(control);};
  const walk=async(row:unknown):Promise<void>=>{
   checkResourceLeaseControl(control);
   if(typeof row==="string"){await write('"');for(let start=0;start<row.length;){let end=Math.min(row.length,start+4096);if(end<row.length&&row.charCodeAt(end-1)>=0xd800&&row.charCodeAt(end-1)<=0xdbff)end--;await write(JSON.stringify(row.slice(start,end)).slice(1,-1));start=end;}await write('"');return;}
   if(Array.isArray(row)){await write("[");for(let i=0;i<row.length;i++){if(i)await write(",");await walk(row[i]);}await write("]");return;}
   if(row&&typeof row==="object"){await write("{");const keys=Object.keys(row).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));for(let i=0;i<keys.length;i++){if(i)await write(",");await walk(keys[i]);await write(":");await walk((row as Record<string,unknown>)[keys[i]!]);}await write("}");return;}
   if(row===null||typeof row==="number"&&Number.isFinite(row)||typeof row==="boolean"){await write(JSON.stringify(row));return;}throw Error("Closed artifact plan primitive required");
  };
  await walk(value);return target.digest("hex");
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
