export const SLICE_LIMIT_FLOOR_MEGABYTES_V1:number;
export const SLICE_LIMIT_SOFT_BASIS_POINTS_V1:number;
export function sliceLimiterV1(ports?:unknown):{create(policy:unknown,job:string):Promise<{attach(pid:number):Promise<void>;onBreach(callback:(breach:{peakMegabytes:number;limitMegabytes:number})=>void):void;release():Promise<unknown>}>}|undefined;
export function hostAvailableBytesV1(read?:(path:string)=>string):number|undefined;
export function sliceLimitPolicyV1(headroom:{limitBytes:number;currentBytes:number;availableBytes?:number},marginMegabytes?:number,elasticity?:{maximumMegabytes?:number;hostShareBasisPoints?:number;hostFloorMegabytes?:number}):{kind:"service";memoryMegabytes:number;swapMegabytes:0;cpuShares:number;pids:number;softRatioBasisPoints:number;enforcement:"hard"}|undefined;
