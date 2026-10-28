/** 🧮 Original grants are authoritative over publicly observed evaluator state. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import cases from './state-cases.json';
import schema from './state.schema.json';
import {compileIfc4x3Express,ExpressEvaluator,ExpressParser,parseExpressExpression} from './index.ts';
const programPromise=compileIfc4x3Express({batchSize:4096}).then(row=>{if(!row.complete)throw Error('Actual released program failed');return row.program;});
test('closed state cases use actual standalone evaluation and genuine compiled program',async()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);expect((await programPromise).rules).toHaveLength(777);});
for(const row of cases.cases)test('original private evaluator '+row.id,async()=>{
 const run=async(mutate:boolean)=>{let evaluator:ExpressEvaluator,attempts=0,blocked=true;const control={batchSize:1,maximumWork:row.maximumWork,maximumDepth:row.maximumDepth,maximumIterations:row.maximumIterations,progress:()=>{if(!mutate)return;attempts++;const value=row.mutation==='control'?{...evaluator.control,maximumWork:100000,maximumDepth:256,maximumIterations:1000}:0,accepted=row.mode==='set'?Reflect.set(evaluator,row.mutation,value):Reflect.defineProperty(evaluator,row.mutation,{value,writable:true,configurable:true});blocked&&=!accepted;}};evaluator=new ExpressEvaluator(await programPromise,control);let code:string|undefined;try{if(row.expected==='iteration-budget'){const parser=new ExpressParser(row.source),statements=parser.statements(new Set());parser.done();await evaluator.statements(statements,new Map());}else await evaluator.evaluate(parseExpressExpression(row.source));}catch(error){code=error&&typeof error==='object'&&'code' in error?String(error.code):String(error);}return{report:{code,work:evaluator.work,depth:evaluator.depth},attempts,blocked};};
 const baseline=await run(false),actual=await run(true);expect(baseline.report.code).toBe(row.expected);expect(actual.attempts).toBeGreaterThan(0);expect(actual.blocked).toBe(true);expect(compare(baseline.report,actual.report)).toEqual([]);expect(actual.report.depth).toBe(0);
});
test('published state observations cannot be reassigned or redefined',async()=>{
 const evaluator=new ExpressEvaluator(await programPromise);expect(Object.isFrozen(evaluator.control)).toBe(true);for(const key of ['work','depth','control']){const descriptor=Object.getOwnPropertyDescriptor(evaluator,key)!;expect(descriptor.configurable).toBe(false);expect(Reflect.set(evaluator,key,Reflect.get(evaluator,key))).toBe(false);expect(Reflect.defineProperty(evaluator,key,{value:0})).toBe(false);}
 expect(evaluator.work).toBe(0);expect(evaluator.depth).toBe(0);
});
