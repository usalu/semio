/** 🏠 Where the one graph service of one identity lives: the sealed implementation digest, the checkout root and the machine state root. Two checkouts, two state roots or two code versions never share a service. */
import {createHash} from "node:crypto";
import {existsSync} from "node:fs";
import {tmpdir,userInfo} from "node:os";
import {dirname,join} from "node:path";
import {machineStateDirectoryV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {canonicalRootV1,worktreeIdV1} from "../🟨️.mjs";

/** 🔢 The workspace identity of a checkout: the twelve hex digits of its canonical root. */
export function workspaceIdV1(root){return worktreeIdV1(canonicalRootV1(root));}

/** 🚫 The longest socket path the platform accepts in `sockaddr_un` (104 on macOS, 108 on Linux, less the terminator and a margin). */
export const HOST_SOCKET_BYTES_V1=96;

/** 🧭 The checkout a directory belongs to: the nearest ancestor holding `.git` or `.🧬semio`, else the directory itself, in its canonical spelling. */
export function checkoutRootOfV1(directory=process.cwd()){
 const start=canonicalRootV1(directory);
 for(let at=start;;at=dirname(at)){
  if(existsSync(join(at,".git"))||existsSync(join(at,".🧬semio")))return at;
  if(dirname(at)===at)return start;
 }
}

/** 🛑 Raised when no runtime directory of this account can hold the socket of the service; carries both languages and the directory that was too long. */
export class HostRuntimePathErrorV1 extends Error{
 constructor(base,bytes){
  super("The graph service has no usable socket path below "+base+" ("+bytes+" bytes, at most "+HOST_SOCKET_BYTES_V1+"): set XDG_RUNTIME_DIR or TMPDIR to a shorter directory of your own / Der Graphdienst hat unter "+base+" keinen nutzbaren Socket-Pfad ("+bytes+" Bytes, höchstens "+HOST_SOCKET_BYTES_V1+"): XDG_RUNTIME_DIR oder TMPDIR auf ein kürzeres eigenes Verzeichnis setzen");
  this.name="HostRuntimePathErrorV1";this.code="runtime-path";this.base=base;this.bytes=bytes;
 }
}

/** 📍 Runtime directory, socket or pipe, key, descriptor and start lock of the host service of one identity (implementation digest, checkout root, machine state root); the socket path is derived below the per-user runtime base in at most two spellings and never moves to a shared folder: when neither fits it raises `HostRuntimePathErrorV1`. See https://pkg.go.dev/os#TempDir */
export function hostEndpointsV1(implementation,environment=process.env,platform=process.platform,user=undefined,root=undefined){
 const owner=user??(()=>{try{return String(userInfo().uid);}catch{return"0";}})(),checkout=checkoutRootOfV1(root),state=machineStateDirectoryV1(environment)??"",tag=createHash("sha256").update("semio-repo-host-v2\0"+implementation+"\0"+checkout+"\0"+state).digest("hex").slice(0,12),identity={implementation,root:checkout,state};
 if(platform==="win32"){
  const directory=join(environment.LOCALAPPDATA||tmpdir(),"semio","repo","host");
  return{tag,identity,directory,endpoint:"\\\\.\\pipe\\semio-repo-host-"+owner+"-"+tag,key:join(directory,"key"),descriptor:join(directory,"host-"+tag+".json"),lock:join(directory,"start-"+tag+".lock"),log:join(directory,"host-"+tag+".log")};
 }
 const base=environment.XDG_RUNTIME_DIR||(environment.TMPDIR||"/tmp").replace(/\/+$/u,""),socket="h-"+tag+".sock";
 const directory=[join(base,"semio-repo",owner),join(base,"sr"+owner)].find(candidate=>Buffer.byteLength(join(candidate,socket))<=HOST_SOCKET_BYTES_V1);
 if(directory===undefined)throw new HostRuntimePathErrorV1(base,Buffer.byteLength(join(base,"sr"+owner,socket)));
 return{tag,identity,directory,endpoint:join(directory,socket),key:join(directory,"key"),descriptor:join(directory,"host-"+tag+".json"),lock:join(directory,"start-"+tag+".lock"),log:join(directory,"host-"+tag+".log")};
}
