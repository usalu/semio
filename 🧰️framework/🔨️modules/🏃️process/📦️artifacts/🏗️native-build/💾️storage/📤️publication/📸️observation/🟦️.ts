import{lstatSync}from"node:fs";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{nativeStorageBackendOfDirectoryV1,nativeStorageMountedAtV1}from"../../🔌️backends/📋️contract/🟦️.ts";
import{nativeImageRootV1,nativeImageAttachedV1}from"../../🌐️platform/📋️contract/🟦️.ts";
import{nativeImageBackingAllocatedBytesV1}from"../../🪶️extent/🟦️.ts";
import{readNativeImageRetirementFileV1}from"../../📄️retirement-file/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{readNativeImagePublicationRequestV1}from"../🟦️.ts";
import type{NativeImagePublicationOptionsV1}from"../🎛️owner/🟦️.ts";
import imageSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import metadataSchema from"../../🧱️fixed/🧬️schema/🔣️.json"with{type:"json"};
import mountSchema from"../../🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json"with{type:"json"};
/** 📷️ Captures current receipt and unchanged physical identities inside the canonical publisher writer. */
export function captureObservedNativeImagePublicationV1(options:NativeImagePublicationOptionsV1,check:()=>void){
 check();const directory=lstatSync(options.directory,{bigint:true}),image=join(options.directory,nativeStorageBackendOfDirectoryV1(options.directory).backingFilename),backing=lstatSync(image,{bigint:true}),rootPath=nativeImageRootV1(options.directory),root=lstatSync(rootPath,{bigint:true}),allocated=nativeImageBackingAllocatedBytesV1(backing),record=readNativeImageRetirementFileV1(join(options.directory,"image.json"),4096);check();
 const receipt=JSON.parse(record.bytes.toString()),schema=nativeStorageMountedAtV1(options.directory)?imageSchema:metadataSchema;
 if(validateJsonSchemaSubset(schema.definitions.ImageReceipt,receipt,schema,[mountSchema]).length||!directory.isDirectory()||directory.isSymbolicLink()||!root.isDirectory()||root.isSymbolicLink()||!backing.isFile()||backing.isSymbolicLink()||backing.nlink!==1n||backing.dev!==directory.dev||String(receipt.dev)!==String(backing.dev)||String(receipt.ino)!==String(backing.ino)||BigInt(receipt.size)!==backing.size||backing.size<1n||allocated<1n||backing.size>BigInt(options.maximumPhysicalBytes)||allocated>BigInt(options.maximumPhysicalBytes))throw Error("Exact bounded current published backing required");
 const initialState=nativeImageAttachedV1(image,rootPath)?"mounted":"inactive";check();
 const request=readNativeImagePublicationRequestV1({version:1,purpose:"current-exact-publication",initialState,receiptSha256:createHash("sha256").update(record.bytes).digest("hex"),directory:{dev:String(directory.dev),ino:String(directory.ino)},backing:{dev:String(backing.dev),ino:String(backing.ino),size:Number(backing.size),allocatedBytes:Number(allocated)},root:{dev:String(root.dev),ino:String(root.ino)}});
 const afterDirectory=lstatSync(options.directory,{bigint:true}),afterBacking=lstatSync(image,{bigint:true}),afterRoot=lstatSync(rootPath,{bigint:true}),afterRecord=readNativeImageRetirementFileV1(join(options.directory,"image.json"),4096);check();
 if(afterDirectory.dev!==directory.dev||afterDirectory.ino!==directory.ino||afterBacking.dev!==backing.dev||afterBacking.ino!==backing.ino||afterBacking.size!==backing.size||nativeImageBackingAllocatedBytesV1(afterBacking)!==allocated||afterRoot.dev!==root.dev||afterRoot.ino!==root.ino||afterRecord.stamp!==record.stamp||!afterRecord.bytes.equals(record.bytes))throw Error("Observed publication facts changed under custody");
 return request;
}
