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
const factFields=schema.required as readonly (keyof CargoArtifactPlanFactsV1)[];
const differencePolicy=schema.definitions.DifferencePolicy.const;
const label=(value:string|undefined):string|null=>value===undefined?null:value.length>differencePolicy.maximumLabelUnits?value.slice(0,differencePolicy.maximumLabelUnits-1)+"…":value;

/** 🔤️ Uses the actual received root verbatim; path normalization belongs to its authenticated owner. */
export function cargoArtifactPlanCanonicalV1(input:unknown):string{
 if(validateJsonSchemaSubset(schema,input).length)throw Error("Closed Cargo artifact plan facts required");const value=input as CargoArtifactPlanFactsV1;
 return JSON.stringify([value.version,value.root,value.platform,value.mode,value.product,value.compiler,value.toolchain,value.configuration,value.source]);
}
/** 🏷️ A content key remains descriptive DATA until an owned acquisition supplies custody. */
export function cargoArtifactPlanIdentityV1(value:unknown):string{return digest(cargoArtifactPlanCanonicalV1(value));}

const planningFields=["planningDigest","compilationDigest","configurationDigest"] as const;
type PlannedUnit=Readonly<Pick<CargoCompilationGraphV1["units"][number],"id"|typeof planningFields[number]>>;
type DiagnosticGraph=Readonly<{roots:readonly string[];units:readonly (PlannedUnit&Readonly<Pick<CargoCompilationGraphV1["units"][number],"sourceInputs">>)[]}>;
type PlanningGraph=Readonly<{workspaceRoot:string;roots:readonly string[];units:readonly PlannedUnit[]}>;

/** 🧭️ Finds Cargo planning drift before Source admission can fail against the replacement graph. */
export function cargoArtifactPlanningDifferenceV1(baseline:PlanningGraph,current:PlanningGraph,check:()=>void){
 check();
 if(baseline.workspaceRoot!==current.workspaceRoot)return {category:"workspace-root",expectedRoot:label(baseline.workspaceRoot),observedRoot:label(current.workspaceRoot)};
 for(let index=0;index<Math.max(baseline.roots.length,current.roots.length);index++){
  check();
  if(baseline.roots[index]!==current.roots[index])return {category:"graph-roots",index,expectedUnit:label(baseline.roots[index]),observedUnit:label(current.roots[index])};
 }
 for(let index=0;index<Math.max(baseline.units.length,current.units.length);index++){
  check();
  const left=baseline.units[index],right=current.units[index];
  if(!left||!right||left.id!==right.id)return {category:"graph-units",index,expectedUnit:label(left?.id),observedUnit:label(right?.id)};
  const changed=planningFields.find(key=>left[key]!==right[key]);
  if(changed)return {category:"unit-metadata",unit:label(left.id),changed,expectedUnitDigest:left[changed],observedUnitDigest:right[changed]};
 }
 return null;
}

/** 🔎️ Identifies one difference using only previously admitted plan facts and graph rows. */
export function cargoArtifactPlanDifferenceV1(expected:CargoArtifactPlanFactsV1,observed:CargoArtifactPlanFactsV1,baseline:DiagnosticGraph,current:DiagnosticGraph,check:()=>void){
 const field=factFields.find(key=>expected[key]!==observed[key]);
 if(field===undefined)return null;
 const fact={field,expectedDigest:field==="source"||field==="configuration"?expected[field]:digest(String(expected[field])),observedDigest:field==="source"||field==="configuration"?observed[field]:digest(String(observed[field]))};
 if(field!=="source")return {...fact,category:"plan-fact"};
 let comparisons=0;
 const allowed=()=>{check();return ++comparisons<=differencePolicy.maximumComparisons;};
 for(let index=0;index<Math.max(baseline.roots.length,current.roots.length);index++){
  if(!allowed())return {...fact,category:"graph-comparison-limit"};
  if(baseline.roots[index]!==current.roots[index])return {...fact,category:"graph-roots",index,expectedUnit:label(baseline.roots[index]),observedUnit:label(current.roots[index])};
 }
 for(let index=0;index<Math.max(baseline.units.length,current.units.length);index++){
  if(!allowed())return {...fact,category:"graph-comparison-limit"};
  const left=baseline.units[index],right=current.units[index];
  if(!left||!right||left.id!==right.id)return {...fact,category:"graph-units",index,expectedUnit:label(left?.id),observedUnit:label(right?.id)};
  for(let input=0;input<Math.max(left.sourceInputs.length,right.sourceInputs.length);input++){
   if(!allowed())return {...fact,category:"graph-comparison-limit"};
   const before=left.sourceInputs[input],after=right.sourceInputs[input];
   if(!before||!after||before.path!==after.path)return {...fact,category:"source-roster",unit:label(left.id),expectedPath:label(before?.path),observedPath:label(after?.path)};
   if(before.digest!==after.digest||before.bytes!==after.bytes)return {...fact,category:"source-file",unit:label(left.id),path:label(before.path),expectedSha256:before.digest,observedSha256:after.digest,expectedBytes:before.bytes,observedBytes:after.bytes};
  }
  const changed=planningFields.find(key=>left[key]!==right[key]);
  if(changed)return {...fact,category:"unit-metadata",unit:label(left.id),changed,expectedUnitDigest:left[changed],observedUnitDigest:right[changed]};
 }
 return {...fact,category:"graph-other"};
}

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
