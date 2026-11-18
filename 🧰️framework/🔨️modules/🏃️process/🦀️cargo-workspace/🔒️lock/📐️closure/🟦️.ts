import{createHash}from"node:crypto";
import{lstat,readFile}from"node:fs/promises";
import{basename}from"node:path";
import{concurrentEditV1}from"../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{runtimeParseTomlV1}from"../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const protocol=schema.definitions.Protocol.const;
const sha=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");
const quoted=/"((?:[^"\\]|\\.)*)"/gu;

/** 🧶️ The part of a `Cargo.lock` one build depends on: `whole` when the file could not be read as a closed lockfile (the digest is then the digest of its bytes), else the digest of the header, the lock rows of the packages reachable from the build roots and the non-package sections, with the identities of those packages. */
export type CargoLockClosureV1=Readonly<{version:1;whole:boolean;digest:string;packages:readonly string[]}>;
type RowV1=Readonly<{name:string;version:string;source:string|null;text:string;dependencies:readonly string[]}>;

const unquote=(value:string):string|undefined=>{try{const parsed:unknown=JSON.parse('"'+value+'"');return typeof parsed==="string"?parsed:undefined;}catch{return undefined;}};
const field=(lines:readonly string[],key:string):string|null|undefined=>{
 const prefix=key+' = "',row=lines.find(line=>line.startsWith(prefix));if(row===undefined)return null;
 if(!row.endsWith('"')||row.length<prefix.length+1)return undefined;return unquote(row.slice(prefix.length,-1));
};
function rowOf(lines:readonly string[]):RowV1|undefined{
 const name=field(lines,"name"),version=field(lines,"version"),source=field(lines,"source");
 if(!name||!version||source===undefined)return;
 const start=lines.findIndex(line=>line==="dependencies = [");let dependencies:string[]=[];
 if(start>=0){
  const end=lines.indexOf("]",start);if(end<0)return;
  for(const line of lines.slice(start+1,end)){const match=/^ "((?:[^"\\]|\\.)*)",$/u.exec(line);if(!match)return;const dependency=unquote(match[1]!);if(dependency===undefined)return;dependencies.push(dependency);}
 }else if(lines.some(line=>line.startsWith("dependencies")))return;
 return Object.freeze({name,version,source,text:lines.join("\n"),dependencies:Object.freeze(dependencies)});
}
const identityOf=(row:RowV1):string=>row.name+" "+row.version+(row.source===null?"":" "+row.source);

/** 🧿️ Projects `Cargo.lock` onto the packages reachable from `roots` (the package names a build selects): a lock row of a package outside that closure, a new, removed or reordered row, changes nothing, any change in the header, in a closure row or in a non-package section changes the digest. Anything that is not a closed lockfile, or a root or dependency the lock does not name, gives the whole-file digest, so the projection can only be coarser than the file, never finer. */
export function projectCargoLockClosureV1(content:string|Uint8Array,roots:readonly string[]):CargoLockClosureV1{
 const whole=():CargoLockClosureV1=>Object.freeze({version:1,whole:true,digest:sha(content),packages:Object.freeze([])});
 if(!roots.length||content.length>protocol.maximumBytes)return whole();
 let text:string;try{text=new TextDecoder("utf-8",{fatal:true}).decode(typeof content==="string"?new TextEncoder().encode(content):content);}catch{return whole();}
 if(text.includes("\r")||text.includes("\0"))return whole();
 const lines=text.split("\n"),header:string[]=[],trailer:string[]=[],rows:RowV1[]=[];let block:string[]|undefined,inPackage=false;
 const close=():boolean=>{if(!block)return true;while(block.length&&block[block.length-1]==="")block.pop();const row=rowOf(block);if(!row)return false;rows.push(row);block=undefined;return true;};
 for(const line of lines){
  if(line==="[[package]]"){if(!close())return whole();block=[line];inPackage=true;continue;}
  if(line.startsWith("[")){if(!close())return whole();inPackage=false;trailer.push(line);continue;}
  if(inPackage&&block)block.push(line);else if(trailer.length)trailer.push(line);else header.push(line);
 }
 if(!close()||rows.length>protocol.maximumPackages)return whole();
 const byName=new Map<string,RowV1[]>();for(const row of rows)byName.set(row.name,[...byName.get(row.name)??[],row]);
 const seen=new Set<RowV1>(),queue:RowV1[]=[];
 for(const root of roots){const named=byName.get(root);if(!named)return whole();for(const row of named)if(!seen.has(row)){seen.add(row);queue.push(row);}}
 while(queue.length){
  const row=queue.pop()!;
  for(const dependency of row.dependencies){
   const match=/^(\S+)(?: (\S+)(?: \((.+)\))?)?$/u.exec(dependency);if(!match)return whole();
   const found=(byName.get(match[1]!)??[]).filter(candidate=>(match[2]===undefined||candidate.version===match[2])&&(match[3]===undefined||candidate.source===match[3]));
   if(!found.length)return whole();
   for(const next of found)if(!seen.has(next)){seen.add(next);queue.push(next);}
  }
 }
 const closure=[...seen].sort((a,b)=>identityOf(a)<identityOf(b)?-1:identityOf(a)>identityOf(b)?1:a.text<b.text?-1:a.text>b.text?1:0);
 return Object.freeze({version:1,whole:false,digest:sha(JSON.stringify([protocol.domain,header.join("\n"),closure.map(row=>row.text),trailer.join("\n")])),packages:Object.freeze(closure.map(identityOf))});
}

