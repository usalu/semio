/** 🎟️ A fixed slot lease identifies its exact registry and reuse generation. */
export type FixedKeySlot=Readonly<{index:number;generation:bigint;issuer:symbol}>;

/** 🧺️ Exact key admission uses declared capacity without hashing, eviction or payload ownership. */
export class FixedKeySlots<K>{
 readonly #issuer=Symbol();
 readonly #keys:Array<K|undefined>;
 readonly #generations:bigint[];
 #size=0;
 constructor(capacity:number){if(!Number.isSafeInteger(capacity)||capacity<0)throw Error("Fixed slot capacity must be a nonnegative integer");this.#keys=Array<K|undefined>(capacity).fill(undefined);this.#generations=Array<bigint>(capacity).fill(0n);}
 get size():number{return this.#size;}
 lookup(key:K):FixedKeySlot|undefined{const index=this.#keys.findIndex(current=>current!==undefined&&(current===key||Object.is(current,key)));return index<0?undefined:{index,generation:this.#generations[index]!,issuer:this.#issuer};}
 claim(key:K):FixedKeySlot|undefined{if(key===undefined)throw Error("Undefined is not a slot key");const current=this.lookup(key);if(current)return current;const index=this.#keys.findIndex((current,index)=>current===undefined&&this.#generations[index]!<0xffffffffffffffffn);if(index<0)return undefined;this.#keys[index]=key;this.#generations[index]!++;this.#size++;return{index,generation:this.#generations[index]!,issuer:this.#issuer};}
 release(key:K,lease:FixedKeySlot):boolean{if(key===undefined||lease.issuer!==this.#issuer||!(this.#keys[lease.index]===key||Object.is(this.#keys[lease.index],key))||this.#generations[lease.index]!==lease.generation)return false;this.#keys[lease.index]=undefined;this.#size--;return true;}
}
