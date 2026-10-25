import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,slotPattern=new RegExp(policy.names.slotPattern),keyPattern=new RegExp(policy.names.keyPattern),day=86400000;
/** 🔢️ The authored numbers of the session slots. */
export const BAZEL_SESSION_POLICY_V1=policy;
export type BazelSessionPolicyV1=typeof policy;
export type BazelMemoryPolicyV1=Readonly<{floorBasisPoints:number;floorMinimumMegabytes:number;floorMaximumMegabytes:number}>;
export type BazelServerHostV1=Readonly<{cores:number;memoryMegabytes:number}>;
export type BazelSlotKindV1="build"|"service";
export type BazelServerPlanV1=Readonly<{slots:number;services:number;heapMegabytes:number;residentMegabytes:number;budgetMegabytes:number;cpuPerServer:number;memoryPerServerMegabytes:number;limit:"memory"|"cores"|"policy"|"floor"}>;
export type BazelSlotStateV1=Readonly<{version:1;slot:string;key:string;keys:readonly string[];usedAt:number;session?:string;agent?:string}>;
export type BazelSessionOwnerV1=Readonly<{session:string;agent:string}>;
export type BazelInventoryV1=Readonly<{slots:readonly Readonly<{slot:string;outputBase:boolean;outputBaseBytes?:number;state?:BazelSlotStateV1;packages:readonly Readonly<{key:string;modifiedAt:number;bytes?:number;temporary?:boolean}>[]}>[];foreign:readonly string[]}>;
export type BazelGarbageV1=Readonly<{packages:readonly Readonly<{slot:string;key:string;reason:"capacity"|"count"|"age"|"pressure"|"growth"}>[];outputBases:readonly Readonly<{slot:string;reason:"capacity"|"idle"|"pressure"|"growth"}>[];temporaries:readonly Readonly<{slot:string;key:string}>[];foreign:readonly string[]}>;
/** 🌡️ What the products image reports before a collection: its free bytes against the soft threshold and the target of the ledger (`reclaim` of the workspaces area) decide whether and how far the pressure rule frees. */
export type BazelPressureV1=Readonly<{freeBytes:number;softBytes:number;targetBytes:number}>;
export type BazelLanguageV1="en"|"de";

/** 🧮️ Bounds the Bazel servers a host affords: memory above the host capacity floor times the authored share divided by one server's resident size, cores divided by the cores one server deserves, the authored maximum, never below the minimum. When even one server does not fit the share, its heap shrinks to what remains but never below the authored minimum. */
export function planBazelServersV1(host:BazelServerHostV1,memory:BazelMemoryPolicyV1,rules:BazelSessionPolicyV1=policy):BazelServerPlanV1{
 if(validateJsonSchemaSubset(schema.definitions.Host,host,schema).length||validateJsonSchemaSubset(schema.definitions.MemoryPolicy,memory,schema).length)throw Error("Closed Bazel server host required");
 const floor=Math.min(memory.floorMaximumMegabytes,Math.max(memory.floorMinimumMegabytes,Math.floor(host.memoryMegabytes*memory.floorBasisPoints/10000))),spare=Math.max(0,host.memoryMegabytes-floor),budget=Math.floor(spare*rules.servers.memoryShareBasisPoints/10000);
 const byMemory=Math.floor(budget/rules.servers.residentMegabytes),byCores=Math.floor(host.cores/rules.servers.coresPerServer),bounded=Math.min(byMemory,byCores,rules.servers.maximum),slots=Math.max(rules.servers.minimum,bounded);
 const limit:BazelServerPlanV1["limit"]=bounded<rules.servers.minimum?"floor":bounded===byMemory?"memory":bounded===byCores?"cores":"policy",overhead=rules.servers.residentMegabytes-rules.servers.heapMegabytes,heap=byMemory>=1?rules.servers.heapMegabytes:Math.max(rules.servers.minimumHeapMegabytes,Math.min(rules.servers.heapMegabytes,budget-overhead));
 const plan={slots,services:rules.servers.serviceMaximum,heapMegabytes:heap,residentMegabytes:heap+overhead,budgetMegabytes:budget,cpuPerServer:Math.max(1,Math.floor(host.cores/slots)),memoryPerServerMegabytes:Math.max(256,Math.floor((spare-budget)/slots)),limit};
 if(validateJsonSchemaSubset(schema.definitions.ServerPlan,plan,schema).length)throw Error("Closed Bazel server plan required");return Object.freeze(plan);
}

