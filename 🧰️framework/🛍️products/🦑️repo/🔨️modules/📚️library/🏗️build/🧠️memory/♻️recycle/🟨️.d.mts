/** ♻️ The declared policy of the slice worker recycling. */
export interface SliceRecyclePolicyV1{readonly version:1;readonly heapMegabytes:number;readonly maximumRecycles:number;readonly heapEnvironment:string;readonly stateEnvironment:string}
export declare const SLICE_RECYCLE_POLICY_V1:SliceRecyclePolicyV1;
/** 📦 The counters a recycled read carries. */
export interface SliceRecycleStateV1{readonly recycles:number;readonly round:number;readonly passes:number;readonly owners:number;readonly heapMegabytes:number}
export interface SliceRecycleDecisionV1{readonly heapMegabytes:number;readonly limitMegabytes:number;readonly recycles:number}
export function sliceRecycleEnvironmentV1(state?:SliceRecycleStateV1,policy?:SliceRecyclePolicyV1):Readonly<Record<string,string>>;
export function sliceRecycleCarriedV1(environment?:Readonly<Record<string,string|undefined>>,policy?:SliceRecyclePolicyV1):SliceRecycleStateV1;
export function sliceRecycleDecisionV1(input:{environment?:Readonly<Record<string,string|undefined>>;heapBytes:number;carried?:SliceRecycleStateV1},policy?:SliceRecyclePolicyV1):SliceRecycleDecisionV1|undefined;
export function sliceRecycleErrorV1(input:{decision:SliceRecycleDecisionV1;round:number;passes:number;owners:number;carried?:SliceRecycleStateV1},policy?:SliceRecyclePolicyV1):Error&{code:"SEMIO_SLICE_RECYCLE";recycle:SliceRecycleStateV1};
export function sliceRecycleOfV1(error:unknown):SliceRecycleStateV1|undefined;
