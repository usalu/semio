/** 📏️ Shared synchronous and controlled unit/placement evaluators retain exact original authority. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './unit-traversal-cases.json';
import schema from './unit-traversal.schema.json';
import {ifcUnitTraversalFixture} from './unit-traversal-fixtures.ts';
import {IfcGeometryGraph} from '../geometry.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
import {IfcEditError} from '../../🧬️schema/🔺️diff/🟦️.ts';

test('unit and placement traversal vectors have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.units)test('controlled unit oracle '+row.id,async()=>{
 const f=ifcUnitTraversalFixture(),graph=(await IfcGeometryGraph.capture(f.snapshot)).artifact!,unit=graph.ref(f.units[row.id]);
 const expected=graph.unitMeasure(unit),actual=await graph.unitMeasureAsync(unit,{batchSize:1});expect(actual).toEqual(expected);expect(actual.scale/row.scale).toBeCloseTo(1,14);expect([...actual.dimensions]).toEqual(row.dimensions);
 if(row.dimensions[0]===1){f.assignment.args[0]=f.a([f.units[row.id]!]);const source=(await IfcGeometryGraph.capture(f.snapshot)).artifact!;expect(await source.lengthScaleAsync({batchSize:1})).toBeCloseTo(row.scale,14);expect(source.lengthScale()).toBeCloseTo(row.scale,14);}
});
for(const id of cases.refusals)test('controlled unit role refusal '+id,async()=>{
 const f=ifcUnitTraversalFixture(),byRef=(v:any)=>f.snapshot.entities.find(row=>row.id===v.value)!;let selected=f.units.foot!,placement=false,length=false;
 if(id==='assignment-kind'){f.assignment.name='IFCGROUP';length=true;}
 if(id==='assignment-member'){f.assignment.args[0]=f.a([f.placement]);length=true;}
 if(id==='duplicate-length'){f.assignment.args[0]=f.a([f.units.metre!,f.units.millimetre!]);length=true;}
 if(id.startsWith('derived-')){selected=f.units['density-kg-per-mm3']!;const unit=byRef(selected),element=byRef((unit.args[0] as any).value[0]);if(id==='derived-child-role')element.args[0]=f.units['pressure-kg-per-m-s2']!;if(id==='derived-single-exp1')unit.args[0]=f.a([{kind:'reference',value:element.id}]);if(id==='derived-noninteger')element.args[1]={kind:'enum',value:'T'};}
 const unit=byRef(selected);
 if(id==='conversion-wrapper'){const factor=byRef(unit.args[3]);(factor.args[0] as any).value.name='IFCMASSMEASURE';}
 if(id==='conversion-cycle')byRef(unit.args[3]).args[1]=selected;
 if(id==='conversion-dimensions')byRef(unit.args[0]).args[0]=f.i(0);
 if(id==='conversion-factor-kind')byRef(unit.args[3]).name='IFCGROUP';
 if(id==='offset-conversion'){unit.name='IFCCONVERSIONBASEDUNITWITHOFFSET';unit.args.push(f.i(0));}
 if(id==='placement-cycle'){byRef(f.placement).args[0]=f.placement;placement=true;}
 const graph=(await IfcGeometryGraph.capture(f.snapshot)).artifact!;
 const run=()=>placement?graph.placementAsync(f.placement,{batchSize:1}):length?graph.lengthScaleAsync({batchSize:1}):graph.unitMeasureAsync(graph.ref(selected),{batchSize:1});
 await expect(run()).rejects.toBeInstanceOf(IfcStepError);
});
for(const operation of cases.operations)for(const frontier of cases.frontiers)for(const fault of cases.faults)test('original traversal '+operation+' '+frontier+' '+fault,async()=>{
 const f=ifcUnitTraversalFixture(),graph=(await IfcGeometryGraph.capture(f.snapshot)).artifact!,before=structuredClone(graph.snapshot),events:{completed:number;total:number}[]=[];
 const run=(control:IfcIoControl)=>operation==='length'?graph.lengthScaleAsync(control):operation==='placement'?graph.placementAsync(f.placement,control):graph.unitMeasureAsync(graph.ref(f.units[operation==='derived'?'density-kg-per-mm3':'inch-via-foot']),control);
 await run({batchSize:1,progress:event=>events.push(event)});expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,original=new AbortController(),replacement=new AbortController();
 const failure=fault==='error'?new Error('original'):fault==='step-error'?new IfcStepError('exchange.number','original'):fault==='edit-error'?new IfcEditError('invalidValue','original'):fault==='number'?17:undefined;
 let calls=0,replacementCalls=0,caught=false;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress(){if(calls++!==target)return;control.signal=replacement.signal;control.progress=()=>replacementCalls++;if(fault==='abort')original.abort();else throw failure;}};
 try{await run(control);}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}
 expect(caught).toBe(true);expect(calls).toBe(target+1);expect(replacementCalls).toBe(0);expect(graph.snapshot).toEqual(before);
});
test('controlled placement folds every original local translation',async()=>{const f=ifcUnitTraversalFixture(),graph=(await IfcGeometryGraph.capture(f.snapshot)).artifact!,actual=await graph.placementAsync(f.placement,{batchSize:1});expect(actual).toEqual(graph.placement(f.placement));expect(actual[12]).toBe(24);});
