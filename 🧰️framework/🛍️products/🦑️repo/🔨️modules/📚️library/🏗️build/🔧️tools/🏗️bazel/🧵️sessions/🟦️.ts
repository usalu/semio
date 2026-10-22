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
export type BazelInventoryV1=Readonly<{slots:readonly Readonly<{slot:string;outputBase:boolean;state?:BazelSlotStateV1;packages:readonly Readonly<{key:string;modifiedAt:number}>[]}>[];foreign:readonly string[]}>;
export type BazelGarbageV1=Readonly<{packages:readonly Readonly<{slot:string;key:string;reason:"capacity"|"count"|"age"}>[];outputBases:readonly Readonly<{slot:string;reason:"capacity"|"idle"}>[];foreign:readonly string[]}>;
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

/** 🏷️ Names the slots of a plan: short ASCII ids that keep every output base far inside the path budgets of all platforms; `s0...` are build slots, `d0...` service slots for the long-running tasks that must not pin a build slot. */
export function bazelSlotIdsV1(plan:Pick<BazelServerPlanV1,"slots"|"services">,kind:BazelSlotKindV1="build"):readonly string[]{return Object.freeze(Array.from({length:kind==="build"?plan.slots:plan.services},(_,index)=>(kind==="build"?"s":"d")+index));}

/** 🔐️ Admits only slot ids of the authored shape, so no path can leave its directory. */
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

/** 🔌️ Everything a session needs from its surroundings: the exclusive claim of a slot, the fair gate that orders waiting sessions, the remembered states, time and progress. Production maps it to repository leases and files; tests map it to memory. */
export type BazelSlotPortV1=Readonly<{
 claim(slot:string,signal:AbortSignal):Promise<(()=>void)|undefined>;
 gate(signal:AbortSignal,kind:BazelSlotKindV1):Promise<()=>void>;
 read(slot:string):Promise<BazelSlotStateV1|undefined>;
 write(slot:string,state:BazelSlotStateV1):Promise<void>;
 sleep(milliseconds:number,signal:AbortSignal):Promise<void>;
 now():number;
 progress(line:string):void|Promise<void>;
}>;
export type BazelSessionRequestV1=Readonly<{key:string;signal:AbortSignal;kind?:BazelSlotKindV1;timeoutMilliseconds?:number;language:BazelLanguageV1;preference?:number;owner?:BazelSessionOwnerV1}>;
export type BazelSessionV1=Readonly<{slot:string;key:string;mode:"warm"|"switch"|"fresh";previous?:BazelSlotStateV1;release():Promise<void>}>;

/** 💬️ Renders one authored message. */
export function bazelSessionMessageV1(id:string,values:Readonly<Record<string,string|number>>,language:BazelLanguageV1):string{
 const text=(schema.definitions.Messages.const[language] as Readonly<Record<string,string>>)[id];if(text===undefined)throw Error(`Unknown Bazel session message: ${id}`);
 return text.replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=values[name];if(value===undefined)throw Error(`Missing Bazel session message value: ${name}`);return String(value);});
}

