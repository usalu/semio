import {bindCatalogV1,handleTextV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorV1,type RepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {formatDiskBytesV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/💽️disk/🟦️.ts";
import {casPartitionBudgetV1,type CasPartitionBudgetV1,type CasPartitionV1} from "../../🗄️cas/🌡️partition/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📚️ The tuning of the recovery: the time one collector may take, how many consumers a refusal names, the entry budget of one size measurement, the environment variable that marks a child of the recovery and the consumers a refusal always names when they hold bytes (the image slack and the replica store). */
export type DiskRecoveryPolicyV1=Readonly<{version:1;collectorBudgetMilliseconds:number;consumerBudgetMilliseconds:number;consumerCount:number;measureEntryLimit:number;environmentChild:string;namedConsumers:readonly string[]}>;
export const DISK_RECOVERY_POLICY_V1:DiskRecoveryPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)));
/** 🗣️ The bilingual catalog of the recovery; its refusal renders at the command line like every other refusal. */
export const DISK_RECOVERY_TEXTS_V1=bindCatalogV1("disk-recovery",schema.definitions.Messages.const);

/** 🌐️ The language of the lines a recovery reports: one of the two, or `both` for a log that is read by whoever opens it (English, then German, joined by a slash). */
export type DiskLanguageV1="en"|"de"|"both";
/** 🗣️ One line of the recovery catalog in the language asked for. */
export function diskTextV1(key:string,language:DiskLanguageV1,values:Record<string,string|number>={}):string{
 return language==="both"?handleTextV1(DISK_RECOVERY_TEXTS_V1,key,"en",values)+" / "+handleTextV1(DISK_RECOVERY_TEXTS_V1,key,"de",values):handleTextV1(DISK_RECOVERY_TEXTS_V1,key,language,values);
}

/** 🗂️ One registered writer: who fills which root, how it is bounded, who collects it and in which order the gate frees it (`priority` ascending), whether freeing it returns bytes to the host volume at once (`relief`), and the exact command that frees it by hand. */
export type DiskWriterRowV1=Readonly<{id:string;priority:number;inodeRank?:number;class:"derivable"|"recoverable"|"content"|"store"|"image";relief:"host"|"image"|"none";root:string;collector:"ledger"|"cas-lru"|"staging-sweep"|"image-sweep"|"age-prune"|"log-rotate"|"native-shards"|"image-reset"|"store-compaction"|"manual";command:string;bound:Readonly<{en:string;de:string}>}>;
export const DISK_WRITERS_V1:readonly DiskWriterRowV1[]=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Writers.const)));

/** 🔌️ A registered writer bound to its collector: what it holds now and how to free it; `collect` answers the bytes it claims to have freed (the recovery trusts the volume, not the claim). */
export type DiskWriterV1=Readonly<{row:DiskWriterRowV1;root:string;measure(signal?:AbortSignal):Promise<number>|number;describe?(signal?:AbortSignal):Promise<Readonly<{root?:string;command?:string}>|undefined>;collect?(request:Readonly<{needBytes:number;signal?:AbortSignal}>):Promise<number>}>;

/** 🌡️ What the volume says: its mount, size and the bytes a writer may still use; `undefined` is "not measurable". */
export type DiskFactsV1=Pick<CasPartitionV1,"partition"|"totalBytes"|"freeBytes">;

