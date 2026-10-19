import {access,readdir,realpath,stat} from "node:fs/promises";
import {constants} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {probeNativeDirectoryCaseCapabilityV1,probeNativeDirectoryExchangeV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import {readNativeWindowsLongPathsEnabledV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🪟️host/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";

const policy=schema.definitions.PlatformPolicy.const,version=schema.definitions.Release.const.version;

export type BazelPlatformFactsV1={readonly platform:"win32"|"darwin"|"linux";readonly exchange:"supported"|"unsupported"|"unprobed";readonly caseSensitive:"yes"|"no"|"unprobed";readonly imageTool:boolean;readonly longPaths:"enabled"|"disabled"|"unprobed";readonly paths:readonly {readonly label:"workspace"|"tools"|"dependencies"|"products"|"temporary";readonly path:string;readonly blocking:boolean}[]};
export type BazelPlatformReportV1={readonly refusals:string[];readonly warnings:string[]};
export type BazelReadinessV1={readonly version:1;readonly bazel:{readonly version:string;readonly platform:string;readonly arch:string;readonly sha256:string;readonly executable:string};readonly modules:{readonly lockSha256:string;readonly blobs:number}};
export type BazelProgressEventV1={readonly phase:"waiting"|"verifying"|"downloading"|"ready";readonly bytes:number;readonly total:number};

/** 🔤️ Locates the first character beyond 7-bit ASCII that Bazel's native Windows launcher cannot canonicalize. */
export function bazelNonAsciiV1(path:string):{readonly index:number;readonly codePoint:number}|undefined{
 for(let index=0;index<path.length;){const codePoint=path.codePointAt(index)!;if(codePoint>0x7f)return{index,codePoint};index+=codePoint>0xffff?2:1;}
 return undefined;
}

/** ⚖️ Reports every unmet platform prerequisite of Bazel and its fixed native images, with the precise remedy. */
export function bazelPlatformProblemsV1(facts:unknown):BazelPlatformReportV1{
 if(validateJsonSchemaSubset(schema.definitions.PlatformFacts,facts).length)throw Error("Closed Bazel platform facts required");
 const actual=facts as BazelPlatformFactsV1,refusals:string[]=[],warnings:string[]=[];
 if(actual.platform==="win32"){
  for(const row of actual.paths){const bad=bazelNonAsciiV1(row.path);if(!bad)continue;(row.blocking?refusals:warnings).push(`Bazel ${version} cannot start on Windows under the non-ASCII ${row.label} path ${row.path} (U+${bad.codePoint.toString(16).toUpperCase().padStart(4,"0")} at index ${bad.index}): its native launcher fails with ${policy.windowsLauncherFailure}. An ASCII SUBST drive or junction does not help because the launcher resolves it back to the real path. Move the checkout to an ASCII-only directory such as C:\\semio and rerun.`);}
  if(actual.longPaths==="disabled")refusals.push("Windows resolves every declared source through the Bazel execroot below the output root, which adds about 90 characters to tracked paths of up to 255 characters and so exceeds MAX_PATH 260 unless long paths are enabled machine-wide: run once in an elevated PowerShell, then open a new terminal: New-ItemProperty -Path HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem -Name LongPathsEnabled -Value 1 -PropertyType DWORD -Force");
 }

 if(actual.platform==="win32"&&actual.exchange!=="supported")refusals.push("The journaled directory swap (write-through MoveFileExW renames of sibling directories) failed its probe beneath the workspace cache root, so the directory-backed cache partitions cannot be published. Use a local NTFS volume for the checkout; no elevation is needed.");
 if((actual.platform==="linux"||actual.platform==="win32")&&actual.caseSensitive!=="yes")refusals.push(actual.platform==="win32"?"Directory-backed cache partitions need per-directory case sensitivity, which could not be set beneath the workspace cache root: it needs NTFS, Windows 10 build 17107 or newer and the optional feature Microsoft-Windows-Subsystem-Linux (run once in an elevated PowerShell, then restart: dism /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux). Setting the flag later needs no elevation.":"Directory-backed cache partitions need a case-sensitive filesystem beneath the workspace cache root; use a native Linux filesystem, not a case-folding bind mount.");
 if(actual.platform==="linux"&&actual.exchange!=="supported")refusals.push("Atomic directory exchange (renameat2 RENAME_EXCHANGE) is unsupported beneath the workspace cache root, so the directory-backed cache partitions cannot be published. Use a native Linux filesystem such as ext4, xfs or btrfs on Linux 3.15 or newer; no privilege is needed.");
 if(actual.platform==="darwin"&&!actual.imageTool)refusals.push(`${policy.darwinImageTool} is missing: the fixed cache images attach through hdiutil, which ships with macOS and needs no privilege.`);
 return {refusals,warnings};
}

/** 🔬️ Observes the invoking process and the declared partition roots without writing anything. */
export async function collectBazelPlatformFactsV1(workspace:string,roots:Readonly<Record<"tools"|"dependencies"|"products",string>>,signal:AbortSignal):Promise<BazelPlatformFactsV1>{
 signal.throwIfAborted();
 const resolved=await realpath(workspace).catch(()=>workspace),paths=[{label:"workspace" as const,path:resolved,blocking:true},...(["tools","dependencies","products"] as const).map(label=>({label,path:roots[label],blocking:true})),{label:"temporary" as const,path:tmpdir(),blocking:false}];
 let exchange:BazelPlatformFactsV1["exchange"]="unprobed",caseSensitive:BazelPlatformFactsV1["caseSensitive"]="unprobed",imageTool=true,longPaths:BazelPlatformFactsV1["longPaths"]="unprobed";
 if(process.platform==="linux"||process.platform==="win32"){let host=roots.tools;for(;;){try{await stat(host);break;}catch{const parent=dirname(host);if(parent===host)break;host=parent;}}try{probeNativeDirectoryExchangeV1(host);exchange="supported";}catch{exchange="unsupported";}try{caseSensitive=probeNativeDirectoryCaseCapabilityV1(host)?"yes":"no";}catch{caseSensitive="no";}}
 if(process.platform==="darwin")imageTool=await access(policy.darwinImageTool,constants.X_OK).then(()=>true,()=>false);
 if(process.platform==="win32")longPaths=readNativeWindowsLongPathsEnabledV1()?"enabled":"disabled";
 signal.throwIfAborted();
 return {platform:process.platform as BazelPlatformFactsV1["platform"],exchange,caseSensitive,imageTool,longPaths,paths};
}

/** 🚧️ Refuses before any image is attached or byte is downloaded when Bazel cannot run on this platform. */
export async function requireBazelPlatformV1(workspace:string,signal:AbortSignal,log:(line:string)=>void=line=>console.warn(line)):Promise<void>{
 const facts=await collectBazelPlatformFactsV1(workspace,{tools:repositoryCachePartitionRootV1(workspace,"tools"),dependencies:repositoryCachePartitionRootV1(workspace,"dependencies"),products:repositoryCachePartitionRootV1(workspace,"products")},signal),report=bazelPlatformProblemsV1(facts);
 for(const warning of report.warnings)log(`[bazel] warning: ${warning}`);
 if(report.refusals.length)throw Error(`Bazel platform preflight refused before any store was attached:\n${report.refusals.join("\n")}`);
}

/** 📈️ Turns acquisition events into bounded console lines: one per phase and one per configured percentage step. */
export function bazelProgressLineV1(event:BazelProgressEventV1,lastPercent:number):{readonly line:string|undefined;readonly lastPercent:number}{
 if(event.phase!=="downloading")return{line:`[bazel] ${event.phase} ${event.bytes}/${event.total}`,lastPercent:event.phase==="waiting"?0:lastPercent};
 const percent=Math.floor(event.bytes*100/Math.max(1,event.total)/policy.downloadStepPercent)*policy.downloadStepPercent;
 return percent>lastPercent?{line:`[bazel] downloading ${percent}% ${event.bytes}/${event.total}`,lastPercent:percent}:{line:undefined,lastPercent};
}

/** 🧾️ Reads the module catalog facts that prove a first offline run can start, and renders the combined readiness line. */
export async function describeBazelReadinessV1(input:{readonly executable:string;readonly platform:string;readonly arch:string;readonly sha256:string;readonly lock:string;readonly repository:string}):Promise<{readonly readiness:BazelReadinessV1;readonly line:string}>{
 const lockSha256=createHash("sha256").update(input.lock).digest("hex"),blobs=(await readdir(join(input.repository,"content_addressable/sha256"))).filter(value=>/^[a-f0-9]{64}$/u.test(value)).length,readiness:BazelReadinessV1={version:1,bazel:{version,platform:input.platform,arch:input.arch,sha256:input.sha256,executable:input.executable},modules:{lockSha256,blobs}};
 if(validateJsonSchemaSubset(schema.definitions.Readiness,readiness).length)throw Error("Closed Bazel readiness required");
 return {readiness,line:`[bazel] ready binary=${version} ${input.platform}/${input.arch} sha256=${input.sha256.slice(0,16)} modules=${blobs} blobs lock=${lockSha256.slice(0,16)}`};
}
