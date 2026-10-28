/** 📋️ The closed numbers, variable names, exit code and EN/DE texts of the graph queue yield. */
export type GraphQueueYieldPolicyV1=Readonly<{version:1;yieldAfterMilliseconds:number;maximumYields:number;wakeMilliseconds:number;pollMilliseconds:number;progressMilliseconds:number;environment:Readonly<{ticket:string;requestedAt:string;round:string}>;exitCode:number;texts:Readonly<Record<"waiting"|"resumed",Readonly<Record<string,string>>>>}>;
type Env=Readonly<Record<string,string|undefined>>;
/** 📜️ The closed numbers, variable names and EN/DE texts of the graph queue yield, exactly as the policy declares them. */
export function graphQueueYieldPolicyV1():GraphQueueYieldPolicyV1;
/** 🕰️ The time the first start of this command asked for the graph; undefined for a first start. */
export function graphQueueRequestedAtV1(environment?:Env):number|undefined;
/** 🔢️ How often this command has already yielded the graph queue. */
export function graphQueueYieldRoundV1(environment?:Env):number;
/** ⚖️ Whether a command that has queued for the declared time for the graph builder yields now. */
export function graphQueueYieldsV1(options:{queuedMs:number;environment?:Env;yieldAfterMilliseconds?:number}):boolean;
/** 🛌️ Ends the waiting process: writes the ticket, tells the user in both languages and exits with the yield code. */
export function yieldGraphQueueV1(options:{generation:number;pointer:string;requestedAt:number;queuedMs?:number;environment?:Env;write?:(text:string)=>void;exit?:(code:number)=>void;now?:()=>number}):void;
/** ⏳️ The front door's wait after a yielded run: polls the pointer until a newer generation was published or the wake time passed. */
export function waitGraphQueueYieldV1(options?:{environment?:Env;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;write?:(text:string)=>void;read?:(pointer:string)=>number;signal?:AbortSignal}):Promise<Readonly<Record<string,string>>|undefined>;
