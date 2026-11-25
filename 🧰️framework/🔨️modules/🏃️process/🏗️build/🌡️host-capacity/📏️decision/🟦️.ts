import {timingSafeEqual} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {resolveLanguageV1,type OsLanguageListV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";

const protocol=schema.definitions.Protocol.const;

/** 📚️ The authored policy of the measured-demand estimates: decay half-life, the quantile of the learned peaks and the margin on it, how many effective samples earn full trust, the bounds of the store, the bounds of a learned size and the active time under which a class counts as a quick, interactive rebuild. */
export type HostEstimatePolicyV1=Readonly<{halfLifeMilliseconds:number;quantileBasisPoints:number;marginBasisPoints:number;minimumSamples:number;maximumSamples:number;maximumClasses:number;minimumRatioBasisPoints:number;maximumRatioBasisPoints:number;minimumMegabytes:number;quickMilliseconds:number;maximumShareBasisPoints:number;fallbackCeilingMegabytes:number;defaultRampMilliseconds:number;maximumRampMilliseconds:number;phaseReferenceMegabytes:number;rebuildPriorMegabytes:number;rebuildShareBasisPoints:number;rebuildMinimumMegabytes?:number;maximumPhaseMegabytes?:number;rebuildQuantileBasisPoints?:number;maximumRebuildSlots:number;quietReferenceMegabytes:number;quietMaximumMegabytes:number;quietFollowUpMilliseconds:number}>;
/** 🌡️ The authored action kinds a job estimate is keyed by; identical to the worker pool vocabulary, never a build tool name. */
export type HostActionKindV1="test"|"check"|"generate"|"build"|"script"|"service";
/** 📐️ One kind's estimate: cores it keeps busy and megabytes it holds at its peak. */
export type HostResourceV1=Readonly<{cpu:number;megabytes:number}>;
/** 🧰️ The resources a target holds at its peak when it runs several workers of its own (end-to-end suites): a target whose name matches `pattern` is sized at least at `cpu` cores and `megabytes` per job, whatever kind the worker pool classifies it as (`Envelope` of the schema). */
export type HostEnvelopeV1=Readonly<{name:string;pattern:string;cpu:number;megabytes:number}>;
/** 🧢️ The most a class of runs may ever ask for (`Ceiling` of the schema): a declared bound on the class key that holds until the class measured enough runs to be capped by its own peaks. */
export type HostCeilingV1=Readonly<{name:string;pattern:string;megabytes:number}>;
/** 🧭️ How one operation sizes its request: `tasks` multiplies per-job estimates by the job count, `fixed` reserves one block; `minimumMegabytes` (fixed shape) is the least memory the operation starts with when its fixed block does not fit in full (see `HostRequestV1`); `forecast` lets a run of the operation hold only a small quiet reservation until work has to run (only where the tool gates its executors). */
export type HostOperationV1=Readonly<{name:string;shape:"tasks"|"fixed";kind?:HostActionKindV1;fixedMegabytes:number;cpu:number;priority:"batch"|"interactive";forecast?:boolean;minimumMegabytes?:number}>;
/** 📜️ The authored host capacity policy (`Policy` of the schema). */
export type HostCapacityPolicyV1=Readonly<{version:1;enabled:boolean;memory:Readonly<{floorBasisPoints:number;floorMinimumMegabytes:number;floorMaximumMegabytes:number;budgetBasisPoints:number;rampMilliseconds:number;maximumRamping:number;rampIdleAfterMilliseconds?:number;rampIdleObservedBasisPoints?:number;rampIdleKeepMegabytes?:number;rampIdleKeepBasisPoints?:number;settledAfterMilliseconds?:number;settledKeepBasisPoints?:number;childStartMegabytes?:number;maximumQuiet:number;quietSoftStopGrants:number}>;swap:Readonly<{ceilingBasisPoints:number;activeBasisPointsPerSecond:number;outBasisPointsPerSecond:number;shortBasisPoints:number}>;pressure:Readonly<{psiWarnPercent:number;psiCriticalPercent:number;availableWarnBasisPoints:number;availableCriticalBasisPoints:number;compressedWarnBasisPoints:number;compressedCriticalBasisPoints:number}>;load:Readonly<{perCoreCeiling:number;overrideAfterMilliseconds:number}>;files:Readonly<{ceilingBasisPoints:number}>;attaches:Readonly<{maximum:number}>;wait:Readonly<{pollMilliseconds:number;progressMilliseconds:number;heartbeatMilliseconds:number;staleMilliseconds:number;lockStaleMilliseconds:number;sampleMaximumAgeMilliseconds:number;maximumWaiters:number;bypassMilliseconds:number;maximumBypasses:number;agingMilliseconds:number;lendAfterMilliseconds:number;jitterBasisPoints:number;deadlineInteractiveMilliseconds?:number;deadlineBatchMilliseconds?:number;starvedMilliseconds?:number}>;kinds:Readonly<Record<HostActionKindV1,HostResourceV1>>;operations:readonly HostOperationV1[];estimates:HostEstimatePolicyV1;envelopes:readonly HostEnvelopeV1[];classCeilings:readonly HostCeilingV1[]}>;
/** 🖥️ One sample of the host (`Host` of the schema); every platform reduces to these fields, `null` marking a counter the platform does not offer, `counters:"partial"` marking a sample whose platform probe failed (pressure and swap unknown; unknown is never read as pressure, the free memory of the operating system decides): the machine-wide open file table (used and maximum) and the number of attached disk images join memory, swap and load. */
export type HostSampleV1=Readonly<{version:1;atMs:number;platform:"darwin"|"linux"|"win32";cores:number;totalMegabytes:number;availableMegabytes:number;pressure:"normal"|"warn"|"critical";swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;load:number|null;openFiles:number|null;openFilesMaximum:number|null;attachedImages:number|null;compressedMegabytes?:number|null;unmanagedMegabytes?:number;counters?:"partial"}>;
/** 🎫️ What one operation asks for (`Request` of the schema); `label` names what it builds (the target of `repo run`), so the progress lines of others can say whom they stand behind; `minimumMegabytes` is the least memory a request that can run smaller still starts with: when its full reservation does not fit but the free memory covers this minimum it is admitted with what is free (`shrunk`), when this minimum can never fit the machine it is refused at once (`never`), and when nothing of ours runs and not even the minimum is free after `load.overrideAfterMilliseconds` it is refused (`starved`), because waiting for memory that no run of ours holds cannot help; `essential` (with `elastic`) marks the FIRST worker of a farm: it is admitted with its minimum jobs whatever the free memory and any soft stop say, as long as no grant of the same label, holder and session runs and no hard memory stop holds, because a farm without a child makes its parent run every owner itself, which is the largest process doing the slowest work; the floor applies to the additional children; `elastic` marks a supplementary request of a run that already holds a grant and goes on without it (the project farm): it is decided after every other request and never makes one wait; `envelopePerJobMegabytes` is the per-job memory a tool may use when it differs from the reserved `perJobMegabytes`; `rampMilliseconds` is how long the run needs to reach its peak (what its class learned, else the policy ramp) and `rampSlot` marks a reservation sized below the default, which may only ramp in a bounded number of slots at once; `quiet` marks the small forecast request of a run expected to execute nothing, which passes the queue barrier (within `memory.maximumQuiet` such grants) because it can starve nobody and is upgraded to its full request if work has to run. */
export type HostRequestV1=Readonly<{id:string;operation:string;priority:"batch"|"interactive"|"lent";minimumJobs:number;preferredJobs:number;fixedMegabytes:number;perJobMegabytes:number;cpuPerJob:number;envelopePerJobMegabytes?:number;rampMilliseconds?:number;rampSlot?:boolean;quiet?:boolean;elastic?:boolean;essential?:boolean;minimumMegabytes?:number;label?:string;holder?:string}>;
/** ⏳️ A request in the queue (`Waiter` of the schema); `upgrade` marks the request of a held quiet grant to grow into its full request: it replaces the grant of the same id and keeps the queue position the run arrived with; `grow` (with `upgrade`) marks the request of a running grant admitted with fewer jobs than it asked for to grow its jobs now that memory freed up. */
export type HostWaiterV1=Readonly<{request:HostRequestV1;arrivedAtMs:number;pid:number;session?:string;weight?:number;upgrade?:boolean;grow?:boolean;rebuild?:string}>;
/** 🔐️ A live admission (`Grant` of the schema); `phase` is the phase the run is in once it reports phases; `quiet` marks a forecast grant that holds only the small reservation of a run expected to execute nothing, `arrivedAtMs` is the queue position it keeps for its upgrade and `upgradedAtMs` when it grew into its full request; `observedMegabytes` is what its process tree holds right now; `lentSinceMs` marks a grant whose process only waits in a queue (its reservation and cores are lent back to the ledger); `resumedAtMs` restarts the ramp when that process continues; `rampMilliseconds` is the time the grant counts its full reservation (the time its class needs to reach its peak) and `rampSlot` marks a shrunk reservation that occupies a ramp slot while it ramps; `grownAtMs` is when a grant admitted with fewer jobs than it asked for last grew its jobs. */
export type HostGrantV1=Readonly<{id:string;operation:string;jobs:number;cpu:number;reservedMegabytes:number;admittedAtMs:number;observedMegabytes:number|null;pid:number;bootAtMs:number;session?:string;lentSinceMs?:number;resumedAtMs?:number;handover?:HostHandoverV1;rampMilliseconds?:number;rampSlot?:boolean;quiet?:boolean;arrivedAtMs?:number;upgradedAtMs?:number;grownAtMs?:number;label?:string;holder?:string;rebuildScope?:string;rebuildBookedAtMs?:number;phase?:"assembly"|"plan"|"rebuild"|"execution"}>;
/** 🤝️ The offer a process makes while its grant waits for the command that replaces it: a one-time token and the identity (id and start time) of the offering process, so only its own replacement can take the grant over. */
export type HostHandoverV1=Readonly<{token:string;fromPid:number;fromStarted:string;offeredAtMs:number}>;
/** 🧾️ Why a request was admitted or kept waiting. */
export type HostReasonV1="fits"|"reduced"|"override"|"memory"|"swap"|"swap-activity"|"pressure-critical"|"pressure-warn"|"files"|"attaches"|"load"|"queue-order"|"ramp"|"quiet-limit"|"shrunk"|"never"|"starved"|"lent"|"session-cap";
/** ⚖️ The verdict for one request. */
export type HostDecisionV1=Readonly<{id:string;verdict:"admit"|"wait";jobs:number;reservedMegabytes:number;reason:HostReasonV1}>;
/** 📊️ The whole evaluation: the derived headroom and one decision per waiter in service order. */
export type HostEvaluationV1=Readonly<{floorMegabytes:number;unrealizedMegabytes:number;headroomMegabytes:number;unmanagedMegabytes?:number;binding:"available"|"budget";loadRoom:number|null;decisions:readonly HostDecisionV1[]}>;
/** 🧩️ The tool-neutral resource envelope a build tool renders into its own flags. */
export type HostPlanV1=Readonly<{version:1;id:string;jobs:number;cpu:number;memoryMegabytes:number;fixedMegabytes:number;floorMegabytes:number}>;
/** 🏷️ Names of the closed definitions a value can be required to match. */
export type HostDefinitionV1="Reservation"|"Policy"|"Host"|"Request"|"RebuildRequest"|"RebuildRefusal"|"Waiter"|"Grant"|"Decision"|"Evaluation"|"Plan"|"GrantLiveness"|"Handover"|"HandoverFacts"|"Estimates"|"EstimateStore"|"Outcome"|"Estimate";

/** 🛂️ Requires a value to match one closed definition of the host capacity schema. */
export function requireHostCapacityV1<T>(definition:HostDefinitionV1,value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw hostCapacityErrorV1("definitionRequired",{definition,detail:errors[0]!});
 return value as T;
}

/** 🔍️ Answers whether a value matches one closed definition without throwing. */
export function isHostCapacityV1(definition:HostDefinitionV1,value:unknown):boolean{
 return validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value).length===0;
}