/** 🎫️ Claims one slot of the requested class (build by default, service for sessions that launch long-running tasks) for one selection: behind the fair gate of that class it tries the slots in the order of their fitness, takes the first free one and holds it until `release`; when every slot is claimed it reports progress and polls until one frees, the signal aborts or the timeout passes. A slot has exactly one holder, so two sessions can never meet in one Bazel server lock. */
export async function acquireBazelSessionV1(port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,request:BazelSessionRequestV1,rules:BazelSessionPolicyV1=policy):Promise<BazelSessionV1>{
 requireBazelKeyV1(request.key);const kind=request.kind??"build",slots=bazelSlotIdsV1(plan,kind),language=request.language,started=port.now(),signal=request.signal;let nextProgress=0,releaseGate:(()=>void)|undefined=await port.gate(signal,kind);
 const openGate=()=>{const done=releaseGate;releaseGate=undefined;done?.();};
 try{
  while(true){
   signal.throwIfAborted();
   const states:Record<string,BazelSlotStateV1|undefined>={};for(const slot of slots)states[slot]=await port.read(slot);
   for(const slot of rankBazelSlotsV1(slots,states,request.key,request.preference)){
    const claimed=await port.claim(slot,signal);if(claimed===undefined)continue;
    let released=false,before:BazelSlotStateV1|undefined;
    try{before=await port.read(slot);openGate();const mode:BazelSessionV1["mode"]=before===undefined?"fresh":before.key===request.key?"warm":"switch";await port.progress(bazelSessionMessageV1("claimed",{slot,mode:bazelSessionMessageV1(mode,{},language)},language));
     return Object.freeze({slot,key:request.key,mode,...(before?{previous:before}:{}),async release(){if(released)return;released=true;try{await port.write(slot,nextBazelSlotStateV1(slot,before,request.key,port.now(),rules,request.owner));}finally{claimed();}}});
    }catch(error){if(!released){released=true;claimed();}throw error;}
   }
   const waited=port.now()-started;if(request.timeoutMilliseconds!==undefined&&waited>=request.timeoutMilliseconds)throw Error("Bazel session slot timed out");
   if(waited>=nextProgress){await port.progress(bazelSessionMessageV1(kind==="build"?"waiting":"waiting-service",{slots:slots.length,seconds:Math.floor(waited/1000)},language));nextProgress=waited+rules.waiting.progressMilliseconds;}
   await port.sleep(rules.waiting.pollMilliseconds,signal);
  }
 }finally{openGate();}
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

/** 📦️ Decides what to collect: output bases and packages of slots beyond the current capacity, packages beyond the retained count or age (the newest and the slot's last package always stay), output bases idle beyond the authored age, and every entry that is not a slot. A slot that a session holds is never touched. */
export function planBazelSessionGarbageV1(inventory:BazelInventoryV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,now:number,held:ReadonlySet<string>=new Set(),rules:BazelSessionPolicyV1=policy):BazelGarbageV1{
 if(validateJsonSchemaSubset(schema.definitions.Inventory,inventory,schema).length)throw Error("Closed Bazel session inventory required");
 const valid=new Set([...bazelSlotIdsV1(plan,"build"),...bazelSlotIdsV1(plan,"service")]),packages:{slot:string;key:string;reason:"capacity"|"count"|"age"}[]=[],outputBases:{slot:string;reason:"capacity"|"idle"}[]=[];
 for(const row of inventory.slots){
  if(held.has(row.slot))continue;
  if(!valid.has(row.slot)){if(row.outputBase)outputBases.push({slot:row.slot,reason:"capacity"});for(const entry of row.packages)packages.push({slot:row.slot,key:entry.key,reason:"capacity"});continue;}
  if(row.outputBase&&row.state!==undefined&&now-row.state.usedAt>rules.retention.outputBaseMaximumIdleDays*day)outputBases.push({slot:row.slot,reason:"idle"});
  const newest=[...row.packages].sort((left,right)=>right.modifiedAt-left.modifiedAt||(left.key<right.key?-1:1));
  newest.forEach((entry,index)=>{if(index===0||entry.key===row.state?.key)return;if(index>=rules.retention.packagesPerSlot)packages.push({slot:row.slot,key:entry.key,reason:"count"});else if(now-entry.modifiedAt>rules.retention.packageMaximumAgeDays*day)packages.push({slot:row.slot,key:entry.key,reason:"age"});});
 }
 const order=(left:string,right:string)=>left<right?-1:left>right?1:0,garbage={packages:packages.sort((a,b)=>order(a.slot+a.key,b.slot+b.key)),outputBases:outputBases.sort((a,b)=>order(a.slot,b.slot)),foreign:[...inventory.foreign].sort(order)};
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
