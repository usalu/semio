import {createHash} from "node:crypto";
import {lstatSync,mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {setImmediate as yieldEventLoop} from "node:timers/promises";

/** 🧾️ One observed path: its kind, device, inode and mode, or `missing`; the identity the ownership walk read. */
export type OwnershipStateV1=readonly (string|number)[];

/** 🔭️ What one ownership walk observed, as the walk's custody reports it. */
export type OwnershipObservationV1=Readonly<{paths:readonly Readonly<{path:string;state:OwnershipStateV1;list?:unknown}>[];texts:readonly Readonly<{path:string;sha256:string}>[]}>;

/** 📜️ The sealed memo of one walk: the identity of every observed path, the physical stamp of every listed directory, the digest of every manifest read, and the aliases derived from them. */
export type OwnershipMemoV1=Readonly<{version:1;implementation:string;workspace:string;owner:string;states:readonly (readonly [string,OwnershipStateV1])[];directories:readonly (readonly [string,string])[];manifests:readonly (readonly [string,string])[];aliases:readonly Readonly<{find:string;flags:string;replacement:string}>[]}>;

/** ⏱️ A directory counts as settled when neither its modification nor its change time lies within this many milliseconds before the walk started: a later change then moves a time beyond what the memo recorded, whatever the timestamp granularity. */
export const OWNERSHIP_MEMO_SETTLE_MILLISECONDS_V1=5000;

const slash=(path:string):string=>path.split(sep).join("/"),natural=(root:string,path:string):string=>join(root,...path.split("/")),kindOf=(entry:{isSymbolicLink():boolean;isDirectory():boolean;isFile():boolean}):string=>entry.isSymbolicLink()?"symlink":entry.isDirectory()?"directory":entry.isFile()?"file":"other";
const stateOf=(path:string):OwnershipStateV1=>{try{const entry=lstatSync(path,{bigint:true});return [kindOf(entry),String(entry.dev),String(entry.ino),Number(entry.mode)];}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return ["missing"];throw error;}};
const stampOf=(path:string):{state:OwnershipStateV1;stamp:string;settledAfter:bigint}|undefined=>{try{const entry=lstatSync(path,{bigint:true});return {state:[kindOf(entry),String(entry.dev),String(entry.ino),Number(entry.mode)],stamp:[entry.dev,entry.ino,entry.mode,entry.mtimeNs,entry.ctimeNs].map(String).join(":"),settledAfter:entry.mtimeNs>entry.ctimeNs?entry.mtimeNs:entry.ctimeNs};}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}};

/** 🔏️ Digests the implementation whose verdicts a memo may serve: the ownership walk and its policy. `undefined` when either cannot be read, which disables the memo. */
export function ownershipImplementationDigestV1(walk:URL,policy:URL):string|undefined{
 try{return createHash("sha256").update(readFileSync(walk)).update("\0").update(readFileSync(policy)).digest("hex");}catch{return undefined;}
}

/** 📍️ The memo file of one workspace and owner under a cache directory. */
export function ownershipMemoPathV1(directory:string,workspace:string,owner:string):string{
 return join(directory,"workspace-ownership-"+createHash("sha256").update(resolve(workspace)).update("\0").update(owner).digest("hex").slice(0,24)+".json");
}

/** 📖️ Reads a memo; a missing, torn or foreign file is no memo. */
export function readOwnershipMemoV1(file:string):OwnershipMemoV1|undefined{
 try{const memo=JSON.parse(readFileSync(file,"utf8")) as OwnershipMemoV1;return memo.version===1&&typeof memo.implementation==="string"&&Array.isArray(memo.directories)&&Array.isArray(memo.manifests)&&Array.isArray(memo.states)&&Array.isArray(memo.aliases)?memo:undefined;}catch{return undefined;}
}

/** 💾️ Seals the memo of a walk that just finished, or answers `undefined` when the observed tree was not settled when the walk began (a later check could not tell an edit from the walk itself). The stamps are read after the walk; a directory changed since the walk began carries a time inside the settle window and refuses the memo. */
export function sealOwnershipMemoV1(options:Readonly<{workspace:string;owner:string;implementation:string;startedAtMs:number;settleMilliseconds?:number;observed:OwnershipObservationV1;aliases:readonly Readonly<{find:RegExp;replacement:string}>[]}>):OwnershipMemoV1|undefined{
 const root=resolve(options.workspace),limit=BigInt(options.startedAtMs-(options.settleMilliseconds??OWNERSHIP_MEMO_SETTLE_MILLISECONDS_V1))*1000000n,states:[string,OwnershipStateV1][]=[],directories:[string,string][]=[];
 for(const row of options.observed.paths){
  const path=slash(relative(root,row.path));
  if(row.list===undefined){states.push([path,row.state]);continue;}
  const live=stampOf(row.path);
  if(!live||live.settledAfter>=limit||JSON.stringify(live.state)!==JSON.stringify(row.state))return undefined;
  directories.push([path,live.stamp]);
 }
 return {version:1,implementation:options.implementation,workspace:root,owner:options.owner,states,directories,manifests:options.observed.texts.map(row=>[slash(isAbsolute(row.path)?relative(root,row.path):row.path),row.sha256] as [string,string]),aliases:options.aliases.map(row=>({find:row.find.source,flags:row.find.flags,replacement:slash(relative(root,row.replacement))}))};
}

/** 💽️ Writes a memo atomically: a reader sees the old file or the new one. */
export function writeOwnershipMemoV1(file:string,memo:OwnershipMemoV1):void{
 mkdirSync(dirname(file),{recursive:true});const staging=file+"."+process.pid+".tmp";writeFileSync(staging,JSON.stringify(memo));renameSync(staging,file);
}

/** ✅️ Answers whether everything a memo observed still holds (`progress` hears every check before it is made): every listed directory carries the recorded stamp, every other observed path the recorded identity and every manifest the recorded digest. Returns the number of checks, or `undefined` at the first difference. Yields to the event loop by time slices and stops at an aborted signal. */
export async function ownershipMemoCurrentV1(memo:OwnershipMemoV1,workspace:string,implementation:string,signal:AbortSignal,progress?:(checked:number,path:string)=>void|Promise<void>):Promise<number|undefined>{
 const root=resolve(workspace);if(memo.implementation!==implementation||memo.workspace!==root)return undefined;
 let checks=0,sliceStart=Number.NEGATIVE_INFINITY;
 const slice=async(path:string):Promise<void>=>{signal.throwIfAborted();if(progress)await progress(checks,path);signal.throwIfAborted();if(performance.now()-sliceStart>=20){await yieldEventLoop();signal.throwIfAborted();sliceStart=performance.now();}};
 for(const[path,stamp]of memo.directories){await slice(path);const live=stampOf(natural(root,path));if(!live||live.stamp!==stamp)return undefined;checks++;}
 for(const[path,state]of memo.states){await slice(path);if(JSON.stringify(stateOf(natural(root,path)))!==JSON.stringify(state))return undefined;checks++;}
 for(const[path,sha256]of memo.manifests){await slice(path);let text:string;try{text=readFileSync(natural(root,path),"utf8");}catch{return undefined;}if(createHash("sha256").update(text).digest("hex")!==sha256)return undefined;checks++;}
 return checks;
}

/** 🗂️ Rebuilds the aliases a memo holds against the workspace. */
export function ownershipMemoAliasesV1(memo:OwnershipMemoV1,workspace:string):readonly Readonly<{find:RegExp;replacement:string}>[]{
 const root=resolve(workspace);return memo.aliases.map(row=>Object.freeze({find:new RegExp(row.find,row.flags),replacement:natural(root,row.replacement)}));
}
