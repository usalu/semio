export function hostPressureLevelV1(options?:{platform?:string;run?:()=>string;read?:(path:string)=>string;policy?:{fullAvg10Percent:number}}):"normal"|"warn"|"critical"|undefined;

/** 📏️ What one reading of the memory pressure sources found: the cgroup `full avg10`, the throttle event count or the free bytes. */
export type PressureMeasuresV1=Readonly<{full?:number;high?:number;freeBytes?:number}>;
/** 📖 The measures of one sample: `full` (PSI full avg10, percent), `high` (cumulative `high` events) and `freeBytes`; each only where the host offers it. */
export function cgroupMeasuresV1(options?: Readonly<{root?:string;read?:(path:string)=>string;own?:()=>string;free?:()=>number|undefined}>): () => PressureMeasuresV1;
/** 🌡 One sample of a pressure watch. */
export type PressureSampleV1=Readonly<{tripped:false;kind?:"psi"|"events"|"free";value?:number;limit?:number;seconds?:number}|{tripped:boolean;kind:"psi"|"events"|"free";value:number;limit:number;seconds:number}>;
/** 🌡 A watch for one load: `sample()` reads the measures and answers `{tripped,kind,value,limit,seconds}` (`kind` is `psi`, `events` or `free`); tripped once the measure has stayed over its limit for `sustainedSeconds`. */
export function createPressureWatchV1(options?: Readonly<{measures?:()=>PressureMeasuresV1;policy?:Readonly<{fullAvg10Percent:number;throttleEventsPerSecond:number;freeFloorMegabytes:number;sustainedSeconds:number}>;now?:()=>number}>): Readonly<{sample():PressureSampleV1}>;