/** ✅️ Proves the registry: unique ids and priorities, a command for every writer, a collector that can run for every row the gate frees, and the manual rows last. */
export function requireDiskWritersV1(rows:readonly DiskWriterRowV1[]=DISK_WRITERS_V1):readonly DiskWriterRowV1[]{
 const ids=new Set<string>(),priorities=new Set<number>(),ranks=new Set<number>();
 for(const row of rows){
  if(ids.has(row.id)||priorities.has(row.priority))throw Error("Unique disk writer ids and priorities required / Eindeutige Plattenschreiber-Kennungen und -Prioritäten erforderlich: "+row.id);
  ids.add(row.id);priorities.add(row.priority);
  if(row.inodeRank!==undefined){if(ranks.has(row.inodeRank))throw Error("Unique inode ranks required / Eindeutige Inode-Ränge erforderlich: "+row.id);ranks.add(row.inodeRank);}
  if(row.command.trim()===""||row.bound.en.trim()===""||row.bound.de.trim()==="")throw Error("A command and a bound in both languages required / Ein Befehl und eine Grenze in beiden Sprachen erforderlich: "+row.id);
  if(row.collector==="manual"&&row.relief==="none")throw Error("A manual writer must say what it relieves / Ein manueller Schreiber muss sagen, was er entlastet: "+row.id);
 }
 const manual=rows.filter(row=>row.collector==="manual").map(row=>row.priority),automatic=rows.filter(row=>row.collector!=="manual").map(row=>row.priority);
 if(manual.length&&automatic.length&&Math.min(...manual)<Math.max(...automatic))throw Error("Manual writers come after every automatic one / Manuelle Schreiber folgen nach allen automatischen");
 return rows;
}

/** 🧭️ The verdict on one command: it fits with its reserve, it fits but a collection should start (below the reserve), or it must wait for a collection (below its cushion); a volume that cannot derive its budget is judged by the fixed shares of the cache store instead. */
export type DiskJudgementV1=Readonly<{action:"allow"|"collect-background"|"collect-now"|"fixed-shares";mustFreeBytes:number;targetFreeBytes:number;budget:CasPartitionBudgetV1}>;

/** 🚦️ Judges a command that declares `needBytes` against the volume: it must find `need + cushion` free to start, a collection starts while `need + reserve` is not free. Pure and deterministic; the Python oracle of the tests judges the same cases. @see ../🗄️cas/🌡️partition/🟦️.ts */
export function judgeDiskV1(facts:DiskFactsV1|undefined,needBytes:number,budget:CasPartitionBudgetV1|undefined=facts&&casPartitionBudgetV1(facts.totalBytes)):DiskJudgementV1{
 const none:CasPartitionBudgetV1=Object.freeze({derivable:false,cushionBytes:0,reserveBytes:0,hysteresisBytes:0});
 if(facts===undefined||budget===undefined||!budget.derivable)return Object.freeze({action:"fixed-shares",mustFreeBytes:0,targetFreeBytes:0,budget:budget??none});
 const mustFreeBytes=needBytes+budget.cushionBytes,targetFreeBytes=needBytes+budget.reserveBytes;
 return Object.freeze({action:facts.freeBytes<mustFreeBytes?"collect-now":facts.freeBytes<targetFreeBytes?"collect-background":"allow",mustFreeBytes,targetFreeBytes,budget});
}

/** 💰️ The disk budget of the whole machine in bytes, derived from the volume and not from a declared total: what the build system may hold is what it holds now plus what is free, never more than the volume, less the reserve that must stay free (0 on a volume that cannot derive its budget). */
export function machineDiskBudgetV1(input:Readonly<{totalBytes:number;freeBytes:number;footprintBytes:number}>):number{
 const budget=casPartitionBudgetV1(input.totalBytes);
 return budget.derivable?Math.max(0,Math.min(input.totalBytes,input.footprintBytes+input.freeBytes)-budget.reserveBytes):0;
}

/** 🧾️ One consumer a refusal names: the writer, the bytes it holds and the command that frees it. */
export type DiskConsumerV1=Readonly<{id:string;root:string;bytes:number;command:string}>;
/** 📋️ What a recovery did: the verdict, the collectors it ran with the bytes the volume gained, the volume at the end and, for a refusal, the largest remaining consumers. */
export type DiskRecoveryV1=Readonly<{verdict:"admit"|"refuse"|"fixed-shares";facts:DiskFactsV1|undefined;needBytes:number;mustFreeBytes:number;targetFreeBytes:number;steps:readonly Readonly<{id:string;freedBytes:number;freeBytes:number;failure?:string}>[];consumers:readonly DiskConsumerV1[]}>;

