import {createHash,randomBytes} from "node:crypto";
import * as nodeFs from "node:fs";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {sealEntriesV1,type PinnedRevisionV1} from "../📌️pin/🟦️.ts";
import {REVISION_POLICY_V1,revisionErrorV1,type InputManifestV1,type ManifestEntryV1,type RevisionV1} from "../🧬️schema/🟦️.ts";
import {importCapturedModuleV1} from "./🧩️module-loader.ts";
import type {ReadFsV1} from "./🟦️.ts";

/** 🧩️ The captured port with the loader of its plan: `importModule` loads a workspace module and its imports through the capture. */
export type CapturingFsV1=ReadFsV1&Readonly<{importModule(path:string):Promise<unknown>}>;

/** 📸️ The finite limits of one capture: files and probes held per plan, the largest file captured (a larger one is served live and listed as `unstaged`). */
export const CAPTURE_POLICY_V1=Object.freeze({maximumFiles:REVISION_POLICY_V1.maximumPaths,maximumProbes:4194304,maximumFileBytes:67108864});

const livePassThrough=(inner:string):boolean=>inner.split("/").some(part=>part==="node_modules"||part===".git"||part===".cache"||part==="target");
const failure=(code:string,path:string,message:string):Error=>Object.assign(new Error(message),{code,errno:-2,syscall:"stat",path});

/** 📸️ The result of a capture: the pin of the plan and what it saw. */
export type CapturedPlanV1<T>=Readonly<{result:T;pinned:PinnedRevisionV1;probes:number;unstaged:readonly string[]}>;

/**
 * 📸️ A read-through `node:fs` port for one plan: the first read of a workspace file stages its bytes into the host store and every later read of the plan answers the staged bytes, every stat, listing and realpath keeps the answer of its first probe (a missing path stays missing), so the plan sees one consistent snapshot whatever other agents edit meanwhile. Dependency, VCS, cache and build directories (`node_modules`, `.git`, `.cache`, `target`) and files above the byte limit are read live. `seal` turns exactly the files the plan read into a pinned revision.
 * @see ../🔓️view/🟦️.ts
 */
