import {createHash} from "node:crypto";
import {lstatSync,openSync,fstatSync,readSync,closeSync,constants,type BigIntStats} from "node:fs";
import {dirname,join} from "node:path";
import type {ScriptInvocation} from "../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {nativeImageRootV1,nativeImageBackingFilenameV1,readNativeProtectedMaintenanceRequestV1,nativeProtectedMaintenanceProtocolV1,type NativeProtectedMaintenanceRequestV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {nativeStorageBackendsV1,nativeStorageBackendOfDirectoryV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";
import {nativeImageBackingAllocatedBytesV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {repositoryCacheImagePublicationOptionsV1} from "../🟦️.ts";
import observationSchema from "./🧬️schema/🔣️.json" with {type:"json"};

const observationProtocol=observationSchema.$defs.Protocol.const;
type Options=ReturnType<typeof repositoryCacheImagePublicationOptionsV1>;

/** 📸️ Captures fresh parent expectation DATA from the genuine setup declaration; the later native owner must reacquire actual EX and validate it. */
export async function observeRepositoryDependencyMaintenanceParentV1(invocation:ScriptInvocation,control:ResourceLeaseControl):Promise<Readonly<{options:Options;parent:NativeProtectedMaintenanceRequestV1}>>{
 const options=repositoryCacheImagePublicationOptionsV1(invocation,control,"dependencies"),check=()=>{checkResourceLeaseControl(control);control.signal.throwIfAborted();},started=performance.now();check();
 const before=captureRepositoryDependencyParentObservationV1(options,check);await control.publish({resource:observationProtocol.progressResource,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();check();
 const after=captureRepositoryDependencyParentObservationV1(options,check);if(after.signature!==before.signature)throw Error("Dependency maintenance parent observation changed");check();return Object.freeze({options,parent:after.parent});
}

/** 🗂️ Selects exactly one existing declared backing; absence and ambiguity never fall back to a host choice. */
function selectObservedRepositoryDependencyBackingV1(names:readonly string[]){
 if(!names.length||new Set(names).size!==names.length||names.some(name=>!nativeStorageBackendsV1.some(row=>row.backingFilename===name)))throw Error("Exact known existing dependency backing required");
 const selected=nativeStorageBackendsV1.filter(row=>names.includes(row.backingFilename));if(selected.length!==1)throw Error("Unambiguous actual dependency backing required");return selected[0]!;
}

/** 📋️ Closes physical observation DATA before producing the native maintenance request, without granting any parent authority. */
function readObservedRepositoryDependencyParentV1(value:unknown):NativeProtectedMaintenanceRequestV1{
 if(validateJsonSchemaSubset(observationSchema as never,value).length)throw Error("Closed bounded dependency parent observation required");
 const row=value as{directory:{dev:number;ino:number;mode:number};backing:{dev:number;ino:number;size:number;allocatedBytes:number};root:{dev:number;ino:number;mode:number};receiptSha256:string;maximumPhysicalBytes:number};
 if(row.backing.size>row.maximumPhysicalBytes||row.backing.allocatedBytes>row.maximumPhysicalBytes)throw Error("Actual dependency backing exceeds declared ceiling");
 const identity=(facts:{dev:number;ino:number;mode:number})=>({dev:facts.dev,ino:facts.ino,mode:facts.mode});
 return readNativeProtectedMaintenanceRequestV1({version:1,purpose:"mounted-parent-maintenance",wait:false,receiptSha256:row.receiptSha256,directory:identity(row.directory),backing:{dev:row.backing.dev,ino:row.backing.ino,size:row.backing.size,allocatedBytes:row.backing.allocatedBytes},root:identity(row.root)});
}

/** 🔢️ Refuses lossy physical identity conversion before entering the number-based native wire. */
function observedRepositoryDependencyNumberV1(value:bigint):number{const result=Number(value);if(!Number.isSafeInteger(result)||result<0||BigInt(result)!==value)throw Error("Exact safe dependency physical identity required");return result;}

/** 🪪️ Records the nofollow identities relevant to a stable physical observation. */
function observedRepositoryDependencyStampV1(row:BigIntStats):string{return JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String));}

/** 🪨️ Fences the physical identity carried by the native wire while leaving payload event observation to the genuine owner. */
function observedRepositoryDependencyPhysicalStampV1(row:BigIntStats,backing=false):string{return JSON.stringify((backing?[row.dev,row.ino,row.mode,row.nlink,row.size,nativeImageBackingAllocatedBytesV1(row)]:[row.dev,row.ino,row.mode]).map(String));}

/** 📄️ Reads at most the native receipt bound through one pinned nofollow descriptor and preserves close failures. */
function readObservedRepositoryDependencyReceiptV1(path:string,before:BigIntStats,check:()=>void):Buffer{
 const maximum=nativeProtectedMaintenanceProtocolV1().maximumReceiptBytes;if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<BigInt(observationProtocol.receiptMinimumBytes)||before.size>BigInt(maximum))throw Error("Bounded single-owner dependency receipt required");
 const failures:unknown[]=[];let descriptor:number|undefined,result:Buffer|undefined;
 try{check();descriptor=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));if(observedRepositoryDependencyStampV1(fstatSync(descriptor,{bigint:true}))!==observedRepositoryDependencyStampV1(before))throw Error("Pinned dependency receipt changed");const bytes=Buffer.alloc(Number(before.size)+1);let offset=0;for(;;){check();const count=readSync(descriptor,bytes,offset,bytes.length-offset,offset);if(!count)break;offset+=count;if(offset>Number(before.size))throw Error("Dependency receipt grew during observation");}check();if(offset!==Number(before.size)||observedRepositoryDependencyStampV1(fstatSync(descriptor,{bigint:true}))!==observedRepositoryDependencyStampV1(before)||observedRepositoryDependencyStampV1(lstatSync(path,{bigint:true}))!==observedRepositoryDependencyStampV1(before))throw Error("Exact dependency receipt observation changed");result=Buffer.from(bytes.subarray(0,offset));}catch(error){failures.push(error);}
 if(descriptor!==undefined)try{closeSync(descriptor);}catch(error){failures.push(error);}if(failures.length>1)throw new AggregateError(failures,"Dependency receipt observation and close failed");if(failures.length)throw failures[0];return result!;
}

