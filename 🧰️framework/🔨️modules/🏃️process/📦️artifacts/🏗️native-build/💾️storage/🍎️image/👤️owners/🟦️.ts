import{realpathSync}from"node:fs";
import imageSchema from"../../🧬️schema/🔣️.json";
const protocol=imageSchema.definitions.ImageProtocol.const;
/** 🔗️ Builds the closed `hdiutil attach` argument list; owners stay on so nested images can attach inside persistent parents. */
export function nativeImageAttachArgumentsV1(root:string,image:string):readonly string[]{return Object.freeze(["attach",...protocol.attachArguments,"-mountpoint",root,image]);}
/** 🔍️ Resolves the innermost mount that hosts `directory` from `mount` output, without a runtime library. */
export function nativeImageHostMountV1(mountOutput:string,directory:string):Readonly<{mountPoint:string;flags:readonly string[];owners:boolean}>{return nativeImageHostMountOfV1(mountOutput,realpathSync(directory));}
/** 🔍️ Resolves the innermost mount of an already canonical absolute path from `mount` output. */
export function nativeImageHostMountOfV1(mountOutput:string,real:string):Readonly<{mountPoint:string;flags:readonly string[];owners:boolean}>{
 let best:{mountPoint:string;flags:string[]}|undefined;
 for(const line of mountOutput.split("\n")){const row=/^\S+ on (.+) \(([^()]*)\)$/u.exec(line);if(!row)continue;const mountPoint=row[1]!;if(mountPoint!==real&&!real.startsWith(mountPoint==="/"?"/":mountPoint+"/"))continue;if(!best||mountPoint.length>best.mountPoint.length)best={mountPoint,flags:row[2]!.split(",").map(flag=>flag.trim())};}
 if(!best)throw Error("Hosting mount of the Native image directory not found in the mount table");
 return Object.freeze({mountPoint:best.mountPoint,flags:Object.freeze(best.flags),owners:!best.flags.includes(protocol.mountWithoutOwnersFlag)});
}
/** 🛑️ Refuses a nested attach inside a parent mounted without owners and names the one-time remount. */
export function requireNativeImageHostOwnersV1(mountOutput:string,directory:string):void{
 const host=nativeImageHostMountV1(mountOutput,directory);
 if(!host.owners)throw Error("Nested Native image cannot attach inside the parent mounted without owners at "+host.mountPoint+"; once, while no process uses it, run `/usr/bin/hdiutil detach "+host.mountPoint+"` and rerun `bun repo`, which re-attaches the parent canonically with owners on");
}
