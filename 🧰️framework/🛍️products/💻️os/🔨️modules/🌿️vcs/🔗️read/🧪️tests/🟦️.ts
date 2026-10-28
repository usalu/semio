import Ajv from 'ajv';
import jsonPatch from 'fast-json-patch';
import corpus from './🧫️fixtures/🔣️.json';
import schema from '../🧬️schema/🔣️.json';
import projectionCorpus from './🧫️fixtures/🔗️projection.json';
import projectionSchema from '../🧬️schema/🔗️projection.json';
import snapshotCorpus from './🧫️fixtures/📸️snapshot.json';
import snapshotSchema from '../🧬️schema/📸️snapshot.json';
import { OriginalHistoryLedger } from '../🟦️.ts';
export function verifyOriginalHistoryRead() {
 const validate=new Ajv({strict:true}).compile(schema);if(!validate(corpus))throw new Error(JSON.stringify(validate.errors));
 for(const law of corpus.cases){const ledger=new OriginalHistoryLedger<typeof law.operations>();const original=law.operations;const key=ledger.insert(original);if(ledger.seal(key,false))throw new Error('denied birth');if(ledger.read(key)!==original)throw new Error('original root changed');ledger.seal(key,true);const reads=law.readOrder.map(index=>ledger.project(key,ops=>ops[index]!));for(let i=0;i<reads.length;i++)if(reads[i]!.value!==original[law.readOrder[i]!])throw new Error('original operation changed');if(reads.length){let refused=false;try{ledger.returnOriginal(key);}catch{refused=true;}if(!refused)throw new Error('live projection return');}for(const read of reads)read.close();if(ledger.returnOriginal(key)!==original)throw new Error('return changed owner');const actual=jsonPatch.applyPatch(structuredClone(law.input),ledger.read(key).map(operation=>{if(operation.op!=='replace')throw new Error('unsupported original operation');return {op:'replace' as const,path:operation.path,value:operation.value};}),true,false).newDocument;if(JSON.stringify(actual)!==JSON.stringify(law.expected))throw new Error(law.id+' independent semantic output');const foreign=new OriginalHistoryLedger<typeof original>();foreign.insert(original);let refused=false;try{foreign.read(key);}catch{refused=true;}if(!refused)throw new Error('foreign authority');ledger.remove(key);ledger.insert(original);refused=false;try{ledger.read(key);}catch{refused=true;}if(!refused)throw new Error('generation reuse');}
}

/** 🧬️ Verifies original opaque projections against independent RFC 6902 and Buffer byte outputs. */
export function verifyOriginalProjectedSource() {
 const validate=new Ajv({strict:true}).compile(projectionSchema);if(!validate(projectionCorpus))throw new Error(JSON.stringify(validate.errors));
 for(const law of projectionCorpus.cases){
  const original={bytes:law.bytes};const ledger=new OriginalHistoryLedger<typeof original>();const key=ledger.insert(original);ledger.seal(key,true);const projection=ledger.project(key,value=>value.bytes);
  if(projection.value!==original.bytes)throw new Error(law.id+' original projected bytes');
  const independent=jsonPatch.applyPatch({bytes:[] as number[]},[{op:'replace',path:'/bytes',value:Array.from(Buffer.from(law.bytes))}],true,false).newDocument;
  if(JSON.stringify(projection.value)!==JSON.stringify(independent.bytes)||JSON.stringify(independent.bytes)!==JSON.stringify(law.expected))throw new Error(law.id+' independent byte output');
  let refused=false;try{ledger.returnOriginal(key);}catch{refused=true;}if(!refused)throw new Error(law.id+' projection still owns authority');
  projection.close();if(ledger.returnOriginal(key)!==original)throw new Error(law.id+' original authority returned');ledger.remove(key);
 }
}

/** 📸️ Validates the native captured-read corpus against independent exact byte outputs. */
export function verifyCapturedSnapshotSourceOracle() {
 const validate=new Ajv({strict:true}).compile(snapshotSchema);if(!validate(snapshotCorpus))throw new Error(JSON.stringify(validate.errors));
 for(const law of snapshotCorpus.cases){const independent=jsonPatch.applyPatch({bytes:[] as number[]},[{op:'replace',path:'/bytes',value:Array.from(Buffer.from(law.bytes))}],true,false).newDocument;if(JSON.stringify(independent.bytes)!==JSON.stringify(law.expected))throw new Error(law.id+' independent captured byte output');}
}

import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import links from "./🧫️fixtures/⏮️links.json";
test("original history reverse links agree with independent SQLite order",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE original_history (ordinal INTEGER PRIMARY KEY, row TEXT)");for(let index=0;index<links.rows.length;index++)db.query("INSERT INTO original_history VALUES (?, ?)").run(index,JSON.stringify(links.rows[index]));
 expect(db.query("SELECT ordinal FROM original_history ORDER BY ordinal DESC").all().map((row:any)=>row.ordinal)).toEqual(links.expected);db.query("DELETE FROM original_history WHERE ordinal = ?").run(1);expect(db.query("SELECT ordinal FROM original_history ORDER BY ordinal DESC").all().map((row:any)=>row.ordinal)).toEqual([2,0]);db.close();
});