export function createCapturingFsV1(options:{cas:CasStoreV1;root:string;fallback?:ReadFsV1;passThrough?(relativePath:string):boolean;limits?:typeof CAPTURE_POLICY_V1}){
 const {cas}=options,live=options.fallback??nodeFs,root=resolve(options.root),limits=options.limits??CAPTURE_POLICY_V1,through=options.passThrough??livePassThrough;
 const files=new Map<string,ManifestEntryV1|{absent:string}>(),probes=new Map<string,{value:unknown}|{error:{code:string;message:string}}>(),unstaged=new Set<string>();
 const inside=(path:unknown):string|undefined=>{
  if(typeof path!=="string")return undefined;
  const inner=relative(root,resolve(isAbsolute(path)?path:join(process.cwd(),path)));
  if(inner==="")return "";
  if(inner.startsWith("..")||isAbsolute(inner))return undefined;
  const normal=inner.split(sep).join("/");
  return through(normal)?undefined:normal;
 };
 const stage=(bytes:Uint8Array):{hash:string;size:number}=>{
  const hash=createHash("sha256").update(bytes).digest("hex"),target=cas.objectPath(hash);
  if(!nodeFs.existsSync(target)){
   nodeFs.mkdirSync(dirname(target),{recursive:true});
   const temporary=join(dirname(target),`.capture-${process.pid}-${randomBytes(6).toString("hex")}`);
   nodeFs.writeFileSync(temporary,bytes,{mode:0o600});
   try{nodeFs.renameSync(temporary,target);}catch(error){nodeFs.rmSync(temporary,{force:true});if(!nodeFs.existsSync(target))throw error;}
   try{nodeFs.chmodSync(target,0o444);}catch{}
  }
  return{hash,size:bytes.byteLength};
 };
 const probe=<T>(key:string,path:string,compute:()=>T,copy:(value:T)=>T=value=>value):T=>{
  const known=probes.get(key);
  if(known!==undefined){if("error" in known)throw failure(known.error.code,path,known.error.message);return copy(known.value as T);}
  if(probes.size>=limits.maximumProbes)throw revisionErrorV1("too-many-paths",{count:probes.size,limit:limits.maximumProbes});
  try{const value=compute();probes.set(key,{value});return copy(value);}
  catch(error){const code=(error as {code?:string}).code;if(code==="ENOENT"||code==="ENOTDIR"){probes.set(key,{error:{code,message:(error as Error).message}});}throw error;}
 };
 const optionsKey=(value:unknown):string=>value===undefined?"":JSON.stringify(value);
 const port={
  statSync:((path:string,opts?:unknown)=>{const inner=inside(path);return inner===undefined?(live.statSync as any)(path,opts):probe("stat\0"+inner+"\0"+optionsKey(opts),path,()=>(live.statSync as any)(path,opts));}) as unknown as ReadFsV1["statSync"],
  lstatSync:((path:string,opts?:unknown)=>{const inner=inside(path);return inner===undefined?(live.lstatSync as any)(path,opts):probe("lstat\0"+inner+"\0"+optionsKey(opts),path,()=>(live.lstatSync as any)(path,opts));}) as unknown as ReadFsV1["lstatSync"],
  existsSync:((path:string)=>{const inner=inside(path);return inner===undefined?live.existsSync(path):probe("exists\0"+inner,path,()=>live.existsSync(path));}) as ReadFsV1["existsSync"],
  readdirSync:((path:string,opts?:unknown)=>{const inner=inside(path);return inner===undefined?(live.readdirSync as any)(path,opts):probe("readdir\0"+inner+"\0"+optionsKey(opts),path,()=>(live.readdirSync as any)(path,opts),value=>Array.isArray(value)?[...value] as any:value);}) as unknown as ReadFsV1["readdirSync"],
  readFileSync:((path:string,opts?:string|{encoding?:string|null}):unknown=>{
   const inner=inside(path);
   if(inner===undefined||inner==="")return (live.readFileSync as any)(path,opts);
   const encoding=typeof opts==="string"?opts:opts?.encoding;
   const give=(bytes:Buffer)=>encoding?bytes.toString(encoding as BufferEncoding):Buffer.from(bytes);
   const known=files.get(inner);
   if(known!==undefined){
    if("absent" in known)throw failure(known.absent,path,`${known.absent}: no such file or directory, open '${path}'`);
    return give(nodeFs.readFileSync(cas.objectPath(known.digest)));
   }
   let bytes:Buffer;
   try{bytes=live.readFileSync(path) as Buffer;}
   catch(error){const code=(error as {code?:string}).code;if(code==="ENOENT"||code==="ENOTDIR"){files.set(inner,{absent:code});}throw error;}
   if(bytes.byteLength>limits.maximumFileBytes){unstaged.add(inner);return give(bytes);}
   if(files.size>=limits.maximumFiles)throw revisionErrorV1("too-many-paths",{count:files.size,limit:limits.maximumFiles});
   let mode:ManifestEntryV1["mode"]="file";
   try{if(process.platform!=="win32"&&(Number(live.statSync(path).mode)&0o111)!==0)mode="executable";}catch{}
   const stored=stage(bytes);
   files.set(inner,{path:inner,mode,digest:stored.hash,size:stored.size});
   return give(bytes);
  }) as unknown as ReadFsV1["readFileSync"],
  realpathSync:undefined as unknown as ReadFsV1["realpathSync"],
 };
 const real=(path:string,opts?:unknown)=>{const inner=inside(path);return inner===undefined?(live.realpathSync as any)(path,opts):probe("real\0"+inner,path,()=>(live.realpathSync as any)(path,opts));};
 port.realpathSync=Object.assign(real,{native:real}) as unknown as ReadFsV1["realpathSync"];
 const capturing=Object.freeze({...port}) as ReadFsV1,reachable=(candidate:string):boolean=>{const inner=inside(candidate);return inner!==undefined&&inner!=="";};
 return Object.freeze({
  fs:Object.freeze({...port,importModule:(path:string):Promise<unknown>=>importCapturedModuleV1({port:capturing,captured:reachable},resolve(path))}) as CapturingFsV1,
  /** 📌️ Seals the files read so far into a pinned revision; the plan's reads and the manifest agree by construction. */
  async seal(seal:{workspace:string;holder:string;journalCursor:string;parent?:Readonly<{revision:RevisionV1;manifest:InputManifestV1}>;ttlMs?:number;now?():number;signal?:AbortSignal}):Promise<PinnedRevisionV1>{
   const entries=[...files.values()].filter((row):row is ManifestEntryV1=>"digest" in row),absent=[...files].filter(([,row])=>"absent" in row).map(([path])=>path);
   return sealEntriesV1({cas,entries,absent,...seal});
  },
  stats:()=>Object.freeze({files:files.size,probes:probes.size,unstaged:[...unstaged].sort()}),
 });
}
