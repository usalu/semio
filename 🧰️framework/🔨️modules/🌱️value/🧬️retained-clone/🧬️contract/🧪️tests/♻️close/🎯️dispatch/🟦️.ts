/** 🎯️ Selects one complete close event for the requested payload body. */
export interface SelectedCloseDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export function selectedCloseDemand(body:number,minimumPayloadBytes:number,structural:SelectedCloseDemand,payload:SelectedCloseDemand):SelectedCloseDemand{return {...(body>=minimumPayloadBytes?payload:structural)}}
