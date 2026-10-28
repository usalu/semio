import {mkdir,readFile,rename,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {randomUUID} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,exact=new Set<string>(policy.classes.flatMap(row=>row.names.map(name=>name.toUpperCase()))),prefixes=policy.classes.flatMap(row=>row.prefixes.map(prefix=>prefix.toUpperCase())),suffixes=policy.classes.flatMap(row=>row.suffixes.map(suffix=>suffix.toUpperCase())),denied=policy.denied.patterns.map(pattern=>new RegExp(pattern,"u"));

/** 🛡️ The authored policy of the environment channel to Bazel action processes. */
export const BAZEL_ACTION_ENVIRONMENT_POLICY_V1=policy;
/** 🧾️ The environment of a process as the build issues it. */
export type BazelActionEnvironmentSourceV1=Readonly<Record<string,string|undefined>>;

/** 🔎️ Tells whether an action receives a variable: the build declared it for a task, or a policy class allows its name and no credential-like pattern matches it; `declared` holds upper-case names. */
export function bazelActionEnvironmentAllowsV1(name:string,declared:ReadonlySet<string>=new Set()):boolean{
 const upper=name.toUpperCase();
 return declared.has(upper)||(exact.has(upper)||prefixes.some(prefix=>upper.startsWith(prefix))||suffixes.some(suffix=>upper.endsWith(suffix)))&&!denied.some(pattern=>pattern.test(upper));
}

/** 📜️ Collects, upper-cased, every environment name a task graph declares anywhere in the given documents: an `env` input row, a `valueEnv` parameter, and the keys of an `env` map (a dashboard choice value). */
export function bazelDeclaredEnvironmentNamesV1(sources:readonly unknown[]):Set<string>{
 const names=new Set<string>(),seen=new WeakSet<object>(),stack:unknown[]=[...sources];
 while(stack.length){
  const value=stack.pop();if(!value||typeof value!=="object"||seen.has(value))continue;
  seen.add(value);
  if(!Array.isArray(value)){
   const row=value as {env?:unknown;valueEnv?:unknown};
   if(typeof row.env==="string")names.add(row.env.toUpperCase());
   else if(row.env&&typeof row.env==="object"&&!Array.isArray(row.env))for(const name of Object.keys(row.env))names.add(name.toUpperCase());
   if(typeof row.valueEnv==="string")names.add(row.valueEnv.toUpperCase());
  }
  for(const child of Object.values(value))stack.push(child);
 }
 return names;
}

/** 🗺️ The environment names a planned task graph declares: the global and per-project named input groups, and the inputs and the metadata (dashboard parameters) of every planned task. */
export function bazelDeclaredEnvironmentOfPlanV1(graph:{readonly namedInputs?:unknown;readonly projects:Readonly<Record<string,{readonly namedInputs?:unknown}>>},plan:{readonly tasks:Readonly<Record<string,{readonly definition:{readonly inputs?:unknown;readonly metadata?:unknown}}>>}):Set<string>{
 return bazelDeclaredEnvironmentNamesV1([graph.namedInputs,...Object.values(graph.projects).map(project=>project.namedInputs),...Object.values(plan.tasks).flatMap(task=>[task.definition.inputs,task.definition.metadata])]);
}

/** 📒️ Records the declared environment names of a plan beside the workspace while the build graph is still held, so that the run phase, which starts after the graph was released and may follow a reused plan, reads this record and never the graph. */
export async function recordBazelDeclaredEnvironmentV1(workspace:string,graph:Parameters<typeof bazelDeclaredEnvironmentOfPlanV1>[0],plan:Parameters<typeof bazelDeclaredEnvironmentOfPlanV1>[1]):Promise<Set<string>>{
 const names=bazelDeclaredEnvironmentOfPlanV1(graph,plan),text=JSON.stringify({version:1,names:[...names].sort()}),path=join(workspace,policy.declared.file);
 if(await readFile(path,"utf8").catch(()=>undefined)===text)return names;
 await mkdir(dirname(path),{recursive:true});const temporary=path+"."+randomUUID().slice(0,8)+".tmp";await writeFile(temporary,text);await rename(temporary,path);
 return names;
}

/** 📖️ Reads the declared environment names a plan recorded; a missing or malformed record is refused in both languages, because an empty set would silently drop a declared input. */
export async function readBazelDeclaredEnvironmentV1(workspace:string):Promise<Set<string>>{
 const path=join(workspace,policy.declared.file),value=JSON.parse(await readFile(path,"utf8").catch(()=>"null")) as {version?:unknown;names?:unknown}|null;
 if(!value||value.version!==1||!Array.isArray(value.names)||value.names.some(name=>typeof name!=="string")){
  const say=(table:typeof schema.definitions.Messages.const.en)=>table.missing.replace("{path}",path);
  throw new Error(say(schema.definitions.Messages.const.en)+" / "+say(schema.definitions.Messages.const.de));
 }
 return new Set(value.names as string[]);
}

/** 🧼️ The environment an action process receives: the declared variables and the allowed classes only, never an ambient desktop, agent, session or credential variable. */
export function bazelActionEnvironmentV1(environment:BazelActionEnvironmentSourceV1,declared:ReadonlySet<string>=new Set()):Record<string,string>{
 return Object.fromEntries(Object.entries(environment).filter((entry):entry is [string,string]=>entry[1]!==undefined&&bazelActionEnvironmentAllowsV1(entry[0],declared)));
}
