export interface PlanFileV1{readonly name:string;readonly bytes:number;readonly ageMilliseconds:number}
export interface PlanRetentionPolicyV1{readonly retainMilliseconds:number;readonly maximumAgeMilliseconds:number;readonly maximumBytes:number;readonly maximumEntries:number}
export function planRetentionV1(options:{files:readonly PlanFileV1[];live?:Iterable<string>;policy:PlanRetentionPolicyV1}):string[];