/** 🧹️ Frees space in the order of the registry until the command's declared need fits with its reserve: the writers that give bytes to the host volume run by priority (derivable artefacts first, then the least recently used cache objects), the volume is read again after each one and judges, a failing or too slow collector never stops the next, and cancellation stops between collectors. The command is admitted when `need + cushion` is free, and refused with the largest remaining consumers when it is not. @see ./🧬️schema/🔣️.json */
export async function recoverDiskV1(input:Readonly<{facts():DiskFactsV1|undefined;needBytes:number;writers:readonly DiskWriterV1[];language:DiskLanguageV1;report?(line:string):void;signal?:AbortSignal;budgetMilliseconds?:number}>):Promise<DiskRecoveryV1>{
 const text=(key:string,values:Record<string,string|number>)=>diskTextV1(key,input.language,values),write=input.report??(()=>undefined);
 let facts=input.facts(),judged=judgeDiskV1(facts,input.needBytes);
 const steps:Array<{id:string;freedBytes:number;freeBytes:number;failure?:string}>=[];
 const done=(verdict:DiskRecoveryV1["verdict"],consumers:readonly DiskConsumerV1[]=[]):DiskRecoveryV1=>Object.freeze({verdict,facts,needBytes:input.needBytes,mustFreeBytes:judged.mustFreeBytes,targetFreeBytes:judged.targetFreeBytes,steps:Object.freeze(steps.slice()),consumers});
 if(facts===undefined||judged.action==="fixed-shares")return done("fixed-shares");
 if(judged.action==="allow")return done("admit");
 facts=(await collectInOrderV1({facts:input.facts,current:facts,targetFreeBytes:judged.targetFreeBytes,writers:input.writers,language:input.language,report:write,signal:input.signal,budgetMilliseconds:input.budgetMilliseconds,steps}))??facts;
 judged=judgeDiskV1(facts,input.needBytes);
 if(facts.freeBytes>=judged.mustFreeBytes){write(text("recovery.done",{partition:facts.partition,free:formatDiskBytesV1(facts.freeBytes)}));return done("admit");}
 return done("refuse",await consumersOfV1(input.writers,input.signal));
}

/** 🧬️ Whether freeing a writer returns bytes to the host volume at once: its registered relief, except that a writer whose relief is the image gives host bytes directly on a platform whose partitions are plain directories (`imageBacked` false), where no image sits between the cache and the volume. */
export function diskReliefV1(row:Pick<DiskWriterRowV1,"relief">,imageBacked:boolean):DiskWriterRowV1["relief"]{return row.relief==="image"&&!imageBacked?"host":row.relief;}

/** 🪜️ Runs the collectors that give bytes to the host volume in the order of the registry until `targetFreeBytes` are free: the volume is read again after each one (the claim of a collector is not trusted), a failing or too slow collector never stops the next, progress lines go to `report` in the language asked for and cancellation stops between collectors. Answers the last reading of the volume. */
export async function collectInOrderV1(input:Readonly<{facts():DiskFactsV1|undefined;current:DiskFactsV1;targetFreeBytes:number;writers:readonly DiskWriterV1[];language:DiskLanguageV1;report?(line:string):void;signal?:AbortSignal;budgetMilliseconds?:number;startKey?:string;sweep?:boolean;scarce?:"inodes";steps:Array<{id:string;freedBytes:number;freeBytes:number;failure?:string}>}>):Promise<DiskFactsV1>{
 const text=(key:string,values:Record<string,string|number>)=>diskTextV1(key,input.language,values),write=input.report??(()=>undefined);
 let facts=input.current;
 write(text(input.startKey??"recovery.start",{partition:facts.partition,free:formatDiskBytesV1(facts.freeBytes),target:formatDiskBytesV1(input.targetFreeBytes)}));
 const order=input.writers.filter(writer=>(writer.row.relief==="host"||input.sweep===true&&writer.row.relief==="image")&&writer.row.collector!=="manual"&&writer.collect!==undefined).sort((first,second)=>input.scarce==="inodes"?(first.row.inodeRank??Infinity)-(second.row.inodeRank??Infinity)||first.row.priority-second.row.priority:first.row.priority-second.row.priority);
 for(const writer of order){
  input.signal?.throwIfAborted();
  write(text("recovery.collector-start",{id:writer.row.id}));
  input.signal?.throwIfAborted();
  const before=facts.freeBytes;let failure:string|undefined;
  const limit=AbortSignal.timeout(input.budgetMilliseconds??DISK_RECOVERY_POLICY_V1.collectorBudgetMilliseconds),signal=input.signal?AbortSignal.any([input.signal,limit]):limit;
  try{await writer.collect!({needBytes:Math.max(0,input.targetFreeBytes-before),signal});}
  catch(error){if(input.signal?.aborted)throw error;failure=String((error as Error)?.message??error).slice(0,200);}
  const next=input.facts();if(next!==undefined)facts=next;
  const freed=Math.max(0,facts.freeBytes-before);
  input.steps.push(failure===undefined?{id:writer.row.id,freedBytes:freed,freeBytes:facts.freeBytes}:{id:writer.row.id,freedBytes:freed,freeBytes:facts.freeBytes,failure});
  write(failure===undefined?text("recovery.step",{id:writer.row.id,freed:formatDiskBytesV1(freed),free:formatDiskBytesV1(facts.freeBytes)}):text("recovery.failed",{id:writer.row.id,message:failure}));
  if(facts.freeBytes>=input.targetFreeBytes&&input.sweep!==true)break;
 }
 return facts;
}

