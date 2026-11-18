import {createHash} from "node:crypto";
import {graphMemoryRefusalOfV1} from "../../../🧠️memory/🟨️.mjs";
import {homedir} from "node:os";
import {lstatSync,mkdirSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {SERVICE_ROUTES_V1,consentIdentityV1,protectedFolderOfV1} from "../../🔐️consent/🟨️.mjs";
import {diskFullOfV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {fullImageOfV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🖼️full/🟦️.ts";
import {DISK_RECLAIM_COMMAND_V1,formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";

const IMAGE_RECLAIM_COMMAND_V1="repo cache gc";
const FAILURE_MESSAGE_CHARACTERS_V1=300;
const FAILURE_FILES_KEPT_V1=16;

/** 📏️ The most bytes of one failure block in the store (a whole stack, both texts and the message fit well within it). */
export const FAILURE_BLOCK_BYTES_V1=65536;

/** 🪜️ The most bytes of stack frames one failure block keeps when the message is longer than the block leaves room for: the frames say where it was thrown, the message is cut instead. */
export const FAILURE_FRAME_BYTES_V1=16384;

/** 🥞️ The first stack frames of a failure (at most six lines of at most 200 characters, 1500 in all), one `at ...` line each after a line break, or an empty text when the failure has no stack: what a client reads of the cause without the service log. */
export function failureFramesV1(error){
 const lines=String(error?.stack??"").split("\n").map(line=>line.trim()).filter(line=>line.startsWith("at ")).slice(0,6).map(line=>"\n  "+line.slice(0,200));
 return lines.join("").slice(0,1500);
}

/** 💬️ The one-line message of a failure for the refusal text: its name, message and code on one line, at most 300 characters, so that a remote machine can read what failed without the service log. */
export function failureMessageV1(error){
 const text=String(error?.message??error).replace(/\s+/gu," ").trim(),code=typeof error?.code==="string"&&!text.includes(error.code)?" ["+error.code+"]":"",owner=typeof error?.semioProject==="string"&&!text.includes(error.semioProject)?" (project "+error.semioProject+")":"";
 return(String(error?.name??"Error")+code+owner+": "+text).slice(0,FAILURE_MESSAGE_CHARACTERS_V1);
}

/** 🔐️ Recognises the macOS file access consent: a relative filesystem call of the service that fails with `EPERM` while the worktree lies below Documents, Desktop or Downloads of the user is a process without the consent of this session (a service whose responsible process is not the launcher `repo`, or a `repo` whose binary was replaced), not a defect of the request. Returns the folder, or `undefined` for every other failure. */
export function accessDeniedOfV1(error,{root=process.cwd(),platform=process.platform,home=homedir()}={}){
 if(!(error?.code==="EPERM"||/\(EPERM\)/u.test(String(error?.message??""))))return undefined;
 return protectedFolderOfV1(root,{platform,home});
}

const errorPathsV1=(error,depth=4)=>typeof error!=="object"||error===null||depth<0?[]:[error.path,error.dest].filter(value=>typeof value==="string"&&value.length>0).concat(errorPathsV1(error.cause,depth-1));

/** 📍️ What a refused filesystem call names: the root and the entry that carry the error (or its cause), as the line the service log prints below the failure;  when the error names nothing. */
export function deniedPlaceOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(typeof error.root==="string")return"root="+error.root+(typeof error.directory==="string"?" directory="+error.directory:"")+(typeof error.entry==="string"?" entry="+error.entry:"")+(typeof error.code==="string"?" code="+error.code:"")+(typeof error.reads==="number"?" reads="+error.reads:"");
 return deniedPlaceOfV1(error.cause,depth-1);
}

/** 🛣️ The facts of the service process that go below every refused place: the route it was started by (`SEMIO_SERVICE_ROUTE`: `launchd` a job under the launcher, `direct` a child of the calling process, else `foreground`), its pid and its parent pid, so that the log says which start produced a process without access. */
export function serviceFactsV1({environment=process.env,pid=process.pid,ppid=process.ppid}={}){return"route="+serviceRouteV1(environment)+" pid="+pid+" ppid="+ppid;}

/** 🛣️ The route of this service process (`launchd`, `direct`, else `foreground`). */
export function serviceRouteV1(environment=process.env){return SERVICE_ROUTES_V1.includes(environment.SEMIO_SERVICE_ROUTE)?environment.SEMIO_SERVICE_ROUTE:"foreground";}

const refusedByPermissionV1=(error,depth=4)=>typeof error==="object"&&error!==null&&depth>=0&&(error.code==="EPERM"||/\(EPERM\)/u.test(String(error.message??""))||refusedByPermissionV1(error.cause,depth-1));

const placePathV1=(error,depth=4)=>{
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(typeof error.root==="string")return typeof error.entry==="string"?join(typeof error.directory==="string"?error.directory:error.root,error.entry):error.root;
 return placePathV1(error.cause,depth-1);
};

const relativeCallOfV1=(error,depth=4)=>typeof error!=="object"||error===null||depth<0?undefined:typeof error.reads==="number"&&typeof error.root==="string"?error:relativeCallOfV1(error.cause,depth-1);

/** 🔬️ Whether a relative filesystem call that the helper reports as refused (`EPERM`, `reads` agreeing readings of its failure number) is really a refusal of the access of this process: the same place is read once more by path, with the node file system, which does not share the native call. A refusal by path as well (`EPERM`) is the macOS privacy refusal and gives `undefined`; any other answer (the place exists, is absent, or is an ordinary permission) is returned with the place, the reads and the answer, because then the consent of the user is not what is missing. @see https://developer.apple.com/documentation/bundleresources/protecting-the-user-s-files */
export function relativeCallDivergenceV1(error,{inspect=path=>lstatSync(path)}={}){
 const call=relativeCallOfV1(error);if(call===undefined)return undefined;
 const place=placePathV1(call);
 try{inspect(place);return{place,reads:call.reads,found:"present"};}
 catch(refusal){const found=typeof refusal?.code==="string"?refusal.code:"?";return found==="EPERM"?undefined:{place,reads:call.reads,found};}
}

const WANTED_FIELDS_V1=Object.freeze(["wanted","arrived","seconds","missing","local","heavy"]);

/** 🙋️ The numbers of a graph read that waited for the fleet and was refused (`build.graph.wanted-expired`): the values of the refusal itself, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🪪️graph-state/🙋️wanted/📖️read/🟦️.ts */
export function wantedRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 const named=error.key==="build.graph.wanted-expired"||/graph\.wanted-expired/u.test(String(error.message??""));
 if(named){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/graph\.wanted-expired (\{.*\})\s*$/u.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  return Object.fromEntries(WANTED_FIELDS_V1.map(name=>[name,String(values[name]??(name==="heavy"?"":0))]));
 }
 return wantedRefusalOfV1(error.cause,depth-1);
}

/** 🪜️ The words of a graph read whose plan (the assembly of a wave) grew past what this machine gives a plan (`build.graph.plan-too-heavy`): the refusal\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
export function planRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error.key==="build.graph.plan-too-heavy"||/graph\.plan-too-heavy/u.test(String(error.message??""))){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/graph\.plan-too-heavy (\{.*\})\s*$/u.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  return{stage:String(values.stage??"?"),operation:String(values.operation??"?"),megabytes:String(values.megabytes??0),limit:String(values.limit??0),available:String(values.available??"?"),reserve:String(values.reserve??"?")};
 }
 return planRefusalOfV1(error.cause,depth-1);
}

/** 🛑️ The words of a graph read that stopped projects too heavy for this machine (`build.graph.owner-too-heavy`): the refusal\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs */
export function heavyRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error.key==="build.graph.owner-too-heavy"||/unserved build\.graph\.owner-too-heavy \{/u.test(String(error.message??""))){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/build\.graph\.owner-too-heavy (\{.*\})\s*$/su.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  if(typeof values?.named==="string"&&values.named!==""&&values.limit!==undefined&&values.producers!==undefined)return{named:values.named,limit:String(values.limit),producers:String(values.producers)};
 }
 return heavyRefusalOfV1(error.cause,depth-1);
}

/** 🔒️ The words of a read refused by the lease order (`SEMIO_LEASE_ORDER`): the violation the error carries (what was wanted, what this process held, how long it waited, who held it) as language-neutral fields, or `undefined` for every other failure. @see ../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts */
export function leaseRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 const row=error.code==="SEMIO_LEASE_ORDER"?error.violation:undefined;
 if(row&&typeof row==="object"&&typeof row.requested==="string"&&typeof row.held==="string"){
  const clip=text=>text.length>90?"…"+text.slice(-89):text;
  return{requested:clip(row.requested),mode:row.requestedMode==="exclusive"?"exclusive":"shared",held:row.held===row.requested?"-":clip(row.held),seconds:String(Math.floor((row.waitedMilliseconds??0)/1000)),kind:String(row.kind),holderPid:row.blockerPid===undefined?"?":String(row.blockerPid),holderCommand:String(row.blockerCommand??"?").slice(0,120),holderSeconds:row.blockerHeldSeconds===undefined?"?":String(row.blockerHeldSeconds),bound:row.boundMilliseconds===undefined?"-":String(Math.floor(row.boundMilliseconds/1000))};
 }
 return leaseRefusalOfV1(error.cause,depth-1);
}