/** 📕️ Closed liveness facts shared by ordinary accounting and inherited admission. */
export type HostGrantLivenessV1=Readonly<{grant:HostGrantV1;expectedId:string;pidAlive:boolean;nowMs:number;bootAtMs:number;modifiedAtMs:number}>;

/** 🚀️ Requires the same live instance, boot and heartbeat before a grant can carry admission. */
export function isCurrentHostGrantV1(value:unknown,policy:HostCapacityPolicyV1):boolean{
 if(!isHostCapacityV1("GrantLiveness",value))return false;
 const facts=value as HostGrantLivenessV1;
 return facts.pidAlive&&facts.grant.id===facts.expectedId&&Math.abs(facts.grant.bootAtMs-facts.bootAtMs)<=schema.definitions.Protocol.const.bootToleranceMilliseconds&&facts.nowMs-facts.modifiedAtMs<=policy.wait.staleMilliseconds;
}

/** ⚓️ The boot instant of this boot, written once beside the grants: `bootAtMs` (the wall clock minus the uptime at the first reader, to the second) and the uptime of that reader. */
export type HostBootAnchorV1=Readonly<{bootAtMs:number;uptimeMs:number}>;

/** 🕰️ Decides the anchor of the boot a process runs in. The instant a grant carries must not follow the wall clock (a step of the clock, by the user, a time service or a resumed virtual machine, moves `now - uptime` by the step wherever the uptime is monotonic, and made every running grant look like a grant of another boot): an anchor taken at an uptime that is not later than ours belongs to this boot and stays as it is; none, or one taken later than now (the uptime ran backwards, so the machine restarted since), is replaced by one taken from the wall clock and the uptime. A restart that is quicker than a heartbeat of the grants is also refused by their age. Pure and deterministic. */
export function hostBootAnchorV1(anchor:HostBootAnchorV1|undefined,wallMs:number,uptimeMs:number):HostBootAnchorV1{
 return anchor!==undefined&&uptimeMs>=anchor.uptimeMs?anchor:Object.freeze({bootAtMs:Math.round((wallMs-uptimeMs)/1000)*1000,uptimeMs});
}

/** 🛑️ Why a grant offered by a replaced command cannot be taken over. */
export type HostHandoverRefusalV1="gone"|"token"|"expired"|"identity"|"operation";
/** 🏭️ What a process knows when it is offered the grant of the command it replaces: the grant row it read (null when gone), whether that row is a current live grant, the offered text of its environment (`<id>.<token>`), the operation it asks for, the identity of its parent process and the clock. */
export type HostHandoverFactsV1=Readonly<{grant:HostGrantV1|null;grantCurrent:boolean;offered:string;operation:string;parent:Readonly<{pid:number;started:string}>|null;nowMs:number}>;
/** ☁️ The verdict on one offer: take the grant over or the first closed reason it is refused. */
export type HostHandoverVerdictV1=Readonly<{adopt:true;id:string}>|Readonly<{adopt:false;reason:HostHandoverRefusalV1}>;

/** ⚙️ Decides whether the offered grant may pass to this process: the offer must be well formed and name a current grant carrying exactly that one-time token, be younger than the declared bound, have been made by this process's own parent (id and start time, which a recycled id cannot fake) and be for the same operation; the first failing check names the reason, so an unrelated process never receives a grant. */
export function decideHostHandoverV1(facts:HostHandoverFactsV1):HostHandoverVerdictV1{
 const protocol=schema.definitions.Protocol.const,match=new RegExp(`^(${schema.definitions.Grant.properties.id.pattern.slice(1,-1)})\\.(${schema.definitions.Handover.properties.token.pattern.slice(1,-1)})$`,"u").exec(facts.offered);
 if(!match)return{adopt:false,reason:"token"};
 const grant=facts.grant,id=match[1]!,token=match[2]!;
 if(grant===null||!facts.grantCurrent||grant.id!==id)return{adopt:false,reason:"gone"};
 const offer=grant.handover,left=Buffer.from(token),right=Buffer.from(offer?.token??"");
 if(offer===undefined||left.length!==right.length||!timingSafeEqual(left,right))return{adopt:false,reason:"token"};
 if(facts.nowMs-offer.offeredAtMs>protocol.handoverMaximumAgeMilliseconds)return{adopt:false,reason:"expired"};
 if(facts.parent===null||facts.parent.pid!==offer.fromPid||facts.parent.started!==offer.fromStarted||grant.pid!==offer.fromPid)return{adopt:false,reason:"identity"};
 if(grant.operation!==facts.operation)return{adopt:false,reason:"operation"};
 return{adopt:true,id};
}

/** 💬️ Names a handover refusal in the chosen language. */
export function describeHostHandoverRefusalV1(language:"en"|"de",reason:HostHandoverRefusalV1):string{
 return schema.definitions.Messages.const[language].handoverReasons[reason];
}

/** 📖️ Reads the authored policy and refuses anything the closed schema or its ordering rules reject. */
export function readHostCapacityPolicyV1(value:unknown):HostCapacityPolicyV1{
 const policy=requireHostCapacityV1<HostCapacityPolicyV1>("Policy",value);
 if(policy.memory.floorMinimumMegabytes>policy.memory.floorMaximumMegabytes)throw hostCapacityErrorV1("floorOrder");
 if(policy.pressure.psiWarnPercent>policy.pressure.psiCriticalPercent||policy.pressure.availableCriticalBasisPoints>policy.pressure.availableWarnBasisPoints||policy.pressure.compressedWarnBasisPoints>policy.pressure.compressedCriticalBasisPoints)throw hostCapacityErrorV1("pressureOrder");
 return policy;
}

