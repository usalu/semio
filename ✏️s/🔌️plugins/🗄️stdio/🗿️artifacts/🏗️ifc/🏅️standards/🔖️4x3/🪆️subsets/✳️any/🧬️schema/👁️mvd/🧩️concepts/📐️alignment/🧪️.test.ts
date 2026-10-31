import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {applyPatch} from 'fast-json-patch';
import cases from './cases.json';
import schema from './🔣️.json';
import official from '../../../../🧪️fixtures/official/cases.json';
import {alignmentFixture} from './🧪️fixture.ts';
import {ifcAlignmentTerminalDefinition,validateIfcAlignmentTerminal} from './🟦️.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {parseIfc4x3Step,writeIfc4x3Step,ifcReal} from '../../../../🚪️io/step.ts';
import {ifcMutationDiffAsync,inverseIfcMutationAsync,applyIfcMutationAsync} from '../../../🧬️mutations/🟦️.ts';
import {applyIfcDiffAsync,inverseIfcDiffAsync} from '../../../🔺️diff/🟦️.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcAlignmentTerminalRequest} from './types.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';

const request:IfcAlignmentTerminalRequest={targets:[{layout:'1',kind:'horizontal',applicable:true}]};
const graph=()=>alignmentFixture(cases.cases[0]!);
test('released predicate definition and neutral cases are closed',()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate(ifcAlignmentTerminalDefinition)).toBe(true);expect(validate({...cases,fullView:true})).toBe(false);expect(validate({cases:[{...cases.cases[0],lengths:[],variant:'typed-measure'}]})).toBe(false);expect(Object.isFrozen(ifcAlignmentTerminalDefinition)).toBe(true);expect(Object.isFrozen(ifcAlignmentTerminalDefinition.source)).toBe(true);expect(ifcAlignmentTerminalDefinition.layouts.every(Object.isFrozen)).toBe(true);
});
test('request and partial report preserve closed exact positive u64 contracts',async()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(request)).toBe(true);for(const layout of ['0','01','-1','18446744073709551616','100000000000000000000'])expect(validate({targets:[{layout,kind:'horizontal',applicable:true}]})).toBe(false);expect(validate({targets:[{layout:'18446744073709551615',kind:'cant',applicable:null}]})).toBe(true);expect(validate(await validateIfcAlignmentTerminal(graph(),request))).toBe(true);expect(validate({...request,view:'AlignmentView'})).toBe(false);
});
for(const row of cases.cases)test('released business terminal '+row.id,async()=>{
 const source=alignmentFixture(row),before=structuredClone(source),report=await validateIfcAlignmentTerminal(source,{targets:[{layout:'1',kind:row.kind as 'horizontal',applicable:row.applicable}]},{batchSize:4096});expect<string>(report.status).toBe(row.expected);expect<string|undefined>(report.targets[0]?.status).toBe(row.expected);expect(report.predicate).toBe('business-terminal-zero');expect(report.geometryConformance).toBe('not-evaluated');expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');expect(source).toEqual(before);
});
test('known terminal failure identifies actual design length pointer',async()=>{
 const source=alignmentFixture(cases.cases[3]!),parameter=source.entities[5]!,ordinal=ifc4x3Catalog.entities[parameter.name]!.allAttributes.findIndex(item=>item.name==='SegmentLength'),report=await validateIfcAlignmentTerminal(source,request);expect(report.targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.terminal.nonzero',path:['RelatedObjects','1','DesignParameters','SegmentLength'],pointer:'/entities/5/args/'+ordinal}));
});
test('source entity order never replaces ordered nesting and caller target order',async()=>{
 const source=graph();source.entities.reverse();expect((await validateIfcAlignmentTerminal(source,request,{batchSize:4096})).status).toBe('pass');const report=await validateIfcAlignmentTerminal(source,{targets:[{layout:'99',kind:'cant',applicable:false},...request.targets,{layout:'98',kind:'vertical',applicable:null}]},{batchSize:4096});expect(report.targets.map(row=>row.layout)).toEqual(['99','1','98']);expect(report.status).toBe('incomplete');
});
test('incomplete source is explicit and does not claim schema validation',async()=>{
 const source=graph();source.header.fileSchema=[];const report=await validateIfcAlignmentTerminal(source,request);expect(report.status).toBe('incomplete');expect(report.diagnostics[0]?.code).toBe('mvd.source');expect(report.schemaConformance).toBe('not-evaluated');
});
test('request refusal precedes source getters and rejects duplicate/open requests',async()=>{
 let reads=0;const source=Object.defineProperty({},'schema',{enumerable:true,get(){reads++;return 'stdio.ifc';}}) as IfcSnapshot;
 for(const input of [{targets:[]},{targets:[...request.targets,...request.targets]},{targets:[{layout:'1',kind:'other',applicable:true}]},{targets:[{layout:'1',kind:'horizontal'}]},{...request,geometry:true}])await expect(validateIfcAlignmentTerminal(source,input as IfcAlignmentTerminalRequest)).rejects.toMatchObject({code:'mvd.request'});expect(reads).toBe(0);
 const report=await validateIfcAlignmentTerminal(source,request);expect(report.status).toBe('incomplete');expect(reads).toBe(0);
});
test('captured source and request survive caller replacement after admission',async()=>{
 const source=graph(),input=structuredClone(request);let changed=false;const report=await validateIfcAlignmentTerminal(source,input,{batchSize:4096,progress:event=>{if(event.phase==='alignment-capture'&&event.completed===1&&event.total===1){changed=true;source.entities=[];(input.targets as unknown[]).length=0;}}});expect(changed).toBe(true);expect(report.status).toBe('pass');
});
test('nonfinite retained real values cannot fall through as terminal zero',async()=>{
 for(const bits of [0x7ff8000000000000n,0x7ff0000000000000n,0xfff0000000000000n]){const source=graph(),last=source.entities.at(-1)!;last.args[ifc4x3Catalog.entities[last.name]!.allAttributes.findIndex(item=>item.name==='SegmentLength')]={kind:'real',value:{bits}};expect((await validateIfcAlignmentTerminal(source,request,{batchSize:4096})).status).toBe('incomplete');}
});
for(const phase of ['alignment-request','alignment-index','alignment-relations','alignment-members','alignment-result'])for(const boundary of ['interior','completion'])test('original alignment cancellation '+phase+' '+boundary,async()=>{
 const source=graph(),before=structuredClone(source),original=new AbortController(),replacement=new AbortController(),input:IfcAlignmentTerminalRequest={targets:[...request.targets,...[2,3,4,5].map(id=>({layout:String(id),kind:'horizontal' as const,applicable:false}))]};let reached=false,foreignCalls=0;
 const control:IfcIoControl={signal:original.signal,batchSize:4,progress:event=>{if(event.phase!==phase||event.completed===0)return;const complete=event.total>0&&event.completed===event.total;if((boundary==='completion')!==complete)return;reached=true;control.signal=replacement.signal;control.progress=()=>foreignCalls++;original.abort();}};
 await expect(validateIfcAlignmentTerminal(source,input,control)).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);expect(foreignCalls).toBe(0);expect(source).toEqual(before);
});
for(const boundary of ['interior','completion'])test('original private alignment capture cancellation '+boundary,async()=>{
 const source=graph(),before=structuredClone(source),original=new AbortController();let reached=false;await expect(validateIfcAlignmentTerminal(source,request,{signal:original.signal,batchSize:1,progress:event=>{if(boundary==='interior'?event.phase!=='alignment-capture-copyOwned':event.phase!=='alignment-capture')return;reached=true;original.abort();}})).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);expect(source).toEqual(before);
});
test('unavailable source result remains under original cancellation',async()=>{
 const original=new AbortController();let reached=false;await expect(validateIfcAlignmentTerminal({} as IfcSnapshot,request,{signal:original.signal,batchSize:1,progress:event=>{if(event.phase==='alignment-result'&&event.completed>0){reached=true;original.abort();}}})).rejects.toMatchObject({code:'io.cancelled'});expect(reached).toBe(true);
});
test('initial cancellation and callback faults preserve their original authority',async()=>{
 const original=new AbortController();original.abort();await expect(validateIfcAlignmentTerminal(graph(),request,{signal:original.signal})).rejects.toMatchObject({code:'io.cancelled'});const fault=new Error('actual observer');await expect(validateIfcAlignmentTerminal(graph(),request,{progress:()=>{throw fault;}})).rejects.toBe(fault);
});
test('official Alignment original edits through central diffs inverse mutations and STEP',async()=>{
 const file='sectioned-solid-horizontal.ifc',entry=official.cases.find(row=>row.file===file)!,bytes=await readFile(new URL('../../../../🧪️fixtures/official/'+file,import.meta.url));expect(createHash('sha256').update(bytes).digest('hex')).toBe(entry.sha256);
 const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),input:IfcAlignmentTerminalRequest={targets:[{layout:'34',kind:'horizontal',applicable:true},{layout:'45',kind:'vertical',applicable:true}]};expect(source.entities.length).toBe(169);expect((await validateIfcAlignmentTerminal(source,input,{batchSize:4096})).targets.map(row=>row.status)).toEqual(['pass','pass']);
 const ordinal=source.entities.findIndex(row=>row.id===1102n),parameter=source.entities[ordinal]!,index=ifc4x3Catalog.entities[parameter.name]!.allAttributes.findIndex(row=>row.name==='SegmentLength'),mutation={mutation:'setEntityArg' as const,id:1102n,index,value:ifcReal(5)},options={yieldEvery:4096};
 const diff=await ifcMutationDiffAsync(source,mutation,options),after=await applyIfcDiffAsync(source,diff,options),wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?item.toString():item));expect(wire(after)).toEqual(applyPatch(wire(source),[{op:'replace',path:'/entities/'+ordinal+'/args/'+index,value:wire(mutation.value)}]).newDocument);expect((await validateIfcAlignmentTerminal(after,input,{batchSize:4096})).targets.map(row=>row.status)).toEqual(['fail','pass']);
 expect(await parseIfc4x3Step(await writeIfc4x3Step(after,{batchSize:4096}),{batchSize:4096})).toEqual(after);expect(await applyIfcDiffAsync(after,await inverseIfcDiffAsync(source,diff,options),options)).toEqual(source);expect(await applyIfcMutationAsync(after,await inverseIfcMutationAsync(source,mutation,options),options)).toEqual(source);expect((await validateIfcAlignmentTerminal(source,input,{batchSize:4096})).status).toBe('pass');
});
