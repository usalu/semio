import {createHash,randomBytes} from "node:crypto";
import {constants,createReadStream} from "node:fs";
import {chmod,copyFile,lstat,mkdir,readFile,readdir,readlink,rename,rm,stat,symlink,utimes,writeFile} from "node:fs/promises";
import {readFileSync,statfsSync} from "node:fs";
import {basename,dirname,join,relative,resolve,sep} from "node:path";
import {churnLanguagesV1} from "../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {observeStableTreeV1,type StableTreeOptionsV1} from "../../../📁️filesystem/🧾️observation/🔁️churn/📸️snapshot/🟦️.ts";
import {machinePathV1} from "../🏠️machine-state/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Protocol.const)) as Readonly<{version:1;directory:string;idCharacters:number;maximumRootCharacters:number;maximumFiles:number;retained:number;minimumFreeBytes:number;holdersName:string;stepPrefix:string;closureRootEntries:readonly string[];maximumTotalBytes:number;receiptName:string;rowsName:string;progressEvery:number;hashInMemoryBytes:number;environment:Readonly<{root:string;state:string;digest:string;origin:string}>;excludeRootEntries:readonly string[];excludeEverywhere:readonly string[];linkEverywhere:readonly string[];texts:Readonly<Record<string,Readonly<Record<"created"|"reused"|"progress"|"pruned"|"tooLong"|"notSnapshot",string>>>>}>);

/** 📸️ The closed numbers, names and EN/DE texts of the checkout snapshot, exactly as the schema declares them.
 * @see ./🧬️schema/🔣️.json */
export function checkoutSnapshotProtocolV1():typeof protocol{return protocol;}

/** 🧾️ One captured entry: a file with the SHA-256 of its bytes, or a symbolic link with its literal target. Paths are relative to the workspace and use `/`. */
export type CheckoutRowV1=Readonly<{kind:"file";path:string;sha256:string;bytes:number;executable:boolean}|{kind:"link";path:string;target:string}>;
/** 📸️ One immutable source generation: where it lives, which live workspace it copies, the digest of its rows and what the capture cost. */
export type CheckoutSnapshotV1=Readonly<{version:1;origin:string;state:string;root:string;id:string;digest:string;files:number;links:number;bytes:number;reused:boolean;attempts:number;rows:readonly CheckoutRowV1[]}>;
/** 📣️ One progress report of a capture. */
export type CheckoutProgressV1=Readonly<{phase:"copy"|"seal";files:number;bytes:number;text:string}>;
/** 🎛️ The ports of one capture: the live workspace, the state directory (default: the machine state of the workspace), an allow-list of root entries, the progress report, the cancellation signal and the options of the stable tree observation. `afterCopy` is a test seam called after each consumed file. */
/** 🧹️ The retention a capture applies before and after it; every field defaults to the protocol. */
export type CheckoutRetentionV1=Readonly<{retained?:number;maximumTotalBytes?:number;minimumFreeBytes?:number;free?:(path:string)=>number}>;
/** 🎛️ The ports of one capture (see above); `retention` overrides the pruning numbers, `step` names the queue step whose attempts share one snapshot and `fresh` demands a new capture for it. */
export type CheckoutSnapshotRequestV1=Readonly<{retention?:CheckoutRetentionV1;workspace:string;state?:string;entries?:readonly string[];signal?:AbortSignal;onProgress?:(event:CheckoutProgressV1)=>void;onEvent?:(text:string)=>void;environment?:Readonly<Record<string,string|undefined>>;observation?:StableTreeOptionsV1;readOnly?:boolean;step?:string;owner?:number;fresh?:boolean;now?:()=>number;afterCopy?:(path:string)=>void|Promise<void>}>;

type ConsumedV1=Readonly<{kind:"file";sha256:string;bytes:number;executable:boolean}|{kind:"link";target:string}>;

function alive(pid:number):boolean{
 if(!Number.isSafeInteger(pid)||pid<=0)return false;
 try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}
}