/** 🛋️ The memory kept for the interactive apps of the host (the editor, the assistant and its helpers, the terminal): a share of its memory between the authored bounds (`interactiveReserve*` of the memory policy, 0 where the policy declares none), never above what the host has. Admission divides only what is left of the machine beside it, so reservations are sized against the total memory minus everything that is not granted, never below this reserve. */
export function hostInteractiveReserveMegabytesV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">):number{
 const memory=policy.memory as typeof policy.memory&Partial<Record<"interactiveReserveBasisPoints"|"interactiveReserveMinimumMegabytes"|"interactiveReserveMaximumMegabytes",number>>;
 if(memory.interactiveReserveBasisPoints===undefined)return 0;
 const share=Math.floor(host.totalMegabytes*memory.interactiveReserveBasisPoints/10000);
 return Math.min(host.totalMegabytes,Math.max(memory.interactiveReserveMinimumMegabytes??0,Math.min(memory.interactiveReserveMaximumMegabytes??host.totalMegabytes,share)));
}

/** 🔒️ The most memory one agent session may hold charged at once (what its grants are booked at in the decision: the reservation while a grant ramps up, what it measurably holds after, so a session whose grants reserved far more than they use is not held at its reservations) (`sessionCapBasisPoints` of the host memory, undefined where the policy declares none): a further request of a session that already holds reservations waits (reason `session-cap`) while the reservations it holds plus its least need would pass the cap; a session that holds nothing is always served, so a single large request still runs. A request that has waited `wait.agingMilliseconds` is starving and no longer judged by the cap: it heads the line (see `orderHostWaitersV1`) and, when memory alone holds it back, becomes the barrier that drains the others, so the cap shapes sharing but never starves a request. The cap shares the host between agent sessions, so it binds only while a second agent session holds a grant or waits: alone on a host (a fleet worker, whose sidecar, ready stage and graph reads are one session) a request is judged by memory alone, otherwise the graph read of a command waits for ever behind the grants of that same command (g1-m10, 25,300 s).
 * @see hostInteractiveReserveMegabytesV1
 * @see ../🧪️tests/🛋️reserve/🟦️.ts */
export function hostSessionCapMegabytesV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">):number|undefined{
 const basis=(policy.memory as typeof policy.memory&{sessionCapBasisPoints?:number}).sessionCapBasisPoints;
 return basis===undefined?undefined:Math.floor(host.totalMegabytes*basis/10000);
}

/** 🧱️ The free-memory floor of one host: a share of its memory between the authored bounds, never above what the host has. */
export function hostFloorMegabytesV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">):number{
 const share=Math.floor(host.totalMegabytes*policy.memory.floorBasisPoints/10000);
 return Math.min(host.totalMegabytes,Math.max(policy.memory.floorMinimumMegabytes,Math.min(policy.memory.floorMaximumMegabytes,share)));
}

/** 🪫️ How long a request that cannot be served waits at the most, whoever holds the memory: past it a request that fits no shrink (the room above the floor stays under its least need, or a hard stop holds) is refused with the reason `starved` and a named error, so the stage that asked fails and retries instead of waiting for ever; unbounded only for a policy without the field.
 * @see decideHostCapacityV1 */
export function hostStarvedMillisecondsV1(policy:HostCapacityPolicyV1):number{
 return policy.wait.starvedMilliseconds??Infinity;
}

/** ⏱️ How long a request of this priority waits for memory before its deadline rules apply (an interactive request sooner than a batch one); no deadline is configured for a policy without the fields, so the wait is then unbounded as it was.
 * @see decideHostCapacityV1 */
export function hostWaitDeadlineMillisecondsV1(policy:HostCapacityPolicyV1,priority:HostRequestV1["priority"]):number{
 return (priority==="batch"?policy.wait.deadlineBatchMilliseconds:policy.wait.deadlineInteractiveMilliseconds)??Infinity;
}

/** 🔢️ Orders waiters for service: interactive before batch, and the lent class of another machine after everything (grow and elastic requests after every other request, and interactive before batch among themselves), then, within one class, the starving waiters (`starving`: waited past `wait.agingMilliseconds`) before the others and among themselves by arrival, so the oldest request is always the head of the line and a large request cannot be overtaken for ever by the smaller requests of lighter sessions, then the agent session holding the least charged memory per unit of budget weight (the `ledger` rows carry what each grant is charged; weighted fair share between sessions; waiters without a session count as holding nothing), then by arrival, then by id; grow requests of running grants and elastic requests come after every other request.
 * @see decideHostCapacityV1 */
export function orderHostWaitersV1(waiters:readonly HostWaiterV1[],ledger:readonly Pick<HostGrantV1,"session"|"reservedMegabytes">[]=[],starving:(waiter:HostWaiterV1)=>boolean=()=>false):HostWaiterV1[]{
 const rank=(waiter:HostWaiterV1)=>(waiter.grow===true||waiter.request.elastic===true?2:0)+(waiter.request.priority==="interactive"?0:waiter.request.priority==="lent"?4:1),held=(waiter:HostWaiterV1)=>waiter.session===undefined?0:ledger.reduce((sum,grant)=>sum+(grant.session===waiter.session?grant.reservedMegabytes:0),0),weight=(waiter:HostWaiterV1)=>waiter.weight??1;
 return [...waiters].sort((left,right)=>{
  const hungry=starving(left),other=starving(right);
  return rank(left)-rank(right)||(hungry===other?0:hungry?-1:1)||(hungry?0:held(left)*weight(right)-held(right)*weight(left))||left.arrivedAtMs-right.arrivedAtMs||(left.request.id<right.request.id?-1:left.request.id>right.request.id?1:0);
 });
}

/** 🌋️ Whether the host is under memory pressure now: pressure above normal or the swap stop; a run measured in such a state does not teach the estimates. */
export function hostUnderPressureV1(policy:HostCapacityPolicyV1,host:HostSampleV1):boolean{
 return host.pressure!=="normal"||hostSwapStopV1(policy,host)!==undefined;
}
/** 🚧️ Names what holds every admission back without being a hard stop: raised memory pressure, an open file table above its ceiling share or as many attached disk images as the maximum; undefined when none applies. These can be waited out, and are overridden only when nothing of ours is running. */
export function softHostReasonV1(policy:HostCapacityPolicyV1,host:HostSampleV1):"pressure-warn"|"files"|"attaches"|undefined{
 if(host.pressure==="warn")return "pressure-warn";
 if(host.openFiles!==null&&host.openFilesMaximum!==null&&host.openFiles*10000>host.openFilesMaximum*policy.files.ceilingBasisPoints)return "files";
 if(host.attachedImages!==null&&host.attachedImages>=policy.attaches.maximum)return "attaches";
 return undefined;
}

/** 🤲️ What one request of a holder was lent: the holder, the megabytes per job and fixed block the request would have been charged, and the megabytes the holder keeps for itself. */
export type HostLendV1=Readonly<{holder:string;fixedMegabytes:number;perJobMegabytes:number;keptMegabytes:number}>;
/** 👁️ The world the decision reads once holders lend: the ledger with the charge of every ramping holder reduced by what its children are charged, the requests that are not lent, the lent requests priced at nothing and capped at the jobs the holder can lend, and what each lent request is worth. */
export type HostLentViewV1=Readonly<{ledger:readonly HostGrantV1[];waiters:readonly HostWaiterV1[];lentWaiters:readonly HostWaiterV1[];lent:ReadonlyMap<string,HostLendV1>}>;

/** 🤲️ Lets a grant that is still ramping up to its reservation lend the unused part of it to the elastic requests that name it (`holder`), the workers of its own run: pure preprocessing of the view, so the decision itself is unchanged and its SQLite oracle stays valid on every view. The holder keeps the larger of what its own tree holds (its observation without its children's) and `Protocol.lendKeepBasisPoints` of its reservation; it lends at most its unrealized part (the reservation less what its tree holds) and at most the reservation less that keep and less what its children are already charged; a request is lent whole jobs only, in arrival order, as many as fit (at least its minimum jobs, else it takes the ordinary path and is charged to the host as any request); a lent request costs the host nothing, since its memory comes out of the reservation the host admitted already, and the holder is charged what its children are charged less, so the sum stays the holder's reservation and the memory used above the reservations does not grow. A holder that no longer ramps, is lent out, quiet or itself a child lends nothing. Without a request that names a holder and without a grant that names one the view is the input.
 * @see ../🧪️tests/🤲️lend/🐍️oracle/🐍️.py */
