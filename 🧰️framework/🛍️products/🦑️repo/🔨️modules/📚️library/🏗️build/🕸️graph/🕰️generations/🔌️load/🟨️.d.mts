import type {GenerationStaleReasonV1,GenerationComputeResultV1,GenerationComputeSpecV1,GenerationGraphPolicyV1,GenerationMembershipV1,GenerationRowV1,GenerationStoreOptionsV1,FrontierV1,GenerationManifestV1} from "../🟨️.mjs";
export declare const GENERATION_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<"current"|"verified"|"waiting"|"building"|"published"|"closure"|"cold"|"stale"|"reason"|"reasonStamp"|"reasonInput"|"reasonListing"|"reasonData"|"reasonAnalysis"|"reasonFact"|"reasonMembership"|"reasonMore"|"withheldDependency"|"withheldSource"|"changeAdded"|"changeRemoved"|"changeChanged",string>>>>;
export function renderGenerationMessageV1(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>,language:"en"|"de"):string;
/** 🚧️ One project the graph withholds: where it is declared, the digest of the observed bytes, the English reason and, for a test project that waits for projects the snapshot lacks, their names. */
export interface WithheldProjectV1{readonly root:string;readonly project?:string;readonly source:string;readonly sha256:string;readonly reason:string;readonly missing?:readonly string[]}
/** 🚧️ Words one withheld project in the language of the caller. */
export function describeUnavailableProjectV1(row:WithheldProjectV1,language:"en"|"de"):string;
/** 🚧️ Withholds the test projects that wait for projects the snapshot lacks; returns their names. */
export function withholdDanglingTestProjectsV1(input:{projects:Record<string,unknown>;configs:Readonly<Record<string,string>>;unavailable:Map<string,WithheldProjectV1>;dangling:readonly Readonly<{source:string;root:string;missing:readonly string[]}>[];digest(file:string):string}):string[];
/** 📣️ Says which projects a partial inference withheld and what repairs each. */
export function announceWithheldProjectsV1(host:Pick<GenerationHostV1,"say">,part:Pick<GenerationPartV1,"owners">):void;
export function extractGraphPolicyV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>):{policy:GenerationGraphPolicyV1;explicit:boolean;args:string[]};
/** 🧱️ The facts of the loaded implementation by consumer: data files for every project and the environment, stamps of environment and all inferrers for the environment. */
export interface ScopedImplementationRowsV1{readonly all:readonly GenerationRowV1[];readonly environment:readonly GenerationRowV1[]}
/** 🧱️ Splits implementation facts by consumer. */
export function scopeImplementationRowsV1(rows:readonly GenerationRowV1[]):ScopedImplementationRowsV1;
/** 🪪️ Digest that stamps a generation. */
export function implementationDigestV1(rows:readonly GenerationRowV1[]):string;
/** 🧩️ What one partial inference produced, in repository terms. */
export interface GenerationPartV1{owners:{name:string;root:string;nativeRoot?:string;config?:string;tag?:string;node?:unknown;unavailable?:unknown;nodeRows?:GenerationRowV1[]}[];edgeOnly:{name:string}[];edges:{source:string;target:string;[key:string]:unknown}[];externalNodes?:Record<string,unknown>;parseUnavailable?:unknown[];rows:GenerationRowV1[];env:{digest:string;rows:GenerationRowV1[]};replaceTags?:string[]}
export function shapeComputeResultV1(input:{workspaceRoot:string;part:GenerationPartV1;stubs:GenerationComputeSpecV1["stubs"];separator?:string;implementation?:ScopedImplementationRowsV1}):GenerationComputeResultV1;
export function createGenerationProgressV1(clock:{begin(phase:string,total:number):unknown;advance(phase:string,done:number):unknown;end(phase:string):unknown}):(event:{phase:string;done:number;total:number})=>Promise<void>;
export interface GenerationHostV1{
 workspaceRoot:string;separator?:string;language:"en"|"de";namedInputs:unknown;registryDigest:string;signal?:AbortSignal;verbose?:boolean;
 identity():Promise<string>;
 implementationRows():Promise<readonly GenerationRowV1[]>;
 withShared<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;
 withBuilder<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;
 storeOptions(store:unknown,control:unknown):GenerationStoreOptionsV1;
 membership(previous:{manifest:GenerationManifestV1;frontier:FrontierV1}|undefined,signal?:AbortSignal):Promise<GenerationMembershipV1>;
 discoverPart(spec:GenerationComputeSpecV1,signal:AbortSignal|undefined,control:unknown):Promise<GenerationPartV1>;
 expand(projects:Record<string,unknown>,control:unknown):Promise<Record<string,unknown>>;
 expansionLimits?:{maximumProjects:number;maximumBytes:number};
 readLean(value:unknown):unknown;
 readGraph(value:unknown):unknown;
 clock(control:unknown):Parameters<typeof createGenerationProgressV1>[0];
 say(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>):void;
}
export function loadGenerationGraphV1(host:GenerationHostV1,options?:{policy?:GenerationGraphPolicyV1;closure?:readonly string[];selection?:unknown;slice?:{selection:unknown;roots:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]};lean?:boolean}):Promise<unknown>;
/** 📦️ Expands compact inputs within the capacity of one capture. */
export function expandInChunksV1(projects:Record<string,unknown>,expand:(chunk:Record<string,unknown>)=>Promise<Record<string,unknown>>,limits:{maximumProjects:number;maximumBytes:number}):Promise<Record<string,unknown>>;
/** 🌊️ Expands compact inputs chunk by chunk while the nodes of the assembly arrive. */
export function expandStreamV1(entries:AsyncIterable<[string,unknown]>|Iterable<[string,unknown]>,expand:(chunk:Record<string,unknown>)=>Promise<Record<string,unknown>>,limits:{maximumProjects:number;maximumBytes:number}):Promise<Record<string,unknown>>;
/** 🍰️ The owners a selected scope needs. */
export function sliceClosureV1(input:{owners:Readonly<Record<string,string>>;fragment:(name:string)=>Promise<{edges:readonly {target:string}[];node?:unknown}>;selection:{kind:string;tasks?:readonly {project:string;target:string}[]};roots?:readonly string[];external?:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]}):Promise<Set<string>>;
/** 🔍️ Names the exact input behind one stale reason of a plan, in the language the caller resolved. */
export function describeGenerationReasonV1(reason:GenerationStaleReasonV1,options:{workspaceRoot:string;separator?:string;current?:ReadonlyMap<string,unknown>;language:"en"|"de"}):{project:string;key:keyof typeof GENERATION_MESSAGES_V1["en"];fields:Record<string,string>;why:string;text:string};
/** 📣️ Says why a build starts: the reason of a cold start, or the stale projects followed by the first stale inputs by name. */
export function announceGenerationBuildV1(host:Pick<GenerationHostV1,"say"|"workspaceRoot"|"separator"|"language">,spec:GenerationComputeSpecV1,rows:readonly GenerationRowV1[]):void;
