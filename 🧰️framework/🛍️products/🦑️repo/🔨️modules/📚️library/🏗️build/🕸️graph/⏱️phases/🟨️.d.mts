/** ⏱️ One finished graph phase. */
export interface GraphPhaseRowV1{readonly phase:string;readonly milliseconds:number;readonly done?:number;readonly total?:number;readonly [detail:string]:unknown}
/** 🏃️ One unit of work currently running in a phase and the seconds it has run, oldest first. */
export interface GraphPhaseRunningV1{readonly name:string;readonly seconds:number}
export interface GraphPhaseClockOptionsV1{publish?:(resource:string)=>void|Promise<void>;write?:(row:GraphPhaseRowV1)=>void;now?:()=>number;minimumMilliseconds?:number;maximumNames?:number;secondsStep?:number;maximumResourceCharacters?:number;maximumNameCharacters?:number}
export interface GraphPhaseClockV1{begin(phase:string,total:number):void|Promise<void>;advance(phase:string,done:number):void|Promise<void>;running(phase:string,running:readonly GraphPhaseRunningV1[]):void|Promise<void>;end(phase:string,details?:Record<string,unknown>):Promise<GraphPhaseRowV1>;rows():GraphPhaseRowV1[]}
export function createGraphPhaseClockV1(options?:GraphPhaseClockOptionsV1):GraphPhaseClockV1;
export function writeGraphPhaseRowV1(row:GraphPhaseRowV1):void;
export function writeGraphPhaseSummaryV1(rows:readonly GraphPhaseRowV1[]):void;
/** 🔁️ One repeated settlement round: its index, the first conflicting source or the failure that made the whole inference start over. */
export interface GraphSettleRoundV1{readonly round:number;readonly conflict?:string;readonly failure?:unknown}
export function writeGraphSettleRowV1(round:GraphSettleRoundV1):void;
