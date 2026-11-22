import type {GenerationStaleReasonV1,GenerationComputeResultV1,GenerationComputeSpecV1,GenerationGraphPolicyV1,GenerationMembershipV1,GenerationRowV1,GenerationRowSourceV1,GenerationStoreOptionsV1,FrontierV1,GenerationManifestV1} from "../🟨️.mjs";
export declare const GENERATION_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<"current"|"verified"|"waiting"|"covered"|"pinned"|"building"|"published"|"indexed"|"swept"|"closure"|"cold"|"stale"|"planned"|"summary"|"reason"|"reasonStamp"|"reasonInput"|"reasonListing"|"reasonData"|"reasonAnalysis"|"reasonFact"|"reasonDerived"|"cutoff"|"reasonMembership"|"reasonMore"|"pendingCrate"|"withheldDependency"|"withheldSource"|"changeAdded"|"changeRemoved"|"changeChanged"|"merged"|"restored"|"restoredHead"|"uploaded"|"uploadedHead"|"exchangeFailed"|"storing"|"indexing"|"sealing"|"resumed"|"resumedWaves"|"sealedAge"|"neverSealed"|"projectNeverSealed"|"fleetHolds"|"progressRunning"|"progressIdle",string>>>>;
/** 🔭️ Words how far the whole read of this machine is; `builder` is the graph builder custody holder or null. */
export function renderWholeReadProgressV1(builder:{readonly phase?:string;readonly done?:number;readonly total?:number;readonly heldSeconds?:number}|null|undefined,language:"en"|"de"):string;
/** ⏳️ Words an age in seconds as days, hours and minutes. */
export function generationAgeTextV1(seconds:number):string;
/** 🕰️ The age in seconds of the sealed generation whose manifest is `digest`; undefined when its manifest cannot be listed. */
export function generationAgeSecondsV1(fs:{list(directory:string):Promise<readonly {name:string;modifiedMs:number}[]>},root:string,digest:string,nowMs:number):Promise<number|undefined>;
export function renderGenerationMessageV1(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>,language:"en"|"de"):string;
/** 🚧️ One project the graph withholds: where it is declared, the digest of the observed bytes, the English reason and, for a test project that waits for projects the snapshot lacks, their names. */
export interface WithheldProjectV1{readonly root:string;readonly project?:string;readonly source:string;readonly sha256:string;readonly reason:string;readonly missing?:readonly string[]}
/** 🕸️ Words one withheld project in the language of the caller. */
export function describeUnavailableProjectV1(row:WithheldProjectV1,language:"en"|"de"):string;
/** 🧪️ Withholds the test projects that wait for projects the snapshot lacks; returns their names. */
export function withholdDanglingTestProjectsV1(input:{projects:Record<string,unknown>;configs:Readonly<Record<string,string>>;unavailable:Map<string,WithheldProjectV1>;dangling:readonly Readonly<{source:string;root:string;missing:readonly string[]}>[];digest(file:string):string}):string[];
/** 📣️ Says which projects a partial inference withheld and what repairs each. */
export function announceWithheldProjectsV1(host:Pick<GenerationHostV1,"say">,part:Pick<GenerationPartV1,"owners">):void;
export function extractGraphPolicyV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>):{policy:GenerationGraphPolicyV1;explicit:boolean;args:string[]};
/** 🧱️ The facts of the loaded implementation by consumer: data files for every project and the environment, stamps of environment and all inferrers for the environment. */
export interface ScopedImplementationRowsV1{readonly all:readonly GenerationRowV1[];readonly environment:readonly GenerationRowV1[]}
/** 🌀️ Splits implementation facts by consumer. */
export function scopeImplementationRowsV1(rows:readonly GenerationRowV1[]):ScopedImplementationRowsV1;
/** 🪪️ Digest that stamps a generation. */
export function implementationDigestV1(rows:readonly GenerationRowV1[]):string;
/** 🧩️ What one partial inference produced, in repository terms. */
export interface GenerationPartV1{inferred?:number;phases?:Record<string,number>;owners:{name:string;root:string;nativeRoot?:string;config?:string;tag?:string;node?:unknown;unavailable?:unknown;nodeRows?:GenerationRowSourceV1;reads?:import("../🧵️edge-reads/🟨️.mjs").EdgeReadsV1}[];edgeOnly:{name:string;streamed?:true}[];reEdge?:(name:string,fragment:{node?:unknown})=>Promise<unknown[]>|unknown[];readsOf?:(name:string)=>import("../🧵️edge-reads/🟨️.mjs").EdgeReadsV1|undefined;carried?:readonly string[];derived?:import("../✂️cutoff/🟨️.mjs").DerivedFactV1[];edges:{source:string;target:string;[key:string]:unknown}[];externalNodes?:Record<string,unknown>;parseUnavailable?:unknown[];rows:GenerationRowSourceV1;env:{digest:string;rows:GenerationRowV1[];parts?:import("../🧵️edge-reads/🟨️.mjs").EnvironmentPartsV1};replaceTags?:string[];dispose?:()=>void}
export function shapeComputeResultV1(input:{workspaceRoot:string;part:GenerationPartV1;stubs:GenerationComputeSpecV1["stubs"];separator?:string;implementation?:ScopedImplementationRowsV1}):GenerationComputeResultV1;
/** 🫀️ Hands every counted phase event of a build also to the claim that holds it. */
export function custodyProgressV1<E extends{phase:string;done:number;total:number;running?:unknown}>(builder:unknown,emit:(event:E)=>unknown):(event:E)=>unknown;
export function createGenerationProgressV1(clock:{begin(phase:string,total:number):unknown;advance(phase:string,done:number):unknown;end(phase:string):unknown}):(event:{phase:string;done:number;total:number})=>Promise<void>;
export interface GenerationHostV1{
 /** 🧭️ Words, in the language of the host, how far the whole read of this machine is; called only when a question about every project finds no sealed generation. */
 wholeReadProgress?():Promise<string|undefined>|string|undefined;
 workspaceRoot:string;separator?:string;language:"en"|"de";namedInputs:unknown;registryDigest:string;signal?:AbortSignal;verbose?:boolean;waitNoticeMilliseconds?:number;now?:()=>number;
 release?(options?:{more?:boolean}):Promise<void>;
 /** ✂️ Drops what the cutoff verification of the plan filled (verified blocks, interned rows, settled reads, string interner) once the owners to infer are known and before the first one starts; optional. */
 releaseCutoff?():Promise<void>;
 identity():Promise<string>;
 implementationRows():Promise<readonly GenerationRowV1[]>;
 externalDependencies?():Promise<readonly string[]>;
 preflight?():Promise<void>;
 withShared<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;
 withBuilder<T>(body:(store:unknown,control:unknown,claim?:import("../🔐️custody/🟨️.mjs").SliceClaimV1)=>Promise<T>,request?:import("../🟨️.mjs").GenerationPendingV1,peerWait?:(blocked:boolean)=>void,purpose?:"build"|"restore"):Promise<T>;
 /** 🤝️ Prepares the stale projects of a pending refresh together with the commands waiting for the same graph before the exclusive build; optional, never fails the command. */
 prefetch?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;structural:boolean;implementation:string}):Promise<void>;
 /** 🧲️ Attaches the command to the live builder of its stale projects on this machine (see `attachToBuilderV1`): `recheck` tries to serve the command from the generation published so far; answers what it served, or undefined when no live builder holds the custody against the command; optional. */
 attach?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;structural:boolean;implementation:string},recheck:()=>Promise<{graph:unknown;stats:GenerationLoadStatsV1}|undefined>):Promise<{graph:unknown;stats:GenerationLoadStatsV1}|undefined>;
 storeOptions(store:unknown,control:unknown):GenerationStoreOptionsV1;
 verifyClosure?:import("../🩹️closure-repair/🟨️.mjs").ClosureRepairVerifierV1;
 refetchClosure?(reference:import("../🩹️closure-repair/🟨️.mjs").ClosureRepairReferenceV1,signal?:AbortSignal):Promise<boolean>;
 membership(previous:{manifest:GenerationManifestV1;frontier:FrontierV1}|undefined,signal?:AbortSignal):Promise<GenerationMembershipV1>;
 discoverPart(spec:GenerationComputeSpecV1,signal:AbortSignal|undefined,control:unknown,ownerDone?:()=>void,ownerRunning?:(running:readonly {name:string;seconds:number;kind?:string;rows?:number}[],decision?:unknown)=>unknown,ownerProgress?:(done:number,total:number)=>unknown):Promise<GenerationPartV1>;
 expand(projects:Record<string,unknown>,control:unknown):Promise<Record<string,unknown>>;
 expansionLimits?:{maximumProjects:number;maximumBytes:number};
 readLean(value:unknown):unknown;
 readGraph(value:unknown):unknown;
 clock(control:unknown):Parameters<typeof createGenerationProgressV1>[0];
 say(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>):void;
 /** 📌️ Told once a load served a graph: the sealed generation, its manifest digest, the owners of that manifest in code point order and the owners (every owner when undefined) the load served from it. */
 /** 🪞️ Whether the change cursor proves that nothing changed since `live` (the membership a delta plan of this command was made from) was observed; absent or false means the builder plans again. */
 planUnchanged?(live:GenerationMembershipV1):Promise<boolean>;
 /** 🌐️ The generation exchange with the shared remote cache of the machine group; absent while no remote cache is configured. */
 exchange?:import("../🌐️remote-exchange/🟦️.ts").GenerationExchangeV1;
 /** 🚪️ Lets one machine of the fleet read the whole workspace while the others wait and fetch what it sealed; absent while no fleet cache is configured. `release` ends the claim after the generation was offered. */
 wholeGate?(request:{identity:string;look():Promise<import("../🌐️remote-exchange/🟦️.ts").GenerationLookV1|undefined>;signal?:AbortSignal}):Promise<{outcome:string;waitedMilliseconds:number;release():Promise<void>}>;
 /** 📍️ Told after every wave the build published, with the generation number, so a build that is cut off keeps a cursor for the newest wave and the next read verifies from it instead of the whole tree; optional. */
 published?(generation:number,info?:{done:number;total:number}):void|Promise<void>;
 /** 🌊️ Told after every wave that was derived but not published (a read publishes one generation, at its end), so the progress row and the memory of the inference are settled between the waves; optional. */
 waveDone?(info:{generation:number;done:number;total:number}):void|Promise<void>;
 sealed?(record:{generation:number;manifest:string;owners:readonly string[];covered:ReadonlySet<string>|undefined;atMs:number}):void;
}
export type GenerationLoadStatsV1=Readonly<{staleFacts:number;closuresVerified:number;ownersReinferred:number;ownersCutOff:number;doubtfulFacts:number;verifiedMilliseconds:number}>;
/** 📊️ The stats of one load. */
export function generationLoadStatsV1(result:unknown,verifiedMilliseconds:number):GenerationLoadStatsV1;
type GenerationExchangeOutcomeV1=import("../🌐️remote-exchange/🟦️.ts").GenerationExchangeOutcomeV1;
export type RestoreExpectedWholeGenerationOutcomeV1=
 | (Omit<GenerationExchangeOutcomeV1,"status">&Readonly<{status:Exclude<GenerationExchangeOutcomeV1["status"],"restored">|"busy"}>)
 | (GenerationExchangeOutcomeV1&Readonly<{status:"restored";generation:number;head:import("../🌐️remote-exchange/🟦️.ts").GenerationHeadEvidenceV1;localManifest:string}>);