/** 🪆️ The plan for a session that starts inside a Bazel action: its parent holds a slot until the action ends, so the nested session gets a few overflow build slots on top and can never deadlock on the slot its own parent holds. */
export function bazelNestedServersV1(plan:BazelServerPlanV1,nested:boolean,rules:BazelSessionPolicyV1=policy):BazelServerPlanV1{
 if(!nested)return plan;const widened={...plan,slots:Math.min(99,plan.slots+rules.servers.nestedExtra)};
 if(validateJsonSchemaSubset(schema.definitions.ServerPlan,widened,schema).length)throw Error("Closed Bazel server plan required");return Object.freeze(widened);
}

/** 🌱️ The extra build slots beyond the planned ones: a session opens one only while every planned slot is in use, the host admission reports idle memory for another server and the products image has room for another workspace and output base, so the slot count follows the live machine and its disk instead of a fixed budget. */
export function bazelGrowthSlotIdsV1(plan:Pick<BazelServerPlanV1,"slots">,rules:BazelSessionPolicyV1=policy):readonly string[]{return Object.freeze(Array.from({length:Math.max(0,rules.servers.growthMaximum-plan.slots)},(_,index)=>"s"+(plan.slots+index)));}
/** 🏷️ Names the slots of a plan: short ASCII ids that keep every output base far inside the path budgets of all platforms; `s0...` are build slots, `d0...` service slots for the long-running tasks that must not pin a build slot. */
export function bazelSlotIdsV1(plan:Pick<BazelServerPlanV1,"slots"|"services">,kind:BazelSlotKindV1="build"):readonly string[]{return Object.freeze(Array.from({length:kind==="build"?plan.slots:plan.services},(_,index)=>(kind==="build"?"s":"d")+index));}

/** 🔖️ Admits only slot ids of the authored shape, so no path can leave its directory. */
export function requireBazelSlotV1(slot:string):string{if(!slotPattern.test(slot))throw Error(`Bazel session slot required: ${slot}`);return slot;}
/** 🔐️ Admits only selection keys of the authored shape. */
export function requireBazelKeyV1(key:string):string{if(!keyPattern.test(key))throw Error(`Bazel session key required: ${key}`);return key;}

/** 📁️ Names the generated package of one selection inside one slot; two sessions never share it because a slot has one holder, and the same selection in two slots has two packages. */
export function bazelSessionWorkspaceV1(workspaces:string,slot:string,key:string):string{
 const path=join(workspaces,requireBazelSlotV1(slot),requireBazelKeyV1(key));if(path.length>256)throw Error("Bazel storage path exceeds 256 characters");return path;
}

/** 🥇️ Orders every slot by how well it serves a selection: the slot whose last selection it is (warm server, warm package), then slots that still keep its package, then slots never used, then the least recently used; ties go to the slot the agent session prefers (its preference modulo the slot count) and then keep the slot number order, so concurrent agents start on different slots. */
export function rankBazelSlotsV1(slots:readonly string[],states:Readonly<Record<string,BazelSlotStateV1|undefined>>,key:string,preference?:number):readonly string[]{
 const tier=(slot:string)=>{const state=states[slot];return state===undefined?2:state.key===key?0:state.keys.includes(key)?1:3;},index=(slot:string)=>preference===undefined?Number(slot.slice(1)):(slots.indexOf(slot)-preference%slots.length+slots.length)%slots.length,recency=(left:string,right:string)=>{const first=tier(left);return first<2?states[right]!.usedAt-states[left]!.usedAt:first===3?states[left]!.usedAt-states[right]!.usedAt:0;};
 return Object.freeze([...slots].sort((left,right)=>tier(left)-tier(right)||(tier(left)===tier(right)?recency(left,right):0)||index(left)-index(right)));
}

