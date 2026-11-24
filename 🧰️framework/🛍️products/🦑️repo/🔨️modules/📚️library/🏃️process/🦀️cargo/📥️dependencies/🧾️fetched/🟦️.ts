import {createHash} from "node:crypto";
import {gunzipSync} from "node:zlib";
import {posix,resolve,sep} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import { runtimeParseTomlV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

const protocol=Object.freeze({...schema.definitions.Protocol.const});
const digestOf=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const underHome=(home:string,path:string)=>{const row=resolve(home,path);if(row!==resolve(home)&&!row.startsWith(resolve(home)+sep))throw Error("Cargo registry stamp escapes its home");return row;};

/** 🧾️ Proves that every package of the named lockfiles was fetched as an archive whose content is the lockfile checksum, so the proof survives any rewrite, touch, move or restoration of an identical archive. */
export type FetchedReceiptV1=Readonly<{version:1;request:string;locks:readonly Readonly<{path:string;digest:string;bytes:number}>[];registry:Readonly<{digest:string;count:number}>}>;
/** 🔌️ Observes the repository Source bytes, the registry directories and the SHA-256 of registry archives; its only write replaces one extracted `registry/src` package directory. */
export type FetchedPortV1=Readonly<{read(path:string):Promise<Uint8Array|undefined>;digest(path:string):Promise<string|undefined>;list(path:string):readonly string[]|undefined;archive(path:string):Promise<Uint8Array|undefined>;unpacked(path:string):Promise<ReadonlyMap<string,string>|undefined>;restore(path:string,files:ReadonlyMap<string,Uint8Array>):Promise<void>;listing(checksum:string):string|undefined;remember(checksum:string,listing:string):void}>;

/** 📜️ Classifies a lockfile: only registry and path packages can be proven fetched by archive checksums; git sources keep the unconditional fetch. */
export function classifyFetchedLockV1(text:string):"registry"|"unsupported"{
 let document:{package?:Array<{source?:unknown}>};try{document=runtimeParseTomlV1(text) as typeof document;}catch{return"unsupported";}
 if(!Array.isArray(document.package))return"unsupported";
 let registry=false;
 for(const row of document.package){if(row.source===undefined)continue;if(typeof row.source!=="string"||!/^(registry|sparse)\+/u.test(row.source))return"unsupported";registry=true;}
 return registry?"registry":"unsupported";
}

/** 🔎️ Names the lockfile cargo writes for one selected manifest: beside the nearest ancestor workspace root, else beside the manifest. */
export async function fetchedLockPathV1(manifest:string,read:FetchedPortV1["read"]):Promise<string|undefined>{
 let directory=posix.dirname(manifest);
 for(let depth=0;depth<64;depth++){
  const bytes=await read(posix.join(directory,"Cargo.toml"));
  if(bytes){let document:{workspace?:unknown;package?:{workspace?:unknown}};try{document=runtimeParseTomlV1(Buffer.from(bytes).toString("utf8")) as typeof document;}catch{return undefined;}
   if(document.workspace!==undefined)return posix.join(directory,"Cargo.lock");
   if(typeof document.package?.workspace==="string"){const explicit=posix.normalize(posix.join(directory,document.package.workspace));return explicit.startsWith("..")?undefined:posix.join(explicit,"Cargo.lock");}}
  if(directory==="."||directory==="")break;
  directory=posix.dirname(directory);
 }
 return posix.join(posix.dirname(manifest),"Cargo.lock");
}

/** 🔐️ The sorted unique `[name-version, checksum]` rows of every registry package a lockfile pins, `undefined` for a lockfile that cannot be read or pins a package without a SHA-256 checksum. */
export function fetchedLockedV1(text:string):ReadonlyArray<readonly[string,string]>|undefined{
 let document:{package?:Array<{name?:unknown;version?:unknown;source?:unknown;checksum?:unknown}>};try{document=runtimeParseTomlV1(text) as typeof document;}catch{return undefined;}
 if(!Array.isArray(document.package))return undefined;
 const rows=new Map<string,readonly[string,string]>();
 for(const row of document.package){if(row.source===undefined)continue;if(typeof row.name!=="string"||typeof row.version!=="string"||/[\\/]/u.test(row.name+row.version)||typeof row.checksum!=="string"||!/^[a-f0-9]{64}$/u.test(row.checksum))return undefined;const name=row.name+"-"+row.version;rows.set(name+" "+row.checksum,[name,row.checksum]);}
 return foldedLockedV1(rows);
}

const foldedLockedV1=(rows:ReadonlyMap<string,readonly[string,string]>)=>[...rows.entries()].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([,row])=>row);

/** 📦️ The `name-version` of every registry package a lockfile pins, `undefined` for a lockfile that cannot be read. */
export function fetchedPackagesV1(text:string):string[]|undefined{const rows=fetchedLockedV1(text);return rows&&[...new Set(rows.map(([name])=>name))];}

