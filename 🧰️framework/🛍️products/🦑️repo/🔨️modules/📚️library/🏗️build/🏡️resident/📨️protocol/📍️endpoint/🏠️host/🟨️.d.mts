export function workspaceIdV1(root:string):string;
export interface HostEndpointsV1{tag:string;directory:string;endpoint:string;key:string;descriptor:string;lock:string;log:string;}
export function hostEndpointsV1(implementation:string,environment?:Record<string,string|undefined>,platform?:string,user?:string):HostEndpointsV1;
