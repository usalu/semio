import {KernelOriginalOperationIssuer,KernelOriginalResourcePool,type OriginalOperationResources,type OriginalOperationAdmission,type OriginalRetainedGrant,type OriginalRetainedProgress,type OriginalOperationReceipt} from "../🟦️.ts";
/** 🏛️ Explicit schema-owned initial stock is separate from storage residency and never refills on a retry. */
export interface OriginalBootstrapStock {readonly version:1;readonly stock:OriginalOperationResources;readonly operation:OriginalOperationResources}
function closedObject(value:unknown,names:readonly string[]):Record<string,unknown>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("original-bootstrap.invalid-stock");const row=value as Record<string,unknown>;if(Object.keys(row).length!==names.length||names.some(name=>!Object.hasOwn(row,name)))throw Error("original-bootstrap.invalid-stock");return row;}
function resources(value:unknown):OriginalOperationResources{const row=closedObject(value,["grant","fuel","maximumElapsedUs"]),raw=closedObject(row.grant,["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]);for(const value of [...Object.values(raw),row.fuel])if(typeof value!=="number"||!Number.isSafeInteger(value)||value<0)throw Error("original-bootstrap.invalid-stock");if(typeof row.maximumElapsedUs!=="string"||!/^(0|[1-9][0-9]*)$/.test(row.maximumElapsedUs))throw Error("original-bootstrap.invalid-stock");const duration=BigInt(row.maximumElapsedUs);if(duration>0xffffffffffffffffn)throw Error("original-bootstrap.invalid-stock");return Object.freeze({grant:Object.freeze({...raw})as unknown as OriginalRetainedGrant,fuel:row.fuel as number,maximumElapsedUs:duration});}
/** 📥️ Validates all initial stock fields before the genuine composition owner creates its one pool. */
export function originalBootstrapStockFromWire(value:unknown):OriginalBootstrapStock{const row=closedObject(value,["version","stock","operation"]);if(row.version!==1)throw Error("original-bootstrap.invalid-stock");return Object.freeze({version:1,stock:resources(row.stock),operation:resources(row.operation)});}
/** 🧵️ One bootstrap owns all admitted original operation sources and fences closure until they return. */
export class KernelOriginalBootstrapStock {
 readonly #pool:KernelOriginalResourcePool;readonly #issuer:KernelOriginalOperationIssuer;readonly #live=new Map<bigint,{original:OriginalOperationAdmission;remaining:OriginalRetainedGrant;fuel:number;epoch:bigint;receipt:OriginalOperationReceipt|undefined;refusal:unknown}>();#closed=false;
 constructor(readonly configuration:OriginalBootstrapStock){this.#pool=new KernelOriginalResourcePool(configuration.stock);this.#issuer=new KernelOriginalOperationIssuer(request=>this.#pool.admit(request));}
 get fuelRemaining():number{return this.#pool.fuelRemaining}
 get retainedRemaining():OriginalRetainedGrant{return this.#pool.remaining}
 get liveSources():number{return this.#live.size}
 admit():OriginalOperationAdmission{if(this.#closed)throw Error("original-bootstrap.closed");const original=this.#issuer.admit(this.configuration.operation);this.#live.set(original.operation,{original,remaining:original.grant,fuel:original.fuel,epoch:0n,receipt:undefined,refusal:undefined});return original;}
 /** 🧾️ The original root receives each child delta once without constructing another executing Source. */
 receive(original:OriginalOperationAdmission,receipt:OriginalOperationReceipt):void{const record=this.#live.get(original.operation);if(!record||record.original!==original||record.refusal!==undefined)throw Error("original-bootstrap.foreign-admission");const input=receipt.input;if(input.operation!==original.operation||input.generation!==original.generation||input.epoch!==record.epoch+1n)throw Error("original-bootstrap.foreign-receipt");for(const axis of Object.keys(record.remaining) as (keyof OriginalRetainedGrant)[])if(input.grant[axis]!==record.remaining[axis])throw Error("original-bootstrap.foreign-grant");const pairs=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]] as const;record.receipt=receipt;if(!Number.isSafeInteger(receipt.workUnitsSpent)||receipt.workUnitsSpent<0||receipt.workUnitsSpent>record.fuel||pairs.some(([axis,key])=>!Number.isSafeInteger(receipt.spent[key])||receipt.spent[key]<0||receipt.spent[key]>record.remaining[axis]||(receipt.nativeRemaining!==undefined&&receipt.nativeRemaining[axis]+receipt.spent[key]!==record.remaining[axis]))||(receipt.nativeRemaining!==undefined&&receipt.nativeRemaining.maximumDepth!==record.remaining.maximumDepth)){record.refusal=receipt;throw Error("original-bootstrap.receipt-refused")}const next={...record.remaining};for(const[axis,key]of pairs)next[axis]-=receipt.spent[key];record.remaining=Object.freeze(next);record.fuel-=receipt.workUnitsSpent;record.epoch=input.epoch;}
 hold(original:OriginalOperationAdmission,refusal:unknown):void{const record=this.#live.get(original.operation);if(!record||record.original!==original||record.refusal!==undefined||refusal===undefined)throw Error("original-bootstrap.foreign-admission");record.refusal=refusal;}
 /** 🤝️ The exact initiating admission remains held until its same worker reports full source closure. */
 returnClosed(original:OriginalOperationAdmission,sourceClosed:boolean):void{if(this.#live.get(original.operation)?.original!==original)throw Error("original-bootstrap.foreign-admission");if(!sourceClosed||this.#live.get(original.operation)?.refusal!==undefined)throw Error("original-bootstrap.source-held");this.#live.delete(original.operation);}
 /** ♻️ Separately funded logical pool metadata close follows all original sources; managed heap release is outside this receipt. */
 closeStep(grant:OriginalRetainedGrant):{complete:boolean;spent:OriginalRetainedProgress}{const zero={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};if(this.#closed)return{complete:true,spent:zero};if(this.#live.size||grant.maximumItems<1||grant.maximumDepth<1)return{complete:false,spent:zero};this.#closed=true;return{complete:true,spent:{...zero,copiedItems:1}};}
}

/** 🏛️ Mandatory original owner port supports actual cross-realm admission and receiving. */
export abstract class OriginalBootstrapOwner {
 abstract admit():Promise<OriginalOperationAdmission>;
 abstract receive(original:OriginalOperationAdmission,receipt:OriginalOperationReceipt):Promise<void>;
 abstract hold(original:OriginalOperationAdmission,refusal:unknown):Promise<void>;
 abstract returnClosed(original:OriginalOperationAdmission,sourceClosed:boolean):Promise<void>;
}
/** 🌳️ Actual root composition forwards the same initiating pool through the asynchronous owner port. */
export class RootOriginalBootstrapOwner extends OriginalBootstrapOwner {
 constructor(readonly stock:KernelOriginalBootstrapStock){super()}
 async admit():Promise<OriginalOperationAdmission>{return this.stock.admit()}
 async receive(original:OriginalOperationAdmission,receipt:OriginalOperationReceipt):Promise<void>{this.stock.receive(original,receipt)}
 async hold(original:OriginalOperationAdmission,refusal:unknown):Promise<void>{this.stock.hold(original,refusal)}
 async returnClosed(original:OriginalOperationAdmission,sourceClosed:boolean):Promise<void>{this.stock.returnClosed(original,sourceClosed)}
}
