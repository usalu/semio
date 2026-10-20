import contract from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=contract.properties.policy.const;
/** 🪪️Exact predecessor identity and UTF-8 text owned by a filesystem port. */
export type Revision=Readonly<{identity:string;text:string}>;
/** 🌳️One physical direct child of a source directory. */
export type Entry=Readonly<{path:string;name:string;directory:boolean}>;
/** 📄️An immutable document observation. */
export type Document=Readonly<{path:string;text:string;revision:Revision}>;
/** 📈️Cancellable source work without invented totals. */
export type Progress=Readonly<{stage:"listing"|"reading"|"staging"|"checking"|"publishing"|"complete";completed:number;total?:number}>;
/** 🔌️Owned host boundary; publication atomically retains its physical predecessor. */
export interface WorkspacePort {
 list(path:string):Promise<readonly Entry[]>;
 read(path:string):Promise<Revision>;
 stage(path:string,text:string):Promise<string>;
 publish(path:string,stage:string):Promise<Readonly<{predecessor:Revision;current:Revision;recoveryPath:string}>>;
 record(event:Readonly<{kind:"save-intent"|"save-committed"|"save-conflict";path:string;stage:string}>):Promise<void>;
 retire(stage:string):Promise<void>;
}
/** 🚦️Source failures expose owned categories rather than host library types. */
export class WorkspaceError extends Error {
 constructor(readonly code:string,readonly path:string,readonly recoveryPath?:string){super(`${code}: ${path}`);}
}
/** 🧭️Uses one canonical portable relative spelling. */
export function validateRelative(path:string):void {
 if(!path||path.length>policy.maximumPathUnits||/[\p{Cc}\\:*?"<>|]/u.test(path)||path.split("/").some(part=>!part||[".","..",policy.stateDirectory].includes(part)||/[. ]$/u.test(part)||/^(?:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|$)/iu.test(part)))throw new WorkspaceError("invalid-path",path);
}
/** 🔤️Preserves exact line endings and refuses binary or oversized source. */
export function validateText(text:string):void {
 if(Buffer.byteLength(text,"utf8")>policy.maximumFileBytes)throw new WorkspaceError("too-large","");
 if(text.includes("\0")||new TextDecoder("utf-8",{fatal:true}).decode(new TextEncoder().encode(text))!==text)throw new WorkspaceError("not-text","");
}
const equal=(a:Revision,b:Revision):boolean=>a.identity===b.identity&&a.text===b.text;

/** 📝️The second source-command implementation, separated from any host tool or library. */
export class Workspace {
 private readonly revisions=new WeakMap<Revision,string>();
 constructor(private readonly port:WorkspacePort){}
 private check(path:string,signal:AbortSignal):void {if(signal.aborted)throw new WorkspaceError("cancelled",path);}
 async list(path:string,signal:AbortSignal,progress:(value:Progress)=>void):Promise<readonly Entry[]> {
  this.check(path,signal);if(path)validateRelative(path);const rows=await this.port.list(path);this.check(path,signal);if(rows.length>policy.maximumDirectoryEntries)throw new WorkspaceError("too-many-entries",path);
  for(let index=0;index<rows.length;index++){this.check(path,signal);validateRelative(rows[index]!.path);progress({stage:"listing",completed:index+1});}
  return [...rows].sort((a,b)=>Number(b.directory)-Number(a.directory)||Buffer.compare(Buffer.from(a.name),Buffer.from(b.name)));
 }
 async load(path:string,signal:AbortSignal,progress:(value:Progress)=>void):Promise<Document> {
  this.check(path,signal);validateRelative(path);const source=await this.port.read(path);this.check(path,signal);validateText(source.text);const revision=Object.freeze({...source});this.revisions.set(revision,path);progress({stage:"reading",completed:Buffer.byteLength(revision.text),total:Buffer.byteLength(revision.text)});return Object.freeze({path,text:revision.text,revision});
 }
 async save(path:string,expected:Revision,text:string,signal:AbortSignal,progress:(value:Progress)=>void):Promise<Document> {
  this.check(path,signal);validateRelative(path);validateText(text);if(this.revisions.get(expected)!==path||!equal(await this.port.read(path),expected))throw new WorkspaceError("conflict",path);this.check(path,signal);
  const stage=await this.port.stage(path,text);let publishing=false,recoveryPath:string|undefined;
  try{
   progress({stage:"staging",completed:Buffer.byteLength(text),total:Buffer.byteLength(text)});this.check(path,signal);await this.port.record({kind:"save-intent",path,stage});progress({stage:"checking",completed:0,total:1});this.check(path,signal);
   if(!equal(await this.port.read(path),expected))throw new WorkspaceError("conflict",path);progress({stage:"publishing",completed:0,total:1});this.check(path,signal);if(!equal(await this.port.read(path),expected))throw new WorkspaceError("conflict",path);this.check(path,signal);
   publishing=true;const result=await this.port.publish(path,stage);recoveryPath=result.recoveryPath;const matches=equal(result.predecessor,expected);await this.port.record({kind:matches?"save-committed":"save-conflict",path,stage});if(!matches)throw new WorkspaceError("conflict",path,result.recoveryPath);
   if(result.current.text!==text)throw new WorkspaceError("conflict",path,result.recoveryPath);const revision=Object.freeze({...result.current});this.revisions.set(revision,path);progress({stage:"complete",completed:1,total:1});return Object.freeze({path,text:revision.text,revision});
  }catch(error){if(recoveryPath&&!(error instanceof WorkspaceError&&error.recoveryPath))throw new WorkspaceError("io",path,recoveryPath);throw error;}finally{if(!publishing)await this.port.retire(stage);}
 }
}