export function lendHostHoldersV1(policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):HostLentViewV1{
 const keep=protocol.lendKeepBasisPoints,ramping=(grant:HostGrantV1)=>nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds);
 const active=ledger.filter(grant=>grant.lentSinceMs===undefined),holders=new Map(active.filter(grant=>grant.holder===undefined&&grant.quiet!==true&&ramping(grant)).map(grant=>[grant.id,grant])),children=new Map<string,HostGrantV1[]>();
 for(const grant of active)if(grant.holder!==undefined&&holders.has(grant.holder))(children.get(grant.holder)??children.set(grant.holder,[]).get(grant.holder)!).push(grant);
 const named=waiters.filter(waiter=>waiter.request.holder!==undefined&&holders.has(waiter.request.holder)&&waiter.request.elastic===true&&waiter.upgrade!==true&&waiter.grow!==true);
 if(children.size===0&&named.length===0)return Object.freeze({ledger,waiters,lentWaiters:Object.freeze([]),lent:new Map()});
 const room=new Map<string,Readonly<{keptMegabytes:number;remaining:number}>>(),charged=new Map<string,number>();
 for(const id of new Set([...children.keys(),...named.map(waiter=>waiter.request.holder!)])){
  const holder=holders.get(id)!,kids=children.get(id)??[],held=kids.reduce((sum,grant)=>sum+grant.reservedMegabytes,0),seen=kids.reduce((sum,grant)=>sum+(grant.observedMegabytes??0),0),own=holder.observedMegabytes===null?0:Math.max(0,holder.observedMegabytes-seen);
  const kept=Math.max(own,Math.floor(holder.reservedMegabytes*keep/10000)),unrealized=Math.max(0,keptHostMegabytesV1(policy,holder,nowMs)-(holder.observedMegabytes??0));
  room.set(id,Object.freeze({keptMegabytes:kept,remaining:Math.max(0,Math.min(holder.reservedMegabytes-kept-held,unrealized-held))}));charged.set(id,held);
 }
 const lent=new Map<string,HostLendV1>(),priced=new Map<string,HostWaiterV1>();
 for(const waiter of [...named].sort((left,right)=>left.arrivedAtMs-right.arrivedAtMs||(left.request.id<right.request.id?-1:left.request.id>right.request.id?1:0))){
  const request=waiter.request,id=request.holder!,state=room.get(id)!;
  let jobs=0;for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(request.fixedMegabytes+candidate*request.perJobMegabytes<=state.remaining){jobs=candidate;break;}
  if(jobs===0)continue;
  room.set(id,Object.freeze({...state,remaining:state.remaining-request.fixedMegabytes-jobs*request.perJobMegabytes}));
  lent.set(request.id,Object.freeze({holder:id,fixedMegabytes:request.fixedMegabytes,perJobMegabytes:request.perJobMegabytes,keptMegabytes:state.keptMegabytes}));
  priced.set(request.id,Object.freeze({...waiter,request:Object.freeze({...request,preferredJobs:jobs,fixedMegabytes:0,perJobMegabytes:0})}));
 }
 return Object.freeze({
  ledger:Object.freeze(ledger.map(grant=>charged.has(grant.id)?Object.freeze({...grant,reservedMegabytes:Math.max(0,grant.reservedMegabytes-charged.get(grant.id)!)}):grant)),
  waiters:Object.freeze(waiters.filter(waiter=>!priced.has(waiter.request.id))),
  lentWaiters:Object.freeze(waiters.filter(waiter=>priced.has(waiter.request.id)).map(waiter=>priced.get(waiter.request.id)!)),
  lent,
 });
}

/** 🪶️ What a ramping grant is still charged for memory it does not hold: its reservation, except that a farm child (a grant that names a holder or carries the label `graph-farm`, whose memory sits inside the tree of the read and so inside what the host already reports used) is charged `childStartMegabytes` per job from its first second, or what it measurably holds when that is more, never more than it reserved, so an idle or booting child holds only its resident memory; and that a grant which held its ramp slot `rampIdleAfterMilliseconds` while using less than `rampIdleObservedBasisPoints` of it is idle, not growing, and keeps only the larger of `rampIdleKeepMegabytes` and `rampIdleKeepBasisPoints` of what it really holds (never more than it reserved): a reservation is what a job needs, so the unused rest of a worst-case estimate is released to the others; without those two numbers the whole reservation stays charged. A grant that has run `settledAfterMilliseconds` since it was admitted or resumed and has been measured is settled: its estimate is a guess and its measurement is a fact, so it is charged `settledKeepBasisPoints` of what it holds (never under `rampIdleKeepMegabytes`, never more than it reserved), re-read at every heartbeat, so growth is followed within one beat and the unused rest of a worst-case estimate (a read of an eight-hour refresh booked 14.3 GB for 4.3 GB held) is spare for the farm and the pools at once. A pure function of its arguments.
 * @see ../🧪️tests/🟦️.ts */
export function keptHostMegabytesV1(policy:HostCapacityPolicyV1,grant:Pick<HostGrantV1,"reservedMegabytes"|"observedMegabytes"|"admittedAtMs"|"resumedAtMs"|"jobs"|"holder"|"label">,nowMs:number):number{
 const rules=policy.memory;
 if(rules.childStartMegabytes!==undefined&&(grant.holder!==undefined||grant.label==="graph-farm"))return Math.min(grant.reservedMegabytes,Math.max(grant.observedMegabytes??0,grant.jobs*rules.childStartMegabytes));
 const settled=rules.settledAfterMilliseconds!==undefined&&rules.settledKeepBasisPoints!==undefined&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)>=rules.settledAfterMilliseconds&&typeof grant.observedMegabytes==="number"&&grant.observedMegabytes>0;
 if(settled)return Math.min(grant.reservedMegabytes,Math.max(rules.rampIdleKeepMegabytes??0,Math.ceil(grant.observedMegabytes!*rules.settledKeepBasisPoints!/10000)));
 const idle=rules.rampIdleAfterMilliseconds!==undefined&&rules.rampIdleObservedBasisPoints!==undefined&&rules.rampIdleKeepMegabytes!==undefined&&rules.rampIdleKeepBasisPoints!==undefined&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)>=rules.rampIdleAfterMilliseconds&&typeof grant.observedMegabytes==="number"&&grant.observedMegabytes*10000<grant.reservedMegabytes*rules.rampIdleObservedBasisPoints;
 return idle?Math.min(grant.reservedMegabytes,Math.max(rules.rampIdleKeepMegabytes!,Math.ceil(grant.observedMegabytes!*rules.rampIdleKeepBasisPoints!/10000))):grant.reservedMegabytes;
}

/** 🧮️ Decides, for a host sample, the live grants and the queue, which requests start now and with how many jobs; a pure function of its arguments. Reservations count in full (or by what the grant already holds when that is more) only while a grant ramps up, i.e. for the time its class needs to reach its peak (`rampMilliseconds` of the grant, else `memory.rampMilliseconds`; restarted by `resumedAtMs`), then the memory it really holds speaks for itself; at most `memory.maximumRamping` grants whose reservation was shrunk below the default (`rampSlot`) may ramp at once, so a wrong estimate can overshoot with a bounded number of runs only. A lent grant (its process only waits in a queue) counts neither memory nor cores nor fair-share weight and does not count as running for the override. A waiter refused for memory or load does not hold later ones back: they pass while they fit. It becomes a strict barrier only if finishing our own grants would admit it (foreign pressure never creates a barrier, draining cannot help it) and it has waited `wait.bypassMilliseconds` or been overtaken `wait.maximumBypasses` times, counting grants admitted since it arrived; so it cannot starve. A batch waiter older than `wait.agingMilliseconds` ranks as interactive, so a stream of interactive requests cannot starve it. The load only scales jobs down: foreign load (editors, indexers) never refuses a request whose memory fits, it is admitted with its minimum jobs; the cores of our own grants plus the request are what must stay under `load.perCoreCeiling` per core. An upgrade request (`upgrade`) replaces the live grant of its id: it is decided as the same request in the world where that grant is released and the memory it observably holds is returned, so only the difference has to fit (an upgrade that grows nothing always fits), and its cores, ramp slot and the override count without the old grant; except that an upgrade is never refused for memory while every other live grant is a quiet grant that itself waits to grow (it executes nothing and releases nothing before it grows, so waiting for it is a circular wait that also blocks the queue behind the barrier): nothing of ours competes, so the first such upgrade in queue order is admitted at the largest size that fits now (never under what the grant holds, reason `override`) with as many jobs as that memory carries at the learned footprint of one job (`fixedMegabytes` + jobs x `perJobMegabytes`, never fewer than one), and the run goes on, instead of waiting for memory that foreign processes or the other waiting runs hold. A quiet request passes the strict barrier while fewer than `memory.maximumQuiet` quiet grants are live, never becomes a barrier head and does not count as overtaking the heads, because it is small, bounded and every hard stop still applies to it. A grow request (`grow`, which only the holder of a grant that runs with fewer jobs than it asked for evaluates, never written to the queue) is decided after every waiting request and takes nothing from them: it waits `queue-order` while any other request waits or the queue is held behind a barrier, `ramp` while the reservation of its previous growth still ramps up (not yet observed, so it cannot grow twice on the same memory; the admission reservation counts in full while it ramps, so a run may grow right after it was admitted), a soft stop like any request, and otherwise grows to the most jobs whose memory fits with the memory of `growSpareJobs` more jobs left free (the protocol's hysteresis margin) and whose cores fit the load room; it never shrinks a reservation, never becomes a barrier and never takes the partial path of an upgrade. An elastic request (`elastic`, the workers a running build asks for beside its own grant and does without) is decided after every other request like a grow request: it waits `queue-order` while any other request waits or a barrier holds, takes only what fits now and never becomes a barrier head or an override. Our own booked cores never keep an agent session from its first job: a request whose session holds no other live grant (and was given none earlier in the same evaluation) is admitted with its minimum jobs whenever its memory fits and no soft stop applies, even when the cores of our grants already reach `load.perCoreCeiling` per core (while they stay below that ceiling plus one core per core of the host), so every agent progresses and the ceiling bounds everything beyond one job per session. An upgrade whose whole request fits the memory but not the cores our other grants book is admitted (`reduced`) with the most jobs those cores leave once the minimum jobs of every fresh or quiet request served after it are set aside (so a growth never takes the cores a later small request needs, with or without an agent session), or with one job under the session rule, and keeps the memory of its whole request (a run's memory does not shrink with its jobs; the grower adds jobs when cores free up), instead of waiting for another run to end. */
export function evaluateHostCapacityV1(policy:HostCapacityPolicyV1,host:HostSampleV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):HostEvaluationV1{
 const view=lendHostHoldersV1(policy,ledger,waiters,nowMs);
 if(view.lentWaiters.length===0)return decideHostCapacityV1(policy,host,view.ledger,view.waiters,nowMs);
 const free=Object.freeze({...host,availableMegabytes:host.totalMegabytes}),unpriced=Object.freeze(view.ledger.map(grant=>Object.freeze({...grant,reservedMegabytes:0,observedMegabytes:0})));
 const lent=decideHostCapacityV1(policy,free,unpriced,view.lentWaiters,nowMs).decisions.map(row=>{const worth=view.lent.get(row.id)!;return row.verdict==="admit"?Object.freeze({...row,reservedMegabytes:worth.fixedMegabytes+row.jobs*worth.perJobMegabytes,reason:"lent" as const}):row;});
 const taken=new Map<string,number>(),started=lent.filter(row=>row.verdict==="admit").map(row=>{
  const waiter=waiters.find(each=>each.request.id===row.id)!,holder=view.lent.get(row.id)!.holder;taken.set(holder,(taken.get(holder)??0)+row.reservedMegabytes);
  return Object.freeze({id:row.id,operation:waiter.request.operation,jobs:row.jobs,cpu:row.jobs*waiter.request.cpuPerJob,reservedMegabytes:row.reservedMegabytes,admittedAtMs:nowMs,observedMegabytes:null,pid:waiter.pid,bootAtMs:0,holder,...waiter.session===undefined?{}:{session:waiter.session}}) as HostGrantV1;
 });
 const evaluation=decideHostCapacityV1(policy,host,[...view.ledger.map(grant=>taken.has(grant.id)?Object.freeze({...grant,reservedMegabytes:Math.max(0,grant.reservedMegabytes-taken.get(grant.id)!)}):grant),...started],view.waiters,nowMs);
 return Object.freeze({...evaluation,decisions:Object.freeze([...lent,...evaluation.decisions])});
}

