import {createHash} from "node:crypto";
import {posix,resolve,sep} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=Object.freeze({...schema.definitions.Protocol.const});
const digestOf=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const underHome=(home:string,path:string)=>{const row=resolve(home,path);if(row!==resolve(home)&&!row.startsWith(resolve(home)+sep))throw Error("Cargo registry stamp escapes its home");return row;};

/** 🧾️ Proves that every package of the named lockfiles was fetched while the registry directories still carried the recorded stamps. */
export type FetchedReceiptV1=Readonly<{version:1;request:string;locks:readonly Readonly<{path:string;digest:string;bytes:number}>[];registry:Readonly<{digest:string;count:number}>}>;
/** 🔌️ Observes the repository Source bytes and the registry directories without granting any write. */
export type FetchedPortV1=Readonly<{read(path:string):Promise<Uint8Array|undefined>;stamp(path:string):Readonly<{dev:string;ino:string;mtimeNs:string}>|undefined;list(path:string):readonly string[]|undefined}>;

/** 📜️ Classifies a lockfile: only registry and path packages can be proven fetched by directory stamps; git sources keep the unconditional fetch. */
export function classifyFetchedLockV1(text:string):"registry"|"unsupported"{
 let document:{package?:Array<{source?:unknown}>};try{document=Bun.TOML.parse(text) as typeof document;}catch{return"unsupported";}
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
  if(bytes){let document:{workspace?:unknown;package?:{workspace?:unknown}};try{document=Bun.TOML.parse(Buffer.from(bytes).toString("utf8")) as typeof document;}catch{return undefined;}
   if(document.workspace!==undefined)return posix.join(directory,"Cargo.lock");
   if(typeof document.package?.workspace==="string"){const explicit=posix.normalize(posix.join(directory,document.package.workspace));return explicit.startsWith("..")?undefined:posix.join(explicit,"Cargo.lock");}}
  if(directory==="."||directory==="")break;
  directory=posix.dirname(directory);
 }
 return posix.join(posix.dirname(manifest),"Cargo.lock");
}

/** 📦️ The `name-version` of every registry package a lockfile pins, `undefined` for a lockfile that cannot be read. */
export function fetchedPackagesV1(text:string):string[]|undefined{
 let document:{package?:Array<{name?:unknown;version?:unknown;source?:unknown}>};try{document=Bun.TOML.parse(text) as typeof document;}catch{return undefined;}
 if(!Array.isArray(document.package))return undefined;
 const rows:string[]=[];
 for(const row of document.package){if(row.source===undefined)continue;if(typeof row.name!=="string"||typeof row.version!=="string"||/[\\/]/u.test(row.name+row.version))return undefined;rows.push(row.name+"-"+row.version);}
 return[...new Set(rows)].sort();
}

/** 📸️ Stamps the downloaded archive of every locked package, in whichever registry index directory holds it, and folds the stamps into one digest. A package that was not fetched yields `undefined`; adding, replacing or removing any other package never changes the digest. */
export function fetchedRegistryStampsV1(home:string,port:FetchedPortV1,packages:readonly string[]):FetchedReceiptV1["registry"]|undefined{
 const listed=port.list(underHome(home,protocol.registryCache));if(listed===undefined||packages.length===0||packages.length>protocol.maximumPackages)return undefined;
 const indexes=[...listed].sort(),rows:string[][]=[];
 for(const name of packages){let found=false;for(const index of indexes){const path=posix.join(protocol.registryCache,index,name+".crate"),stamp=port.stamp(underHome(home,path));if(stamp){rows.push([path,stamp.dev,stamp.ino,stamp.mtimeNs]);found=true;break;}}if(!found)return undefined;}
 return Object.freeze({digest:digestOf(Buffer.from(JSON.stringify(rows))),count:rows.length});
}

/** 🧾️ Builds the receipt after a successful locked fetch; refuses any lockfile that is not purely registry-sourced. */
export async function captureFetchedV1(input:Readonly<{request:string;manifests:readonly string[];home:string}>,port:FetchedPortV1):Promise<FetchedReceiptV1|undefined>{
 const paths=new Set<string>();
 for(const manifest of input.manifests){const lock=await fetchedLockPathV1(manifest,port.read);if(!lock)return undefined;paths.add(lock);}
 if(!paths.size||paths.size>protocol.maximumLocks)return undefined;
 const locks:Array<FetchedReceiptV1["locks"][number]>=[],packages=new Set<string>();
 for(const path of[...paths].sort()){const bytes=await port.read(path);if(!bytes||bytes.byteLength>protocol.maximumLockBytes)return undefined;const text=Buffer.from(bytes).toString("utf8"),named=fetchedPackagesV1(text);if(classifyFetchedLockV1(text)!=="registry"||!named)return undefined;for(const name of named)packages.add(name);locks.push(Object.freeze({path,digest:digestOf(bytes),bytes:bytes.byteLength}));}
 const registry=fetchedRegistryStampsV1(input.home,port,[...packages].sort());if(!registry)return undefined;
 const receipt:FetchedReceiptV1=Object.freeze({version:1,request:input.request,locks:Object.freeze(locks),registry});
 return validateJsonSchemaSubset(schema.definitions.Receipt,receipt,schema).length||Buffer.byteLength(JSON.stringify(receipt))>protocol.maximumReceiptBytes?undefined:receipt;
}

/** ✅️ Admits stored receipt text only when it is schema-valid and names this exact request. */
export function readFetchedReceiptV1(text:string,request:string):FetchedReceiptV1|undefined{
 let value:unknown;try{value=JSON.parse(text);}catch{return undefined;}
 return validateJsonSchemaSubset(schema.definitions.Receipt,value,schema).length||(value as FetchedReceiptV1).request!==request?undefined:value as FetchedReceiptV1;
}

/** 🛡️ A receipt holds while every lockfile digest and every registry directory stamp is unchanged. */
export async function fetchedHoldsV1(receipt:FetchedReceiptV1,home:string,port:FetchedPortV1):Promise<boolean>{
 const packages=new Set<string>();
 for(const lock of receipt.locks){const bytes=await port.read(lock.path);if(!bytes||bytes.byteLength!==lock.bytes||digestOf(bytes)!==lock.digest)return false;const named=fetchedPackagesV1(Buffer.from(bytes).toString("utf8"));if(!named)return false;for(const name of named)packages.add(name);}
 const now=fetchedRegistryStampsV1(home,port,[...packages].sort());
 return now!==undefined&&now.count===receipt.registry.count&&now.digest===receipt.registry.digest;
}

/** 🏷️ Where one request's receipt lives inside the registry-ready namespace. */
export function fetchedEntryNameV1(slot:number):string{return "f"+slot+protocol.entrySuffix;}
