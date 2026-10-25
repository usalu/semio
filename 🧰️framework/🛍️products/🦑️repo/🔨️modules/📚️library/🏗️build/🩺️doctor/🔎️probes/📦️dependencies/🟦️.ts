import {existsSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import type {DependencyScopeFactsV1} from "../../🟦️.ts";
import {readJsonFileV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1} from "../📋️contract/🟦️.ts";
import {selectPackageManagerV1} from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";

const manager=selectPackageManagerV1("javascript"),INPUTS=[...manager.manifestFileNames(),manager.primaryLockName(),...manager.configurationNames("scope")],READY="bindings.json",KEY=/^[a-f0-9]{20}$/u;
type Pointer=Readonly<{version?:unknown;purpose?:unknown;scope?:unknown;generation?:unknown;publishedAtMs?:unknown;bytes?:unknown;retired?:unknown}>;

const mtime=(path:string):number|null=>{try{return statSync(path).mtimeMs;}catch{return null;}};

/** 📦️ The published dependency generations per scope: whether each pointer reaches a finished generation, whether the scope's own inputs changed after publication, and how many superseded or unfinished generations lie around. */
export const dependenciesProbeV1:DoctorProbeV1=Object.freeze({
 area:"dependencies",phase:1,
 async read(ports){
  const mount=join(ports.root,...budget.partitions.dependencies.directory.split("/"),"m");
  if(!existsSync(mount))return {unavailable:"The dependency image is not attached or was never created"};
  const pointerDirectory=join(mount,"pointer"),generations=join(mount,"j"),now=ports.now();
  let names:string[]=[];try{names=readdirSync(pointerDirectory).filter(name=>name.endsWith(".pointer.json"));}catch{}
  const scopes:DependencyScopeFactsV1[]=[],referenced=new Set<string>(),sizes:number[]=[];
  for(const name of names){
   const pointer=readJsonFileV1(join(pointerDirectory,name),65536) as Pointer|undefined;
   if(!pointer||pointer.version!==1||pointer.purpose!=="dependency-generation-pointer"||typeof pointer.scope!=="string"||typeof pointer.generation!=="string"||!/^[a-f0-9]{20,64}$/u.test(pointer.generation))continue;
   const key=pointer.generation.slice(0,20),directory=join(generations,key);referenced.add(key);
   if(typeof pointer.bytes==="number")sizes.push(pointer.bytes);
   for(const row of Array.isArray(pointer.retired)?pointer.retired:[]){const id=typeof row==="string"?row:(row as {generation?:unknown})?.generation;if(typeof id==="string"&&id.length>=20)referenced.add(id.slice(0,20));}
   const published=typeof pointer.publishedAtMs==="number"?pointer.publishedAtMs:now,base=pointer.scope==="."?ports.root:join(ports.root,pointer.scope);
   const newer=INPUTS.find(file=>{const time=mtime(join(base,file));return time!==null&&time>published+2000;});
   scopes.push({scope:pointer.scope,generation:pointer.generation,directory:!existsSync(directory)?"missing":existsSync(join(directory,READY))?"ready":"unready",inputsNewer:newer===undefined?null:(pointer.scope==="."?newer:pointer.scope+"/"+newer)});
  }
  let orphans=0,unready=0,oldest:number|null=null;
  try{
   for(const entry of readdirSync(generations,{withFileTypes:true})){
    if(!entry.isDirectory()||!KEY.test(entry.name)||referenced.has(entry.name))continue;
    const path=join(generations,entry.name);
    if(existsSync(join(path,READY)))orphans++;
    else{const time=mtime(path);if(time!==null){unready++;const age=Math.max(0,Math.floor((now-time)/1000));if(oldest===null||age>oldest)oldest=age;}}
   }
  }catch{}
  const average=sizes.length?Math.round(sizes.reduce((sum,value)=>sum+value,0)/sizes.length):0;
  return {sections:{dependencies:{pointers:names.length?"present":"absent",scopes,orphans,orphanEstimatedBytes:orphans*average,unready,unreadyOldestSeconds:oldest}}};
 },
});
