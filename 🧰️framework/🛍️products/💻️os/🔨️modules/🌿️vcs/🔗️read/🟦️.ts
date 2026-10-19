export type HistoryKey = Readonly<{ledger: object; index: number; generation: number}>;
export class OriginalHistoryLedger<T> {
 private readonly authority = {};
 private readonly slots: Array<{generation: number; value: T | undefined; readers: number; sealed: boolean}> = [];
 insert(value: T): HistoryKey {const index=this.slots.findIndex(slot=>slot.value===undefined);if(index>=0){const slot=this.slots[index]!;slot.generation++;slot.value=value;return {ledger:this.authority,index,generation:slot.generation};}this.slots.push({generation:1,value,readers:0,sealed:false});return {ledger:this.authority,index:this.slots.length-1,generation:1};}
 private slot(key: HistoryKey) {if(key.ledger!==this.authority)throw new Error('foreign-ledger');const slot=this.slots[key.index];if(!slot||slot.generation!==key.generation||slot.value===undefined)throw new Error('stale-generation');return slot;}
 read(key: HistoryKey): T {return this.slot(key).value!;}
 seal(key: HistoryKey, admitted: boolean): boolean {const slot=this.slot(key);if(!admitted)return false;slot.sealed=true;return true;}
 project<U>(key: HistoryKey, project: (value:T)=>U): {value:U; close:()=>void} {const slot=this.slot(key);if(!slot.sealed)throw new Error('read-before-birth');slot.readers++;let closed=false;return {value:project(slot.value!),close:()=>{if(closed)throw new Error('replayed-return');closed=true;slot.readers--;}};}
 returnOriginal(key: HistoryKey): T {const slot=this.slot(key);if(slot.readers!==0)throw new Error('return-with-live-projection');slot.sealed=false;return slot.value!;}
 remove(key: HistoryKey): T {const slot=this.slot(key);if(slot.sealed||slot.readers!==0)throw new Error('remove-while-read');const value=slot.value!;slot.value=undefined;return value;}
 write(key:HistoryKey, write:(value:T)=>void):void {const slot=this.slot(key);if(slot.sealed||slot.readers!==0)throw new Error('write-while-read');write(slot.value!);}
}
