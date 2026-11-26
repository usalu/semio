export function ownerFailureOfV1(error:unknown,depth?:number):{owners:string;message:string}|undefined;
export function accessDeniedOfV1(error:unknown,options?:{root?:string;platform?:string;home?:string}):string|undefined;
export function failureRefusalV1(error:unknown,options:{log:(event:string,detail?:string)=>void;logPath:string;fullImage?:(root:string,request:Readonly<{paths:readonly string[]}>)=>unknown;inspect?:(path:string)=>unknown;retry?:"later"|"never";root?:string;platform?:string;home?:string;environment?:Record<string,string|undefined>;pid?:number;ppid?:number;streak?:{cause:string;count:number};sameCauseMaximum?:number}):{code:"disk"|"internal"|"unserved"|"consent";key:string;fields:Record<string,string>;retry:"later"|"never"};
export function wantedRefusalOfV1(error:unknown,depth?:number):Record<"wanted"|"arrived"|"seconds"|"missing"|"local",string>|undefined;
export function deniedPlaceOfV1(error:unknown,depth?:number):string|undefined;
export function serviceFactsV1(options?:{environment?:Record<string,string|undefined>;pid?:number;ppid?:number}):string;
export function serviceRouteV1(environment?:Record<string,string|undefined>):"launchd"|"direct"|"foreground";
export const FAILURE_BLOCK_BYTES_V1:number;
export function failureMessageV1(error:unknown):string;
export function failureFramesV1(error:unknown):string;
export function failureBlockV1(input:{reference:string;detail:string;error:unknown;refusal:{key:string;fields:Record<string,string>};at?:Date;limit?:number}):string;
export function mirrorFailureV1(block:string,reference:string,options?:{environment?:Record<string,string|undefined>;root?:string}):string|undefined;

/** 🪜️ The most bytes of stack frames one failure block keeps when the message is longer than the block leaves room for: the frames say where it was thrown, the message is cut instead. */
export const FAILURE_FRAME_BYTES_V1: 16384;
/** 🏷️ The text a failure is logged by: its stack, led by its name, code, message and the project it belongs to (`semioProject`) whenever the stack does not carry them, so that a failure whose stack lost its message still says what failed and for which project. */
export function failureDetailV1(error: unknown): string;
/** 🛑️ The words of a graph read that stopped projects too heavy for this machine (`build.graph.owner-too-heavy`): the refusal\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs */
export function heavyRefusalOfV1(error: unknown, depth?: number): Readonly<{named:string;limit:string;producers:string}>|undefined;
/** 🔒️ A lease ordering violation as the strings a client renders. */
export type LeaseRefusalV1=Readonly<{requested:string;mode:"exclusive"|"shared";held:string;seconds:string;kind:string;holderPid:string;holderCommand:string;holderSeconds:string;bound:string}>;
/** 🔒️ The words of a read refused by the lease order (`SEMIO_LEASE_ORDER`): the violation the error carries (what was wanted, what this process held, how long it waited, who held it) as language-neutral fields, or `undefined` for every other failure. @see ../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts */
export function leaseRefusalOfV1(error: unknown, depth?: number): LeaseRefusalV1|undefined;
/** ⌛️ The words of a lease wait that ran out (`SEMIO_LEASE_TIMEOUT`, the typed refusal of every lease class): the resource, the mode, the seconds waited and the holder the lease registry names, as language-neutral fields, or `undefined` for every other failure. @see ../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts */
export function leaseTimeoutRefusalOfV1(error: unknown, depth?: number): Readonly<{requested:string;mode:"exclusive"|"shared";seconds:string;holderPid:string;holderCommand:string;holderSeconds:string}>|undefined;
/** 🚧 The owner whose own source failed a read (`BUILD_SOURCE_UNPARSEABLE`, in the error or its causes): the source and the first line of the failure, and whether the replica has not settled (the source was written a moment ago); `undefined` for every other failure. A failure of one owner's source is a fact about that owner, never an unexpected failure of the service. @see ../../../🕸️graph/📥️inference/📦️project-nodes/🚧️source-failure/🟨️.mjs */
export function ownerSourceOfV1(error: unknown, depth?: number): Readonly<{unsettled:boolean;fields:Readonly<{source:string;message:string}>}>|undefined;
/** 🪜️ The words of a graph read whose plan (the assembly of a wave) grew past what this machine gives a plan (`build.graph.plan-too-heavy`): the refusal\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
export function planRefusalOfV1(error: unknown, depth?: number): Readonly<Record<"stage"|"operation"|"megabytes"|"limit"|"available"|"reserve",string>>|undefined;
/** 🔬️ Whether a relative filesystem call that the helper reports as refused (`EPERM`, `reads` agreeing readings of its failure number) is really a refusal of the access of this process: the same place is read once more by path, with the node file system, which does not share the native call. A refusal by path as well (`EPERM`) is the macOS privacy refusal and gives `undefined`; any other answer (the place exists, is absent, or is an ordinary permission) is returned with the place, the reads and the answer, because then the consent of the user is not what is missing. @see https://developer.apple.com/documentation/bundleresources/protecting-the-user-s-files */
export function relativeCallDivergenceV1(error: unknown, options?: Readonly<{inspect?:(path:string)=>unknown}>): Readonly<{place:string;reads:number;found:string}>|undefined;
