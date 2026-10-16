import {isAbsolute,relative,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/🟦️.ts";
import {cargoDepInfoResolvedSourcesV1,type CargoProvenanceV1} from "../../🟦️.ts";
import sourceSchema from "../../../../📤️publication/📥️sources/🧬️schema/🔣️.json";
import schemaDocument from "../🧬️schema/🔣️.json";
const schema=JSON.parse(JSON.stringify(schemaDocument)) as typeof schemaDocument;

export interface CompletedCargoOriginV1{readonly version:1;readonly workspace:string;readonly mode:"prototype"|"release";readonly manifest:string;readonly cwd:string;readonly buildDirectory:string;readonly targetDirectory:string;readonly cargoHome:string}
export interface CompletedCargoClosureRowV1{readonly path:string;readonly kind:"file"|"directory"|"absent";readonly custody:"current"|"retained";readonly sha256:string|null}
const maximumRows=sourceSchema.properties.maximumEntries.const,maximumIndexBytes=sourceSchema.properties.maximumManifestBytes.const;
const inside=(root:string,path:string):boolean=>{const tail=relative(root,path);return tail===""||(!isAbsolute(tail)&&tail.split(/[\\/]/u)[0]!=="..");};
function canonical(value:unknown):string{if(typeof value!=="string"||!isAbsolute(value)||resolve(value)!==value||value.includes("\0")||value.length>schema.definitions.Path.maxLength)throw Error("Canonical finite compilation path required");return value;}

/** 🧭️ Validates origin DATA; this operation cannot issue any compiler, storage or retained reader authority. */
export function readCompletedCargoOriginV1(value:unknown):CompletedCargoOriginV1{
 if(validateJsonSchemaSubset(schema.definitions.Origin,value,schema).length)throw Error("Closed actual compiler origin DATA required");const origin=value as CompletedCargoOriginV1;for(const [name,path]of Object.entries(origin))if(!["version","mode"].includes(name))canonical(path);if(!inside(origin.workspace,origin.manifest)||!inside(origin.workspace,origin.cwd)||origin.buildDirectory===origin.targetDirectory)throw Error("Distinct exact compilation origin roots required");return Object.freeze({...origin});
}

/** 📋️ Derives a complete finite read roster from actual observation DATA without recursively scanning a compiler tree. */
export function completedCargoClosureRosterV1(observation:CargoProvenanceV1,value:unknown):readonly CompletedCargoClosureRowV1[]{
 const origin=readCompletedCargoOriginV1(value);if(observation.status!==0||observation.cancelled||observation.manifest!==origin.manifest||observation.cwd!==origin.cwd||observation.buildDirectory!==origin.buildDirectory||!observation.units.length)throw Error("Completed exact compiler origin observation required");const rows=new Map<string,CompletedCargoClosureRowV1>();let indexBytes=Buffer.byteLength(JSON.stringify({version:1,origin,rows:[]}));
 const add=(path:unknown,kind:CompletedCargoClosureRowV1["kind"],sha:unknown)=>{const file=canonical(path);if(kind==="absent"?sha!==null:typeof sha!=="string"||!/^[a-f0-9]{64}$/u.test(sha))throw Error("Exact compiler input/artifact digest required");const sha256=sha as string|null,previous=rows.get(file);if(previous){if(previous.kind!==kind||previous.sha256!==sha256)throw Error("Contradictory completed compiler read roster");return;}if(rows.size>=maximumRows)throw Error("Completed compiler roster exceeds source inventory bound");const row:CompletedCargoClosureRowV1=Object.freeze({path:file,kind,sha256,custody:inside(origin.buildDirectory,file)||inside(origin.targetDirectory,file)?"retained":"current"}),bytes=Buffer.byteLength(JSON.stringify(row))+1;if(bytes>maximumIndexBytes-indexBytes)throw Error("Completed compiler index exceeds publication manifest extent");indexBytes+=bytes;rows.set(file,row);};
 for(const row of observation.invocationInputs)add(row.path,row.sha256===null?"absent":"file",row.sha256);
 for(const unit of observation.units){for(const row of unit.inputs)add(row.path,row.kind??"absent",row.sha256);for(const row of unit.artifacts)add(row.path,"file",row.sha256);for(const dep of unit.depInfo){add(dep.path,"file",dep.sha256);for(const file of cargoDepInfoResolvedSourcesV1(dep))if(!rows.has(file)&&!unit.inputs.some(input=>input.path===file))throw Error("Retained dep-info source has no actual input digest");}}
 const tracked=(value:unknown)=>{const path=canonical(value);if(!rows.has(path)||rows.get(path)?.kind!=="file")throw Error("Resource source lacks its actual tracked compiler input");};
 for(const resource of[...observation.buildResources,...observation.compilerResources]){add(resource.path,"file",resource.sha256);for(const row of resource.resources){const input=row.input;if(input.kind==="failed")throw Error("Failed compiler resource cannot become completed evidence");add(input.path,input.kind==="directory"?"directory":"file",row.sha256);if(input.kind==="copy")add(input.output,"file",row.outputSha256);const operation=input.operation;if(operation&&typeof operation==="object"&&!Array.isArray(operation))tracked(Reflect.get(operation,"source"));const callsite=input.callsite;if(callsite&&typeof callsite==="object"&&!Array.isArray(callsite))tracked(Reflect.get(callsite,"source"));}}
 return Object.freeze([...rows.values()].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0));
}

/** 🔗️ Identifies only actual producer outputs; ordinary inputs never become relocatable by path or role text. */
export function completedCargoOutputClaimsV1(observation:CargoProvenanceV1,value:unknown):readonly CompletedCargoClosureRowV1[]{
 const origin=readCompletedCargoOriginV1(value),roster=completedCargoClosureRosterV1(observation,origin),eligible=new Set<string>();for(const unit of observation.units)for(const artifact of unit.artifacts)eligible.add(artifact.path);for(const resource of[...observation.buildResources,...observation.compilerResources]){if("producerUnit"in resource&&resource.producerUnit&&resource.callerUnit)eligible.add(resource.path);for(const row of resource.resources)if(row.input.kind==="copy"&&typeof row.input.output==="string")eligible.add(row.input.output);}const rows=roster.filter(row=>eligible.has(row.path));if(rows.some(row=>row.kind!=="file"||row.custody!=="retained"))throw Error("Actual producer outputs must belong to its funded build/target roots");return Object.freeze(rows);
}