/** 🏷️ The text a failure is logged by: its stack, led by its name, code, message and the project it belongs to (`semioProject`) whenever the stack does not carry them, so that a failure whose stack lost its message still says what failed and for which project. */
export function failureDetailV1(error){
 const stack=String(error?.stack??error);
 if(typeof error!=="object"||error===null)return stack;
 const message=String(error.message??""),code=typeof error.code==="string"?error.code:undefined,project=typeof error.semioProject==="string"?error.semioProject:undefined,lead=[String(error.name??"Error")+(message?": "+message:""),code&&!stack.includes(code)?"code="+code:undefined,project&&!stack.includes(project)?"project="+project:undefined].filter(Boolean);
 return stack.startsWith(lead[0])&&lead.length===1?stack:lead.join(" ")+"\n"+stack;
}

/** 🧯 Turns an unexpected failure of a request into the refusal the client receives. The same unexpected failure (name, code and message) `sameCauseMaximum` times in a row, counted in `streak`, is deterministic and becomes the never retried `internal-repeated` refusal, as a slice worker that ends that often with the same text stops the plan. A full disk (`ENOSPC`, `database or disk is full`) is the typed `disk` refusal: when a cache image of the worktree holds the failed path or is itself full, it names that image, its size, fill, largest subtrees and `repo cache gc` (`image-full`); otherwise it names the host partition, its free bytes and the command that frees space; anything else is the `internal` refusal that carries a reference, and the failure itself (with its stack) is written to the service log under that same reference, so the message can say which log file holds the details. @see ../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts */
/** 🤝 The hand-over a read ended with (the stale implementation code of the generation custody, in the error or its causes): its text, first line, cut. */
function handoverOfV1(error,depth=0){
 if(error===null||typeof error!=="object"||depth>6)return undefined;
 if(error.code==="SEMIO_STALE_IMPLEMENTATION")return String(error.message??"").split("\n")[0].slice(0,300);
 return handoverOfV1(error.cause,depth+1);
}
function failureRefusalCoreV1(error,{log,logPath,retry="later",root,platform,home,fullImage=fullImageOfV1,inspect,environment=process.env,pid=process.pid,ppid=process.ppid,streak,sameCauseMaximum=HOST_POLICY_V1.sliceRestartSameCauseMaximum}){
 const handed=handoverOfV1(error);
  if(handed!==undefined){log("graph-handover",handed);return{code:"handover",key:"host-graph-handover",fields:{message:handed},retry:"later"};}
  const plan=planRefusalOfV1(error);
 if(plan!==undefined){log("graph-plan-too-heavy",JSON.stringify(plan));return{code:"unserved",key:"host-graph-plan-too-heavy",fields:plan,retry:"later"};}
 const heavy=heavyRefusalOfV1(error);
 if(heavy!==undefined){log("graph-owner-too-heavy",JSON.stringify(heavy));return{code:"unserved",key:"host-graph-owner-too-heavy",fields:heavy,retry:"later"};}
 const lease=leaseRefusalOfV1(error);
 if(lease!==undefined){log("lease-refused",JSON.stringify(lease));return{code:"unserved",key:lease.holderPid==="?"?"host-lease-refused-unregistered":"host-lease-refused",fields:lease,retry:"later"};}
 const refused=graphMemoryRefusalOfV1(error);
 if(refused!==undefined){const megabytes=bytes=>String(Math.round(bytes/1048576)),fields={key:refused.detail.key,need:megabytes(refused.detail.needBytes),granted:megabytes(refused.detail.availableBytes),free:megabytes(refused.detail.freeBytes),seconds:String(refused.detail.seconds)};log("graph-memory-refused",JSON.stringify(fields));return{code:"unserved",key:"host-graph-memory-refused",fields,retry:"later"};}
 const wanted=wantedRefusalOfV1(error);
 if(wanted!==undefined){log("graph-wanted-expired",JSON.stringify(wanted));return{code:"unserved",key:"host-graph-wanted-expired",fields:wanted,retry:"later"};}
 const facts=diskFullOfV1(error),image=facts&&root?fullImage(root,{paths:errorPathsV1(error)}):undefined,folder=accessDeniedOfV1(error,{root,platform,home}),detail=failureDetailV1(error),reference=createHash("sha256").update(detail).digest("hex").slice(0,8);
 const place=deniedPlaceOfV1(error),permission=refusedByPermissionV1(error),service=serviceFactsV1({environment,pid,ppid});
 const divergence=folder!==undefined&&!facts?relativeCallDivergenceV1(error,inspect?{inspect}:{}):undefined;
 log(facts?"disk-full":"request-failed",reference+" "+detail+(place||permission?"\nwhere: "+(place??"root=? entry=? code="+(typeof error?.code==="string"?error.code:"EPERM"))+" "+service+(divergence?" pathcheck="+divergence.found:""):""));
 if(image)return{code:"disk",key:"image-full",fields:{image:image.id,directory:image.directory,size:formatDiskBytesV1(image.sizeBytes),used:formatDiskBytesV1(image.usedBytes),free:formatDiskBytesV1(image.freeBytes),host:image.hostFreeBytes===undefined?"?":formatDiskBytesV1(image.hostFreeBytes),filledBy:image.filledBy||"?",command:IMAGE_RECLAIM_COMMAND_V1},retry};
 if(facts)return{code:"disk",key:facts.freeBytes===undefined?"disk-full-unmeasured":"disk-full",fields:{partition:facts.partition,...(facts.freeBytes===undefined?{}:{free:formatDiskBytesV1(facts.freeBytes)}),command:DISK_RECLAIM_COMMAND_V1},retry};
 if(divergence)return{code:"unserved",key:"host-relative-call-refused",fields:{place:divergence.place,reads:String(divergence.reads),found:divergence.found,reference,log:logPath},retry};
 if(folder!==undefined)return{code:"consent",key:"access-denied",fields:{folder,reference,log:logPath,place:placePathV1(error)??"?",route:serviceRouteV1(environment),pid:String(pid),ppid:String(ppid),identity:consentIdentityV1({platform:platform??process.platform,home:home??homedir()})?.path??"repo"},retry};
 const message=failureMessageV1(error),count=streak===undefined?1:(streak.count=streak.cause===message?streak.count+1:1,streak.cause=message,streak.count),fields={reference,log:logPath,message,frames:failureFramesV1(error)};
 return count>=sameCauseMaximum?{code:"internal",key:"internal-repeated",fields:{...fields,count:String(count)},retry:"never"}:{code:"internal",key:"internal",fields,retry};
}

