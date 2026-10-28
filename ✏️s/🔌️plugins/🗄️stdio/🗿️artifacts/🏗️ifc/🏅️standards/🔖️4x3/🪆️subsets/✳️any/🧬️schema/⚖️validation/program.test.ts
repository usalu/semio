/** 🔐 Full-schema validation retains only the exact immutable compiler-issued program. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import Ajv2020 from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import cases from './program-cases.json';
import schema from './program.schema.json';
import astSchema from './ast.schema.json';
import model from './models.json';
import {compileIfc4x3Express,validateIfc4x3Express,ExpressEvaluationError,type ExpressProgram} from './index.ts';
import {parseIfc4x3Step} from '../../🚪️io/step.ts';
import {IfcEditError} from '../🔺️diff/🟦️.ts';
import {hostContinuations} from '@semio-tech/framework-async/continuation';
const programPromise=compileIfc4x3Express({batchSize:4096}).then(result=>{if(!result.complete)throw Error('Actual released compiler failed');return result.program;});
const source=()=>parseIfc4x3Step(model.header+cases.source+model.footer);
const baselinePromise=Promise.all([source(),programPromise]).then(([snapshot,program])=>validateIfc4x3Express(snapshot,{batchSize:1},program));
const fault=(kind:string):unknown=>kind==='error'?Error('original seal observer'):kind==='edit-invalid'?new IfcEditError('invalidValue','original seal observer'):kind==='edit-cancelled'?new IfcEditError('cancelled','original seal observer'):kind==='evaluation-cancelled'?new ExpressEvaluationError('cancelled','original seal observer'):kind==='number'?23:undefined;
function attempt(program:ExpressProgram,kind:string):boolean{
 if(kind==='clear-rules'){Array.prototype.pop.call(program.rules);return true;}
 if(kind==='replace-rules')return Reflect.set(program,'rules',[]);
 if(kind==='expression-offset')return Reflect.set(program.rules[0]!.expression,'offset',999999);
 if(kind==='clear-statements'){Array.prototype.pop.call(program.functions.find(fn=>fn.body.length)!.body);return true;}
 if(kind==='delete-variable')return Reflect.deleteProperty(program.functions.find(fn=>fn.parameters.length)!.parameters[0]!,'type');
 return Reflect.set(program.inverse[0]!,'attribute','REMOVED');
}
test('closed program vectors retain the genuine released AST and independently named semantic failure',async()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(cases)).toBe(true);const program=await programPromise;expect(new Ajv({strict:true}).compile(astSchema)(program)).toBe(true);expect(program.rules).toHaveLength(777);const baseline=await baselinePromise;expect(baseline.complete).toBe(true);expect(baseline.valid).toBe(false);expect(baseline.diagnostics.map(row=>row.owner+'.'+row.rule)).toEqual([cases.expectedFailure]);
});
for(const row of cases.cases)test('issued program '+row.id,async()=>{
 const program=(await compileIfc4x3Express({batchSize:4096})).program,before=JSON.stringify(program);let reached=false,blocked=false;
 const result=await validateIfc4x3Express(await source(),{batchSize:1,progress:event=>{if(event.phase===row.phase&&!reached){reached=true;try{blocked=!attempt(program,row.mutation);}catch(error){if(!(error instanceof TypeError))throw error;blocked=true;}}}},program);
 expect(reached).toBe(true);expect(blocked).toBe(true);expect(JSON.stringify(program)).toBe(before);expect(compare(await baselinePromise,result)).toEqual([]);
});
test('every published program node and diagnostic record is immutable',async()=>{
 const compilation=await compileIfc4x3Express({maximumDepth:0,batchSize:4096}),objects:object[]=[compilation],seen=new Set<object>();let count=0;
 while(objects.length){const value=objects.pop()!;if(seen.has(value))continue;seen.add(value);expect(Object.isFrozen(value)).toBe(true);count++;for(const entry of Object.values(value))if(entry!==null&&typeof entry==='object')objects.push(entry);}
 expect(count).toBeGreaterThan(1000);const program=await programPromise,stack:object[]=[program];while(stack.length){const value=stack.pop()!;expect(Object.isFrozen(value)).toBe(true);for(const entry of Object.values(value))if(entry!==null&&typeof entry==='object')stack.push(entry);}
});
for(const kind of ['deep-clone','shallow-clone','empty-rosters'])test('unissued '+kind+' cannot assert complete IFC semantics',async()=>{
 const original=await programPromise,program=kind==='deep-clone'?structuredClone(original):kind==='shallow-clone'?{...original}:{schema:'IFC4X3_ADD2' as const,functions:[],globalRules:[],rules:[],derived:[],unique:[],bounds:[],inverse:[]},result=await validateIfc4x3Express(await source(),{},program);
 expect(result.complete).toBe(false);expect(result.valid).toBe(false);expect(result.diagnostics.some(row=>row.code==='express.program.origin')).toBe(true);expect(original.rules).toHaveLength(777);
});
test('an issued incomplete compilation never becomes a complete supplied program',async()=>{
 const compiled=await compileIfc4x3Express({maximumDepth:0,batchSize:4096});expect(compiled.complete).toBe(false);expect(compiled.diagnostics.length).toBeGreaterThan(100);const result=await validateIfc4x3Express(await source(),{},compiled.program);expect(result.complete).toBe(false);expect(result.valid).toBe(false);expect(result.diagnostics).toEqual([...compiled.diagnostics]);
});
for(const kind of ['error','edit-invalid','edit-cancelled','evaluation-cancelled','undefined','number'])test('original compiler seal refusal '+kind,async()=>{
 const failure=fault(kind);let reached=false,caught:unknown=Symbol();try{await compileIfc4x3Express({batchSize:4096,progress:row=>{if(row.phase==='compile-seal'){reached=true;throw failure;}}});}catch(error){caught=error;}expect(reached).toBe(true);expect(caught).toBe(failure);
});
test('original seal cancellation and original seal work denial publish no compilation',async()=>{
 const controller=new AbortController();let cancelled=false;await expect(compileIfc4x3Express({signal:controller.signal,batchSize:4096,progress:row=>{if(row.phase==='compile-seal'){cancelled=true;controller.abort();}}})).rejects.toMatchObject({code:'cancelled'});expect(cancelled).toBe(true);
 let reached=false;await expect(compileIfc4x3Express({maximumWork:cases.compilerSourceWork,batchSize:4096,progress:row=>{if(row.phase==='compile-seal')reached=true;}})).rejects.toMatchObject({code:'work-budget'});expect(reached).toBe(true);
});
test('sealing yields to the real host before a compilation is issued',async()=>{
 const controller=new AbortController();let ran=false,cancel:undefined|(()=>void);try{await expect(compileIfc4x3Express({signal:controller.signal,batchSize:4096,progress:row=>{if(row.phase==='compile-seal'&&!cancel)cancel=hostContinuations.schedule(()=>{ran=true;controller.abort();},0);}})).rejects.toMatchObject({code:'cancelled'});expect(ran).toBe(true);}finally{cancel?.();}
});
