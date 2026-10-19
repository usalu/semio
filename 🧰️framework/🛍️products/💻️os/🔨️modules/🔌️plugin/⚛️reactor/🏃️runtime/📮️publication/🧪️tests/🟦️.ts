/** 📮️ The portable batch issuance is checked with independent SQLite event ordering and Buffer ABI. */
import {test,expect} from 'bun:test';
import {Database} from 'bun:sqlite';
import {readFileSync} from 'node:fs';
import {semioSchemaAjvV1} from '@semio-tech/schema-ts/oracles/validator';
const fixture=JSON.parse(readFileSync(new URL('../🧪️.json',import.meta.url),'utf8'));
const schema=JSON.parse(readFileSync(new URL('../🧬️.json',import.meta.url),'utf8'));
test('original native publication batch fixture preserves all five independent currencies',()=>{
 expect(semioSchemaAjvV1().compile(schema)(fixture)).toBe(true);
 expect(fixture.maximumPages).toBe(8);
 expect(fixture.receiverGrant.maximumCopyBytes).toBe(4096);
 expect(fixture.receiverGrant.maximumCapacityBytes).toBe(65536);
 expect(fixture.requiredOrder.indexOf('source-close')).toBeLessThan(fixture.requiredOrder.indexOf('publish'));
 expect(fixture.requiredOrder.indexOf('publish')).toBeLessThan(fixture.requiredOrder.indexOf('ack'));
});
test('SQLite independently keeps eight original stream identities through close publish and acknowledgement',()=>{
 const db=new Database(':memory:');
 try{
 db.exec('CREATE TABLE custody(receiver INTEGER,operation TEXT,generation TEXT,sequence INTEGER,attempt INTEGER,stage TEXT,position INTEGER)');
 const insert=db.prepare('INSERT INTO custody VALUES(?,?,?,?,?,?,?)');
 for(let operation=0;operation<fixture.maximumPages;operation++)for(const [position,stage] of fixture.requiredOrder.entries())insert.run(7,String(18446744073709551615n-BigInt(operation)),'18446744073709551615',0,1,stage,position);
 const rows=db.query('SELECT receiver,operation,generation,sequence,attempt,group_concat(stage) AS stages FROM (SELECT * FROM custody ORDER BY position) GROUP BY receiver,operation,generation,sequence,attempt').all() as {receiver:number,operation:string,generation:string,sequence:number,attempt:number,stages:string}[];
 expect(rows.length).toBe(8);
 for(const row of rows){expect(row.stages.split(',')).toEqual(fixture.requiredOrder);expect(row.generation).toBe('18446744073709551615');}
 }finally{db.close()}
});
test('Buffer ABI keeps full original u64 identities and a 4096 body beyond the small sender allocation ceiling',()=>{
 const magic=Buffer.from('semio.typed-operation-page.v1\0');
 for(const length of fixture.pageLengths){
 const header=Buffer.alloc(magic.length+30);magic.copy(header);header.writeUInt32LE(7,magic.length);header.writeBigUInt64LE(18446744073709551615n,magic.length+4);header.writeBigUInt64LE(18446744073709551615n,magic.length+12);header.writeUInt32LE(0,magic.length+20);header[magic.length+24]=1;header[magic.length+25]=15;header.writeUInt32LE(length,magic.length+26);
 const wire=Buffer.concat([header,Buffer.alloc(length,81)]);
 expect(wire.readBigUInt64LE(magic.length+4)).toBe(18446744073709551615n);expect(wire.readUInt32LE(magic.length+26)).toBe(length);expect(wire.length).toBe(header.length+length);expect(wire.length).toBeLessThanOrEqual(fixture.receiverGrant.maximumCapacityBytes);
 if(length===4096)expect(wire.length).toBeGreaterThan(fixture.receiverGrant.maximumCopyBytes);
 }
});
