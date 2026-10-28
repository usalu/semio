/** 🧪️ Real private-graph BIM/FEM traversal must honor the original source and every semantic fanout. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './traversal-cases.json';
import schema from './traversal.schema.json';
import {bimSpatialToIfc4x3,ifc4x3ToBimSpatial} from '../bim.ts';
import {ifc4x3ToFem3d} from '../fem.ts';
import {ifcExchangeSourceFixture} from './exchange-source-fixtures.ts';
import {ifcReal,type IfcIoControl} from '../step.ts';
import {IfcStepError} from '../control.ts';
import type {IfcSnapshot,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {applyIfcMutationAsync} from '../../🧬️schema/🧬️mutations/🟦️.ts';
import {applyIfcDiffAsync,inverseIfcDiffAsync} from '../../🧬️schema/🔺️diff/🟦️.ts';
const aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),str=(value:string):IfcValue=>({kind:'string',value}),unset:IfcValue={kind:'unset'};
async function fixture(kind:string):Promise<IfcSnapshot>{if(kind==='fem')return(await ifcExchangeSourceFixture('ifc-to-fem')).snapshot;const result=await bimSpatialToIfc4x3({schema:'s.bim.model@1',project:{name:'Probe',description:'',author:'',organization:'',phase_names:[]},sites:{site:{name:'Site',latitude:47.5,longitude:8.5,elevation:400,true_north:0,boundary:[]}},buildings:{building:{name:'Building',site:'site',origin:{x:1,y:2},rotation:.3,elevation:4}},storeys:{g:{name:'Ground',building:'building',level:0,height:3,cut_height:1.2},u:{name:'Upper',building:'building',level:1,height:3}}});if(!result.artifact)throw Error('Source refused');return result.artifact;}
const project=(kind:string,source:IfcSnapshot,control:IfcIoControl={})=>kind==='fem'?ifc4x3ToFem3d(source,control):ifc4x3ToBimSpatial(source,control);
function irrelevant(source:IfcSnapshot,count:number):void{let id=source.entities.reduce((n,row)=>row.id>n?row.id:n,0n);for(let i=0;i<count;i++)source.entities.push({id:++id,name:'IFCCARTESIANPOINT',args:[aggregate([ifcReal(0),ifcReal(0),ifcReal(0)])],complex:[]});}
function extraProperties(kind:string,source:IfcSnapshot,count:number):void{const owner=kind==='fem'?source.entities.find(row=>row.name==='IFCMATERIALPROPERTIES'):undefined,items:IfcValue[]=owner?(owner.args[2]as Extract<IfcValue,{kind:'aggregate'}>).value:[];let id=source.entities.reduce((n,row)=>row.id>n?row.id:n,0n);for(let i=0;i<count;i++){source.entities.push({id:++id,name:'IFCPROPERTYSINGLEVALUE',args:[str('Property'+i),unset,{kind:'typedValue',value:{name:kind==='fem'?'IFCLABEL':'IFCTEXT',items:[str('original')]}},unset],complex:[]});items.push({kind:'reference',value:id});}if(kind==='bim'){const holder=source.entities.find(row=>row.name==='IFCBUILDINGSTOREY')!,set=++id;source.entities.push({id:set,name:'IFCPROPERTYSET',args:[str(String(set).padStart(22,'0')),unset,str('Fanout'),unset,aggregate(items)]});const relation=++id;source.entities.push({id:relation,name:'IFCRELDEFINESBYPROPERTIES',args:[str(String(relation).padStart(22,'0')),unset,unset,unset,aggregate([{kind:'reference',value:holder.id}]),{kind:'reference',value:set}]});}}
test('source traversal endpoint laws have a closed neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const kind of cases.endpoints){
 test(kind+' prepares one borrowed relation index without source-sized synchronous filters',async()=>{
  const source=await fixture(kind);irrelevant(source,cases.irrelevantRows);const filter=Array.prototype.filter,scans:number[]=[];const phases=new Map<string,number>();
  Array.prototype.filter=function<T>(predicate:(value:T,index:number,array:T[])=>unknown,thisArg?:unknown):T[]{const stack=new Error().stack??'',selected=stack.includes('io/'+kind+'.ts')||stack.includes('io\\\\'+kind+'.ts');if(selected&&this.length>=source.entities.length)scans.push(this.length);return filter.call(this,predicate,thisArg);};
  let result;try{result=await project(kind,source,{batchSize:1,progress(p){phases.set(p.phase,(phases.get(p.phase)??0)+1);}});}finally{Array.prototype.filter=filter;}
  expect(result.artifact).toBeDefined();expect(scans).toEqual([]);expect(phases.get(cases.phases.prepare)).toBeGreaterThan(source.entities.length);expect(phases.get(cases.phases.lookup)).toBeGreaterThan(0);
 });
 test(kind+' property fanout yields per original batch and preserves declared semantics',async()=>{
  const source=await fixture(kind),before=await project(kind,source);extraProperties(kind,source,cases.propertyRows);let propertyTurns=0;const phase=kind==='fem'?cases.phases.femProperties:cases.phases.bimOccurrenceProperties;
  const next=await project(kind,source,{batchSize:1,progress(p){if(p.phase===phase)propertyTurns++;}});if(kind==='bim'){expect(next.artifact).toBeDefined();const{properties,...spatial}=next.artifact as NonNullable<Awaited<ReturnType<typeof ifc4x3ToBimSpatial>>['artifact']>,holder=source.entities.find(row=>row.name==='IFCBUILDINGSTOREY')!;expect(spatial).toEqual(before.artifact as typeof spatial);expect(properties?.['ifc'+holder.id]?.Fanout).toEqual(Object.fromEntries(Array.from({length:cases.propertyRows},(_,index)=>['Property'+index,{Text:{value:'original'}}])));}else expect(next.artifact).toEqual(before.artifact);expect(propertyTurns).toBeGreaterThanOrEqual(cases.propertyRows);
 });
 for(const frontier of cases.frontiers)for(const fault of cases.failures)test(kind+' relation '+frontier+' original '+fault+' cannot publish',async()=>{
  const source=await fixture(kind),events:number[]=[];await project(kind,source,{batchSize:8,progress(p){if(p.phase===cases.phases.prepare)events.push(p.completed);}});expect(events.length).toBeGreaterThan(2);
  const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,abort=new AbortController(),failure=fault==='error'?new IfcStepError('io.cancelled','caller'):undefined;let calls=0,redirected=0,caught=false,artifact:unknown;
  const control:IfcIoControl={signal:abort.signal,batchSize:8,progress(p){if(p.phase!==cases.phases.prepare||calls++!==target)return;control.signal=new AbortController().signal;control.progress=()=>redirected++;if(fault==='abort')abort.abort();else throw failure;}};
  try{artifact=(await project(kind,source,control)).artifact;}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}
  expect(caught).toBe(true);expect(artifact).toBeUndefined();expect(calls).toBe(target+1);expect(redirected).toBe(0);
 });
 test(kind+' original source mutation during prepared semantic callbacks cannot rewrite private rows',async()=>{const source=await fixture(kind),before=await project(kind,source);let mutated=false;const result=await project(kind,source,{batchSize:8,progress(p){if(p.phase===cases.phases.prepare&&!mutated){mutated=true;source.entities.length=0;}}});expect(mutated).toBe(true);expect(result.artifact).toEqual(before.artifact);});
 test(kind+' exact central property edit/inverse survives indexed projection',async()=>{
  const source=await fixture(kind),before=await project(kind,source),name=kind==='fem'?'YoungModulus':'CutHeight',property=source.entities.find(row=>row.name==='IFCPROPERTYSINGLEVALUE'&&row.args[0]?.kind==='string'&&row.args[0].value===name)!,value=structuredClone(property.args[2])as Extract<IfcValue,{kind:'typedValue'}>;value.value.items[0]=ifcReal(kind==='fem'?210000000000:1.3);
  const edited=await applyIfcMutationAsync(source,{mutation:'setEntityArg',id:property.id,index:2,value}),next=await project(kind,edited);
  const diff={entities:{modified:[{id:property.id,diff:{args:{modified:[{index:2,value}]}}}]}},inverse=await inverseIfcDiffAsync(source,diff),restored=await applyIfcDiffAsync(edited,inverse);
  expect(next.artifact).toBeDefined();expect(next.artifact).not.toEqual(before.artifact);expect(restored).toEqual(source);expect((await project(kind,restored)).artifact).toEqual(before.artifact);
 });
}
import refusalCases from './traversal-refusal-cases.json';
import refusalSchema from './traversal-refusal.schema.json';
test('deferred predicate/refusal corpus has a closed schema',()=>expect(new Ajv({strict:true}).compile(refusalSchema)(refusalCases)).toBe(true));
for(const row of refusalCases.cases)for(const kind of row.endpoints)test(kind+' source-order relation predicate '+row.id,async()=>{
 const source=await fixture(kind),before=await project(kind,source);let id=source.entities.reduce((n,item)=>item.id>n?item.id:n,0n);const target=source.entities.find(item=>item.name===(kind==='bim'?'IFCBUILDINGSTOREY':'IFCSTRUCTURALCURVEMEMBER'))!,definition=source.entities.find(item=>item.name==='IFCRELDEFINESBYPROPERTIES')!;
 const relation=(object:bigint,value:IfcValue,objects:IfcValue=aggregate([{kind:'reference',value:object}]))=>({id:++id,name:'IFCRELDEFINESBYPROPERTIES',args:[str('0000000000000000000000'),unset,unset,unset,objects,value],complex:[]});
 if(row.id==='ignored-unrelated-payload'){const foreign=source.entities.find(item=>item.name==='IFCCARTESIANPOINT')!;source.entities.push(relation(foreign.id,{kind:'integer',value:1n}));}
 if(row.id==='first-predicate-refusal-before-late-aggregate'){source.entities.push(relation(target.id,{kind:'integer',value:1n}));source.entities.push(relation(target.id,definition.args[5]!,unset));}
 if(row.id==='duplicate-related-target-occurrence'){const values=definition.args[4]as Extract<IfcValue,{kind:'aggregate'}>;values.value.push(structuredClone(values.value[0]!));}
 if(row.id==='duplicate-aggregate-parent'){const copy=structuredClone(source.entities.find(item=>item.name==='IFCRELAGGREGATES')!);copy.id=++id;source.entities.push(copy);}
 if(row.id==='conflicting-engineering-property'){const property=structuredClone(source.entities.find(item=>item.name==='IFCPROPERTYSINGLEVALUE'&&item.args[0]?.kind==='string'&&item.args[0].value==='YoungModulus')!);property.id=++id;source.entities.push(property);const properties=source.entities.find(item=>item.name==='IFCMATERIALPROPERTIES')!.args[2]as Extract<IfcValue,{kind:'aggregate'}>;properties.value.push({kind:'reference',value:property.id});}
 const result=await project(kind,source,{batchSize:1});expect(!!result.artifact).toBe(row.artifact);if(row.artifact)expect(result.artifact).toEqual(before.artifact);else expect(result.diagnostics[0]!.message).toContain(row.message!);
});