/** 🌊️ Whether the host's swap is a reason to wait: swap that is merely occupied never is (a long-running host parks cold pages there for days and a gate that refused on occupancy would never admit again), only swap that is worked while memory is short: the pages swapped in and out per second reach `activeBasisPointsPerSecond` of the host's memory while the occupied swap is above `ceilingBasisPoints` of it and the memory the host can still give out is below `shortBasisPoints` of it (`swap`), or the traffic alone exceeds `outBasisPointsPerSecond` of the memory per second (`swap-activity`: the disk is saturated whatever the free memory says). Every threshold is a share of the host's memory, so a 16 GB and a 64 GB host judge the same state alike.
 * @see decideHostCapacityV1 */
export function hostSwapStopV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes"|"availableMegabytes"|"swapUsedMegabytes"|"swapOutMegabytesPerSecond">):"swap"|"swap-activity"|undefined{
 const rate=host.swapOutMegabytesPerSecond;
 if(rate===null)return undefined;
 const rules=policy.swap,total=host.totalMegabytes,swapping=rate*10000>=total*rules.activeBasisPointsPerSecond,short=host.availableMegabytes*10000<total*rules.shortBasisPoints,crowded=host.swapUsedMegabytes!==null&&host.swapUsedMegabytes*10000>total*rules.ceilingBasisPoints;
 return crowded&&swapping&&short?"swap":rate*10000>total*rules.outBasisPointsPerSecond?"swap-activity":undefined;
}

/** 🧮️ The decision proper on the view `evaluateHostCapacityV1` hands it: a pure function of its arguments that knows nothing of holders.
 * @see evaluateHostCapacityV1 */
