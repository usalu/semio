/** 🧪️ Borrowed relationship preparation preserves source order, exact role pointers and deferred refusals. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './cases.json';
import schema from './schema.json';
import {sqliteRelationOracle} from './oracle.ts';
import {ifc4x3Catalog} from '../../../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {runIfcIoSteps,type IfcIoControl} from '../../../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/control.ts';
import type {IfcRelationPlan} from './🟦️.ts';
import type {IfcQueryEntity} from '../🟦️.ts';
const load=()=>import('./🟦️.ts'),plan=cases.plan as IfcRelationPlan;
const drain=<T>(g:Generator<void,T,void>):T=>{for(;;){const row=g.next();if(row.done)return row.value;}};
const owner=(rows:readonly IfcQueryEntity<unknown>[])=>{const index=new Map(rows.map((entity,ordinal)=>[entity,{entity,ordinal}]));return{rows,indexed:(entity:IfcQueryEntity<unknown>)=>index.get(entity)};};
const reader={reference(value:unknown):string|undefined{return value!==null&&typeof value==='object'&&(value as {kind?:string}).kind==='reference'?(value as {value:string}).value:undefined;},aggregate(value:unknown):readonly unknown[]|undefined{return value!==null&&typeof value==='object'&&(value as {kind?:string}).kind==='aggregate'?(value as {value:unknown[]}).value:undefined;}};
test('relation preparation neutral vectors have a closed schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('borrowed source index '+row.id,async()=>{
 const {prepareIfcRelationsSteps}=await load(),source:readonly IfcQueryEntity<unknown>[]=row.entities,before=structuredClone(source),index=drain(prepareIfcRelationsSteps(ifc4x3Catalog,owner(source),plan,reader));
 expect(index.source).toBe(source);expect(index.family('groups').map(item=>item.ordinal)).toEqual(row.family);
 const hits=drain(index.relatedSteps(row.role,row.target)),actual:{ordinal:number;issue:string|null;pointer:string}[]=hits.map(hit=>({ordinal:hit.row.ordinal,issue:hit.issue,pointer:hit.query.matches[0]?.pointer??''}));
 expect(actual).toEqual(row.hits);expect(actual).toEqual(sqliteRelationOracle(ifc4x3Catalog,source,plan,row.role,row.target));
 for(const hit of hits){expect(hit.row.entity).toBe(source[hit.row.ordinal]);for(const match of hit.query.matches){const part=match.part?source[hit.row.ordinal]!.complex![match.part-1]!:source[hit.row.ordinal]!;expect(match.value).toBe(part.args[match.index]);}}
 expect(source).toEqual(before);
});
test('preparation never opens ignored payloads or resolves referenced property definitions',async()=>{
 const {prepareIfcRelationsSteps}=await load(),ignored={name:'IFCPROPERTYSINGLEVALUE',get args():unknown[]{throw new Error('ignored payload touched');}},source=[ignored];
 const index=drain(prepareIfcRelationsSteps(ifc4x3Catalog,owner(source),plan,reader));expect(index.family('groups')).toEqual([]);expect(drain(index.relatedSteps('objects','unused'))).toEqual([]);
});
test('aggregate access and each target membership remain per-step rather than one cold fanout',async()=>{
 const {prepareIfcRelationsSteps}=await load(),values=Array.from({length:1024},(_,i)=>({kind:'reference',value:String(i)})),source=[{name:'IFCRELASSIGNSTOGROUP',args:[{kind:'unset'},{kind:'unset'},{kind:'unset'},{kind:'unset'},{kind:'aggregate',value:values},{kind:'unset'},{kind:'reference',value:'g'}]}];
 let reads=0;const observed={reference(value:unknown){reads++;return reader.reference(value);},aggregate(value:unknown){reads++;return reader.aggregate(value);}},steps=prepareIfcRelationsSteps(ifc4x3Catalog,owner(source),plan,observed);let turns=0;
 for(;;){const before=reads,next=steps.next();expect(reads-before).toBeLessThanOrEqual(1);turns++;if(next.done){expect(drain(next.value.relatedSteps('objects','1023')).map(hit=>hit.row.ordinal)).toEqual([0]);break;}}
 expect(turns).toBeGreaterThan(values.length);expect(reads).toBeGreaterThan(values.length);
});
test('role query after preparation borrows the original source ordinal and rejects foreign row identity',async()=>{
 const {prepareIfcRelationsSteps}=await load(),source=cases.cases[0]!.entities,index=drain(prepareIfcRelationsSteps(ifc4x3Catalog,owner(source),plan,reader)),query=drain(index.attributeSteps(source[0]!, 'RelatedObjects','IFCRELASSIGNS'));
 expect(query.matches[0]!.pointer).toBe('/entities/0/args/4');expect(()=>drain(index.attributeSteps(structuredClone(source[0]!), 'RelatedObjects','IFCRELASSIGNS'))).toThrow();
});
for(const phase of ['prepare','lookup'])for(const at of ['initial','interior','completion'])for(const fault of ['abort','undefined','error'])test('original relation '+phase+' '+at+' '+fault,async()=>{
 const {prepareIfcRelationsSteps}=await load(),source=Array.from({length:32},()=>structuredClone(cases.cases[0]!.entities[0]!)),make=():Generator<void,unknown,void>=>phase==='prepare'?prepareIfcRelationsSteps(ifc4x3Catalog,owner(source),plan,reader):drain(prepareIfcRelationsSteps(ifc4x3Catalog,owner(source),plan,reader)).relatedSteps('objects','a');
 const batch=phase==='lookup'?8:128,events:number[]=[];await runIfcIoSteps(make(),{batchSize:batch,progress:p=>events.push(p.completed)},phase);expect(events.length).toBeGreaterThan(2);
 const target=at==='initial'?0:at==='interior'?1:events.length-1,abort=new AbortController(),failure=fault==='error'?new Error('caller'):undefined;let calls=0,redirected=0,caught=false,output:unknown;
 const control:IfcIoControl={signal:abort.signal,batchSize:batch,progress(){if(calls++!==target)return;control.signal=new AbortController().signal;control.progress=()=>redirected++;if(fault==='abort')abort.abort();else throw failure;}};
 try{output=await runIfcIoSteps(make(),control,phase);}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}
 expect(caught).toBe(true);expect(output).toBeUndefined();expect(calls).toBe(target+1);expect(redirected).toBe(0);
});
import sourceCases from './source-cases.json';
import sourceSchema from './source.schema.json';
test('borrowed source identity laws have a closed schema',()=>expect(new Ajv({strict:true}).compile(sourceSchema)(sourceCases)).toBe(true));
for(const fault of sourceCases.cases)test('prepared source locator '+fault+' cannot substitute authority',async()=>{const {prepareIfcRelationsSteps}=await load(),rows=cases.cases[0]!.entities;let reads=0;const observed={reference(value:unknown){reads++;return reader.reference(value);},aggregate(value:unknown){reads++;return reader.aggregate(value);}},source={rows,indexed:(entity:IfcQueryEntity<unknown>)=>fault==='missing'?undefined:{entity:fault==='same-content-row'?structuredClone(entity):fault==='foreign-row'?rows[1]!:entity,ordinal:fault==='wrong-ordinal'?1:0}};expect(()=>drain(prepareIfcRelationsSteps(ifc4x3Catalog,source,plan,observed))).toThrow(RangeError);expect(reads).toBe(0);});
