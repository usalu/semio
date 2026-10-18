import{lstatSync,opendirSync}from"node:fs";
import{resolve}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import exportedSchema from"./🧬️schema/🔣️.json" with{type:"json"};
const schema=JSON.parse(JSON.stringify(exportedSchema))as typeof exportedSchema;
/** 📂️ Exact underlying directory observations carry no image, path, port or retirement authority. */
export type NativeImageMountDirectoryV1=Readonly<{dev:string;ino:string;mode:number}>;
/** 🧬️ Validates copied finite directory DATA against the sole cross-platform receipt schema. */
export function readNativeImageMountDirectoryV1(value:unknown):NativeImageMountDirectoryV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed underlying mount directory identity required");const row=value as NativeImageMountDirectoryV1;return Object.freeze({dev:row.dev,ino:row.ino,mode:row.mode});}
/** 🔎️ Observes a real no-link directory before activation; only the actual creator retains its claim. */
export function observeNativeImageMountDirectoryV1(path:string):NativeImageMountDirectoryV1{if(path!==resolve(path)||path.length>256)throw Error("Canonical bounded underlying mount directory required");const row=lstatSync(path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink())throw Error("Actual underlying mount directory required");return readNativeImageMountDirectoryV1({dev:String(row.dev),ino:String(row.ino),mode:Number(row.mode)});}
/** 🔏️ Refuses changed or replaced original directories after actual kernel detachment. */
export function requireNativeImageMountDirectoryV1(path:string,expected:NativeImageMountDirectoryV1):void{const held=readNativeImageMountDirectoryV1(expected),actual=observeNativeImageMountDirectoryV1(path);if(actual.dev!==held.dev||actual.ino!==held.ino||actual.mode!==held.mode)throw Error("Original underlying mount directory changed");}

/** 🛑️ Read-only preflight refuses unknown entries before a private creator may remove any owned file. */
export function requireEmptyNativeImageMountDirectoryV1(path:string,expected:NativeImageMountDirectoryV1,check:()=>void):void{const captured=readNativeImageMountDirectoryV1(expected),protocol=schema.definitions.TeardownProtocol.const;check();requireNativeImageMountDirectoryV1(path,captured);check();const directory=opendirSync(path);let failed=false,failure:unknown;try{check();const entry=directory.readSync();check();if(entry||protocol.maximumEntries!==0)throw Error("Unknown entry in original underlying mount directory");requireNativeImageMountDirectoryV1(path,captured);check();}catch(error){failed=true;failure=error;}try{directory.closeSync();}catch(error){if(failed&&!Object.is(failure,error))throw new AggregateError([failure,error],"Mount preflight and descriptor close failed");throw error;}if(failed)throw failure;check();requireNativeImageMountDirectoryV1(path,captured);check();}

import{types as nativeMountTypesV1}from"node:util";
/** 🪟️ Carries stable native underlying identity without inventing Unix metadata or ownership. */
export type NativeImageWindowsMountDirectoryV1=Readonly<{version:1;platform:"win32";dev:string;ino:string;volumeSerial:string;fileId:string;volume:string;attributes:number;creationTime:string;securitySha256:string;streamsSha256:string}>;
/** 🧬️ Captures only closed detached native receipt DATA and never issues a physical capability. */
export function readNativeImageWindowsMountDirectoryV1(value:unknown):NativeImageWindowsMountDirectoryV1{if(!value||typeof value!=="object"||nativeMountTypesV1.isProxy(value)||Object.getPrototypeOf(value)!==Object.prototype)throw Error("Plain native underlying directory DATA required");const result:Record<string,unknown>={};for(const key of Reflect.ownKeys(value)){const item=Object.getOwnPropertyDescriptor(value,key);if(typeof key!=="string"||!item||!("value"in item)||!item.enumerable)throw Error("Own native underlying directory DATA required");Object.defineProperty(result,key,{value:item.value,enumerable:true});}if(validateJsonSchemaSubset(schema.definitions.WindowsDirectory,result,schema).length||[result.dev,result.ino,result.volumeSerial].some(part=>BigInt(part as string)>18446744073709551615n))throw Error("Closed native underlying directory identity required");return Object.freeze(result)as NativeImageWindowsMountDirectoryV1;}
