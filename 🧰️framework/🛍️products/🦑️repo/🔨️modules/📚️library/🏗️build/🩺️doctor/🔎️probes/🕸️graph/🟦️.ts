import {existsSync,lstatSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import type {DoctorPolicyV1,GraphFactsV1} from "../../🟦️.ts";
import {readJsonFileV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1} from "../📋️contract/🟦️.ts";

const SOURCE=/\.(ts|mjs|js|json)$/u;

/** 🕰️ Finds the newest file time below the implementation roots, skipping tests and fixtures; the walk is bounded and reports nothing when a root is missing. */
export function newestImplementationMillisecondsV1(libraryRoot:string,policy:DoctorPolicyV1["graph"]):number|null{
 let newest:number|null=null,seen=0;const skip=new Set(policy.implementationSkip),stack=policy.implementationRoots.map(root=>join(libraryRoot,...root.split("/")));
 while(stack.length&&seen<policy.walkEntries){
  const directory=stack.pop()!;let names:import("node:fs").Dirent[];
  try{names=readdirSync(directory,{withFileTypes:true});}catch{continue;}
  for(const entry of names){
   if(++seen>policy.walkEntries)break;
   const path=join(directory,entry.name);
   if(entry.isDirectory()){if(!skip.has(entry.name))stack.push(path);continue;}
   if(!entry.isFile()||!SOURCE.test(entry.name))continue;
   try{const time=statSync(path).mtimeMs;if(newest===null||time>newest)newest=time;}catch{}
  }
 }
 return newest;
}

/** 🕸️ The state of the published project graph generation: the pointer, its manifest and whether the implementation changed after it was published (judged by file times, not by digest). */
export const graphProbeV1:DoctorProbeV1=Object.freeze({
 area:"graph",phase:1,
 async read(ports,context){
  const mount=join(ports.root,...budget.partitions.graph.directory.split("/"),"m");
  if(!existsSync(mount))return {unavailable:"The graph image is not attached or was never created"};
  const store=join(mount,"generations"),pointerPath=join(store,"current.json");
  let stat;try{stat=lstatSync(pointerPath);}catch{
   const absent:GraphFactsV1={pointer:"absent",generation:null,manifest:"none",owners:null,publishedAgeSeconds:null,implementationNewerSeconds:null};
   return {sections:{graph:absent}};
  }
  const pointer=readJsonFileV1(pointerPath,65536) as {version?:unknown;generation?:unknown;manifest?:unknown}|undefined,age=Math.max(0,Math.floor((ports.now()-stat.mtimeMs)/1000));
  if(!pointer||pointer.version!==1||!Number.isSafeInteger(pointer.generation)||!(pointer.manifest===null||typeof pointer.manifest==="string"&&/^[a-f0-9]{64}$/u.test(pointer.manifest)))
   return {sections:{graph:{pointer:"damaged",generation:null,manifest:"none",owners:null,publishedAgeSeconds:age,implementationNewerSeconds:null}}};
  const generation=pointer.generation as number;
  let manifest:GraphFactsV1["manifest"]="none",owners:number|null=null;
  if(typeof pointer.manifest==="string"){
   const path=join(store,"m",pointer.manifest+".json");
   if(!existsSync(path))manifest="missing";
   else{
    const value=readJsonFileV1(path,268435456) as {owners?:unknown}|undefined;
    if(value&&typeof value==="object"&&value.owners&&typeof value.owners==="object"){manifest="present";owners=Object.keys(value.owners).length;}else manifest="damaged";
   }
  }
  const newest=newestImplementationMillisecondsV1(context.libraryRoot,context.policy.graph),newer=newest===null?null:Math.max(0,Math.floor((newest-stat.mtimeMs)/1000));
  return {sections:{graph:{pointer:"present",generation,manifest,owners,publishedAgeSeconds:age,implementationNewerSeconds:newer}}};
 },
});
