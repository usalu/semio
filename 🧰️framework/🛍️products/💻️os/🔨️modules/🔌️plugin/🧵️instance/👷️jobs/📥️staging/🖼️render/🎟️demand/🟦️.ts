/** 🎟️ Original render authority prices each independent physical currency without increasing its caller grant. */
export interface RenderDemand {copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface RenderGrant {maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export type RenderReadiness="enter"|"pending"|"refused"|"cancelled";
const fits=(d:RenderDemand,g:RenderGrant):boolean=>g.maximumItems>0&&d.copyBytes<=g.maximumCopyBytes&&d.capacityBytes<=g.maximumCapacityBytes&&d.releaseBytes<=g.maximumReleaseBytes&&d.depth<=g.maximumDepth;
export function renderReadiness(demand:RenderDemand,remaining:RenderGrant,total:RenderGrant,cancelled=false):RenderReadiness {if(cancelled)return "cancelled";if(!fits(demand,total))return "refused";return fits(demand,remaining)?"enter":"pending";}
