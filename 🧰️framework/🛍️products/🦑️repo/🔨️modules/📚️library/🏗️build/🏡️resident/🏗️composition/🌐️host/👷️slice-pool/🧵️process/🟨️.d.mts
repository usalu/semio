export const SLICE_REQUEST_VARIABLE_V1:string;
export const SLICE_SPOOL_VARIABLE_V1:string;
export function createProcessWorkerV1(url:URL|string,data:unknown,env:Record<string,string|undefined>,options?:{spawn?:unknown;graceMilliseconds?:number;platform?:string}):{pid?:number;on(event:"message"|"error"|"exit",listener:(value:any)=>void):void;terminate():Promise<void>};
export const workerData:any;
export const parentPort:{postMessage(message:unknown):void};
