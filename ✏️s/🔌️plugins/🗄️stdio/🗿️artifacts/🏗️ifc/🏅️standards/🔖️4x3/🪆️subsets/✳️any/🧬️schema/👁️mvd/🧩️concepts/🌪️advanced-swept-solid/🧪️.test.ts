/** 🌪️ Exact released AdvancedSweptSolid clauses retain separate schema and concept evidence. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv';
import {compare,applyPatch} from 'fast-json-patch';
import * as api from './🟦️.ts';
import {validateIfcBodyAdvancedSweptSolid as standardEndpoint,ifcBodyAdvancedSweptSolidDefinition as standardDefinition} from '../../../../🟦️.ts';
import cases from './cases.json';
import schema from './🔣️.json';
import provenance from './provenance.json';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
import {parseIfc4x3Step,writeIfc4x3Step} from '../../../../🚪️io/step.ts';
import {ifcMutationDiffAsync,applyIfcMutationAsync,inverseIfcMutationAsync} from '../../../🧬️mutations/🟦️.ts';
import {applyIfcDiffAsync,inverseIfcDiffAsync} from '../../../🔺️diff/🟦️.ts';
const text=(value:string):IfcValue=>({kind:'string',value}),aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),reference=(value:bigint):IfcValue=>({kind:'reference',value});
const request={targets:[{representation:'1',applicable:true}]};
function graph(row=cases.cases[0]!):IfcSnapshot{return{schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[aggregate([text('IFC4X3_ADD2')])]},entities:[{id:1n,name:'IFCSHAPEREPRESENTATION',args:[{kind:'unset'},text(row.identifier),text(row.type),aggregate(row.items.map((_,index)=>reference(BigInt(index+2))))],complex:[]},...row.items.map((name,index)=>({id:BigInt(index+2),name:name.toUpperCase(),args:[],complex:[]}))]};}
const validate=new Ajv({strict:true}).compile(schema);
test('standard API exports the actual shared-executor endpoint and frozen definition',async()=>{expect(standardEndpoint).toBe(api.validateIfcBodyAdvancedSweptSolid);expect(standardDefinition).toBe(api.ifcBodyAdvancedSweptSolidDefinition);expect((await standardEndpoint(graph(),request)).status).toBe('pass');});
test('closed concept definition preserves the exact two HTML families and separate EXPRESS provenance',()=>{
 expect(validate(api.ifcBodyAdvancedSweptSolidDefinition)).toBe(true);expect(validate(cases)).toBe(true);expect(validate(provenance)).toBe(true);expect(validate({...cases,completeView:true})).toBe(false);
 expect(api.ifcBodyAdvancedSweptSolidDefinition.predicates[2]?.value).toEqual(['IFCSWEPTAREASOLID','IFCSWEPTDISKSOLID']);expect(provenance.express.families).toContain('IFCSECTIONEDSOLIDHORIZONTAL');expect(Object.isFrozen(api.ifcBodyAdvancedSweptSolidDefinition)).toBe(true);expect(Object.isFrozen(api.ifcBodyAdvancedSweptSolidDefinition.source)).toBe(true);for(const row of api.ifcBodyAdvancedSweptSolidDefinition.predicates){expect(Object.isFrozen(row)).toBe(true);if(Array.isArray(row.value))expect(Object.isFrozen(row.value)).toBe(true);}
});
test('pinned actual EXPRESS source retains its separate broader three-family branch',async()=>{
 const bytes=await Bun.file(new URL('../../../📚️catalog/IFC4X3_ADD2.exp',import.meta.url)).arrayBuffer(),source=new TextDecoder().decode(bytes);expect(new Bun.CryptoHasher('sha256').update(bytes).digest('hex')).toBe(provenance.express.sha256);
 const lines=source.split(/\r?\n/u).slice(provenance.express.lines[0]-1,provenance.express.lines[1]);expect(lines[0]).toContain("'AdvancedSweptSolid'");for(const family of provenance.express.families)expect(lines.join('\n')).toContain('IFC4X3_ADD2.'+family);
});
for(const row of cases.cases)test('selected AdvancedSweptSolid '+row.id,async()=>{
 const source=graph(row),before=structuredClone(source),input={targets:[{representation:'1',applicable:row.applicable}]},report=await api.validateIfcBodyAdvancedSweptSolid(source,input,{batchSize:64});
 expect(String(report.status)).toBe(row.expected);expect(String(report.targets[0]?.status)).toBe(row.expected);expect(report.source).toEqual(provenance.concept);expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');expect(compare(before,source)).toEqual([]);expect(validate(report)).toBe(true);
});
for(const mode of ['missing-target','duplicate-target','unknown-target','missing-item','duplicate-item','unknown-item','nonreference-item','late-unknown-item','duplicate-owner','late-unknown-owner']as const)test('AdvancedSweptSolid unresolved evidence '+mode,async()=>{
 const source=graph(),target=source.entities[0]!,item=source.entities[1]!;
 if(mode==='missing-target')source.entities.shift();if(mode==='duplicate-target')source.entities.push(structuredClone(target));if(mode==='unknown-target')target.complex!.push({name:'IFCUNKNOWN',args:[]});if(mode==='missing-item')source.entities.pop();if(mode==='duplicate-item')source.entities.push(structuredClone(item));if(mode==='unknown-item')item.name='IFCUNKNOWN';if(mode==='nonreference-item')target.args[3]=aggregate([text('invalid')]);if(mode==='late-unknown-item')item.complex!.push({name:'IFCUNKNOWN',args:[]});
 if(mode==='duplicate-owner'||mode==='late-unknown-owner'){target.name='IFCREPRESENTATION';target.complex!.push({name:'IFCSHAPEREPRESENTATION',args:[]},{name:mode==='duplicate-owner'?'IFCREPRESENTATION':'IFCUNKNOWN',args:structuredClone(target.args)});}
 const result=await api.validateIfcBodyAdvancedSweptSolid(source,request);expect(result.status).toBe('incomplete');expect(result.targets[0]?.diagnostics.length).toBeGreaterThan(0);expect(validate(result)).toBe(true);
});
test('qualified declaration owner and complete mixed-item evidence retain exact pointers',async()=>{
 const source=graph(),target=source.entities[0]!;target.name='IFCREPRESENTATION';target.complex!.push({name:'IFCSHAPEREPRESENTATION',args:[]},{name:'IFCMATERIAL',args:[text('foreign'),{kind:'unset'},{kind:'unset'}]});expect((await api.validateIfcBodyAdvancedSweptSolid(source,request)).status).toBe('pass');
 target.args[1]=text('Axis');let report=await api.validateIfcBodyAdvancedSweptSolid(source,request);expect(report.targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.equals',pointer:'/entities/0/args/1'}));
 target.args[1]=text('Body');source.entities[1]!.name='IFCSECTIONEDSOLIDHORIZONTAL';report=await api.validateIfcBodyAdvancedSweptSolid(source,request);expect(report.status).toBe('fail');expect(report.targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.items.type',pointer:'/entities/0/args/3/value/0',label:expect.objectContaining({en:expect.any(String),de:expect.any(String)})}));
 target.args[3]=aggregate([reference(2n),reference(99n)]);report=await api.validateIfcBodyAdvancedSweptSolid(source,request);expect(report.status).toBe('incomplete');expect(report.targets[0]?.diagnostics.map(row=>row.code)).toEqual(['mvd.items.type','mvd.items.missing']);
});
test('closed exact-u64 request admission precedes any source getter',async()=>{
 let reads=0;const source=Object.defineProperty({},'schema',{enumerable:true,get(){reads++;return'stdio.ifc';}})as IfcSnapshot;
 for(const input of [{targets:[]},{targets:[{representation:'0',applicable:true}]},{targets:[{representation:'01',applicable:true}]},{targets:[{representation:'18446744073709551616',applicable:true}]},{targets:[...request.targets,...request.targets]},{...request,view:'ReferenceView'}])await expect(api.validateIfcBodyAdvancedSweptSolid(source,input as typeof request)).rejects.toMatchObject({code:'mvd.request'});expect(reads).toBe(0);
 expect(validate({targets:[{representation:'18446744073709551615',applicable:null}]})).toBe(true);expect(validate({targets:[{representation:'18446744073709551616',applicable:true}]})).toBe(false);
});
for(const phase of ['mvd-request','mvd-index','mvd-concept'])for(const boundary of ['initial','interior','completion'])test('original AdvancedSweptSolid cancellation '+phase+' '+boundary,async()=>{
 const original=new AbortController(),replacement=new AbortController();let reached=false,redirected=0;const control:IfcIoControl={signal:original.signal,batchSize:1,progress:event=>{if(event.phase!==phase)return;const at=event.completed===0?'initial':event.total>0&&event.completed===event.total?'completion':'interior';if(at!==boundary)return;reached=true;control.signal=replacement.signal;control.progress=()=>redirected++;original.abort();}};
 await expect(api.validateIfcBodyAdvancedSweptSolid(graph(),{targets:[...request.targets,{representation:'99',applicable:false}]},control)).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);expect(redirected).toBe(0);
});
for(const refusal of [undefined,Object.freeze({marker:'advanced-swept-refusal'})])test('AdvancedSweptSolid original observer failure '+String(refusal),async()=>{
 for(const phase of ['mvd-request','mvd-capture','mvd-index','mvd-concept']){let reached=false,failed=false,actual:unknown;try{await api.validateIfcBodyAdvancedSweptSolid(graph(),request,{batchSize:1,progress:event=>{if(event.phase===phase){reached=true;throw refusal;}}});}catch(error){failed=true;actual=error;}expect(reached).toBe(true);expect(failed).toBe(true);expect(actual).toBe(refusal);}
});
test('original captured request and source survive callback replacement',async()=>{
 const source=graph(),input=structuredClone(request);let requestChanged=false,sourceChanged=false;const report=await api.validateIfcBodyAdvancedSweptSolid(source,input,{batchSize:1,progress:event=>{if(event.phase==='mvd-request'&&event.completed===event.total){input.targets=[];requestChanged=true;}if(event.phase==='mvd-capture'&&event.completed===event.total){source.entities=[];sourceChanged=true;}}});expect(requestChanged&&sourceChanged).toBe(true);expect(report.targets[0]?.representation).toBe('1');expect(report.status).toBe('pass');
});
for(const row of provenance.originals)test('original official selected AdvancedSweptSolid '+row.file,async()=>{
 const source=await parseIfc4x3Step(await Bun.file(new URL('../../../../🧪️fixtures/official/'+row.file,import.meta.url)).text(),{batchSize:4096}),report=await api.validateIfcBodyAdvancedSweptSolid(source,{targets:[{representation:row.representation,applicable:true}]},{batchSize:4096});expect(String(report.status)).toBe(row.expected);expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');
});
test('official fixed-reference edit uses central diff inverse mutation and exact STEP roundtrip',async()=>{
 const source=await parseIfc4x3Step(await Bun.file(new URL('../../../../🧪️fixtures/official/fixed-reference-swept-area-solid.ifc',import.meta.url)).text(),{batchSize:4096}),input={targets:[{representation:'122',applicable:true}]},ordinal=source.entities.findIndex(row=>row.id===122n),mutation={mutation:'setEntityArg'as const,id:122n,index:1,value:text('Axis')},options={yieldEvery:4096};
 const wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?String(item):item)),diff=await ifcMutationDiffAsync(source,mutation,options),after=await applyIfcDiffAsync(source,diff,options);
 expect(compare(wire(after),applyPatch(wire(source),[{op:'replace',path:'/entities/'+ordinal+'/args/1',value:wire(mutation.value)}]).newDocument)).toEqual([]);expect((await api.validateIfcBodyAdvancedSweptSolid(after,input)).status).toBe('fail');expect(await parseIfc4x3Step(await writeIfc4x3Step(after,{batchSize:4096}),{batchSize:4096})).toEqual(after);expect(await applyIfcDiffAsync(after,await inverseIfcDiffAsync(source,diff,options),options)).toEqual(source);expect(await applyIfcMutationAsync(after,await inverseIfcMutationAsync(source,mutation,options),options)).toEqual(source);expect((await api.validateIfcBodyAdvancedSweptSolid(source,input)).status).toBe('pass');
});
