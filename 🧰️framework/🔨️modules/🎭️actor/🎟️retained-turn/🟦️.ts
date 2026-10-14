/** 🎟️ Kernel admission and worker custody of one original operation. */
export interface OriginalRetainedGrant { readonly maximumItems:number;readonly maximumCopyBytes:number;readonly maximumCapacityBytes:number;readonly maximumReleaseBytes:number;readonly maximumDepth:number }
export interface OriginalRetainedProgress { readonly copiedItems:number;readonly copiedBytes:number;readonly retainedCapacityBytes:number;readonly releasedBytes:number }
export interface OriginalRetainedInput { readonly operation:bigint;readonly generation:bigint;readonly epoch:bigint;readonly grant:OriginalRetainedGrant }
export interface OriginalOperationResources { readonly grant:OriginalRetainedGrant;readonly fuel:number;readonly maximumElapsedUs:bigint }
export interface OriginalOperationAdmission extends OriginalOperationResources { readonly operation:bigint;readonly generation:bigint }
export interface OriginalOperationTurn { readonly input:OriginalRetainedInput;readonly fuel:number;readonly deadlineUs:bigint;readonly cancelled:boolean }
export interface OriginalOperationReceipt { readonly input:OriginalRetainedInput;readonly spent:OriginalRetainedProgress;readonly workUnitsSpent:number;readonly nativeReceipt?:OriginalWireReceipt;readonly nativeRemaining?:OriginalRetainedGrant }
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
 constructor(private readonly admitResources:(request:OriginalOperationResources)=>OriginalOperationResources){}
 admit(request:OriginalOperationResources):OriginalOperationAdmission{const operation=identity(this.#next);if(operation===U64)throw Error('original-turn.identity-exhausted');const admitted=this.admitResources(request);const owned=grant(admitted.grant),fuel=amount(admitted.fuel),elapsed=admitted.maximumElapsedUs;if(typeof elapsed!=='bigint'||elapsed<0n||elapsed>U64)throw Error('original-turn.invalid-duration');this.#next+=1n;return Object.freeze({operation,generation:1n,grant:owned,fuel,maximumElapsedUs:elapsed})}
}
/** ⚠️ A returned original receipt stays identifiable when its physical effects exceed authority. */
export class OriginalOperationReceiptRefusal extends Error {
 constructor(readonly receipt:OriginalOperationReceipt){super('original-turn.receipt-refused')}
}
/** 🧵️ One same-worker clock binding issues remaining authority after each exact original return. */
export class OriginalOperationSource {
 readonly operation:bigint;readonly generation:bigint;readonly deadlineUs:bigint;
 #remaining:OriginalRetainedGrant;#fuel:number;#epoch=1n;#outstanding:OriginalOperationTurn|undefined;#cancelled=false;#closed=false;#receiptRefusal:OriginalOperationReceipt|undefined;#producerRefusal:unknown;#nativeRefusal:unknown;#lastClock:bigint;#physical:OriginalOperationPhysicalSpent=Object.freeze({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0,workUnitsSpent:0});
 constructor(admission:OriginalOperationAdmission,private readonly clock:()=>bigint){this.operation=identity(admission.operation);this.generation=identity(admission.generation);this.#remaining=grant(admission.grant);this.#fuel=amount(admission.fuel);const now=clock();if(typeof now!=='bigint'||now<0n||now>U64||typeof admission.maximumElapsedUs!=='bigint'||admission.maximumElapsedUs<0n||admission.maximumElapsedUs>U64-now)throw Error('original-turn.deadline-overflow');this.#lastClock=now;this.deadlineUs=now+admission.maximumElapsedUs}
 operationClockUs():bigint{const now=this.clock();if(typeof now!=='bigint'||now<this.#lastClock||now>U64)throw Error('original-turn.clock-refused');this.#lastClock=now;return now}
 get fuelRemaining():number{return this.#fuel}
 get retainedRemaining():OriginalRetainedGrant{return this.#remaining}
 get outstanding():boolean{return this.#outstanding!==undefined}
 get receiptRefusal():OriginalOperationReceipt|undefined{return this.#receiptRefusal}
 get nativeRefusal():unknown{return this.#nativeRefusal}
 holdNativeRefusal(call:unknown):void{if(this.#nativeRefusal!==undefined||call===undefined)throw Error("original-turn.native-return-refused");this.#nativeRefusal=call}
 get producerRefusal():unknown{return this.#producerRefusal}
 holdProducerRefusal(report:unknown):void{if(this.#producerRefusal!==undefined||report===undefined)throw Error("original-turn.producer-return-refused");this.#producerRefusal=report}
 get physicalSpent():OriginalOperationPhysicalSpent{return this.#physical}
 cancel():void{if(this.#closed)throw Error('original-turn.closed');this.#cancelled=true}
 issue():OriginalOperationTurn{if(this.#closed||this.#outstanding||this.#receiptRefusal||this.#producerRefusal!==undefined||this.#nativeRefusal!==undefined)throw Error('original-turn.source-not-ready');const input=Object.freeze({operation:this.operation,generation:this.generation,epoch:identity(this.#epoch),grant:this.#remaining});const turn=Object.freeze({input,fuel:this.#fuel,deadlineUs:this.deadlineUs,cancelled:this.#cancelled});this.#outstanding=turn;return turn}
 receive(receipt:OriginalOperationReceipt):void{const issued=this.#outstanding;if(!issued||receipt.input.operation!==issued.input.operation||receipt.input.generation!==issued.input.generation||receipt.input.epoch!==issued.input.epoch)throw Error('original-turn.foreign-receipt');for(const key of Object.keys(issued.input.grant) as (keyof OriginalRetainedGrant)[])if(receipt.input.grant[key]!==issued.input.grant[key])throw Error('original-turn.foreign-grant');amount(receipt.workUnitsSpent);for(const [,axis]of currencies)amount(receipt.spent[axis]);const physical={...this.#physical};for(const [,axis]of currencies)physical[axis]=amount(physical[axis]+receipt.spent[axis]);physical.workUnitsSpent=amount(physical.workUnitsSpent+receipt.workUnitsSpent);let refused=receipt.workUnitsSpent>this.#fuel;if(receipt.nativeRemaining){for(const[axis,key]of currencies)refused ||=receipt.nativeRemaining[axis]+receipt.spent[key]!==receipt.input.grant[axis];refused ||=receipt.nativeRemaining.maximumDepth!==receipt.input.grant.maximumDepth;}for(const [axis,spent]of currencies)refused ||=receipt.spent[spent]>this.#remaining[axis];this.#physical=Object.freeze(physical);this.#outstanding=undefined;if(refused){this.#receiptRefusal=receipt;throw new OriginalOperationReceiptRefusal(receipt)}const next={...this.#remaining};for(const [axis,spent]of currencies)next[axis]-=receipt.spent[spent];this.#remaining=Object.freeze(next);this.#fuel-=receipt.workUnitsSpent;if(this.#epoch===U64)throw Error('original-turn.epoch-exhausted');this.#epoch+=1n}
 close():void{if(this.#outstanding||this.#receiptRefusal||this.#producerRefusal!==undefined||this.#nativeRefusal!==undefined)throw Error('original-turn.original-return-pending');this.#closed=true}
}
/** 📦️ Lowers only the issued original input to the component's u64 record. */
export function originalTurnWit(input:OriginalRetainedInput){return {operation:input.operation,generation:input.generation,epoch:input.epoch,grant:{maximumItems:BigInt(input.grant.maximumItems),maximumCopyBytes:BigInt(input.grant.maximumCopyBytes),maximumCapacityBytes:BigInt(input.grant.maximumCapacityBytes),maximumReleaseBytes:BigInt(input.grant.maximumReleaseBytes),maximumDepth:BigInt(input.grant.maximumDepth)}}}

/** 🌐️ Embeds the same defining original source into the actual component worker without a second implementation. */
export function originalOperationWorkerSource():string{return `const U64=0xffffffffffffffffn;const currencies=${JSON.stringify(currencies)};${amount.toString()}\n${identity.toString()}\n${grant.toString()}\n${OriginalOperationReceiptRefusal.toString()}\n${OriginalOperationSource.toString()}\n${originalTurnWit.toString()}\n${originalReceiptFromWit.toString()}`;}

/** 🧾️ Preserves the native exact receipt before any semantic browser projection. */
export function originalReceiptFromWit(receipt:OriginalWireReceipt,workUnitsSpent:bigint):OriginalOperationReceipt{const extent=(value:bigint):number=>{if(typeof value!=="bigint"||value<0n||value>BigInt(Number.MAX_SAFE_INTEGER))throw Error("original-turn.native-extent-refused");return Number(value)};const raw=receipt.input,input={operation:raw.operation,generation:raw.generation,epoch:raw.epoch,grant:{maximumItems:extent(raw.grant.maximumItems),maximumCopyBytes:extent(raw.grant.maximumCopyBytes),maximumCapacityBytes:extent(raw.grant.maximumCapacityBytes),maximumReleaseBytes:extent(raw.grant.maximumReleaseBytes),maximumDepth:extent(raw.grant.maximumDepth)}};const spent={copiedItems:extent(receipt.spent.copiedItems),copiedBytes:extent(receipt.spent.copiedBytes),retainedCapacityBytes:extent(receipt.spent.retainedCapacityBytes),releasedBytes:extent(receipt.spent.releasedBytes)},remaining=receipt.remaining;const nativeRemaining=remaining?{maximumItems:extent(remaining.maximumItems),maximumCopyBytes:extent(remaining.maximumCopyBytes),maximumCapacityBytes:extent(remaining.maximumCapacityBytes),maximumReleaseBytes:extent(remaining.maximumReleaseBytes),maximumDepth:extent(remaining.maximumDepth)}:undefined;return {input,spent,workUnitsSpent:extent(workUnitsSpent),nativeReceipt:receipt,nativeRemaining}}
