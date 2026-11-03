export declare function createFrontierV1(input:{directories:Iterable<[string,number,number]>;members:Iterable<[string,number]>;capturedAt:number}):{version:1;capturedAt:number;directories:[string,number,number][];members:[string,number][]};
export declare function frontierFactHoldsV1(present:number,recorded:number):boolean;
export declare function frontierHoldsV1(frontier:any,stat:(path:string)=>Promise<any>,options?:{signal?:AbortSignal;concurrency?:number;racyMilliseconds?:number}):Promise<boolean>;
export declare function stableDirectoriesV1(before:any,after:any,options?:{racyMilliseconds?:number}):Set<string>;
export declare function diffFrontiersV1(before:any,after:any):{path:string;kind:"added"|"removed"|"changed";directory:boolean}[];
