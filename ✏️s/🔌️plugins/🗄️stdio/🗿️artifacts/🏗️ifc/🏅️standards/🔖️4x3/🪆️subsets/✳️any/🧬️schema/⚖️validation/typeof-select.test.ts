/** 🎨️ Actual released SELECT relationships enforce named formal propositions. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './typeof-select-cases.json';
import schema from './typeof-select.schema.json';
import models from './models.json';
import {ExpressEvaluator} from './evaluator.ts';
import {parseExpressExpression} from './parser.ts';
import {validateIfc4x3Express} from './population.ts';
import {parseIfc4x3Step} from '../../🚪️io/step.ts';
import {readFile} from 'node:fs/promises';
import {ifc4x3Catalog} from '../📚️catalog/🟦️.ts';
test('TYPEOF SELECT closure vectors have a closed schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
test('immutable native SELECT parent index retains every released direct edge',async()=>{
 const expected=new Map<string,string[]>();for(const type of Object.values(ifc4x3Catalog.types))if(type.kind==='select')for(const member of type.members){const parents=expected.get(member)??[];parents.push(type.name.toUpperCase());expected.set(member,parents);}
 const rows=[...expected].map(([name,parents])=>[name,parents.sort()]).sort(([a],[b])=>String(a)<String(b)?-1:String(a)>String(b)?1:0),source=await readFile(new URL('select-parents.rs',import.meta.url),'utf8');
 const actual=[...source.matchAll(/^\s*\("([^"]+)", &\[(.*)\]\),$/gm)].map(match=>[match[1],JSON.parse('['+match[2]+']')]);expect(actual).toEqual(rows);
});
for(const row of cases.expressions)test('TYPEOF SELECT '+row.id,async()=>{
 const value=await new ExpressEvaluator().evaluate(parseExpressExpression(row.expression));expect(value).toEqual({kind:'logical',value:String(row.expected)});
});
for(const row of cases.seeds)test('TYPEOF explicit seed '+row.id,async()=>{
 const types=[...row.names],before=[...types],result=await new ExpressEvaluator().call('TYPEOF',[{kind:'entity',types,identity:1n}]);
 expect(result.kind).toBe('aggregate');expect(result.aggregate).toBe('SET');const names=(result.value as {kind:string;value:string}[]).map(value=>value.value);expect(new Set(names).size).toBe(names.length);
 for(const name of row.includes)expect(names).toContain(name);for(const name of row.excludes)expect(names).not.toContain(name);expect(types).toEqual(before);
});
for(const row of cases.models)test('official MaxOneColour '+row.id,async()=>{
 const source=await parseIfc4x3Step(models.header+row.body+models.footer),before=structuredClone(source),report=await validateIfc4x3Express(source);
 expect(report.complete).toBe(true);expect(report.valid).toBe(row.valid);expect(String(report.results.find(result=>result.entityId===3n&&result.rule===row.rule)?.result)).toBe(row.result);expect(source).toEqual(before);
});
test('TYPEOF SELECT preparation respects work and cancellation budgets',async()=>{
 const value={kind:'entity' as const,types:['IFCCOLOURRGB','IFCCOLOURSPECIFICATION','IFCPRESENTATIONITEM'],identity:1n};
 await expect(new ExpressEvaluator(undefined,{maximumWork:4}).call('TYPEOF',[value])).rejects.toHaveProperty('code','work-budget');
 const abort=new AbortController();let cancelled=false;
 await expect(new ExpressEvaluator(undefined,{signal:abort.signal,batchSize:1,progress:progress=>{if(progress.completed>=5){cancelled=true;abort.abort();}}}).call('TYPEOF',[value])).rejects.toHaveProperty('code','cancelled');expect(cancelled).toBe(true);
});

