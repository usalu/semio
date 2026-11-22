import {copyFile,mkdir,readdir,readFile,rename,rm,stat} from "node:fs/promises";
import {dirname,join,resolve} from "node:path";
import {randomBytes} from "node:crypto";
import {fleetBlobOpenV1,fleetBlobPublishFileV1,fleetBlobStorePathV1,hashFleetFileV1,type FleetFetchContextV1} from "../🟦️.ts";
import {claimFleetWorkV1,type WorkClaimV1} from "../🎟️work-claim/🟦️.ts";

/** 📌️ One crate file of a lockfile: Cargo pins its bytes by the sha256 of the `.crate` archive. */
export type CratePinV1=Readonly<{name:string;version:string;sha256:string}>;

const REGISTRY="registry+https://github.com/rust-lang/crates.io-index",DEFAULT_DIRECTORY="index.crates.io-1949cf8c6b5b557f",MAXIMUM_PINS=40000,PARALLEL=8;

/** 🔒️ Reads the crates.io packages of a `Cargo.lock` with their checksums; every other source (path, git, other registries) has no pin and is skipped. */
export function cargoLockRegistryCratesV1(text:string):readonly CratePinV1[]{
 const rows:CratePinV1[]=[];
 for(const block of text.split(/^\[\[package\]\]\s*$/mu).slice(1)){
  const field=(name:string)=>block.match(new RegExp(`^${name} = "([^"\\n]*)"`,"mu"))?.[1];
  const name=field("name"),version=field("version"),source=field("source"),checksum=field("checksum");
  if(name&&version&&source===REGISTRY&&checksum&&/^[a-f0-9]{64}$/u.test(checksum))rows.push({name,version,sha256:checksum});
 }
 return rows.slice(0,MAXIMUM_PINS);
}

/** 🗂️ The crates.io download cache of a Cargo home: the directory Cargo already made, else the one name Cargo derives for the sparse crates.io index. */
export async function cargoRegistryCacheDirectoryV1(home:string):Promise<string>{
 const base=join(home,"registry","cache");
 try{const found=(await readdir(base)).find(name=>name.startsWith("index.crates.io-"));if(found)return join(base,found);}catch{}
 return join(base,DEFAULT_DIRECTORY);
}

/** 🔎️ Finds the `Cargo.lock` that governs a manifest: the nearest one at or above its directory, not above the workspace root. */
export async function findCargoLockV1(manifest:string,workspace:string):Promise<string|undefined>{
 const top=resolve(workspace);
 for(let directory=dirname(resolve(manifest));;directory=dirname(directory)){
  const candidate=join(directory,"Cargo.lock");
  if((await stat(candidate).catch(()=>undefined))?.isFile())return candidate;
  if(directory===top||dirname(directory)===directory)return undefined;
 }
}

const pool=async<T>(rows:readonly T[],work:(row:T)=>Promise<void>):Promise<void>=>{
 let next=0;
 await Promise.all(Array.from({length:Math.min(PARALLEL,rows.length)},async()=>{for(;;){const index=next++;if(index>=rows.length)return;await work(rows[index]!);}}));
};

/** 📥️ Places the crate files a lockfile pins into the Cargo home's download cache from the fleet (group store, then cache server), by their lockfile sha256; files already there are left alone. Cargo then finds them cached and downloads none of them. */
export async function fleetCratesRestoreV1(context:FleetFetchContextV1,home:string,pins:readonly CratePinV1[]):Promise<Readonly<{placed:number;present:number;missing:number}>>{
 const directory=await cargoRegistryCacheDirectoryV1(home);let placed=0,present=0,missing=0;
 await mkdir(directory,{recursive:true});
 await pool(pins,async pin=>{
  const target=join(directory,`${pin.name}-${pin.version}.crate`);
  if((await stat(target).catch(()=>undefined))?.isFile()){present++;return;}
  const blob=await fleetBlobOpenV1(context,pin.sha256);
  if(!blob){missing++;return;}
  const temporary=`${target}.${randomBytes(4).toString("hex")}.tmp`;
  try{await copyFile(blob.path,temporary);await rename(temporary,target);placed++;}catch{await rm(temporary,{force:true}).catch(()=>{});missing++;}finally{await blob.dispose();}
 });
 return {placed,present,missing};
}

/** 📤️ Publishes the crate files of the download cache that the lockfile pins and the fleet lacks, after proving each against its lockfile sha256. */
export async function fleetCratesPublishV1(context:FleetFetchContextV1,home:string,pins:readonly CratePinV1[]):Promise<number>{
 const directory=await cargoRegistryCacheDirectoryV1(home);let published=0;
 await pool(pins,async pin=>{
  const source=join(directory,`${pin.name}-${pin.version}.crate`),facts=await stat(source).catch(()=>undefined);
  if(!facts?.isFile())return;
  if(context.storeRoot!==undefined&&(await stat(fleetBlobStorePathV1(context.storeRoot,pin.sha256)).catch(()=>undefined))?.isFile())return;
  const actual=await hashFleetFileV1(source);
  if(!actual||actual.sha256!==pin.sha256)return;
  if((await fleetBlobPublishFileV1(context,pin.sha256,source,actual.bytes)).length>0)published++;
 });
 return published;
}

/** 🦀️ Runs a `cargo fetch` between a restore and a publish: the crate files come from the fleet when it has them, the fetch downloads what is still missing from crates.io, and what it downloaded goes back to the fleet. The index entries stay with Cargo. The restore runs under the fleet claim of the lockfile's crate set (`claimFleetWorkV1`, kind `crates`): of any number of machines with one lockfile exactly one downloads while the others wait and restore what it published. Every failure of the fleet is swallowed; the fetch itself runs exactly as before. */
export async function fleetCargoRegistryAroundV1<T>(context:FleetFetchContextV1,request:Readonly<{home:string;manifest:string;workspace:string}>,fetch:()=>Promise<T>):Promise<T>{
 let pins:readonly CratePinV1[]=[],claim:WorkClaimV1|undefined;
 try{const lock=await findCargoLockV1(request.manifest,request.workspace);if(lock)pins=cargoLockRegistryCratesV1(await readFile(lock,"utf8"));if(pins.length)claim=await claimFleetWorkV1(context,{kind:"crates",identity:pins.map(pin=>pin.sha256).sort().join("\n"),ready:async()=>(await fleetCratesRestoreV1(context,request.home,pins)).missing===0});}catch{}
 
 try{
  const result=await fetch();
  try{if(pins.length)await fleetCratesPublishV1(context,request.home,pins);}catch{}
  return result;
 }finally{await claim?.release().catch(()=>{});}
}
