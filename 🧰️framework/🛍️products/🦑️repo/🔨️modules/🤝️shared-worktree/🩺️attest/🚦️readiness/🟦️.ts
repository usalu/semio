import {readFile,readdir} from "node:fs/promises";
import {join} from "node:path";

/** 🚦️ What a worker says about its own readiness in `<store>/v1/origins/<origin>/ready.json` (written by the build sidecar role `ready`; the closed schema lives next to it in `📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json`): only the fields the fleet table shows are read, and a file that does not have them counts as absent. */
export type ReadinessRowV1=Readonly<{originId:string;kind:"worker"|"server";state:string|null;ready:boolean|null;digest:string|null;stage:string|null;stageIndex:number;stageCount:number;attempts:number;exit:number|null;peakMegabytes:number|null;refusal:Readonly<{code:string;en:string;de:string}>|null;ageSeconds:number|null}>;

const isRefusal=(value:unknown):value is {code:string;en:string;de:string}=>typeof value==="object"&&value!==null&&["code","en","de"].every(key=>typeof (value as Record<string,unknown>)[key]==="string");

/** 📋️ One row per origin of the store, sorted by name: the group server (`<group>-server`) is a `server` row without readiness, every other origin is a worker with its `ready.json` or none (`state: null`). */
export async function readinessReportV1(storeRoot:string,now:number=Date.now()):Promise<readonly ReadinessRowV1[]>{
 const base=join(storeRoot,"v1","origins");
 let names:string[];
 try{names=(await readdir(base,{withFileTypes:true})).filter(entry=>entry.isDirectory()).map(entry=>entry.name).sort();}catch{return[];}
 return Promise.all(names.map(async originId=>{
  const kind=originId.endsWith("-server")?"server":"worker",empty:ReadinessRowV1={originId,kind,state:null,ready:null,digest:null,stage:null,stageIndex:0,stageCount:0,attempts:0,exit:null,peakMegabytes:null,refusal:null,ageSeconds:null};
  if(kind==="server")return empty;
  try{
   const doc=JSON.parse(await readFile(join(base,originId,"ready.json"),"utf8")) as Record<string,unknown>;
   if(doc.version!==1||typeof doc.state!=="string"||typeof doc.ready!=="boolean")return empty;
   const number=(key:string):number=>typeof doc[key]==="number"?doc[key] as number:0;
   return{originId,kind,state:doc.state,ready:doc.ready,digest:typeof doc.digest==="string"?doc.digest:null,stage:typeof doc.stage==="string"?doc.stage:null,stageIndex:number("stageIndex"),stageCount:number("stageCount"),attempts:number("attempts"),exit:typeof doc.exit==="number"?doc.exit:null,peakMegabytes:typeof doc.peakMegabytes==="number"?doc.peakMegabytes:null,refusal:isRefusal(doc.refusal)?doc.refusal:null,ageSeconds:typeof doc.updatedMs==="number"?Math.max(0,Math.round((now-doc.updatedMs)/1000)):null};
  }catch{return empty;}
 }));
}
