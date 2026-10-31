/** 🚆️ Released direct counterparts remain selected, reversible and distinct from whole-view acceptance. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {applyPatch} from 'fast-json-patch';
import cases from './direct-cases.json';
import caseSchema from './direct-cases.schema.json';
import schema from './direct.schema.json';
import sharedSchema from './🔣️.json';
import official from '../../../../🧪️fixtures/official/cases.json';
import {ifcAlignmentDirectFixture} from './direct-fixture.ts';
import {validateIfcAlignmentDirectCounterpart,ifcAlignmentDirectDefinition} from './direct.ts';
import type {IfcAlignmentDirectRequest,IfcAlignmentDirectTarget} from './direct-types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {parseIfc4x3Step,writeIfc4x3Step,ifcReal} from '../../../../🚪️io/step.ts';
import {ifcMutationDiffAsync,inverseIfcMutationAsync,applyIfcMutationAsync} from '../../../🧬️mutations/🟦️.ts';
import {applyIfcDiffAsync,inverseIfcDiffAsync,IfcEditError} from '../../../🔺️diff/🟦️.ts';
import {IfcStepError,type IfcIoControl} from '../../../../🚪️io/control.ts';
const fixture=()=>ifcAlignmentDirectFixture(cases.cases[0]!),request=(id:bigint):{targets:IfcAlignmentDirectTarget[]}=>({targets:[{segment:String(id),kind:'horizontal',applicable:true}]}),id=(value:{kind:string;value?:unknown})=>value.value as bigint;
const validate=new Ajv({strict:true}).addSchema(sharedSchema).compile(schema);
test('direct request definition report and language-neutral vectors have closed released schemas',async()=>{
 expect(new Ajv({strict:true}).compile(caseSchema)(cases)).toBe(true);expect(validate(ifcAlignmentDirectDefinition)).toBe(true);expect(Object.isFrozen(ifcAlignmentDirectDefinition.source)).toBe(true);
 const graph=fixture();expect(validate(request(id(graph.segment)))).toBe(true);expect(validate(await validateIfcAlignmentDirectCounterpart(graph.snapshot,request(id(graph.segment)),{batchSize:4096}))).toBe(true);
 for(const segment of ['0','01','-1','18446744073709551616'])expect(validate(request(segment as unknown as bigint))).toBe(false);
});
for(const row of cases.cases)test('selected direct counterpart '+row.id,async()=>{
 const graph=ifcAlignmentDirectFixture(row),source=graph.snapshot,before=structuredClone(source),input=request(id(graph.segment));input.targets[0]={...input.targets[0]!,kind:row.id==='wrong-kind'?'vertical':row.kind as 'horizontal',applicable:row.applicable};
 const report=await validateIfcAlignmentDirectCounterpart(source,input,{batchSize:4096});expect<string>(report.status).toBe(row.expected);expect<string|undefined>(report.targets[0]?.status).toBe(row.expected);expect(validate(report)).toBe(true);expect(source).toEqual(before);
 expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');expect(report.geometryConformance).toBe('not-evaluated');expect(report.layoutTerminalConformance).toBe('not-evaluated');expect(report.parentCurveConformance).toBe('not-evaluated');
});
test('direct request refusal precedes all source observation',async()=>{
 let reads=0;const source=Object.defineProperty({},'schema',{enumerable:true,get(){reads++;return 'stdio.ifc';}}) as IfcSnapshot,valid=request(1n);
 for(const input of [{targets:[]},{...valid,fullView:true},{targets:[...valid.targets,...valid.targets]},{targets:[{segment:'1',kind:'horizontal',applicable:null}]},{targets:[{segment:'18446744073709551616',kind:'cant',applicable:true}]}])await expect(validateIfcAlignmentDirectCounterpart(source,input as IfcAlignmentDirectRequest)).rejects.toMatchObject({code:'mvd.request'});expect(reads).toBe(0);
});
test('source failure is global incomplete even for explicit false targets',async()=>{
 const graph=fixture(),input=request(id(graph.segment));input.targets[0]={...input.targets[0]!,applicable:false};graph.snapshot.header.fileSchema=[];const report=await validateIfcAlignmentDirectCounterpart(graph.snapshot,input);expect(report.status).toBe('incomplete');expect(report.targets[0]?.status).toBe('not-applicable');expect(report.diagnostics[0]?.code).toBe('mvd.direct.source');
});
test('declared curve length pointer and input row ordering remain exact',async()=>{
 const graph=ifcAlignmentDirectFixture({id:'curve-nonzero-decoy'}),input=request(id(graph.segment));input.targets.push({segment:'9999',kind:'cant',applicable:false});graph.snapshot.entities.reverse();const report=await validateIfcAlignmentDirectCounterpart(graph.snapshot,input,{batchSize:4096}),ordinal=graph.snapshot.entities.findIndex(entity=>entity.id===id(graph.curve)),length=ifc4x3Catalog.entities.IFCCURVESEGMENT!.allAttributes.findIndex(attribute=>attribute.name==='SegmentLength');
 expect(report.targets.map(row=>row.status)).toEqual(['fail','not-applicable']);expect(report.targets[0]?.diagnostics).toContainEqual(expect.objectContaining({code:'mvd.direct.curve.nonzero',pointer:'/entities/'+ordinal+'/args/'+length}));
});
test('admitted request and graph survive later caller replacement',async()=>{
 const graph=fixture(),input=request(id(graph.segment));let changed=false;const report=await validateIfcAlignmentDirectCounterpart(graph.snapshot,input,{batchSize:4096,progress:event=>{if(event.phase==='alignment-direct-capture'&&event.completed===1){changed=true;graph.snapshot.entities=[];input.targets.length=0;}}});expect(changed).toBe(true);expect(report.status).toBe('pass');
});
test('incomplete target outranks local failure while retaining every ordered row verdict',async()=>{
 const graph=ifcAlignmentDirectFixture({id:'curve-nonzero-decoy'}),input=request(id(graph.segment));input.targets.push({segment:'9998',kind:'cant',applicable:false},{segment:'9999',kind:'vertical',applicable:true});
 const report=await validateIfcAlignmentDirectCounterpart(graph.snapshot,input,{batchSize:4096});expect(report.status).toBe('incomplete');expect(report.targets.map(row=>row.status)).toEqual(['fail','not-applicable','incomplete']);expect(validate(report)).toBe(true);
});
for(const [file,segment,kind,expected]of [['sectioned-solid-horizontal.ifc','1101','horizontal','pass'],['sectioned-solid-horizontal.ifc','2001','vertical','pass'],['linear-placement-of-signal.ifc','6001','cant','pass'],['segmented-reference-curve.ifc','149','cant','incomplete']]as const)test('original direct counterpart '+kind+' '+file,async()=>{
 const bytes=await readFile(new URL('../../../../🧪️fixtures/official/'+file,import.meta.url)),entry=official.cases.find(row=>row.file===file)!;expect(createHash('sha256').update(bytes).digest('hex')).toBe(entry.sha256);const source=await parseIfc4x3Step(bytes.toString('utf8'),{batchSize:4096}),report=await validateIfcAlignmentDirectCounterpart(source,{targets:[{segment,kind,applicable:true}]},{batchSize:4096});expect<string>(report.status).toBe(expected);
});
test('official direct geometry edits through central diff inverse mutation and STEP',async()=>{
 const source=await parseIfc4x3Step(await readFile(new URL('../../../../🧪️fixtures/official/sectioned-solid-horizontal.ifc',import.meta.url),'utf8'),{batchSize:4096}),input=request(1101n),ordinal=source.entities.findIndex(entity=>entity.id===1201n),index=ifc4x3Catalog.entities.IFCCURVESEGMENT!.allAttributes.findIndex(attribute=>attribute.name==='SegmentLength'),mutation={mutation:'setEntityArg' as const,id:1201n,index,value:{kind:'typedValue' as const,value:{name:'IFCLENGTHMEASURE',items:[ifcReal(1)]}}},options={yieldEvery:4096};
 const diff=await ifcMutationDiffAsync(source,mutation,options),edited=await applyIfcDiffAsync(source,diff,options),wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?String(item):item));expect(wire(edited)).toEqual(applyPatch(wire(source),[{op:'replace',path:'/entities/'+ordinal+'/args/'+index,value:wire(mutation.value)}]).newDocument);expect((await validateIfcAlignmentDirectCounterpart(edited,input,{batchSize:4096})).status).toBe('fail');
 const parsed=await parseIfc4x3Step(await writeIfc4x3Step(edited,{batchSize:4096}),{batchSize:4096});expect((await validateIfcAlignmentDirectCounterpart(parsed,input,{batchSize:4096})).status).toBe('fail');expect(await applyIfcDiffAsync(edited,await inverseIfcDiffAsync(source,diff,options),options)).toEqual(source);expect(await applyIfcMutationAsync(edited,await inverseIfcMutationAsync(source,mutation,options),options)).toEqual(source);
});
for(const phase of ['alignment-direct-request','alignment-direct-index','alignment-direct-targets','alignment-direct-result'])for(const frontier of ['initial','interior','completion'])for(const kind of ['edit','step','undefined','number','cancel'])test('direct original '+phase+' '+frontier+' '+kind,async()=>{
 const graph=fixture(),source=graph.snapshot,before=structuredClone(source),input=request(id(graph.segment)),controller=new AbortController(),replacement=new AbortController(),failure=kind==='edit'?new IfcEditError('invalidValue','actual observer'):kind==='step'?new IfcStepError('io.cancelled','actual observer'):kind==='number'?7:undefined;input.targets.push(...[1,2,3,4].map(n=>({segment:String(9990+n),kind:'cant' as const,applicable:false})));let reached=false,foreign=0,caught:unknown=Symbol('missing');
 const control:IfcIoControl={signal:controller.signal,batchSize:4,progress:event=>{const hit=frontier==='initial'?event.completed===0:frontier==='interior'?event.completed>0&&event.total===0:event.completed>0&&event.completed===event.total;if(event.phase===phase&&hit){reached=true;control.signal=replacement.signal;control.progress=()=>foreign++;if(kind==='cancel')controller.abort();else throw failure;}}};
 try{await validateIfcAlignmentDirectCounterpart(source,input,control);}catch(error){caught=error;}expect(reached).toBe(true);expect(foreign).toBe(0);if(kind==='cancel')expect(caught).toMatchObject({code:'io.cancelled'});else expect(caught).toBe(failure);expect(source).toEqual(before);
});
for(const phase of ['alignment-direct-request-copyOwned','alignment-direct-capture-copyOwned','alignment-direct-capture'])for(const failure of [undefined,29,new IfcEditError('invalidValue','original copy observer')])test('direct capture observer '+phase+' '+String(failure),async()=>{
 const graph=fixture(),input=request(id(graph.segment));let reached=false,caught:unknown=Symbol('missing');try{await validateIfcAlignmentDirectCounterpart(graph.snapshot,input,{batchSize:1,progress:event=>{if(event.phase===phase){reached=true;throw failure;}}});}catch(error){caught=error;}expect(reached).toBe(true);expect(caught).toBe(failure);
});
