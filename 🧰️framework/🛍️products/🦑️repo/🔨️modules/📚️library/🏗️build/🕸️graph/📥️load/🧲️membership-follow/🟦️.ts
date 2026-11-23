import {isAbsolute,join,relative,sep} from "node:path";
import {concurrentEditOfV1,STALE_IMPLEMENTATION_CODES_V1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

type OutcomesV1=Readonly<{memo:number;checkpoint:number;discovery:number}>;
type TextV1=Readonly<{en:string;de:string}>;
const texts=schema.$defs.Texts.const;
const fill=(template:string,values:Readonly<Record<string,string|number>>)=>template.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]));
const both=(text:{readonly en:string;readonly de:string},values:Readonly<Record<string,string|number>>):TextV1=>({en:fill(text.en,values),de:fill(text.de,values)});

/** 🧲️ The absolute paths a `BUILD_MEMBERSHIP_CHANGED` refusal names (root-relative ones are placed under `root`); none when the refusal names no path.
 * @see ./🧪️tests/🧲️membership-follow */
export function membershipChangePathsV1(root:string,error:unknown):string[]{
  const named=(error as {paths?:unknown} | null)?.paths;
  return Array.isArray(named)?named.filter((path):path is string=>typeof path==="string").map(path=>isAbsolute(path)?path:join(root,path)):[];
}

const insideRootV1=(root:string,path:string):boolean=>{const distance=relative(root,path);return distance!==""&&distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);};

/** 👻️ The absolute member path below `root` that a vanished-path failure (`ENOENT`/`ENOTDIR`) of the inventory walk names; the root itself, paths outside it and every other error answer none.
 * @see ./🧪️tests/🧲️membership-follow */
export function vanishedMemberPathV1(root:string,error:unknown):string|undefined{
  const failure=error as NodeJS.ErrnoException|null,path=failure?.path;
  if(failure?.code!=="ENOENT"&&failure?.code!=="ENOTDIR"||typeof path!=="string"||!isAbsolute(path))return undefined;
  return insideRootV1(root,path)?path:undefined;
}

/** 🌊️ The absolute paths below `root` that a failure of the inventory names as moved under the walk (a vanished or retyped member, a directory that disappeared between its listing and its read, a typed concurrent edit, also as the cause of a wrapper), so the follow-up rounds re-observe only the owners that read them instead of ending the process; a stale implementation, an error that names no path of the tree and every other failure answer none and stay fatal.
 * @see ./🧪️tests/🌊️vanished-directory */
export function followedPathsV1(root:string,error:unknown):string[]{
  for(let depth=0,value:unknown=error;value!==null&&typeof value==="object"&&depth<=4;depth++,value=(value as {cause?:unknown}).cause){
    if(STALE_IMPLEMENTATION_CODES_V1.has(String((value as {code?:unknown}).code)))return [];
    const vanished=vanishedMemberPathV1(root,value);if(vanished!==undefined)return [vanished];
  }
  const edit=concurrentEditOfV1(error);
  return edit!==undefined&&isAbsolute(edit.path)&&insideRootV1(root,edit.path)?[edit.path]:[];
}

/** 🗣️ The bilingual line of one membership follow-up round.
 * @see ./🧪️tests/🧲️membership-follow */
export function graphRetryTextV1(round:number,path:string,paths:number,evicted:number,outcomes:OutcomesV1,elapsedMilliseconds:number):TextV1{
  return both(texts.retry,{round,path,paths,evicted,...outcomes,seconds:Math.round(elapsedMilliseconds/1000)});
}

/** 🗣️ The bilingual line of the round that settled the inventory after follow-ups.
 * @see ./🧪️tests/🧲️membership-follow */
export function graphInventoryTextV1(round:number,outcomes:OutcomesV1,elapsedMilliseconds:number):TextV1{
  return both(texts.settled,{round,...outcomes,seconds:Math.round(elapsedMilliseconds/1000)});
}

/** 🗣️ The bilingual line of one settle round of the owner phase: which paths kept the round alive (a contradiction inside one owner, so that owner is observed again) or were read in two states by different owners (each owner keeps its own consistent snapshot, a later read re-checks the stale one).
 * @see ./🧪️tests/🧲️membership-follow */
export function graphSettleTextV1(round:number,count:number,first:readonly string[],evicted:number,settled:boolean):TextV1{
  return both(settled?texts.churn:texts.contradiction,{round,count,first:first.join(", "),evicted});
}

const names=(list:readonly string[],limit=5)=>list.slice(0,limit).join(", ")+(list.length>limit?" (+"+(list.length-limit)+")":"");

/** 🗣️ The bilingual line of one inventory follow-up round: the dirty owners that are observed again and for how many rounds in a row the same ones were dirty.
 * @see ./🧪️tests/🧲️membership-follow */
export function graphDirtyTextV1(round:number,paths:readonly string[],owners:readonly string[],streak:number):TextV1{
  return both(texts.dirty,{round,paths:paths.length,first:names(paths,3),owners:owners.length,names:names(owners),streak});
}

/** 🗣️ The bilingual line of the inventory that sealed with volatile owners and paths.
 * @see ./🧪️tests/🧲️membership-follow */
export function graphVolatileTextV1(paths:readonly string[],owners:readonly string[]):TextV1{
  return both(texts.volatile,{paths:paths.length,first:names(paths,3),owners:owners.length,names:names(owners)});
}
