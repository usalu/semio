/** ✂️ The declared ladder of the graph work cut. */
export interface GraphCutPolicyV1{readonly version:1;readonly levels:number;readonly maximumConcurrency:number;readonly minimumWaveBasisPoints:number;readonly unboundedMegabytes:number;readonly relaxBasisPoints:number;readonly heavyHandoffs:number;readonly concurrencyEnvironment:string;readonly waveEnvironment:string}
export declare const GRAPH_CUT_POLICY_V1:GraphCutPolicyV1;
/** 🪜 One level of the ladder. */
export interface GraphCutV1{readonly level:number;readonly concurrency?:number;readonly waveBasisPoints:number}
export function graphCutV1(level:number,policy?:GraphCutPolicyV1):GraphCutV1;
export function graphCutAfterV1(input:{level:number;treeMegabytes:number;limitMegabytes:number},policy?:GraphCutPolicyV1):number|undefined;
export function graphCutStartLevelV1(input:{limitMegabytes:number},policy?:GraphCutPolicyV1):number;
export function graphCutEnvironmentV1(cut:GraphCutV1,policy?:GraphCutPolicyV1):Readonly<Record<string,string>>;
export function graphCutConcurrencyV1(environment?:Readonly<Record<string,string|undefined>>,policy?:GraphCutPolicyV1):number;
export function graphCutLearnedV1(input:{level:number;peakMegabytes:number;limitMegabytes:number},policy?:GraphCutPolicyV1):number;
