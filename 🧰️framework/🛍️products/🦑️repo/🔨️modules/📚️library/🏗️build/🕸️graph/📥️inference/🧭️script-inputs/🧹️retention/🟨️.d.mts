/** 🧹️ What one collection of a closure store directory removed and kept, by kind; `cancelled` alone when the collection stopped early. */
export interface ClosureStoreReportV1{readonly cancelled:boolean;readonly dryRun?:boolean;readonly kept?:{readonly entries:number;readonly layers:number;readonly values:number};readonly removed?:{readonly entries:number;readonly layers:number;readonly values:number};readonly bytes?:number}
/** 🧹️ Collects one closure store directory (`e`, `l`, `v`) under the retention plan; the entries, layers and values it removes are counted, and removed only unless `dryRun`.
 * @see ./🟨️.mjs */
export function collectClosureStoreV1(options:{readonly directory:string;readonly graceMilliseconds:number;readonly now?:number;readonly dryRun?:boolean;readonly cancelled?:()=>boolean}):Promise<ClosureStoreReportV1>;

/** 🧮️ One closure store entry the retention rule reads. */
export interface ClosureRetentionRowV1{readonly name:string;readonly mtimeMs:number;readonly record?:Readonly<{recipe?:Readonly<Record<string,unknown>>;layers:readonly string[];valueDigest:string}>|null}
/** 🪢️ The slot of a closure recipe: the identity of what it answers (function, project, command and working directory, role, name, view, port, entry files) without the settings and inputs it was derived from, so a newer entry of a slot supersedes the older ones. @see ../💾️entries/🟨️.mjs */
export function closureSlotOfV1(recipe: Readonly<Record<string,unknown>>|undefined): string;
/** 🧮️ The pure retention rule: per slot the newest entry (ties: the larger name) is current; an older entry is superseded and goes once the current one is at least `grace` old; a damaged entry goes once it is `grace` old; a layer or value that no kept entry names goes once it was neither written nor reused for `grace`. */
export function planClosureRetentionV1(input: Readonly<{entries:readonly ClosureRetentionRowV1[];layers:readonly Readonly<{name:string;mtimeMs:number}>[];values:readonly Readonly<{name:string;mtimeMs:number}>[];now:number;grace:number}>): Readonly<{entries:string[];layers:string[];values:string[]}>;
