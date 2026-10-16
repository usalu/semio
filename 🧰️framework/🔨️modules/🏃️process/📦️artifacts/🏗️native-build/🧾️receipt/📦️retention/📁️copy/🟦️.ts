import {constants} from "node:fs";
import {lstat,open} from "node:fs/promises";
import {dirname,isAbsolute,resolve} from "node:path";
import {createHash} from "node:crypto";
import {withArtifactDescriptorV1,artifactReadFlagsV1} from "../../../../../../📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import {withArtifactDescriptorSlotV1} from "../../../../📤️publication/📥️sources/🟦️.ts";
import {requireIssuedResourceControlV1,withIssuedResourceOperationV1} from "../../../../../🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";
import sourceSchema from "../../../../📤️publication/📥️sources/🧬️schema/🔣️.json";
const maximumBytes=sourceSchema.properties.maximumBytes.const,chunkBytes=sourceSchema.properties.chunkBytes.const;
type Identity=Readonly<{dev:bigint;ino:bigint;size:bigint;mode:bigint;mtimeNs:bigint;ctimeNs:bigint;nlink:bigint}>;
export interface RetainedCompilationFileV1{readonly path:string;readonly sha256:string;readonly byteLength:number;readonly mode:number}
const same=(a:Identity,b:Identity)=>a.dev===b.dev&&a.ino===b.ino&&a.size===b.size&&a.mode===b.mode&&a.mtimeNs===b.mtimeNs&&a.ctimeNs===b.ctimeNs&&a.nlink===b.nlink;
async function ancestors(path:string,control:ResourceLeaseControl){const rows:{path:string;dev:bigint;ino:bigint}[]=[];for(let parent=dirname(path);;parent=dirname(parent)){checkResourceLeaseControl(control);const row=await lstat(parent,{bigint:true});checkResourceLeaseControl(control);if(!row.isDirectory()||row.isSymbolicLink())throw Error("Real compilation copy ancestry required");rows.push({path:parent,dev:row.dev,ino:row.ino});if(dirname(parent)===parent)break;}return rows;}
async function recheck(rows:readonly {path:string;dev:bigint;ino:bigint}[],control:ResourceLeaseControl){for(const row of rows){checkResourceLeaseControl(control);const actual=await lstat(row.path,{bigint:true});checkResourceLeaseControl(control);if(!actual.isDirectory()||actual.isSymbolicLink()||actual.dev!==row.dev||actual.ino!==row.ino)throw Error("Compilation copy ancestor changed");}}

/** 📑️ Copies one exact actual descriptor extent; Native destination custody is checked by the private calling producer. */
export function copyCompilationFileV1(source:string,destination:string,sha256:string,remainingBytes:number,control:ResourceLeaseControl):Promise<RetainedCompilationFileV1>{
 requireIssuedResourceControlV1(control);return withIssuedResourceOperationV1(control,()=>withArtifactDescriptorSlotV1(()=>withArtifactDescriptorSlotV1(async()=>{
  for(const path of[source,destination])if(!isAbsolute(path)||resolve(path)!==path||path.includes("\0")||path.length>256)throw Error("Canonical finite compilation copy path required");if(source===destination||!/^[a-f0-9]{64}$/u.test(sha256)||!Number.isSafeInteger(remainingBytes)||remainingBytes<0||remainingBytes>maximumBytes)throw Error("Finite exact compilation copy request required");
  const parents=await ancestors(source,control),outputs=await ancestors(destination,control),before=await lstat(source,{bigint:true});checkResourceLeaseControl(control);if(!before.isFile()||before.isSymbolicLink()||before.size>BigInt(remainingBytes))throw Error("Actual compilation copy extent exceeds remaining budget");const input=await open(source,artifactReadFlagsV1());
  return withArtifactDescriptorV1(input,async()=>{checkResourceLeaseControl(control);if(!same(before,await input.stat({bigint:true})))throw Error("Compilation source changed during descriptor admission");const output=await open(destination,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),Number(before.mode&0o777n));
   return withArtifactDescriptorV1(output,async()=>{const buffer=Buffer.allocUnsafe(chunkBytes),hash=createHash("sha256"),extent=Number(before.size),started=performance.now();let offset=0,chunks=0;while(offset<extent){checkResourceLeaseControl(control);const length=Math.min(chunkBytes,extent-offset),row=await input.read(buffer,0,length,offset);checkResourceLeaseControl(control);if(row.bytesRead!==length)throw Error("Compilation source short read");hash.update(buffer.subarray(0,length));let written=0;while(written<length){checkResourceLeaseControl(control);const result=await output.write(buffer,written,length-written,offset+written);checkResourceLeaseControl(control);if(result.bytesWritten===0)throw Error("Compilation destination short write");written+=result.bytesWritten;}offset+=length;if(++chunks%64===0){await control.publish({resource:"compilation-retention:copy",mode:"exclusive",elapsedMs:performance.now()-started});await control.yieldContinuation();checkResourceLeaseControl(control);}}
    if((await input.read(buffer,0,1,extent)).bytesRead!==0||hash.digest("hex")!==sha256||!same(before,await input.stat({bigint:true}))||!same(before,await lstat(source,{bigint:true})))throw Error("Actual compilation source identity, extent or digest changed");await recheck(parents,control);await recheck(outputs,control);const final=await output.stat({bigint:true});checkResourceLeaseControl(control);if(!final.isFile()||final.nlink!==1n||final.size!==before.size)throw Error("Actual compilation destination changed");return Object.freeze({path:destination,sha256,byteLength:extent,mode:Number(before.mode&0o777n)});
   });
  });
 })));
}
