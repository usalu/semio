import{lstatSync}from"node:fs";
import exportedSchema from"../🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
export const nativeDirectorySchemaV1=JSON.parse(JSON.stringify(exportedSchema))as typeof exportedSchema;
const protocol=nativeDirectorySchemaV1.definitions.Protocol.const;

/** 🪪️ Names one directory by device, inode and mode only; it grants no path or mutation authority. */
export type NativeDirectoryIdentityV1=Readonly<{dev:string;ino:string;mode:number}>;
/** 📦️ Resolves the closed physical members of one directory-backed partition. */
export type NativeDirectoryLayoutV1=Readonly<{directory:string;anchor:string;volume:string;root:string;receipt:string}>;
/** 🔀️ Reports the only two physical states: the volume is the root, or the empty placeholder is. */
export type NativeDirectoryStateV1="inactive"|"attached";
/** 🛟️ Reports what recovery of an interrupted directory swap did. */
export type NativeDirectoryRecoveryV1="clean"|"rolled-back"|"completed";
/** 🔌️ The one platform seam of a directory partition: how two directory names are swapped, flushed, recovered and prepared for case-sensitive content. */
export type NativeDirectoryPlatformPortV1=Readonly<{
 kind:"exchange"|"journaled";
 swap(first:string,second:string):void;
 flush(path:string):void;
 pending(directory:string):boolean;
 recover(directory:string):NativeDirectoryRecoveryV1;
 prepareVolume(directory:string):void;
}>;

/** 🧬️ Returns the immutable first-party protocol DATA for directory-backed partitions. */
export function nativeDirectoryProtocolV1():typeof protocol{return protocol;}

/** 🔎️ Observes a real directory without following links. */
export function observeNativeDirectoryIdentityV1(path:string):NativeDirectoryIdentityV1{const row=lstatSync(path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink())throw Error("Actual directory required");return Object.freeze({dev:String(row.dev),ino:String(row.ino),mode:Number(row.mode)});}
/** 📸️ Detaches caller DATA into one closed identity. */
export function readNativeDirectoryIdentityV1(value:unknown):NativeDirectoryIdentityV1{if(validateJsonSchemaSubset(nativeDirectorySchemaV1.definitions.Identity,value,nativeDirectorySchemaV1).length)throw Error("Closed directory identity required");const row=value as NativeDirectoryIdentityV1;return Object.freeze({dev:row.dev,ino:row.ino,mode:row.mode});}
/** ⚖️ Compares two identities field by field. */
export const sameNativeDirectoryIdentityV1=(left:NativeDirectoryIdentityV1,right:NativeDirectoryIdentityV1)=>left.dev===right.dev&&left.ino===right.ino&&left.mode===right.mode;
