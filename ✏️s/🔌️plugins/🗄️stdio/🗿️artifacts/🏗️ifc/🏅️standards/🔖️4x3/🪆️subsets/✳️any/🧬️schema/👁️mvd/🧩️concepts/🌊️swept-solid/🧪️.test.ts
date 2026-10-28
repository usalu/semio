/** 🌊️ Closed released Body SweptSolid cases distinguish concept, schema and geometry. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {compare,applyPatch} from 'fast-json-patch';
import * as api from './🟦️.ts';
import cases from './cases.json';
import schema from './🔣️.json';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
import {parseIfc4x3Step,writeIfc4x3Step} from '../../../../🚪️io/step.ts';
import {ifcMutationDiffAsync,applyIfcMutationAsync,inverseIfcMutationAsync} from '../../../🧬️mutations/🟦️.ts';
import {applyIfcDiffAsync,inverseIfcDiffAsync} from '../../../🔺️diff/🟦️.ts';
const text=(value:string):IfcValue=>({kind:'string',value}),aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),reference=(value:bigint):IfcValue=>({kind:'reference',value});
const request={targets:[{representation:'1',applicable:true}]};
function graph(row=cases.cases[0]!):IfcSnapshot{return{schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[aggregate([text('IFC4X3_ADD2')])]},entities:[{id:1n,name:'IFCSHAPEREPRESENTATION',args:[{kind:'unset'},text(row.identifier),text(row.type),aggregate(row.items.map((_,at)=>reference(BigInt(at+2))))],complex:[]},...row.items.map((name,at)=>({id:BigInt(at+2),name:name.toUpperCase(),args:[],complex:[]}))]};}
test('released definition and neutral cases have a closed immutable contract',()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(api.ifcBodySweptSolidDefinition)).toBe(true);expect(validate(cases)).toBe(true);expect(validate({...cases,allViewsComplete:true})).toBe(false);expect(Object.isFrozen(api.ifcBodySweptSolidDefinition)).toBe(true);expect(Object.isFrozen(api.ifcBodySweptSolidDefinition.source)).toBe(true);for(const row of api.ifcBodySweptSolidDefinition.predicates){expect(Object.isFrozen(row)).toBe(true);if(Array.isArray(row.value))expect(Object.isFrozen(row.value)).toBe(true);if('exclude'in row)expect(Object.isFrozen(row.exclude)).toBe(true);}
});
for(const row of cases.cases)test('released selected SweptSolid '+row.id,async()=>{
 const source=graph(row),before=structuredClone(source),report=await api.validateIfcBodySweptSolid(source,{targets:[{representation:'1',applicable:row.applicable}]},{batchSize:64});
 expect(String(report.status)).toBe(row.expected);expect(String(report.targets[0]?.status)).toBe(row.expected);expect(report.source).toEqual(api.ifcBodySweptSolidDefinition.source);expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');expect(compare(before,source)).toEqual([]);expect(new Ajv({strict:true}).compile(schema)(report)).toBe(true);
});
for(const kind of ['target-missing','target-duplicate','target-unknown','item-missing','item-duplicate','item-unknown','item-nonreference','item-late-unknown','duplicate-owner','late-owner-unknown'] as const)test('unresolved SweptSolid evidence '+kind,async()=>{
 const source=graph(),target=source.entities[0]!,item=source.entities[1]!;
 if(kind==='target-missing')source.entities.shift();if(kind==='target-duplicate')source.entities.push(structuredClone(target));if(kind==='target-unknown')target.complex!.push({name:'IFCUNKNOWN',args:[]});
 if(kind==='item-missing')source.entities.pop();if(kind==='item-duplicate')source.entities.push(structuredClone(item));if(kind==='item-unknown')item.name='IFCUNKNOWN';if(kind==='item-nonreference')target.args[3]=aggregate([text('bad')]);if(kind==='item-late-unknown')item.complex!.push({name:'IFCUNKNOWN',args:[]});
 if(kind==='duplicate-owner'||kind==='late-owner-unknown'){target.name='IFCREPRESENTATION';target.complex!.push({name:'IFCSHAPEREPRESENTATION',args:[]},{name:kind==='duplicate-owner'?'IFCREPRESENTATION':'IFCUNKNOWN',args:structuredClone(target.args)});}
 const report=await api.validateIfcBodySweptSolid(source,request);expect(report.status).toBe('incomplete');expect(report.targets[0]?.diagnostics.length).toBeGreaterThan(0);
});
test('qualified owner attributes preserve exact editable pointers and late evidence',async()=>{
 const source=graph(),target=source.entities[0]!;target.name='IFCREPRESENTATION';target.complex!.push({name:'IFCSHAPEREPRESENTATION',args:[]},{name:'IFCMATERIAL',args:[text('unrelated'),{kind:'unset'},{kind:'unset'}]});
 expect((await api.validateIfcBodySweptSolid(source,request)).status).toBe('pass');target.args[1]=text('Axis');expect((await api.validateIfcBodySweptSolid(source,request)).targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.equals',pointer:'/entities/0/args/1'}));
 source.entities[1]!.name='IFCEXTRUDEDAREASOLIDTAPERED';target.args[1]=text('Body');expect((await api.validateIfcBodySweptSolid(source,request)).targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.items.type',pointer:'/entities/0/args/3/value/0'}));
});
test('closed request admission precedes source reads and retains exact u64 IDs',async()=>{
 let reads=0;const source=Object.defineProperty({},'schema',{enumerable:true,get(){reads++;return'stdio.ifc';}})as IfcSnapshot;
 for(const input of [{targets:[]},{targets:[{representation:'0',applicable:true}]},{targets:[{representation:'18446744073709551616',applicable:true}]},{targets:[...request.targets,...request.targets]},{...request,view:'ReferenceView'}])await expect(api.validateIfcBodySweptSolid(source,input as typeof request)).rejects.toMatchObject({code:'mvd.request'});expect(reads).toBe(0);
 const validate=new Ajv({strict:true}).compile(schema);expect(validate({targets:[{representation:'18446744073709551615',applicable:null}]})).toBe(true);expect(validate({targets:[{representation:'18446744073709551616',applicable:true}]})).toBe(false);
});
for(const phase of ['mvd-request','mvd-index','mvd-concept'])for(const boundary of ['initial','interior','completion'])test('original SweptSolid cancellation '+phase+' '+boundary,async()=>{
 const original=new AbortController(),replacement=new AbortController(),source=graph();let reached=false,redirected=0;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress:event=>{if(event.phase!==phase)return;const state=event.completed===0?'initial':event.total>0&&event.completed===event.total?'completion':'interior';if(state!==boundary)return;reached=true;control.signal=replacement.signal;control.progress=()=>redirected++;original.abort();}};
 await expect(api.validateIfcBodySweptSolid(source,{targets:[...request.targets,{representation:'99',applicable:false}]},control)).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);expect(redirected).toBe(0);
});
for(const refusal of [undefined,Object.freeze({marker:'swept-refusal'})])test('SweptSolid original observer failure '+String(refusal),async()=>{
 for(const phase of ['mvd-request','mvd-capture','mvd-index','mvd-concept']){let reached=false,failed=false,actual:unknown;try{await api.validateIfcBodySweptSolid(graph(),request,{batchSize:1,progress:event=>{if(event.phase===phase){reached=true;throw refusal;}}});}catch(error){failed=true;actual=error;}expect(reached).toBe(true);expect(failed).toBe(true);expect(actual).toBe(refusal);}
});
test('privately captured source survives caller replacement at the capture boundary',async()=>{
 const source=graph();let changed=false;const report=await api.validateIfcBodySweptSolid(source,request,{batchSize:1,progress:event=>{if(event.phase==='mvd-capture'&&event.completed===event.total){source.entities=[];changed=true;}}});expect(changed).toBe(true);expect(report.status).toBe('pass');
});
for(const [file,ids]of [['extruded-solid.ifc',['1020']],['slab-openings.ifc',['314','323','334']]] as const)test('original official SweptSolid '+file,async()=>{
 const source=await parseIfc4x3Step(await Bun.file(new URL('../../../../🧪️fixtures/official/'+file,import.meta.url)).text(),{batchSize:4096});
 const report=await api.validateIfcBodySweptSolid(source,{targets:ids.map(representation=>({representation,applicable:true}))},{batchSize:4096});expect(report.status).toBe('pass');expect(report.targets.map(row=>row.status)).toEqual(ids.map(()=>'pass'));
});
test('official concept edit follows central diff inverse mutation and exact STEP roundtrip',async()=>{
 const source=await parseIfc4x3Step(await Bun.file(new URL('../../../../🧪️fixtures/official/extruded-solid.ifc',import.meta.url)).text(),{batchSize:4096}),input={targets:[{representation:'1020',applicable:true}]},ordinal=source.entities.findIndex(row=>row.id===1020n),mutation={mutation:'setEntityArg'as const,id:1020n,index:1,value:text('Axis')},options={yieldEvery:4096};
 const wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?String(item):item)),diff=await ifcMutationDiffAsync(source,mutation,options),after=await applyIfcDiffAsync(source,diff,options);
 expect(compare(wire(after),applyPatch(wire(source),[{op:'replace',path:'/entities/'+ordinal+'/args/1',value:wire(mutation.value)}]).newDocument)).toEqual([]);expect((await api.validateIfcBodySweptSolid(after,input)).status).toBe('fail');expect(await parseIfc4x3Step(await writeIfc4x3Step(after,{batchSize:4096}),{batchSize:4096})).toEqual(after);
 expect(await applyIfcDiffAsync(after,await inverseIfcDiffAsync(source,diff,options),options)).toEqual(source);expect(await applyIfcMutationAsync(after,await inverseIfcMutationAsync(source,mutation,options),options)).toEqual(source);expect((await api.validateIfcBodySweptSolid(source,input)).status).toBe('pass');
});