export function decideHostCapacityV1(policy:HostCapacityPolicyV1,host:HostSampleV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):HostEvaluationV1{
 const floor=hostFloorMegabytesV1(policy,host),cap=Math.max(0,host.totalMegabytes-floor),unmanaged=Math.max(host.unmanagedMegabytes??0,hostInteractiveReserveMegabytesV1(policy,host)),sessionCap=hostSessionCapMegabytesV1(policy,host),share=Math.max(0,cap-unmanaged);
 const ramping=(grant:HostGrantV1)=>grant.quiet!==true&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),active=ledger.filter(grant=>grant.lentSinceMs===undefined);
 const idleAtGate=(grant:HostGrantV1)=>policy.memory.rampIdleAfterMilliseconds!==undefined&&policy.memory.rampIdleObservedBasisPoints!==undefined&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)>=policy.memory.rampIdleAfterMilliseconds&&typeof grant.observedMegabytes==="number"&&grant.observedMegabytes*10000<grant.reservedMegabytes*policy.memory.rampIdleObservedBasisPoints;
 const unrealizedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(0,keptHostMegabytesV1(policy,grant,nowMs)-(grant.observedMegabytes??0)):0,committedOf=(grant:HostGrantV1)=>ramping(grant)?Math.max(keptHostMegabytesV1(policy,grant,nowMs),grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;
 const unrealized=active.reduce((sum,grant)=>sum+unrealizedOf(grant),0),committed=active.reduce((sum,grant)=>sum+committedOf(grant),0);
 const observed=host.availableMegabytes-floor-unrealized,budget=Math.floor(share*policy.memory.budgetBasisPoints/10000)-committed,binding=budget<observed?"budget" as const:"available" as const;
 let roomObserved=observed,roomBudget=budget,unmanagedPending=unmanaged;
 const headroom=Math.min(observed,budget),ledgerCpu=active.reduce((sum,grant)=>sum+grant.cpu,0),ceiling=policy.load.perCoreCeiling*host.cores;
 let loadRoom=host.load===null&&ledgerCpu===0?Infinity:ceiling-Math.max(host.load??0,ledgerCpu);
 const reportedLoadRoom=loadRoom===Infinity?null:loadRoom;
 const soft=softHostReasonV1(policy,host);
 const hard:HostReasonV1|undefined=host.pressure==="critical"?"pressure-critical":hostSwapStopV1(policy,host);
 const drainedRoom=Math.min(host.availableMegabytes+ledger.reduce((sum,grant)=>sum+(grant.observedMegabytes??0),0),host.totalMegabytes)-floor,drainedBudget=Math.floor(share*policy.memory.budgetBasisPoints/10000);
 const aged=waiters.map(waiter=>waiter.request.priority==="batch"&&nowMs-waiter.arrivedAtMs>=policy.wait.agingMilliseconds?Object.freeze({...waiter,request:Object.freeze({...waiter.request,priority:"interactive" as const})}):waiter);
 const sessionOf=new Map(waiters.map(waiter=>[waiter.request.id,waiter.session] as const));
 const growing=new Set(waiters.filter(waiter=>waiter.upgrade===true).map(waiter=>waiter.request.id)),decisions:HostDecisionV1[]=[],heads:{bypasses:number}[]=[],sessions=new Set<string>();let blocked=false,waits=false,localWaits=false,admitted=0,own=ledgerCpu,softLeft=policy.memory.quietSoftStopGrants-active.filter(grant=>grant.quiet===true).length,rampers=active.filter(grant=>grant.rampSlot===true&&ramping(grant)&&!idleAtGate(grant)).length,quietLeft=policy.memory.maximumQuiet-active.filter(grant=>grant.quiet===true).length;
 const charged=active.map(grant=>({session:grant.session,reservedMegabytes:committedOf(grant)})),ordered=orderHostWaitersV1(aged.filter(waiter=>waiter.rebuild===undefined||waiter.upgrade===true&&ledger.some(grant=>grant.id===waiter.request.id&&grant.rebuildScope===waiter.rebuild)),charged,waiter=>nowMs-waiter.arrivedAtMs>=policy.wait.agingMilliseconds);
 for(const [at,waiter] of ordered.entries()){
  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs,past=waited>=hostWaitDeadlineMillisecondsV1(policy,request.priority),quiet=request.quiet===true,grows=waiter.grow===true&&waiter.upgrade===true,elastic=request.elastic===true,avoids=quiet&&softLeft>0,ownChild=elastic&&request.holder!==undefined&&active.some(grant=>grant.id===request.holder),light=(request.operation==="graph"||request.operation==="graph-plan")&&request.minimumJobs===1&&(host.load??0)+request.cpuPerJob<=ceiling,pause=avoids?undefined:soft;
  const booked=waiter.upgrade===true?active.find(grant=>grant.id===request.id):undefined,replaced=booked??(waiter.rebuild!==undefined?ledger.find(grant=>grant.id===request.id):undefined),held=replaced?.observedMegabytes??0;
  const credit=booked===undefined&&replaced!==undefined?Math.min(held,unmanagedPending):0,remainingShare=Math.max(0,cap-unmanagedPending),creditedShare=Math.max(0,cap-unmanagedPending+credit),budgetCredit=Math.floor(creditedShare*policy.memory.budgetBasisPoints/10000)-Math.floor(remainingShare*policy.memory.budgetBasisPoints/10000);
  const freeObserved=roomObserved+(booked?unrealizedOf(booked):0),freeBudget=roomBudget+(booked?committedOf(booked):budgetCredit),freeLoad=loadRoom===Infinity||!booked?loadRoom:loadRoom+Math.max(host.load??0,ledgerCpu)-Math.max(host.load??0,ledgerCpu-booked.cpu),freeRampers=rampers-(booked?.rampSlot===true&&ramping(booked)?1:0);
  const costObserved=(jobs:number)=>Math.max(0,need(jobs)-held),costBudget=(jobs:number)=>Math.max(need(jobs),held),fits=(jobs:number)=>costObserved(jobs)<=freeObserved&&costBudget(jobs)<=freeBudget||booked!==undefined&&costObserved(jobs)<=unrealizedOf(booked)&&costBudget(jobs)<=committedOf(booked);
  const essential=request.essential===true&&elastic&&replaced===undefined&&!active.some(grant=>grant.label===request.label&&grant.holder===request.holder&&grant.session===waiter.session&&grant.id!==request.id);
  const refuse=(reason:HostReasonV1):void=>{if(essential&&hard===undefined){admit(request.minimumJobs,"override",need(request.minimumJobs));return;}if(request.priority!=="lent")localWaits=true;if(!grows&&!elastic)waits=true;decisions.push(Object.freeze({id:request.id,verdict:"wait",jobs:0,reservedMegabytes:0,reason}));};
  const admit=(jobs:number,reason:HostReasonV1,megabytes?:number):void=>{const reserved=megabytes??need(jobs);roomObserved=freeObserved-Math.max(0,reserved-held);roomBudget=freeBudget-Math.max(reserved,held);unmanagedPending-=credit;if(loadRoom!==Infinity)loadRoom=freeLoad-jobs*request.cpuPerJob;own+=jobs*request.cpuPerJob-(booked?.cpu??0);admitted++;if(waiter.session!==undefined)sessions.add(waiter.session);rampers=freeRampers+(request.rampSlot===true?1:0);if(quiet){quietLeft--;softLeft--;}else for(const head of heads){head.bypasses++;if(head.bypasses>=policy.wait.maximumBypasses)blocked=true;}decisions.push(Object.freeze({id:request.id,verdict:"admit",jobs,reservedMegabytes:reserved,reason}));};
  if(waiter.session!==undefined&&sessionCap!==undefined&&waited<policy.wait.agingMilliseconds&&waiter.rebuild===undefined&&!request.elastic&&waiter.upgrade!==true&&waiter.grow!==true){
   const holds=(session:string,id:string)=>active.reduce((sum,grant)=>sum+(grant.session===session&&grant.id!==id?committedOf(grant):0),0)+decisions.reduce((sum,row)=>sum+(row.verdict==="admit"&&row.id!==id&&sessionOf.get(row.id)===session?row.reservedMegabytes:0),0),least=(each:HostRequestV1)=>Math.min(each.minimumMegabytes??each.fixedMegabytes+each.minimumJobs*each.perJobMegabytes,cap),holding=holds(waiter.session,request.id);
   const competed=ordered.some(other=>other.session!==undefined&&other.session!==waiter.session&&other.request.priority!=="lent"&&other.upgrade!==true&&other.grow!==true&&other.request.elastic!==true&&!decisions.some(row=>row.id===other.request.id&&row.verdict==="admit")&&(holds(other.session,other.request.id)===0||holds(other.session,other.request.id)+least(other.request)<=sessionCap));
   if(competed&&(active.some(grant=>grant.session===waiter.session&&grant.id!==request.id)||holding>0)&&holding+least(request)>sessionCap){refuse("session-cap");continue;}
  }
  const leastMegabytes=request.minimumMegabytes??request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,limitMegabytes=Math.floor(cap*policy.memory.budgetBasisPoints/10000);
  if(waiter.rebuild!==undefined&&leastMegabytes>limitMegabytes){refuse("never");continue;}
  if(hard){
   refuse(waiter.rebuild===undefined&&replaced===undefined&&!elastic&&!quiet&&waited>=hostStarvedMillisecondsV1(policy)?"starved":hard);
   continue;
  }
  if(request.priority==="lent"&&localWaits){refuse("queue-order");continue;}
  if(grows){
   if(blocked||waits){refuse("queue-order");continue;}
   if(replaced===undefined||replaced.grownAtMs!==undefined&&ramping(replaced)){refuse("ramp");continue;}
   if(soft!==undefined){refuse(soft);continue;}
   const roomy=(jobs:number)=>{const wanted=Math.min(request.fixedMegabytes+(jobs+protocol.growSpareJobs)*request.perJobMegabytes,cap);return Math.max(0,wanted-held)<=freeObserved&&Math.max(wanted,held)<=freeBudget;};
   let jobs=0;for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(roomy(candidate)&&candidate*request.cpuPerJob<=freeLoad){jobs=candidate;break;}
   if(jobs>0)admit(jobs,jobs<request.preferredJobs?"reduced":"fits",Math.max(replaced.reservedMegabytes,need(jobs)));else refuse(roomy(request.minimumJobs)?"load":"memory");
   continue;
  }
  if(elastic&&(blocked||waits)||blocked&&!(quiet&&quietLeft>0)){refuse("queue-order");continue;}
  if(quiet&&quietLeft<=0){refuse("quiet-limit");continue;}
  if(request.rampSlot===true&&freeRampers>=policy.memory.maximumRamping){refuse("ramp");continue;}
  if(request.minimumMegabytes!==undefined&&leastMegabytes>limitMegabytes&&replaced===undefined&&!elastic&&!quiet){refuse("never");continue;}
  if(waiter.rebuild!==undefined&&request.minimumMegabytes!==undefined&&replaced!==undefined&&!fits(request.minimumJobs)&&(pause===undefined||past)){
   const booking=Math.max(replaced.reservedMegabytes,Math.min(need(request.minimumJobs),held+freeObserved,freeBudget));
   if(booking>=request.minimumMegabytes){admit(request.minimumJobs,"shrunk",booking);continue;}
  }
  const memoryFits=fits(request.minimumJobs),shrinkable=request.minimumMegabytes!==undefined&&replaced===undefined&&!elastic&&!quiet&&request.minimumMegabytes<need(request.minimumJobs),room=Math.min(freeObserved,freeBudget),shrunk=shrinkable&&!memoryFits&&room>=request.minimumMegabytes!?Math.min(need(request.minimumJobs),room):0,first=!elastic&&waiter.session!==undefined&&own<ceiling+host.cores&&!sessions.has(waiter.session)&&active.every(grant=>grant.id===request.id||grant.session!==waiter.session);
  let jobs=0;
  if(pause===undefined)for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(fits(candidate)&&(avoids||ownChild||candidate*request.cpuPerJob<=freeLoad)){jobs=candidate;break;}
  if(jobs===0&&(pause===undefined||shrunk>0&&(past||active.length-(booked?1:0)===0&&admitted===0))&&(memoryFits||shrunk>0)&&(avoids||light||first&&replaced===undefined||own-(booked?.cpu??0)+request.minimumJobs*request.cpuPerJob<=ceiling))jobs=request.minimumJobs;
  if(jobs>0){if(shrunk>0&&jobs===request.minimumJobs)admit(jobs,"shrunk",shrunk);else admit(jobs,jobs<request.preferredJobs?"reduced":"fits");continue;}
  if(shrunk>0&&active.length===0&&admitted===0&&waited>=policy.load.overrideAfterMilliseconds){admit(request.minimumJobs,"shrunk",shrunk);continue;}
  if(shrinkable&&!memoryFits&&shrunk===0&&active.length===0&&waited>=policy.load.overrideAfterMilliseconds||!memoryFits&&shrunk===0&&waiter.rebuild===undefined&&replaced===undefined&&!elastic&&!quiet&&waited>=hostStarvedMillisecondsV1(policy)){refuse("starved");continue;}
  if(waiter.rebuild===undefined&&pause===undefined&&memoryFits&&replaced!==undefined){const cores=Math.min(request.preferredJobs,Math.floor((ceiling-own+(booked?.cpu??0)-ordered.slice(at+1).filter(later=>later.upgrade!==true&&later.grow!==true&&later.request.elastic!==true).reduce((sum,later)=>sum+later.request.minimumJobs*later.request.cpuPerJob,0))/request.cpuPerJob));if(cores>=1||first){admit(Math.max(1,cores),"reduced",need(request.minimumJobs));continue;}}
  const reason:HostReasonV1=!memoryFits?"memory":pause??"load";
  if(waiter.rebuild===undefined&&reason==="memory"&&replaced!==undefined&&active.every(grant=>grant.id===request.id||grant.quiet===true&&growing.has(grant.id))&&admitted===0&&!quiet){const fitted=Math.min(need(request.minimumJobs),held+freeObserved,freeBudget),units=request.perJobMegabytes>0?Math.max(1,Math.min(request.preferredJobs,Math.floor((fitted-request.fixedMegabytes)/request.perJobMegabytes))):request.preferredJobs;admit(units,"override",Math.max(replaced.reservedMegabytes,Math.min(fitted,need(units))));}
  else if(reason!=="memory"&&!elastic&&active.length-(booked?1:0)===0&&admitted===0&&waited>=policy.load.overrideAfterMilliseconds)admit(request.minimumJobs,"override");
  else{
   refuse(reason);
   if((reason==="memory"||reason==="load")&&!quiet&&!elastic){
    const drains=soft===undefined&&need(request.minimumJobs)<=Math.min(drainedRoom,drainedBudget)&&request.minimumJobs*request.cpuPerJob<=ceiling,bypasses=ledger.filter(grant=>grant.id!==request.id&&grant.quiet!==true&&grant.admittedAtMs>=waiter.arrivedAtMs).length;
    if(drains&&(waited>=policy.wait.bypassMilliseconds||bypasses>=policy.wait.maximumBypasses))blocked=true;else if(drains)heads.push({bypasses});
   }
  }
 }
 return Object.freeze({floorMegabytes:floor,unrealizedMegabytes:unrealized,headroomMegabytes:headroom,...host.unmanagedMegabytes===undefined?{}:{unmanagedMegabytes:unmanaged},binding,loadRoom:reportedLoadRoom,decisions:Object.freeze(decisions)});
}

