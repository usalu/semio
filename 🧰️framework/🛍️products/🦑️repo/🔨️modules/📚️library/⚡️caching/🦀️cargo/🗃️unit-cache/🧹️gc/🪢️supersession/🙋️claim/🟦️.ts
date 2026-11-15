import {readdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout;

/** 🙋️ Makes a key current again in its slot: its newest variant is rewritten with the time of now (a temporary file renamed into place, the old file removed), as a build does when it gets a hit on an older key after a revert. Skipped when the key was stored or claimed within `minimumAgeMilliseconds` (a hot key is not rewritten on every hit). Answers whether it rewrote. */
export function claimUnitV1(root:string,key:string,options:Readonly<{nowMs?:number;minimumAgeMilliseconds?:number;kind?:"entries"|"runs"}>={}):boolean{
 const nowMs=options.nowMs??Date.now(),directory=join(root,options.kind==="runs"?layout.runs:layout.entries,key.slice(0,2),key);
 try{
  const names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort(),newest=names[names.length-1];if(newest===undefined)return false;
  const entry=JSON.parse(readFileSync(join(directory,newest),"utf8")) as {slot?:unknown;createdMs?:number};
  if(typeof entry.slot!=="string")return false;
  if(nowMs-(entry.createdMs??0)<(options.minimumAgeMilliseconds??300000))return false;
  entry.createdMs=nowMs;
  const work=join(root,layout.temporary);try{readdirSync(work);}catch{return false;}
  const temporary=join(work,`claim-${process.pid}-${Math.random().toString(36).slice(2)}`),name=String(nowMs).padStart(15,"0")+`-${Math.random().toString(16).slice(2,8)}.json`;
  writeFileSync(temporary,JSON.stringify(entry));renameSync(temporary,join(directory,name));rmSync(join(directory,newest),{force:true});return true;
 }catch{return false;}
}