/** 📝️ The state a slot remembers after a session: the key first, older keys behind it, bounded by the packages a slot keeps. */
export function nextBazelSlotStateV1(slot:string,previous:BazelSlotStateV1|undefined,key:string,now:number,rules:BazelSessionPolicyV1=policy,owner?:BazelSessionOwnerV1):BazelSlotStateV1{
 const state={version:1 as const,slot:requireBazelSlotV1(slot),key:requireBazelKeyV1(key),keys:[key,...(previous?.keys??[]).filter(value=>value!==key)].slice(0,rules.retention.packagesPerSlot),usedAt:now,...owner===undefined?{}:{session:owner.session,agent:owner.agent}};
 if(validateJsonSchemaSubset(schema.definitions.SlotState,state,schema).length)throw Error("Closed Bazel slot state required");return Object.freeze(state);
}

/** 🚶️ One registered slot waiter: `first` says whether every earlier live waiter of its class has left, `leave` withdraws it. Waiters are served in arrival order without anyone holding the picker. */
export type BazelSlotWaiterV1=Readonly<{first():boolean;leave():void}>;
/** 🔌️ Everything a session needs from its surroundings: the exclusive claim of a slot, the picker gate that orders only the short claim attempts, the queue of slot waiters, the remembered states, time and progress. Production maps it to repository leases and files; tests map it to memory. */
export type BazelSlotPortV1=Readonly<{
 claim(slot:string,signal:AbortSignal):Promise<(()=>void)|undefined>;
 gate(signal:AbortSignal,kind:BazelSlotKindV1):Promise<()=>void>;
 queued(kind:BazelSlotKindV1):number;
 spare():Promise<number>;
 imageFree?():Promise<number>;
 enqueue(kind:BazelSlotKindV1):BazelSlotWaiterV1;
 read(slot:string):Promise<BazelSlotStateV1|undefined>;
 write(slot:string,state:BazelSlotStateV1):Promise<void>;
 sleep(milliseconds:number,signal:AbortSignal):Promise<void>;
 now():number;
 progress(line:string):void|Promise<void>;
}>;
export type BazelSessionRequestV1=Readonly<{key:string;signal:AbortSignal;kind?:BazelSlotKindV1;growth?:boolean;timeoutMilliseconds?:number;language:BazelLanguageV1;preference?:number;owner?:BazelSessionOwnerV1}>;
export type BazelSessionV1=Readonly<{slot:string;key:string;mode:"warm"|"switch"|"fresh";previous?:BazelSlotStateV1;release():Promise<void>}>;

/** 💬️ Renders one authored message. */
export function bazelSessionMessageV1(id:string,values:Readonly<Record<string,string|number>>,language:BazelLanguageV1):string{
 const text=(schema.definitions.Messages.const[language] as Readonly<Record<string,string>>)[id];if(text===undefined)throw Error(`Unknown Bazel session message: ${id}`);
 return text.replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=values[name];if(value===undefined)throw Error(`Missing Bazel session message value: ${name}`);return String(value);});
}

