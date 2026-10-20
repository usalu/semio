import{lstatSync,opendirSync}from"node:fs";
import{dirname,resolve}from"node:path";
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

/** 🪢️ Binds publication DATA to the current anchored backing namespace without transferring ownership. */
export function bindPublishedNativeImageMountDirectoryV1(value:unknown):NativeImageMountDirectoryV1{if(validateJsonSchemaSubset(schema.definitions.PublishedBinding,value,schema).length)throw Error("Closed published Native directory binding required");const input=value as Readonly<{published:NativeImageMountDirectoryV1;publishedBackingDevice:string;currentBackingDevice:string}>;if(input.published.dev!==input.publishedBackingDevice)throw Error("Original published backing device namespace required");return readNativeImageMountDirectoryV1({dev:input.currentBackingDevice,ino:input.published.ino,mode:input.published.mode});}

/** 🧭️ Captures fresh bounded no-link backing ancestry as observations without retirement authority. */
export function captureNativeImageBackingAncestryV1(directory:string):()=>void{const held:Readonly<{path:string;identity:NativeImageMountDirectoryV1}>[]=[];for(let path=directory;;path=dirname(path)){if(held.length>=schema.definitions.LiveAncestryProtocol.const.maximumEntries)throw Error("Bounded live Native backing ancestry required");held.push(Object.freeze({path,identity:observeNativeImageMountDirectoryV1(path)}));if(dirname(path)===path)break;}return()=>{for(const row of held)requireNativeImageMountDirectoryV1(row.path,row.identity);};}
