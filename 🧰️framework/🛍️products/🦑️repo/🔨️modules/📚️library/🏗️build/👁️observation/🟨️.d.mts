import type * as fs from "node:fs";
import type {readFile} from "node:fs/promises";
import type {ResourceLeaseControl} from "@semio-tech/process/leases";
import type{ExactSourceReadBatchV1,ExactSourceFileV1}from"../📥️inputs/🧮️digest/📥️read/🟨️.mjs";

/** 🚧️ The declared limits of the observation module: its schema file is their single source. */
export const OBSERVATION_LIMITS_V1: Readonly<{settledReadMaximumFileBytes:number;settleRounds:number;conflictRounds:number;conflictPauseMilliseconds:number;holdNestingLimit:number;racyMilliseconds:number;sourceAnalysisBytes:number;sourceAnalysisOverheadBytes:number;settledReadBytes:number;fileRecordImageBytes:number;fileRecordImageMaximumBytes:number;verifiedBlockRows:number;blockMinimumRows:number;blockCutMask:number;blockRefusalMilliseconds:number;detachedLayerRows:number;leafReadChunkBytes:number;sweepGraceMilliseconds:number;fingerprintProgressRows:number;sweepHeaderProgressRows:number;sweepBlockProgressRows:number;maximumIdentifierCharacters:number;nativeRealpathMaximumCharacters:number}>;

/** 🧾️ Describes exactly one byte image admitted by the defining observation schema. */
export type SourceFileFactV1 = Readonly<{size:number;mtimeMs:number;ctimeMs:number;ino:number;sha256:string}>;
/** 🧯️ Retains a physical read failure without inventing a source image. */
export type SourceReadFailureV1 = Readonly<{error:string}>;
/** 🧠️ Records one registered parser's JSON result and complete nested consumed facts. */
export type SourceAnalysisFactV1 = Readonly<{provider:string;revision:string;source:SourceFileFactV1;resultSha256:string;nestedObservations:readonly SourceObservationV1[]}>;
/** 👁️ Uses the canonical schema's observation kinds and operation-specific fact shapes. */
export type SourceObservationV1 =
 | Readonly<{kind:"file";path:string;fact:SourceFileFactV1|SourceReadFailureV1}>
 | Readonly<{kind:"analysis";path:string;fact:SourceAnalysisFactV1}>
 | Readonly<{kind:"exists"|"fileExists"|"directoryExists";path:string;fact:boolean}>
 | Readonly<{kind:"stamp";path:string;fact:string}>
 | Readonly<{kind:"field";path:string;fact:Readonly<{format:string;pointer:string;digest:string;paths?:readonly string[]}>}>
 | Readonly<{kind:"stat"|"lstat"|"readdir"|"realpath"|"realpathNative"|"resolve"|"readdirNames"|"directories"|"typescriptText"|"typescriptRealpath"|"registry";path:string;fact:string}>;
