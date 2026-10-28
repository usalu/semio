/** 🏷️ Exchange construction never recycles identity merely because source rows were retired or reordered. */
import{expect,test}from'bun:test';
import Ajv from'ajv';
import cases from'./builder-identity-cases.json';
import schema from'./builder-identity.schema.json';
import{sqliteBuilderIdentity}from'./builder-identity-oracle.ts';
import{createIfc4x3ExchangeGraph}from'../gltf.ts';
test('exchange builder identity programs have a closed neutral schema',()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,sourceValidity:true})).toBe(false);});
for(const row of cases.programs)test('actual builder identities match independent SQLite '+row.id,()=>{const graph=createIfc4x3ExchangeGraph(row.id),actual:number[]=[];expect(graph.snapshot.entities).toHaveLength(cases.initialRows);for(const step of row.steps)if(step.operation==='add'){const ref=graph.add('IFCCARTESIANPOINT',[{kind:'aggregate',value:[]}]);expect(ref.kind).toBe('reference');if(ref.kind==='reference')actual.push(Number(ref.value));}else if(step.operation==='remove')graph.snapshot.entities=graph.snapshot.entities.filter(entity=>entity.id!==BigInt(step.id));else graph.snapshot.entities.reverse();const expected=row.steps.filter(step=>step.operation==='add').map(step=>step.id);expect(sqliteBuilderIdentity.observe(cases.initialRows,row.steps)).toEqual(expected);expect(actual).toEqual(expected);expect(new Set(graph.snapshot.entities.map(entity=>entity.id)).size).toBe(graph.snapshot.entities.length);});
