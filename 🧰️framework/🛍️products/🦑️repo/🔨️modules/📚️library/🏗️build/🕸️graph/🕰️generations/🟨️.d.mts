/** 🕰️ One observation row: what a computation read and the fact it saw. */
export interface GenerationRowV1{readonly kind:string;readonly path:string;readonly fact:unknown}
export interface GenerationPolicyV1{readonly version:1;readonly pointer:string;readonly manifests:string;readonly fragments:string;readonly indexes:string;readonly frontiers:string;readonly envOwner:string;readonly policies:readonly ["fresh","current"];readonly retainGenerations:number;readonly graceMilliseconds:number;readonly verifyChunkRows:number;readonly verifyParallelChunks:number;readonly bisectBudgetCalls:number;readonly racyMilliseconds:number;readonly readConcurrency:number;readonly publishWaitMilliseconds:number;readonly pollMaximumMilliseconds:number;readonly maximumOwners:number;readonly maximumManifestBytes:number;readonly maximumFragmentBytes:number;readonly maximumIndexBytes:number;readonly compactionFactor:number;readonly compactionFloor:number;readonly brotliQuality:number;readonly brotliWindow:number}
export const GENERATION_POLICY_V1:GenerationPolicyV1;
export type GenerationGraphPolicyV1="fresh"|"current";
export interface GenerationPointerV1{readonly version:1;readonly generation:number;readonly manifest:string|null;readonly parent:string|null;readonly reason?:string}
export interface GenerationOwnerRowV1{readonly root:string;readonly kind:"project"|"unavailable";readonly fragment:string;readonly bytes:number;readonly edgesEnv:string;readonly targets:readonly string[];readonly config?:string;readonly nativeRoot?:string;readonly tag?:string}
export interface GenerationManifestV1{readonly version:1;readonly generation:number;readonly parent:string|null;readonly identity:string;readonly env:string;readonly membership:string;readonly frontier:string|null;readonly factIndex:string;readonly global:string;readonly owners:Readonly<Record<string,GenerationOwnerRowV1>>;readonly built:{readonly kind:"cold"|"delta"|"refresh";readonly base:number|null;readonly dirty:readonly string[]}}
export interface GenerationFragmentV1{readonly version:1;readonly name:string;readonly root:string;readonly kind:"project"|"unavailable";readonly node:unknown;readonly edges:readonly {source:string;target:string;[key:string]:unknown}[];readonly edgesEnv:string;readonly targets:readonly string[];readonly config?:string;readonly nativeRoot?:string}
export interface GenerationSealedV1{readonly pointer:GenerationPointerV1;readonly manifest:GenerationManifestV1|undefined;readonly missing?:boolean}
export interface FactIndexV1{version:1;owners:string[];rows:GenerationRowV1[];classes:{r:number[];o:number[]}[]}
export interface FrontierV1{version:1;capturedAt:number;directories:[string,number,number][];members:[string,number][]}
export interface MembershipChangeV1{path:string;kind:"added"|"removed"|"changed";directory?:boolean;config?:boolean}
export interface GenerationMembershipV1{digest:string;frontier:FrontierV1;changes:MembershipChangeV1[]}
export interface GenerationPlanV1{kind:"cold"|"clean"|"delta";reason?:string;dirty:Set<string>;added:string[];removed:string[];envStale:boolean;checked:number}
export interface GenerationProgressV1{phase:"facts"|"assembly";done:number;total:number}
export interface GenerationComputeSpecV1{cold:boolean;owners:string[];discover:string[];removed:string[];previousEnv:string|null;envStale:boolean;stubs:Record<string,{root:string;kind:"project"|"unavailable";targets:readonly string[];nativeRoot?:string;config?:string;tag?:string}>}
export interface GenerationComputedOwnerV1{name:string;partial?:"edges";root?:string;kind?:"project"|"unavailable";node?:unknown;edges:unknown[];targets?:string[];config?:string;nativeRoot?:string;tag?:string;rows?:GenerationRowV1[]}
export interface GenerationComputeResultV1{owners:GenerationComputedOwnerV1[];env:{digest:string;rows:GenerationRowV1[]};global?:unknown;replaceTags?:string[]}
export interface GenerationFilePortV1{read(path:string):Promise<Uint8Array|undefined>;write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}):Promise<boolean>;names(directory:string):Promise<string[]>;list(directory:string):Promise<{name:string;modifiedMs:number}[]>;remove(path:string):Promise<void>}
export interface GenerationLeasePortV1{exclusive<T>(resource:string,body:()=>Promise<T>,options?:{signal?:AbortSignal;timeoutMs?:number}):Promise<T>}
export interface GenerationStoreOptionsV1{root:string;fs?:GenerationFilePortV1;lease?:GenerationLeasePortV1;holds:(rows:readonly GenerationRowV1[],signal?:AbortSignal)=>Promise<boolean>;resources:{facts:string;builder?:string};now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}
export interface GenerationRefreshOptionsV1{identity:string;policy?:GenerationGraphPolicyV1;only?:ReadonlySet<string>;membership:(previous:{manifest:GenerationManifestV1;frontier:FrontierV1}|undefined,signal?:AbortSignal)=>Promise<GenerationMembershipV1>;compute:(spec:GenerationComputeSpecV1,signal?:AbortSignal)=>Promise<GenerationComputeResultV1>;signal?:AbortSignal;progress?:(event:GenerationProgressV1)=>void|Promise<void>;builder?:<T>(body:()=>Promise<T>)=>Promise<T>}
export interface GenerationRefreshResultV1{current:GenerationSealedV1;served:"current"|"verified"|"built";built:boolean;published:boolean;computed:number;plan?:GenerationPlanV1}
export interface GenerationAssembledV1{projects:Record<string,unknown>;dependencies:unknown[];externalNodes:Record<string,unknown>;unavailable:{root:string;[key:string]:unknown}[];generation:number;membership:string}
export interface GenerationStoreV1{
 read(signal?:AbortSignal):Promise<GenerationSealedV1|undefined>;readIndex(digest:string):Promise<FactIndexV1|undefined>;readFrontier(digest:string|null):Promise<FrontierV1|undefined>;readFragment(row:GenerationOwnerRowV1):Promise<GenerationFragmentV1|undefined>;
 plan(options:{current:GenerationSealedV1|undefined;identity:string;changes?:MembershipChangeV1[];only?:ReadonlySet<string>;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):Promise<GenerationPlanV1>;
 build(options:Omit<GenerationRefreshOptionsV1,"policy"|"only"|"builder">):Promise<{current:GenerationSealedV1;built:boolean;published:boolean;computed:number;plan:GenerationPlanV1}>;
 refresh(options:GenerationRefreshOptionsV1):Promise<GenerationRefreshResultV1>;
 refreshClosure(options:Omit<GenerationRefreshOptionsV1,"only"|"policy">&{roots:readonly string[];neighbors?:(fragment:GenerationFragmentV1)=>readonly string[];policy?:GenerationGraphPolicyV1}):Promise<GenerationRefreshResultV1&{names:Set<string>}>;
 closureOf(current:GenerationSealedV1|undefined,roots:readonly string[],options?:{neighbors?:(fragment:GenerationFragmentV1)=>readonly string[];signal?:AbortSignal}):Promise<Set<string>>;
 assemble(current:GenerationSealedV1,options?:{names?:readonly string[];compare?:(a:string,b:string)=>number;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):Promise<GenerationAssembledV1>;
 snapshot(options?:{roots?:readonly string[];neighbors?:(fragment:GenerationFragmentV1)=>readonly string[];compare?:(a:string,b:string)=>number;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):Promise<{current:GenerationSealedV1;graph:GenerationAssembledV1}>;
 retire(options?:{signal?:AbortSignal}):Promise<void>;invalidate(options?:{reason?:string;signal?:AbortSignal}):Promise<boolean>;
 publish(options:{manifestDigest:string|null;expectedGeneration:number;reason?:string;signal?:AbortSignal}):Promise<{published:boolean;pointer?:GenerationPointerV1}>;
 readonly stats:{holdCalls:number;rowsChecked:number;computed:number;published:number;casConflicts:number;publishHeldMs:number[];indexLoads:number;fragmentReads:number};
}
export function createGenerationStoreV1(options:GenerationStoreOptionsV1):GenerationStoreV1;
export function createNodeFilePortV1():GenerationFilePortV1;
export function createFileLeasePortV1(directory:string,options?:{staleMilliseconds?:number;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;onHold?:(row:{resource:string;heldMs:number;waitedMs:number})=>void}):GenerationLeasePortV1;
export function readGenerationPointerV1(bytes:Uint8Array):GenerationPointerV1;
export function readGenerationManifestV1(bytes:Uint8Array):GenerationManifestV1;
export function readGenerationFragmentV1(value:unknown):GenerationFragmentV1;
export function canonicalJsonV1(value:unknown):string;
export function compareCodePointsV1(a:string,b:string):number;
export function rowKeyV1(row:GenerationRowV1):string;
export function createFactIndexV1(ownerRows:ReadonlyMap<string,readonly GenerationRowV1[]>):FactIndexV1;
export function patchFactIndexV1(index:FactIndexV1,change:{drop?:readonly string[];upserts?:ReadonlyMap<string,readonly GenerationRowV1[]>;append?:ReadonlyMap<string,readonly GenerationRowV1[]>}):FactIndexV1;
export function refineFactIndexV1(index:FactIndexV1):FactIndexV1;
export function factConsumersV1(index:FactIndexV1):[string,string[]][];
export function staleFactRowsV1(index:FactIndexV1,ids:readonly number[],hold:(rows:GenerationRowV1[],signal?:AbortSignal)=>Promise<boolean>,options?:{signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"];chunk?:number;parallel?:number;budget?:number}):Promise<{stale:Set<number>;calls:number}>;
export function selectFactClassesV1(index:FactIndexV1,only?:ReadonlySet<string>):{classes:number[];ids:number[]};
export function staleConsumersV1(index:FactIndexV1,classes:readonly number[],stale:ReadonlySet<number>):Set<string>;
export function createFrontierV1(value:{directories:Iterable<[string,number,number]>;members:Iterable<[string,number]>;capturedAt:number}):FrontierV1;
export function frontierHoldsV1(frontier:FrontierV1,stat:(path:string)=>Promise<{directory:boolean;file:boolean;mtimeMs:number;ino:number;mode:number}|undefined>,options?:{signal?:AbortSignal;concurrency?:number;racyMilliseconds?:number}):Promise<boolean>;
export function diffFrontiersV1(before:FrontierV1,after:FrontierV1):MembershipChangeV1[];