async function recordedStepV1(record:string,owner:number):Promise<string>{
 try{
  const row=JSON.parse(await readFile(record,"utf8")) as {id?:unknown;owner?:unknown};
  if(row.owner===owner&&typeof row.id==="string"&&alive(owner))return row.id;
 }catch{}
 await rm(record,{force:true});
 return "";
}

const posix=(path:string):string=>path.split(sep).join("/");
const byBytes=(left:CheckoutRowV1,right:CheckoutRowV1):number=>Buffer.compare(Buffer.from(left.path,"utf8"),Buffer.from(right.path,"utf8"));
const sha256Of=(text:string|Uint8Array):string=>createHash("sha256").update(text).digest("hex");

function say(key:keyof typeof protocol.texts[string],values:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string{
 return churnLanguagesV1(environment).map(language=>protocol.texts[language]![key].replace(/\{([a-z]+)\}/gu,(_,name:string)=>String(values[name]))).join("\n");
}

/** 📜️ The canonical text of rows: one line per entry sorted by the UTF-8 bytes of its path, `f<TAB>path<TAB>sha256<TAB>x|-` for a file and `l<TAB>path<TAB>target` for a link. Any language reproduces it from the tree.
 * @see ./🧪️tests/🧫️fixtures/🔣️.json */
export function checkoutRowsTextV1(rows:readonly CheckoutRowV1[]):string{
 return [...rows].sort(byBytes).map(row=>{
  if(/[\t\n]/u.test(row.path))throw Error("Snapshot paths must not contain tabs or line breaks: "+JSON.stringify(row.path));
  return row.kind==="file"?"f\t"+row.path+"\t"+row.sha256+"\t"+(row.executable?"x":"-")+"\n":"l\t"+row.path+"\t"+row.target+"\n";
 }).join("");
}

/** 🔏️ The digest of rows: the SHA-256 of their canonical text. @see https://csrc.nist.gov/pubs/fips/180-4/upd1/final */
export const checkoutRowsDigestV1=(rows:readonly CheckoutRowV1[]):string=>sha256Of(checkoutRowsTextV1(rows));

/** 🆔️ The directory name of a snapshot: the leading characters of its digest. */
export const checkoutSnapshotIdV1=(digest:string):string=>digest.slice(0,protocol.idCharacters);

/** 🗂️ The directory below which snapshots of `workspace` live: `<machine state>/s`, so a short root that the replica never copies. */
export function checkoutSnapshotBaseV1(workspace:string,state?:string):string{return join(state??machinePathV1(resolve(workspace),".cache"),protocol.directory);}

async function hashFileV1(path:string,bytes:number):Promise<string>{
 if(bytes<=protocol.hashInMemoryBytes)return sha256Of(await readFile(path));
 const hash=createHash("sha256");
 for await(const chunk of createReadStream(path))hash.update(chunk);
 return hash.digest("hex");
}

async function linkTypeV1(target:string,from:string):Promise<"junction"|"file"|undefined>{
 if(process.platform!=="win32")return undefined;
 return(await stat(resolve(dirname(from),target)).catch(()=>undefined))?.isDirectory()?"junction":"file";
}

async function removeTreeV1(path:string):Promise<void>{
 try{await rm(path,{recursive:true,force:true,maxRetries:3});return;}catch{}
 const open=async(directory:string):Promise<void>=>{
  await chmod(directory,0o700).catch(()=>{});
  for(const entry of await readdir(directory,{withFileTypes:true}).catch(()=>[])){
   const child=join(directory,entry.name);
   if(entry.isDirectory())await open(child);else await chmod(child,0o600).catch(()=>{});
  }
 };
 await open(path);
 await rm(path,{recursive:true,force:true,maxRetries:3});
}

async function pool<T>(items:readonly T[],width:number,body:(item:T)=>Promise<void>):Promise<void>{
 let next=0;
 await Promise.all(Array.from({length:Math.min(width,items.length)},async()=>{for(;next<items.length;){const item=items[next++]!;await body(item);}}));
}

async function validReceiptV1(root:string,digest:string):Promise<boolean>{
 try{const receipt=JSON.parse(await readFile(join(root,protocol.receiptName),"utf8")) as {digest?:string};return receipt.digest===digest;}catch{return false;}
}

/** 📸️ Captures the live `workspace` as ONE stable generation into `<state>/s/<id>` and returns it. Every file is cloned (copy-on-write where the filesystem offers it, a plain copy elsewhere) and hashed from the copy exactly once; two listings of the live tree must agree to the byte of every stamp and an entry edited within the settle window is waited out, otherwise the churn policy of the `snapshot` layer repeats only what changed. The directory is named after the digest of its rows, so an unchanged tree reuses the earlier snapshot; files are made read-only; `node_modules` links are kept as links (their target stays the live installed packages) and `.git`, build outputs and the machine state are left out. Concurrent captures of one digest race by atomic rename and the loser reuses the winner's directory.
 * @see https://nodejs.org/api/fs.html#fscopyfilesrc-dest-mode
 * @see ../../../📁️filesystem/🧾️observation/🔁️churn/📸️snapshot/🟦️.ts */
export async function captureCheckoutSnapshotV1(request:CheckoutSnapshotRequestV1):Promise<CheckoutSnapshotV1>{
 const origin=resolve(request.workspace),state=request.state?resolve(request.state):machinePathV1(origin,".cache"),base=join(state,protocol.directory),environment=request.environment??process.env,clock=request.now??Date.now,started=clock();
 const allow=new Set(request.entries??protocol.closureRootEntries),linked=new Set<string>(),stage=join(base,".t"+process.pid+"-"+randomBytes(4).toString("hex"));
 const longest=join(base,"0".repeat(protocol.idCharacters));
 if(longest.length>protocol.maximumRootCharacters)throw Error(say("tooLong",{root:longest,length:longest.length,maximum:protocol.maximumRootCharacters},environment));
 const include=(path:string):boolean=>{
  const name=basename(path);
  if(path===base||path.startsWith(base+sep))return false;
  if(protocol.linkEverywhere.includes(name)){linked.add(path);return false;}
  if(protocol.excludeEverywhere.includes(name))return false;
  if(dirname(path)===origin&&(protocol.excludeRootEntries.includes(name)||!allow.has(name)))return false;
  return true;
 };
 await mkdir(base,{recursive:true});
 const record=request.step===undefined||request.owner===undefined||!alive(request.owner)?undefined:join(base,protocol.stepPrefix+request.step.replace(/[^A-Za-z0-9._-]/gu,"_"));
 if(record!==undefined&&request.fresh!==true){
  const id=await recordedStepV1(record,request.owner!),root=join(base,id),receipt=id===""?undefined:JSON.parse(await readFile(join(root,protocol.receiptName),"utf8").catch(()=>"null")) as {origin?:string;digest?:string;files:number;links:number;bytes:number}|null|undefined;
  if(receipt&&receipt.origin===origin&&typeof receipt.digest==="string"&&checkoutSnapshotIdV1(receipt.digest)===id){
   await holdCheckoutSnapshotV1(origin,id,state);
   const now=new Date(clock());await utimes(join(root,protocol.receiptName),now,now).catch(()=>{});
   const reusedSnapshot:CheckoutSnapshotV1=Object.freeze({version:1,origin,state,root,id,digest:receipt.digest,files:receipt.files,links:receipt.links,bytes:receipt.bytes,reused:true,attempts:0,rows:Object.freeze([])});
   request.onEvent?.(say("reused",{id,files:receipt.files,root},environment));
   return reusedSnapshot;
  }
 }
 await pruneCheckoutSnapshotsV1({...request.retention,workspace:origin,state,environment,...request.onEvent?{onEvent:request.onEvent}:{}}).catch(()=>[]);
 await mkdir(stage,{recursive:true});
 let files=0,bytes=0;
 const progress=(phase:"copy"|"seal"):void=>{
  const text=say("progress",{phase,files,mebibytes:Math.round(bytes/1048576)},environment);
  request.onProgress?.(Object.freeze({phase,files,bytes,text}));
 };
 try{
  const roots=(await readdir(origin)).sort().map(name=>join(origin,name)).filter(include);
  const consume=async(path:string):Promise<ConsumedV1>=>{
   request.signal?.throwIfAborted();
   const destination=join(stage,relative(origin,path)),row=await lstat(path);
   await mkdir(dirname(destination),{recursive:true});
   await rm(destination,{force:true});
   let value:ConsumedV1;
   if(row.isSymbolicLink()){
    const target=await readlink(path);
    await symlink(target,destination,await linkTypeV1(target,destination));
    value={kind:"link",target};
   }else{
    await copyFile(path,destination,constants.COPYFILE_FICLONE);
    const copied=await lstat(destination);
    value={kind:"file",sha256:await hashFileV1(destination,copied.size),bytes:copied.size,executable:process.platform!=="win32"&&(row.mode&0o111)!==0};
    bytes+=copied.size;
   }
   if(++files%protocol.progressEvery===0)progress("copy");
   await request.afterCopy?.(path);
   return value;
  };
  const seen=await observeStableTreeV1(roots,consume,{...request.observation,...request.signal?{signal:request.signal}:{},environment,include});
  if(seen.files.size>protocol.maximumFiles)throw Error("Bounded checkout snapshot file count required");
  const rows:CheckoutRowV1[]=[...seen.files].map(([path,value])=>({...value,path:posix(relative(origin,path))}) as CheckoutRowV1);
  for(const directory of seen.directories)await mkdir(join(stage,relative(origin,directory)),{recursive:true});
  for(const path of [...linked].sort()){
   const info=await lstat(path).catch(()=>undefined);
   if(!info)continue;
   const target=info.isSymbolicLink()?await readlink(path):path,destination=join(stage,relative(origin,path));
   await mkdir(dirname(destination),{recursive:true});
   await rm(destination,{force:true});
   await symlink(target,destination,await linkTypeV1(target,destination));
   rows.push({kind:"link",path:posix(relative(origin,path)),target});
  }
  if(seen.attempts>1||files>seen.files.size){
   const known=new Set(rows.map(row=>row.path)),sweep=async(directory:string):Promise<void>=>{
    for(const entry of await readdir(directory,{withFileTypes:true})){
     const path=join(directory,entry.name);
     if(entry.isDirectory())await sweep(path);
     else if(!known.has(posix(relative(stage,path))))await rm(path,{force:true});
    }
   };
   await sweep(stage);
  }
  rows.sort(byBytes);
  progress("seal");
  const text=checkoutRowsTextV1(rows),digest=sha256Of(text),id=checkoutSnapshotIdV1(digest),root=join(base,id);
  await holdCheckoutSnapshotV1(origin,id,state);
  const fileRows=rows.filter(row=>row.kind==="file") as Extract<CheckoutRowV1,{kind:"file"}>[],total=fileRows.reduce((sum,row)=>sum+row.bytes,0);
  const receipt={version:1,origin,digest,files:fileRows.length,links:rows.length-fileRows.length,bytes:total};
  let reused=false;
  if(await validReceiptV1(root,digest))reused=true;
  else{
   await writeFile(join(stage,protocol.rowsName),text);
   await writeFile(join(stage,protocol.receiptName),JSON.stringify(receipt)+"\n");
   if(request.readOnly!==false)await pool(fileRows,64,row=>chmod(join(stage,...row.path.split("/")),row.executable?0o555:0o444));
   try{await rename(stage,root);}
   catch(error){if(!await validReceiptV1(root,digest))throw error;reused=true;}
  }
  if(reused){const now=new Date(clock());await utimes(join(root,protocol.receiptName),now,now).catch(()=>{});}
  if(record!==undefined)await writeFile(record,JSON.stringify({id,owner:request.owner}));
  const snapshot:CheckoutSnapshotV1=Object.freeze({version:1,origin,state,root,id,digest,files:fileRows.length,links:rows.length-fileRows.length,bytes:total,reused,attempts:seen.attempts,rows:Object.freeze(rows)});
  request.onEvent?.(say(reused?"reused":"created",{id,files:snapshot.files,mebibytes:Math.round(total/1048576),seconds:Math.round((clock()-started)/1000),attempts:seen.attempts,root},environment));
  await pruneCheckoutSnapshotsV1({...request.retention,workspace:origin,state,keep:[id],environment,...request.onEvent?{onEvent:request.onEvent}:{}}).catch(()=>[]);
  return snapshot;
 }finally{
  await removeTreeV1(stage).catch(()=>{});
 }
}

/** 🌍️ The environment under which a build sees the snapshot as its workspace root and the live machine state as its state: the declared state directory is the live `.cache`, so unit caches, leases and published modules stay shared. */
export function checkoutSnapshotEnvironmentV1(snapshot:Pick<CheckoutSnapshotV1,"root"|"state"|"digest"|"origin">):Readonly<Record<string,string>>{
 const names=protocol.environment;
 return Object.freeze({[names.root]:snapshot.root,[names.state]:snapshot.state,[names.digest]:snapshot.digest,[names.origin]:snapshot.origin});
}

/** 🪞️ Rewrites the snapshot root in diagnostics back to the live workspace, so messages and logs name the paths a developer edits. */
export function checkoutSnapshotLiveTextV1(text:string,snapshot:Pick<CheckoutSnapshotV1,"root"|"origin">):string{
 let result=text;
 for(const form of new Set([snapshot.root,snapshot.root.replaceAll("\\","/"),snapshot.root.replaceAll("\\","\\\\")]))result=result.split(form).join(snapshot.origin);
 return result;
}

/** 📖️ Reads the rows a snapshot recorded when it was captured, as `path` to `sha256` for files. */
export async function readCheckoutSnapshotRowsV1(root:string):Promise<ReadonlyMap<string,string>>{
 const rows=new Map<string,string>();
 for(const line of (await readFile(join(root,protocol.rowsName),"utf8")).split("\n")){
  const parts=line.split("\t");
  if(parts[0]==="f"&&parts[1]!==undefined&&parts[2]!==undefined)rows.set(parts[1],parts[2]);
 }
 return rows;
}

/** 🧪️ Whether every `[path, sha256]` pair (the sources a compilation consumed, relative to the workspace root as the receipts name them) is exactly what the snapshot holds: the proof that a receipt describes the compiled generation. */
export async function checkoutSnapshotProvesV1(root:string,sources:Iterable<readonly [string,string]>):Promise<Readonly<{proved:number;missing:readonly string[];differing:readonly string[]}>>{
 const rows=await readCheckoutSnapshotRowsV1(root),missing:string[]=[],differing:string[]=[];
 let proved=0;
 for(const [path,sha256] of sources){const held=rows.get(path);if(held===undefined)missing.push(path);else if(held!==sha256)differing.push(path);else proved++;}
 return Object.freeze({proved,missing:Object.freeze(missing),differing:Object.freeze(differing)});
}

/** 🔬️ Hashes the files on disk again and compares them with the digest the snapshot was captured with: `changed` names every recorded file whose bytes differ or that is gone, `extra` every file that was added since (a build output is not a source change). */
export async function verifyCheckoutSnapshotV1(root:string):Promise<Readonly<{ok:boolean;digest:string;changed:readonly string[];extra:readonly string[]}>>{
 const receipt=JSON.parse(await readFile(join(root,protocol.receiptName),"utf8")) as {digest:string},rows=await readCheckoutSnapshotRowsV1(root),changed:string[]=[],extra:string[]=[],seen=new Set<string>();
 const walk=async(directory:string):Promise<void>=>{
  for(const entry of await readdir(directory,{withFileTypes:true})){
   const path=join(directory,entry.name),name=posix(relative(root,path));
   if(name===protocol.receiptName||name===protocol.rowsName)continue;
   if(entry.isDirectory())await walk(path);
   else if(entry.isSymbolicLink())continue;
   else{seen.add(name);const held=rows.get(name);if(held===undefined)extra.push(name);else if(held!==await hashFileV1(path,(await lstat(path)).size))changed.push(name);}
  }
 };
 await walk(root);
 for(const name of rows.keys())if(!seen.has(name))changed.push(name);
 const recorded=sha256Of(await readFile(join(root,protocol.rowsName)));
 return Object.freeze({ok:recorded===receipt.digest&&changed.length===0,digest:recorded,changed:Object.freeze(changed.sort()),extra:Object.freeze(extra.sort())});
}

/** 🔒️ Registers the calling process as a holder of snapshot `id`: `<base>/.holders/<id>/<pid>`. A snapshot with a live holder is never removed; a holder is live while its process exists. */
export async function holdCheckoutSnapshotV1(workspace:string,id:string,state?:string,pid:number=process.pid):Promise<void>{
 const directory=join(checkoutSnapshotBaseV1(workspace,state),protocol.holdersName,id);
 await mkdir(directory,{recursive:true});
 await writeFile(join(directory,String(pid)),"");
}


async function liveHoldersV1(base:string,id:string):Promise<number>{
 const directory=join(base,protocol.holdersName,id);let live=0;
 for(const name of await readdir(directory).catch(()=>[])){
  const pid=Number(name);
  if(Number.isSafeInteger(pid)&&pid>0&&alive(pid))live++;else await rm(join(directory,name),{force:true});
 }
 return live;
}

/** 🧹️ Removes snapshots, oldest use first, while there are more than `retained`, their recorded bytes exceed `maximumTotalBytes` or the free space of the volume is below `minimumFreeBytes`. Only a snapshot a live process holds (see {@link holdCheckoutSnapshotV1}) or one named in `keep` is protected: age protects nothing. Abandoned stage directories of dead captures are removed too. Returns the ids removed. */
export async function pruneCheckoutSnapshotsV1(request:Readonly<{workspace:string;state?:string;keep?:readonly string[];retained?:number;maximumTotalBytes?:number;minimumFreeBytes?:number;free?:(path:string)=>number;onEvent?:(text:string)=>void;environment?:Readonly<Record<string,string|undefined>>}>):Promise<readonly string[]>{
 const base=checkoutSnapshotBaseV1(request.workspace,request.state),environment=request.environment??process.env,kept=new Set(request.keep??[]),rows:{id:string;used:number;bytes:number;held:boolean}[]=[];
 const free=request.free??((path:string):number=>{const row=statfsSync(path);return Number(row.bavail)*Number(row.bsize);});
 for(const entry of await readdir(base,{withFileTypes:true}).catch(()=>[])){
  if(entry.isFile()&&entry.name.startsWith(protocol.stepPrefix)){
   const row=(()=>{try{return JSON.parse(readFileSync(join(base,entry.name),"utf8")) as {owner?:unknown};}catch{return {} as {owner?:unknown};}})();
   if(typeof row.owner!=="number"||!alive(row.owner))await rm(join(base,entry.name),{force:true});
   continue;
  }
  if(!entry.isDirectory())continue;
  if(entry.name.startsWith(".t")){
   const owner=/^\.t(\d+)-/u.exec(entry.name);
   if(owner===null||!alive(Number(owner[1])))await removeTreeV1(join(base,entry.name)).catch(()=>{});
   continue;
  }
  if(entry.name.startsWith("."))continue;
  const receiptPath=join(base,entry.name,protocol.receiptName),info=await lstat(receiptPath).catch(()=>undefined),receipt=JSON.parse(await readFile(receiptPath,"utf8").catch(()=>"{}")) as {bytes?:number};
  rows.push({id:entry.name,used:info===undefined?0:info.mtimeMs,bytes:receipt.bytes??0,held:kept.has(entry.name)||await liveHoldersV1(base,entry.name)>0});
 }
 rows.sort((left,right)=>left.used-right.used||(left.id<right.id?-1:1));
 const retained=request.retained??protocol.retained,budget=request.maximumTotalBytes??protocol.maximumTotalBytes,floor=request.minimumFreeBytes??protocol.minimumFreeBytes,removed:string[]=[];
 let count=rows.length,total=rows.reduce((sum,row)=>sum+row.bytes,0);
 const measure=():number=>{try{return free(base);}catch{return Number.POSITIVE_INFINITY;}};
 for(const row of rows){
  if(row.held)continue;
  if(!(count>retained||total>budget||measure()<floor))break;
  await removeTreeV1(join(base,row.id));
  await rm(join(base,protocol.holdersName,row.id),{recursive:true,force:true});
  count--;total-=row.bytes;removed.push(row.id);
  request.onEvent?.(say("pruned",{id:row.id},environment));
 }
 return Object.freeze(removed);
}

/** 🛂️ Requires `root` to be a checkout snapshot of the live workspace `origin` (its receipt names that origin and the digest of its rows) and returns its receipt; a build refuses any other directory as a source root.
 * @see ./🧬️schema/🔣️.json */
export async function requireCheckoutSnapshotRootV1(root:string,origin:string,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<Readonly<{digest:string;files:number;bytes:number}>>{
 try{
  const receipt=JSON.parse(await readFile(join(root,protocol.receiptName),"utf8")) as {origin?:string;digest?:string;files:number;bytes:number},digest=sha256Of(await readFile(join(root,protocol.rowsName)));
  if(receipt.origin===resolve(origin)&&receipt.digest===digest&&basename(root)===checkoutSnapshotIdV1(digest))return Object.freeze({digest,files:receipt.files,bytes:receipt.bytes});
 }catch{}
 throw Error(say("notSnapshot",{root,origin},environment));
}

/** 🔁️ Rebases an absolute path of the live workspace onto the snapshot root; any other path is returned unchanged. */
export function checkoutSnapshotPathV1(snapshot:Pick<CheckoutSnapshotV1,"root"|"origin">,path:string):string{
 const distance=relative(snapshot.origin,path);
 return distance===""?snapshot.root:distance.startsWith("..")||distance.split(sep)[0]===".."||resolve(distance)===distance?path:join(snapshot.root,distance);
}

/** 🔙️ The inverse of {@link checkoutSnapshotPathV1}: a path below the snapshot root is returned below the live workspace. */
export function checkoutSnapshotOriginPathV1(snapshot:Pick<CheckoutSnapshotV1,"root"|"origin">,path:string):string{
 const distance=relative(snapshot.root,path);
 return distance===""?snapshot.origin:distance.startsWith("..")||resolve(distance)===distance?path:join(snapshot.origin,distance);
}


/** 🪪️ Whether `root` is a checkout snapshot directory of the live workspace `origin` (`<state>/s/<id>`), judged by its location alone, so a receipt written from a snapshot that was pruned since still names a genuine generation of that workspace. */
export function checkoutSnapshotOfV1(root:string,origin:string,state?:string):boolean{
 const directory=resolve(root);
 return dirname(directory)===checkoutSnapshotBaseV1(origin,state)&&new RegExp("^[0-9a-f]{"+protocol.idCharacters+"}$","u").test(basename(directory));
}

/** 🔚️ Forgets the snapshot a named step reuses, so the next capture of that step reads the live tree again. */
export async function forgetCheckoutSnapshotStepV1(workspace:string,step:string,state?:string):Promise<void>{
 await rm(join(checkoutSnapshotBaseV1(workspace,state),protocol.stepPrefix+step.replace(/[^A-Za-z0-9._-]/gu,"_")),{force:true});
}

export {checkoutSnapshotRelativeV1,checkoutSnapshotRootOfV1} from "./🧭️place/🟦️.ts";
