import {readFileSync,lstatSync,statfsSync,mkdirSync,existsSync,realpathSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {captureOwnedCommandV1} from "../../../../🎛️owned-execution/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {openNativeFixedPlatformV1,nativeLinuxLoopMountV1,type NativeFixedPlatformOptionsV1,type NativeFixedPlatformPortV1} from "../🧱️fixed/🟦️.ts";
import type {NativeImageStorageV1} from "../🌐️platform/📋️contract/🟦️.ts";

const linux:NativeFixedPlatformPortV1={filename:"fixed.img",prepare:async(options,image,root,fresh)=>{
 checkResourceLeaseControl(options.control);const capabilities=/^CapEff:\s*([0-9a-f]+)$/imu.exec(readFileSync("/proc/self/status","utf8"));if(!capabilities||!(BigInt("0x"+capabilities[1])&(1n<<21n)))throw Error("Linux fixed-volume provisioning requires actual CAP_SYS_ADMIN; no unbounded fallback");
 if(fresh&&existsSync(options.directory))throw Error("Fresh original Linux image directory required");let parent=dirname(options.directory);while(!existsSync(parent)&&dirname(parent)!==parent)parent=dirname(parent);const fs=statfsSync(parent);if(fresh&&fs.bavail*fs.bsize<options.maximumPhysicalBytes+67108864)throw Error("Host capacity cannot admit fixed Linux image");
 const capture=(program:string,args:readonly string[])=>captureOwnedCommandV1(program,args,parent,{control:options.control});await capture("fallocate",["--version"]);await capture("mkfs.ext4",["-V"]);await capture("mount",["--version"]);
 if(fresh){mkdirSync(root,{recursive:true});await capture("fallocate",["--length",String(options.filesystemBytes),"--",image]);const file=lstatSync(image);if(file.size!==options.filesystemBytes||file.blocks*512<file.size||file.blocks*512>options.maximumPhysicalBytes)throw Error("Actual fully allocated fixed Linux image required");await capture("mkfs.ext4",["-q","-F","-E","nodiscard",image]);}await capture("mount",["-t","ext4","-o","loop,nodev,nosuid",image,root]);
},observe:async(options,image,root)=>{
 checkResourceLeaseControl(options.control);const file=lstatSync(image);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size!==options.filesystemBytes||file.blocks*512<file.size||file.blocks*512>options.maximumPhysicalBytes)throw Error("Actual fixed Linux allocation required");
 const mount=nativeLinuxLoopMountV1(readFileSync("/proc/self/mountinfo","utf8"),root),loop="/sys/dev/block/"+mount.device+"/loop/",backing=readFileSync(loop+"backing_file","utf8").trim().replace(/\\(040|011|012|134)/gu,(_,value)=>String.fromCharCode(parseInt(value,8)));
 if(realpathSync(backing)!==resolve(image)||readFileSync(loop+"offset","utf8").trim()!=="0"||readFileSync(loop+"sizelimit","utf8").trim()!=="0")throw Error("Kernel loop mapping must own the exact complete fixed file");return JSON.stringify(mount);
}};

/** 🐧️ Opens only a real fixed loop-backed filesystem; missing namespace/mount capability refuses before provisioning. */
export async function openNativeLinuxImageStorageV1(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>{checkResourceLeaseControl(options.control);if(process.platform!=="linux")throw Error("Linux fixed loop backend unavailable on this platform");return openNativeFixedPlatformV1(options,linux);}