/** 🎫️ Claims one slot of the requested class (build by default, service for sessions that launch long-running tasks) for one selection. The picker is a short critical section: under it a session either takes the first free slot in the order of their fitness, or, when every slot is claimed or earlier waiters are queued, registers as a slot waiter and releases the picker at once. Waiters are served in arrival order; the oldest takes the picker only for one claim attempt per poll, reports progress, and gives up when the signal aborts or the timeout passes. A slot has exactly one holder, so two sessions can never meet in one Bazel server lock, and nobody holds the picker while waiting for a slot. The planned slots are the floor: when every one of them is in use and the host admission reports idle memory for another server (`growthSpareMegabytes`), a build opens an extra slot up to `growthMaximum`, so the slot count follows the live machine and a long activation that waits on compiler locks cannot starve short runs while memory is idle. */
export async function acquireBazelSessionV1(port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,request:BazelSessionRequestV1,rules:BazelSessionPolicyV1=policy):Promise<BazelSessionV1>{
 let imageReported=false;requireBazelKeyV1(request.key);const kind=request.kind??"build",slots=bazelSlotIdsV1(plan,kind),language=request.language,started=port.now(),signal=request.signal;let nextProgress=0;
 const attempt=async():Promise<BazelSessionV1|undefined>=>{
  const states:Record<string,BazelSlotStateV1|undefined>={};for(const slot of slots)states[slot]=await port.read(slot);
  for(const slot of rankBazelSlotsV1(slots,states,request.key,request.preference)){
   const claimed=await port.claim(slot,signal);if(claimed===undefined)continue;
   let released=false,before:BazelSlotStateV1|undefined;
   try{before=await port.read(slot);const mode:BazelSessionV1["mode"]=before===undefined?"fresh":before.key===request.key?"warm":"switch";await port.progress(bazelSessionMessageV1("claimed",{slot,mode:bazelSessionMessageV1(mode,{},language)},language));
    return Object.freeze({slot,key:request.key,mode,...(before?{previous:before}:{}),async release(){if(released)return;released=true;try{await port.write(slot,nextBazelSlotStateV1(slot,before,request.key,port.now(),rules,request.owner));}finally{claimed();}}});
   }catch(error){if(!released){released=true;claimed();}throw error;}
  }
  if(kind==="build"&&request.growth!==false){
   const growth=bazelGrowthSlotIdsV1(plan,rules);
   if(growth.length){
    const spare=await port.spare(),image=port.imageFree?await port.imageFree():Infinity;
    if(spare>=rules.servers.growthSpareMegabytes&&image<rules.servers.growthImageFreeBytes&&!imageReported){imageReported=true;await port.progress(bazelSessionMessageV1("growth-image",{free:Math.floor(image/1048576),need:Math.floor(rules.servers.growthImageFreeBytes/1048576),slot:growth[0]!},language));}
    if(spare>=rules.servers.growthSpareMegabytes&&image>=rules.servers.growthImageFreeBytes){
     const extra:Record<string,BazelSlotStateV1|undefined>={};for(const slot of growth)extra[slot]=await port.read(slot);
     for(const slot of rankBazelSlotsV1(growth,extra,request.key,request.preference)){
      const claimed=await port.claim(slot,signal);if(claimed===undefined)continue;
      let released=false,before:BazelSlotStateV1|undefined;
      try{before=await port.read(slot);const mode:BazelSessionV1["mode"]=before===undefined?"fresh":before.key===request.key?"warm":"switch";await port.progress(bazelSessionMessageV1("grown",{slot,spare:Math.floor(spare)},language));await port.progress(bazelSessionMessageV1("claimed",{slot,mode:bazelSessionMessageV1(mode,{},language)},language));
       return Object.freeze({slot,key:request.key,mode,...(before?{previous:before}:{}),async release(){if(released)return;released=true;try{await port.write(slot,nextBazelSlotStateV1(slot,before,request.key,port.now(),rules,request.owner));}finally{claimed();}}});
      }catch(error){if(!released){released=true;claimed();}throw error;}
     }
    }
   }
  }
  return undefined;
 };
 const picked=async<T>(body:()=>Promise<T>):Promise<T>=>{signal.throwIfAborted();const release=await port.gate(signal,kind);try{return await body();}finally{release();}};
 let waiter:BazelSlotWaiterV1|undefined;
 try{
  const first=await picked(async()=>{if(port.queued(kind)===0){const session=await attempt();if(session)return session;}waiter=port.enqueue(kind);return undefined;});
  if(first)return first;
  while(true){
   signal.throwIfAborted();
   if(waiter!.first()){const session=await picked(attempt);if(session){waiter!.leave();waiter=undefined;return session;}}
   const waited=port.now()-started;if(request.timeoutMilliseconds!==undefined&&waited>=request.timeoutMilliseconds)throw Error("Bazel session slot timed out");
   if(waited>=nextProgress){await port.progress(bazelSessionMessageV1(kind==="build"?"waiting":"waiting-service",{slots:slots.length,seconds:Math.floor(waited/1000)},language));nextProgress=waited+rules.waiting.progressMilliseconds;}
   await port.sleep(rules.waiting.pollMilliseconds,signal);
  }
 }finally{waiter?.leave();}
}