/** 📸️ Resolves the immutable registry view of the locked archives: each `name-version` must exist, in whichever registry index directory, with the exact lockfile checksum as content digest. The view digest folds only name and checksum, so the shared registry may drift in inode, mtime, index placement or unrelated packages without changing it; a missing or different archive yields `undefined`. */
export async function fetchedRegistryViewV1(home:string,port:FetchedPortV1,locked:ReadonlyArray<readonly[string,string]>):Promise<FetchedReceiptV1["registry"]|undefined>{
 const listed=port.list(underHome(home,protocol.registryCache));if(listed===undefined||locked.length===0||locked.length>protocol.maximumPackages)return undefined;
 const indexes=[...listed].sort(),held=async([name,checksum]:readonly[string,string])=>{for(const index of indexes)if(await port.digest(underHome(home,posix.join(protocol.registryCache,index,name+".crate")))===checksum)return true;return false;};
 for(let start=0;start<locked.length;start+=32)if((await Promise.all(locked.slice(start,start+32).map(held))).includes(false))return undefined;
 return Object.freeze({digest:digestOf(Buffer.from(JSON.stringify(locked))),count:locked.length});
}

/** ⏺️ Builds the receipt after a successful locked fetch; refuses any lockfile that is not purely registry-sourced. */
export async function captureFetchedV1(input:Readonly<{request:string;manifests:readonly string[];home:string}>,port:FetchedPortV1):Promise<FetchedReceiptV1|undefined>{
 const paths=new Set<string>();
 for(const manifest of input.manifests){const lock=await fetchedLockPathV1(manifest,port.read);if(!lock)return undefined;paths.add(lock);}
 if(!paths.size||paths.size>protocol.maximumLocks)return undefined;
 const locks:Array<FetchedReceiptV1["locks"][number]>=[],packages=new Map<string,readonly[string,string]>();
 for(const path of[...paths].sort()){const bytes=await port.read(path);if(!bytes||bytes.byteLength>protocol.maximumLockBytes)return undefined;const text=Buffer.from(bytes).toString("utf8"),named=fetchedLockedV1(text);if(classifyFetchedLockV1(text)!=="registry"||!named)return undefined;for(const row of named)packages.set(row.join(" "),row);locks.push(Object.freeze({path,digest:digestOf(bytes),bytes:bytes.byteLength}));}
 const registry=await fetchedRegistryViewV1(input.home,port,foldedLockedV1(packages));if(!registry)return undefined;
 const receipt:FetchedReceiptV1=Object.freeze({version:1,request:input.request,locks:Object.freeze(locks),registry});
 return validateJsonSchemaSubset(schema.definitions.Receipt,receipt,schema).length||Buffer.byteLength(JSON.stringify(receipt))>protocol.maximumReceiptBytes?undefined:receipt;
}

/** ✅️ Admits stored receipt text only when it is schema-valid and names this exact request. */
export function readFetchedReceiptV1(text:string,request:string):FetchedReceiptV1|undefined{
 let value:unknown;try{value=JSON.parse(text);}catch{return undefined;}
 return validateJsonSchemaSubset(schema.definitions.Receipt,value,schema).length||(value as FetchedReceiptV1).request!==request?undefined:value as FetchedReceiptV1;
}

/** 🛡️ A receipt holds while every lockfile digest is unchanged and every locked archive still has its lockfile checksum. */
export async function fetchedHoldsV1(receipt:FetchedReceiptV1,home:string,port:FetchedPortV1):Promise<boolean>{
 const packages=new Map<string,readonly[string,string]>();
 for(const lock of receipt.locks){const bytes=await port.read(lock.path);if(!bytes||bytes.byteLength!==lock.bytes||digestOf(bytes)!==lock.digest)return false;const named=fetchedLockedV1(Buffer.from(bytes).toString("utf8"));if(!named)return false;for(const row of named)packages.set(row.join(" "),row);}
 const now=await fetchedRegistryViewV1(home,port,foldedLockedV1(packages));
 return now!==undefined&&now.count===receipt.registry.count&&now.digest===receipt.registry.digest;
}

