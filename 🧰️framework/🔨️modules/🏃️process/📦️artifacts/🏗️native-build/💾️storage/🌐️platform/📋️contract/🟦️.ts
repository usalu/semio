import {lstatSync,opendirSync} from "node:fs";
import {join,resolve,relative,isAbsolute,sep} from "node:path";
import {requireMaterializedPathV1,type ResourceLeaseWriterProofV1,type ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";
import type {NativeStoragePortV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const protocol=schema.definitions.ImageProtocol.const;

/** 🚦️ Chooses required quiescence or an immediate optional-maintenance attempt. */
export type NativeImageMaintenanceRequestV1=Readonly<{version:1;wait:boolean}>;
/** 🧷️ Binds optional historical backing authority to one exact stable receipt and physical allocation. */
export type NativeImageRetirementExpectationV1=Readonly<{receiptSha256:string;dev:number;ino:number;size:number;allocatedBytes:number}>;
/** ♻️ Separates historical retirement admission from reader maintenance. */
export type NativeImageRetirementRequestV1=Readonly<{version:1;wait:boolean;expectedImage?:NativeImageRetirementExpectationV1}>;
/** 📸️ Detaches caller data before any queued exclusive acquisition. */
export function readNativeImageRetirementRequestV1(value:unknown):NativeImageRetirementRequestV1{if(validateJsonSchemaSubset(schema.definitions.ImageRetirementRequest,value,schema).length)throw Error("Closed original image retirement request required");const request=value as NativeImageRetirementRequestV1,expected=request.expectedImage;const captured={version:1 as const,wait:request.wait,...(expected?{expectedImage:Object.freeze({receiptSha256:expected.receiptSha256,dev:expected.dev,ino:expected.ino,size:expected.size,allocatedBytes:expected.allocatedBytes})}:{})};if(validateJsonSchemaSubset(schema.definitions.ImageRetirementRequest,captured,schema).length)throw Error("Closed original image retirement request required");return Object.freeze(captured);}

/** 📎️ Distinguishes retained readers from an actual exclusive maintenance result. */
export type NativeImageMaintenanceResultV1<T>=Readonly<{acquired:false}|{acquired:true;value:T}>;
/** 🛂️ Refuses undeclared image maintenance authority before releasing a reader. */
export function readNativeImageMaintenanceRequestV1(value:unknown):NativeImageMaintenanceRequestV1{if(validateJsonSchemaSubset(schema.definitions.ImageMaintenanceRequest,value,schema).length)throw Error("Closed original image maintenance request required");return Object.freeze({version:1,wait:(value as NativeImageMaintenanceRequestV1).wait});}

/** 🔐️ Distinguishes an actual retained kernel writer from a fresh private physical owner. */
export type NativeImageMaintenanceCustodyV1=Readonly<{kind:"kernel-ex";proof:ResourceLeaseWriterProofV1;control:ResourceLeaseControl}|{kind:"fresh-private"}>;

/** 💾️ Retains one enforcing native fixed-volume identity and original usage custody. */
export interface NativeImageStorageV1{
 readonly root:string;
 readonly image:string;
 readonly maximumPhysicalBytes:number;
 readonly filesystemBytes:number;
 readonly port:NativeStoragePortV1;
 recheck():Promise<void>;
 close():Promise<void>;
 withExclusiveMaintenance<T>(request:NativeImageMaintenanceRequestV1,body:(port:NativeStoragePortV1,custody:NativeImageMaintenanceCustodyV1)=>Promise<T>):Promise<NativeImageMaintenanceResultV1<T>>;
}

/** 🧭️ Resolves the compact canonical mount namespace without a path alias. */
export function nativeImageRootV1(directory:string):string{return requireMaterializedPathV1(join(directory,protocol.mountDirectory));}

/** 📂️ Rejects symlinks and paths escaping the enforced filesystem before materialization. */
export function requireNativeStoragePathV1(root:string,path:string):string{
 root=resolve(root);path=requireMaterializedPathV1(path);const suffix=relative(root,path);if(suffix===".."||suffix.startsWith(".."+sep)||isAbsolute(suffix))throw Error("Native storage path escapes enforcing boundary");
 const owner=lstatSync(root);if(!owner.isDirectory()||owner.isSymbolicLink())throw Error("Native storage root identity changed");const device=owner.dev,parts=suffix.split(/[\\/]/u).filter(Boolean);
 let current=root;for(const[index,part]of parts.entries()){current=join(current,part);let status;try{status=lstatSync(current);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(status.isSymbolicLink())throw Error("Native storage symlink escapes enforcing boundary");if(status.dev!==device)throw Error("Native storage path crosses filesystem boundary");if(index<parts.length-1&&!status.isDirectory())throw Error("Native storage ancestor must be an original directory");}return path;
}


/** 📏️ Refuses missing, mutable or non-finite physical envelopes before image or lease side effects. */
export function requireNativeImageEnvelopeV1(options:Readonly<{filesystemBytes:number;maximumPhysicalBytes:number}>):void{if(validateJsonSchemaSubset(schema.definitions.ImageRequest,{filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes},schema).length||options.filesystemBytes%protocol.unitBytes!==0||options.maximumPhysicalBytes<options.filesystemBytes+protocol.unitBytes)throw Error("Invalid fixed image allocation envelope");}


/** 📁️ Captures only a finite direct inventory without materializing an unbounded directory. */
export function nativeImageDirectoryNamesV1(path:string,maximum:number):readonly string[]{const before=lstatSync(path,{bigint:true});if(!Number.isSafeInteger(maximum)||maximum<0||maximum>4||!before.isDirectory()||before.isSymbolicLink())throw Error("Bounded physical Native directory required");const directory=opendirSync(path),names:string[]=[];let failed=false;try{for(;;){const row=directory.readSync();if(!row)break;if(names.length>=maximum)throw Error("Native directory inventory exceeds its owned bound");names.push(row.name);}const after=lstatSync(path,{bigint:true});if(after.isSymbolicLink()||after.dev!==before.dev||after.ino!==before.ino)throw Error("Native directory inventory changed");return names;}catch(error){failed=true;throw error;}finally{try{directory.closeSync();}catch(error){if(!failed)throw error;}}}

/** 🚧️ Keeps every pending publication invisible to ordinary readers, including unsafe namespace shapes. */
export function requireNativeImagePublicationSettledV1(directory:string):void{const path=requireMaterializedPathV1(join(directory,protocol.publicationDirectory));try{lstatSync(path);}catch(error){if(error&&typeof error==="object"&&"code"in error&&error.code==="ENOENT")return;throw error;}throw Error("Pending Native image publication must settle before ordinary admission");}
