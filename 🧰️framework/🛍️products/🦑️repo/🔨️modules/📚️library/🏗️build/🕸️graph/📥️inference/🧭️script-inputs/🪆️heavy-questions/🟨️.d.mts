/** 🪆️ The declared policy of the heavy derived questions. */
export const HEAVY_QUESTION_POLICY_V1:Readonly<{version:1;environment:string;memoryLimitMegabytes:number;pollMilliseconds:number;timeoutMilliseconds:number;heartbeatMilliseconds:number;livenessMilliseconds:number;silenceMilliseconds:number;maximumScale:number;kinds:readonly("program"|"derive")[]}>;
/** 🐢️ How many times longer than on the reference machine a child may show no sign of life. */
export function heavyQuestionScaleV1(policy?:typeof HEAVY_QUESTION_POLICY_V1,environment?:Record<string,string|undefined>,load?:number):number;
/** 🪆️ The ports of a law: the scale of the machine and the reader of the processor time of a child. */
export type HeavyQuestionPortsV1=Readonly<{scale?:(policy:typeof HEAVY_QUESTION_POLICY_V1)=>number;cpuOf?:(pid:number)=>number|undefined}>;
/** 🪆️ Whether a derived row must be answered in a child process. */
export function isHeavyQuestionV1(row:{path:string},environment?:Record<string,string|undefined>):boolean;
/** 🪆️ Asks one heavy derived question in a child process, one at a time. */
export function askHeavyQuestionV1(row:{kind:string;path:string;fact:unknown},options:{workspaceRoot:string;signal?:AbortSignal;policy?:typeof HEAVY_QUESTION_POLICY_V1;script?:string;ports?:HeavyQuestionPortsV1}):Promise<{fact:unknown;rows:readonly unknown[]}>;


/** 🧰️ Derives the shared closure entries in the capped child and stores them below `storeDirectory`. */
export function primeSharedClosuresInChildV1(options:{workspaceRoot:string;storeDirectory:string;signal?:AbortSignal;policy?:typeof HEAVY_QUESTION_POLICY_V1;script?:string;ports?:HeavyQuestionPortsV1}):Promise<{recipes:number;derived:number;loaded:number;stale:number}>;
/** 🛃️ Walks every declared tool closure and provider port in the capped child, once per process and policy, and refuses with what it found. */
export function preflightToolClosuresInChildV1(options:{workspaceRoot:string;signal?:AbortSignal;ports?:HeavyQuestionPortsV1}):Promise<{checked:number}>;
/** 🛃️ The refusal of a preflight. */
export function refusedToolClosuresV1(rows:readonly {name:string;code:string;key:string;values:Record<string,string|number>}[]):Error&{code:string;key:string;values:Record<string,string|number>};
