import type {GenerationRowV1} from "../🟨️.mjs";
import type {DirectoryStampsV1} from "../🗂️directory-stamps/🟨️.mjs";
/** 🔎️ Decides which fact rows need the precise check from the directory stamps and the file stamps of one verify epoch. */
export interface EpochDoubtV1{
 classify(rows:readonly unknown[],options?:{signal?:AbortSignal;progress?:(event:unknown)=>Promise<void>}):Promise<Uint8Array>;
 settled(input:{rows:readonly unknown[];ids:readonly number[];stale:ReadonlySet<number>}):Promise<boolean>;
 published(input:{rows:readonly unknown[];index?:unknown;observedAt:number;signal?:AbortSignal}):Promise<boolean>;
 stats():{directories:number;files:number};
}
export function createEpochDoubtV1(options:{stamps:DirectoryStampsV1;statFile?:(path:string)=>Promise<unknown>;concurrency?:number}):EpochDoubtV1;

/** 📂️ The directory and base name of the file a row found. */
export type DoubtEntryV1=Readonly<{directory:string;name:string}>;
/** 🧱️ The directory and child name of a row that says an entry exists (`exists`, `fileExists`, `directoryExists` true, `stat` of a type): its answer cannot change while the directory keeps its stamp and the entry is no symbolic link (a plain entry only changes by unlink and create, which move the directory's times); undefined for every other row. */
export function existingEntryV1(row: GenerationRowV1): DoubtEntryV1|undefined;
/** 🧭️ Every directory and content path a row and its nested observations depend on. */
export function rowDependenciesV1(row: GenerationRowV1, directories: Set<string>, paths?: Set<string>): void;
/** 🔢️ The doubt classifier `selectDoubtfulRowsV1` asks: by rows, or by ids of a lazily opened index. */
export interface DoubtClassifierV1{classify(rows:readonly GenerationRowV1[],options?:{signal?:AbortSignal;progress?:(event:Readonly<{phase:string;done:number;total:number}>)=>Promise<void>|void}):Promise<ArrayLike<number|boolean>>;classifyIds?(index:DoubtIndexV1,ids:readonly number[],options?:{signal?:AbortSignal;progress?:(event:Readonly<{phase:string;done:number;total:number}>)=>Promise<void>|void}):Promise<ArrayLike<number|boolean>>}
/** 🗂️ The row table a doubt selection reads. */
export interface DoubtIndexV1{readonly rows:readonly GenerationRowV1[];readonly rowPathIdOf?:(id:number)=>number;readonly textOf?:(id:number)=>string}
/** 🔢️ Keeps the ids whose row is doubtful: `doubt.classify(rows)` answers 1 for a row that must be checked, 0 for one a journal or a stamp vouches for; `doubt.classifyIds(index,ids)` answers the same from the kind and the path string of a lazily opened index, so no row and no fact is decoded for a row the journal vouches for. Cancellable, reports `{phase:"doubt",done,total}`. */
export function selectDoubtfulRowsV1(index: DoubtIndexV1, ids: readonly number[], doubt: DoubtClassifierV1, options: Readonly<{signal?:AbortSignal;progress?:(event:Readonly<{phase:string;done:number;total:number}>)=>Promise<void>|void;chunk:number}>): Promise<number[]>;
/** 🧭️ The directory and base name a relative or absolute module resolution found its file in: the probes of Node's file resolution (the name, then each extension) are all entries of that one directory, so the answer cannot change while the directory keeps its stamp and none of the probed names (every name that starts with the base name) is a symbolic link; undefined for bare specifiers and directory resolutions (their probe chains are not recorded). */
export function siblingResolutionV1(row: GenerationRowV1): DoubtEntryV1|undefined;
/** 🗂️ The directory whose stamp vouches for a structural row: the listed directory of a listing, the parent of the entry of an `lstat` or of an absence; undefined for every other row (an existing entry may be a symlink whose target moves without touching its directory). */
export function structuralDirectoryV1(row: GenerationRowV1): string|undefined;
