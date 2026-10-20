/** 📤️ Selects the complete current alias or original-authority handback before any transfer. */
export interface SourceTakeDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export function sourceTakeDemand(state:Readonly<{closing:boolean;owner:SourceTakeDemand|null;lease:SourceTakeDemand|null}>):SourceTakeDemand|null{return state.closing||state.owner===null?null:state.lease??state.owner;}
