import {deepStrictEqual,strictEqual,throws} from 'node:assert/strict';
import fixture from './📜️fixtures/🔣️.json' with {type:'json'};
import {NumericIndex} from '../🟦️.ts';
/** 🧮️ Shared operations and funded paged lifecycle retain exact ordered-index behavior. */
export function testNumericScratch(oracle:(operations:typeof fixture.operations)=>unknown[]){
 const index=new NumericIndex<number>(),output:unknown[]=[];
 for(const row of fixture.operations){const key='key' in row?row.key!:0;switch(row.kind){case'set':output.push(index.set(key,row.value!)??null);break;case'get':output.push(index.get(key)??null);break;case'remove':output.push(index.remove(key)??null);break;case'first':output.push(index.popFirst()??null);break;case'reset':index.reset();output.push(null);}}
 deepStrictEqual(output,fixture.expected);deepStrictEqual(output,JSON.parse(JSON.stringify(oracle(fixture.operations))));
 const retained=new NumericIndex<number>();let allocations=0;
 for(let key=0;key<fixture.retained.keys;key++){
  if(!retained.hasReservedSlot){throws(()=>retained.setReserved(key,key),/reserved/);for(let turn=0;!retained.hasReservedSlot;turn++){const quote=retained.nextAllocationBytes();strictEqual(quote>0,true);strictEqual(retained.reserveOne(quote-1).progressed,false);const progress=retained.reserveOne(quote);strictEqual(progress.progressed,true);strictEqual(progress.allocatedBytes,quote);allocations++;if(turn>32)throw Error('reserve stalled');}}
  strictEqual(retained.setReserved(key,key),undefined);
 }
 for(let key=0;key<fixture.retained.keys;key++)strictEqual(retained.get(key),key);
 strictEqual(retained.depth<=16,true);const stored=retained.storedKeys;retained.reset();for(let key=stored-1;key>=0;key--)retained.setReserved(key,key*2);strictEqual(retained.storedKeys,stored);
 let turns=0,released=0;while(!retained.terminalIsEmpty()){const quote=retained.nextCloseReleaseByteDemand(),copy=retained.nextCloseCopyByteDemand();if(copy>0)strictEqual(retained.closeCopyStepFunded(1,copy-1,quote).progressed,false);strictEqual(retained.closeCopyStepFunded(0,copy,quote).progressed,false);if(quote>0)strictEqual(retained.closeCopyStepFunded(1,copy,quote-1).progressed,false);const result=retained.closeCopyStepFunded(1,copy,quote);strictEqual(result.progressed,true);released+=result.releasedAllocationBytes;if(++turns>fixture.maximumCloseTurns)throw Error('close stalled');}
 strictEqual(retained.storedKeys,0);strictEqual(released>0,true);strictEqual(retained.allocatedBytes,0);return{operations:output.length,keys:stored,allocations,closeTurns:turns,releasedCapacityBytes:released};
}