export type LocalWholeGenerationAdoptionV1=Readonly<{generation:number;localManifest:string}>;
export function restoreExpectedWholeGenerationV1(host:GenerationHostV1,expected:import("../🌐️remote-exchange/🟦️.ts").GenerationHeadCandidateV1,adoption?:Readonly<{beforePublication(local:LocalWholeGenerationAdoptionV1):Promise<void>}>):Promise<RestoreExpectedWholeGenerationOutcomeV1>;
export function loadGenerationGraphV1(host:GenerationHostV1,options?:{policy?:GenerationGraphPolicyV1;closure?:readonly string[];sealedOnly?:boolean;selection?:unknown;slice?:{selection:unknown;roots:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]};lean?:boolean;externalInputs?:readonly string[];pin?:import("../🧷️pin/🟨️.mjs").GenerationPinV1}):Promise<{graph:unknown;stats:GenerationLoadStatsV1}>;
/** 📦️ Expands compact inputs within the capacity of one capture. */
export function expandInChunksV1(projects:Record<string,unknown>,expand:(chunk:Record<string,unknown>)=>Promise<Record<string,unknown>>,limits:{maximumProjects:number;maximumBytes:number}):Promise<Record<string,unknown>>;
/** 🌊️ Expands compact inputs chunk by chunk while the nodes of the assembly arrive. */
export function expandStreamV1(entries:AsyncIterable<[string,unknown]>|Iterable<[string,unknown]>,expand:(chunk:Record<string,unknown>)=>Promise<Record<string,unknown>>,limits:{maximumProjects:number;maximumBytes:number}):Promise<Record<string,unknown>>;
/** 🍰️ The owners a selected scope needs. */
export function sliceClosureV1(input:{owners:Readonly<Record<string,string>>;fragment:(name:string)=>Promise<{edges:readonly {target:string}[];node?:unknown}>;selection:{kind:string;tasks?:readonly {project:string;target:string}[]};roots?:readonly string[];external?:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]}):Promise<Set<string>>;
/** 🔍️ Names the exact input behind one stale reason of a plan, in the language the caller resolved. */
export function describeGenerationReasonV1(reason:GenerationStaleReasonV1,options:{workspaceRoot:string;separator?:string;current?:ReadonlyMap<string,unknown>;language:"en"|"de"}):{project:string;key:keyof typeof GENERATION_MESSAGES_V1["en"];fields:Record<string,string>;why:string;text:string};
/** 💬️ Says why a build starts: the reason of a cold start, or the stale projects followed by the first stale inputs by name. */
export function announceGenerationBuildV1(host:Pick<GenerationHostV1,"say"|"workspaceRoot"|"separator"|"language">,spec:GenerationComputeSpecV1,rows:readonly GenerationRowV1[]):void;
/** 🪦️ The previous owners a build still knows: removed owners are never kept as stubs. */
export function survivingStubsV1<T>(stubs:Readonly<Record<string,T>>,removed:Iterable<string>):Record<string,T>;

/** 🪶️ The part of a fragment the slice closure reads (target edges; per target the projects of its inputs, its `dependsOn` and its declared orchestration children). */
export function sliceViewV1(fragment:{edges:readonly {target:string}[];node?:{targets?:Record<string,{inputs?:readonly unknown[];dependsOn?:unknown;metadata?:unknown}>;namedInputs?:Record<string,readonly unknown[]>}}):{edges:{target:string}[];node:{targets:Record<string,{inputs:(string|{input?:string;projects?:unknown;dependencies?:boolean})[];dependsOn:unknown;metadata?:{semio:{execution:{kind:"orchestration";children:{project:string;target:string}[]}}}}>;namedInputs?:Record<string,(string|{input?:string;projects?:unknown;dependencies?:boolean})[]>}};