/** 🧾️ The block the store keeps for one unexpected failure: the reference, name, code, the full message, the refusal text in English and German and the stack (`detail`), cut at `limit` bytes; a message longer than the room the block leaves is cut, never the stack frames. */
export function failureBlockV1({reference,detail,error,refusal,at=new Date(),limit=FAILURE_BLOCK_BYTES_V1}){
 let words;try{words=renderBothV1(refusal.key,refusal.fields);}catch{words={en:"",de:""};}
 const head=["request-failed "+reference+" "+at.toISOString(),"name: "+String(error?.name??typeof error),"code: "+(typeof error?.code==="string"?error.code:"-"),"en: "+(words.en??""),"de: "+(words.de??""),"message: "+String(error?.message??error).slice(0,4000),"detail:",""].join("\n");
 const frame=detail.indexOf("\n    at "),frames=frame<0?"":Buffer.from(detail.slice(frame)).subarray(0,FAILURE_FRAME_BYTES_V1).toString("utf8"),message=frame<0?detail:detail.slice(0,frame),room=Math.max(0,limit-Buffer.byteLength(head)-Buffer.byteLength(frames)-3);
 const kept=Buffer.byteLength(message)<=room?message:Buffer.from(message).subarray(0,room).toString("utf8")+"...";
 return Buffer.from(head+kept+frames).subarray(0,limit).toString("utf8");
}

