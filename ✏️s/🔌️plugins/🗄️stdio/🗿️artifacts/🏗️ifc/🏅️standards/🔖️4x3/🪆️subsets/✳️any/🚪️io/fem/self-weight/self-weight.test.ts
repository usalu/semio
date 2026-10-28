/** 🏋️ Released per-case weight ratios map only the actual FEM boolean domain. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {applyPatch} from 'fast-json-patch';
import cases from './cases.json';
import schema from './schema.json';
import flags from './commands.json';
import commandSchema from './commands.schema.json';
import {fem3dToIfc4x3,ifc4x3ToFem3d} from '../../fem.ts';
import {femConnectivityFixture} from '../../🧪️tests/fem-isolated-fixtures.ts';
import {ifcReal,parseIfc4x3Step} from '../../step.ts';
import {writeIfc4x3Step} from '../../step-writer.ts';
import {ifcMutationDiffAsync,applyIfcMutationAsync} from '../../../🧬️schema/🧬️mutations/🟦️.ts';
import {inverseIfcDiffAsync,applyIfcDiffAsync} from '../../../🧬️schema/🔺️diff/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../../../🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../control.ts';

const source=(selfWeight:boolean)=>{const model=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:['b']});model.loadCases[0]!.selfWeight=selfWeight;return model;};
const loadCase=(graph:IfcSnapshot)=>graph.entities.find(row=>row.name==='IFCSTRUCTURALLOADCASE')!;
const aggregate=(values:IfcValue[]):IfcValue=>({kind:'aggregate',value:values});
const exported=async()=>{const result=await fem3dToIfc4x3(source(false));if(!result.artifact)throw Error('Base export');return result.artifact;};
const vector=(row:typeof cases.cases[number]):IfcValue|undefined=>{
 if(row.kind==='missing')return undefined;if(row.kind==='unset'||row.kind==='derived')return {kind:row.kind};if(row.kind==='scalar')return ifcReal(-1);
 const items=(row.values??[]).map(n=>row.kind==='integer'?{kind:'integer' as const,value:BigInt(n)}:row.kind==='typed'||row.kind==='wrong-typed'?{kind:'typedValue' as const,value:{name:row.kind==='typed'?'IFCRATIOMEASURE':'IFCLENGTHMEASURE',items:[ifcReal(n)]}}:ifcReal(n));if(row.kind==='component-unset')items[1]={kind:'unset'};return aggregate(items);
};
test('closed language-neutral self-weight cases',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
test('closed shared canonical flag transitions',()=>expect(new Ajv({strict:true}).compile(commandSchema)(flags)).toBe(true));
for(const [ordinal,selfWeight] of flags.entries())test('actual self-weight round trip '+ordinal+' '+selfWeight,async()=>{
 const model=source(selfWeight),before=structuredClone(model),outcome=await fem3dToIfc4x3(model);expect(outcome.artifact).toBeDefined();if(!outcome.artifact)return;const graph=outcome.artifact;
 expect(loadCase(graph).args[10]).toEqual(selfWeight?aggregate([ifcReal(0),ifcReal(0),ifcReal(-1)]):{kind:'unset'});expect(loadCase(graph).args[8]).toEqual(ifcReal(1));expect(graph.entities.filter(row=>row.name==='IFCSTRUCTURALPOINTACTION')).toHaveLength(1);
 const restored=await ifc4x3ToFem3d(await parseIfc4x3Step(await writeIfc4x3Step(graph)));expect(restored.artifact?.loadCases[0]?.selfWeight).toBe(selfWeight);expect(restored.artifact?.loadCases[0]?.loads).toHaveLength(1);expect(model).toEqual(before);
});
test('source self-weight requires a boolean rather than truthiness',async()=>{const model=source(false);(model.loadCases[0] as unknown as {selfWeight:unknown}).selfWeight='false';const result=await fem3dToIfc4x3(model);expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe('fem.self-weight');});
test('self-weight capture excludes later caller mutation',async()=>{const model=source(true),result=await fem3dToIfc4x3(model,{progress(progress){if(progress.phase==='fem-export')model.loadCases[0]!.selfWeight=false;}});expect(result.artifact).toBeDefined();if(result.artifact)expect((await ifc4x3ToFem3d(result.artifact)).artifact?.loadCases[0]?.selfWeight).toBe(true);});
for(const fault of ['factor','density-unit','area-unit'])test('self-weight preserves independent '+fault+' refusal',async()=>{const graph=await exported();loadCase(graph).args[10]=aggregate([ifcReal(0),ifcReal(0),ifcReal(-1)]);if(fault==='factor')loadCase(graph).args[8]=ifcReal(2);else{const name=fault==='density-unit'?'MassDensity':'CrossSectionArea',property=graph.entities.find(row=>row.name==='IFCPROPERTYSINGLEVALUE'&&row.args[0]?.kind==='string'&&row.args[0].value===name)!;property.args[3]={kind:'unset'};}const result=await ifc4x3ToFem3d(graph);expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe(fault==='factor'?'fem.load-factor':'exchange.reference');});
for(const row of cases.cases)test('foreign self-weight '+row.id,async()=>{
 const graph=await exported(),value=vector(row),owner=loadCase(graph);if(value===undefined)owner.args.pop();else owner.args[10]=value;const before=structuredClone(graph),outcome=await ifc4x3ToFem3d(graph);
 if('expected'in row)expect(outcome.artifact?.loadCases[0]?.selfWeight).toBe(row.expected);else{expect(outcome.artifact).toBeUndefined();expect(outcome.diagnostics.some(row=>row.code==='fem.self-weight')).toBe(true);}expect(graph).toEqual(before);
});
test('self-weight-only and mixed cases keep original case ownership',async()=>{
 const model=source(true);model.loadCases[0]!.loads=[];model.loadCases.push({...model.loadCases[0]!,id:'live',name:'Live',selfWeight:false});const result=await fem3dToIfc4x3(model);expect(result.artifact).toBeDefined();if(!result.artifact)return;const restored=await ifc4x3ToFem3d(result.artifact);expect(restored.artifact?.loadCases.map(row=>({name:row.name,selfWeight:row.selfWeight,loads:row.loads.length}))).toEqual([{name:model.loadCases[0]!.name,selfWeight:true,loads:0},{name:'Live',selfWeight:false,loads:0}]);expect(result.artifact.entities.some(row=>row.name==='IFCSTRUCTURALPOINTACTION')).toBe(false);
});
test('canonical IFC edit diff and inverse change only the declared case ratio',async()=>{
 const graph=await exported(),owner=loadCase(graph),value=aggregate([ifcReal(0),ifcReal(0),ifcReal(-1)]),mutation={mutation:'setEntityArg' as const,id:owner.id,index:10,value},diff=await ifcMutationDiffAsync(graph,mutation),next=await applyIfcMutationAsync(graph,mutation),wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,v)=>typeof v==='bigint'?String(v):v)),ordinal=graph.entities.findIndex(row=>row.id===owner.id);
 expect(wire(next)).toEqual(applyPatch(wire(graph),[{op:'replace',path:'/entities/'+ordinal+'/args/10',value:wire(value)}],true,false).newDocument);expect((await ifc4x3ToFem3d(next)).artifact?.loadCases[0]?.selfWeight).toBe(true);const restored=await applyIfcDiffAsync(next,await inverseIfcDiffAsync(graph,diff));expect(restored).toEqual(graph);expect((await ifc4x3ToFem3d(restored)).artifact?.loadCases[0]?.selfWeight).toBe(false);
});
for(const direction of ['export','import'])for(const fault of ['abort','throw'])test(direction+' self-weight original '+fault,async()=>{
 const model=source(true),graph=await exported();loadCase(graph).args[10]=aggregate([ifcReal(0),ifcReal(0),ifcReal(-1)]);const abort=new AbortController(),failure={original:true};let called=0,caught=false;const control:IfcIoControl={signal:abort.signal,batchSize:1,progress(progress){if(progress.phase!=='fem-self-weight')return;called++;control.signal=new AbortController().signal;control.progress=()=>{};if(fault==='abort')abort.abort();else throw failure;}};
 try{if(direction==='export')await fem3dToIfc4x3(model,control);else await ifc4x3ToFem3d(graph,control);}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}expect(called).toBe(1);expect(caught).toBe(true);
});
