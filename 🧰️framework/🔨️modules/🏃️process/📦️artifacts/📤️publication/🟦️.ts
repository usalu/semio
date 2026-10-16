import { existsSync, lstatSync, readdirSync, constants, mkdirSync, rmSync, renameSync } from "node:fs";
import { createHash } from "node:crypto";
import { open,lstat } from "node:fs/promises";
import{withArtifactSourceCaptureV1,artifactCaptureEntriesV1,artifactCaptureDirectoryV1,recheckArtifactCaptureV1,withArtifactCapturedFileV1,commitArtifactCaptureV1,prepareArtifactCaptureCommitV1,drainArtifactCaptureV1,writeArtifactCaptureManifestV1,type ArtifactSourceCaptureV1}from"./📥️sources/📦️capture/🟦️.ts";
import{withArtifactDescriptorSlotV1}from"./📥️sources/🟦️.ts";
import{readArtifactPublicationManifestV1,type ArtifactPublicationManifestV1}from"./📋️manifest/🟦️.ts";
import { setTimeout as delay } from "node:timers/promises";
import { basename, dirname, resolve, join, isAbsolute, relative } from "node:path";
import { acquireResourceLease, captureResourceLeaseWriterProofV1, requireResourceLeaseWriterProofV1, resourceLeasePathV1, checkResourceLeaseControl, type ResourceLeaseControl, type LeaseWait } from "../../🔒️leases/🟦️.ts";

import{requireAcceptedPhysicalCleanupControlV1,readAcceptedPhysicalCleanupV1,beginAcceptedPhysicalCleanupV1,checkAcceptedPhysicalCleanupV1}from"../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type{AcceptedPhysicalCleanupV1}from"../../🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts";

export type ArtifactPublicationOptions = { readonly control?: ResourceLeaseControl; readonly signal?: AbortSignal; readonly leaseDirectory: string; readonly onWait?: (progress: LeaseWait) => void };

/** 🧮️ Compares file bytes in bounded async chunks while honoring publication cancellation. */
async function haveIdenticalBytes(capture:ArtifactSourceCaptureV1,name:string,destination:string,signal:AbortSignal,control?:ResourceLeaseControl):Promise<boolean>{
 return withArtifactCapturedFileV1(capture,name,async(read,entry)=>withArtifactDescriptorSlotV1(async()=>{const before=await lstat(destination,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size!==BigInt(entry.byteLength))return false;const right=await open(destination,constants.O_RDONLY|(constants.O_NOFOLLOW??0)),same=(row:typeof before)=>row.isFile()&&row.dev===before.dev&&row.ino===before.ino&&row.size===before.size&&row.mode===before.mode&&row.nlink===1n&&row.mtimeNs===before.mtimeNs&&row.ctimeNs===before.ctimeNs;let identical=false,failed=false,primary:unknown;try{if(!same(await right.stat({bigint:true})))throw Error("Published artifact descriptor changed during comparison");identical=await(async()=>{const rightBytes=Buffer.allocUnsafe(65536);let offset=0;while(offset<entry.byteLength){signal.throwIfAborted();if(control)checkResourceLeaseControl(control);const length=Math.min(rightBytes.length,entry.byteLength-offset),[left,result]=await Promise.all([read(offset,length),right.read(rightBytes,0,length,offset)]);if(result.bytesRead!==length||!Buffer.from(left).equals(rightBytes.subarray(0,length)))return false;offset+=length;}return(await right.read(rightBytes,0,1,offset)).bytesRead===0;})();if(!same(await right.stat({bigint:true}))||!same(await lstat(destination,{bigint:true})))throw Error("Published artifact identity changed during comparison");}catch(error){failed=true;primary=error;}let closeFailed=false,closeFailure:unknown;try{await right.close();}catch(error){closeFailed=true;closeFailure=error;}if(failed){if(closeFailed)throw new AggregateError([primary,closeFailure],"Artifact comparison and close failed");throw primary;}if(closeFailed)throw closeFailure;return identical;}));
}

/** 🔍️ Recognizes identical owned files without replacing resources already open in consumers. */
async function hasIdenticalArtifacts(staging: string, marker: string, owner: string, capture: ArtifactSourceCaptureV1, metadata:ArtifactPublicationManifestV1|undefined, signal: AbortSignal, control?: ResourceLeaseControl): Promise<boolean> {
  if (!metadata) return false;
  const entries=artifactCaptureEntriesV1(capture),names = entries.map(row=>row.name).sort();
  if (metadata.version !== 1 || metadata.owner !== owner || JSON.stringify(metadata.files) !== JSON.stringify(names)) return false;
  const paths = new Set([marker]);
  for (const name of names) {
    const destination = resolve(staging, name);
    for (let path = destination; path !== resolve(staging); path = dirname(path)) paths.add(relative(staging, path).replaceAll("\\", "/"));
  }
  const inspect = (directory: string): boolean => readdirSync(directory, { withFileTypes: true }).every(entry => {
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    const path = join(directory, entry.name), name = relative(staging, path).replaceAll("\\", "/");
    return paths.has(name) && !entry.isSymbolicLink() && (entry.isDirectory() ? inspect(path) : entry.isFile());
  });
  if (!inspect(staging)) return false;
  for (const authored of entries) {
    const name=authored.name;
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    const destination = join(staging, name);
    if (!existsSync(destination)) return false;
    const published = lstatSync(destination);
    if (!published.isFile() || published.isSymbolicLink() || published.nlink!==1 || published.size !== authored.byteLength || (published.mode & 0o777) !== authored.mode || !await haveIdenticalBytes(capture,name,destination,signal,control)) return false;
  }
  return true;
}