/** 🚫️ Whether a request can never run on this machine, whatever else runs: the least it needs (its `minimumMegabytes`, else its reservation at the minimum jobs) exceeds the memory the host may give out at all (the memory above its floor, within the authored budget share). Quiet, elastic and held (replacing) requests are judged by the grant they grow from, never here. A pure function the gate applies the moment a request arrives, so no request waits for what no release can ever give.
 * @see decideHostCapacityV1 */
export function hostRequestNeverFitsV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">,request:HostRequestV1):boolean{
 const limit=Math.floor(Math.max(0,host.totalMegabytes-hostFloorMegabytesV1(policy,host))*policy.memory.budgetBasisPoints/10000);
 return request.quiet!==true&&request.elastic!==true&&(request.minimumMegabytes??request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes)>limit;
}

/** 🛟️ Whether a request can run below its estimate after waiting for memory that the live grants cannot free. The headroom must still cover both its declared minimum, if any, and the policy minimum. Quiet, elastic and replacing requests and any soft stop keep their own rules.
 * @see evaluateHostCapacityV1 */
export function unsatisfiableHostRequestV1(policy:HostCapacityPolicyV1,facts:Readonly<{waitedMilliseconds:number;roomMegabytes:number;drainedMegabytes:number;needMegabytes:number;minimumMegabytes?:number;admitted:number;replaced:boolean;quiet:boolean;elastic:boolean;pause:unknown}>):boolean{
 const minimum=Math.max(facts.minimumMegabytes??0,policy.estimates.minimumMegabytes);
 return facts.pause===undefined&&!facts.replaced&&!facts.quiet&&!facts.elastic&&facts.admitted===0&&facts.drainedMegabytes<facts.needMegabytes&&facts.waitedMilliseconds>=policy.wait.bypassMilliseconds&&facts.roomMegabytes>=minimum;
}

/** 🪴️ How many consecutive grow attempts of one grant were admitted and the fewest jobs any of them admitted. */
export type HostGrowthSettleV1=Readonly<{attempts:number;jobs:number}>;

/** ⏲️ The time hysteresis of a growth: a grant grows only after `growSettleAttempts` consecutive attempts (one per heartbeat) were admitted, and then to the fewest jobs any of them admitted; one refused attempt starts the count again, so memory that frees up for a moment never grows a run.
 * @see ../🧪️tests/🔼️upgrade/🌱️grow/🐍️oracle/🐍️.py */
export function settleHostGrowthV1(previous:HostGrowthSettleV1|undefined,decision:Pick<HostDecisionV1,"verdict"|"jobs">):Readonly<{next:HostGrowthSettleV1|undefined;jobs:number|undefined}>{
 if(decision.verdict!=="admit")return Object.freeze({next:undefined,jobs:undefined});
 const next=Object.freeze({attempts:(previous?.attempts??0)+1,jobs:Math.min(previous?.jobs??decision.jobs,decision.jobs)});
 return next.attempts>=protocol.growSettleAttempts?Object.freeze({next:undefined,jobs:next.jobs}):Object.freeze({next,jobs:undefined});
}

/** 🎯️ The plan a process carries, followed to the grant row as it is now: fewer jobs after a partial growth, more after a grant admitted with fewer jobs grew, the cores of the grant and the memory the jobs may hold in proportion; the same plan when the row is quiet, belongs to another grant or holds the same jobs. */
export function followHostGrantPlanV1(plan:HostPlanV1,grant:Pick<HostGrantV1,"id"|"jobs"|"cpu"|"quiet">):HostPlanV1{
 if(grant.id!==plan.id||grant.quiet===true||grant.jobs===plan.jobs)return plan;
 return requireHostCapacityV1<HostPlanV1>("Plan",Object.freeze({...plan,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:Math.floor(plan.memoryMegabytes*grant.jobs/plan.jobs)}));
}

/** 📏️ Sizes the request of one named operation, or answers undefined when the operation is light and passes ungated. */
export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[];targets?:readonly string[];host?:Pick<HostSampleV1,"totalMegabytes">}>):HostRequestV1|undefined{
 const operation=policy.operations.find(row=>row.name===input.operation);
 if(!operation)return undefined;
 const reach=input.host===undefined?Infinity:Math.max(operation.minimumMegabytes??0,Math.floor(Math.max(0,input.host.totalMegabytes-hostFloorMegabytesV1(policy,input.host))*policy.estimates.maximumShareBasisPoints/10000)),fixed=Math.min(operation.fixedMegabytes,reach);
 if(operation.shape==="fixed")return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:fixed,perJobMegabytes:0,cpuPerJob:operation.cpu,...operation.minimumMegabytes===undefined?{}:{minimumMegabytes:operation.minimumMegabytes}}));
 const kinds=[...new Set<HostActionKindV1>([...operation.kind?[operation.kind]:input.kinds??[],"build"])],rows:HostResourceV1[]=[...kinds.map(kind=>policy.kinds[kind]),...policy.envelopes.filter(row=>(input.targets??[]).some(target=>new RegExp(row.pattern,"u").test(target)))];
 const declared=Math.max(...rows.map(row=>row.megabytes)),perJob=Math.min(declared,Math.max(policy.estimates.minimumMegabytes,reach-fixed));
 return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:input.id,operation:operation.name,priority:operation.priority,minimumJobs:1,preferredJobs:Math.max(1,Math.min(input.preferredJobs??1,protocol.maximumJobs)),fixedMegabytes:fixed,perJobMegabytes:perJob,...perJob<declared?{envelopePerJobMegabytes:declared}:{},cpuPerJob:Math.max(...rows.map(row=>row.cpu))}));
}

/** 🎁️ Renders an admitted request into the tool-neutral envelope: jobs, cores, and the memory the jobs may hold beside the fixed block; the envelope per job is the default size (`envelopePerJobMegabytes`) when a learned estimate shrank the reservation, so a build tool is never starved by an estimate. */
export function planHostCapacityV1(policy:HostCapacityPolicyV1,host:Pick<HostSampleV1,"totalMegabytes">,request:HostRequestV1,jobs:number):HostPlanV1{
 if(!Number.isSafeInteger(jobs)||jobs<request.minimumJobs||jobs>request.preferredJobs)throw hostCapacityErrorV1("planJobs");
 return requireHostCapacityV1<HostPlanV1>("Plan",Object.freeze({version:1,id:request.id,jobs,cpu:jobs*request.cpuPerJob,memoryMegabytes:jobs*(request.envelopePerJobMegabytes??request.perJobMegabytes),fixedMegabytes:request.fixedMegabytes,floorMegabytes:hostFloorMegabytesV1(policy,host)}));
}

/** 🗣️ Picks the message language by the shared language rule; no language is assumed. */
export function resolveHostCapacityLanguageV1(environment:Readonly<Record<string,string|undefined>>,osLanguages?:OsLanguageListV1):"en"|"de"{
 return resolveLanguageV1(["en","de"],[],environment,osLanguages).language as "en"|"de";
}

