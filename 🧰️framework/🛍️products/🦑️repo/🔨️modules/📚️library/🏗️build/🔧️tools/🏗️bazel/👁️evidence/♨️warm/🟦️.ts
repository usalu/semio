import {parseParityJsonObjectsV1} from "../../../../🪞️parity/📥️evidence/🟦️.ts";

/** ♨️ One spawn of a Bazel execution log with everything that forms its action key: the argument vector, the environment, every input with its digest, and the produced output digests. */
export type BazelWarmSpawnV1=Readonly<{label:string;mnemonic:string;runner:string;cacheHit:boolean;cacheable:boolean;args:readonly string[];env:Readonly<Record<string,string>>;inputs:Readonly<Record<string,string>>;outputs:Readonly<Record<string,string>>}>;

/** 🔎️ Why one action of the warm run did or did not execute: `hit` restored from a cache, `uncached` the action opts out of every cache (`no-cache`, e.g. no declared outputs), `new` absent from the cold run, `key-changed` an input, argument or environment differs, `executed-with-stable-key` identical key yet executed (uncached, evicted or non-hermetic). */
export type BazelWarmVerdictV1=Readonly<{label:string;verdict:"hit"|"uncached"|"new"|"key-changed"|"executed-with-stable-key";changedInputs:readonly string[];addedInputs:readonly string[];removedInputs:readonly string[];argumentsChanged:boolean;environmentChanged:readonly string[];outputsIdentical:boolean|null;derivedInputsChanged:boolean;sourceInputsChanged:number}>;

const record=(value:unknown):Record<string,unknown>|undefined=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as Record<string,unknown>:undefined;
const order=(left:string,right:string):number=>left<right?-1:left>right?1:0;
const digestOf=(value:unknown):string=>String(record(record(value)?.digest)?.hash??"");
const table=(rows:unknown,keyName:string,valueOf:(row:unknown)=>string):Record<string,string>=>Object.fromEntries((Array.isArray(rows)?rows:[]).flatMap(row=>{const key=record(row)?.[keyName];return typeof key==="string"?[[key,valueOf(row)]]:[];}));
const derivedInput=/^((implementation|adapter)\.json|t_[0-9a-f]+\.(input|json))$/u;

/** 📖️ Reads the execution log (pretty-printed or one object per line) into spawn rows. */
export function parseBazelWarmSpawnsV1(text:string):BazelWarmSpawnV1[]{
 return parseParityJsonObjectsV1(text).flatMap(object=>{
  const row=record(object);if(!row||typeof row.targetLabel!=="string")return [];
  const args=Array.isArray(row.commandArgs)?row.commandArgs.filter((value):value is string=>typeof value==="string"):[];
  return [{label:row.targetLabel,mnemonic:typeof row.mnemonic==="string"?row.mnemonic:"",runner:typeof row.runner==="string"?row.runner:"",cacheHit:row.cacheHit===true,cacheable:row.cacheable!==false,args,env:table(row.environmentVariables,"name",entry=>String(record(entry)?.value??"")),inputs:table(row.inputs,"path",digestOf),outputs:table(row.actualOutputs,"path",digestOf)}];
 });
}

/** 🧮️ Attributes every action of the warm run: whether it was restored and, when it ran, which part of its key moved (a stable key that still ran is the hermeticity defect; a moved key names the inputs). */
export function bazelWarmVerdictsV1(cold:readonly BazelWarmSpawnV1[],warm:readonly BazelWarmSpawnV1[]):BazelWarmVerdictV1[]{
 const before=new Map(cold.map(row=>[row.label,row]));
 return [...warm].sort((left,right)=>order(left.label,right.label)).map(row=>{
  const prior=before.get(row.label);
  if(row.cacheHit)return {label:row.label,verdict:"hit",changedInputs:[],addedInputs:[],removedInputs:[],argumentsChanged:false,environmentChanged:[],outputsIdentical:null,derivedInputsChanged:false,sourceInputsChanged:0};
  if(!row.cacheable)return {label:row.label,verdict:"uncached",changedInputs:[],addedInputs:[],removedInputs:[],argumentsChanged:false,environmentChanged:[],outputsIdentical:null,derivedInputsChanged:false,sourceInputsChanged:0};
  if(!prior)return {label:row.label,verdict:"new",changedInputs:[],addedInputs:[],removedInputs:[],argumentsChanged:false,environmentChanged:[],outputsIdentical:null,derivedInputsChanged:false,sourceInputsChanged:0};
  const changed=Object.keys(row.inputs).filter(path=>path in prior.inputs&&prior.inputs[path]!==row.inputs[path]).sort(order),added=Object.keys(row.inputs).filter(path=>!(path in prior.inputs)).sort(order),removed=Object.keys(prior.inputs).filter(path=>!(path in row.inputs)).sort(order);
  const names=[...new Set([...Object.keys(prior.env),...Object.keys(row.env)])].filter(name=>prior.env[name]!==row.env[name]).sort(order),argumentsChanged=JSON.stringify(prior.args)!==JSON.stringify(row.args);
  const moved=changed.length+added.length+removed.length>0||argumentsChanged||names.length>0;
  const outputPaths=new Set([...Object.keys(prior.outputs),...Object.keys(row.outputs)]),outputsIdentical=outputPaths.size===0?null:[...outputPaths].every(path=>prior.outputs[path]===row.outputs[path]);
  return {label:row.label,verdict:moved?"key-changed":"executed-with-stable-key",changedInputs:changed,addedInputs:added,removedInputs:removed,argumentsChanged,environmentChanged:names,outputsIdentical,derivedInputsChanged:changed.some(path=>derivedInput.test(path)),sourceInputsChanged:changed.filter(path=>!derivedInput.test(path)).length+added.length+removed.length};
 });
}