/** 🎟️ Names the immutable parser handle; structural copies remain refused by the runtime registry. */
export type SourceAnalysisProviderV1 = Readonly<{id:string;revision:string}>;
/** 📸️ Captures the actual result and every original Source and load-authority observation. */
export type CapturedSourceInputsV1<T> = Readonly<{value:T;observations:Iterable<SourceObservationV1>&{readonly length:number};authorityObservations:readonly SourceObservationV1[]}>;
/** 🧩️ Supplies only neutral compiler filesystem methods; external compiler types are not required. */
export interface SourceTypeScriptSystemV1 {
 fileExists(path:string):boolean;
 directoryExists(path:string):boolean;
 readFile(path:string,encoding?:string):string|undefined;
 getDirectories(path:string):string[];
 realpath?:((path:string)=>string)|undefined;
}
/** 📂️ Keeps system filesystem overloads while recording their original results. */
export const observedFs:Readonly<Pick<typeof fs,"readFileSync"|"existsSync"|"statSync"|"lstatSync"|"readdirSync"|"realpathSync"> & {readFile:typeof readFile}>;
/** 🔏️ Registers only the original parser function for one explicit identity and revision. */
export function registerSourceAnalysisV1(declaration:Readonly<{id:string;revision:string;analyze:(path:string,bytes:Buffer)=>unknown}>):SourceAnalysisProviderV1;
/** 🔬️ Returns an admitted JSON image; consumers must validate its defining parser result. */
export function observedSourceAnalysisV1(provider:SourceAnalysisProviderV1,path:fs.PathLike):unknown;
/** 👥️ Returns two ordered parser results from one source image, preserving each provider's original facts. */
export function observedSourceAnalysisPairV1(first:SourceAnalysisProviderV1,second:SourceAnalysisProviderV1,path:fs.PathLike,options?:Readonly<{recordSource?:boolean}>):readonly [unknown,unknown];
/** 🌊️ Retains original parser and nested Source observations through asynchronous input. */
export function observedSourceAnalysisAsyncV1(provider:SourceAnalysisProviderV1,path:fs.PathLike):Promise<unknown>;
/** 🧿️ Seals and fingerprints the original module-load authority without issuing a grant. */
export function inferenceAuthorityDigestV1():string;
/** 🪪️ The identity every persisted project record of this process is written under. */
export function inferenceRecordIdentityV1():string;
/** 🫱️ Seals the module-load authority of this process and answers its rows. */
export function inferenceAuthorityRowsV1():readonly {readonly kind:string;readonly path:string;readonly fact:unknown}[];
/** 🎁️ Serves under a parent's sealed authority unless an own load row conflicts; answers the conflicts (`kind:path`), empty when adopted. */
export function adoptInferenceAuthorityV1(rows:readonly {readonly kind:string;readonly path:string;readonly fact:unknown}[]):string[];
/** 🧊️ Registers a block a farm worker validated and the caller verified by its hash. */
export function adoptVerifiedBlockV1(path:string,text:string):void;
/** 📦️ Admits one physical project's awaited JSON computation and complete observations. */
export function observeProjectInferenceV1<T>(options:Readonly<{cachePath:string;key:unknown;owner:string;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>;stamps?:readonly string[];prepare?:()=>boolean|PromiseLike<boolean>;fleet?:string;verdict?:(asked:{readonly headerDigest:string})=>PromiseLike<{readonly verdict?:string;readonly headerDigest?:string}|undefined>;verdictFinal?:boolean;prepareAttempts?:number;stale?:(row:Readonly<{kind:string;path:string;fact:string;recorded?:string;current?:string;why?:string}>)=>void}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 🤝️ The verdict of one process on one stored project record: `holds`, `stale`, `other` (a record of another key) or `missing`, with the digest of the header line it judged. */
export function verifyStoredProjectRecordV1(options:Readonly<{cachePath:string;key:unknown}>):Promise<{readonly verdict:"holds"|"stale"|"other"|"missing";readonly headerDigest?:string;readonly rows?:number;readonly staleRow?:Readonly<{kind:string;path:string;fact:string;why?:string}>;readonly verifier:number}>;
/** 🔖️ Registers the declared stamps of the inference protocol as `[path, stamp]` pairs and the path fragments they cover, which the module-load authority names by stamp instead of by bytes. */
export function registerInferenceStampsV1(entries:Iterable<readonly [string,string]>,fragments?:readonly string[]):void;
/** 📌️ Records the stamp of each named inferrer in the active inference scope. */
export function observedInferenceStampsV1(paths:readonly string[]):void;
/** 🧵️ Replays complete original observations into every receiving synchronous scope. */
export function memoizedObservedComputationV1<T>(cache:Map<unknown,unknown>,key:unknown,compute:()=>T):T;
/** 🧪️ Replays original facts while checking the receiving caller's continuation. */
export function memoizedObservedAsyncComputationV1<T>(cache:Map<unknown,unknown>,key:unknown,compute:()=>Promise<T>,checkpoint:()=>Promise<void>):Promise<T>;
/** 🕸️ Preserves every Source fact even when a computation fails. */
export function captureObservedSourceInputsV1<T>(compute:()=>T|PromiseLike<T>,onFailure?:((observations:SourceObservationV1[],authorityObservations:SourceObservationV1[])=>void)|undefined,includeAuthority?:boolean,spill?:undefined):Promise<CapturedSourceInputsV1<Awaited<T>>&{readonly observations:readonly SourceObservationV1[]}>;
export function captureObservedSourceInputsV1<T,List extends Iterable<unknown>&{readonly length:number}>(compute:()=>T|PromiseLike<T>,onFailure:((observations:SourceObservationV1[],authorityObservations:SourceObservationV1[])=>void)|undefined,includeAuthority:boolean|undefined,spill:{builder():{add(rows:Iterable<SourceObservationV1>):void;finish():List}}):Promise<Readonly<{value:Awaited<T>;observations:List;authorityObservations:readonly SourceObservationV1[]}>>;
/** ✍️ Records actual bounded bytes without accepting a caller-provided digest or file observation. */
export function observedExactSourceFileV1(batch:ExactSourceReadBatchV1,root:string,name:string):Promise<ExactSourceFileV1|null>;
/** 📍️ Records the actual system module resolver result or failure. */
export function observedRequireResolve(from:string|URL,request:string):string;
/** 🧭️ The spelling in which a module resolution is recorded: the dependency generation id is a placeholder. */
export function canonicalResolutionV1(path:string):string;
type ProjectObservationRowV1={kind:string;path:string;fact:unknown};
type ClosureVerificationFailureV1=(row:ProjectObservationRowV1|undefined,why?:string)=>void;
/** 🔗️ The seam of the shared command closure entries: how the answer of a `closure` fact is read now and how the entries named by facts are brought up to date. */
export function registerClosureProviderV1(provider:{answer(path:string):string;settle(paths:readonly string[],value?:unknown):Promise<void>;verify?(rows:readonly ProjectObservationRowV1[],value?:unknown,failed?:ClosureVerificationFailureV1):Promise<boolean>}):void;
/** 🏁️ Whether the rows one inference observed still hold when it ends (closure rows are verified by the provider). */
export function projectRowsHoldV1(rows:readonly ProjectObservationRowV1[],value:unknown,holdsRest:(rest:readonly ProjectObservationRowV1[])=>boolean,closureFailed?:(row:ProjectObservationRowV1|undefined,why:string|undefined,count:number)=>void):Promise<boolean>;
export function projectRowsHoldAtEndV1(rows:readonly {kind:string;path:string;fact:unknown}[],value:unknown,nested:boolean,parent:unknown):Promise<boolean>;
/** 🚚️ Brings the closure entries named by the `closure` facts among `rows` up to date. */
export function settleClosureRowsV1(rows:readonly {kind:string;path:string}[],value?:unknown):Promise<void>;
/** 📝️ Records the answer one closure entry gave to the running inference. */
export function recordClosureFactV1(path:string,answer:string):void;
/** 🖊️ Records the registry row (root and manifest) one inference read for one owner name, or its absence. */
/** 🕊️ Drops every in-process cache a graph construction filled; safe only when no inference runs. */
export function releaseInferenceMemoryV1():void;
/** 📏️ The entry counts of the in-process caches a graph construction fills. */
export function observationCollectionsV1():{name:string;entries:number;sample?:()=>unknown[];bytes?:number}[];
export function inferenceCacheSizesV1():{interned:number;rowStore:{rows:number;strings:number;characters:number;bytes:number};analysisImages:number;analysisBytes:number;settledReadBytes:number;fileRecordImageBytes:number;verifiedBlockRows:number;settledReads:number;projectImages:number;publications:number;registryFiles:number};
/** 🗄️ Uses a machine store of source analyses (a directory shared by the processes of the machine), or none. */
export function useSourceAnalysisStoreV1(store:{root:string;directory:string}|undefined):void;
/** 🌐️ Registers the answer of one global fact a record may depend on. */
export function registerGlobalFactV1(name:string,holds:(workspaceRoot:string,segments:any,fact:unknown)=>boolean,prepare?:(rows:readonly {kind:string;path:string;fact?:unknown}[])=>Promise<void>):void;
/** 🛎️ Gets the global fact resolvers with asynchronous preparation ready for the `global` rows among `rows`. */
export function prepareGlobalFactsV1(rows:readonly {kind:string;path:string;fact?:unknown}[]):Promise<void>;
/** ✍️ Records what the running inference read of one global fact. */
export function observeGlobalFactV1(name:string,workspaceRoot:string,segments:readonly string[],fact:unknown):void;
/** 🔭️ The collection of rows the running inference records into. */
export function observationScopeV1():object|undefined;
export function recordRegistryOwnerV1(workspaceRoot:string,name:string,row:{root:string;manifest:string}|undefined|null):void;
/** ❓️ Whether recorded rows still hold, sharing the checks of the running inference round. */
export function observationRowsHoldV1(rows:readonly unknown[]):boolean;
/** 🤔️ Whether the shared rows of one reused computation still hold, checked once per validation round. */
export function observationLayerHoldsV1(layer:readonly unknown[]):boolean;
/** ⏱️ Where the lookups of stored project records spent their milliseconds in this process. */
export function observationLookupClockV1():Record<string,number>;
/** 🧪️ Whether one stored row still holds, probed against the tree now and remembered nowhere. */
export function observationRowFreshV1(row:unknown):boolean;
/** 🪪️ The stamps of the few files and directories whose state decides a set of rows that all hold now, `typed` rows (existence and type of a name, probed by name), `live` rows probed again by every verification, and the parser revisions it was taken under; undefined while it cannot be taken. */
export type ObservationWitnessV1 = Readonly<{paths:readonly string[];tuples:readonly string[];typed:readonly (readonly [string,string,unknown])[];code:readonly (readonly [string,string])[];live:readonly string[]}>;
export function observationWitnessV1(rows:Iterable<unknown>,options?:{since?:number}):ObservationWitnessV1|undefined;
export function observationWitnessHoldsV1(witness:ObservationWitnessV1,options?:{tuples?:Map<string,string>;verdicts?:Map<string,boolean>;step?:()=>Promise<void>}):Promise<boolean>;
/** 🪪️ The same verdict without turns, for a synchronous judge. */
export function observationWitnessHoldsSyncV1(witness:ObservationWitnessV1,options?:{tuples?:Map<string,string>;verdicts?:Map<string,boolean>}):boolean;
/** 🏃️ The running validation round, if any. */
export function observationRoundV1():object|undefined;
export function forgetRoundReadsV1():void;
/** 🔂️ Runs `compute` inside one validation round (the open one when there is one), so stored facts and memoized computations are validated once per round. */
export function withObservationRoundV1<T>(compute:()=>T):T;
/** ▶️ Runs a computation with its observations detached from the running inference; returns its rows and the shared row arrays it reused. */
export function observeDetachedV1<T>(compute:()=>Promise<T>):Promise<{value:T;rows:unknown[];layers:(readonly unknown[])[]}>;
/** 📚️ The facts a detached computation read before it failed, or undefined for an error that carries none. */
export function detachedFailureV1(error:unknown):{rows:unknown[];layers:(readonly unknown[])[]}|undefined;
/** 🧲️ Records the facts of a failed detached computation into the running observation. */
export function retainDetachedFailureV1(error:unknown):void;
/** 🧹️ Removes the observation blocks of one record directory that no record header names any more. */
export function sweepSupersededObservationBlocksV1(directory:string,options?:{headerPattern?:RegExp;graceMilliseconds?:number;now?:number;dryRun?:boolean;minimumBlocks?:number;blocksPerHeader?:number;signal?:AbortSignal;progress?:(row:{phase:string;done:number;total:number})=>void|Promise<void>}):Promise<{headers:number;live:number;blocks:number;removed:number;bytes:number;kept:number;skipped?:true}>;
/** ↩️ Returns the original neutral compiler system with observed filesystem ports. */
export function observedTypeScriptHost<T extends SourceTypeScriptSystemV1>(sys:T):T;
/** 🧳️ Memoizes only the current JSON image while retaining its original input identity. */
export function memoizedRecord<T>(namespace:string,inputs:readonly(string|Uint8Array)[],compute:()=>T):T;
/** 📖️ Parses one observed physical file through its original byte image. */
export function memoizedFileRecord<T>(namespace:string,path:fs.PathLike,compute:(bytes:Buffer)=>T):T;
/** 🪶️ Reads one observed file asynchronously without changing its parser or byte identity. */
export function memoizedFileRecordAsyncV1<T>(namespace:string,path:fs.PathLike,compute:(bytes:Buffer)=>T):Promise<Awaited<T>>;
/** 🪨️ Reports a current inference round through its original caller callback. */
export type SourceSettlementRoundV1 = Readonly<{round:number;settled?:true;conflict?:string;paths?:readonly string[];failure?:unknown}>;
/** ⏳️ Settles mutually consistent original facts under the existing signal and finite round limit. */
export function settleObservedComputationV1<T>(compute:()=>T|PromiseLike<T>,options?:Readonly<{signal?:AbortSignal;limit?:number;onRound?:(round:SourceSettlementRoundV1)=>void|Promise<void>}>):Promise<Awaited<T>>;
/** 🗃️ Reuses an inference result only while its complete current facts hold. */
export function observeInference<T>(options:Readonly<{cachePath:string;key:unknown;verdictFinal?:boolean;prepare?:()=>boolean|PromiseLike<boolean>;prepareAttempts?:number;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>;verdict?:(asked:{readonly headerDigest:string})=>PromiseLike<{readonly verdict?:string;readonly headerDigest?:string}|undefined>}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 🪢️ Publishes dependencies only with complete original inference and consumed facts. */
export function observeDependencies<T>(options:Readonly<{receiptPath:string;recordDirectory?:string}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 🎛️ Receives the original preparation control without constructing operation authority. */
export type SourcePreparationControlV1 = Pick<ResourceLeaseControl,"check"|"advance">;
/** 📥️ Captures and rechecks original preparation facts under the same control. */
export function captureSourcePreparationV1<T>(compute:()=>T|PromiseLike<T>,control:SourcePreparationControlV1):Promise<Readonly<{value:Awaited<T>;original:readonly SourceObservationV1[];received:readonly SourceObservationV1[]}>>;
/** 🔐️ Replays each original Source preparation fact through its unchanged receiving control. */
export function receiveSourcePreparationV1(original:readonly SourceObservationV1[],control:SourcePreparationControlV1):Promise<readonly SourceObservationV1[]>;
/** 📜️ Reads the exact persisted header without consuming its result image. */
export function inferenceHeaderLine(path:string):string|null;
/** 🖋️ A persisted record header with its fact blocks expanded. */
export function readInferenceHeaderV1(path:string):{digest:string;resultSha256:string;authorityObservations:readonly SourceObservationV1[];blocks:readonly string[];observations:readonly SourceObservationV1[]};
/** ✅️ Rechecks untrusted captured facts without admitting a provider or renewing Source authority. */
export function observationsHold(observations:readonly unknown[],limit?:number,stableDirectories?:Set<string>,symlinks?:Set<string>|true,signal?:AbortSignal,onStale?:((row:SourceObservationV1)=>void)|undefined,onVerified?:(rows:number)=>void,exhaustive?:boolean):Promise<boolean>;
export function observationsHoldWithinRoundV1(observations:readonly unknown[],limit:number,signal?:AbortSignal,onStale?:(row:SourceObservationV1)=>void,onVerified?:(rows:number)=>void):Promise<boolean>;
/** 📊️ Exposes owned computation counts without affecting admission. */
export function projectInferenceCountsV1():Record<string,number>;
/** 🌐️ The facts one project record consumed, whether it was computed or reused in this process. */
export function projectInferenceObservationsV1(cachePath:string):readonly SourceObservationV1[]|undefined;
/** 🚀️ Runs a partial inference in project scope without an all-or-nothing round. */
export function observeInferenceSnapshotV1<T>(compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 📈️ Reports original Source and memo bookkeeping without exposing or discarding facts. */
export function sourceObservationStatsV1():Readonly<{loadObservations:number;projectImages:number;projectImageBytes:number;originalInferencePublications:number;projectObservationRows:number;captureObservations:number;recordStores:number;analysisEntries:number;analysisBytes:number}>;

export function requireCurrentObservationV1():void;
export function registerLoadedImplementationRowsV1(rows:readonly {kind:string;path:string;fact:{sha256:string;ignore?:unknown}}[]):void;
export function finishOnLoadedImplementationV1():void;
export function loadedImplementationRecordedFactV1(kind:string,path:string,fact:unknown):unknown;
export function requirePinnedImplementationFactsV1(observations:readonly SourceObservationV1[],signal?:AbortSignal):void;
export function staleSourceFactsErrorV1(rows:readonly {kind:string;path:string}[]):Error;

/** 🪡️ Projects finite aggregate observation budget admission DATA. */
export function observationBlockBudgetAdmitsV1(used:number,next:number,maximum:number):boolean;
/** 🧱️ Names the canonical file of one persisted observation block below an inference directory. */
export function inferenceObservationBlockPathV1(directory:string,digest:string):string;

export function withObservedWorldV1<T>(port: Pick<typeof import("node:fs"), "statSync"|"lstatSync"|"readFileSync"|"readdirSync"|"realpathSync"|"existsSync">, body: () => T): T;
export function importObservedModuleV1(path: string): Promise<unknown> | undefined;

/** 🧩️ Reads one field of a data file and records only the digest of the extracted value. */
export function observedFieldV1(path:string,format:string,pointer:string,options?:Readonly<{paths?:readonly string[]}>):unknown;
/** 🔌️ Registers one more field extraction format. */
export function registerFieldExtractorV1(format:string,extract:(text:string,pointer:string,path:string,readOther:(path:string)=>string)=>unknown):void;
/** 📍️ The value at an RFC 6901 JSON pointer, `undefined` when a step is absent. */
export function jsonPointerValueV1(document:unknown,pointer:string):unknown;
/** 🧲️ Merges stored row arrays into the running observation scope by reference. */
export function retainObservedRowsV1(rows:readonly unknown[]):void;
/** 🪢️ Makes the rows of a stored or detached closure one live instance per distinct row, in place, and answers the same array. */
export function shareObservationRowsV1<Row>(rows:Row[]):Row[];
/** 🫧️ Runs a synchronous computation with its observations returned, not recorded into the running scope. */
export function observeDetachedSyncV1<T>(compute:()=>T):Readonly<{value:T;rows:readonly unknown[];layers:readonly (readonly unknown[])[]}>;
/** 🔁️ Runs a computation inside a caller-owned validation round. */
export function withObservationPassRoundV1<T>(round:Map<unknown,unknown>,compute:()=>T):T;

/** 🔢️ How many distinct observation rows the running inference recording at `cachePath` holds. */
export function observedInferenceRowsV1(cachePath:string):number;
/** 🧹️ Forgets the row counter of a finished inference. */
export function forgetObservedInferenceRowsV1(cachePath:string):void;

/** 🫧️ The verdict on the shared rows of one reused computation, row by row with `step` awaited between rows. */
export function observationLayerHoldsAsyncV1(layer:readonly unknown[],step:()=>Promise<void>):Promise<boolean>;
/** 🫧️ Whether every recorded fact still holds, row by row with `step` awaited between rows. */
export function observationRowsHoldAsyncV1(rows:readonly unknown[],step:()=>Promise<void>):Promise<boolean>;

/** 🗂️ Declares the absolute directories whose entry sets are proven unchanged, so recorded listings and absences in them hold without a probe. */
export function useStableDirectoriesV1(directories:ReadonlySet<string>):void;
/** 🔢️ How many directory listings this process made. */
export function observationProbeCountsV1():{readdir:number};
export function observationCarriedCountsV1():{reused:number;probed:number};
/** 📊️ How often the memo of directory facts answered by stamps, had to list again, remembered a fact or refused one, and how many facts it holds. */
export function observationDirectoryFactCountsV1():{hits:number;misses:number;recorded:number;refused:number;entries:number};

/** 🌐️ The port through which project inference records travel through the fleet cache; `undefined` keeps them local. */
/** 🚚️ How the closure entries of a project record travel with it. */
export type ProjectClosureTravelV1 = Readonly<{ export(root: string, image: string): Promise<readonly Readonly<{ recipeKey: string; bundle: unknown }>[]>; import(root: string, bundles: readonly Readonly<{ recipeKey: string; bundle: unknown }>[]): Promise<number> }>;
export function registerProjectClosureTravelV1(travel: ProjectClosureTravelV1): void;
export function useProjectFleetV1(port:undefined|Readonly<{gate(info:unknown):Promise<unknown>;offer(info:unknown):void}>):void;
/** 🔑️ The key text a project record is filed under in the fleet. */
export function projectRecordKeyTextV1(key:unknown):string;
/** 🚚️ The closure travel registered by the closure owner, or undefined. */
export function projectClosureTravelV1():ProjectClosureTravelV1|undefined;
/** 📤️ The record at `cachePath` in portable form or undefined. */
export function exportProjectRecordV1(cachePath:string):undefined|Readonly<{digest:string;authority:readonly unknown[];observations:readonly unknown[];image:string}>;
/** 📥️ Writes a portable record as the local record at `cachePath`. */
export function importProjectRecordV1(cachePath:string,portable:unknown):Promise<boolean>;
/** 🧾️ The JSON text of one observation row, serialised once per row object. */
export function observationRowJsonV1(row:unknown):string;
/** 🧹️ Drops what the inference record at `cachePath` still retains in memory. */
export function releaseProjectInferenceRetentionV1(cachePath:string):void;
export function settleObservationLeafV1<T>(attempt:()=>T,options?:{attempts?:number}):T;
export function readInferenceRecordOrMissV1(cachePath:string,options?:{read?:(path:string,optional:boolean)=>unknown;say?:(line:string)=>void}):any;
/** 🔦️ The directory fact of `path` for one of the kinds that read a directory (`readdir`, `readdirNames`, `directories`), as a record row would hold it. */
export function directoryFactOfV1(kind:"readdir"|"readdirNames"|"directories",path:string):string;
/** 🔎️ The first row of a stale record that does not hold now. */
export function firstFailingRowV1(rows:readonly unknown[],value:unknown):Promise<Readonly<{kind:string;path:string;fact:string;why?:string}>|undefined>;