/** 🧿️ Captures and fences actual backing, mounted root, receipt and ordinary ancestor identities without opening the persistent receiver. */
function captureRepositoryDependencyParentObservationV1(options:Options,check:()=>void):Readonly<{parent:NativeProtectedMaintenanceRequestV1;signature:string}>{
 check();const paths=[options.directory,nativeImageRootV1(options.directory),join(options.directory,observationProtocol.receiptName),...nativeStorageBackendsV1.map(row=>join(options.directory,nativeImageBackingFilenameV1(row.id)))];if(paths.some(path=>path.length>observationProtocol.maximumPathUnits))throw Error("Bounded dependency parent observation paths required");
 const ancestors:Array<{path:string;stamp:string}>=[];for(let path=options.directory;;path=dirname(path)){check();const facts=lstatSync(path,{bigint:true});if(!facts.isDirectory()||facts.isSymbolicLink()||ancestors.length>=observationProtocol.maximumAncestors)throw Error("Ordinary bounded dependency parent ancestry required");ancestors.push({path,stamp:observedRepositoryDependencyPhysicalStampV1(facts)});if(dirname(path)===path)break;}
 const backings=nativeStorageBackendsV1.map(row=>{check();const path=join(options.directory,nativeImageBackingFilenameV1(row.id));try{return{row,path,facts:lstatSync(path,{bigint:true})};}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{row,path,facts:null};throw error;}}),selected=selectObservedRepositoryDependencyBackingV1(backings.filter(row=>row.facts!==null).map(row=>row.row.backingFilename));if(nativeStorageBackendOfDirectoryV1(options.directory).id!==selected.id)throw Error("Current dependency backing selection changed");
 const backing=backings.find(row=>row.row.id===selected.id)!,folder=lstatSync(options.directory,{bigint:true}),mounted=lstatSync(nativeImageRootV1(options.directory),{bigint:true}),receiptPath=join(options.directory,observationProtocol.receiptName),receipt=lstatSync(receiptPath,{bigint:true}),bytes=readObservedRepositoryDependencyReceiptV1(receiptPath,receipt,check),number=observedRepositoryDependencyNumberV1,identity=(row:BigIntStats)=>({dev:number(row.dev),ino:number(row.ino),mode:number(row.mode),kind:row.isSymbolicLink()?"link":row.isDirectory()?"directory":row.isFile()?"file":"other"}),file=backing.facts!;
 const parent=readObservedRepositoryDependencyParentV1({version:1,directory:identity(folder),backing:{...identity(file),nlink:number(file.nlink),size:number(file.size),allocatedBytes:number(nativeImageBackingAllocatedBytesV1(file))},root:identity(mounted),receiptSha256:createHash("sha256").update(bytes).digest("hex"),maximumPhysicalBytes:options.maximumPhysicalBytes});
 for(const before of ancestors){check();if(observedRepositoryDependencyPhysicalStampV1(lstatSync(before.path,{bigint:true}))!==before.stamp)throw Error("Dependency parent ancestry changed");}
 for(const before of backings){check();let after:BigIntStats|null;try{after=lstatSync(before.path,{bigint:true});}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;after=null;}if((before.facts===null)!==(after===null)||before.facts&&after&&observedRepositoryDependencyPhysicalStampV1(before.facts,true)!==observedRepositoryDependencyPhysicalStampV1(after,true))throw Error("Dependency backing roster changed");}
 if(observedRepositoryDependencyPhysicalStampV1(lstatSync(nativeImageRootV1(options.directory),{bigint:true}))!==observedRepositoryDependencyPhysicalStampV1(mounted)||observedRepositoryDependencyStampV1(lstatSync(receiptPath,{bigint:true}))!==observedRepositoryDependencyStampV1(receipt)||nativeStorageBackendOfDirectoryV1(options.directory).id!==selected.id)throw Error("Dependency root or receipt changed");check();return Object.freeze({parent,signature:JSON.stringify([parent,ancestors,selected.id,observedRepositoryDependencyPhysicalStampV1(file,true),observedRepositoryDependencyPhysicalStampV1(mounted),observedRepositoryDependencyStampV1(receipt)])});
}
