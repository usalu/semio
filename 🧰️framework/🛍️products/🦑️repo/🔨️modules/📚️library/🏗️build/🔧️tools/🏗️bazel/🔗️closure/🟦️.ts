/** 🔗️ Resolves the content-addressed closure markers of a hash-representation build graph into the exact workspace files they name, so Bazel declares every imported module and every compacted project Source as an input instead of fingerprinting an environment variable nobody sets. */
import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readExactSourceInventoryV1,withManagedGraphSourceReadBatchV1} from "../../../📥️inputs/🧮️digest/📥️read/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy=schema.definitions.Policy.const,marker=new RegExp(policy.markerPattern,"u");

/** 🔗️ The two closure families a graph names: exact Source inventories and selected Cargo lock projections. */
export type BazelClosureKindV1="source"|"lock";

/** 🔖️ One closure the graph references by the digest of its manifest. */
export interface BazelClosureMarkerV1{readonly kind:BazelClosureKindV1;readonly key:string}

/** 📥️ Fetches verified, parsed manifests by key; the managed variant reads the graph store, the filesystem variant any directory of content-addressed files. */
export type BazelClosureFetchV1=(kind:BazelClosureKindV1,keys:readonly string[])=>Promise<ReadonlyMap<string,unknown>>;

/** 🗂️ The files each loaded marker expands to. */
export interface BazelClosuresV1{files(marker:BazelClosureMarkerV1):readonly string[]}

/** 🔎️ Reads one task input entry as a closure marker; any other entry is not one. */
export function bazelClosureMarkerV1(input:unknown):BazelClosureMarkerV1|undefined{
 if(typeof input!=="object"||input===null||!("env" in input)||typeof input.env!=="string")return undefined;
 const match=marker.exec(input.env);
 return match?{kind:match[1]==="SOURCE_CLOSURE"?"source":"lock",key:match[2]!}:undefined;
}

/** 🧮️ Lists every distinct marker the given named input groups reference, in first-seen order. */
export function bazelClosureMarkersV1(groups:Iterable<readonly unknown[]>):BazelClosureMarkerV1[]{
 const found=new Map<string,BazelClosureMarkerV1>();
 for(const group of groups)for(const input of group){const row=bazelClosureMarkerV1(input);if(row&&!found.has(row.kind+":"+row.key))found.set(row.kind+":"+row.key,row);}
 return [...found.values()];
}

/** 🕸️ The closure markers a plan can hash: those named by its tasks' own inputs and by the named inputs of every project it can reach, through `^` dependency edges and through `{input, projects}` references, which is exactly the reach of Nx's expansion. Unreached projects contribute nothing, so a selection loads only its own manifests. */
export function bazelClosureMarkersForPlanV1(graph:{readonly namedInputs?:Readonly<Record<string,readonly unknown[]>>;readonly dependencies:readonly {readonly source:string;readonly target:string}[];readonly projects:Readonly<Record<string,{readonly namedInputs?:Readonly<Record<string,readonly unknown[]>>}|undefined>>},tasks:Iterable<{readonly project:string;readonly definition:{readonly inputs?:readonly unknown[]}}>):BazelClosureMarkerV1[]{
 const adjacency=new Map<string,string[]>(),seen=new Set<string>(),pending:string[]=[],groups:(readonly unknown[])[]=[...Object.values(graph.namedInputs??{})];
 for(const edge of graph.dependencies){const row=adjacency.get(edge.source)??[];row.push(edge.target);adjacency.set(edge.source,row);}
 const visit=(project:string)=>{if(!seen.has(project)&&graph.projects[project]){seen.add(project);pending.push(project);}};
 const refer=(inputs:readonly unknown[])=>{for(const input of inputs)if(typeof input==="object"&&input!==null&&"input" in input){const owners=(input as {projects?:string|readonly string[]}).projects;for(const owner of typeof owners==="string"?[owners]:owners??[])visit(owner);}};
 for(const task of tasks){visit(task.project);const inputs=task.definition.inputs??[];groups.push(inputs);refer(inputs);}
 for(let position=0;position<pending.length;position++){
  const project=pending[position]!;
  for(const next of adjacency.get(project)??[])visit(next);
  for(const group of Object.values(graph.projects[project]!.namedInputs??{})){groups.push(group);refer(group);}
 }
 return bazelClosureMarkersV1(groups);
}

