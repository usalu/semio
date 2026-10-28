/** 🪟 Shared native root laws retain actual source and derived dependency semantics. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import contract from './root-read-cases.json';
import schema from './root-read-cases.schema.json';
import memo from './root-memo-cases.json';
import memoSchema from './root-memo-cases.schema.json';
import {compileIfc4x3Express,ExpressParser,IfcExpressContext} from './index.ts';
import {parseIfc4x3Step} from '../../🚪️io/step.ts';
import {numericOwnerSnapshot} from './🧪️numeric/fixture.ts';
test('retained root read and memo contracts are closed',()=>{const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(contract)).toBe(true);expect(ajv.compile(memoSchema)(memo)).toBe(true);});
for(const row of memo.cases)test('actual derived root dependency '+row.id,async()=>{const program=(await compileIfc4x3Express()).program,p=new ExpressParser(row.procedure.replace(/END_FUNCTION;\s*$/,''));p.require('FUNCTION');const probe=p.function(p.take().text),context=new IfcExpressContext(await parseIfc4x3Step(await Bun.file(new URL(row.source,import.meta.url)).text()),{...program,functions:[...program.functions,probe]});await context.initialize();const before=[...context.entities].map(([id,value])=>[String(id),numericOwnerSnapshot(value)]),result=await context.evaluator.call('PROBE',[context.entities.get(BigInt(row.entity))!]);expect(String(result.value)).toBe(row.expected);expect([...context.entities].map(([id,value])=>[String(id),numericOwnerSnapshot(value)])).toEqual(before);});
