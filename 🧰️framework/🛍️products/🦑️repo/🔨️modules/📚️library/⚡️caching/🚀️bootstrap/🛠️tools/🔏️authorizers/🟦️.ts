import {createHash} from "node:crypto";
import {lstat,readFile} from "node:fs/promises";
import {isBuiltin} from "node:module";
import {dirname,extname,isAbsolute,join,relative,resolve} from "node:path";

/** 🧬️ Exact runtime source names and bytes compiled into one release toolkit. */
export interface NxAuthorizerSourceManifestV1 {readonly version:1;readonly mode:"release";readonly entry:string;readonly sources:readonly {readonly path:string;readonly sha256:string}[];}
/** 🧊️ Frozen compiler inputs stay behind repository-owned values. */
export interface NxAuthorizerSourcesV1 {readonly manifest:NxAuthorizerSourceManifestV1;readonly digest:string;}
/** 📌️ The exact pinned tool intrinsics shared by runtime source capture and compilation. */
export const nxAuthorizerExternalModulesV1=Object.freeze(["nx/src/tasks-runner/create-task-graph","nx/src/hasher/task-hasher","nx/src/utils/workspace-context","nx/src/daemon/client/client","nx/src/native","nx/src/utils/consume-messages-from-socket","nx/src/daemon/socket-utils","nx/src/daemon/client/daemon-environment"]);
const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>();
/** 🧷️ Returns only the byte census issued by an actual no-follow Source capture. */
export function nxAuthorizerCapturedBytesV1(source:NxAuthorizerSourcesV1):ReadonlyMap<string,Uint8Array>{const bytes=capturedBytes.get(source);if(!bytes)throw Error("Original captured authorizer bytes required");return new Map([...bytes].map(([path,value])=>[path,new Uint8Array(value)]));}
const external=new Set(nxAuthorizerExternalModulesV1);
const hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const compare=(left:string,right:string)=>Buffer.compare(Buffer.from(left),Buffer.from(right));
function scannedImport(path:string):string{
 if(!/[\u0080-\u00ff]/u.test(path)||/[\u0100-\uffff]/u.test(path))return path;
 const decoded=Buffer.from(path,"latin1").toString("utf8");return Buffer.from(decoded,"utf8").toString("latin1")===path?decoded:path;
}
function sourcePath(root:string,path:string):string{
 if(!path||isAbsolute(path)||/[:\\\0]/u.test(path)||path.split("/").some(part=>!part||part==="."||part===".."))throw Error("Closed authorizer relative source path required");
 const absolute=join(root,path);if(absolute.length>256||[...absolute].length>256)throw Error("Authorizer materialized source path exceeds 256 characters");return absolute;
}
/** 🔒️ Reads actual regular source bytes without following substituted ancestry. */
export async function readNxAuthorizerSourceV1(root:string,path:string,signal:AbortSignal):Promise<Uint8Array>{
 signal.throwIfAborted();const absolute=sourcePath(root,path),parents=[root];let current=root;
 for(const part of path.split("/").slice(0,-1)){current=join(current,part);parents.push(current);}
 const ancestry=await Promise.all(parents.map(async path=>{const status=await lstat(path,{bigint:true});if(!status.isDirectory()||status.isSymbolicLink())throw Error("Authorizer source requires no-follow directories");return {path,device:status.dev,inode:status.ino};}));
 const before=await lstat(absolute,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.size>4_194_304n)throw Error("Authorizer requires bounded original regular source");
 const bytes=await readFile(absolute),after=await lstat(absolute,{bigint:true});signal.throwIfAborted();
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Authorizer source changed during capture");
 for(const row of ancestry){const status=await lstat(row.path,{bigint:true});if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==row.device||status.ino!==row.inode)throw Error("Authorizer source ancestry changed during capture");}
 signal.throwIfAborted();return bytes;
}
/** 📥️ Uses the system compiler's import scanner while refusing unresolved runtime dependencies. */
export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal):Promise<NxAuthorizerSourcesV1>{
 const root=resolve(workspace),files=new Map<string,string>(),contents=new Map<string,Uint8Array>(),pending=[entry];let total=0;
 while(pending.length){signal.throwIfAborted();const path=pending.pop()!;if(files.has(path))continue;const bytes=await readNxAuthorizerSourceV1(root,path,signal);files.set(path,hash(bytes));contents.set(path,new Uint8Array(bytes));total+=bytes.byteLength;if(files.size>256||total>16_777_216)throw Error("Authorizer source closure capacity exceeded");
  if(extname(path)===".json"){JSON.parse(Buffer.from(bytes).toString("utf8"));continue;}
  const source=Buffer.from(bytes).toString("utf8").replace(/^#![^\r\n]*/u,"");
  const scan=new Bun.Transpiler({loader:/\.[cm]?tsx?$/u.test(path)?"ts":"js",target:"node",define:{"import.meta.vitest":"undefined"}}).scan(source).imports;
  for(const row of scan){const specifier=scannedImport(row.path);if(specifier.startsWith("node:")||specifier==="bun:sqlite"||isBuiltin(specifier)||external.has(specifier))continue;if(!specifier.startsWith("."))throw Error("Authorizer runtime dependency refused: "+specifier);const child=relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\","/");sourcePath(root,child);pending.push(child);}
 }
 const manifest:NxAuthorizerSourceManifestV1=Object.freeze({version:1,mode:"release",entry,sources:Object.freeze([...files].sort(([left],[right])=>compare(left,right)).map(([path,sha256])=>Object.freeze({path,sha256})))});
 const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\n")});capturedBytes.set(result,contents);return result;
}
/** ✅️ Re-enumerates the actual runtime closure before publishing compiled authorizers. */
export async function nxAuthorizerSourcesHoldV1(root:string,original:NxAuthorizerSourcesV1,signal:AbortSignal):Promise<boolean>{
 try{return (await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal)).digest===original.digest;}catch(error){signal.throwIfAborted();return false;}
}
