type Slot<K,V> = {key:K,value:V}|null|undefined;
/** 🗂️ Fixed native-style scratch ownership preserves tombstones and refuses growth beyond its declared limit. */
export class ScratchHashMap<K,V>{
 readonly #slots:Slot<K,V>[];readonly #maximum:number;readonly #hash:(key:K)=>number;readonly #equal:(a:K,b:K)=>boolean;#size=0;
 constructor(maximum:number,hash:(key:K)=>number=ScratchHashMap.hash,equal:(a:K,b:K)=>boolean=Object.is){
  if(!Number.isSafeInteger(maximum)||maximum<0||maximum>1048576)throw new RangeError("scratch hash maximum exceeds its fixed contract");
  let slots=maximum===0?0:2;while(slots<maximum*2)slots*=2;
  this.#slots=Array.from({length:slots},()=>undefined);this.#maximum=maximum;this.#hash=hash;this.#equal=equal;
 }
 static hash(key:unknown):number{let hash=2166136261;for(const char of String(key)){hash=Math.imul(hash^char.codePointAt(0)!,16777619);}return hash>>>0;}
 get size():number{return this.#size;}
 #find(key:K):number|undefined{
  if(this.#slots.length===0)return;
  let at=(this.#hash(key)>>>0)&(this.#slots.length-1);
  for(let probe=0;probe<this.#slots.length;probe++,at=(at+1)&(this.#slots.length-1)){
   const slot=this.#slots[at];if(slot===undefined)return;if(slot!==null&&this.#equal(slot.key,key))return at;
  }
 }
 get(key:K):V|undefined{const at=this.#find(key);return at===undefined?undefined:this.#slots[at]!.value;}
 tryInsert(key:K,value:V):{accepted:boolean,previous:V|undefined}{
  const present=this.#find(key);
  if(present!==undefined){const slot=this.#slots[present]!;const previous=slot.value;slot.value=value;return{accepted:true,previous};}
  if(this.#size===this.#maximum)return{accepted:false,previous:undefined};
  let at=(this.#hash(key)>>>0)&(this.#slots.length-1);
  for(let probe=0;probe<this.#slots.length;probe++,at=(at+1)&(this.#slots.length-1)){
   if(this.#slots[at]==null){this.#slots[at]={key,value};this.#size++;return{accepted:true,previous:undefined};}
  }
  throw new Error("scratch hash vacancy invariant violated");
 }
 remove(key:K):V|undefined{const at=this.#find(key);if(at===undefined)return;const value=this.#slots[at]!.value;this.#slots[at]=null;this.#size--;return value;}
 *entries():IterableIterator<[K,V]>{for(const slot of this.#slots)if(slot!=null)yield[slot.key,slot.value];}
}