/** 🖨️ Fills one catalogue message of the chosen language; unknown placeholders stay visible. */
export function formatHostCapacityV1(language:"en"|"de",name:"waiting"|"admitted"|"estimated"|"phased"|"quiet"|"quietFollowUp"|"upgrading"|"upgraded"|"upgradedPartly"|"upgradedReduced"|"upgradeWithdrawn"|"grantQuiet"|"waiterUpgrade"|"reduced"|"shrunk"|"shrunkStopped"|"pastDeadline"|"blockerSelf"|"lent"|"reducedGrows"|"grown"|"override"|"unsatisfiable"|"bypassed"|"status"|"grant"|"grantLent"|"waiter"|"handover"|"adopted"|"handoverRefused"|"treeUnobserved"|"footprintFallback"|"countersPartial"|"availableUnread"|"blocker"|"blockerHost"|"waitingShort",values:Readonly<Record<string,string|number>>):string{
 return fillHostCapacityV1(schema.definitions.Messages.const[language][name],values);
}

/** 🔤️ Fills the placeholders of one catalogue text; unknown placeholders stay visible. */
function fillHostCapacityV1(text:string,values:Readonly<Record<string,string|number>>):string{
 return text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
}

/** 🚨️ Names of the catalogue errors a refused host capacity value, plan or platform raises. */
export type HostCapacityErrorV1=keyof typeof schema.definitions.Messages.const.en.errors;

/** 🌐️ Fills one catalogue error of the chosen language; `{detail}` carries the first finding of the schema validator. */
export function formatHostCapacityErrorV1(language:"en"|"de",name:HostCapacityErrorV1,values:Readonly<Record<string,string|number>>={}):string{
 return fillHostCapacityV1(schema.definitions.Messages.const[language].errors[name],values);
}

/** ❌️ Builds the error of one refusal in the language of the environment, so every diagnostic of the gate speaks English or German like its progress lines. */
export function hostCapacityErrorV1(name:HostCapacityErrorV1,values:Readonly<Record<string,string|number>>={},environment:Readonly<Record<string,string|undefined>>=process.env):Error{
 return Error(formatHostCapacityErrorV1(resolveHostCapacityLanguageV1(environment),name,values));
}

/** 💤️ One thing of ours a waiting request stands behind: the label of its request, whether it runs, is quiet, waits to grow or waits ahead in the queue, its jobs and the megabytes it holds (committed) or asks for. */
export type HostBlockerV1=Readonly<{label:string;state:"running"|"growing"|"quiet"|"waiting";jobs:number;megabytes:number}>;

/** 📛️ Names what a request that waits in `evaluation` stands behind, decided by its reason: for `memory` the live grants of ours by the megabytes they count (committed: the reservation while it ramps, then what the tree holds), for `load` the grants by the cores they book, for `ramp` the grants that ramp in a slot (each with the requests admitted before it in the same evaluation, which run from now on), for `quiet-limit` the quiet grants, for `queue-order` the requests served before it that wait; a lent grant and the request's own grant never count; at most `limit`, largest first, then by label; empty for a reason that is the host's own (pressure, swap, files, images). A pure function.
 * @see ../🧪️tests/🧾️blockers/🐍️oracle/🐍️.py */
export function hostBlockersV1(policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],evaluation:HostEvaluationV1,id:string,nowMs:number,limit:number=3):readonly HostBlockerV1[]{
 const position=evaluation.decisions.findIndex(row=>row.id===id),mine=evaluation.decisions[position];
 if(mine===undefined||mine.verdict!=="wait")return Object.freeze([]);
 const idleAtGate=(grant:HostGrantV1)=>policy.memory.rampIdleAfterMilliseconds!==undefined&&policy.memory.rampIdleObservedBasisPoints!==undefined&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)>=policy.memory.rampIdleAfterMilliseconds&&typeof grant.observedMegabytes==="number"&&grant.observedMegabytes*10000<grant.reservedMegabytes*policy.memory.rampIdleObservedBasisPoints;
 const ramping=(grant:HostGrantV1)=>grant.quiet!==true&&nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),committed=(grant:HostGrantV1)=>ramping(grant)?Math.max(keptHostMegabytesV1(policy,grant,nowMs),grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;
 const growing=new Set(waiters.filter(waiter=>waiter.upgrade===true).map(waiter=>waiter.request.id)),grants=ledger.filter(grant=>grant.lentSinceMs===undefined&&grant.id!==id);
 const ofGrant=(grant:HostGrantV1,megabytes:number):HostBlockerV1=>Object.freeze({label:grant.label??`${grant.operation} ${grant.id.slice(0,6)}`,state:grant.quiet===true?growing.has(grant.id)?"growing" as const:"quiet" as const:"running" as const,jobs:grant.jobs,megabytes});
 const ahead=evaluation.decisions.slice(0,position).filter(row=>row.verdict==="wait").flatMap(row=>waiters.filter(waiter=>waiter.request.id===row.id).slice(0,1)).map(waiter=>Object.freeze({label:waiter.request.label??`${waiter.request.operation} ${waiter.request.id.slice(0,6)}`,state:"waiting" as const,jobs:waiter.request.minimumJobs,megabytes:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes}));
 const admitted=evaluation.decisions.slice(0,position).filter(row=>row.verdict==="admit"&&!ledger.some(grant=>grant.id===row.id)).flatMap(row=>waiters.filter(waiter=>waiter.request.id===row.id).slice(0,1).map(waiter=>Object.freeze({blocker:Object.freeze({label:waiter.request.label??`${waiter.request.operation} ${waiter.request.id.slice(0,6)}`,state:"running" as const,jobs:row.jobs,megabytes:row.reservedMegabytes}),slot:waiter.request.rampSlot===true})));
 const rows=mine.reason==="memory"?[...grants.map(grant=>ofGrant(grant,committed(grant))),...admitted.map(row=>row.blocker)]:mine.reason==="load"?[...grants.filter(grant=>grant.cpu>0).map(grant=>ofGrant(grant,committed(grant))),...admitted.map(row=>row.blocker)]:mine.reason==="ramp"?[...grants.filter(grant=>grant.rampSlot===true&&ramping(grant)&&!idleAtGate(grant)).map(grant=>ofGrant(grant,committed(grant))),...admitted.filter(row=>row.slot).map(row=>row.blocker)]:mine.reason==="quiet-limit"?grants.filter(grant=>grant.quiet===true).map(grant=>ofGrant(grant,committed(grant))):mine.reason==="queue-order"?ahead:[];
 const key=(row:HostBlockerV1)=>mine.reason==="load"?row.jobs:row.megabytes;
 return Object.freeze([...rows].sort((left,right)=>key(right)-key(left)||(left.label<right.label?-1:left.label>right.label?1:0)).slice(0,limit));
}

/** 🗂️ Renders what a waiter stands behind in the chosen language: each blocker as `label (state, jobs, megabytes)`, or the host itself when nothing of ours holds it back. */
export function describeHostBlockersV1(language:"en"|"de",blockers:readonly HostBlockerV1[]):string{
 const messages=schema.definitions.Messages.const[language];
 return blockers.length===0?messages.blockerHost:blockers.map(row=>fillHostCapacityV1(messages.blocker,{label:row.label,state:messages.blockerStates[row.state],jobs:row.jobs,megabytes:row.megabytes})).join(", ");
}

/** ⌛️ Names a wait or override reason in the chosen language. */
export function describeHostReasonV1(language:"en"|"de",reason:Exclude<HostReasonV1,"fits"|"reduced"|"override"|"shrunk"|"lent">):string{
 return schema.definitions.Messages.const[language].reasons[reason];
}

/** 🧺️ The memory held by everything the gate did not grant: what the machine uses (total minus available) beyond what the live grants hold, a grant counted by what its process tree observably holds and, while unobserved, by its reservation, so nothing is counted twice; a lent grant holds nothing. */
export function unmanagedHostMegabytesV1(host:Pick<HostSampleV1,"totalMegabytes"|"availableMegabytes">,ledger:readonly Pick<HostGrantV1,"observedMegabytes"|"reservedMegabytes"|"lentSinceMs">[]):number{
 const managed=ledger.reduce((sum,grant)=>sum+(grant.lentSinceMs!==undefined?0:grant.observedMegabytes??grant.reservedMegabytes),0);
 return Math.max(0,host.totalMegabytes-host.availableMegabytes-managed);
}

/** 🧮️ The sample with the unmanaged memory measured against the live grants, so admission divides only what is left of the machine after the agents, editors and watchers that run beside the gate. */
export function withUnmanagedHostV1(host:HostSampleV1,ledger:readonly HostGrantV1[]):HostSampleV1{
 return Object.freeze({...host,unmanagedMegabytes:unmanagedHostMegabytesV1(host,ledger)});
}

/** 🕰️ The sample admission may trust at `nowMs`: a sample older than `staleSampleMilliseconds` (the live read failed or was starved) is no longer evidence of the host, so it reads as raised pressure with partial counters, which admits one run at a time (the soft pause with its override) and grows nothing, instead of the unbounded admission an empty or ancient sample would give; a fresher sample is returned unchanged.
 * @see ../🧪️tests/🕰️stale-sample/🟦️.ts */
export function agedHostSampleV1(host:HostSampleV1,nowMs:number):HostSampleV1{
 if(nowMs-host.atMs<=protocol.staleSampleMilliseconds)return host;
 return Object.freeze({...host,pressure:host.pressure==="critical"?"critical":"warn",counters:"partial"});
}
