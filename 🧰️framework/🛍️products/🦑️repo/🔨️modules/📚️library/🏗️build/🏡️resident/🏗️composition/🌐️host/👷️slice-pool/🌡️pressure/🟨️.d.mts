export type PressureMeasuresV1={full?:number;high?:number;freeBytes?:number};
export type PressureVerdictV1={tripped:boolean;kind?:"psi"|"events"|"free";value?:number;limit?:number;seconds?:number};
export function cgroupMeasuresV1(options?:{root?:string;read?:(path:string)=>string;own?:()=>string;free?:()=>number}):()=>PressureMeasuresV1;
export function createPressureWatchV1(options?:{measures?:()=>PressureMeasuresV1;policy?:{fullAvg10Percent:number;throttleEventsPerSecond:number;freeFloorMegabytes:number;sustainedSeconds:number};now?:()=>number}):{sample():PressureVerdictV1};