/** 📮️ Writes a failure block into the origin folder of the shared store (`<store>/v1/origins/<origin>/service-failure.txt` and `service-failure-<reference>.txt`, the newest 16 references kept) the moment it happens, from the service process itself, so that the cause of a failure on a machine without a shell is readable on the server within seconds. Returns the path written, or `undefined` when the environment names no store or origin or the write fails. */
export function mirrorFailureV1(block,reference,{environment=process.env}={}){
 const store=environment.SEMIO_FLEET_STORE_ROOT,origin=environment.SEMIO_FLEET_ORIGIN;
 if(!store||!origin||/[\\/]|^\.+$/u.test(origin))return undefined;
 try{
  const directory=join(store,"v1","origins",origin);mkdirSync(directory,{recursive:true});
  for(const name of["service-failure-"+reference+".txt","service-failure.txt"]){const file=join(directory,name),temporary=file+"."+process.pid+".tmp";writeFileSync(temporary,block);renameSync(temporary,file);}
  const kept=readdirSync(directory).filter(name=>/^service-failure-[0-9a-f]{8}\.txt$/u.test(name)).map(name=>[name,statSync(join(directory,name)).mtimeMs]).sort((a,b)=>b[1]-a[1]);
  for(const[name]of kept.slice(FAILURE_FILES_KEPT_V1))rmSync(join(directory,name),{force:true});
  return join(directory,"service-failure.txt");
 }catch{return undefined;}
}

/** 🛟️ Turns an unexpected failure of a request into the refusal the client receives (see `failureRefusalCoreV1` in this file for the kinds), and writes the block of every failure that carries a reference into the origin folder of the store. */
export function failureRefusalV1(error,options){
 const refusal=failureRefusalCoreV1(error,options),reference=refusal.fields?.reference;
 if(typeof reference==="string"&&/^[0-9a-f]{8}$/u.test(reference))mirrorFailureV1(failureBlockV1({reference,detail:failureDetailV1(error),error,refusal}),reference,{environment:options.environment});
 return refusal;
}
