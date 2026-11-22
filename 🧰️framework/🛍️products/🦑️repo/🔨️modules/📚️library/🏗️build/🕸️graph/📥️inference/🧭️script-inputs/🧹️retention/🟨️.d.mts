/** 🧹️ What one collection of a closure store directory removed and kept, by kind; `cancelled` alone when the collection stopped early. */
export interface ClosureStoreReportV1{readonly cancelled:boolean;readonly dryRun?:boolean;readonly kept?:{readonly entries:number;readonly layers:number;readonly values:number};readonly removed?:{readonly entries:number;readonly layers:number;readonly values:number};readonly bytes?:number}
/** 🧹️ Collects one closure store directory (`e`, `l`, `v`) under the retention plan; the entries, layers and values it removes are counted, and removed only unless `dryRun`.
 * @see ./🟨️.mjs */
export function collectClosureStoreV1(options:{readonly directory:string;readonly graceMilliseconds:number;readonly now?:number;readonly dryRun?:boolean;readonly cancelled?:()=>boolean}):Promise<ClosureStoreReportV1>;
