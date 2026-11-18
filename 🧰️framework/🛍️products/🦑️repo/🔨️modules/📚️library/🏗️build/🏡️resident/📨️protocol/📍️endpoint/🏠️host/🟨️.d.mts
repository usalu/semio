export function workspaceIdV1(root:string):string;
export const HOST_SOCKET_BYTES_V1:96;
export function checkoutRootOfV1(directory?:string):string;
export class HostRuntimePathErrorV1 extends Error{code:"runtime-path";base:string;bytes:number;constructor(base:string,bytes:number);}
export interface HostIdentityV1{implementation:string;root:string;state:string;}
export interface HostEndpointsV1{tag:string;identity:HostIdentityV1;directory:string;endpoint:string;key:string;descriptor:string;lock:string;log:string;}
export function hostEndpointsV1(implementation:string,environment?:Record<string,string|undefined>,platform?:string,user?:string,root?:string):HostEndpointsV1;
