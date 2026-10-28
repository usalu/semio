/** 🗺️ Canonical captured row ordinals feed real declared-owner queries and independent SQL lookups. */
import {expect,test} from 'bun:test';
import {Database} from 'bun:sqlite';
import Ajv from 'ajv/dist/2020';
import cases from './geometry-index-cases.json';
import schema from './geometry-index.schema.json';
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {IfcGeometryGraph} from '../geometry.ts';
import {runIfcIoSteps} from '../control.ts';
import {ifc4x3Catalog} from '../../🧬️schema/📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps} from '../../../../../../📚️catalog/🔎️query/🟦️.ts';
test('closed original source index contract',()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,valid:true})).toBe(false);});
for(const row of cases.rows)test('captured source ordinal and owner '+row.entity,async()=>{
 const source=createIfc4x3ExchangeGraph('Index').snapshot,graph=(await IfcGeometryGraph.capture(source)).artifact!,indexed=graph.indexed(BigInt(row.id))!;
 expect(indexed.entity.name).toBe(row.entity);expect(indexed.ordinal).toBe(row.ordinal);expect(indexed.entity===graph.snapshot.entities[row.ordinal]).toBe(cases.expected.borrowed);expect(graph.lookup(BigInt(row.id))).toBe(indexed.entity);expect(indexed.entity===source.entities[row.ordinal]).toBe(false);
 expect(Reflect.set(indexed,'ordinal',999)).toBe(cases.expected.replacement);
 const query=await runIfcIoSteps(ifcAttributeQuerySteps(ifc4x3Catalog,indexed,row.attribute,row.owner),{batchSize:1},'query');expect(query.status).toBe('unique');expect(query.matches[0]!.pointer).toBe(row.pointer);
});
test('one captured index matches independent source-ordered SQL after caller mutation',async()=>{
 const source=createIfc4x3ExchangeGraph('Index').snapshot,db=new Database(':memory:');
 try{db.exec('CREATE TABLE source (id TEXT PRIMARY KEY, ordinal INTEGER, name TEXT)');const insert=db.prepare('INSERT INTO source VALUES (?, ?, ?)');source.entities.forEach((row,ordinal)=>insert.run(String(row.id),ordinal,row.name));
  let changed=false;const outcome=await IfcGeometryGraph.capture(source,{batchSize:1,progress:event=>{if(event.phase==='index-entities'&&!changed){changed=true;source.entities.reverse();source.entities[0]!.id=999n;}}}),graph=outcome.artifact!;
  expect(changed).toBe(true);for(const row of db.query('SELECT id, ordinal, name FROM source ORDER BY ordinal').all() as {id:string;ordinal:number;name:string}[]){const entry=graph.indexed(BigInt(row.id))!;expect(entry.ordinal).toBe(row.ordinal);expect(entry.entity.name).toBe(row.name);expect(entry.entity===graph.lookup(BigInt(row.id))).toBe(true);}
  expect(graph.indexed(999n)===undefined).toBe(cases.expected.missing);expect(graph.lookup(999n)).toBeUndefined();
 }finally{db.close();}
});
