/** 🛌️ What compaction on idle needs from the machine: the clock, the due rule of an image, the stamp of the last compaction, and the in-place reclaim, which takes the exclusive lease of the image with one non-waiting try (so it only runs when this process was the last holder) and ends with the receipt the product prints. */
export type IdleCompactionPortsV1=Readonly<{
 now():number;
 due(partition:string):boolean;
 lastAt(partition:string):number|undefined;
 stamp(partition:string,at:number):void;
 reclaim(partition:string,signal:AbortSignal):Promise<Readonly<{outcome:"reclaimed";beforeBytes:number;afterBytes:number}|{outcome:"skipped";reason:string}>>;
}>;
/** 🛌️ What happened to one image: `compacted` (blocks returned), `not-due`, `too-soon` (the minimum interval since its last compaction has not passed), `busy` (another holder keeps the image, nothing was touched and nobody waited) or `failed`. */
export type IdleCompactionOutcomeV1=Readonly<{partition:string;result:"compacted"|"not-due"|"too-soon"|"busy"|"failed";beforeBytes?:number;afterBytes?:number;message?:string}>;

/** 🧭️ The images worth a try now, in the given order: due by the bloat rule and last compacted at least `minimumIntervalMilliseconds` ago. Pure; the Python oracle of the tests agrees. */
export function planIdleCompactionV1(rows:readonly Readonly<{partition:string;due:boolean;lastAt:number|undefined}>[],now:number,minimumIntervalMilliseconds:number):readonly Readonly<{partition:string;verdict:"try"|"not-due"|"too-soon"}>[]{
 return rows.map(row=>({partition:row.partition,verdict:!row.due?"not-due":row.lastAt!==undefined&&now-row.lastAt<minimumIntervalMilliseconds?"too-soon":"try"}));
}

/** 🛌️ Compacts the images this process was the last holder of: for every image that the plan admits it tries the exclusive lease once and runs the bounded in-place reclaim (detach, attach through the UNMAP-capable stack, settle); an opener that arrives meanwhile waits on the existing lease and never fails, a holder that is still there makes the try end `busy` at once. The whole pass stops at `maximumMilliseconds` or when the caller cancels; only a completed cycle stamps the image, so a busy image is tried again at the next release. @see ../🚰️reclaim/🟦️.ts */
export async function compactOnIdleV1(input:Readonly<{partitions:readonly string[];ports:IdleCompactionPortsV1;minimumIntervalMilliseconds:number;maximumMilliseconds:number;signal?:AbortSignal}>):Promise<readonly IdleCompactionOutcomeV1[]>{
 const{ports}=input,signal=input.signal===undefined?AbortSignal.timeout(input.maximumMilliseconds):AbortSignal.any([input.signal,AbortSignal.timeout(input.maximumMilliseconds)]),out:IdleCompactionOutcomeV1[]=[];
 const plan=planIdleCompactionV1(input.partitions.map(partition=>({partition,due:ports.due(partition),lastAt:ports.lastAt(partition)})),ports.now(),input.minimumIntervalMilliseconds);
 for(const row of plan){
  if(row.verdict!=="try"){out.push({partition:row.partition,result:row.verdict});continue;}
  if(signal.aborted){out.push({partition:row.partition,result:"busy",message:"pass bound reached"});continue;}
  try{
   const receipt=await ports.reclaim(row.partition,signal);
   if(receipt.outcome==="reclaimed"){ports.stamp(row.partition,ports.now());out.push({partition:row.partition,result:"compacted",beforeBytes:receipt.beforeBytes,afterBytes:receipt.afterBytes});}
   else out.push({partition:row.partition,result:receipt.reason==="lease-timeout"||receipt.reason==="not-due"||receipt.reason==="retirement-pending"?"busy":"failed",message:receipt.reason});
  }catch(error){out.push({partition:row.partition,result:"failed",message:String((error as Error)?.message??error).slice(0,200)});}
 }
 return Object.freeze(out);
}
