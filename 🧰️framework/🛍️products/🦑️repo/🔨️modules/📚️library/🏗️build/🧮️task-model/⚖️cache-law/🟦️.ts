import type {BuildGraphV1,BuildTargetV1} from "../../🕸️graph/📋️model/🟦️.ts";
import {remoteGuardV1} from "../🛡️remote-guard/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ⚖️ The authored conformance law.
 * @see ./🧬️schema/🔣️.json */
export const CACHE_LAW_POLICY_V1=schema.definitions.Policy.const;
const policy=CACHE_LAW_POLICY_V1,absolute=policy.absolutePath.map(pattern=>new RegExp(pattern,"u")),identity=policy.machineIdentity.map(pattern=>new RegExp(pattern,"u"));

/** 🚦️ Where one target of the generated task set stands. */
export type CacheLawStateV1="shared"|"guarded"|"uncacheable"|"violation";
/** 🧾️ The verdict on one target. */
export type CacheLawRowV1=Readonly<{id:string;state:CacheLawStateV1;reason?:string;findings:readonly string[]}>;
/** 🔌️ What the law needs of the Nx projection: the effective Nx target and whether outputs are restorable. */
export type CacheLawProjectionV1=Readonly<{project:(target:BuildTargetV1,name:string)=>{cache?:boolean;outputs?:readonly string[];inputs?:readonly unknown[]};restorable:(outputs:readonly string[])=>boolean}>;

const strip=(text:string)=>{let out=text;for(const token of policy.tokens)out=out.replaceAll(token,"");return out.replace(/^\/+/u,"");};
const textOf=(value:unknown)=>JSON.stringify(value);

/** 🔎️ Finds what is wrong with one effective cacheable task. */
export function cacheLawFindingsV1(target:BuildTargetV1,effective:{outputs?:readonly string[];inputs?:readonly unknown[]},projection:CacheLawProjectionV1,exemptOutputs:boolean):string[]{
 const findings=new Set<string>(),outputs=(effective.outputs??[]).filter(output=>typeof output==="string"&&!output.startsWith("!"));
 if(!exemptOutputs&&outputs.length===0)findings.add("outputs-missing");
 for(const output of outputs){
  const bare=strip(output);
  if(!policy.tokens.some(token=>output.startsWith(token))&&absolute.some(pattern=>pattern.test(output)))findings.add("output-absolute");
  if(identity.some(pattern=>pattern.test(output)))findings.add("output-machine-identity");
  if(bare.split("/").includes(".."))findings.add("output-escapes-workspace");
 }
 if(outputs.length&&!projection.restorable(outputs))findings.add("output-not-restorable");
 const runtimeDigest=(effective.inputs??[]).some(input=>input!==null&&typeof input==="object"&&"runtime" in (input as object));
 for(const input of effective.inputs??[]){
  if(typeof input==="string"){
   const bare=strip(input.replace(/^[!^]/u,""));
   if(!input.startsWith("^")&&!input.startsWith("!")&&!/^[a-z][A-Za-z0-9]*$/u.test(input)&&policy.ignoredInputDirectories.some(directory=>bare.split("/").includes(directory))&&!policy.ignoredInputExceptions.some(exception=>input.startsWith(exception))&&!runtimeDigest)findings.add("input-gitignored-without-digest");
  }else if(input!==null&&typeof input==="object"){
   const row=input as Record<string,unknown>;
   if(typeof row.env==="string"&&policy.machineEnvironment.includes(row.env))findings.add("input-machine-environment");
   if(typeof row.runtime!=="string"&&typeof row.env!=="string"&&typeof row.fileset!=="string"&&typeof row.input!=="string"&&!Array.isArray(row.externalDependencies)&&!("dependentTasksOutputFiles" in row))findings.add("input-uninterpretable");
  }else findings.add("input-uninterpretable");
 }
 const command=textOf((target.execution as {command?:unknown;commands?:unknown;env?:unknown}));
 if(identity.some(pattern=>pattern.test(command)))findings.add("command-machine-identity");
 return [...findings].sort();
}

/** ⚖️ Judges every target of the graph: the effective Nx cache decision (after the projection and its output downgrade) names the state; a cacheable target is checked against the law. */
export function cacheLawV1(graph:BuildGraphV1,projection:CacheLawProjectionV1):CacheLawRowV1[]{
 const rows:CacheLawRowV1[]=[];
 for(const [projectName,project] of Object.entries(graph.projects).sort(([a],[b])=>a<b?-1:1))for(const [name,target] of Object.entries(project.targets).sort(([a],[b])=>a<b?-1:1)){
  const id=`${projectName}:${name}`,effective=projection.project(target,name),decision=(target.metadata as {semio?:{cache?:{class?:string;reason?:string}}}|undefined)?.semio?.cache,why=decision?.class?`${decision.class}/${decision.reason}`:undefined;
  if(effective.cache!==true){
   const downgraded=target.cache===true&&decision?.class==="deterministic"&&effective.cache!==true;
   rows.push({id,state:"uncacheable",reason:downgraded?"outputs-not-restorable":why??(target.cache===false?"declared":"undeclared"),findings:[]});continue;
  }
  const exempt=target.execution.kind==="noop"||decision?.reason==="check-without-outputs"||(target.metadata as {semio?:{rustBuildIntent?:unknown;rustUnitAggregate?:unknown;hostPlatformOutputsV1?:unknown}}|undefined)?.semio?.rustBuildIntent!==undefined||(target.metadata as {semio?:{rustUnitAggregate?:unknown}}|undefined)?.semio?.rustUnitAggregate!==undefined||(target.metadata as {semio?:{hostPlatformOutputsV1?:unknown}}|undefined)?.semio?.hostPlatformOutputsV1!==undefined;
  const findings=cacheLawFindingsV1(target,effective,projection,exempt),guard=remoteGuardV1(name);
  rows.push(guard?{id,state:"guarded",reason:guard.en,findings}:findings.length?{id,state:"violation",reason:why??"",findings}:{id,state:"shared",...(why?{reason:why}:{}),findings});
 }
 return rows;
}

/** 📊️ Counts rows by state, with the uncacheable ones by reason. */
export function cacheLawCountsV1(rows:readonly CacheLawRowV1[]):{states:Record<CacheLawStateV1,number>;uncacheable:Record<string,number>;violations:Record<string,number>}{
 const states={shared:0,guarded:0,uncacheable:0,violation:0},uncacheable:Record<string,number>={},violations:Record<string,number>={};
 for(const row of rows){
  states[row.state]++;
  if(row.state==="uncacheable")uncacheable[row.reason??"?"]=(uncacheable[row.reason??"?"]??0)+1;
  if(row.state==="violation")for(const finding of row.findings)violations[finding]=(violations[finding]??0)+1;
 }
 return {states,uncacheable,violations};
}
