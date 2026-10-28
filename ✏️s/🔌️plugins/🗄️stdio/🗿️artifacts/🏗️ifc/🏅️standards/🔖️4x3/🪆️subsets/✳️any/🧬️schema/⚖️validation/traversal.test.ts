/** 🧮 Original-controlled population cohorts and inverse cardinality use actual compiled declarations. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import cases from './traversal-cases.json';
import schema from './traversal.schema.json';
import models from './models.json';
import {compileIfc4x3Express,IfcExpressContext,validateIfc4x3Express} from './index.ts';
import type {ExpressControl} from './ast.ts';
import type {ExpressValue} from './evaluator.ts';
import {parseIfc4x3Step} from '../../🚪️io/step.ts';
const programPromise=compileIfc4x3Express({batchSize:4096}).then(row=>{if(!row.complete)throw Error('Actual schema compilation incomplete');return row.program;});
const parse=(body:string)=>parseIfc4x3Step(models.header+body+models.footer,{batchSize:4096});
const prepared=async(body:string,control:ExpressControl={})=>{const context=new IfcExpressContext(await parse(body),await programPromise,control);await context.initialize();return context;};
const identities=(value:ExpressValue)=>((value.value??[]) as ExpressValue[]).map(item=>String(item.identity));
const body=(name:string)=>cases.sources[name as keyof typeof cases.sources];
test('closed neutral traversal cases retain genuine program and explicit projection scope',async()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);expect((await programPromise).rules).toHaveLength(777);expect(cases.physicalValidity).toBe('not-claimed');});
for(const row of cases.cohorts)test('population cohort '+row.id,async()=>{
 const context=await prepared(body(row.source)),start=context.evaluator.work,value=await context.population(row.type);
 expect(value.kind).toBe('aggregate');expect(value.aggregate).toBe('SET');expect(compare(row.ids,identities(value))).toEqual([]);
 for(const item of value.value as ExpressValue[])expect(context.entities.get(item.identity as bigint)).toBe(item);
 expect(context.evaluator.work-start).toBeGreaterThanOrEqual(row.ids.length);
 expect(context.evaluator.work-start).toBeLessThanOrEqual(row.ids.length+2);
});
for(const row of cases.inverses)test('progressive inverse '+row.id,async()=>{
 const context=await prepared(body(row.source)),program=await programPromise,rule=program.inverse.find(rule=>rule.owner===row.owner&&rule.name===row.name)!;
 expect(rule).toBeDefined();const target=context.entities.get(BigInt(row.target))!,matches=await context.inverseValues(target,rule);
 expect(compare(row.ids,matches.map(item=>String(item.identity)))).toEqual([]);
 const projected=await context.attribute(target,row.name);expect(projected.kind==='unknown'?[]:[String(projected.identity)]).toEqual(row.ids.slice(0,1));
 const report=await validateIfc4x3Express(await parse(body(row.source)),{batchSize:4096},program);
 expect(report.complete).toBe(true);expect(report.results.some(item=>item.entityId===BigInt(row.target)&&item.kind==='inverse'&&item.owner===row.owner&&item.rule===row.name&&item.result===row.cardinality)).toBe(true);
});
for(const row of cases.globals)test('actual full global cohort '+row.id,async()=>{
 const report=await validateIfc4x3Express(await parse(row.body),{batchSize:4096},await programPromise);
 expect(report.complete).toBe(true);expect(String(report.results.find(item=>item.kind==='global'&&item.owner==='IFCSINGLEPROJECTINSTANCE')?.result)).toBe(row.result);
});
test('projected population arrays cannot mutate the retained population index',async()=>{
 const context=await prepared(cases.sources.two),first=await context.population('IFCMATERIALDEFINITION');(first.value as ExpressValue[]).length=0;
 expect(identities(await context.population('IFCMATERIALDEFINITION'))).toEqual(['1','10','11']);
});
const manyBody=()=>Array.from({length:cases.controls.population},(_,at)=>'#'+(at+1)+'=IFCCARTESIANPOINT((0.,0.));').join('');
test('population copy consumes the original finite work allowance',async()=>{
 const source=manyBody(),baseline=await prepared(source,{batchSize:4096}),start=baseline.evaluator.work,context=await prepared(source,{batchSize:4096,maximumWork:start+cases.controls.workAllowance});
 expect(context.evaluator.work).toBe(start);await expect(context.population('IFCCARTESIANPOINT')).rejects.toMatchObject({code:'work-budget'});
 expect(context.evaluator.work).toBe(start+cases.controls.workAllowance+1);
});
test('population copy cancellation and original observer refusal remain exact',async()=>{
 for(const thrown of ['cancel',undefined,Object.freeze({marker:'cohort-refusal'})]){
  const controller=new AbortController();let copying=false,calls=0;
  const context=await prepared(manyBody(),{signal:controller.signal,batchSize:cases.controls.batchSize,progress:()=>{if(copying&&++calls===2){if(thrown==='cancel')controller.abort();else throw thrown;}}});
  copying=true;let failed=false,actual:unknown;try{await context.population('IFCCARTESIANPOINT');}catch(error){failed=true;actual=error;}
  expect(failed).toBe(true);expect(calls).toBe(2);if(thrown==='cancel')expect(actual).toMatchObject({code:'cancelled'});else expect(actual).toBe(thrown);
 }
});
test('an actual host timer can cancel population projection between bounded batches',async()=>{
 const controller=new AbortController(),context=await prepared(manyBody(),{signal:controller.signal,batchSize:cases.controls.batchSize}),start=context.evaluator.work;
 const timer=setTimeout(()=>controller.abort(),0);try{await expect(context.population('IFCCARTESIANPOINT')).rejects.toMatchObject({code:'cancelled'});}finally{clearTimeout(timer);}
 expect(context.evaluator.work-start).toBeLessThan(cases.controls.population);
});
test('empty projected populations still honor original cancellation',async()=>{
 const controller=new AbortController(),context=await prepared(cases.sources.zero,{signal:controller.signal});controller.abort();
 await expect(context.population('IFCWALL')).rejects.toMatchObject({code:'cancelled'});
});
test('a cohort requires completed initialization and refuses partial failed state',async()=>{
 for(const fail of [false,true]){const context=new IfcExpressContext(await parse(cases.sources.two),await programPromise,fail?{maximumWork:4}:{});if(fail)await expect(context.initialize()).rejects.toMatchObject({code:'work-budget'});await expect(context.population('IFCMATERIALDEFINITION')).rejects.toMatchObject({code:fail?'work-budget':cases.initialization.absent});}
});
test('repeated initialization cannot duplicate source cohorts or incoming edges',async()=>{
 const context=await prepared(cases.sources.two);await expect(context.initialize()).rejects.toMatchObject({code:cases.initialization.repeated});expect(identities(await context.population('IFCMATERIALDEFINITION'))).toEqual(['1','10','11']);
});
test('inverse identity deduplication excludes another role and declared entity type',async()=>{
 const context=await prepared(cases.sources.two),program=await programPromise,rule=program.inverse.find(rule=>rule.owner==='IFCMATERIALLAYER'&&rule.name==='TOMATERIALLAYERSET')!,target=context.entities.get(1n)!,edges=context.incoming.get(1n)!;
 edges.push(edges[0]!,{entity:edges[0]!.entity,attribute:'OTHERROLE'},{entity:context.entities.get(9n)!,attribute:rule.attribute});
 const values=await context.inverseValues(target,rule);expect(values.map(value=>String(value.identity))).toEqual(['10','11']);values.length=0;expect((await context.inverseValues(target,rule)).map(value=>String(value.identity))).toEqual(['10','11']);
});
test('inverse matching consumes original work and cancellation for every incoming edge',async()=>{
 const program=await programPromise,rule=program.inverse.find(rule=>rule.owner==='IFCMATERIALLAYER'&&rule.name==='TOMATERIALLAYERSET')!,baseline=await prepared(cases.sources.one),controller=new AbortController();let querying=false,calls=0;
 const context=await prepared(cases.sources.one,{signal:controller.signal,batchSize:cases.controls.batchSize,maximumWork:baseline.evaluator.work+cases.controls.population,progress:()=>{if(querying&&++calls===2)controller.abort();}}),edge=context.incoming.get(1n)![0]!;
 context.incoming.set(1n,Array.from({length:cases.controls.population},()=>edge));querying=true;await expect(context.inverseValues(context.entities.get(1n)!,rule)).rejects.toMatchObject({code:'cancelled'});expect(calls).toBe(2);
 const finite=await prepared(cases.sources.one,{maximumWork:baseline.evaluator.work+cases.controls.workAllowance});finite.incoming.set(1n,Array.from({length:cases.controls.population},()=>edge));await expect(finite.inverseValues(finite.entities.get(1n)!,rule)).rejects.toMatchObject({code:'work-budget'});
});
