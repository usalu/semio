/** 🔒️ Access mode of one lease, identical to the lease layer's own. */
export type LeaseModeV1="shared"|"exclusive";
/** 🧭️ How one rule reacts: refuse the request, only record it, or stay silent. */
export type LeaseOrderRuleV1="enforce"|"observe"|"off";
/** 🏷️ One lock class: a rank in the declared order and the resource names that belong to it. */
export type LeaseClassV1=Readonly<{id:string;rank:number;scope:"machine"|"checkout"|"project"|"crate"|"generation"|"path"|"session";owner:string;inert?:boolean;title:Readonly<{en:string;de:string}>;match:readonly Readonly<{kind:"prefix"|"exact";value:string}>[]}>;
/** 📜️ The authored lock order policy (`Policy` of the schema). */
export type LeaseOrderPolicyV1=Readonly<{version:1;rules:Readonly<{outOfOrder:LeaseOrderRuleV1;holdAndWait:LeaseOrderRuleV1;selfDeadlock:LeaseOrderRuleV1;cycle:LeaseOrderRuleV1;strictWait?:LeaseOrderRuleV1}>;strictWaits?:readonly Readonly<{held:readonly string[];requested:readonly string[];reason:Readonly<{en:string;de:string}>}>[];holdAndWait:Readonly<{graceMilliseconds:number;pollMilliseconds:number}>;cycle:Readonly<{intervalMilliseconds:number}>;registry:Readonly<{flushMilliseconds:number;maximumEntryBytes:number;maximumViolations:number}>;fairness:Readonly<{queuedWithResourceLeases:boolean;reentrantShared?:boolean;writerPreferenceMilliseconds?:number}>;media:readonly Readonly<{prefix:string}>[];gate:Readonly<{releaseWhileWaiting:boolean;pollInitialMilliseconds:number;pollMaximumMilliseconds:number;admissionPatienceMilliseconds:number;claimSweepIntervalMilliseconds:number;claimSweepPressureFreeRatio:number;claimSweepPressureIntervalMilliseconds:number}>;classes:readonly LeaseClassV1[];allowed:readonly Readonly<{from:string;to:string;binding?:Readonly<{kind:"same-directory-suffix";fromPrefix:string;toPrefix:string}>;reason:Readonly<{en:string;de:string}>}>[]}>;
/** 🧾️ One violation (`Violation` of the schema), in data form so tools can render it in any language. */
export type LeaseViolationV1=Readonly<{version:1;kind:"out-of-order"|"hold-and-wait"|"self-deadlock"|"cycle";held:string;heldMode:LeaseModeV1;heldRank:number;heldClass:string;requested:string;requestedMode:LeaseModeV1;requestedRank:number;requestedClass:string;waitedMilliseconds?:number;blockerPid?:number;blockerHeldSeconds?:number;cycle?:readonly string[];atMs:number;pid:number;enforced?:boolean}>;
/** 🎒️ One lease a task currently holds. */
export type HeldLeaseV1=Readonly<{directory:string;resource:string;mode:LeaseModeV1;atMs:number;task:number}>;
/** 🕸️ One participant of the wait-for graph: the leases it holds and the one it waits for. */
export type LeaseNodeV1=Readonly<{id:string;holds:readonly Readonly<{key:string;mode:LeaseModeV1}>[];wait?:Readonly<{key:string;mode:LeaseModeV1;sinceMs:number;queued:boolean}>;ancestors?:readonly string[]}>;
/** 🔁️ A deadlock cycle and the participant chosen to fail. */
export type LeaseCycleV1=Readonly<{members:readonly string[];victim:string}>;
