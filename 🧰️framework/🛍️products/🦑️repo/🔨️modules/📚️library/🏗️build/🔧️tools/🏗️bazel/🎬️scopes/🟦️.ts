import{refuseBuildV1,sayBuildV1}from"../../../📖️texts/🟦️.ts";
/** 🎬️ The narrow implementation scopes of the Bazel adapter: per scope, the lazy roots and byte-only leaves that the runtime traces of its action classes proved necessary beyond the static imports of the executor; a class in no scope keeps the full closure. See `./🔣️.json` (derived data) and `./🧪️tests/🟦️.ts` (the law). */
import {readFileSync} from "node:fs";
import table from "./🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {BuildSourceScopeV1} from "../../../📥️source/🔏️closure/🟦️.ts";
import {buildSchedulingPrefixesV1,buildSchedulingV1} from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";

/** 🏷️ What an executor process knows about the action it runs; the trace records the same five facts; `owner` is `native` when the task runs a discovered native owner command, which makes the executor load the Cargo execution backend. */
export interface BazelActionClassV1{readonly command:"action"|"launch"|"test"|"launcher";readonly workerKind:string;readonly executionKind:string;readonly semioKind:string;readonly owner:"none"|"native"}

/** 🧾️ What the runtime trace of one action class opened: processes traced and the modules, reads and physical captures seen. */
export interface BazelTracedV1{readonly processes:number;readonly modules:readonly string[];readonly reads:readonly string[];readonly physical:readonly string[]}

/** 🔭️ One proven scope: the capture scope, the scheduling domains whose files stay out of its action key and the action classes whose traces it holds, each with its evidence. */
export interface BazelScopeV1{readonly id:string;readonly scope:Extract<BuildSourceScopeV1,{kind:"static"}>;readonly scheduling:readonly string[];readonly classes:readonly{readonly select:BazelActionClassV1;readonly traced:BazelTracedV1}[]}

const classKey=(value:BazelActionClassV1)=>[value.command,value.workerKind,value.executionKind,value.semioKind,value.owner].join("|");

/** 🧭️ Names the class of a planned action: its action kind, the worker kind of its target, its execution kind, the semio execution kind its metadata declares and whether it is a native owner command. */
export function bazelActionClassV1(task:{definition:{execution:{kind?:unknown;nativeOwnerCommand?:unknown};metadata?:unknown}},kind:"task"|"launch"|"test"|"launcher",workerKind:string):BazelActionClassV1{
 const semio=(task.definition.metadata as {semio?:{execution?:{kind?:string}}}|undefined)?.semio;
 return {command:kind==="task"?"action":kind,workerKind,executionKind:String(task.definition.execution.kind),semioKind:semio?.execution?.kind??"none",owner:task.definition.execution.nativeOwnerCommand===undefined?"none":"native"};
}

/** 📚️ Admits the declared scope table and refuses duplicates, so a class selects at most one scope. */
export function bazelScopesV1(candidate:unknown=table):readonly BazelScopeV1[]{
 if(validateJsonSchemaSubset(schema,candidate).length)throw refuseBuildV1("invalid","bazel.scope-table-required");
 const rows=(candidate as {scopes:{id:string;roots:string[];leaves:string[];scheduling?:string[];classes:BazelScopeV1["classes"][number][]}[]}).scopes,ids=new Set<string>(),seen=new Set<string>();
 return rows.map(row=>{if(ids.has(row.id))throw refuseBuildV1("invalid","bazel.scope-unique-required",{id:String(row.id)});ids.add(row.id);for(const item of row.classes){const key=classKey(item.select);if(seen.has(key))throw refuseBuildV1("invalid","bazel.scope-unique-required",{id:String(row.id)});seen.add(key);}buildSchedulingPrefixesV1(buildSchedulingV1(),row.scheduling??[]);return Object.freeze({id:row.id,scope:Object.freeze({kind:"static" as const,roots:row.roots,leaves:row.leaves}),scheduling:Object.freeze(row.scheduling??[]),classes:Object.freeze(row.classes)});});
}

let declared:readonly BazelScopeV1[]|undefined;

/** 🏓️ The declared table, admitted once; a table that fails its schema must never stop a build, so every class keeps the full closure and the refusal is printed (the law of the table fails instead). */
function declaredScopesV1():readonly BazelScopeV1[]{
 if(declared===undefined)try{declared=bazelScopesV1();}catch(error){declared=[];sayBuildV1("bazel.scope-table-refused",{reason:String((error as Error).message)},"warn");}
 return declared;
}

const candidates=new Map<string,readonly BazelScopeV1[]>();

/** 🧪️ The declared table, or, for a trial run that must not touch the fleet, the candidate table a caller names in `SEMIO_BUILD_IMPLEMENTATION_SCOPES`: every process of one run sees the same table because the planner child inherits the environment. */
export function bazelScopesFromEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):readonly BazelScopeV1[]{
 const path=environment.SEMIO_BUILD_IMPLEMENTATION_SCOPES;if(path===undefined||path==="")return declaredScopesV1();
 let rows=candidates.get(path);if(!rows){rows=bazelScopesV1(JSON.parse(readFileSync(path,"utf8")));candidates.set(path,rows);}return rows;
}

/** 🎯️ Finds the proven scope of an action class, or nothing when the class keeps the full closure. */
export function bazelScopeOfV1(value:BazelActionClassV1,rows:readonly BazelScopeV1[]=declaredScopesV1()):BazelScopeV1|undefined{
 const key=classKey(value);return rows.find(row=>row.classes.some(item=>classKey(item.select)===key));
}

/** 🔎️ Finds a declared scope by its id. */
export function bazelScopeByIdV1(id:string,rows:readonly BazelScopeV1[]=declaredScopesV1()):BazelScopeV1{
 const row=rows.find(candidate=>candidate.id===id);if(!row)throw refuseBuildV1("missing","bazel.scope-declared-required",{id:String(id)});return row;
}
