import{accessSync,chmodSync,constants,lstatSync,mkdirSync,mkdtempSync,opendirSync,rmSync,unlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
const protocol=schema.definitions.Protocol.const,logPattern=new RegExp(protocol.logPattern,"u");
/** 🤫️ Names the closed observable state of the macOS volume services that must stay silent on one mounted image. */
export type NativeImageVolumeSilenceFactsV1=Readonly<{directory:"absent"|"directory"|"other";writable:boolean;marker:"absent"|"file"|"other";spotlightMarker:"absent"|"file"|"other";logFiles:number;logBytes:number}>;
/** 🎯️ Distinguishes the fresh attach, which prunes every log, from an open of an already mounted image, which tolerates a small residue. */
export type NativeImageVolumeSilencePhaseV1="attach"|"open";
/** 🧭️ Names one effect the policy may apply under the partition lease. */
export type NativeImageVolumeSilenceActionV1="create-directory"|"write-marker"|"write-spotlight-marker"|"prune-logs";
/** 🚫️ Names why the policy cannot be established by this process. */
export type NativeImageVolumeSilenceRefusalV1="unsafe-service-directory"|"unsafe-marker"|"foreign-service-directory";
/** 📋️ Holds the pure decision over observed facts. */
export type NativeImageVolumeSilenceDecisionV1=Readonly<{actions:readonly NativeImageVolumeSilenceActionV1[];refusal:NativeImageVolumeSilenceRefusalV1|null}>;
/** 🔌️ Supplies cancellation and progress to the bounded observation and pruning. */
export type NativeImageVolumeSilenceHooksV1=Readonly<{check:()=>void;progress?:(done:number,total:number)=>void}>;
const kind=(path:string):"absent"|"file"|"directory"|"other"=>{try{const row=lstatSync(path);return row.isSymbolicLink()?"other":row.isFile()&&row.nlink===1?"file":row.isDirectory()?"directory":"other";}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return"absent";throw error;}};
const entry=(path:string)=>{const value=kind(path);return value==="directory"?"other":value;};
const logNames=(directory:string,check:()=>void):string[]=>{const stream=opendirSync(directory),names:string[]=[];let count=0;try{for(;;){const row=stream.readSync();if(!row)break;if(++count>protocol.maximumEntries)throw Error("Native volume service entry bound exceeded");if(logPattern.test(row.name))names.push(row.name);if(count%protocol.pruneBatch===0)check();}}finally{stream.closeSync();}return names.sort();};
/** 🔎️ Reads the service directory, markers and log residue of one mounted image without following links. */
export function observeNativeImageVolumeSilenceV1(root:string,check:()=>void=()=>{}):NativeImageVolumeSilenceFactsV1{
 const directory=join(root,protocol.serviceDirectory),state=kind(directory),shape=state==="file"?"other":state;
 const anchor=shape==="directory"?lstatSync(directory,{bigint:true}):null;if(anchor&&!anchor.isDirectory())throw Error("Native image service directory changed during observation");
 let writable=false,logFiles=0,logBytes=0,marker:"absent"|"file"|"other"="absent";
 if(shape==="directory"){try{accessSync(directory,constants.R_OK|constants.W_OK|constants.X_OK);writable=true;}catch{writable=false;}
  try{marker=entry(join(directory,protocol.serviceMarker));for(const name of logNames(directory,check)){try{const row=lstatSync(join(directory,name));if(row.isFile()&&row.nlink===1){logFiles++;logBytes+=row.size;}}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;let current;try{current=lstatSync(directory,{bigint:true,throwIfNoEntry:false});}catch(cause){throw new AggregateError([error,cause],"Native image service directory identity could not be retained");}if(!current?.isDirectory()||current.dev!==anchor!.dev||current.ino!==anchor!.ino)throw error;}}}catch(error){if((error as NodeJS.ErrnoException).code!=="EACCES")throw error;}}
 return Object.freeze({directory:shape,writable,marker,spotlightMarker:entry(join(root,protocol.spotlightMarker)),logFiles,logBytes});
}
/** ⚖️ Decides the exact effects that establish the silence policy; pure and language-agnostic. */
export function decideNativeImageVolumeSilenceV1(facts:NativeImageVolumeSilenceFactsV1,phase:NativeImageVolumeSilencePhaseV1):NativeImageVolumeSilenceDecisionV1{
 if(facts.directory==="other")return Object.freeze({actions:Object.freeze([]),refusal:"unsafe-service-directory"});
 if(facts.marker==="other"||facts.spotlightMarker==="other")return Object.freeze({actions:Object.freeze([]),refusal:"unsafe-marker"});
 const actions:NativeImageVolumeSilenceActionV1[]=[];
 if(facts.directory==="absent")actions.push("create-directory");
 if(facts.marker!=="file")actions.push("write-marker");
 if(facts.spotlightMarker!=="file")actions.push("write-spotlight-marker");
 if(phase==="attach"?facts.logFiles>0:facts.logFiles>protocol.residualLogFiles||facts.logBytes>protocol.residualLogBytes)actions.push("prune-logs");
 if(facts.directory==="directory"&&!facts.writable&&(actions.includes("write-marker")||actions.includes("prune-logs")))return Object.freeze({actions:Object.freeze([]),refusal:"foreign-service-directory"});
 return Object.freeze({actions:Object.freeze(actions),refusal:null});
}
/** 🔕️ Reports whether the mounted image already satisfies the policy for the given phase. */
export function nativeImageVolumeSilencedV1(root:string,phase:NativeImageVolumeSilencePhaseV1,check:()=>void=()=>{}):boolean{const decision=decideNativeImageVolumeSilenceV1(observeNativeImageVolumeSilenceV1(root,check),phase);return!decision.actions.length&&decision.refusal===null;}
/** 🔇️ Reports whether the volume carries the regular single-link `no_log` marker that removes its macOS event service. */
export function hasNativeImageVolumeSilenceMarkerV1(root:string):boolean{return kind(join(root,protocol.serviceDirectory))==="directory"&&kind(join(root,protocol.serviceDirectory,protocol.serviceMarker))==="file";}
const refusalText=(refusal:NativeImageVolumeSilenceRefusalV1,root:string)=>refusal==="foreign-service-directory"?"Native image service directory at "+root+" is owned by another user and cannot be silenced; once, while no process uses it, run `/usr/bin/hdiutil detach "+root+"` and recreate the image":refusal==="unsafe-service-directory"?"Native image service directory at "+root+" must be a real directory":"Native image silence markers at "+root+" must be regular single-link files";
const markerFile=(path:string)=>{try{writeFileSync(path,"",{flag:"wx",mode:protocol.markerMode});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}};
/** 🤫️ Establishes and verifies the policy under the caller's partition lease; every effect is idempotent and touches only the service directory and one root marker, so racing readers stay safe. Prunes event logs in cancellable batches and refuses when the policy cannot be established. */
export async function applyNativeImageVolumeSilenceV1(root:string,phase:NativeImageVolumeSilencePhaseV1,hooks:NativeImageVolumeSilenceHooksV1):Promise<NativeImageVolumeSilenceFactsV1>{
 hooks.check();const decision=decideNativeImageVolumeSilenceV1(observeNativeImageVolumeSilenceV1(root,hooks.check),phase);
 if(decision.refusal)throw Error(refusalText(decision.refusal,root),{cause:Object.freeze({version:1,refusal:decision.refusal,root})});
 const directory=join(root,protocol.serviceDirectory);
 if(decision.actions.includes("create-directory")){mkdirSync(directory,{mode:protocol.serviceDirectoryMode});chmodSync(directory,protocol.serviceDirectoryMode);}
 if(decision.actions.includes("write-marker"))markerFile(join(directory,protocol.serviceMarker));
 if(decision.actions.includes("write-spotlight-marker"))markerFile(join(root,protocol.spotlightMarker));
 if(decision.actions.includes("prune-logs")){const names=logNames(directory,hooks.check);let done=0;for(const name of names){const path=join(directory,name);try{const row=lstatSync(path);if(row.isFile()&&row.nlink===1)unlinkSync(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}if(++done%protocol.pruneBatch===0){hooks.progress?.(done,names.length);hooks.check();await new Promise<void>(resolve=>setImmediate(resolve));}}hooks.progress?.(done,names.length);}
 hooks.check();const facts=observeNativeImageVolumeSilenceV1(root,hooks.check),after=decideNativeImageVolumeSilenceV1(facts,phase);
 if(after.actions.length||after.refusal)throw Error("Native image silence not established at "+root,{cause:Object.freeze({version:1,facts,actions:after.actions,refusal:after.refusal})});
 return facts;
}
/** 🌱️ Builds the private seed folder that carries both markers into a fresh volume before macOS first mounts it, because macOS then creates `.fseventsd` as root. */
export function createNativeImageVolumeSilenceSeedV1():Readonly<{directory:string;dispose:()=>void}>{
 const directory=mkdtempSync(join(tmpdir(),protocol.seedPrefix)),service=join(directory,protocol.serviceDirectory);
 try{mkdirSync(service,{mode:protocol.serviceDirectoryMode});chmodSync(service,protocol.serviceDirectoryMode);markerFile(join(service,protocol.serviceMarker));markerFile(join(directory,protocol.spotlightMarker));}catch(error){rmSync(directory,{recursive:true,force:true});throw error;}
 return Object.freeze({directory,dispose:()=>rmSync(directory,{recursive:true,force:true})});
}
/** 🧱️ Builds the closed `hdiutil create` argument list of one seeded fixed image. */
export function nativeImageSeededCreateArgumentsV1(filesystemBytes:number,image:string,seed:string):readonly string[]{
 const unit=imageSchema.definitions.ImageProtocol.const.unitBytes;
 if(!Number.isSafeInteger(filesystemBytes)||filesystemBytes<=0||filesystemBytes%unit!==0)throw Error("Unit-aligned Native image size required");
 return Object.freeze(["create","-size",String(filesystemBytes/unit)+"m","-format",protocol.seededFormat,"-nospotlight","-fs",imageSchema.definitions.ImageProtocol.const.filesystem,"-volname","semio","-srcfolder",seed,image]);
}
