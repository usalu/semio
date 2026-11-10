/** 🪆️ The declared policy of the heavy derived questions. */
export const HEAVY_QUESTION_POLICY_V1:Readonly<{version:1;environment:string;memoryLimitMegabytes:number;pollMilliseconds:number;timeoutMilliseconds:number;kinds:readonly("program"|"derive")[]}>;
/** 🪆️ Whether a derived row must be answered in a child process. */
export function isHeavyQuestionV1(row:{path:string},environment?:Record<string,string|undefined>):boolean;
/** 🪆️ Asks one heavy derived question in a child process, one at a time. */
export function askHeavyQuestionV1(row:{kind:string;path:string;fact:unknown},options:{workspaceRoot:string;signal?:AbortSignal}):Promise<{fact:unknown;rows:readonly unknown[]}>;


/** 🧰️ Derives the shared closure entries in the capped child and stores them below `storeDirectory`. */
export function primeSharedClosuresInChildV1(options:{workspaceRoot:string;storeDirectory:string;signal?:AbortSignal}):Promise<{recipes:number;derived:number;loaded:number;stale:number}>;