/** 🚚️ Waits for transient directory contention while preserving cancellation and bounded recovery. */
async function renameArtifactDirectory(source: string, destination: string, signal: AbortSignal, onWait?: (elapsedMs: number) => void, control?: ResourceLeaseControl): Promise<void> {
  const started = Date.now();
  for (let attempt = 0; ; attempt++) {
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    try { renameSync(source, destination); return; }
    catch (error) {
      const elapsedMs = Date.now() - started;
      if (!["EPERM", "EACCES", "EBUSY"].includes((error as NodeJS.ErrnoException).code ?? "") || elapsedMs >= 5000) throw error;
      onWait?.(elapsedMs);
      signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
      try { await delay(Math.min(25 * 2 ** Math.min(attempt, 3), 5000 - elapsedMs, control ? checkResourceLeaseControl(control) ?? 5000 : 5000), undefined, { signal }); }
      catch (error) { signal.throwIfAborted(); if (control) checkResourceLeaseControl(control); throw error; }
    }
  }
}

/** 🧱️ Reuses identical owned artifacts or replaces their tree with rollback on publication failure. */
export function stageArtifacts(staging:string,owner:string,files:ReadonlyMap<string,string>,options:ArtifactPublicationOptions):Promise<void>{return withArtifactSourceCaptureV1(resolve(staging),files,{...options,owner},capture=>executeArtifactPublicationV1(staging,owner,capture,options));}

/** 🛟️ Requires the exact pre-funded token and body authority before accepting the actual publication writer. */
export function stageOwnedArtifactsV1(staging:string,owner:string,files:ReadonlyMap<string,string>,options:ArtifactPublicationOptions&Readonly<{control:ResourceLeaseControl;cleanup:AcceptedPhysicalCleanupV1}>):Promise<void>{const captured=Object.freeze({...options}),facts=readAcceptedPhysicalCleanupV1(captured.cleanup);requireAcceptedPhysicalCleanupControlV1(captured.cleanup,captured.control);if(facts.kind!=="artifact-scratch"||facts.resource!=="artifact:"+resolve(staging))throw Error("Exact artifact publication cleanup enrollment required");return withArtifactSourceCaptureV1(resolve(staging),files,{...captured,owner,cleanup:captured.cleanup},capture=>executeArtifactPublicationV1(staging,owner,capture,captured,captured.cleanup));}

