/** 🏚️ Teaches this process that facts recorded under `former` are about the tree now at `live`. */
export function noteFormerRootV1(former:string,live:string):boolean;
/** 🏡️ Registers the live root of a workspace this process serves. */
export function noteLiveRootV1(root:string):void;
/** 🧹️ Forgets every taught root. */
export const forgetFormerRootsV1:()=>void;
/** 🗺️ The roots taught so far as `[former, live]` pairs. */
export const formerRootsV1:()=>readonly(readonly[string,string])[];
/** ✏️ The text with every taught former root replaced by the live one. */
export function rebaseTextV1(text:string):string;
/** 🔁️ The value with the former roots replaced in every string. */
export function rebaseValueV1<T>(value:T):T;
/** 🧾️ One recorded row as it reads in the live tree. */
export function rebaseRowV1<T extends{path:string;fact:unknown}>(row:T):T;
/** 📚️ The rows as they read in the live tree. */
export function rebaseRowsV1<T extends{path:string;fact:unknown}>(rows:ArrayLike<T>&Iterable<T>):ArrayLike<T>&Iterable<T>;
/** 🔎️ The workspace roots a set of recorded rows names. */
export function recordedRootsV1(rows:ArrayLike<{kind:string;path:string}>):string[];
/** 🔎️ The same roots read from a lazily opened fact index. */
export function recordedRootsOfIndexV1(index:{rows:ArrayLike<{kind:string;path:string}>}):string[];
/** 🏡️ Teaches the process every root a fact index was recorded under that is not `live`. */
export function adoptRecordedRootsV1(index:{rows:ArrayLike<{kind:string;path:string}>},live:string|undefined):boolean;
