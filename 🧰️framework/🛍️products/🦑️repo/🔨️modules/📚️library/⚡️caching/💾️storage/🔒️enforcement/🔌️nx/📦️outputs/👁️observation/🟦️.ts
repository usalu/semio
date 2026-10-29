import {posix} from "node:path";
import {AsyncLocalStorage} from "node:async_hooks";
import schema from "./🧬️schema/🔣️.json";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireIssuedResourceControlV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const policy=schema.$defs.Policy.const;
/** 🪞️ One no-follow observation; directory identities identify the inode, file identities also identify its revision. */
export type OutputObservationV1=Readonly<{kind:"directory"|"file"|"link"|"missing";identity:string;target?:string}>;
/** 🎯️ An exact directory subscription, with recursive content separate from ordinary directory membership. */
export type OutputObservationScopeV1=Readonly<{path:string;mode:"directory"|"recursive";identity:string}>;
/** 📋️ Coverage and the physical observations that must still agree after subscriptions start. */
export type OutputObservationPlanV1=Readonly<{scopes:readonly OutputObservationScopeV1[];observations:ReadonlyMap<string,OutputObservationV1>;aliases:readonly OutputObservationAliasV1[];outputs:readonly string[]}>;
/** 🔗️ Resolves physical content and link membership changes to the logical coordinates the output tracker recorded. */
export type OutputObservationAliasV1=Readonly<{physical:string;logical:string;mode:"content"|"membership"}>;
const sameObservation=(a:OutputObservationV1,b:OutputObservationV1)=>a.kind===b.kind&&a.identity===b.identity&&a.target===b.target;
const inside=(root:string,path:string)=>path===root||path.startsWith((root==="/"?"":root)+"/");
const coordinate=(value:string,expression=true)=>{if(typeof value!=="string"||!value||value.includes("\\")||value.includes("\0")||value.startsWith("/")||value.includes(":")||expression&&(value.startsWith("!")||/\{(?:projectRoot|workspaceRoot|options|params|args|configuration)[}.]/u.test(value)))throw Error("Exact interpolated output coordinate required");const normalized=posix.normalize(value).replace(/\/$/u,"");if(normalized===".."||normalized.startsWith("../")||normalized===".")throw Error("Bounded output coordinate required");return normalized;};