/** 🪡️ The package names one Cargo invocation builds: the `-p`/`--package` selectors (a `name@version` or `name:version` spec names `name`), else the package of the selected manifest; none when the invocation selects the whole workspace or the manifest names no package, which makes the projection whole. */
export function cargoLockRootsV1(manifestText:string|null,args:readonly string[]):readonly string[]{
 const names=new Set<string>(),boundary=args.indexOf("--"),selected=boundary<0?args:args.slice(0,boundary);
 for(let index=0;index<selected.length;index++){
  const arg=selected[index]!;
  if(arg==="--workspace"||arg==="--all"||arg==="--exclude"||arg.startsWith("--exclude="))return Object.freeze([]);
  const value=arg==="-p"||arg==="--package"?selected[++index]:arg.startsWith("--package=")?arg.slice(10):arg.startsWith("-p")&&arg.length>2?arg.slice(2):undefined;
  if(value!==undefined){const name=value.split(/[@:]/u)[0]!;if(!name)return Object.freeze([]);names.add(name);}
 }
 if(names.size)return Object.freeze([...names].sort());
 if(manifestText===null)return Object.freeze([]);
 let inPackage=false;for(const line of manifestText.split("\n")){
  const text=line.trim();if(text.startsWith("[")){inPackage=text==="[package]";continue;}
  if(inPackage){const match=/^name\s*=\s*"([^"\\]+)"\s*(?:#.*)?$/u.exec(text);if(match)return Object.freeze([match[1]!]);}
 }
 return Object.freeze([]);
}

const canonical=(value:unknown):string=>{
 if(value===null||typeof value==="string"||typeof value==="boolean")return JSON.stringify(value);
 if(typeof value==="number")return Number.isFinite(value)?JSON.stringify(value):JSON.stringify(String(value));
 if(typeof value==="bigint")return JSON.stringify(String(value)+"n");
 if(value instanceof Date)return JSON.stringify(value.toISOString());
 if(Array.isArray(value))return"["+value.map(canonical).join(",")+"]";
 if(typeof value==="object")return"{"+Object.keys(value as object).sort().map(key=>JSON.stringify(key)+":"+canonical((value as Record<string,unknown>)[key])).join(",")+"}";
 throw Error("Closed manifest value required");
};

/** 🧷️ The part of a `Cargo.toml` one build depends on: the digest of its parsed value without comments, formatting, key order and the workspace's `members`, `exclude` and `default-members` lists (a new crate in the workspace is one more line there, and the compilation plan, which is rechecked on its own, decides whether the build's closure moved). A file that does not parse gives the digest of its bytes. */
export function projectCargoManifestV1(content:string|Uint8Array):Readonly<{whole:boolean;digest:string}>{
 const whole=Object.freeze({whole:true,digest:sha(content)});
 if(content.length>protocol.maximumBytes)return whole;
 try{
  const text=new TextDecoder("utf-8",{fatal:true}).decode(typeof content==="string"?new TextEncoder().encode(content):content),value=runtimeParseTomlV1(text) as Record<string,unknown>;
  const workspace=value.workspace;
  const kept=workspace&&typeof workspace==="object"&&!Array.isArray(workspace)?{...value,workspace:Object.fromEntries(Object.entries(workspace).filter(([key])=>!protocol.manifestElided.includes(key)))}:value;
  return Object.freeze({whole:false,digest:sha(JSON.stringify([protocol.manifestDomain,canonical(kept)]))});
 }catch{return whole;}
}