/** 📦️ The regular files of a gzip tar archive keyed by path below the single top directory; `undefined` for an archive that is not a well-formed ustar/pax/GNU tar or escapes its directory. */
export function fetchedArchiveFilesV1(bytes:Uint8Array):Map<string,Uint8Array>|undefined{
 let tar:Buffer;try{tar=gunzipSync(bytes,{maxOutputLength:512*1024*1024});}catch{return undefined;}
 const files=new Map<string,Uint8Array>(),text=(start:number,length:number)=>{const end=tar.indexOf(0,start);return tar.toString("utf8",start,end<0||end>start+length?start+length:end);};
 let offset=0,longName:string|undefined,pax:Record<string,string>={};
 while(offset+512<=tar.length){
  if(tar.subarray(offset,offset+512).every(byte=>byte===0))return files;
  const size=Number.parseInt(text(offset+124,12).trim()||"0",8),kind=String.fromCharCode(tar[offset+156]||48),body=offset+512,next=body+Math.ceil(size/512)*512;
  if(!Number.isSafeInteger(size)||size<0||body+size>tar.length)return undefined;
  if(kind==="L")longName=text(body,size);
  else if(kind==="x"){const record=tar.toString("utf8",body,body+size);pax={};for(const line of record.split("\n")){const match=/^\d+ ([^=]+)=(.*)$/u.exec(line);if(match)pax[match[1]!]=match[2]!;}}
  else if(kind==="g"){}
  else{
   const prefix=tar.toString("latin1",offset+257,offset+263)==="ustar\0"?text(offset+345,155):"",name=pax.path??longName??(prefix?prefix+"/":"")+text(offset,100);longName=undefined;pax={};
   if(kind==="0"||kind==="7"){
    const parts=posix.normalize(name).split("/");if(posix.isAbsolute(name)||parts.length<2||parts.includes("..")||parts.includes(""))return undefined;
    files.set(parts.slice(1).join("/"),new Uint8Array(tar.subarray(body,body+size)));
   }
  }
  offset=next;
 }
 return undefined;
}

/** 🧱️ Proves every extracted `registry/src` package directory holds exactly the files of its checksum-verified archive plus the Cargo completion marker, and with `repair` re-extracts a missing, partial or altered directory from that archive. `false` means a directory is unusable and could not be, or was not allowed to be, repaired. */
export async function fetchedSourcesV1(home:string,port:FetchedPortV1,locked:ReadonlyArray<readonly[string,string]>,repair:boolean):Promise<boolean>{
 const listed=port.list(underHome(home,protocol.registryCache));if(listed===undefined)return false;
 const indexes=[...listed].sort(),marker=new TextEncoder().encode('{"v":1}');
 const sound=async([name,checksum]:readonly[string,string]):Promise<boolean>=>{
  for(const index of indexes){
   const archive=underHome(home,posix.join(protocol.registryCache,index,name+".crate"));if(await port.digest(archive)!==checksum)continue;
   const directory=underHome(home,posix.join(protocol.registrySource,index,name)),have=await port.unpacked(directory),remembered=port.listing(checksum);
   if(remembered!==undefined&&have?.has(".cargo-ok")&&fetchedListingV1([...have].filter(([path])=>path!==".cargo-ok"))===remembered)return true;
   const bytes=await port.archive(archive),files=bytes&&fetchedArchiveFilesV1(bytes);if(!files)return false;
   const content=[...files].map(([path,body])=>[path,digestOf(body)] as const);port.remember(checksum,fetchedListingV1(content));
   if(have&&have.has(".cargo-ok")&&content.every(([path,digest])=>have.get(path)===digest))return true;
   if(!repair)return false;
   await port.restore(directory,new Map([...files,[".cargo-ok",marker]]));
   const again=await port.unpacked(directory);return again!==undefined&&again.has(".cargo-ok")&&content.every(([path,digest])=>again.get(path)===digest);
  }
  return false;
 };
 for(let start=0;start<locked.length;start+=16)if((await Promise.all(locked.slice(start,start+16).map(sound))).includes(false))return false;
 return true;
}

/** 📇️ The SHA-256 of an archive's sorted `path NUL file-digest LF` rows: a pure function of the archive bytes, so one remembered under the archive checksum can never go stale, and an extracted directory holding exactly the same rows needs no archive read. */
export function fetchedListingV1(rows:Iterable<readonly[string,string]>):string{return createHash("sha256").update([...rows].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([path,digest])=>path+"\0"+digest+"\n").join("")).digest("hex");}

/** 🔐️ The locked `[name-version, checksum]` rows of every lockfile the manifests own; `undefined` when a lockfile is missing, unreadable, not registry-sourced or exceeds the protocol bounds. */
export async function fetchedLockedOfV1(manifests:readonly string[],read:FetchedPortV1["read"]):Promise<ReadonlyArray<readonly[string,string]>|undefined>{
 const paths=new Set<string>(),rows=new Map<string,readonly[string,string]>();
 for(const manifest of manifests){const lock=await fetchedLockPathV1(manifest,read);if(!lock)return undefined;paths.add(lock);}
 if(!paths.size||paths.size>protocol.maximumLocks)return undefined;
 for(const path of paths){const bytes=await read(path);if(!bytes||bytes.byteLength>protocol.maximumLockBytes)return undefined;const text=Buffer.from(bytes).toString("utf8"),named=fetchedLockedV1(text);if(classifyFetchedLockV1(text)!=="registry"||!named)return undefined;for(const row of named)rows.set(row.join(" "),row);}
 return rows.size>protocol.maximumPackages?undefined:foldedLockedV1(rows);
}

/** 🏷️ Where one request's receipt lives inside the registry-ready namespace. */
export function fetchedEntryNameV1(slot:number):string{return "f"+slot+protocol.entrySuffix;}