/** 🧭️ Plans only actual output subtrees and nonrecursive ancestry, including both sides of links; no missing prefix becomes a recursive workspace watch. */
export async function planOutputObservationV1(root:string,expressions:readonly string[],observe:(path:string)=>Promise<OutputObservationV1>,check:()=>void):Promise<OutputObservationPlanV1>{
 check();if(!root||root.includes("\\")||root.includes("\0")||root.length>policy.maximumPathUnits||!(root.startsWith("/")||/^[A-Za-z]:\//u.test(root))||root!=="/"&&root.endsWith("/"))throw Error("Canonical output workspace required");
 const rootParts=root.replace(/^(?:[A-Za-z]:\/|\/\/|\/)/u,"").split("/");if(root!=="/"&&(rootParts.some(part=>!part||part==="."||part==="..")||root.startsWith("//")&&rootParts.length<2))throw Error("Canonical output workspace required");
 if(expressions.length>policy.maximumExpressions)throw Error("Output observation expression capacity exhausted");
 const observations=new Map<string,OutputObservationV1>(),scopes=new Map<string,OutputObservationScopeV1>(),aliases=new Map<string,OutputObservationAliasV1>(),outputs=new Set<string>(),distance=(path:string)=>path===root?".":path.slice((root==="/"?"":root).length+1);
 const alias=(physical:string,logical:string,mode:"content"|"membership")=>{if(physical===logical)return;aliases.set(mode+":"+physical+":"+logical,Object.freeze({physical,logical,mode}));if(aliases.size>policy.maximumAliases)throw Error("Output observation alias capacity exhausted");};
 const read=async(path:string)=>{check();if(path.length>policy.maximumPathUnits)throw Error("Bounded output observation path required");let value=observations.get(path);if(value===undefined){value=Object.freeze({...await observe(path)});if(Object.keys(value).some(key=>!["kind","identity","target"].includes(key))||!["directory","file","link","missing"].includes(value.kind)||typeof value.identity!=="string"||value.kind==="link"&&typeof value.target!=="string")throw Error("Closed physical output observation required");observations.set(path,value);}return value;};
 const add=(path:string,mode:"directory"|"recursive",identity:string)=>{if(mode==="directory"&&scopes.get(path)?.mode==="recursive")return;scopes.set(path,Object.freeze({path,mode,identity}));};
 const visit=async(relative:string,recursive:boolean,depth:number,logical:string):Promise<string>=>{
  if(depth>policy.maximumLinkDepth)throw Error("Output observation link depth exhausted");
  alias(relative,logical,"content");
  const base=await read(root);if(base.kind!=="directory")throw Error("Ordinary output workspace required");add(root,"directory",base.identity);
  const parts=relative.split("/");let current=root;
  for(let index=0;index<parts.length;index++){
   const parent=current;current=(current==="/"?"":current)+"/"+parts[index];const value=await read(current);
   if(value.kind==="missing")return (root==="/"?"":root)+"/"+relative;
   if(value.kind==="link"){
    const target=value.target!;if(target.includes("\\")||target.includes("\0"))throw Error("Canonical output link target required");
    let selected:string;
    if(target.startsWith("/")||/^[A-Za-z]:/u.test(target)){if(!inside(root,target))throw Error("Output link leaves workspace");selected=target===root?".":target.slice((root==="/"?"":root).length+1);}
    else{const parentRelative=parent===root?".":parent.slice((root==="/"?"":root).length+1);selected=posix.normalize(posix.join(parentRelative,target));if(selected===".."||selected.startsWith("../"))throw Error("Output link leaves workspace");}
    alias(distance(current),logical,"membership");const tail=parts.slice(index+1).join("/"),combined=posix.join(selected,tail);if(combined===".")throw Error("Output link selects workspace root");return visit(coordinate(combined),recursive,depth+1,logical);
   }
   if(value.kind==="file")return (root==="/"?"":root)+"/"+relative;
   add(current,index===parts.length-1&&recursive?"recursive":"directory",value.identity);
  }return (root==="/"?"":root)+"/"+relative;
 };
 for(const expression of expressions){const normalized=coordinate(expression),wild=normalized.search(/[*?\[{]/u),prefix=wild<0?normalized:posix.dirname(normalized.slice(0,wild)+"_");if((root==="/"?0:root.length)+1+normalized.length>policy.maximumPathUnits)throw Error("Bounded output observation path required");if(prefix===".")throw Error("Whole workspace output observation refused");outputs.add(prefix);alias(distance(await visit(prefix,true,0,prefix)),prefix,"content");}
 const recursive=new Set([...scopes.values()].filter(scope=>scope.mode==="recursive").map(scope=>scope.path));
 const selected=[...scopes.values()].filter(scope=>{let path=scope.path;while(path!==root){check();path=path.slice(0,path.lastIndexOf("/"))||"/";if(recursive.has(path))return false;}return true;}).sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
 if(selected.length>policy.maximumWatchers)throw Error("Output observation watcher capacity exhausted");
 return Object.freeze({scopes:Object.freeze(selected),observations,aliases:Object.freeze([...aliases.values()]),outputs:Object.freeze([...outputs])});
}

/** 📨️ Watch backends report workspace-relative coordinates; directory membership may report its own path. */
export type OutputObservationEventV1=Readonly<{path:string;type:"create"|"update"|"delete"}>;
/** 🔌️ A successful start establishes kernel coverage before resolving; flush drains backend buffering and stop settles its resources. */
export interface OutputObservationHandleV1{flush():readonly OutputObservationEventV1[]|Promise<readonly OutputObservationEventV1[]>;stop():void|Promise<void>}
/** 🧰️ System and native adapters remain private behind these owned interfaces. */
export interface OutputObservationPortsV1{observe(path:string):Promise<OutputObservationV1>;watch(scope:OutputObservationScopeV1,receive:(error:unknown,events:readonly OutputObservationEventV1[])=>void):Promise<OutputObservationHandleV1>}
/** 🚦️ A daemon-owned observer registers before baseline publication, receives raw Source events, and drains every channel before a warm response. */
export interface RepositoryOutputObservationV1{register(expressions:readonly string[]):Promise<void>;receiveSource(events:readonly OutputObservationEventV1[]):void;flush():Promise<void>;stop():Promise<void>}

/** 🕸️ Retains finite physical coverage for the owning daemon lifetime, with serialized reconciliation and fail-closed errors. */
export function createRepositoryOutputObservationV1(options:Readonly<{root:string;control:ResourceLeaseControl;ports:OutputObservationPortsV1;receive(error:unknown,events:readonly OutputObservationEventV1[]):void|Promise<void>}>):RepositoryOutputObservationV1{
 const {root,control,ports}=options,expressions=new Set<string>();
 type RecordV1={scope:OutputObservationScopeV1;handle:OutputObservationHandleV1;live:boolean};
 const handles=new Map<string,RecordV1>(),retired=new Set<RecordV1>(),delivery=new AsyncLocalStorage<{live:boolean}>(),notificationErrors:unknown[]=[];requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);
 let tail:Promise<void>=Promise.resolve(),notification:Promise<void>=Promise.resolve(),failure:{error:unknown}|undefined,closing=false,closed:Promise<void>|undefined,epoch=0,queuedEvents=0,last:ReadonlyMap<string,OutputObservationV1>=new Map(),aliases:readonly OutputObservationAliasV1[]=[],outputs:readonly string[]=[];
 const check=()=>{checkResourceLeaseControl(control);if(failure)throw failure.error;};
 const admit=()=>{if(closing)throw Error("Output observer admission closed");if(delivery.getStore()?.live)throw Error("Output observer callback cannot await its own admission");check();};
 const notify=(error:unknown,events:readonly OutputObservationEventV1[])=>{const frame={live:true};return delivery.run(frame,async()=>{try{await options.receive(error,events);}finally{frame.live=false;}});};
 const fail=(error:unknown)=>{if(failure)return;failure={error};notification=Promise.resolve().then(()=>notify(error,[])).catch(error=>{notificationErrors.push(error);});};
 const enqueue=(body:()=>Promise<void>)=>{const work=tail.then(body);tail=work.catch(fail);return work;};
 const validateEvent=(event:OutputObservationEventV1)=>{if(!event||!["create","update","delete"].includes(event.type)||event.path!=="."&&(coordinate(event.path,false)!==event.path||(root==="/"?0:root.length)+1+event.path.length>policy.maximumPathUnits))throw Error("Closed output observation event required");};
 const deliver=async(events:readonly OutputObservationEventV1[])=>{
  if(events.length>policy.maximumEvents)throw Error("Output observation event capacity exhausted");const expanded:OutputObservationEventV1[]=[],seen=new Set<string>(),started=performance.now();let workUnits=0;
  const yieldDelivery=async()=>{await control.publish({resource:"filesystem:output-events",mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();check();};
  const add=(event:OutputObservationEventV1)=>{validateEvent(event);const key=event.type+":"+event.path;if(seen.has(key))return;seen.add(key);if(expanded.length>=policy.maximumEvents)throw Error("Output observation event capacity exhausted");expanded.push(Object.freeze({path:event.path,type:event.type}));};
  for(const event of events){if(++workUnits%policy.deliveryYieldInterval===0)await yieldDelivery();check();add(event);if(event.path==="."){for(const path of outputs){if(++workUnits%policy.deliveryYieldInterval===0)await yieldDelivery();add({path,type:event.type});}continue;}for(const alias of aliases){if(++workUnits%policy.deliveryYieldInterval===0)await yieldDelivery();check();if(inside(event.path,alias.physical))add({path:alias.logical,type:event.type});else if(alias.mode==="content"&&inside(alias.physical,event.path))add({path:alias.logical+event.path.slice(alias.physical.length),type:event.type});}}
  if(expanded.length)await notify(null,Object.freeze(expanded));
 };
 const feed=(error:unknown,events:readonly OutputObservationEventV1[])=>{
  if(closing)return;if(error!==null&&error!==undefined){epoch++;fail(error);return;}
  let batch:readonly OutputObservationEventV1[];try{if(!Array.isArray(events)||queuedEvents+events.length>policy.maximumEvents)throw Error("Output observation pending event capacity exhausted");if(!events.length)return;batch=Object.freeze(Array.from(events,event=>{const row={path:event?.path,type:event?.type};validateEvent(row);return Object.freeze(row);}));}catch(error){fail(error);return;}
  const count=batch.length;queuedEvents+=count;epoch++;void enqueue(async()=>{try{check();await deliver(batch);await reconcile();}finally{queuedEvents-=count;}}).catch(fail);
 };
 const stopRecords=async(records:Iterable<RecordV1>)=>{const errors:unknown[]=[];for(const record of records){record.live=false;retired.add(record);try{await record.handle.stop();retired.delete(record);}catch(error){errors.push(error);}}if(errors.length===1)throw errors[0];if(errors.length)throw new AggregateError(errors,"Output observation subscriptions did not close");};
 const reconcile=async()=>{
  check();let stable=false;
  for(let attempt=0;attempt<policy.maximumAttempts;attempt++){
   let observed=0;const started=performance.now(),plan=await planOutputObservationV1(root,[...expressions],async path=>{if(observed++%64===0){await control.publish({resource:"filesystem:output-observation:"+observed,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();}check();return ports.observe(path);},check),wanted=new Set(plan.scopes.map(scope=>scope.mode+":"+scope.path)),created=new Map<string,RecordV1>();
   try{for(const scope of plan.scopes){check();const key=scope.mode+":"+scope.path,existing=handles.get(key);if(existing?.scope.identity===scope.identity)continue;
    const record={scope,handle:undefined as unknown as OutputObservationHandleV1,live:true};record.handle=await ports.watch(scope,(error,events)=>{if(record.live)feed(error,events);});created.set(key,record);check();
   }}catch(error){try{await stopRecords(created.values());}catch(cleanup){throw new AggregateError([error,cleanup],"Output observation registration failed");}throw error;}
   const changed:string[]=[];try{for(const [path,before]of plan.observations){check();if(!sameObservation(before,await ports.observe(path)))changed.push(path);}}catch(error){try{await stopRecords(created.values());}catch(cleanup){throw new AggregateError([error,cleanup],"Output observation recheck failed");}throw error;}
   if(changed.length){await stopRecords(created.values());await deliver(changed.map(path=>({path:path===root?".":path.slice((root==="/"?"":root).length+1),type:"update"})));continue;}
   const changes:OutputObservationEventV1[]=[];for(const [path,before]of last){const after=plan.observations.get(path);if(after&&!sameObservation(before,after))changes.push({path:path===root?".":path.slice((root==="/"?"":root).length+1),type:after.kind==="missing"?"delete":"update"});}
   const obsolete:RecordV1[]=[];for(const [key,record]of created){const previous=handles.get(key);if(previous)obsolete.push(previous);handles.set(key,record);}
   for(const [key,record]of handles)if(!wanted.has(key)){obsolete.push(record);handles.delete(key);}
   last=plan.observations;aliases=plan.aliases;outputs=plan.outputs;await stopRecords(obsolete);await deliver(changes);
   stable=true;break;
  }
  if(!stable)throw Error("Output observation changed during subscription");
 };
 return Object.freeze<RepositoryOutputObservationV1>({
  register:async values=>{admit();if(values.length>policy.maximumExpressions)throw Error("Output observation expression capacity exhausted");return enqueue(async()=>{check();for(const value of values){check();expressions.add(value);}if(expressions.size>policy.maximumExpressions)throw Error("Output observation expression capacity exhausted");await reconcile();});},
  receiveSource:events=>feed(null,events),
  flush:async()=>{admit();for(let attempt=0;attempt<policy.maximumAttempts;attempt++){await tail;check();const before=epoch;await enqueue(async()=>{check();for(const record of handles.values())await deliver(await record.handle.flush());await reconcile();});await tail;check();if(before===epoch)return;}throw Error("Output observation did not settle before response");},
  stop:()=>{if(delivery.getStore()?.live)return Promise.reject(Error("Output observer callback cannot await its own settlement"));return closed??=(async()=>{closing=true;await tail;const errors:unknown[]=[];try{await stopRecords(new Set([...handles.values(),...retired]));}catch(error){errors.push(error);}handles.clear();await notification;errors.push(...notificationErrors);if(failure)errors.unshift(failure.error);if(errors.length===1)throw errors[0];if(errors.length>1)throw new AggregateError(errors,"Output observer settlement failed");})();}
 });
}
