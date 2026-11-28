/** 🚦️ What the host asks one slice pool to load (any further field travels to the worker as its data); loads with the same root, arguments, tool and policy share one worker. */
export type SliceLoadRequestV1=Readonly<{root:string;arguments?:readonly string[];tool?:string;graphPolicy?:"fresh"|"current";grant?:string;owner?:string;priority?:number}&Readonly<Record<string,unknown>>>;
/** 📣️ How a caller hears the counted phases of a load it waits for. */
export type SliceLoadObserverV1=Readonly<{signal?:AbortSignal;onProgress?:(...row:never[])=>void;wantsReport?:()=>boolean}>;
/** 🏊️ The ports of one pool: every one has a default of the running host, so a test replaces only what it observes. */
export type SlicePoolOptionsV1=Readonly<{
 workerUrl:URL;maximum?:number;heartbeatMilliseconds?:number;budgetMegabytes?:number;marginMegabytes?:number;
 elasticity?:Readonly<Record<string,number>>;
 headroom?:()=>Readonly<{limitBytes:number;currentBytes:number;availableBytes?:number}>;
 sampleMilliseconds?:number;note?:()=>void;
 createWorker?:(url:URL,data:SliceLoadRequestV1&Readonly<{heartbeatMilliseconds:number}>,environment:Readonly<Record<string,string|undefined>>)=>unknown;
 pressure?:()=>unknown;limiter?:unknown;learned?:Readonly<{read(scope?:string):number;write(megabytes:number,scope?:string):void}>;
 reserve?:(request:Readonly<{megabytes:number;minimumMegabytes:number;parentGrant?:string;signal?:AbortSignal;onWait:(facts:unknown)=>void}>)=>unknown;
 observe?:(pid:number)=>unknown;unlearnedMegabytes?:number;hostMegabytes?:number;graceMilliseconds?:number;claimRefreshMilliseconds?:number;reaper?:unknown;
 growth?:unknown;coalesce?:Readonly<{gatherMilliseconds:number;maximumTasks:number}>;planGuard?:unknown;restartPolicy?:unknown;quietPolicy?:unknown;implementationStamps?:unknown;wallNow?:()=>number;cutPolicy?:unknown;recycle?:unknown;
 predecessor?:()=>Readonly<{pid:number;busy:number}>|undefined;predecessorBoundMilliseconds?:number;hostPressure?:()=>unknown;claims?:Readonly<{claim(pid:number,claim:Readonly<{grantMegabytes?:number;stopped:boolean;owner?:string}>):void;release(pid:number):void}>;report?:(line:string)=>void;now?:()=>number;planLimitMegabytes?:number;
}>;
/** 🔢️ Clamps a request priority to 0..9; 5 when it is no such integer. */
export function slicePriorityV1(priority:unknown):number;
export function sliceWorkerEnvironmentV1(base:Readonly<Record<string,string|undefined>>,request?:Readonly<{owner?:string}>):Record<string,string|undefined>;
/** 🔑️ The key under which loads of the same request share one worker. */
export function shareKeyV1(request:Readonly<{root:unknown;arguments?:readonly string[]|null;tool?:string|null;graphPolicy?:string|null}&Readonly<Record<string,unknown>>>):string;
/** 🏊️ Creates a pool of slice worker processes that loads one graph slice per request within the memory budget of the host. */
export function createSlicePoolV1(options:SlicePoolOptionsV1):Readonly<{
 run(request:SliceLoadRequestV1,observer?:SliceLoadObserverV1):Promise<{graph:unknown;generationPin?:string}>;
 stats():Readonly<Record<string,number>>;
 memory(now?:number):readonly Readonly<Record<string,unknown>>[];
 busy():number;
 handover(error:Error):void;
 close():Promise<void>;
}>;