/** 🧹️ Runs every collector of the registry that has one, the writers that relieve the image included, once and in priority order, whatever the volume says in between: a write that failed with "no space left" inside an image or a quota does not show in the free bytes of the host, so the stop condition of `recoverNeedV1` would never collect. Answers the steps taken, the bytes the host gained and the largest consumers left. */
export async function sweepDiskV1(input:Readonly<{facts():DiskFactsV1|undefined;needBytes:number;scarce?:"inodes";writers:readonly DiskWriterV1[];language:DiskLanguageV1;report?(line:string):void;signal?:AbortSignal;budgetMilliseconds?:number}>):Promise<DiskRecoveryV1>{
 const steps:Array<{id:string;freedBytes:number;freeBytes:number;failure?:string}>=[],facts=input.facts();
 const done=(consumers:readonly DiskConsumerV1[]=[]):DiskRecoveryV1=>Object.freeze({verdict:steps.length?"admit" as const:"refuse" as const,facts:input.facts()??facts,needBytes:input.needBytes,mustFreeBytes:input.needBytes,targetFreeBytes:input.needBytes,steps:Object.freeze(steps.slice()),consumers});
 if(facts===undefined)return done();
 await collectInOrderV1({facts:input.facts,current:facts,targetFreeBytes:facts.freeBytes+input.needBytes,writers:input.writers,language:input.language,report:input.report,signal:input.signal,budgetMilliseconds:input.budgetMilliseconds,startKey:"recovery.start-need",sweep:true,scarce:input.scarce,steps});
 return done(steps.length?[]:await consumersOfV1(input.writers,input.signal));
}

