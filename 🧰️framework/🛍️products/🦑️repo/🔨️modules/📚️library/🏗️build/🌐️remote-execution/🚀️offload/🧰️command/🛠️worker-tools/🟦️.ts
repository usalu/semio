import {createHash} from "node:crypto";
import {accessSync,constants,readFileSync,statSync} from "node:fs";
import {delimiter,dirname,join} from "node:path";
import type {RemoteExecutionPropertyV1} from "../../../🟦️.ts";
import type {OffloadToolV1} from "../../🧰️toolchain/🟦️.ts";
import {COMMAND_POLICY_V1,commandToolKeyV1,commandToolPropertyV1,commandToolSourceV1} from "../📐️policy/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;

/** 🖥️ What a worker offers for command offload: its resolved tools by `<tool>/<digest>` (an empty table still enables the shipped tools) and the platform properties that say which commands it can run. */
export type OffloadWorkerCommandsV1=Readonly<{tools:Readonly<Record<string,OffloadToolV1>>;properties:readonly RemoteExecutionPropertyV1[]}>;

/** 🔭️ How a resolved tool is found on this machine: the path of its file, `undefined` when it is not installed. */
export type OffloadToolFinderV1=()=>string|undefined|Promise<string|undefined>;

const digests=new Map<string,string>();
/** 🔏️ The SHA-256 of a tool file, remembered per path, size and modification time so a worker hashes each binary once. */
export function commandToolDigestV1(path:string):string{
 const info=statSync(path),key=`${path}|${info.size}|${info.mtimeMs}`,known=digests.get(key);
 if(known!==undefined)return known;
 const digest=createHash("sha256").update(readFileSync(path)).digest("hex");digests.set(key,digest);return digest;
}

const onPath=(name:string,environment:Environment):string|undefined=>{
 for(const directory of (environment.PATH??"").split(delimiter)){if(directory==="")continue;const full=join(directory,name);try{if(statSync(full).isFile()){accessSync(full,constants.X_OK);return full;}}catch{}}
 return undefined;
};

/** 🧰️ The finders of the resolved tools of the allow-list: `wasm-opt` is the pinned Binaryen the checkout prepared (`preparedBinaryen`, checksum-verified against its receipt), `node` the first one on the PATH. */
export function defaultCommandToolFindersV1(root:string,environment:Environment):Readonly<Record<string,OffloadToolFinderV1>>{
 return {
  "wasm-opt":async()=>{try{return (await import("../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🟦️.ts")).preparedBinaryen(root);}catch{return undefined;}},
  node:()=>onPath(process.platform==="win32"?"node.exe":"node",environment),
 };
}

/** 🔭️ Finds every resolved tool of the allow-list on this machine and offers each by the digest of its own file: the table the worker runs commands from and one platform property per tool (`semio-command-<tool>=<digest>`) beside the command format, so the scheduler places a command only here when this machine has exactly that tool. A tool that is not installed is simply not offered. */
export async function detectWorkerCommandsV1(root:string,environment:Environment=process.env,finders:Readonly<Record<string,OffloadToolFinderV1>>=defaultCommandToolFindersV1(root,environment)):Promise<OffloadWorkerCommandsV1>{
 const tools:Record<string,OffloadToolV1>={},properties:RemoteExecutionPropertyV1[]=[{name:COMMAND_POLICY_V1.property.format,value:COMMAND_POLICY_V1.property.formatValue}];
 for(const name of Object.keys(COMMAND_POLICY_V1.tools)){
  if(commandToolSourceV1(name)!=="resolved")continue;
  try{
   const path=await finders[name]?.();
   if(path===undefined)continue;
   const digest=commandToolDigestV1(path);
   tools[commandToolKeyV1(name,digest)]=Object.freeze({path,readable:Object.freeze([dirname(dirname(path))])});
   properties.push(commandToolPropertyV1(name,digest));
  }catch{}
 }
 return Object.freeze({tools:Object.freeze(tools),properties:Object.freeze(properties)});
}
