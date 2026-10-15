import {existsSync,lstatSync,readdirSync,rmdirSync,statSync,unlinkSync} from "node:fs";
import {join} from "node:path";
import {captureOwnedCommandV1,captureOwnedCommandResultV1} from "../../../../../../🎛️owned-execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../🏗️build/🎟️capacity/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../🧭️routing/📥️invocation/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";
import {nativeImageInformationV1,provisionNativeImageStorageV1} from "../../🟦️.ts";
import imageFixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import cleanupSchema from "../../../🧹️cleanup/🧬️schema/🔣️.json" with {type:"json"};
import {throwNativeImageCleanupFailuresV1,type NativeImageCleanupFailureRecordV1} from "../../../🧹️cleanup/📋️state/🟦️.ts";

/** 🛑️ Stages the actual macOS cancellation regression under one received caller; execute only after the capacity hold is cleared. */
export async function proveTemporaryImageCancellationCleanupV1(directory:string,original:ScriptInvocation):Promise<void>{
 if(process.platform!=="darwin")throw Error("Actual macOS image cleanup qualification requires darwin");
 if(existsSync(directory))throw Error("Cancellation fixture requires an absent owned directory");
 const outer=scriptCompilerAdmissionControlV1(original,"image:cleanup:fixture",90000),controller=new AbortController(),body=scriptCompilerAdmissionControlV1(original,"image:cleanup:body",60000,controller.signal),reason=Error("Original image fixture cancellation"),protocol=cleanupSchema.definitions.Protocol.const;
 const image=await provisionNativeImageStorageV1({...imageFixture.image,directory,control:body}),owner=lstatSync(directory),backing=lstatSync(image.image),mounted=statSync(image.root).dev;
 const exact=()=>{const actualDirectory=lstatSync(directory),actual=lstatSync(image.image);if(actualDirectory.isSymbolicLink()||actualDirectory.dev!==owner.dev||actualDirectory.ino!==owner.ino||actual.isSymbolicLink()||!actual.isFile()||actual.dev!==backing.dev||actual.ino!==backing.ino||actual.size!==backing.size||actual.nlink!==1)throw Error("Cancellation fixture physical identity changed; retained for explicit review");};
 const information=async()=>nativeImageInformationV1(await captureOwnedCommandV1("hdiutil",["info","-plist"],process.cwd(),{control:outer,output:{maximumBytes:protocol.maximumInformationBytes,maximumLines:protocol.maximumInformationLines}}));
 const mapping=async()=>{const value=await information();if(!Array.isArray(value.images))throw Error("Complete image inventory required");const candidates=(value.images as Record<string,unknown>[]).filter(row=>row["image-path"]===image.image);if(candidates.length>1)throw Error("Ambiguous fixture backing activation");return candidates.some(row=>Array.isArray(row["system-entities"])&&(row["system-entities"] as Record<string,unknown>[]).some(entity=>entity["mount-point"]===image.root));};
 const failures:NativeImageCleanupFailureRecordV1[]=[];let publications=0,acknowledged=0;
 try{
  exact();if(!(await mapping())||mounted===backing.dev)throw Error("Actual fixture mount identity required before cancellation");
  const observed:ResourceLeaseControl={...body,publish:async progress=>{publications++;await body.publish(progress);acknowledged++;controller.abort(reason);}};
  let bodyFailure:unknown;try{await captureOwnedCommandResultV1(process.execPath,["--eval","console.log('[DEBUG] Original cancellation body started');setTimeout(()=>{},30000)"],image.root,{control:observed,output:{maximumBytes:4096,maximumLines:16}});}catch(error){bodyFailure=error;}
  if(bodyFailure!==reason||publications!==1||acknowledged!==1||!body.signal.aborted)throw Error("Actual owned child progress and original cancellation were not observed");
  const closeStarted=performance.now();let closeFailed=false,closeFailure:unknown;try{await image.close();}catch(error){closeFailed=true;closeFailure=error;}
  const closeMilliseconds=performance.now()-closeStarted,remains=await mapping();console.log("[DEBUG] Actual original image cancellation close "+JSON.stringify({publications,acknowledged,closeMilliseconds,closeFailure:closeFailed?String(closeFailure):null,mounted:remains,directoryExists:existsSync(directory)}));
  if(closeFailed)throw Error("Temporary image close rejected the already accepted Original cancellation",{cause:closeFailure});
  if(closeMilliseconds>protocol.maximumCleanupMilliseconds)throw Error("Temporary image cleanup exceeded its finite settlement allowance");
  if(remains||existsSync(directory))throw Error("Temporary image cancellation cleanup did not settle accepted physical ownership");
 }catch(error){failures.push({role:"body",cause:error});}
 try{
  if(existsSync(directory)){
   exact();const entries=readdirSync(directory).sort();if(JSON.stringify(entries)!==JSON.stringify(["fixed.dmg","m"]))throw Error("Unknown cancellation fixture material retained");
   if(await mapping()){if(statSync(image.root).dev!==mounted)throw Error("Fixture mount device changed; retained");await captureOwnedCommandV1("hdiutil",["detach",image.root],process.cwd(),{control:outer});}
   exact();if(await mapping())throw Error("Fixture activation remains; backing retained");
   const root=lstatSync(image.root);if(root.isSymbolicLink()||!root.isDirectory()||root.dev!==backing.dev||readdirSync(image.root).length)throw Error("Fixture host mount directory is not empty; backing retained");
   unlinkSync(image.image);rmdirSync(image.root);rmdirSync(directory);console.log("[DEBUG] Original fixture supervisor settled only its exact accepted mount and backing after regression observation");
  }
 }catch(cleanupFailure){failures.push({role:"cleanup",cause:cleanupFailure});}
 throwNativeImageCleanupFailuresV1(failures);
}