/** 🪢️ Runs a body inside one session and always releases the slot, also when the body throws or the invocation is cancelled. */
export async function withBazelSessionV1<T>(port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,request:BazelSessionRequestV1,body:(session:BazelSessionV1)=>Promise<T>,rules:BazelSessionPolicyV1=policy):Promise<T>{
 const session=await acquireBazelSessionV1(port,plan,request,rules);let failed=false;
 try{return await body(session);}catch(error){failed=true;throw error;}finally{try{await session.release();}catch(error){if(!failed)throw error;}}
}

/** 🚀️ The startup options one slot's server needs beyond the output base: idle exit and a heap bounded by the plan. */
export function bazelSessionStartupV1(plan:Pick<BazelServerPlanV1,"heapMegabytes">,idleSeconds:number):readonly string[]{
 if(!Number.isSafeInteger(idleSeconds)||idleSeconds<1)throw Error("Positive Bazel server idle seconds required");return Object.freeze([`--max_idle_secs=${idleSeconds}`,`--host_jvm_args=-Xmx${plan.heapMegabytes}m`]);
}

/** 📦️ Decides what to collect: output bases and packages of slots beyond the current capacity, packages beyond the retained count or age (the newest and the slot's last package always stay), output bases idle beyond the authored age, leftover capture folders of the packages that stay, and every entry that is not a slot. Under pressure (free bytes below the soft threshold) it also takes the oldest unprotected packages (all but the newest of a slot and its remembered selection, none younger than the grace) and then the longest idle output bases until the target free space is reached; bytes that were not measured count as nothing, so an unmeasured inventory takes every candidate. A slot that a session holds is never touched. */
export function planBazelSessionGarbageV1(inventory:BazelInventoryV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,now:number,held:ReadonlySet<string>=new Set(),rules:BazelSessionPolicyV1=policy,pressure?:BazelPressureV1):BazelGarbageV1{
 if(validateJsonSchemaSubset(schema.definitions.Inventory,inventory,schema).length)throw Error("Closed Bazel session inventory required");
 const valid=new Set([...bazelSlotIdsV1(plan,"build"),...bazelSlotIdsV1(plan,"service"),...bazelGrowthSlotIdsV1(plan,rules)]),packages:{slot:string;key:string;reason:"capacity"|"count"|"age"|"pressure"|"growth"}[]=[],outputBases:{slot:string;reason:"capacity"|"idle"|"pressure"|"growth"}[]=[],temporaries:{slot:string;key:string}[]=[],order=(left:string,right:string)=>left<right?-1:left>right?1:0;
 const rows=inventory.slots.filter(row=>!held.has(row.slot)),newest=(row:BazelInventoryV1["slots"][number])=>[...row.packages].sort((left,right)=>right.modifiedAt-left.modifiedAt||order(left.key,right.key));
 for(const row of rows){
  if(!valid.has(row.slot)){if(row.outputBase)outputBases.push({slot:row.slot,reason:"capacity"});for(const entry of row.packages)packages.push({slot:row.slot,key:entry.key,reason:"capacity"});continue;}
  if(row.outputBase&&row.state!==undefined&&now-row.state.usedAt>rules.retention.outputBaseMaximumIdleDays*day)outputBases.push({slot:row.slot,reason:"idle"});
  newest(row).forEach((entry,index)=>{if(index===0||entry.key===row.state?.key)return;if(index>=rules.retention.packagesPerSlot)packages.push({slot:row.slot,key:entry.key,reason:"count"});else if(now-entry.modifiedAt>rules.retention.packageMaximumAgeDays*day)packages.push({slot:row.slot,key:entry.key,reason:"age"});});
 }
 if(pressure!==undefined&&pressure.freeBytes<pressure.softBytes){
  const gone=new Set(packages.map(row=>row.slot+"/"+row.key)),goneBases=new Set(outputBases.map(row=>row.slot)),candidates:{slot:string;key:string;modifiedAt:number;bytes:number}[]=[],idle:{slot:string;usedAt:number;bytes:number}[]=[];
  let need=pressure.targetBytes-pressure.freeBytes;
  for(const row of rows){
   for(const entry of row.packages)if(gone.has(row.slot+"/"+entry.key))need-=entry.bytes??0;
   if(goneBases.has(row.slot))need-=row.outputBaseBytes??0;
   if(!valid.has(row.slot))continue;
   const kept=new Set(newest(row).slice(0,rules.pressure.protectedPackagesPerSlot).map(entry=>entry.key));if(row.state)kept.add(row.state.key);
   for(const entry of row.packages)if(!kept.has(entry.key)&&!gone.has(row.slot+"/"+entry.key)&&now-entry.modifiedAt>=rules.pressure.packageGraceMinutes*60000)candidates.push({slot:row.slot,key:entry.key,modifiedAt:entry.modifiedAt,bytes:entry.bytes??0});
   if(row.outputBase&&row.state!==undefined&&!goneBases.has(row.slot)&&now-row.state.usedAt>=rules.pressure.outputBaseMinimumIdleHours*3600000)idle.push({slot:row.slot,usedAt:row.state.usedAt,bytes:row.outputBaseBytes??0});
  }
  for(const entry of candidates.sort((left,right)=>left.modifiedAt-right.modifiedAt||order(left.slot+left.key,right.slot+right.key))){if(need<=0)break;packages.push({slot:entry.slot,key:entry.key,reason:"pressure"});gone.add(entry.slot+"/"+entry.key);need-=entry.bytes;}
  for(const entry of idle.sort((left,right)=>left.usedAt-right.usedAt||order(left.slot,right.slot))){if(need<=0)break;outputBases.push({slot:entry.slot,reason:"pressure"});need-=entry.bytes;}
  const extra=new Set(bazelGrowthSlotIdsV1(plan,rules)),retired:{slot:string;usedAt:number;packages:{key:string;bytes:number}[];base:number|undefined}[]=[];
  for(const row of rows){
   if(!extra.has(row.slot)||now-(row.state?.usedAt??0)<rules.pressure.growthIdleMinutes*60000)continue;
   const left=row.packages.filter(entry=>!gone.has(row.slot+"/"+entry.key)).map(entry=>({key:entry.key,bytes:entry.bytes??0})),base=row.outputBase&&!outputBases.some(entry=>entry.slot===row.slot)?row.outputBaseBytes??0:undefined;
   if(left.length||base!==undefined)retired.push({slot:row.slot,usedAt:row.state?.usedAt??0,packages:left,base});
  }
  for(const entry of retired.sort((left,right)=>left.usedAt-right.usedAt||order(left.slot,right.slot))){
   if(need<=0)break;
   for(const item of entry.packages){packages.push({slot:entry.slot,key:item.key,reason:"growth"});need-=item.bytes;}
   if(entry.base!==undefined){outputBases.push({slot:entry.slot,reason:"growth"});need-=entry.base;}
  }
 }
 const removed=new Set(packages.map(row=>row.slot+"/"+row.key));
 for(const row of rows)for(const entry of row.packages)if(entry.temporary&&!removed.has(row.slot+"/"+entry.key))temporaries.push({slot:row.slot,key:entry.key});
 const garbage={packages:packages.sort((a,b)=>order(a.slot+a.key,b.slot+b.key)),outputBases:outputBases.sort((a,b)=>order(a.slot,b.slot)),temporaries:temporaries.sort((a,b)=>order(a.slot+a.key,b.slot+b.key)),foreign:[...inventory.foreign].sort(order)};
 if(validateJsonSchemaSubset(schema.definitions.Garbage,garbage,schema).length)throw Error("Closed Bazel session garbage required");return Object.freeze(garbage);
}

