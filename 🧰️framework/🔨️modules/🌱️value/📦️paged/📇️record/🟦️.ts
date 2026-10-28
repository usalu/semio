/** 📇️ Original immutable record ownership retains only key addresses beside its one enumerable value authority. */
export class OriginalIndexedRecord<V>{readonly #keys:string[];
 private constructor(entries:Iterable<readonly[string,V]>,seal=true){this.#keys=[];for(const[key,value]of entries)OriginalIndexedRecord.#append(this,key,value);if(seal)Object.preventExtensions(this)}
 static #append<V>(owner:OriginalIndexedRecord<V>,key:string,value:V){if(Object.hasOwn(owner,key))throw Error('Duplicate original record key');owner.#keys.push(key);Object.defineProperty(owner,key,{value,enumerable:true,writable:false,configurable:false})}
 static builderCold<V>():OriginalRecordBuilder<V>{const owner=new OriginalIndexedRecord<V>([],false);let active=true;return{append(key,value){if(!active)throw Error('Original record builder is closed');OriginalIndexedRecord.#append(owner,key,value)},finish(){if(!active)throw Error('Original record builder is closed');active=false;Object.preventExtensions(owner);return owner as OriginalRecord<V>}}}
 static fromEntriesCold<V>(entries:Iterable<readonly[string,V]>):OriginalRecord<V>{return new OriginalIndexedRecord(entries)as OriginalRecord<V>}
 static is<V>(source:unknown):source is OriginalRecord<V>{return source!==null&&typeof source==='object'&&#keys in source}
 static size<V>(source:OriginalRecord<V>):number{return source.#keys.length}
 static entryAt<V>(source:OriginalRecord<V>,ordinal:number):readonly[string,V]|undefined{const key=source.#keys[ordinal];return key===undefined?undefined:[key,source[key]!]}
 static *entries<V>(source:OriginalRecord<V>):IterableIterator<readonly[string,V]>{for(const key of source.#keys)yield[key,source[key]!]}
 static replaceCold<V>(source:OriginalRecord<V>,key:string,value:V):OriginalRecord<V>{function* entries(){let found=false;for(const[held,current]of OriginalIndexedRecord.entries(source)){if(held===key){found=true;yield[held,value]as const}else yield[held,current]as const}if(!found)yield[key,value]as const}return OriginalIndexedRecord.fromEntriesCold(entries())}
 static removeCold<V>(source:OriginalRecord<V>,key:string):OriginalRecord<V>{function* entries(){for(const[held,value]of OriginalIndexedRecord.entries(source))if(held!==key)yield[held,value]as const}return OriginalIndexedRecord.fromEntriesCold(entries())}
}
export type OriginalRecord<V>=Readonly<Record<string,V>>&OriginalIndexedRecord<V>;

export interface OriginalRecordBuilder<V>{append(key:string,value:V):void;finish():OriginalRecord<V>}