/** 🔭️ One physical read of a file with the identity it had before and after it, or `undefined` when it does not exist. */
export type CargoLockReadV1=(path:string)=>Promise<Readonly<{bytes:Uint8Array;before:string;after:string}>|undefined>;
const physicalRead:CargoLockReadV1=async path=>{
 const identity=async():Promise<string|undefined>=>{try{const row=await lstat(path,{bigint:true});return row.isFile()&&!row.isSymbolicLink()?[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":"):"other";}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}};
 const before=await identity();if(before===undefined||before==="other")return;
 const bytes=await readFile(path),after=await identity();if(after===undefined)return;
 return Object.freeze({bytes,before,after});
};
const pause=(milliseconds:number):Promise<void>=>new Promise(done=>setTimeout(done,milliseconds));
async function stableRead(path:string,read:CargoLockReadV1,sleep:(milliseconds:number)=>Promise<void>):Promise<Uint8Array|undefined>{
 for(let attempt=1;attempt<=protocol.readAttempts;attempt++){
  const row=await read(path);if(!row)return;
  if(row.before===row.after)return row.bytes;
  if(attempt<protocol.readAttempts)await sleep(protocol.readPauseMilliseconds);
 }
 throw concurrentEditV1("bytes",path,"Cargo.lock kept changing while it was read","snapshot");
}

/** 🧵️ What a build keeps of one observed file: the digest its recipe is keyed by, and the check that what it was built from is still on disk. */
export type CargoLockClosureWitnessV1=Readonly<{path:string;digest:string;packages:readonly string[];recheck:()=>Promise<void>}>;
type ReadOptionsV1=Readonly<{read?:CargoLockReadV1;sleep?:(milliseconds:number)=>Promise<void>}>;
async function observeProjectionV1(path:string,project:(bytes:Uint8Array)=>Readonly<{whole:boolean;digest:string;packages?:readonly string[]}>,moved:string,options:ReadOptionsV1):Promise<CargoLockClosureWitnessV1|undefined>{
 const read=options.read??physicalRead,sleep=options.sleep??pause,bytes=await stableRead(path,read,sleep);if(!bytes)return;
 const first=project(bytes);if(first.whole)return;
 return Object.freeze({path,digest:first.digest,packages:first.packages??Object.freeze([]),recheck:async()=>{
  const current=await stableRead(path,read,sleep);if(!current)throw concurrentEditV1("vanished",path,basename(path)+" disappeared","build");
  const now=project(current);
  if(now.whole||now.digest!==first.digest)throw concurrentEditV1("input",path,moved,"build");
 }});
}
/** 🪢️ Observes `path` through its closure projection; `undefined` when the file is absent or the projection is the whole file, so the caller keeps its byte-exact observation for those. The recheck re-reads the file and passes while the projection is unchanged, however the rest of the lock moved; a changed closure raises a concurrent `input` edit of the `build` layer (the plan is stale), a vanished file a `vanished` one. */
export function observeCargoLockClosureV1(path:string,roots:readonly string[],options:ReadOptionsV1={}):Promise<CargoLockClosureWitnessV1|undefined>{
 return observeProjectionV1(path,bytes=>projectCargoLockClosureV1(bytes,roots),"Cargo.lock changed in the packages the build depends on",options);
}
/** 📃️ The same observation for a `Cargo.toml`, through {@link projectCargoManifestV1}. */
export function observeCargoManifestV1(path:string,options:ReadOptionsV1={}):Promise<CargoLockClosureWitnessV1|undefined>{
 return observeProjectionV1(path,projectCargoManifestV1,"Cargo.toml changed in what the build depends on",options);
}

/** 🧮️ The inputs of a Cargo target recipe: every path's digest from `physical`, except a `Cargo.lock` and a `Cargo.toml` whose projection holds, which are keyed and rechecked by their projections. */
export async function cargoRecipeInputsV1(paths:readonly string[],roots:readonly string[],physical:Readonly<{digest:(path:string)=>Promise<string|undefined>;recheck:()=>Promise<void>}>,step:(path:string)=>Promise<void>=async()=>{},options:ReadOptionsV1={}):Promise<Readonly<{inputs:readonly Readonly<{path:string;sha256:string|null}>[];projected:readonly string[];recheck:()=>Promise<void>}>>{
 const inputs:{path:string;sha256:string|null}[]=[],witnesses:CargoLockClosureWitnessV1[]=[];
 for(const path of paths){
  const name=basename(path),witness=name==="Cargo.lock"?await observeCargoLockClosureV1(path,roots,options):name==="Cargo.toml"?await observeCargoManifestV1(path,options):undefined;
  if(witness)witnesses.push(witness);
  inputs.push({path,sha256:witness?witness.digest:await physical.digest(path)??null});
  await step(path);
 }
 return Object.freeze({inputs:Object.freeze(inputs),projected:Object.freeze(witnesses.map(witness=>witness.path)),recheck:async()=>{await physical.recheck();for(const witness of witnesses)await witness.recheck();}});
}
