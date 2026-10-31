import fixedSchema from"../../../🧱️fixed/🧬️schema/🔣️.json"with{type:"json"};
import{readNativeDirectoryAnchorFileV1,observeNativeDirectoryStateV1}from"../../../📁️directory/🟦️.ts";
import schema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import mountSchema from"../../../🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{lstatSync}from"node:fs";
import{join,dirname}from"node:path";
import{readNativeImageRetirementFileV1}from"../../../📄️retirement-file/🟦️.ts";
import{nativeStorageBackendOfDirectoryV1}from"../../../🔌️backends/📋️contract/🟦️.ts";
import{nativeImageRootV1,nativeImageAttachedV1,nativeImageFilesystemBytesV1,requireNativeImagePublicationSettledV1}from"../../../🌐️platform/📋️contract/🟦️.ts";
import{nativeImageBackingAllocatedBytesV1}from"../../../🪶️extent/🟦️.ts";
import{type NativeStoragePolicyV1}from"../../../📜️journal/🟦️.ts";
import{nativeStorageFundingPublishedEnvelopeV1,nativeStorageFundingDirectoryEnvelopeV1}from"../📋️model/🟦️.ts";
/** 📸️ Captures only the current published enforcement ceiling and actual unchanged physical backing. */
export function captureNativeStorageFundingBackingV1(policy:NativeStoragePolicyV1,check:()=>void):Readonly<{stamp:string;capacityBytes:number;check():void}>{
 const directory=dirname(policy.root),backend=nativeStorageBackendOfDirectoryV1(directory),mountedVolume=backend.capabilities.includes("mounted-volume"),root=nativeImageRootV1(directory),image=join(directory,backend.backingFilename);
 const observe=()=>{check();requireNativeImagePublicationSettledV1(directory);if(root!==policy.root)throw Error("Canonical Native funding root required");const record=readNativeImageRetirementFileV1(join(directory,"image.json"),4096),authored=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(record.bytes)),folder=lstatSync(directory),file=lstatSync(image),mounted=lstatSync(root);
 const selectedSchema=mountedVolume?schema:fixedSchema;if(validateJsonSchemaSubset(selectedSchema.definitions.ImageReceipt,authored,selectedSchema,[mountSchema]).length||!mountedVolume&&authored.platform!==process.platform||!folder.isDirectory()||folder.isSymbolicLink()||!file.isFile()||file.isSymbolicLink()||file.nlink!==1||!mounted.isDirectory()||mounted.isSymbolicLink()||folder.dev!==file.dev||authored.dev!==file.dev||authored.ino!==file.ino||authored.size!==file.size||!nativeImageAttachedV1(image,root))throw Error("Exact published Native funding physical backing required");
 const directoryCapacity=mountedVolume?policy.capacityBytes:nativeStorageFundingDirectoryEnvelopeV1(readNativeDirectoryAnchorFileV1(directory),observeNativeDirectoryStateV1(directory),policy),capacityBytes=Math.min(directoryCapacity,nativeStorageFundingPublishedEnvelopeV1({filesystemBytes:authored.filesystemBytes,maximumPhysicalBytes:authored.maximumPhysicalBytes,size:file.size,allocatedBytes:nativeImageBackingAllocatedBytesV1(file),observedFilesystemBytes:nativeImageFilesystemBytesV1(root,authored.filesystemBytes)},policy)),stamp=JSON.stringify([record.stamp,folder.dev,folder.ino,folder.mode,file.dev,file.ino,file.size,mounted.dev,mounted.ino,mounted.mode]);check();return{stamp,capacityBytes};},original=observe(),verify=()=>{const current=observe();if(current.stamp!==original.stamp||current.capacityBytes!==original.capacityBytes)throw Error("Published Native funding backing changed");};verify();return Object.freeze({...original,check:verify});
}
