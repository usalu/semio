export const SLICE_REQUEST_VARIABLE_V1:string;
export const SLICE_SPOOL_VARIABLE_V1:string;
export function createProcessWorkerV1(url:URL|string,data:unknown,env:Record<string,string|undefined>,options?:{spawn?:unknown;graceMilliseconds?:number;platform?:string}):{pid?:number;on(event:"message"|"error"|"exit",listener:(value:any)=>void):void;terminate():Promise<void>};
export const workerData:any;
export const parentPort:{postMessage(message:unknown):void};
export function errorMessageV1(error:unknown):{message:string;stack:string;data?:Record<string,unknown>;refusal?:{code:string;key:string;values:Record<string,unknown>}};
export function errorOfMessageV1(message:{type?:string;message:string;stack?:string;data?:Record<string,unknown>;refusal?:{code:string;key:string;values:Record<string,unknown>}}):Error&Record<string,any>;

/** 🗣 The line of a worker's last words that says why it ended: the last line that names an error, else the last line, without colour codes and at most 300 characters; nothing when the worker said nothing. */
export function workerCauseV1(text: string|undefined): string;
