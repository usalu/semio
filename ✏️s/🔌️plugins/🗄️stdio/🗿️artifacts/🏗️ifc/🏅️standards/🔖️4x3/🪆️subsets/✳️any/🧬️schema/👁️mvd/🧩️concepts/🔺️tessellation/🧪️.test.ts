import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import cases from './cases.json';
import schema from './🔣️.json';
import {validateIfcBodyTessellation,ifcBodyTessellationDefinition} from './🟦️.ts';
import type {IfcBodyTessellationRequest} from './types.ts';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';

const text=(value:string):IfcValue=>({kind:'string',value}),aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),reference=(value:bigint):IfcValue=>({kind:'reference',value});
function graph(row=cases.cases[0]!):IfcSnapshot{return {schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[aggregate([text('IFC4X3_ADD2')])]},entities:[{id:1n,name:'IFCSHAPEREPRESENTATION',args:[{kind:'unset'},text(row.identifier),text(row.type),aggregate(row.items.map((_,i)=>reference(BigInt(i+2))))],complex:[]},...row.items.map((name,i)=>({id:BigInt(i+2),name:name.toUpperCase(),args:[],complex:[]}))]};}
const request:IfcBodyTessellationRequest={targets:[{representation:'1',applicable:true}]};
test('released definition and language-neutral fixtures are closed and immutable',()=>{
 const ajv=new Ajv({strict:true}),validate=ajv.compile(schema);expect(validate(ifcBodyTessellationDefinition)).toBe(true);expect(validate(cases)).toBe(true);expect(validate({...cases,allViewsComplete:true})).toBe(false);expect(Object.isFrozen(ifcBodyTessellationDefinition)).toBe(true);expect(Object.isFrozen(ifcBodyTessellationDefinition.source)).toBe(true);expect(ifcBodyTessellationDefinition.predicates.every(Object.isFrozen)).toBe(true);
});
test('neutral concept request/report contract is closed and exact positive u64',async()=>{
 const ajv=new Ajv({strict:true}),validate=ajv.compile(schema);expect(validate(request)).toBe(true);expect(validate({...request,view:'ReferenceView'})).toBe(false);for(const representation of ['0','01','-1','18446744073709551616','100000000000000000000'])expect(validate({targets:[{representation,applicable:true}]})).toBe(false);expect(validate({targets:[{representation:'18446744073709551615',applicable:null}]})).toBe(true);expect(validate(await validateIfcBodyTessellation(graph(),request))).toBe(true);
});
for(const row of cases.cases)test('released explicit concept '+row.id,async()=>{
 const source=graph(row),before=structuredClone(source),report=await validateIfcBodyTessellation(source,{targets:[{representation:'1',applicable:row.applicable}]},{batchSize:64});expect<string>(report.status).toBe(row.expected);expect<string|undefined>(report.targets[0]?.status).toBe(row.expected);expect(report.concept).toBe(ifcBodyTessellationDefinition.id);expect(report.source).toEqual(ifcBodyTessellationDefinition.source);expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');expect(source).toEqual(before);
});
test('counterexamples identify actual editable attribute pointers',async()=>{
 const source=graph(cases.cases.find(row=>row.id==='wrong-identifier')!),report=await validateIfcBodyTessellation(source,request);expect(report.targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.equals',path:['RepresentationIdentifier'],pointer:'/entities/0/args/1'}));
 source.entities[0]!.args[1]=text('Body');source.entities[1]!.name='IFCPOLYLINE';const item=await validateIfcBodyTessellation(source,request);expect(item.targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.items.type',path:['Items','0'],pointer:'/entities/0/args/3/value/0'}));
});
for(const kind of ['missing-target','ambiguous-target','missing-item','ambiguous-item','unknown-item','nonreference-item'] as const)test('unresolved concept evidence is incomplete '+kind,async()=>{
 const source=graph();if(kind==='missing-target')source.entities.shift();if(kind==='ambiguous-target')source.entities.push(structuredClone(source.entities[0]!));if(kind==='missing-item')source.entities.pop();if(kind==='ambiguous-item')source.entities.push(structuredClone(source.entities[1]!));if(kind==='unknown-item')source.entities[1]!.name='IFCUNKNOWNENTITY';if(kind==='nonreference-item')source.entities[0]!.args[3]=aggregate([text('not a reference')]);
 const report=await validateIfcBodyTessellation(source,request);expect(report.status).toBe('incomplete');expect(report.targets[0]?.diagnostics.length).toBeGreaterThan(0);expect(report.viewConformance).toBe('not-evaluated');
});
test('complex constituents resolve declared attributes and inherited membership without flattening source',async()=>{
 const source=graph();source.entities[0]!.name='IFCREPRESENTATION';source.entities[0]!.complex=[{name:'IFCSHAPEREPRESENTATION',args:[]}];source.entities[1]!.name='IFCTESSELLATEDFACESET';source.entities[1]!.complex=[{name:'IFCTRIANGULATEDFACESET',args:[]}];expect((await validateIfcBodyTessellation(source,request)).status).toBe('pass');
 source.entities[0]!.args[2]=text('SurfaceModel');expect((await validateIfcBodyTessellation(source,request)).targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.equals',pointer:'/entities/0/args/2'}));
 source.entities[0]!.complex.push({name:'IFCREPRESENTATION',args:structuredClone(source.entities[0]!.args)});expect((await validateIfcBodyTessellation(source,request)).status).toBe('incomplete');
});
test('typed IfcLabel preserves defined-domain equality and empty Items adds no invented cardinality rule',async()=>{
 const source=graph();source.entities[0]!.args[1]={kind:'typedValue',value:{name:'IFCLABEL',items:[text('Body')]}};expect((await validateIfcBodyTessellation(source,request)).status).toBe('pass');source.entities[0]!.args[1]={kind:'typedValue',value:{name:'IFCTEXT',items:[text('Body')]}};expect((await validateIfcBodyTessellation(source,request)).status).toBe('fail');source.entities[0]!.args[1]=text('Body');source.entities[0]!.args[3]=aggregate([]);const report=await validateIfcBodyTessellation(source,request);expect(report.status).toBe('pass');expect(report.schemaConformance).toBe('not-evaluated');
});
test('aggregate status separates known failure, incomplete facts and excluded targets',async()=>{
 const source=graph();source.entities[0]!.id=18446744073709551615n;const targets=[{representation:'18446744073709551615',applicable:true},{representation:'99',applicable:false}];expect((await validateIfcBodyTessellation(source,{targets})).status).toBe('pass');targets[0]!.applicable=false;expect((await validateIfcBodyTessellation(source,{targets})).status).toBe('not-applicable');targets[0]!.applicable=true;source.entities[0]!.args[1]=text('Axis');expect((await validateIfcBodyTessellation(source,{targets})).status).toBe('fail');expect((await validateIfcBodyTessellation(source,{targets:[...targets,{representation:'100',applicable:true}]})).status).toBe('incomplete');
});
test('request admission precedes source access and rejects open/duplicate/invalid targets',async()=>{
 let touched=0;const source=Object.defineProperty({},'schema',{enumerable:true,get(){touched++;return 'stdio.ifc';}}) as IfcSnapshot;
 for(const input of [{targets:[]},{targets:[{representation:'1'}]},{targets:[...request.targets,...request.targets]},{targets:[{representation:'18446744073709551616',applicable:true}]},{...request,scope:'ReferenceView'}])await expect(validateIfcBodyTessellation(source,input as IfcBodyTessellationRequest)).rejects.toMatchObject({code:'mvd.request'});expect(touched).toBe(0);
 const invalid=await validateIfcBodyTessellation(source,request);expect(invalid.status).toBe('incomplete');expect(invalid.diagnostics[0]?.code).toBe('mvd.source');expect(touched).toBe(0);
});
test('private source capture survives caller replacement before concept callbacks',async()=>{
 const source=graph();let changed=false;const report=await validateIfcBodyTessellation(source,request,{batchSize:64,progress:event=>{if(event.phase==='mvd-capture'&&event.completed===event.total){source.entities=[];changed=true;}}});expect(changed).toBe(true);expect(report.status).toBe('pass');
});
for(const phase of ['mvd-index','mvd-concept'])for(const boundary of ['interior','completion'])test('original concept cancellation '+phase+' '+boundary,async()=>{
 const source=graph(),before=structuredClone(source),original=new AbortController(),other=new AbortController();let reached=false,otherCalls=0;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress:event=>{if(event.phase!==phase||event.completed===0)return;const complete=event.total>0&&event.completed===event.total;if((boundary==='completion')!==complete)return;reached=true;control.signal=other.signal;control.progress=()=>otherCalls++;original.abort();}};
 await expect(validateIfcBodyTessellation(source,request,control)).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);expect(otherCalls).toBe(0);expect(source).toEqual(before);
});
for(const boundary of ['interior','completion'])test('original request census cancellation '+boundary,async()=>{
 const original=new AbortController(),source=graph();let reached=false;await expect(validateIfcBodyTessellation(source,{targets:[...request.targets,{representation:'3',applicable:false}]},{signal:original.signal,batchSize:1,progress:event=>{if(event.phase!=='mvd-request'||event.completed===0)return;const complete=event.completed===event.total;if((boundary==='completion')!==complete)return;reached=true;original.abort();}})).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);
});
test('incomplete-source result generation remains cancellable and callbacks retain their failures',async()=>{
 const original=new AbortController();let reached=false;await expect(validateIfcBodyTessellation({} as IfcSnapshot,request,{signal:original.signal,batchSize:1,progress:event=>{if(event.phase==='mvd-result'&&event.completed>0){reached=true;original.abort();}}})).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);
 const refusal=new Error('target');await expect(validateIfcBodyTessellation(graph(),request,{progress:()=>{throw refusal;}})).rejects.toBe(refusal);
});
test('the exact released FILE_SCHEMA is required without claiming full schema validation',async()=>{
 const source=graph();source.header.fileSchema=[aggregate([text('IFC4')])];const report=await validateIfcBodyTessellation(source,request);expect(report.status).toBe('incomplete');expect(report.diagnostics[0]?.code).toBe('mvd.source');expect(report.schemaConformance).toBe('not-evaluated');
});
