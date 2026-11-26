import outputPage from "./💼️job/📄️output-page/🧬️schema/🔣️.json" with {type:"json"};
import stockPolicy from "./🔁️succession/📋️policy.json" with {type:"json"};
import { actorInputWireSourceV1, createActorInputWireV1 } from "./📨️actor-input-wire/🟦️.ts";
const actorInputWire=createActorInputWireV1();
/** 🎟️ Kernel admission and worker custody of one original operation. */
export interface OriginalRetainedGrant { readonly maximumItems:number;readonly maximumCopyBytes:number;readonly maximumCapacityBytes:number;readonly maximumReleaseBytes:number;readonly maximumDepth:number }
export interface OriginalRetainedProgress { readonly copiedItems:number;readonly copiedBytes:number;readonly retainedCapacityBytes:number;readonly releasedBytes:number }
export interface OriginalRetainedInput { readonly operation:bigint;readonly generation:bigint;readonly epoch:bigint;readonly grant:OriginalRetainedGrant }
export interface OriginalOperationResources { readonly grant:OriginalRetainedGrant;readonly fuel:number;readonly maximumElapsedUs:bigint }
export interface OriginalOperationAdmission extends OriginalOperationResources { readonly operation:bigint;readonly generation:bigint }
export interface OriginalOperationTurn { readonly input:OriginalRetainedInput;readonly fuel:number;readonly budgetFuel:number;readonly deadlineUs:bigint;readonly cancelled:boolean }
export interface OriginalOperationReceipt { readonly input:OriginalRetainedInput;readonly spent:OriginalRetainedProgress;readonly workUnitsSpent:number;readonly nativeReceipt?:OriginalWireReceipt;readonly nativeRemaining?:OriginalRetainedGrant;readonly successor?:bigint }
/** 🔁️ The kernel-issued successor one named predecessor turn admits (`🔁️succession/🧬️schema/🔣️.json`, Rust `OriginalActorSuccession`). */
export interface OriginalOperationSuccession { readonly operation:bigint;readonly generation:bigint;readonly epoch:bigint;readonly successor:bigint;readonly grant:OriginalRetainedGrant;readonly fuel:number }
/** 🪫️ The stock axis a Source ran out of; `maximumElapsedUs` is the Source identity's own deadline, which no successor extends. */
export type OriginalStockAxis="maximumItems"|"maximumCopyBytes"|"maximumCapacityBytes"|"maximumReleaseBytes"|"fuel"|"maximumElapsedUs";
/** 🪫️ The framework fault of an exhausted Source, readable by every host without parsing its message. */
export interface OriginalStockFault { readonly origin:"framework";readonly code:"plugin.original-stock-exhausted";readonly severity:"error";readonly message:string;readonly scope:{};readonly params:{readonly axis:OriginalStockAxis;readonly remaining:string;readonly floor:string};readonly retryable:true }
export type OriginalWireGrant={readonly [K in keyof OriginalRetainedGrant]:bigint};
export interface OriginalWireInput {readonly operation:bigint;readonly generation:bigint;readonly epoch:bigint;readonly grant:OriginalWireGrant}
export interface OriginalWireReceipt {readonly input:OriginalWireInput;readonly spent:{readonly [K in keyof OriginalRetainedProgress]:bigint};readonly remaining?:OriginalWireGrant}
export interface OriginalOperationPhysicalSpent extends OriginalRetainedProgress { readonly workUnitsSpent:number }
const U64=0xffffffffffffffffn;
const currencies=[['maximumItems','copiedItems'],['maximumCopyBytes','copiedBytes'],['maximumCapacityBytes','retainedCapacityBytes'],['maximumReleaseBytes','releasedBytes']] as const;
function amount(value:number):number{if(!Number.isSafeInteger(value)||value<0)throw Error('original-turn.invalid-amount');return value}
function identity(value:bigint):bigint{if(typeof value!=='bigint'||value<=0n||value>U64)throw Error('original-turn.invalid-identity');return value}
function grant(value:OriginalRetainedGrant):OriginalRetainedGrant{return Object.freeze({maximumItems:amount(value.maximumItems),maximumCopyBytes:amount(value.maximumCopyBytes),maximumCapacityBytes:amount(value.maximumCapacityBytes),maximumReleaseBytes:amount(value.maximumReleaseBytes),maximumDepth:amount(value.maximumDepth)})}
/** 🪙️ Finite kernel stock supplies admission; spent stock cannot be recreated by a retry. */
export class KernelOriginalResourcePool {
 #remaining:OriginalRetainedGrant;#fuel:number;readonly #maximumElapsedUs:bigint;
 constructor(stock:OriginalOperationResources){this.#remaining=grant(stock.grant);this.#fuel=amount(stock.fuel);if(typeof stock.maximumElapsedUs!=='bigint'||stock.maximumElapsedUs<0n||stock.maximumElapsedUs>U64)throw Error('original-turn.invalid-duration');this.#maximumElapsedUs=stock.maximumElapsedUs}
 get remaining():OriginalRetainedGrant{return this.#remaining}
 get fuelRemaining():number{return this.#fuel}
 admit(request:OriginalOperationResources):OriginalOperationResources{const wanted=grant(request.grant),fuel=amount(request.fuel);if(typeof request.maximumElapsedUs!=='bigint'||request.maximumElapsedUs<0n||request.maximumElapsedUs>this.#maximumElapsedUs||fuel>this.#fuel||wanted.maximumDepth>this.#remaining.maximumDepth)throw Error('original-turn.kernel-resource-denied');for(const [axis]of currencies)if(wanted[axis]>this.#remaining[axis])throw Error('original-turn.kernel-resource-denied');const next={...this.#remaining};for(const [axis]of currencies)next[axis]-=wanted[axis];this.#remaining=Object.freeze(next);this.#fuel-=fuel;return Object.freeze({grant:wanted,fuel,maximumElapsedUs:request.maximumElapsedUs})}
}
/** 🏛️ Allocates canonical identities only after the defining kernel resource admission accepts. */
export class KernelOriginalOperationIssuer {
 #next=1n;
 private readonly admitResources: (request:OriginalOperationResources)=>OriginalOperationResources;
 constructor(admitResources:(request:OriginalOperationResources)=>OriginalOperationResources){ this.admitResources = admitResources; }
 admit(request:OriginalOperationResources):OriginalOperationAdmission{const operation=identity(this.#next);if(operation===U64)throw Error('original-turn.identity-exhausted');const admitted=this.admitResources(request);const owned=grant(admitted.grant),fuel=amount(admitted.fuel),elapsed=admitted.maximumElapsedUs;if(typeof elapsed!=='bigint'||elapsed<0n||elapsed>U64)throw Error('original-turn.invalid-duration');this.#next+=1n;return Object.freeze({operation,generation:1n,grant:owned,fuel,maximumElapsedUs:elapsed})}
}
/** 🚱️ A Source below its floor refuses new work with the typed fault while its cleanup and succession stay funded. */
export class OriginalStockExhausted extends Error {
 readonly fault:OriginalStockFault;
 constructor(axis:OriginalStockAxis,remaining:bigint,floor:bigint,cause:unknown){super(`plugin.original-stock-exhausted: ${axis} has ${remaining} of its floor ${floor}`,{cause});this.fault=Object.freeze({origin:"framework",code:"plugin.original-stock-exhausted",severity:"error",message:this.message,scope:{},params:{axis,remaining:remaining.toString(),floor:floor.toString()},retryable:true})}
}
/** ⚠️ A returned original receipt stays identifiable when its physical effects exceed authority. */
export class OriginalOperationReceiptRefusal extends Error {
 readonly receipt: OriginalOperationReceipt;
 constructor(receipt:OriginalOperationReceipt){super('original-turn.receipt-refused'); this.receipt = receipt;}
}
/** 🧵️ One same-worker clock binding issues remaining authority after each exact original return. */
export class OriginalOperationSource {
 readonly operation:bigint;readonly generation:bigint;readonly deadlineUs:bigint;
 #remaining:OriginalRetainedGrant;#fuel:number;#tenure:{readonly operation:bigint;readonly grant:OriginalRetainedGrant;readonly fuel:number};#epoch=1n;#reserved=false;#outstanding:OriginalOperationTurn|undefined;#cancelled=false;#closed=false;#receiptRefusal:OriginalOperationReceipt|undefined;#producerRefusal:unknown;#nativeRefusal:unknown;#lastClock:bigint;#physical:OriginalOperationPhysicalSpent=Object.freeze({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0,workUnitsSpent:0});
 private readonly clock: ()=>bigint;
 constructor(admission:OriginalOperationAdmission,clock:()=>bigint){this.clock = clock; this.operation=identity(admission.operation);this.generation=identity(admission.generation);this.#remaining=grant(admission.grant);this.#fuel=amount(admission.fuel);this.#tenure=Object.freeze({operation:this.operation,grant:this.#remaining,fuel:this.#fuel});const now=clock();if(typeof now!=='bigint'||now<0n||now>U64||typeof admission.maximumElapsedUs!=='bigint'||admission.maximumElapsedUs<0n||admission.maximumElapsedUs>U64-now)throw Error('original-turn.deadline-overflow');this.#lastClock=now;this.deadlineUs=now+admission.maximumElapsedUs}
 operationClockUs():bigint{const now=this.clock();if(typeof now!=='bigint'||now<this.#lastClock||now>U64)throw Error('original-turn.clock-refused');this.#lastClock=now;return now}
 get fuelRemaining():number{return this.#fuel}
 get retainedRemaining():OriginalRetainedGrant{return this.#remaining}
 get outstanding():boolean{return this.#outstanding!==undefined}
 get cancelled():boolean{return this.#cancelled}
 /** 🪪️ The kernel operation whose stock funds this Source now; its own identity never changes. */
 get tenure():bigint{return this.#tenure.operation}
 #below(divisor:number):readonly [OriginalStockAxis,number,number]|undefined{for(const [axis] of currencies){const floor=Math.ceil(this.#tenure.grant[axis]/divisor);if(this.#remaining[axis]<floor)return [axis,this.#remaining[axis],floor]}const floor=Math.ceil(this.#tenure.fuel/divisor);return this.#fuel<floor?["fuel",this.#fuel,floor]:undefined}
 /** 🪫️ Below its low-water mark a Source asks the kernel for a successor as soon as its actor is quiescent. */
 get lowWater():boolean{return this.#below(ORIGINAL_STOCK_LOW_WATER)!==undefined}
 /** 🚱️ New work needs stock above the floor; cleanup and succession spend the floor. */
 /** 🛟️ Fuel one gated turn never receives, so a single poll cannot drain the stock and exhaustion settles typed at the next gate. */
 get reserveFuel():number{return Math.ceil(this.#tenure.fuel/ORIGINAL_STOCK_RESERVE)}
 /** 🪙️ Cleanup and succession turns spend the reserve. */
 spendReserve():void{this.#reserved=false}
 requireStock(cause?:unknown):void{this.#reserved=true;const below=this.#below(ORIGINAL_STOCK_FLOOR);if(below!==undefined)throw new OriginalStockExhausted(below[0],BigInt(below[1]),BigInt(below[2]),cause);if(this.operationClockUs()>=this.deadlineUs)throw new OriginalStockExhausted("maximumElapsedUs",0n,1n,cause)}
 /** 🔁️ Names the exact turn after the next one as the predecessor that pays for and closes its own stock. */
 quoteSuccession(successor:OriginalOperationAdmission):OriginalOperationSuccession{if(this.#cancelled||identity(successor.operation)<=this.#tenure.operation)throw Error('original-turn.succession-refused');return Object.freeze({operation:this.operation,generation:this.generation,epoch:identity(this.#epoch+1n),successor:successor.operation,grant:grant(successor.grant),fuel:amount(successor.fuel)})}
 /** 🤝️ Receives the predecessor's last receipt, then replaces only the funds; identity, epoch order, clock and deadline continue. */
 succeed(receipt:OriginalOperationReceipt,successor:OriginalOperationAdmission):void{if(this.#cancelled||this.#closed||receipt.successor!==identity(successor.operation)||successor.operation<=this.#tenure.operation)throw Error('original-turn.succession-refused');const next=grant(successor.grant),fuel=amount(successor.fuel);this.receive(receipt);this.#remaining=next;this.#fuel=fuel;this.#tenure=Object.freeze({operation:successor.operation,grant:next,fuel})}
 get receiptRefusal():OriginalOperationReceipt|undefined{return this.#receiptRefusal}
 get nativeRefusal():unknown{return this.#nativeRefusal}
 holdNativeRefusal(call:unknown):void{if(this.#nativeRefusal!==undefined||call===undefined)throw Error("original-turn.native-return-refused");this.#nativeRefusal=call}
 get producerRefusal():unknown{return this.#producerRefusal}
 holdProducerRefusal(report:unknown):void{if(this.#producerRefusal!==undefined||report===undefined)throw Error("original-turn.producer-return-refused");this.#producerRefusal=report}
 get physicalSpent():OriginalOperationPhysicalSpent{return this.#physical}
 cancel():void{if(this.#closed)throw Error('original-turn.closed');this.#cancelled=true}
 issue():OriginalOperationTurn{if(this.#closed||this.#outstanding||this.#receiptRefusal||this.#producerRefusal!==undefined||this.#nativeRefusal!==undefined)throw Object.assign(Error('original-turn.source-not-ready'),{cause:this.#nativeRefusal??this.#producerRefusal??this.#receiptRefusal});const input=Object.freeze({operation:this.operation,generation:this.generation,epoch:identity(this.#epoch),grant:this.#remaining});const turn=Object.freeze({input,fuel:this.#fuel,budgetFuel:this.#reserved?this.#fuel-this.reserveFuel:this.#fuel,deadlineUs:this.deadlineUs,cancelled:this.#cancelled});this.#outstanding=turn;return turn}
 receive(receipt:OriginalOperationReceipt):void{const issued=this.#outstanding;if(!issued||receipt.input.operation!==issued.input.operation||receipt.input.generation!==issued.input.generation||receipt.input.epoch!==issued.input.epoch)throw Error('original-turn.foreign-receipt');for(const key of Object.keys(issued.input.grant) as (keyof OriginalRetainedGrant)[])if(receipt.input.grant[key]!==issued.input.grant[key])throw Error('original-turn.foreign-grant');amount(receipt.workUnitsSpent);for(const [,axis]of currencies)amount(receipt.spent[axis]);const physical={...this.#physical};for(const [,axis]of currencies)physical[axis]=amount(physical[axis]+receipt.spent[axis]);physical.workUnitsSpent=amount(physical.workUnitsSpent+receipt.workUnitsSpent);let refused=receipt.workUnitsSpent>this.#fuel;if(receipt.nativeRemaining){for(const[axis,key]of currencies)refused ||=receipt.nativeRemaining[axis]+receipt.spent[key]!==receipt.input.grant[axis];refused ||=receipt.nativeRemaining.maximumDepth!==receipt.input.grant.maximumDepth;}for(const [axis,spent]of currencies)refused ||=receipt.spent[spent]>this.#remaining[axis];this.#physical=Object.freeze(physical);this.#outstanding=undefined;if(refused){this.#receiptRefusal=receipt;throw new OriginalOperationReceiptRefusal(receipt)}const next={...this.#remaining};for(const [axis,spent]of currencies)next[axis]-=receipt.spent[spent];this.#remaining=Object.freeze(next);this.#fuel-=receipt.workUnitsSpent;if(this.#epoch===U64)throw Error('original-turn.epoch-exhausted');this.#epoch+=1n}
 close():void{if(this.#outstanding||this.#receiptRefusal||this.#producerRefusal!==undefined||this.#nativeRefusal!==undefined)throw Error('original-turn.original-return-pending');this.#closed=true}
}
export interface OriginalWireWork {readonly clockBinding:bigint;readonly absoluteDeadlineUs:bigint;readonly fuel:bigint}
export interface OriginalTurnInputWit extends OriginalWireInput {readonly work:OriginalWireWork}
export interface OriginalContextBudgetWit {readonly input:OriginalTurnInputWit;readonly retainedPolicy:OriginalWireGrant;readonly fuel:bigint;readonly deadlineUs:bigint;readonly cancelled:boolean}
export interface OriginalJobBudgetWit {readonly input:OriginalTurnInputWit;readonly retainedPolicy:OriginalWireGrant;readonly fuel:bigint;readonly deadlineUs:bigint;readonly cancelled:boolean}
export interface OriginalTurnShape {readonly maxEffects:number;readonly maxPatchBytes:number;readonly maxFrames:number}
export interface OriginalGuestJobs {startJob(job:bigint,budget:OriginalJobBudgetWit):Promise<any>;stepJob(job:bigint,budget:OriginalJobBudgetWit):Promise<any>;readJobOutput(job:bigint,budget:OriginalJobBudgetWit):Promise<any>;acknowledgeJobOutput(job:bigint,budget:OriginalJobBudgetWit):Promise<any>;cancelJob(job:bigint,budget:OriginalJobBudgetWit):Promise<any>;closeJobs(budget:OriginalJobBudgetWit):Promise<any>}
export interface OriginalGuestReactor {admitActorContext(budget:OriginalContextBudgetWit):Promise<any>;admitActorInput(wire:Uint8Array,budget:OriginalContextBudgetWit):Promise<any>;poll(budget:OriginalContextBudgetWit):Promise<any>}
export interface OriginalGuestExports {readonly reactor:OriginalGuestReactor;readonly jobs:OriginalGuestJobs}
const ORIGINAL_SELF_ISSUED_GRANT:OriginalRetainedGrant=Object.freeze({maximumItems:1<<20,maximumCopyBytes:64<<20,maximumCapacityBytes:64<<20,maximumReleaseBytes:64<<20,maximumDepth:64});
const ORIGINAL_JOB_WORK_FUEL=1<<24;
/** 📄️ The copy bound granted to one read-job-output crossing, from `💼️job/📄️output-page/🧬️schema/🔣️.json` (Rust `OriginalJobBudget::OUTPUT_PAGE_BYTES`). */
const ORIGINAL_JOB_OUTPUT_PAGE_BYTES:number=outputPage.properties.outputPageBytes.const;
const ORIGINAL_JOB_CONTROL_WALL_US=30_000_000n;
const ORIGINAL_JOB_PENDING_PAGES=1024;
/** 🪫️ The low-water and floor fractions of one operation's stock, from `🔁️succession/📋️policy.json`. */
const ORIGINAL_STOCK_LOW_WATER:number=stockPolicy.lowWaterDivisor;
const ORIGINAL_STOCK_FLOOR:number=stockPolicy.floorDivisor;
const ORIGINAL_STOCK_DEFERRAL:number=stockPolicy.maximumDeferral;
const ORIGINAL_STOCK_RESERVE:number=stockPolicy.reserveDivisor;
const ORIGINAL_SELF_ISSUED_ELAPSED_US=1n<<52n;
/** 🎁️ jco lifts `option<t>` as a tagged `{tag:"none"|"some"}` variant and lowers it from `t|undefined`; the host reads the bare value. */
export function optionValue(value:any):any{if(value===null||value===undefined)return undefined;if(typeof value==="object"&&(value.tag==="none"||value.tag==="some"))return value.tag==="none"?undefined:value.val;return value}
/** 🪙️ Lowers a retained grant to the component's u64 record. */
export function originalGrantWit(grant:OriginalRetainedGrant):OriginalWireGrant{return {maximumItems:BigInt(grant.maximumItems),maximumCopyBytes:BigInt(grant.maximumCopyBytes),maximumCapacityBytes:BigInt(grant.maximumCapacityBytes),maximumReleaseBytes:BigInt(grant.maximumReleaseBytes),maximumDepth:BigInt(grant.maximumDepth)}}
/** 📦️ Lowers one issued original turn to `reactor.retained-turn-input`: identity, the original work record bound to the issuer clock, and the remaining grant. */
export function originalTurnWit(turn:OriginalOperationTurn):OriginalTurnInputWit{const input=turn.input;return {operation:input.operation,generation:input.generation,epoch:input.epoch,work:{clockBinding:input.operation,absoluteDeadlineUs:turn.deadlineUs,fuel:BigInt(turn.fuel)},grant:originalGrantWit(input.grant)}}
/** 🎚️ Lowers one issued original turn to `reactor.actor-context-budget`, the single budget every context, describe, checkpoint and codec crossing carries. */
export function originalContextBudgetWit(turn:OriginalOperationTurn):OriginalContextBudgetWit{return {input:originalTurnWit(turn),retainedPolicy:originalGrantWit(turn.input.grant),fuel:BigInt(turn.budgetFuel),deadlineUs:turn.deadlineUs,cancelled:false}}
/** ⏳️ The wall a job request may take: the caller's step deadline, or the control-call default. */
export function originalJobWallUs(budget?:{readonly deadlineMs:number}):bigint{return budget===undefined?ORIGINAL_JOB_CONTROL_WALL_US:BigInt(budget.deadlineMs)*1000n}

/** 📥️ What one job request holds of a job's output between calls, so a stalled or aborted collection resumes at the offset the guest kept instead of faulting `output-offset`. */
export type OriginalJobOutputHold = Map<bigint,{parts:Uint8Array[];length:bigint;outcome?:{readonly state:string;readonly byteLength:bigint}}>;
/** 🎛️ The caller's levers on one job request: cancellation, the held output and a progress sink fed per output page. */
export interface OriginalJobControl {readonly signal?:AbortSignal;readonly held?:OriginalJobOutputHold;readonly onProgress?:(progress:{readonly job:bigint;readonly bytes:bigint;readonly expected:bigint})=>void}

type OriginalJobCrossing = <T extends OriginalWireReceipt & { readonly workUnitsSpent: bigint; readonly refusal?: unknown }>(
  call: (budget: OriginalJobBudgetWit) => Promise<T>, output: boolean,
) => Promise<T>;

/** 💼️ Funds of one guest job request: a retained turn (self-issued, or issued by the actor's Kernel source) whose bounded loan and work credit each crossing settles, like the native `JobFunds` (`🔌️plugin/🖥️host/🎭️guest-runtime/💼️job-crossing/🦀️.rs`). */
export class OriginalJobFunds {
 readonly #input:OriginalTurnInputWit;readonly #deadlineUs:bigint;#granted:OriginalWireGrant;#policy:OriginalWireGrant;#work:bigint;readonly #fuel:bigint;readonly #clock:(()=>bigint)|undefined;readonly #drive:OriginalJobCrossing|undefined;
 constructor(operation:bigint,nowUs:bigint,wallUs:bigint,clock?:()=>bigint,turn?:OriginalOperationTurn,drive?:OriginalJobCrossing){this.#clock=clock;this.#drive=drive;if(turn===undefined){const grant=originalGrantWit(ORIGINAL_SELF_ISSUED_GRANT);this.#input={operation:operation>0n?operation:1n,generation:1n,epoch:1n,work:{clockBinding:1n,absoluteDeadlineUs:nowUs+wallUs,fuel:BigInt(ORIGINAL_JOB_WORK_FUEL)},grant};this.#work=BigInt(ORIGINAL_JOB_WORK_FUEL)}else{this.#input=originalTurnWit(turn);this.#work=BigInt(turn.budgetFuel)}const wall=nowUs+wallUs,issuedDeadline=this.#input.work.absoluteDeadlineUs;this.#deadlineUs=wall<issuedDeadline?wall:issuedDeadline;this.#fuel=this.#work;this.#granted=this.#input.grant;this.#policy={...this.#input.grant}}
 crossing():OriginalJobBudgetWit{if(this.#work===0n)throw Error("original-job.work-exhausted");return {input:this.#input,retainedPolicy:this.#policy,fuel:this.#work,deadlineUs:this.#deadlineUs,cancelled:false}}
 /** 📃️ Narrows only the copy axis, so the guest pages by what the host grants while the loan still settles the real spend. */
 outputCrossing():OriginalJobBudgetWit{const budget=this.crossing(),page=BigInt(ORIGINAL_JOB_OUTPUT_PAGE_BYTES),policy=budget.retainedPolicy;return {...budget,retainedPolicy:{...policy,maximumCopyBytes:policy.maximumCopyBytes<page?policy.maximumCopyBytes:page}}}
 /** 🧾️ The one receipt the issuing source receives for everything this request spent. */
 receipt(turn:OriginalOperationTurn):OriginalOperationReceipt{const spent=(axis:keyof OriginalWireGrant):number=>Number(this.#granted[axis]-this.#policy[axis]);return {input:turn.input,spent:{copiedItems:spent("maximumItems"),copiedBytes:spent("maximumCopyBytes"),retainedCapacityBytes:spent("maximumCapacityBytes"),releasedBytes:spent("maximumReleaseBytes")},workUnitsSpent:Number(this.#fuel-this.#work)}}
 settle(call:any,policy:OriginalWireGrant=this.#policy):void{const input=this.#input;if(call.input.operation!==input.operation||call.input.generation!==input.generation||call.input.epoch!==input.epoch||call.input.work?.clockBinding!==input.work.clockBinding||call.input.work?.absoluteDeadlineUs!==input.work.absoluteDeadlineUs||call.input.work?.fuel!==input.work.fuel||call.input.grant?.maximumDepth!==input.grant.maximumDepth||currencies.some(([axis])=>call.input.grant?.[axis]!==input.grant[axis]))throw Error("original-job.foreign-receipt");const next:{-readonly [K in keyof OriginalWireGrant]:bigint}={...this.#policy};for(const [axis,spent] of currencies){const used=call.spent[spent];if(typeof used!=="bigint"||used<0n||used>policy[axis]||used>next[axis])throw Error("original-job.receipt-refused");next[axis]-=used}if(typeof call.workUnitsSpent!=="bigint"||call.workUnitsSpent<0n||call.workUnitsSpent>this.#work)throw Error("original-job.receipt-refused");this.#policy=next;this.#work-=call.workUnitsSpent;if(optionValue(call.refusal)!==undefined)throw Object.assign(Error("original-job.call-refused"),{refusal:call.refusal})}
 #pending(attempt:number,message:string):void {
  if(attempt>=ORIGINAL_JOB_PENDING_PAGES||(this.#clock!==undefined&&this.#clock()>=this.#deadlineUs))throw Error(message);
 }
 async #request<T extends OriginalWireReceipt & {readonly workUnitsSpent:bigint;readonly refusal?:unknown}>(call:(budget:OriginalJobBudgetWit)=>Promise<T>,output=false):Promise<T>{
  if(this.#drive!==undefined)return this.#drive(call,output);
  const budget=output?this.outputCrossing():this.crossing();
  const reply=await call(budget);
  this.settle(reply,budget.retainedPolicy);
  return reply;
 }
 async start(guest:OriginalGuestExports,job:bigint,kind:string,input:Uint8Array):Promise<void>{
  let source:{job:bigint;kind:string;input:Uint8Array;checkpoint:null}|null={job,kind,input,checkpoint:null};
  for(let pending=0;;pending++){
   const wire=actorInputWire.encode({operation:"start-job",source});
   const admission=await this.#request((budget)=>guest.reactor.admitActorInput(wire,budget));
   if(!admission.ready||admission.disposition==="returned")throw Error("original-job.input-refused");
   const reply=await this.#request((budget)=>guest.jobs.startJob(job,budget));
   if(reply.admitted)return;
   this.#pending(pending,"original-job.start-stalled");
   source=null;
  }
 }
 async step(jobs:OriginalGuestJobs,job:bigint,control:OriginalJobControl={}):Promise<{status:string;progress?:Uint8Array;value?:Uint8Array}>{let outcome=control.held?.get(job)?.outcome;if(outcome===undefined){const call=await this.#request((budget)=>jobs.stepJob(job,budget));outcome=optionValue(call.outcome);if(outcome===undefined)return {status:"running"}}const bytes=await this.#collect(jobs,job,outcome!,control);if(outcome.state==="running")return {status:"running",progress:bytes.length>0?bytes:undefined};if(outcome.state==="done")return {status:"done",value:bytes};if(outcome.state==="failed")return {status:"failed",value:bytes};throw Error("original-job.outcome-state")}
 async cancel(jobs:OriginalGuestJobs,job:bigint,control:OriginalJobControl={}):Promise<void>{const unfinished=control.held?.get(job)?.outcome;if(unfinished!==undefined){await this.#collect(jobs,job,unfinished,{signal:control.signal,held:control.held});if(unfinished.state!=="running")return}for(let pending=0;!(await this.#request((budget)=>jobs.cancelJob(job,budget))).admitted;pending++)this.#pending(pending,"original-job.cancel-stalled");control.held?.delete(job)}
 /** ♻️ Closes every registered job of the actor under paid steps; the context close stays blocked until this answers admitted. */
 async close(jobs:OriginalGuestJobs):Promise<void>{for(let pending=0;;){const call=await this.#request((budget)=>jobs.closeJobs(budget));if(call.admitted)return;const spent=call.spent;if(spent.copiedItems+spent.copiedBytes+spent.retainedCapacityBytes+spent.releasedBytes>0n)pending=0;else this.#pending(pending++,"original-job.close-stalled")}}
 async #collect(jobs:OriginalGuestJobs,job:bigint,outcome:{readonly state:string;readonly byteLength:bigint},control:OriginalJobControl):Promise<Uint8Array>{const expected=outcome.byteLength;const held:{parts:Uint8Array[];length:bigint;outcome?:typeof outcome}=control.held?.get(job)??{parts:[],length:0n};held.outcome=outcome;control.held?.set(job,held);let pending=0;for(;;){if(control.signal?.aborted)throw Error("original-job.aborted");const call=await this.#request((budget)=>jobs.readJobOutput(job,budget),true);const chunk=optionValue(call.chunk);if(chunk===undefined)break;const page=optionValue(chunk.bytes);if(page!==undefined){if(chunk.offset!==held.length)throw Error("original-job.output-offset");held.parts.push(page);held.length+=BigInt(page.length);pending=0;control.onProgress?.({job,bytes:held.length,expected})}if(chunk.outputClosed)break;if(page===undefined)this.#pending(pending++,"original-job.output-stalled")}if(held.length!==expected)throw Error("original-job.output-length");for(let pending=0;;pending++){const ack=await this.#request((budget)=>jobs.acknowledgeJobOutput(job,budget));if(ack.acknowledged)break;this.#pending(pending,"original-job.output-ack-stalled")}const bytes=new Uint8Array(Number(held.length));let at=0;for(const part of held.parts){bytes.set(part,at);at+=part.length}control.held?.delete(job);return bytes}
}

/** 🏛️ Issues a new actor turn for each guest crossing while preserving the job request deadline. */
export async function originalSourceJob<T>(source:OriginalOperationSource,report:(receipt:OriginalOperationReceipt)=>void,wallUs:bigint,run:(funds:OriginalJobFunds)=>Promise<T>,cleanup=false):Promise<T>{
 const clock=()=>source.operationClockUs(),started=clock();
 const drive:OriginalJobCrossing=async(call,output)=>{
  if(cleanup)source.spendReserve();else source.requireStock();
  const turn=source.issue();
  const funds=new OriginalJobFunds(source.operation,started,wallUs,clock,turn);
  const budget=output?funds.outputCrossing():funds.crossing();
  let reply;
  try{reply=await call(budget)}
  catch(failure){source.holdNativeRefusal(failure);throw failure}
  let refusal:unknown;
  try{funds.settle(reply,budget.retainedPolicy)}
  catch(failure){
   if(!(failure instanceof Error)||failure.message!=="original-job.call-refused"){source.holdNativeRefusal(reply);throw failure}
   refusal=failure;
  }
  const receipt=funds.receipt(turn);
  source.receive(receipt);
  report(receipt);
  if(refusal!==undefined)throw refusal;
  return reply;
 };
 return run(new OriginalJobFunds(source.operation,started,wallUs,clock,undefined,drive));
}

/** 🏭️ Drives a guest that has no Kernel grant (the document browser actor child): one self-issued original source whose finite stock the guest's retained-turn custody conserves from turn to turn, so exhaustion is a typed fault (the recovery path re-reserves a child with a fresh source) and a failure keeps its own error and holds the source with it as the cause. Job requests are standalone self-issued funds, like the native relay (`relay_job_funds`). */
export class OriginalSelfIssuedTurns {
 readonly #guest:OriginalGuestExports;readonly #source:OriginalOperationSource;readonly #clock:()=>bigint;readonly #held:OriginalJobOutputHold=new Map();#ready=false;
 constructor(guest:OriginalGuestExports,clock:()=>bigint){this.#guest=guest;this.#clock=clock;this.#source=new OriginalOperationSource(Object.freeze({operation:1n,generation:1n,grant:ORIGINAL_SELF_ISSUED_GRANT,fuel:Number.MAX_SAFE_INTEGER,maximumElapsedUs:ORIGINAL_SELF_ISSUED_ELAPSED_US}),clock)}
 #stock():void{this.#source.requireStock()}
 #receive(call:any,workUnitsSpent:bigint):void{this.#source.receive(originalReceiptFromWit(call,workUnitsSpent))}
 #hold(failure:unknown):void{try{if(this.#source.outstanding&&this.#source.nativeRefusal===undefined)this.#source.holdNativeRefusal(failure??Error("original-turn.native-return-refused"))}catch{}}
 async #context():Promise<void>{while(!this.#ready){this.#stock();const turn=this.#source.issue();let call:any;try{call=await this.#guest.reactor.admitActorContext(originalContextBudgetWit(turn))}catch(failure){this.#hold(failure);throw failure}this.#receive(call,call.workUnitsSpent);if(optionValue(call.refusal)!==undefined)throw Object.assign(Error("original-turn.context-refused"),{originalCall:call});this.#ready=call.ready;if(!this.#ready&&call.spent.copiedItems===0n&&call.spent.copiedBytes===0n&&call.spent.retainedCapacityBytes===0n&&call.spent.releasedBytes===0n)throw Object.assign(Error("original-turn.context-held"),{originalCall:call})}}
 async #admit(value:unknown):Promise<void>{this.#stock();const turn=this.#source.issue();let call:any;try{call=await this.#guest.reactor.admitActorInput(actorInputWire.encode(value),originalContextBudgetWit(turn))}catch(failure){this.#hold(failure);throw failure}this.#receive(call,call.workUnitsSpent);if(optionValue(call.refusal)!==undefined||!call.ready||call.disposition==="returned")throw Object.assign(Error("original-turn.input-refused"),{originalCall:call})}
 async poll(events:readonly unknown[],shape:OriginalTurnShape):Promise<any>{if(events.length>1)throw Error("actor-poll.event-count: a reactor turn admits at most one event");await this.#context();await this.#admit({operation:"poll",events:events.map(actorInputWire.event),shape});this.#stock();const turn=this.#source.issue();let result;try{result=await this.#guest.reactor.poll(originalContextBudgetWit(turn))}catch(failure:any){try{if(failure&&failure.input&&failure.spent&&typeof failure.workUnitsSpent==="bigint")this.#receive(failure,failure.workUnitsSpent)}catch{}this.#hold(failure);throw failure}this.#receive(result.retainedReceipt,result.retainedReceipt.workUnitsSpent);return result}
 async startJob(job:bigint,kind:string,input:Uint8Array):Promise<void>{await this.#context();return originalSourceJob(this.#source,()=>{},originalJobWallUs(),(funds)=>funds.start(this.#guest,job,kind,input))}
 stepJob(job:bigint,budget:{readonly deadlineMs:number},control:OriginalJobControl={}):Promise<{status:string;progress?:Uint8Array;value?:Uint8Array}>{return originalSourceJob(this.#source,()=>{},originalJobWallUs(budget),(funds)=>funds.step(this.#guest.jobs,job,{...control,held:this.#held}))}
 cancelJob(job:bigint):Promise<void>{return originalSourceJob(this.#source,()=>{},originalJobWallUs(),(funds)=>funds.cancel(this.#guest.jobs,job,{held:this.#held}),true)}
}

/** 📮️ The kernel route of one actor's receipts, held refusals and successor draws. */
export interface OriginalKernelReturn {readonly receipt:(receipt:OriginalOperationReceipt)=>void;readonly hold:(refusal:unknown)=>void;readonly succeed:()=>Promise<OriginalOperationAdmission>}
/** 🔁️ Asks the kernel for a successor below low water and admits it through the guest's paid context admission, only while no job, held output or turn is outstanding; a refusal defers the next attempt and never loses the successor. */
export class OriginalStockSuccession {
 readonly #source:OriginalOperationSource;readonly #reactor:OriginalGuestReactor;readonly #kernel:OriginalKernelReturn;readonly #held:OriginalJobOutputHold;readonly #jobs=new Set<bigint>();#successor:OriginalOperationAdmission|undefined;#refusal:unknown;#wait=0;#deferral=1;
 constructor(source:OriginalOperationSource,reactor:OriginalGuestReactor,kernel:OriginalKernelReturn,held:OriginalJobOutputHold){if(typeof kernel.succeed!=="function")throw Error("original-turn.succession-unoffered: the served shard worker is older than this component bridge; run `repo run @semio-tech/framework-plugin-web:support`");this.#source=source;this.#reactor=reactor;this.#kernel=kernel;this.#held=held}
 /** 🧾️ Why the last attempt did not renew the stock. */
 get refusal():unknown{return this.#refusal}
 started(job:bigint):void{this.#jobs.add(job)}
 settled(job:bigint):void{this.#jobs.delete(job);this.#wait=0}
 #defer(refusal:unknown):void{this.#refusal=refusal;this.#wait=this.#deferral;this.#deferral=Math.min(this.#deferral*2,ORIGINAL_STOCK_DEFERRAL)}
 async #cross(call:(budget:OriginalContextBudgetWit)=>Promise<any>,successor?:OriginalOperationAdmission):Promise<any>{const source=this.#source,turn=(source.spendReserve(),source.issue());let reply:any;try{reply=await call(originalContextBudgetWit(turn))}catch(failure){source.holdNativeRefusal(failure);this.#kernel.hold(failure);throw failure}let receipt:OriginalOperationReceipt|undefined;const admitted=successor!==undefined&&reply.ready&&optionValue(reply.refusal)===undefined,proven=admitted&&reply.spent.copiedItems===1n&&reply.spent.retainedCapacityBytes===0n&&reply.spent.releasedBytes===0n&&reply.workUnitsSpent===1n;try{receipt=originalReceiptFromWit(reply,reply.workUnitsSpent);if(proven){receipt={...receipt,successor:successor!.operation};source.succeed(receipt,successor!)}else source.receive(receipt)}catch(failure){if(receipt)this.#kernel.receipt(receipt);source.holdNativeRefusal(reply);this.#kernel.hold(reply);throw failure}this.#kernel.receipt(receipt);if(admitted&&!proven){const unproven=Error("original-turn.succession-unproven: the guest answered ready without the one paid succession receipt; rebuild the plugin");source.holdNativeRefusal(unproven);this.#kernel.hold(unproven);throw unproven}return reply}
 async advance():Promise<void>{
  const source=this.#source;
  if(!source.lowWater||source.cancelled||source.outstanding||this.#held.size>0||this.#jobs.size>0)return;
  if(this.#wait>0){this.#wait-=1;return}
  try{this.#successor??=await this.#kernel.succeed()}catch(refusal){this.#defer(refusal);return}
  const successor=this.#successor,wire=actorInputWire.encode({operation:"succeed",succession:source.quoteSuccession(successor)});
  const admitted=await this.#cross((budget)=>this.#reactor.admitActorInput(wire,budget));
  if(optionValue(admitted.refusal)!==undefined||!admitted.ready||admitted.disposition==="returned"){this.#defer(optionValue(admitted.refusal)??"input-held");return}
  const context=await this.#cross((budget)=>this.#reactor.admitActorContext(budget),successor);
  if(optionValue(context.refusal)!==undefined||!context.ready){this.#defer(optionValue(context.refusal)??"stock-short");return}
  this.#successor=undefined;this.#refusal=undefined;this.#deferral=1;
 }
}

/** 🌐️ Embeds the same defining original source into the actual component worker without a second implementation. */
export function originalOperationWorkerSource():string{return `const U64=0xffffffffffffffffn;const currencies=${JSON.stringify(currencies)};const ORIGINAL_SELF_ISSUED_GRANT=${JSON.stringify(ORIGINAL_SELF_ISSUED_GRANT)};const ORIGINAL_JOB_WORK_FUEL=${ORIGINAL_JOB_WORK_FUEL};const ORIGINAL_JOB_OUTPUT_PAGE_BYTES=${ORIGINAL_JOB_OUTPUT_PAGE_BYTES};const ORIGINAL_JOB_CONTROL_WALL_US=${ORIGINAL_JOB_CONTROL_WALL_US}n;const ORIGINAL_JOB_PENDING_PAGES=${ORIGINAL_JOB_PENDING_PAGES};const ORIGINAL_STOCK_LOW_WATER=${ORIGINAL_STOCK_LOW_WATER};const ORIGINAL_STOCK_FLOOR=${ORIGINAL_STOCK_FLOOR};const ORIGINAL_STOCK_DEFERRAL=${ORIGINAL_STOCK_DEFERRAL};const ORIGINAL_STOCK_RESERVE=${ORIGINAL_STOCK_RESERVE};const ORIGINAL_SELF_ISSUED_ELAPSED_US=${ORIGINAL_SELF_ISSUED_ELAPSED_US}n;${amount.toString()}\n${identity.toString()}\n${grant.toString()}\n${optionValue.toString()}\n${OriginalOperationReceiptRefusal.toString()}\n${OriginalStockExhausted.toString()}\n${OriginalOperationSource.toString()}\n${originalGrantWit.toString()}\n${originalTurnWit.toString()}\n${actorInputWireSourceV1()}
${originalContextBudgetWit.toString()}\n${originalJobWallUs.toString()}\n${originalReceiptFromWit.toString()}\n${OriginalJobFunds.toString()}\n${originalSourceJob.toString()}\n${OriginalStockSuccession.toString()}\n${OriginalSelfIssuedTurns.toString()}`;}

/** 🧾️ Preserves the native exact receipt before any semantic browser projection. */
export function originalReceiptFromWit(receipt:OriginalWireReceipt,workUnitsSpent:bigint):OriginalOperationReceipt{const extent=(value:bigint):number=>{if(typeof value!=="bigint"||value<0n||value>BigInt(Number.MAX_SAFE_INTEGER))throw Error("original-turn.native-extent-refused");return Number(value)};const raw=receipt.input,input={operation:raw.operation,generation:raw.generation,epoch:raw.epoch,grant:{maximumItems:extent(raw.grant.maximumItems),maximumCopyBytes:extent(raw.grant.maximumCopyBytes),maximumCapacityBytes:extent(raw.grant.maximumCapacityBytes),maximumReleaseBytes:extent(raw.grant.maximumReleaseBytes),maximumDepth:extent(raw.grant.maximumDepth)}};const spent={copiedItems:extent(receipt.spent.copiedItems),copiedBytes:extent(receipt.spent.copiedBytes),retainedCapacityBytes:extent(receipt.spent.retainedCapacityBytes),releasedBytes:extent(receipt.spent.releasedBytes)},remaining=receipt.remaining;const nativeRemaining=remaining?{maximumItems:extent(remaining.maximumItems),maximumCopyBytes:extent(remaining.maximumCopyBytes),maximumCapacityBytes:extent(remaining.maximumCapacityBytes),maximumReleaseBytes:extent(remaining.maximumReleaseBytes),maximumDepth:extent(remaining.maximumDepth)}:undefined;return {input,spent,workUnitsSpent:extent(workUnitsSpent),nativeReceipt:receipt,nativeRemaining}}
