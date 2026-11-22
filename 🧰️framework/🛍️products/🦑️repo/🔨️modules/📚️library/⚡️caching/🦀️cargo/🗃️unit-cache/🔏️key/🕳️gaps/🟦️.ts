import {mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import {dirname,join,resolve} from "node:path";
import {FLEET_POLICY_V1,localTextV1,portableTextV1,type PortableRootV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import type {RecordedGapV1} from "../🔗️sources/🟦️.ts";

const fleet=FLEET_POLICY_V1,MAXIMUM_FILES=512,HEX=/^[0-9a-f]{64}$/u;
const gapFile=(root:string,slot:string)=>join(root,fleet.layout.state,fleet.layout.gaps,slot.slice(0,2),slot);

/** 🕳️ The files the dependency file of an earlier compilation of this unit identity (its slot: toolchain, target, profile, package, crate, kind, features) listed although the source scan did not reach them, as the key of the next compilation takes them: each by its portable text (roots as tokens), the path on this machine put back. Empty when nothing was recorded. */
export function readGapsV1(root:string,slot:string,roots:readonly PortableRootV1[]):readonly RecordedGapV1[]{
 if(!HEX.test(slot))return[];
 try{
  const value=JSON.parse(readFileSync(gapFile(root,slot),"utf8")) as {version?:number;files?:unknown};
  if(value.version!==1||!Array.isArray(value.files))return[];
  return value.files.filter((name):name is string=>typeof name==="string"&&name!=="").slice(0,MAXIMUM_FILES).map(name=>({name,path:localTextV1(name,roots)}));
 }catch{return[];}
}

/** 🕳️ Remembers the files a finished compilation read that the key did not cover (paths of its dependency file resolved against `cwd`), merged with those recorded before, so the next compilation of the unit identity keys them and is complete instead of skipped for ever. Returns whether the record grew. Best effort, atomic, bounded. */
export function recordGapsV1(root:string,slot:string,input:Readonly<{cwd:string;paths:readonly string[];roots:readonly PortableRootV1[]}>):boolean{
 if(!HEX.test(slot)||input.paths.length===0)return false;
 try{
  const known=new Set(readGapsV1(root,slot,input.roots).map(row=>row.name)),before=known.size;
  for(const path of input.paths)known.add(portableTextV1(resolve(input.cwd,path).replaceAll("\\","/"),input.roots));
  if(known.size===before||known.size>MAXIMUM_FILES)return false;
  const file=gapFile(root,slot),temporary=file+"."+randomBytes(3).toString("hex");
  mkdirSync(dirname(file),{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,files:[...known].sort()}));renameSync(temporary,file);return true;
 }catch{return false;}
}