/** 🧷️ Admits one workspace-relative name: no absolute, parent or empty segment and a bounded length. */
function requireWorkspaceName(name:string):string{
 if(name.length>policy.maximumPathUnits||name.startsWith("/")||name.includes("\\")||name.includes("\0")||name.split("/").some(part=>part===""||part==="."||part===".."))throw new Error("Workspace-relative closure file required: "+name);
 return name;
}

/** 📜️ Validates one manifest against its closed schema and returns the workspace files it stands for; a lock projection stands for its crate manifest only: the locked resolution reaches the action key as the crate's resolution identity, not as the whole Cargo.lock file. */
export function parseBazelClosureManifestV1(kind:BazelClosureKindV1,value:unknown):readonly string[]{
 const definition=kind==="source"?schema.definitions.SourceManifest:schema.definitions.LockManifest;
 if(validateJsonSchemaSubset(definition as never,value as never).length)throw new Error("Closed Bazel closure manifest required");
 const names=kind==="source"?(value as {files:string[]}).files:[(value as {manifest:string}).manifest];
 if(names.length>policy.maximumFiles)throw new Error("Bazel closure exceeds its finite file bound");
 return Object.freeze([...new Set(names.map(requireWorkspaceName))].sort());
}

/** 📦️ Loads exactly the markers a plan references, one fetch per family, and refuses a missing manifest. */
export async function loadBazelClosuresV1(markers:readonly BazelClosureMarkerV1[],fetch:BazelClosureFetchV1):Promise<BazelClosuresV1>{
 const unique=new Map(markers.map(row=>[row.kind+":"+row.key,row])),files=new Map<string,readonly string[]>();
 if(unique.size>policy.maximumManifests)throw new Error("Bazel closure manifest count exceeds its finite bound");
 for(const kind of ["source","lock"] as const){
  const keys=[...unique.values()].filter(row=>row.kind===kind).map(row=>row.key).sort();
  if(!keys.length)continue;
  const rows=await fetch(kind,keys);
  for(const key of keys){const value=rows.get(key);if(value===undefined)throw new Error(`Bazel closure manifest missing: ${kind} ${key}`);files.set(kind+":"+key,parseBazelClosureManifestV1(kind,value));}
 }
 return{files(row){const found=files.get(row.kind+":"+row.key);if(!found)throw new Error(`Bazel closure manifest not loaded: ${row.kind} ${row.key}`);return found;}};
}

/** 🗃️ Reads content-addressed manifests from a directory laid out as the graph store is: each file is named by the sha256 of its bytes, which is verified. */
export function filesystemBazelClosureFetchV1(directory:string):BazelClosureFetchV1{
 return async(kind,keys)=>{
  const rows=new Map<string,unknown>();
  for(const key of keys){const bytes=await readFile(join(directory,policy.namespaces[kind],key+".json"));if(createHash("sha256").update(bytes).digest("hex")!==key)throw new Error("Exact Source inventory identity changed");rows.set(key,JSON.parse(bytes.toString("utf8")));}
  return rows;
 };
}

/** 🏪️ Reads the manifests through the graph store consumer the Nx hasher uses, under the caller's carried control. */
export function managedBazelClosureFetchV1(root:string,control:ResourceLeaseControl):BazelClosureFetchV1{
 return (kind,keys)=>withManagedGraphSourceReadBatchV1(root,control,async batch=>{
  const rows=new Map<string,unknown>();
  for(const key of keys){const file=await readExactSourceInventoryV1(batch,policy.namespaces[kind] as Parameters<typeof readExactSourceInventoryV1>[1],key);if(!file||file.sha256!==key)throw new Error("Exact Source inventory identity changed");rows.set(key,file.value);}
  return rows;
 });
}
