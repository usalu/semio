/** 🫳️ Real IFC-source projections admit private graphs before semantic work. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './exchange-source-cases.json';
import schema from './exchange-source.schema.json';
import {ifcExchangeSourceFixture} from './exchange-source-fixtures.ts';

test('IFC source admission vectors have a closed schema',()=>{
 expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true);
});
for(const row of cases.endpoints){
 test(`${row.id} isolates semantic progress callbacks from its admitted graph`,async()=>{
  const {snapshot,project}=await ifcExchangeSourceFixture(row.id),expected=await project(snapshot);expect(expected.artifact).toBeDefined();let changed=false;
  const result=await project(snapshot,{progress:progress=>{if(progress.phase===row.phase&&!changed){changed=true;snapshot.entities=[];snapshot.header.fileSchema=[];}}});
  expect(changed).toBe(true);expect(result).toEqual(expected);
 });
 test(`${row.id} refuses duplicate identities and finite source limits before projection`,async()=>{
  const {snapshot,project}=await ifcExchangeSourceFixture(row.id),duplicate=structuredClone(snapshot);duplicate.entities.at(-1)!.id=duplicate.entities[0]!.id;
  for(const [source,control] of [[duplicate,{}],...cases.limits.map(limit=>[snapshot,limit])] as const){
   const result=await project(source,control);expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe(cases.sourceRefusal);
  }
 });
 test(`${row.id} rejects source accessors without invoking caller code`,async()=>{
  const {snapshot,project}=await ifcExchangeSourceFixture(row.id),entity=snapshot.entities[0]!,name=entity.name;let reads=0;
  Object.defineProperty(entity,'name',{get:()=>{reads++;return name;},enumerable:true,configurable:true});
  const result=await project(snapshot);expect(reads).toBe(0);expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe(cases.sourceRefusal);
 });
 test(`${row.id} cancels cooperative source capture before semantic projection`,async()=>{
  const {snapshot,project}=await ifcExchangeSourceFixture(row.id),before=structuredClone(snapshot),abort=new AbortController();let captured=false,projected=false;
  await expect(project(snapshot,{signal:abort.signal,batchSize:1,progress:progress=>{if(progress.phase===row.phase)projected=true;if(progress.phase.startsWith(cases.capturePhasePrefix)&&progress.completed>=cases.captureCancelAfter){captured=true;abort.abort();}}})).rejects.toHaveProperty('code',cases.cancellation);
  expect(captured).toBe(true);expect(projected).toBe(false);expect(snapshot).toEqual(before);
 });
 test(`${row.id} cancels cooperative graph indexing before semantic projection`,async()=>{
  const {snapshot,project}=await ifcExchangeSourceFixture(row.id),before=structuredClone(snapshot),abort=new AbortController();let indexed=false,projected=false;
  await expect(project(snapshot,{signal:abort.signal,batchSize:cases.indexCancelAfter,progress:progress=>{if(progress.phase===row.phase)projected=true;if(progress.phase===cases.indexPhase&&progress.completed>=cases.indexCancelAfter){indexed=true;abort.abort();}}})).rejects.toHaveProperty('code',cases.cancellation);
  expect(indexed).toBe(true);expect(projected).toBe(false);expect(snapshot).toEqual(before);
 });
}
