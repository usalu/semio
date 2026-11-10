export function openCommandV1(tag:string,args:readonly string[]):string;
export function createWatchHubV1(options:{binary:string;spawnHub?:unknown;maximumRoots?:number}):{stats:{roots:number;frames:number;restarts:number};spawnProcess(binary:string,args:readonly string[],options?:unknown):any;stop():Promise<void>};
