/** 🔄️ A window authority keeps its displaced original until its issued queue admits custody. */
export interface WindowRefreshGrant{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export interface WindowRefreshDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface WindowRefreshProgress{copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number}
export interface WindowRefreshSnapshot{windowId:string}
export interface WindowRefreshQueue<T>{length:number;hasReservedSlot():boolean;reserveCapacity():number;frameCapacity(value:T):number;reserveStep(grant:WindowRefreshGrant):WindowRefreshProgress;admitOwned(value:T,grant:WindowRefreshGrant):WindowRefreshProgress|{error:Error;original:T}}
export type WindowRefreshStep={kind:"pending"|"ready";progress:WindowRefreshProgress};
const empty=():WindowRefreshProgress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
export function windowRefreshDemand<T>(pending:T|undefined,queue:WindowRefreshQueue<T>):WindowRefreshDemand{return{copyBytes:0,capacityBytes:pending===undefined?0:queue.hasReservedSlot()?queue.frameCapacity(pending):queue.reserveCapacity(),releaseBytes:0,depth:pending===undefined?1:queue.length+1};}
export function transferWindowRefresh<T extends WindowRefreshSnapshot>(current:{value:T},pending:{value:T|undefined},queue:WindowRefreshQueue<T>,grant:WindowRefreshGrant):WindowRefreshStep{
 const demand=windowRefreshDemand(pending.value,queue);
 if(grant.maximumItems<1||grant.maximumCopyBytes<demand.copyBytes||grant.maximumCapacityBytes<demand.capacityBytes||grant.maximumReleaseBytes<demand.releaseBytes||grant.maximumDepth<demand.depth)return{kind:"pending",progress:empty()};
 if(!queue.hasReservedSlot())return{kind:"pending",progress:queue.reserveStep(grant)};
 if(!pending.value)return{kind:"pending",progress:empty()};
 const next=pending.value,previous=current.value;pending.value=undefined;
 const address=previous.windowId;previous.windowId=next.windowId;next.windowId=address;current.value=next;
 const result=queue.admitOwned(previous,grant);
 if("error"in result){current.value=result.original;const address=current.value.windowId;current.value.windowId=next.windowId;next.windowId=address;pending.value=next;throw result.error;}
 return{kind:"ready",progress:result};
}
