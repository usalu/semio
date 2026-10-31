import {createHash} from "node:crypto";
import {open} from "node:fs/promises";
import {withCapturedCargoArtifactsTaskV1,type CapturedCargoArtifactsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {PublishedCargoExecutableReceiptV1} from "../../../../⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";

/** 🧾️ Observes the actual produced bytes while completed compiler capture custody is live. */
export function capturedCargoExecutableReceiptV1(capture:CapturedCargoArtifactsV1,name:string):Promise<PublishedCargoExecutableReceiptV1>{
 return withCapturedCargoArtifactsTaskV1(capture,async view=>{
  const path=view.files.get(name);if(!path)throw Error("Declared executable absent from actual Cargo capture");
  const file=await open(path,"r"),hash=createHash("sha256"),buffer=Buffer.alloc(65536);let bytes=0;
  try{
   const before=await file.stat();if(!before.isFile()||before.nlink!==1||!Number.isSafeInteger(before.size)||before.size<1)throw Error("Actual produced executable extent required");
   for(;;){checkResourceLeaseControl(view.control);const row=await file.read(buffer,0,buffer.length,null);if(!row.bytesRead)break;bytes+=row.bytesRead;if(bytes>before.size)throw Error("Produced executable grew during observation");hash.update(buffer.subarray(0,row.bytesRead));if(bytes%4194304===0){await view.control.publish({resource:"cargo:produced-executable",mode:"shared",elapsedMs:0});await view.control.yieldContinuation();}}
   const after=await file.stat();if(bytes!==before.size||before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeMs!==after.mtimeMs||before.ctimeMs!==after.ctimeMs||before.mode!==after.mode)throw Error("Produced executable changed during observation");
   return Object.freeze({version:1,name,digest:hash.digest("hex"),bytes,permissions:before.mode&0o777});
  }finally{await file.close();}
 });
}
