export type ImplementationQuietPolicyV1={readonly milliseconds:number;readonly maximumMilliseconds:number};
export type ImplementationQuietVerdictV1=Readonly<{quiet:boolean;quietForMilliseconds:number;newestPath?:string}>;
export function implementationDriftPathsV1(cause:unknown):string[];
export function implementationStampsV1(paths:Iterable<string>,stat?:(path:string)=>{ctimeMs:number}):Map<string,number|undefined>;
export function implementationQuietV1(input:{stamps:Iterable<readonly [string,number|undefined]>;nowMs:number;quietMilliseconds:number}):ImplementationQuietVerdictV1;
export function implementationPathShownV1(root:unknown,path:string|undefined):string;
export function awaitImplementationQuietV1(input:{paths:Iterable<string>;policy:ImplementationQuietPolicyV1;heartbeatMilliseconds:number;signal?:AbortSignal;announce?:(verdict:ImplementationQuietVerdictV1,waitedMilliseconds:number)=>void;stamps?:(paths:Iterable<string>)=>Map<string,number|undefined>;now?:()=>number}):Promise<number>;
