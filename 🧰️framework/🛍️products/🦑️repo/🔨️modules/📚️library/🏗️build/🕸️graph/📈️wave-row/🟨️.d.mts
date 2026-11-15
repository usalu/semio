/** 📈️ The shape of the wave row. */
export interface GraphWaveRowV1{readonly version:1;readonly generation:number;readonly done:number;readonly total:number;readonly seconds:number;readonly parentMegabytes:number|null;readonly managedMegabytes:number;readonly peakMegabytes:number;readonly peakProjects:readonly string[];readonly collections:number;readonly collectMilliseconds:number;readonly farm:Readonly<{workers:number;projects:number;adopted:number;failed:number;lost:number;declined:number}>}
export declare const GRAPH_WAVE_POLICY_V1:Readonly<{marker:string;environment:string;maximumPeakProjects:number;languages:readonly("en"|"de")[]}>;
export declare const GRAPH_WAVE_TEXTS_V1:Readonly<Record<"en"|"de",Readonly<Record<"wave"|"peakAt"|"unread",string>>>>;
export function graphWaveRowsWantedV1(environment?:Readonly<Record<string,string|undefined>>):boolean;
export interface GraphWaveMeterV1{observe(bytes:number,names:Iterable<string>):void;parent(memory:Readonly<{peakBytes:number;liveBytes:number;collections:number;collectMilliseconds:number}>):void;farm(tally:Readonly<{workers:number;projects?:number;adopted?:number;failed?:number;lost?:number;declined?:number}>):void;progress(info:Readonly<{done?:number;total?:number}>):void;take(input:Readonly<{generation:number;parentMegabytes?:number|null;done?:number;total?:number}>):GraphWaveRowV1}
export function createGraphWaveMeterV1(options?:{now?:()=>number}):GraphWaveMeterV1;
export declare const graphWaveMeterV1:GraphWaveMeterV1;
export function renderGraphWaveRowV1(row:GraphWaveRowV1):string;
export function parseGraphWaveRowV1(line:string):GraphWaveRowV1|undefined;
export function renderGraphWaveTextV1(row:GraphWaveRowV1,language:"en"|"de"):string;
export function announceGraphWaveV1(input:Readonly<{generation:number;info?:Readonly<{done?:number;total?:number}>;environment?:Readonly<Record<string,string|undefined>>;write?:(line:string)=>void;meter?:GraphWaveMeterV1;observe?:(pid:number)=>Promise<number|null>|number|null;pid?:number;limitMilliseconds?:number}>):Promise<GraphWaveRowV1|undefined>;
