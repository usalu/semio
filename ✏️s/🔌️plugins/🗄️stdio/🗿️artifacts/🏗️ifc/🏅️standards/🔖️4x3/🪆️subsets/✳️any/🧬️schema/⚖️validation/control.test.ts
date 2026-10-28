/** 🚦️ Genuine released programs retain original controls across compiler and semantic publication boundaries. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import Ajv2020 from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import cases from './control-cases.json';
import schema from './control.schema.json';
import astSchema from './ast.schema.json';
import {compileIfc4x3Express,validateIfc4x3Express,validateIfc4x3Domains,ExpressEvaluator,ExpressEvaluationError,ExpressParser,parseExpressExpression,type ExpressControl} from './index.ts';
import {hostContinuations} from '@semio-tech/framework-async/continuation';
import {IfcEditError} from '../🔺️diff/🟦️.ts';
import {ifcAlignmentDirectFixture} from '../👁️mvd/🧩️concepts/📐️alignment/direct-fixture.ts';
const programPromise=compileIfc4x3Express({batchSize:4096}).then(compiled=>{if(!compiled.complete)throw Error('Actual released program did not compile');return compiled.program;});
const source=()=>ifcAlignmentDirectFixture({id:'zero-length'}).snapshot;
const fault=(kind:string):unknown=>kind==='error'?new Error('original observer'):kind==='edit-invalid'?new IfcEditError('invalidValue','original observer'):kind==='edit-cancelled'?new IfcEditError('cancelled','original observer'):kind==='evaluation-cancelled'?new ExpressEvaluationError('cancelled','original observer'):kind==='number'?23:undefined;
test('closed control vectors use the genuine completed released program and physically valid source',async()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(cases)).toBe(true);const program=await programPromise;expect(new Ajv({strict:true}).compile(astSchema)(program)).toBe(true);expect(program.rules).toHaveLength(777);expect(program.functions).toHaveLength(48);
 const result=await validateIfc4x3Express(source(),{batchSize:4096},program);expect(result.complete).toBe(true);expect(result.valid).toBe(true);expect(result.diagnostics).toEqual([]);
});
for(const row of cases.cases)test('original EXPRESS '+row.id,async()=>{
 const program=await programPromise,snapshot=source(),before=structuredClone(snapshot),failure=fault(row.fault);let reached=false,caught:unknown=Symbol('not caught');
 const control:ExpressControl={batchSize:row.endpoint==='compiler'?4096:4,progress:event=>{if(event.phase===row.phase&&(row.boundary==='first'||event.total>0&&event.completed===event.total)){reached=true;throw failure;}}};
 try{if(row.endpoint==='population')await validateIfc4x3Express(snapshot,control,program);else if(row.endpoint==='compiler')await compileIfc4x3Express(control);else if(row.endpoint==='domains')await validateIfc4x3Domains(snapshot,control);else await new ExpressEvaluator(program,control).evaluate(parseExpressExpression('SIZEOF([1:20])'));}catch(error){caught=error;}
 expect(reached).toBe(true);expect(caught).toBe(failure);expect(snapshot).toEqual(before);
});
for(const endpoint of ['population','compiler','domains','evaluator'])test(endpoint+' captures the original observer once',async()=>{
 const program=await programPromise;let original=0,replacement=0;const control:ExpressControl={batchSize:4,progress:()=>{original++;control.progress=()=>replacement++;}};
 if(endpoint==='population')expect((await validateIfc4x3Express(source(),control,program)).valid).toBe(true);else if(endpoint==='compiler')expect((await compileIfc4x3Express(control)).complete).toBe(true);else if(endpoint==='domains')expect(await validateIfc4x3Domains(source(),control)).toEqual([]);else expect((await new ExpressEvaluator(program,control).evaluate(parseExpressExpression('SIZEOF([1:20])'))).value).toBe(20n);
 expect(original).toBeGreaterThan(1);expect(replacement).toBe(0);
});
test('population retains the original signal after caller replacement',async()=>{
 const original=new AbortController(),replacement=new AbortController();let replaced=false,aborted=false;const control:ExpressControl={signal:original.signal,batchSize:4,progress:event=>{if(event.phase==='admit'&&!replaced){replaced=true;control.signal=replacement.signal;}if(event.phase==='normalize'&&!aborted){aborted=true;original.abort();}}},result=await validateIfc4x3Express(source(),control,await programPromise);
 expect(replaced).toBe(true);expect(aborted).toBe(true);expect(result.complete).toBe(false);expect(result.valid).toBe(false);expect(replacement.signal.aborted).toBe(false);
});
test('original population work grant cannot be enlarged from a callback',async()=>{
 const program=await programPromise,snapshot=source(),baseline=await validateIfc4x3Express(snapshot,{batchSize:4096,maximumWork:1000},program);expect(baseline.complete).toBe(false);let changed=false;
 const control:ExpressControl={batchSize:4096,maximumWork:1000,progress:event=>{if(event.phase==='admit'&&!changed){changed=true;control.maximumWork=100000;}}},actual=await validateIfc4x3Express(snapshot,control,program);expect(changed).toBe(true);expect(actual.complete).toBe(false);expect(actual.valid).toBe(false);expect(compare(baseline,actual)).toEqual([]);
});
test('evaluator work depth and iteration limits are retained independently of the caller object',async()=>{
 const work:ExpressControl={maximumWork:4,batchSize:1,progress:()=>{work.maximumWork=100000;}};await expect(new ExpressEvaluator(undefined,work).evaluate(parseExpressExpression('SIZEOF([1:100])'))).rejects.toMatchObject({code:'work-budget'});
 const depth:ExpressControl={maximumDepth:1,batchSize:1,progress:()=>{depth.maximumDepth=256;}};await expect(new ExpressEvaluator(undefined,depth).evaluate(parseExpressExpression('1+(2*3)'))).rejects.toMatchObject({code:'depth'});
 expect((await new ExpressEvaluator(undefined,{maximumIterations:0}).evaluate(parseExpressExpression('1'))).value).toBe(1n);
});
for(const phase of ['admit','normalize','structure','evaluate','validation-result'])test('genuine population cancellation cannot publish complete success at '+phase,async()=>{
 const controller=new AbortController();let reached=false;const result=await validateIfc4x3Express(source(),{signal:controller.signal,batchSize:4,progress:event=>{if(event.phase===phase){reached=true;controller.abort();}}},await programPromise);expect(reached).toBe(true);expect(result.complete).toBe(false);expect(result.valid).toBe(false);
});
test('compiler completion cancellation checks the original signal after its final observer',async()=>{
 const controller=new AbortController();let reached=false;await expect(compileIfc4x3Express({signal:controller.signal,batchSize:4096,progress:event=>{if(event.phase==='compile'&&event.total>0&&event.completed===event.total){reached=true;controller.abort();}}})).rejects.toMatchObject({code:'cancelled'});expect(reached).toBe(true);
});
test('standalone domain completion cancellation retains its rejection contract',async()=>{
 const controller=new AbortController();let reached=false;await expect(validateIfc4x3Domains(source(),{signal:controller.signal,batchSize:4,progress:event=>{if(event.phase==='structure'&&event.total>0&&event.completed===event.total){reached=true;controller.abort();}}})).rejects.toMatchObject({code:'cancelled'});expect(reached).toBe(true);
});
for(const field of ['batchSize','maximumWork','maximumDepth','maximumIterations'] as const)for(const value of [NaN,Infinity,-1,1.5,null])test('invalid finite EXPRESS control '+field+' '+String(value),async()=>{
 const control={[field]:value} as unknown as ExpressControl;await expect(compileIfc4x3Express(control)).rejects.toMatchObject({code:'control'});
});
test('zero work and depth deny charged evaluation while batch zero is invalid',async()=>{
 await expect(new ExpressEvaluator(undefined,{maximumWork:0}).evaluate(parseExpressExpression('1'))).rejects.toMatchObject({code:'work-budget'});await expect(new ExpressEvaluator(undefined,{maximumDepth:0}).evaluate(parseExpressExpression('1'))).rejects.toMatchObject({code:'depth'});await expect(compileIfc4x3Express({batchSize:0})).rejects.toMatchObject({code:'control'});
});
test('zero compiler depth denies every parsed expression while retaining inverse metadata',async()=>{
 const compiled=await compileIfc4x3Express({maximumDepth:0,batchSize:4096}),program=compiled.program,expected=cases.zeroDepth;
 expect(compiled.complete).toBe(false);for(const rows of [program.rules,program.functions,program.globalRules,program.derived,program.unique,program.bounds])expect(rows.length).toBe(expected.expressionRecords);
 expect(program.inverse).toHaveLength(expected.inverseMetadata);expect(program.inverse.every(row=>!row.lower&&!row.upper)).toBe(true);let accepted=true;try{parseExpressExpression('1',0);}catch{accepted=false;}expect(accepted).toBe(expected.helperAcceptsExpression);
});
for(const endpoint of ['population','compiler','domains','evaluator'])test(endpoint+' reads each original control property exactly once',async()=>{
 const program=await programPromise,reads=new Map<string,number>(),values:ExpressControl={signal:new AbortController().signal,batchSize:4096,maximumWork:1000000,maximumDepth:256,maximumIterations:1000,progress:()=>{}},control={};for(const key of Object.keys(values) as (keyof ExpressControl)[])Object.defineProperty(control,key,{get:()=>{reads.set(key,(reads.get(key)??0)+1);return values[key];}});
 if(endpoint==='population')expect((await validateIfc4x3Express(source(),control,program)).valid).toBe(true);else if(endpoint==='compiler')expect((await compileIfc4x3Express(control)).complete).toBe(true);else if(endpoint==='domains')expect(await validateIfc4x3Domains(source(),control)).toEqual([]);else expect((await new ExpressEvaluator(program,control).evaluate(parseExpressExpression('1'))).value).toBe(1n);
 expect([...reads.values()]).toEqual(Object.keys(values).map(()=>1));
});
test('original iteration denial cannot be enlarged by a progress observer',async()=>{
 const control:ExpressControl={maximumIterations:0,batchSize:1,progress:()=>{control.maximumIterations=1000;}},parser=new ExpressParser('REPEAT i:=1 TO 2; END_REPEAT;'),statements=parser.statements(new Set());parser.done();await expect(new ExpressEvaluator(await programPromise,control).statements(statements,new Map())).rejects.toMatchObject({code:'iteration-budget'});
});
test('compiler yields through the real host scheduler before publishing an AST',async()=>{
 const controller=new AbortController();let ran=false;const cancel=hostContinuations.schedule(()=>{ran=true;controller.abort();},0);try{await expect(compileIfc4x3Express({signal:controller.signal,batchSize:4096})).rejects.toMatchObject({code:'cancelled'});expect(ran).toBe(true);}finally{cancel();}
});
