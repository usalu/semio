/** 🧷️ The declared constants of the generation pin. */
export interface GenerationPinPolicyV1{readonly version:1;readonly environment:"SEMIO_BUILD_GENERATION_PIN";readonly invocationEnvironment:"SEMIO_BUILD_INVOCATION_PID";readonly chainEnvironment:"SEMIO_BUILD_INVOCATION_PIDS";readonly maximumBytes:number;readonly maximumAgeMilliseconds:number;readonly tools:readonly string[]}
export const GENERATION_PIN_POLICY_V1:GenerationPinPolicyV1;
export type GenerationPinMessageKeyV1="uncovered"|"retired"|"identity"|"malformed";
export const GENERATION_PIN_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<GenerationPinMessageKeyV1,string>>>>;
export function renderGenerationPinMessageV1(key:GenerationPinMessageKeyV1,fields:Readonly<Record<string,string|number>>,language:"en"|"de"):string;
/** 🚫️ The refusal of a nested command whose pinned generation cannot serve it. */
export interface GenerationPinRefusalV1 extends Error{readonly code:"GENERATION_PIN_REFUSED";readonly reason:GenerationPinMessageKeyV1;readonly fields:Readonly<Record<string,string|number>>}
export function generationPinRefusalV1(key:GenerationPinMessageKeyV1,fields?:Readonly<Record<string,string|number>>):GenerationPinRefusalV1;
/** 📌️ A closed pin: the owning invocation, the sealed manifest and the verified stamp of the owners it served. */
export interface GenerationPinV1{readonly version:1;readonly owner:number;readonly generation:number;readonly manifest:string;readonly covered:{readonly atMs:number;readonly count:number;readonly owners:string}}
export function encodeGenerationPinV1(input:{owner:number;generation:number;manifest:string;owners:readonly string[];covered?:ReadonlySet<string>;atMs:number}):string;
export function readGenerationPinV1(text:string):GenerationPinV1;
export function generationPinOwnerV1(environment:Readonly<Record<string,string|undefined>>,pid?:number):number;
export type GenerationPinStateV1={readonly state:"none"}|{readonly state:"bound"|"foreign";readonly pin:GenerationPinV1};
export function generationPinStateV1(environment:Readonly<Record<string,string|undefined>>,options?:{alive?:(pid:number)=>boolean;self?:number;now?:number}):GenerationPinStateV1;
export function generationPinOpenV1(input:{pin:GenerationPinV1;owners:readonly string[];needed?:Iterable<string>}):string[];
export function readPinnedGenerationV1(input:{engine:{readManifestAt(digest:string,signal?:AbortSignal):Promise<unknown>;closureOf(current:unknown,roots:readonly string[],options?:{signal?:AbortSignal}):Promise<Set<string>>};pin:GenerationPinV1;identity:string;roots?:readonly string[];closure?:(current:unknown,options?:{signal?:AbortSignal})=>Promise<Set<string>>;signal?:AbortSignal}):Promise<{current:{readonly pointer:{readonly version:1;readonly generation:number;readonly manifest:string;readonly parent:string|null};readonly manifest:any};served:"pinned";built:false;published:false;computed:0;names?:Set<string>}>;
