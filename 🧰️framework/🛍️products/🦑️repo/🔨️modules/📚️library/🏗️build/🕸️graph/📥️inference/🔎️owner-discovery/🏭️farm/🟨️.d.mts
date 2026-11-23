export declare const OWNER_FARM_V1:Readonly<{version:number;maximumChildren:number;childBytes:number;recycleBytes:number;batch:number;requestTimeoutMilliseconds:number;startTimeoutMilliseconds:number;failuresToStop:number;idleMilliseconds:number;environment:string;disabledValue:string}>;
export declare const ownerFarmOffV1:(environment?:Record<string,string|undefined>)=>boolean;
export declare const ownerFarmSizeV1:(capacity:{cpus:number;availableBytes:number})=>number;
export declare function ownerFactsFarmV1(workspaceRoot:string,options?:{environment?:Record<string,string|undefined>;capacity?:{cpus:number;availableBytes:number};size?:number;spawn?:Function;module?:string}):Readonly<{size:number;ready():boolean;discover(request:Readonly<Record<string,unknown>>,signal?:AbortSignal):Promise<unknown>;stats():unknown}>|undefined;
export declare const closeOwnerFactsFarmV1:(workspaceRoot:string)=>void;
