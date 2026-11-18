/** 🪶️ The declared numbers of the lean slice plan. */
export interface LeanSlicePolicyV1{readonly version:1;readonly modeEnvironment:string;readonly modes:readonly string[];readonly parentMegabytes:number;readonly childMegabytes:number;readonly ownersPerChild:number;readonly loopsPerChild:number;readonly minimumChildren:number;readonly coreReserve:number;readonly ceilingBasisPoints:number}
export declare const LEAN_SLICE_POLICY_V1:LeanSlicePolicyV1;
export declare const LEAN_SLICE_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<string,string>>>>;
export function renderLeanSliceMessageV1(key:"lean"|"tooSmall",fields:Readonly<Record<string,string|number>>,environment?:Readonly<Record<string,string|undefined>>):string[];
/** 🪶️ The plan of a lean slice read. */
export interface LeanSlicePlanV1{readonly lean:boolean;readonly limitMegabytes:number|null;readonly parentMegabytes:number;readonly childMegabytes:number;readonly children:number;readonly ownersPerChild:number;readonly loopsPerChild:number;readonly fits:boolean}
export function leanSlicePlanV1(input:{limitMegabytes:number;ceilingMegabytes?:number;cores?:number;residentMegabytes?:number;environment?:Readonly<Record<string,string|undefined>>;policy?:LeanSlicePolicyV1}):LeanSlicePlanV1;
export function leanFarmPolicyV1<T extends object>(farm:T,plan:LeanSlicePlanV1):T;