/** 📥️ Makes `needBytes` free on a volume for a stage that waits to start: the collectors of the registry run in priority order (derivable artefacts first, the least recently used cache objects and the user files later, the image slack last) until the volume has the need, with progress lines and cancellation; when nothing collectable remains it answers the largest consumers, each with the exact command that frees it. It judges by the need alone (no cushion or reserve of the derived budget) and works on any volume size. @see ./🧪️tests/🟦️.ts */
export async function recoverNeedV1(input:Readonly<{facts():DiskFactsV1|undefined;needBytes:number;writers:readonly DiskWriterV1[];language:DiskLanguageV1;report?(line:string):void;signal?:AbortSignal;budgetMilliseconds?:number;consumerBudgetMilliseconds?:number}>):Promise<DiskRecoveryV1>{
 const text=(key:string,values:Record<string,string|number>)=>diskTextV1(key,input.language,values),write=input.report??(()=>undefined);
 let facts=input.facts();
 const steps:Array<{id:string;freedBytes:number;freeBytes:number;failure?:string}>=[],done=(verdict:DiskRecoveryV1["verdict"],consumers:readonly DiskConsumerV1[]=[]):DiskRecoveryV1=>Object.freeze({verdict,facts,needBytes:input.needBytes,mustFreeBytes:input.needBytes,targetFreeBytes:input.needBytes,steps:Object.freeze(steps.slice()),consumers});
 if(facts===undefined)return done("fixed-shares");
 if(facts.freeBytes>=input.needBytes)return done("admit");
 facts=await collectInOrderV1({facts:input.facts,current:facts,targetFreeBytes:input.needBytes,writers:input.writers,language:input.language,report:write,signal:input.signal,budgetMilliseconds:input.budgetMilliseconds,startKey:"recovery.start-need",steps});
 if(facts.freeBytes>=input.needBytes){write(text("recovery.done",{partition:facts.partition,free:formatDiskBytesV1(facts.freeBytes)}));return done("admit");}
 const budget=input.consumerBudgetMilliseconds??DISK_RECOVERY_POLICY_V1.consumerBudgetMilliseconds,deadline=AbortSignal.timeout(budget);
 const consumers=await consumersOfV1(input.writers,input.signal,undefined,{deadline,report:id=>write(text("recovery.consumer-measure",{id}))});
 if(deadline.aborted)write(text("recovery.consumer-budget",{budget,free:formatDiskBytesV1(facts.freeBytes),target:formatDiskBytesV1(input.needBytes)}));
 return done("refuse",consumers);
}

/** 🔎️ The largest consumers of the machine by registered writer, biggest first, each with the exact command that frees it (a writer that knows more than its row, the image with the most slack or the store of this machine, describes its own root and command); the consumers named by the policy are listed besides the largest ones whenever they hold bytes; a writer that cannot be measured is skipped, not guessed. */
export async function consumersOfV1(writers:readonly DiskWriterV1[],signal?:AbortSignal,count:number=DISK_RECOVERY_POLICY_V1.consumerCount,options?:Readonly<{deadline:AbortSignal;report?(id:string):void}>):Promise<readonly DiskConsumerV1[]>{
 const rows:DiskConsumerV1[]=[],deadline=options?.deadline,scope=deadline&&signal?AbortSignal.any([signal,deadline]):deadline??signal;
 for(const writer of writers){
  signal?.throwIfAborted();
  if(deadline?.aborted)break;
  options?.report?.(writer.row.id);
  try{
   scope?.throwIfAborted();
   const bytes=await writer.measure(scope);
   scope?.throwIfAborted();
   if(Number.isFinite(bytes)&&bytes>0){
    const detail=await writer.describe?.(scope).catch(()=>undefined);
    scope?.throwIfAborted();
    rows.push({id:writer.row.id,root:detail?.root??writer.root,bytes,command:detail?.command??writer.row.command});
   }
  }catch(error){if(signal?.aborted)throw error;}
 }
 const sorted=rows.sort((first,second)=>second.bytes-first.bytes||(first.id<second.id?-1:1)),top=sorted.slice(0,count);
 return Object.freeze([...top,...sorted.slice(count).filter(row=>DISK_RECOVERY_POLICY_V1.namedConsumers.includes(row.id))]);
}

/** 🚨️ The typed bilingual refusal of a command the volume cannot take even after every automatic collection: it names the volume, its free bytes, what the command needs and the largest remaining consumers by root with their sizes and the exact command that frees each. */
export function refuseDiskRecoveryV1(outcome:DiskRecoveryV1,partition:DiskFactsV1):RepoErrorV1{
 const budget=casPartitionBudgetV1(partition.totalBytes),consumers=outcome.consumers.map(row=>`${row.root} (${formatDiskBytesV1(row.bytes)}): \`${row.command}\``).join("; ")||"-";
 return repoErrorV1(DISK_RECOVERY_TEXTS_V1,"refused","low-space-consumers",{partition:partition.partition,free:formatDiskBytesV1(partition.freeBytes),usedPercent:Math.round((1-partition.freeBytes/Math.max(1,partition.totalBytes))*1000)/10,must:formatDiskBytesV1(outcome.mustFreeBytes),need:formatDiskBytesV1(outcome.needBytes),cushion:formatDiskBytesV1(budget.cushionBytes),consumers});
}
