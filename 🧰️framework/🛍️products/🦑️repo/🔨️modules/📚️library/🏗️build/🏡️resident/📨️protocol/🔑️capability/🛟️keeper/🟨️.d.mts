export interface RootKeyKeeperOptionsV1{readonly path:string;readonly key:Uint8Array;readonly touch?:readonly string[];readonly refreshMilliseconds:number;readonly now?:()=>number;}
export function keepRootKeyV1(options:RootKeyKeeperOptionsV1):{tick():"intact"|"refreshed"|"restored"|"failed"};
