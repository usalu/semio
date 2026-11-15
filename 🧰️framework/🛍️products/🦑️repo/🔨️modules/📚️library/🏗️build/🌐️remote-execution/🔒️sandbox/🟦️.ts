import {spawnSync} from "node:child_process";
import {accessSync,constants,lstatSync,readlinkSync,realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {HERMETIC_TABLE_V1} from "../../🏃️execution/🔒️hermetic/🟦️.ts";
import type {RemoteExecutionSandboxV1} from "../🟦️.ts";

/** 🧱️ What a sandbox must confine: the command, its exact environment, where it runs, the one tree it may write (the private input root with its scratch directory) and whether it may use the network. */
export type RemoteExecutionSandboxSpecV1=Readonly<{argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string;root:string;network:boolean;readable?:readonly string[]}>;

/** 🔒️ The command line a sandbox turns a spec into. */
export type RemoteExecutionSandboxedCommandV1=Readonly<{argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string}>;

/** 🛡️ One confinement strategy of the hermetic table, with the same names the table uses for builds. */
export interface RemoteExecutionSandboxStrategyV1{
 readonly name:RemoteExecutionSandboxV1;
 available():boolean;
 wrap(spec:RemoteExecutionSandboxSpecV1):RemoteExecutionSandboxedCommandV1;
}

const DARWIN_HIDDEN=["/Users","/Volumes","/private/tmp","/private/var/folders","/private/var/tmp","/private/etc/semio","/private/var/lib","/private/var/root","/private/var/db","/private/var/at","/private/etc/ssh","/Library/Keychains","/System/Volumes/Data/Users","/System/Volumes/Data/Volumes","/System/Volumes/Data/private/etc/semio","/System/Volumes/Data/private/var/lib","/System/Volumes/Data/private/var/root","/System/Volumes/Data/private/var/db"];
const LINUX_DIRECTORIES=["/usr","/bin","/sbin","/lib","/lib32","/lib64","/libx32","/etc/alternatives","/etc/ld.so.conf.d"],LINUX_FILES=["/etc/ld.so.cache","/etc/ld.so.conf","/etc/passwd","/etc/group"];

/** 🗺️ What the host has at one path: a directory, a file, a symbolic link (with its target) or nothing. */
export type RemoteExecutionHostEntryV1=Readonly<{kind:"directory"|"file"}|{kind:"symlink";target:string}>|undefined;

const hostEntry=(path:string):RemoteExecutionHostEntryV1=>{try{const info=lstatSync(path);return info.isSymbolicLink()?{kind:"symlink",target:readlinkSync(path)}:{kind:info.isDirectory()?"directory":"file"};}catch{return undefined;}};

const quote=(value:string):string=>`"${value.replace(/\\/gu,"\\\\").replace(/"/gu,'\\"')}"`;

/** 📜️ The macOS sandbox profile: everything stays allowed except network access (unless asked for), writes outside the input root and the system temporary locations, and reading file contents under the home directories and volumes except the input root. */
export function remoteExecutionDarwinProfileV1(root:string,network:boolean,readable:readonly string[]=[]):string{
 return [
  "(version 1)","(allow default)",
  ...(network?[]:["(deny network*)"]),
  "(deny file-write*)",
  `(allow file-write* (subpath ${quote(root)}) (subpath "/dev") (literal "/private/var/run/utmpx"))`,
  `(deny file-read-data ${DARWIN_HIDDEN.map(path=>`(subpath "${path}")`).join(" ")})`,
  `(allow file-read-data (subpath ${quote(root)}))`,
  ...readable.map(path=>`(allow file-read-data (subpath ${quote(path)}))`),
 ].join("\n");
}

/** 🧊️ The bubblewrap command line of the Linux sandbox: NOT the host. Only the system directories and libraries a toolchain needs (read-only, symbolic links such as `/bin` kept as links), the directories the action's tools declare readable, a private `/tmp`, fresh devices and process table, the input root writable, no network unless asked for, and everything dies with the parent. Home directories, `~/.ssh`, the worktree, the store and every credential file simply do not exist inside. */
export function remoteExecutionBubblewrapArgumentsV1(spec:RemoteExecutionSandboxSpecV1,host:(path:string)=>RemoteExecutionHostEntryV1=hostEntry):string[]{
 const system=[...LINUX_DIRECTORIES,...LINUX_FILES].flatMap(path=>{const entry=host(path);return entry===undefined?[]:entry.kind==="symlink"?["--symlink",entry.target,path]:["--ro-bind",path,path];});
 return ["bwrap",...(spec.network?[]:["--unshare-net"]),"--unshare-pid","--unshare-ipc","--die-with-parent","--new-session",...system,"--dev","/dev","--proc","/proc","--tmpfs","/tmp",...(spec.readable??[]).flatMap(path=>["--ro-bind",path,path]),"--bind",spec.root,spec.root,"--chdir",spec.cwd,"--clearenv",...Object.entries(spec.env).flatMap(([name,value])=>["--setenv",name,value]),"--",...spec.argv];
}

/** 🩺️ The command line that proves a Linux host can run the real sandbox: exactly the namespaces, devices and `/proc` mount of {@link remoteExecutionBubblewrapArgumentsV1} around `/bin/true`, so a container that allows a bare `bwrap --ro-bind / /` but refuses a new `/proc` or process namespace is not mistaken for a host that confines (the worker then takes the private root and says so). */
export function remoteExecutionBubblewrapProbeArgumentsV1(root:string):string[]{
 return remoteExecutionBubblewrapArgumentsV1({argv:["/bin/true"],env:{},cwd:root,root,network:false}).slice(1);
}

const probe=(command:string,args:readonly string[]):boolean=>{
 try{return spawnSync(command,args,{stdio:"ignore",timeout:10000}).status===0;}catch{return false;}
};
const executable=(path:string):boolean=>{try{accessSync(path,constants.X_OK);return true;}catch{return false;}};

const STRATEGIES:Readonly<Record<RemoteExecutionSandboxV1,RemoteExecutionSandboxStrategyV1>>=Object.freeze({
 "darwin-sandbox":{
  name:"darwin-sandbox",
  available:()=>process.platform==="darwin"&&executable("/usr/bin/sandbox-exec")&&probe("/usr/bin/sandbox-exec",["-p","(version 1)(allow default)","/usr/bin/true"]),
  wrap:spec=>({argv:["/usr/bin/sandbox-exec","-p",remoteExecutionDarwinProfileV1(realpathSync(spec.root),spec.network,(spec.readable??[]).map(path=>realpathSync(path))),...spec.argv],env:spec.env,cwd:spec.cwd}),
 },
 "linux-sandbox":{
  name:"linux-sandbox",
  available:()=>process.platform==="linux"&&probe("bwrap",remoteExecutionBubblewrapProbeArgumentsV1(realpathSync(tmpdir()))),
  wrap:spec=>({argv:remoteExecutionBubblewrapArgumentsV1(spec),env:{PATH:spec.env.PATH??"/usr/bin:/bin"},cwd:spec.root}),
 },
 "isolated-root":{
  name:"isolated-root",
  available:()=>true,
  wrap:spec=>({argv:spec.argv,env:spec.env,cwd:spec.cwd}),
 },
});

/** 🧭️ The strategies of a host in the order the hermetic table lists them (`sandboxed` becomes the host's own sandbox; `remote` is the scheduler's business and is skipped). */
export function remoteExecutionSandboxOrderV1(platform:string=process.platform):readonly RemoteExecutionSandboxV1[]{
 const row=(HERMETIC_TABLE_V1.strategies as Record<string,readonly string[]>)[platform];
 if(!row)throw Error(`Hermetic strategies required for platform: ${platform}`);
 const sandbox:RemoteExecutionSandboxV1|undefined=platform==="darwin"?"darwin-sandbox":platform==="linux"?"linux-sandbox":undefined;
 return row.flatMap(name=>name==="sandboxed"&&sandbox?[sandbox]:name==="isolated-root"?["isolated-root" as const]:[]);
}

/** 🎛️ Picks the strategy for a worker: an explicit name must be available, `auto` takes the first available one of the host's table. */
export function selectRemoteExecutionSandboxV1(requested:"auto"|RemoteExecutionSandboxV1,platform:string=process.platform):RemoteExecutionSandboxStrategyV1{
 if(requested!=="auto"){
  const strategy=STRATEGIES[requested];
  if(!strategy.available())throw Error(`Sandbox strategy not available on this host: ${requested}`);
  return strategy;
 }
 for(const name of remoteExecutionSandboxOrderV1(platform)){const strategy=STRATEGIES[name];if(strategy.available())return strategy;}
 return STRATEGIES["isolated-root"];
}
