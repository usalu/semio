/** 🧪️ Optional IFC force components retain explicit authored zero records. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {applyPatch} from 'fast-json-patch';
import cases from './cases.json';
import schema from './schema.json';
import {fem3dToIfc4x3,ifc4x3ToFem3d} from '../../fem.ts';
import {femConnectivityFixture} from '../../🧪️tests/fem-isolated-fixtures.ts';
import {writeIfc4x3Step} from '../../step-writer.ts';
import {parseIfc4x3Step,ifcReal} from '../../step.ts';
import {IfcStepError,type IfcIoControl} from '../../control.ts';
import {ifcMutationDiffAsync,applyIfcMutationAsync} from '../../../🧬️schema/🧬️mutations/🟦️.ts';
import {inverseIfcDiffAsync,applyIfcDiffAsync} from '../../../🧬️schema/🔺️diff/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../../../🧬️schema/📸️snapshot/🟦️.ts';
type Dof=typeof cases.dofs[number];
const selectedDof=(dof:string)=>{switch(dof){case 'Tx':case 'Ty':case 'Tz':case 'Rx':case 'Ry':case 'Rz':return dof;default:throw Error('Undeclared DOF');}};
const raw=(bits:string):IfcValue=>({kind:'real',value:{bits:BigInt(bits)}});
const source=(dof:Dof='Tz',bits='0')=>{const model=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:['b']});model.loadCases[0]!.loads[0]={kind:'nodal',id:'explicit-zero',nodeId:'b',dof:selectedDof(dof),value:{bits:BigInt(bits)}};return model;};
async function exported(dof:Dof='Tz',bits='0'){const result=await fem3dToIfc4x3(source(dof,bits));if(!result.artifact)throw Error('Export refused');return result.artifact;}
const force=(graph:IfcSnapshot)=>graph.entities.find(row=>row.name==='IFCSTRUCTURALLOADSINGLEFORCE')!;
const projected=(graph:IfcSnapshot)=>ifc4x3ToFem3d(graph);
const components=(graph:IfcSnapshot,items:readonly {dof:string;bits:string}[])=>{const load=force(graph);for(let k=0;k<6;k++)load.args[k+1]={kind:'unset'};for(const item of items)load.args[cases.dofs.indexOf(item.dof)+1]=raw(item.bits);};
const records=(outcome:Awaited<ReturnType<typeof projected>>)=>outcome.artifact?.loadCases[0]?.loads.map(row=>{if(row.kind!=='nodal')throw Error('Unexpected load kind');return{dof:String(row.dof),bits:String(row.value.bits)};});
test('closed neutral optional-component contract',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const dof of cases.dofs)for(const value of cases.values)test('actual selected '+dof+' '+value.id,async()=>{
 const graph=await exported(dof,value.bits),load=force(graph),selected=cases.dofs.indexOf(dof);
 expect(load.args[selected+1]).toEqual(raw(value.bits));for(let k=0;k<6;k++)if(k!==selected)expect(load.args[k+1]).toEqual({kind:'unset'});
 const outcome=await projected(graph);expect(records(outcome)).toEqual([{dof,bits:value.bits}]);expect(outcome.artifact!.loadCases[0]!.loads[0]).toHaveProperty('nodeId',outcome.artifact!.nodes[1]!.id);
 const parsed=await parseIfc4x3Step(await writeIfc4x3Step(graph));expect(records(await projected(parsed))).toEqual([{dof,bits:value.bits}]);
});
for(const row of cases.vectors)test('foreign explicit component presence '+row.id,async()=>{const graph=await exported();components(graph,row.components);expect(records(await projected(graph))).toEqual(row.components);expect(records(await projected(await parseIfc4x3Step(await writeIfc4x3Step(graph))))).toEqual(row.components);});
for(const fault of cases.refusals)test('zero component refuses '+fault,async()=>{
 const graph=await exported();components(graph,[{dof:fault.includes('torque')?'Rx':'Tx',bits:'0'}]);
 if(fault==='no-components')components(graph,[]);
 if(fault==='invalid-component')force(graph).args[1]={kind:'enum',value:'T'};
 if(fault==='derived-component')force(graph).args[1]={kind:'derived'};
 if(fault==='wrong-force-unit-name')graph.entities.find(row=>row.name==='IFCSIUNIT'&&row.args[1]?.kind==='enum'&&row.args[1].value==='FORCEUNIT')!.args[3]={kind:'enum',value:'PASCAL'};
 if(fault==='scaled-force-unit')graph.entities.find(row=>row.name==='IFCSIUNIT'&&row.args[1]?.kind==='enum'&&row.args[1].value==='FORCEUNIT')!.args[2]={kind:'enum',value:'KILO'};
 if(fault==='wrong-torque-dimensions'){const torque=graph.entities.find(row=>row.name==='IFCDERIVEDUNIT'&&row.args[1]?.kind==='enum'&&row.args[1].value==='TORQUEUNIT')!,parts=torque.args[0];if(parts?.kind!=='aggregate'||parts.value[0]?.kind!=='reference')throw Error('Torque elements');graph.entities.find(row=>row.id===(parts.value[0] as Extract<IfcValue,{kind:'reference'}>).value)!.args[1]={kind:'integer',value:2n};}
 if(fault==='missing-force-unit'||fault==='missing-torque-unit'){const type=fault==='missing-force-unit'?'FORCEUNIT':'TORQUEUNIT',ids=new Set(graph.entities.filter(row=>row.args[1]?.kind==='enum'&&row.args[1].value===type).map(row=>row.id)),units=graph.entities.find(row=>row.name==='IFCUNITASSIGNMENT')!.args[0];if(units?.kind!=='aggregate')throw Error('Unit roster');units.value=units.value.filter(row=>row.kind!=='reference'||!ids.has(row.value));}
 if(fault==='duplicate-force-unit'||fault==='duplicate-torque-unit'){const type=fault==='duplicate-force-unit'?'FORCEUNIT':'TORQUEUNIT',copy=structuredClone(graph.entities.find(row=>row.args[1]?.kind==='enum'&&row.args[1].value===type)!);copy.id=graph.entities.reduce((n,row)=>row.id>n?row.id:n,0n)+1n;graph.entities.push(copy);const units=graph.entities.find(row=>row.name==='IFCUNITASSIGNMENT')!.args[0];if(units?.kind!=='aggregate')throw Error('Unit roster');units.value.push({kind:'reference',value:copy.id});}
 const before=structuredClone(graph),result=await projected(graph);expect(result.artifact).toBeUndefined();expect(result.diagnostics.some(row=>row.severity==='error')).toBe(true);expect(graph).toEqual(before);
});
for(const fault of cases.sourceRefusals)test('source zero component refuses '+fault,async()=>{const model=source(),load=model.loadCases[0]!.loads[0]!;if(load.kind!=='nodal')throw Error('Nodal');if(fault==='missing-node')load.nodeId='absent';else (load as {dof:string}).dof='invalid';const before=structuredClone(model),outcome=await fem3dToIfc4x3(model);expect(outcome.artifact).toBeUndefined();expect(model).toEqual(before);});
test('explicit zero central edit and inverse retain actual component records',async()=>{
 const graph=await exported();components(graph,[{dof:'Tz',bits:'0'}]);const load=force(graph),value=ifcReal(5),mutation={mutation:'setEntityArg' as const,id:load.id,index:3,value},diff=await ifcMutationDiffAsync(graph,mutation),next=await applyIfcMutationAsync(graph,mutation),wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?String(item):item)),ordinal=graph.entities.findIndex(row=>row.id===load.id);
 expect(wire(next)).toEqual(applyPatch(wire(graph),[{op:'replace',path:'/entities/'+ordinal+'/args/3',value:wire(value)}],true,false).newDocument);expect(records(await projected(graph))).toEqual([{dof:'Tz',bits:'0'}]);expect(records(await projected(next))).toEqual([{dof:'Tz',bits:'4617315517961601024'}]);const restored=await applyIfcDiffAsync(next,await inverseIfcDiffAsync(graph,diff));expect(restored).toEqual(graph);expect(records(await projected(restored))).toEqual([{dof:'Tz',bits:'0'}]);
});
for(const direction of ['export','import'])for(const fault of ['abort','undefined','error'])test(direction+' zero traversal original '+fault,async()=>{
 const model=source();for(let i=1;i<4;i++)model.loadCases[0]!.loads.push({...model.loadCases[0]!.loads[0]!,id:'zero'+i});const graph=(await fem3dToIfc4x3(model)).artifact!,abort=new AbortController(),failure=fault==='error'?new IfcStepError('fem.load-unit','caller'):undefined;let calls=0,caught=false,result:unknown;const control:IfcIoControl={batchSize:1,signal:abort.signal,progress(progress){if(progress.phase!==(direction==='export'?'fem-load-actions':'fem-load-components'))return;if(++calls!==3)return;control.signal=new AbortController().signal;control.progress=()=>{};if(fault==='abort')abort.abort();else throw failure;}};
 try{result=direction==='export'?await fem3dToIfc4x3(model,control):await ifc4x3ToFem3d(graph,control);}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}expect(calls).toBe(3);expect(caught).toBe(true);expect(result).toBeUndefined();
});