/** 🔒️ Names the hierarchical restore locks of one output snapshot: every root and every ancestor directory of a root is a resource, the roots exclusive and their ancestors shared, so two restores conflict exactly when one writes inside the other's tree and restores of unrelated trees run side by side. A snapshot with more resources than the authored bound falls back to the exclusive root resource that every restore holds shared. */
export function bazelRestoreResourcesV1(roots:readonly string[],rules:BazelSessionPolicyV1=policy):readonly Readonly<{resource:string;mode:"shared"|"exclusive"}>[]{
 const modes=new Map<string,"shared"|"exclusive">([[rules.leases.restorePrefix,"shared"]]);
 for(const root of roots){
  if(typeof root!=="string"||!root.length||root.startsWith("/")||root.split("/").some(part=>part===""||part==="."||part===".."))throw Error("Invalid Bazel output snapshot");
  const parts=root.split("/");for(let length=1;length<parts.length;length++){const name=rules.leases.restorePrefix+parts.slice(0,length).join("/");if(!modes.has(name))modes.set(name,"shared");}
  modes.set(rules.leases.restorePrefix+root,"exclusive");
 }
 if(modes.size>rules.leases.restoreMaximumResources)return Object.freeze([{resource:rules.leases.restorePrefix,mode:"exclusive" as const}]);
 return Object.freeze([...modes].sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0).map(([resource,mode])=>({resource,mode})));
}

