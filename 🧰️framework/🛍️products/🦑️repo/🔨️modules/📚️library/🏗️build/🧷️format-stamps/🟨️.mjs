import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import schemaContract from"./🧬️schema/🔣️.json"with{type:"json"};
import{createHash}from"node:crypto";
import{closeSync,constants,fstatSync,openSync,readSync,writeSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{abortLeaseRootV1,registerLeaseImplementationProviderV1,registerLeaseYieldHandlerV1}from"../../../../../../🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts";
import{physicalSourceOriginV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
import{STALE_IMPLEMENTATION_CODE_V1,STALE_IMPLEMENTATION_CODES_V1,churnReasonTextV1,concurrentEditOfV1,installRestartableFailureHandlerV1}from"../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
const protocol=JSON.parse(JSON.stringify(schemaContract.$defs.Protocol.const));
const staleCode=STALE_IMPLEMENTATION_CODE_V1,tableCode="SEMIO_STAMP_TABLE_INVALID";
/** 📖️ The closed protocol of the stamp table exactly as the schema declares it. */
export function formatStampProtocolV1(){return JSON.parse(JSON.stringify(protocol));}
/** 🌐️ The languages a message is rendered in: the one the language protocol chooses, both declared languages when it chooses none. */
export function formatStampLanguagesV1(environment=process.env){
 return [...noticeLanguagesV1(protocol.languages,environment)];
}
/** 💬️ Renders one declared message in one language; every placeholder must be given. */
export function formatStampTextV1(language,key,values={}){
 const table=protocol.texts[language];if(!table||typeof table[key]!=="string")throw Error("Declared stamp text required: "+language+"."+key);
 return table[key].replace(/\{([a-z]+)\}/gu,(_,name)=>{if(!Object.hasOwn(values,name))throw Error("Stamp text placeholder required: "+name);return String(values[name]);});
}
/** 📣️ The localized report of moved stamps: one line per moved stamp (bounded), then what the process does with the resource. */
export function renderStaleImplementationV1(languages,moved,context="",kind="yield"){
 const shown=moved.slice(0,protocol.maximumReportedStamps),hidden=moved.length-shown.length;
 return languages.map(language=>[...shown.map(row=>formatStampTextV1(language,"moved",row)),...(hidden>0?[formatStampTextV1(language,"more",{count:hidden})]:[]),formatStampTextV1(language,kind,{resource:context||"-"})].join(" ")).join("\n");
}
/** 🚧️ Raised when the declared stamps on disk differ from the stamps of the loaded implementation: the holder must yield without producing. */
export class StaleImplementationErrorV1 extends Error{
 constructor(moved,{context="",kind="yield",languages=protocol.languages}={}){
  super(renderStaleImplementationV1(languages,moved,context,kind));
  this.name="StaleImplementationErrorV1";this.code=staleCode;this.moved=Object.freeze(moved.map(row=>Object.freeze({...row})));this.context=context;this.kind=kind;
 }
}
const sorted=value=>Array.isArray(value)?value.map(sorted):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,sorted(value[key])])):value;
const canonical=value=>JSON.stringify(sorted(value));
const isMap=value=>value!==null&&typeof value==="object"&&!Array.isArray(value);
const scalar=value=>typeof value==="string"?value:canonical(value);
function differences(id,own,live){
 if(isMap(own)&&isMap(live)){
  const keys=[...new Set([...Object.keys(own),...Object.keys(live)])].sort();
  return keys.filter(key=>canonical(own[key])!==canonical(live[key])).map(key=>({id:id+"."+key,own:own[key]===undefined?"-":scalar(own[key]),live:live[key]===undefined?"-":scalar(live[key])}));
 }
 return canonical(own)===canonical(live)?[]:[{id,own:scalar(own),live:scalar(live)}];
}
const invalid=(languages,key,values)=>Object.assign(Error(languages.map(language=>formatStampTextV1(language,key,values)).join("\n")),{code:tableCode});
function extract(entry,document,languages,path){
 let value=document;
 for(const part of entry.pointer.split("/").slice(1).map(part=>part.replace(/~1/gu,"/").replace(/~0/gu,"~"))){
  if(value===null||typeof value!=="object"||!Object.hasOwn(value,part))throw invalid(languages,"missing",{id:entry.id,pointer:entry.pointer,path});
  value=value[part];
 }
 if(entry.member===undefined)return value;
 if(!isMap(value))throw invalid(languages,"missing",{id:entry.id,pointer:entry.pointer,path});
 return Object.fromEntries(Object.entries(value).map(([key,row])=>[key,row?.[entry.member]]));
}
const pause=milliseconds=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};
function liveJsonReader(limits){
 const cache=new Map();
 const read=path=>{
  const descriptor=openSync(path,constants.O_RDONLY);
  try{
   const row=fstatSync(descriptor,{bigint:true}),key=[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":"),cached=cache.get(path),settled=Date.now()-Number(row.mtimeNs/1000000n)>limits.settleMilliseconds;
   if(cached?.key===key)return cached.document;
   if(row.size>BigInt(limits.maximumSchemaBytes))throw Error("Bounded stamp schema required");
   const bytes=Buffer.alloc(Number(row.size));let offset=0;
   while(offset<bytes.length){const count=readSync(descriptor,bytes,offset,bytes.length-offset,offset);if(!count)break;offset+=count;}
   if(offset!==bytes.length)throw Error("Stamp schema shortened during read");
   const document=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes));if(settled)cache.set(path,{key,document});else cache.delete(path);return document;
  }finally{closeSync(descriptor);}
 };
 return path=>{for(let attempt=1;;attempt++){try{return read(path);}catch(error){if(attempt>=limits.readAttempts)throw error;pause(limits.readRetryMilliseconds);}}};
}
/** 🔖️ Creates one stamp registry over a workspace library root: processes declare the stamps they loaded, the registry compares them with the table declared on disk now. */
export function createFormatStampsV1({base,table=protocol,environment=process.env,read}={}){
 if(typeof base!=="string"||!base)throw Error("Library root of the stamp table required");
 const entries=new Map(table.stamps.map(entry=>[entry.id,entry])),owned=new Map(),reader=read??liveJsonReader(table);
 const entryOf=id=>{const entry=entries.get(id);if(!entry)throw invalid(formatStampLanguagesV1(environment),"unknown",{id});return entry;};
 const live=id=>{
  const entry=entryOf(id),path=join(base,...entry.schema.split("/"));let document;
  try{document=reader(path);}catch(error){throw invalid(formatStampLanguagesV1(environment),"unreadable",{path,reason:String(error?.message??error)});}
  return extract(entry,document,formatStampLanguagesV1(environment),path);
 };
 const moved=ids=>[...ids].filter(id=>owned.has(id)).flatMap(id=>differences(id,owned.get(id),live(id)));
 const ensureCurrent=(ids=[...owned.keys()],context="",kind="yield")=>{const rows=moved(ids);if(rows.length)throw new StaleImplementationErrorV1(rows,{context,kind,languages:formatStampLanguagesV1(environment)});};
 const idsOf=resource=>[...new Set(table.resources.filter(row=>resource.startsWith(row.prefix)).flatMap(row=>row.stamps))].filter(id=>owned.has(id));
 const guard=admission=>{if(admission.mode!=="exclusive"||typeof admission.resource!=="string")return;const ids=idsOf(admission.resource);if(ids.length)ensureCurrent(ids,admission.resource,"yield");};
 return Object.freeze({declare(id,document){owned.set(id,extract(entryOf(id),document,formatStampLanguagesV1(environment),"declared document"));},owned:()=>new Map(owned),live,moved,ensureCurrent,idsOf,guard});
}
/** 🛡️ Registers a registry as an admission guard of the lease layer: it runs for every granted exclusive lease and every wait round of a queued exclusive request. */
export function registerFormatStampGuardV1(registry,target=globalThis){
 const key=Symbol.for(protocol.admissionGuardRegistry),guards=target[key]??=new Set();guards.add(registry.guard);
 return()=>guards.delete(registry.guard);
}
/** 🧱️ Whether an error is the load failure of a module another process was editing: a file that no longer parses, resolves or exports what its importer names. The next process loads the finished edit. A failure aggregated by a bundler (`Bun.build`) is a product Source problem, never this class. */
export function isHalfEditedModuleLoadV1(error){
 if(error===null||typeof error!=="object")return false;
 const policy=protocol.loadFailure;
 return policy.names.includes(error.name)||error.name==="SyntaxError"&&String(error.message).startsWith(policy.syntaxErrorPrefix);
}
/** 🔎️ Finds a stale-implementation, half-edited-module or escaped concurrent-edit failure in an error, its causes and aggregated errors, also when only the exit code of a captured child crossed the process boundary. */
export function staleImplementationOfV1(error){
 const seen=new Set(),walk=(value,depth,bundled=false)=>{
  if(value===null||typeof value!=="object"||seen.has(value)||depth>8)return undefined;seen.add(value);
  if(STALE_IMPLEMENTATION_CODES_V1.has(value.code))return{origin:"error",moved:value.moved??[],message:String(value.message)};
  if(!bundled&&isHalfEditedModuleLoadV1(value))return{origin:"load",moved:[],message:String(value.message).split("\n")[0].slice(0,300)+(typeof value.position?.file==="string"?" ("+value.position.file+(Number.isInteger(value.position.line)?":"+value.position.line:"")+")":"")};
  if(value.name==="OwnedCommandFailure"&&value.outcome?.status===protocol.exitCode)return{origin:"exit",moved:[],message:""};
  for(const inner of[value.cause,...(Array.isArray(value.errors)?value.errors:[])]){const found=walk(inner,depth+1,bundled||Array.isArray(value.errors));if(found)return found;}
  return undefined;
 };
 const found=walk(error,0);if(found)return found;
 const edit=concurrentEditOfV1(error);if(edit===undefined)return undefined;
 return{origin:"edit",moved:[],message:String(error?.message??error).split("\n")[0].slice(0,300),edit};
}
/** 💬️ The announcement of a restart after a concurrent edit escaped every layer of the process, one text per selected language, each with the wording of the reason in that language. */
export function renderEditRestartV1(languages,edit,values){return languages.map(language=>formatStampTextV1(language,"editfailure",{path:edit.path,reason:churnReasonTextV1(language,edit.reason),...values})).join("\n");}
/** 🚪️ Ends a captured command process with the declared exit code when its failure is a stale implementation, a half-edited module or a concurrent edit that escaped every layer (that one first waits the declared pause, so the restart finds the edit settled); any other failure is left to the caller. */
export function exitIfStaleImplementationV1(error,{write=text=>{try{writeSync(2,text);}catch{}},exit=code=>process.exit(code),sleepSync=pause,environment=process.env}={}){
 const stale=staleImplementationOfV1(error);if(!stale)return;
 if(stale.origin==="edit"){const attempt=Number.parseInt(environment[protocol.reexecutionEnvironment]??"0",10)||0,delays=protocol.editFailure.backoffMs,delay=delays[Math.min(attempt,delays.length-1)];write(renderEditRestartV1(formatStampLanguagesV1(environment),stale.edit,{delay,attempt:attempt+1,maximum:protocol.maximumReexecutions})+"\n");sleepSync(delay);}
 else if(stale.origin!=="exit")write(stale.message+"\n");
 exit(protocol.exitCode);
}
installRestartableFailureHandlerV1(error=>exitIfStaleImplementationV1(error));
const origin=physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL);
/** 🏷️ The registry of this process, rooted at the live library tree; it guards the exclusive resources it has declared stamps for. */
export const formatStampsV1=createFormatStampsV1({base:fileURLToPath(new URL("../../",origin))});
registerFormatStampGuardV1(formatStampsV1);
/** 🖊️ Declares the stamp of the implementation this process loaded from its own imported schema document. */
export function declareFormatStampV1(id,document){formatStampsV1.declare(id,document);}
/** ✅️ Throws a stale-implementation error when any of the declared stamps moved since this process loaded them. */
export function requireCurrentFormatStampsV1(ids,context,kind){formatStampsV1.ensureCurrent(ids,context,kind);}
registerLeaseImplementationProviderV1({components:()=>Object.fromEntries([...formatStampsV1.owned()].map(([id,value])=>[id,createHash("sha256").update(canonical(value)).digest("hex").slice(0,12)])),bound:resource=>formatStampsV1.idsOf(resource),moved:resource=>formatStampsV1.idsOf(resource).filter(id=>formatStampsV1.moved([id]).length>0)});
/** 🤝️ A holder asked to give a resource up ends the way an outdated implementation ends, whatever the reason: the outermost command starts again (exit 75) with the current code, and a stuck holder (no output and an idle process tree for the declared time) is restarted the same way instead of failing for the person who ran it; the request's message names why. */
registerLeaseYieldHandlerV1(request=>{
 const error=new StaleImplementationErrorV1([],{context:request.resource,kind:"yield"});error.message=request.message;if(!abortLeaseRootV1(error))exitIfStaleImplementationV1(error);
});
