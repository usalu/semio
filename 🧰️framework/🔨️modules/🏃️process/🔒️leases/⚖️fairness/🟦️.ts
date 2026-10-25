/** 🎟️ One request that arrived earlier in a resource queue, as its ticket file states it. */
export type QueueTicketV1=Readonly<{pid:number;mode:"shared"|"exclusive";sharedCohort:boolean;yielding:boolean;arrivedAtMs:number}>;
/** 🙋️ The request whose turn is decided; `holdsShared` says its process already holds the resource shared. */
export type QueueRequesterV1=Readonly<{pid:number;mode:"shared"|"exclusive";sharedCohort:boolean;yielding:boolean;holdsShared:boolean}>;

/** 🙇️ Whether an earlier exclusive ticket stands aside for the requester: a yielding writer never delays a request that is not itself yielding, and a writer keeps its preference over later shared requests only for `writerPreferenceMilliseconds`, so a writer that waits for long holders can neither close the queue for hours nor starve the holders' own descendants queued behind it. Other writers keep arrival order. */
export function queueTicketDefersV1(requester:QueueRequesterV1,ahead:QueueTicketV1,nowMs:number,writerPreferenceMilliseconds:number):boolean{
 if(ahead.mode!=="exclusive")return false;
 if(requester.mode==="shared")return ahead.yielding||nowMs-ahead.arrivedAtMs>=writerPreferenceMilliseconds;
 return ahead.yielding&&!requester.yielding;
}

/** 🚦️ Whether the requester may try its kernel lease now: every earlier ticket that does not stand aside is a shared cohort ticket of a shared cohort request, a shared ticket when the requester's process or its owning ancestor already holds the resource shared (shared tickets never conflict, and the owner may wait for this very request, as an Nx daemon waits behind its loader's held ticket), or there is none. */
export function queueMayProceedV1(requester:QueueRequesterV1,ahead:readonly QueueTicketV1[],nowMs:number,writerPreferenceMilliseconds:number):boolean{
 const blocking=ahead.filter(row=>!queueTicketDefersV1(requester,row,nowMs,writerPreferenceMilliseconds));
 return blocking.length===0
  ||requester.sharedCohort&&blocking.every(row=>row.mode==="shared"&&row.sharedCohort)
  ||requester.mode==="shared"&&requester.holdsShared&&blocking.every(row=>row.mode==="shared");
}