async function executeArtifactPublicationV1(staging:string,owner:string,capture:ArtifactSourceCaptureV1,options:ArtifactPublicationOptions,cleanup?:AcceptedPhysicalCleanupV1):Promise<void>{
 options=Object.freeze({...options});if(!isAbsolute(options.leaseDirectory))throw Error("Explicit absolute publication lease directory required");staging=resolve(staging);const control=options.control,signal=control?.signal??options.signal??new AbortController().signal;if(control){checkResourceLeaseControl(control);if(options.signal&&options.signal!==signal)throw Error("Artifact publication requires original signal");}const resource="artifact:"+staging,started=Date.now(),lease=await acquireResourceLease({directory:options.leaseDirectory,resource,mode:"exclusive",signal,control,onWait:options.onWait});let guardLive=true,temporary:string|undefined,previous:string|undefined,published=false,cleanupStarted=false,parentIdentity:ReturnType<typeof lstatSync>|undefined,temporaryIdentity:ReturnType<typeof lstatSync>|undefined,previousIdentity:ReturnType<typeof lstatSync>|undefined;const failures:unknown[]=[];
 const same=(path:string,identity:ReturnType<typeof lstatSync>|undefined)=>{const value=lstatSync(path);if(!identity||!value.isDirectory()||value.isSymbolicLink()||value.dev!==identity.dev||value.ino!==identity.ino)throw Error("Exact retained artifact directory inode required");};
 const cleanupSignal=async():Promise<AbortSignal>=>{try{await drainArtifactCaptureV1(capture);}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}if(!cleanup)return new AbortController().signal;if(!guardLive)throw Error("Actual retained artifact writer required");if(!cleanupStarted){cleanupStarted=true;try{await beginAcceptedPhysicalCleanupV1(cleanup);}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}}checkAcceptedPhysicalCleanupV1(cleanup);if(parentIdentity)same(dirname(staging),parentIdentity);return readAcceptedPhysicalCleanupV1(cleanup).cleanupSignal;};
 try{
  if(cleanup){const proof=captureResourceLeaseWriterProofV1(lease,options.leaseDirectory,resource,control!);requireResourceLeaseWriterProofV1(proof,{directory:options.leaseDirectory,resource,path:resourceLeasePathV1(options.leaseDirectory,resource)},control!);}
  const files=artifactCaptureEntriesV1(capture),marker=".semio-artifact.json";for(let parent=staging;dirname(parent)!==parent;parent=dirname(parent))if(existsSync(parent)&&lstatSync(parent).isSymbolicLink())throw Error(`Symlink artifact destination: ${parent}`);
  let metadata:ArtifactPublicationManifestV1|undefined;if(existsSync(staging)){const markerPath=join(staging,marker);if(!existsSync(markerPath))throw Error(`Unowned artifact directory: ${staging}`);metadata=await readArtifactPublicationManifestV1(staging,{control,signal});if(metadata.owner!==owner)throw Error(`Unowned artifact directory: ${staging}`);}
  const identical=await hasIdenticalArtifacts(staging,marker,owner,capture,metadata,signal,control);if(identical)await recheckArtifactCaptureV1(capture);signal.throwIfAborted();if(control)checkResourceLeaseControl(control);
  if(!identical){mkdirSync(dirname(staging),{recursive:true});parentIdentity=lstatSync(dirname(staging));temporary=artifactCaptureDirectoryV1(capture);temporaryIdentity=lstatSync(temporary);previous=temporary+".previous";await writeArtifactCaptureManifestV1(capture,owner);signal.throwIfAborted();if(control)checkResourceLeaseControl(control);const commit=await prepareArtifactCaptureCommitV1(capture,control?captureResourceLeaseWriterProofV1(lease,options.leaseDirectory,resource,control):undefined);const onWait=()=>options.onWait?.({resource,mode:"exclusive",elapsedMs:Date.now()-started});if(existsSync(staging)){previousIdentity=lstatSync(staging);await renameArtifactDirectory(staging,previous,signal,onWait,control);}try{await renameArtifactDirectory(temporary,staging,signal,onWait,control);commitArtifactCaptureV1(commit);published=true;}catch(error){failures.push(error);try{if(existsSync(previous)){const settledSignal=await cleanupSignal();if(cleanup)same(previous,previousIdentity);await renameArtifactDirectory(previous,staging,settledSignal,onWait);}}catch(rollback){failures.push(rollback);}}}
 }catch(error){failures.push(error);}
 try{await cleanupSignal();if(published&&previous&&existsSync(previous)){if(cleanup)same(previous,previousIdentity);rmSync(previous,{recursive:true,force:true});}if(cleanup)checkAcceptedPhysicalCleanupV1(cleanup);}catch(error){failures.push(error);}finally{guardLive=false;lease.release();}
 if(failures.length>1)throw new AggregateError(failures,"Artifact publication and retained cleanup failed");if(failures.length)throw failures[0];
}

/** 🧊️ Acquires immutable executable bytes before releasing the publisher's shared lease. */
export async function pinExecutableArtifact(source: string, cacheDirectory: string, owner: string, options: ArtifactPublicationOptions): Promise<string> {
  if (!isAbsolute(source) || !isAbsolute(cacheDirectory) || !isAbsolute(options.leaseDirectory)) throw Error("Explicit absolute executable publication paths required");
  const control=options.control,signal=control?.signal??options.signal??new AbortController().signal;
  if(control){checkResourceLeaseControl(control);if(options.signal&&options.signal!==signal)throw Error("Executable acquisition requires original signal");}
  const lease=await acquireResourceLease({directory:options.leaseDirectory,resource:`artifact:${dirname(source)}`,mode:"shared",signal,control,onWait:options.onWait});
  try {
    const digest=await executableDigest(source,signal,control),name=basename(source),directory=join(cacheDirectory,digest);
    await stageArtifacts(directory,owner,new Map([[name,source]]),options);
    const executable=join(directory,name);
    if(await executableDigest(executable,signal,control)!==digest)throw Error("Executable artifact changed during acquisition");
    return executable;
  } finally {lease.release();}
}

/** 🔏️ Hashes executable bytes in bounded cancellable reads. */
async function executableDigest(path: string, signal: AbortSignal, control?: ResourceLeaseControl): Promise<string> {
  signal.throwIfAborted();if(control)checkResourceLeaseControl(control);
  const file=await open(path,"r"),hash=createHash("sha256"),bytes=Buffer.allocUnsafe(65536),started=Date.now();let chunks=0;
  try {
    for(;;){signal.throwIfAborted();if(control)checkResourceLeaseControl(control);const {bytesRead}=await file.read(bytes,0,bytes.length,null);signal.throwIfAborted();if(control)checkResourceLeaseControl(control);if(!bytesRead)return hash.digest("hex");hash.update(bytes.subarray(0,bytesRead));if(control&&++chunks%64===0){await control.publish({resource:`artifact:${path}`,mode:"shared",elapsedMs:Date.now()-started});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);}}
  } finally {await file.close();}
}
