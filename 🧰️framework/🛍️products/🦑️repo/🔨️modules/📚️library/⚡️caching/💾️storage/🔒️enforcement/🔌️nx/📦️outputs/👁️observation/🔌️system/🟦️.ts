import {plainWindowsFilesystemPathV1,resolveWindowsFilesystemPathV1} from "../../../../../../../../../../../🔨️modules/📁️filesystem/📂️paths/🪟️windows/🟦️.ts";
import {watch} from "node:fs";
import {lstat,readlink} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {setImmediate as yieldTurn} from "node:timers/promises";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {OutputObservationV1,OutputObservationPortsV1} from "../🟦️.ts";

/** 🖥️ Uses synchronous system membership registration and an injected ready recursive backend without exporting its runtime types. */
export function createSystemOutputObservationPortsV1(root:string,control:ResourceLeaseControl,recursive:OutputObservationPortsV1["watch"]):OutputObservationPortsV1{
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);const workspace=resolve(process.platform==="win32"?plainWindowsFilesystemPathV1(root):root),wire=workspace.replaceAll("\\","/");
 if(wire!==root)throw Error("Canonical system output workspace required");
 const coordinate=(path:string)=>{const actual=resolve(process.platform==="win32"?plainWindowsFilesystemPathV1(path):path),distance=relative(workspace,actual);if(distance===".."||distance.startsWith(".."+sep)||isAbsolute(distance)||actual.length>256)throw Error("Bounded system output path required");return{actual,distance:distance.replaceAll("\\","/")||"."};};
 const observe=async(path:string):Promise<OutputObservationV1>=>{
  checkResourceLeaseControl(control);const {actual}=coordinate(path);let stat;try{stat=await lstat(actual,{bigint:true});}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return{kind:"missing",identity:""};throw error;}
  checkResourceLeaseControl(control);const identity=stat.dev+":"+stat.ino;
  if(stat.isSymbolicLink()){let link:string;try{link=await readlink(actual);}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return{kind:"missing",identity:""};throw error;}checkResourceLeaseControl(control);const target=process.platform==="win32"?resolveWindowsFilesystemPathV1(dirname(actual),link):resolve(dirname(actual),link),selected=coordinate(target);return{kind:"link",identity:identity+":"+stat.ctimeNs,target:selected.distance==="."?wire:wire+"/"+selected.distance};}
  if(stat.isDirectory())return{kind:"directory",identity:identity+":"+(stat.birthtimeNs||stat.ctimeNs)};
  if(stat.isFile())return{kind:"file",identity:identity+":"+stat.nlink+":"+stat.size+":"+stat.mtimeNs+":"+stat.ctimeNs};
  throw Error("Ordinary physical output observation required");
 };
 return Object.freeze<OutputObservationPortsV1>({observe,watch:async(scope,receive)=>{
  checkResourceLeaseControl(control);coordinate(scope.path);const before=await observe(scope.path);if(before.kind!=="directory"||before.identity!==scope.identity)throw Error("Output subscription identity changed before start");
  if(scope.mode==="recursive")return recursive(scope,receive);
  let stopped=false,ended=false,terminal=false,closed!:()=>void;const settled=new Promise<void>(done=>closed=done),watcher=watch(scope.path,{recursive:false,encoding:"utf8"},(_type,name)=>{
   if(stopped)return;try{const selected=coordinate(name===null?scope.path:join(scope.path,String(name)));receive(null,[{path:selected.distance,type:"update"}]);}catch(error){receive(error,[]);}
  });
  watcher.on("error",error=>{if(!stopped&&!terminal){terminal=true;receive(error,[]);}});watcher.once("close",()=>{ended=true;closed();if(!stopped&&!terminal){terminal=true;receive(Error("Output membership watcher closed unexpectedly"),[]);}});
  return Object.freeze({flush:async()=>{await yieldTurn();return[];},stop:()=>{stopped=true;if(!ended)watcher.close();return settled;}});
 }});
}