export type BazelSlotStatusV1=Readonly<{slot:string;held:boolean;state?:BazelSlotStateV1}>;
/** 📋️ Reads which slots are claimed right now (a claim is probed and dropped at once) and what each remembers. */
export async function bazelSessionStatusV1(port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,signal:AbortSignal):Promise<readonly BazelSlotStatusV1[]>{
 const rows:BazelSlotStatusV1[]=[];
 for(const slot of [...bazelSlotIdsV1(plan,"build"),...bazelSlotIdsV1(plan,"service")]){const probe=await port.claim(slot,signal),state=await port.read(slot);probe?.();rows.push(Object.freeze({slot,held:probe===undefined,...(state?{state}:{})}));}
 return Object.freeze(rows);
}
/** 🖨️ Renders the status table in the chosen language. */
export function renderBazelSessionStatusV1(rows:readonly BazelSlotStatusV1[],language:BazelLanguageV1):string{
 return [bazelSessionMessageV1("status-header",{},language),...rows.map(row=>`${row.slot.padEnd(6)}${bazelSessionMessageV1(row.held?"status-held":"status-free",{},language).padEnd(8)}${(row.state?.key??"-").padEnd(18)}${row.state?new Date(row.state.usedAt).toISOString():"-"}`)].join("\n");
}

/** 🗂️ Names the repository contents cache shared by every slot: beside the outputs on the same partition, so external repositories are fetched once and linked into each output base. */
export function bazelRepositoryContentsV1(outputs:string):string{return join(outputs,"..","contents");}
/** 🧰️ The command options that share what slots fetch: one repository contents cache for all output bases. */
export function bazelSessionCommandV1(contents:string):readonly string[]{return Object.freeze([`--repo_contents_cache=${contents}`]);}
/** ⚖️ The local resources one server hands its actions when no capacity grant bounds the invocation, so concurrently running servers never oversubscribe the host together. */
export function bazelSessionResourcesV1(plan:Pick<BazelServerPlanV1,"cpuPerServer"|"memoryPerServerMegabytes">):readonly string[]{return Object.freeze([`--local_resources=cpu=${plan.cpuPerServer}`,`--local_resources=memory=${plan.memoryPerServerMegabytes}`]);}
/** 🎛️ Reads the explicit counts (`SEMIO_BAZEL_SERVERS` for build slots, `SEMIO_BAZEL_SERVICES` for service slots, each 1 to 99) that override the derived ones; anything else is refused instead of ignored. */
export function bazelServerOverrideV1(plan:BazelServerPlanV1,environment:Readonly<Record<string,string|undefined>>):BazelServerPlanV1{
 const read=(name:string):number|undefined=>{const raw=environment[name];if(raw===undefined||raw==="")return undefined;const value=Number(raw);if(!/^[0-9]{1,2}$/u.test(raw)||value<1)throw Error(`${name} must be an integer from 1 to 99`);return value;};
 const slots=read("SEMIO_BAZEL_SERVERS")??plan.slots,services=read("SEMIO_BAZEL_SERVICES")??plan.services;if(slots===plan.slots&&services===plan.services)return plan;
 const overridden={...plan,slots,services,cpuPerServer:Math.max(1,Math.floor(plan.cpuPerServer*plan.slots/slots)),memoryPerServerMegabytes:Math.max(256,Math.floor(plan.memoryPerServerMegabytes*plan.slots/slots))};
 if(validateJsonSchemaSubset(schema.definitions.ServerPlan,overridden,schema).length)throw Error("Closed Bazel server plan required");return Object.freeze(overridden);
}
