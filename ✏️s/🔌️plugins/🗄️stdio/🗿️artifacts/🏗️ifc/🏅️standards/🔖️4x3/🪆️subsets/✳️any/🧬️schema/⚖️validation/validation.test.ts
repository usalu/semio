/** 🔬 Canonical AST and language-neutral semantic contracts. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "./ast.schema.json";
import cases from "./cases.json";
import {compileIfc4x3Express,parseExpressExpression,evaluateExpressExpression} from "./index.ts";

test("the complete official EXPRESS corpus compiles to the owned AST schema",async()=>{
  const compiled=await compileIfc4x3Express();
  expect(compiled.diagnostics).toEqual([]);
  expect(compiled.program.rules).toHaveLength(777);
  expect(compiled.program.derived).toHaveLength(60);
  expect(compiled.program.functions).toHaveLength(48);
  expect(compiled.program.globalRules).toHaveLength(2);
  const validate=new Ajv({allErrors:true}).compile(schema);
  expect(validate(compiled.program)).toBe(true);
});

for(const row of cases) test(`EXPRESS ${row.expression}`,async()=>{
  const result=await evaluateExpressExpression(parseExpressExpression(row.expression));
  expect(result.kind).toBe(row.kind);
  expect(String(result.value)).toBe(row.value);
});
import models from './models.json';
import {parseIfc4x3Step} from '../../🚪️io/step.ts';
import {validateIfc4x3Express,IfcExpressContext,ExpressEvaluator} from './index.ts';
const programPromise=compileIfc4x3Express().then(value=>value.program);
test('adversarial model matches independent named EXPRESS failures',async()=>{
 const snapshot=await parseIfc4x3Step(models.header+models.body.join('\n')+models.footer),program=await programPromise;
 const result=await validateIfc4x3Express(snapshot,{},program);
 expect(result.complete).toBe(true);expect(result.valid).toBe(false);
 expect([...new Set(result.diagnostics.map(row=>`${row.owner}.${row.rule}`))].sort()).toEqual(models.expectedFailures.toSorted());
 expect(result.results.some(row=>row.kind==='bound')).toBe(true);
});
for(const name of ['triangulated','alignment','structural'])test(`official ${name} EXPRESS differential`,async()=>{
 const source=await Bun.file(new URL(`../../🚪️io/🧪️tests/${name}.ifc`,import.meta.url)).text();
 const result=await validateIfc4x3Express(await parseIfc4x3Step(source),{},await programPromise);
 expect(result.complete).toBe(true);expect(result.diagnostics).toEqual([]);expect(result.results.length).toBeGreaterThan(100);
},30_000);
test('derived arrays preserve zero-based indices and value assignment isolation',async()=>{
 const snapshot=await parseIfc4x3Step(models.header+models.body.join('\n')+models.footer),context=new IfcExpressContext(snapshot,await programPromise);
 await context.initialize();const weights=await context.attribute(context.entities.get(13n)!,'WEIGHTS');
 expect(weights.aggregate).toBe('ARRAY');expect(weights.lower).toBe(0n);expect((weights.value as {value:number}[])[0]!.value).toBe(-1);
 const evaluator=context.evaluator;const direction=await evaluator.call('IFCDIRECTION',[{kind:'aggregate',value:[{kind:'real',value:2},{kind:'real',value:0},{kind:'real',value:0}]}]);
 const normal=await evaluator.call('IFCNORMALISE',[direction]);expect((normal.fields!.get('DIRECTIONRATIOS')!.value as {value:number}[])[0]!.value).toBe(1);
 expect((direction.fields!.get('DIRECTIONRATIOS')!.value as {value:number}[])[0]!.value).toBe(2);
});
test('budgets cancellation cyclic graphs and unsupported populations are explicit',async()=>{
 const controller=new AbortController();controller.abort();await expect(evaluateExpressExpression(parseExpressExpression('1+2'),new Map(),{signal:controller.signal})).rejects.toMatchObject({code:'cancelled'});
 await expect(evaluateExpressExpression(parseExpressExpression('SIZEOF([1:100])'),new Map(),{maximumWork:10})).rejects.toMatchObject({code:'work-budget'});
 const snapshot=await parseIfc4x3Step(models.header+'#1=IFCDIRECTION((1.,0.));'+models.footer);const cycle:any={kind:'aggregate',value:[]};cycle.value.push(cycle);snapshot.entities[0]!.args=[cycle];
 const result=await validateIfc4x3Express(snapshot,{},await programPromise);expect(result.complete).toBe(false);expect(result.valid).toBe(false);expect(result.diagnostics[0]!.code).toBe('express.admission');
});
test('complex IFC constructors execute the concrete leaf and inherited rules',async()=>{
 const source=models.header+'#1=(IFCDIRECTION((0.,0.,0.)) IFCGEOMETRICREPRESENTATIONITEM() IFCREPRESENTATIONITEM());'+models.footer;
 const result=await validateIfc4x3Express(await parseIfc4x3Step(source),{},await programPromise);
 expect(result.complete).toBe(true);expect(result.diagnostics.map(row=>`${row.owner}.${row.rule}`)).toEqual(['IFCDIRECTION.MAGNITUDEGREATERZERO']);
});
test('scalar inverse rejects two relationships as well as zero',async()=>{
 const source=models.header+models.body.join('\n')+'\n#16=IFCRELVOIDSELEMENT(\'0000000000000000000003\',$,$,$,#10,#10);\n#17=IFCRELVOIDSELEMENT(\'0000000000000000000004\',$,$,$,#10,#10);'+models.footer;
 const result=await validateIfc4x3Express(await parseIfc4x3Step(source),{},await programPromise);
 expect(result.complete).toBe(true);expect(result.results.some(row=>row.entityId===10n&&row.kind==='inverse'&&row.rule==='VOIDSELEMENTS'&&row.result==='false')).toBe(true);
});
test('known numeric overflow is incomplete rather than an indeterminate success',async()=>{
 await expect(evaluateExpressExpression(parseExpressExpression('1e308 * 1e308'))).rejects.toMatchObject({code:'numeric-overflow'});
});
import {ifc4x3Catalog} from '../📚️catalog/🟦️.ts';
test('every retained rule and function executes with explicit indeterminate inputs',async()=>{
 const program=await programPromise,declarations=Object.entries(ifc4x3Catalog.entities);
 const snapshot:any={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:declarations.map(([name,entity],index)=>({id:BigInt(index+1),name,args:entity.allAttributes.map(()=>({kind:'unset'})),complex:[]}))};
 const context=new IfcExpressContext(snapshot,program);await context.initialize();let rules=0,functions=0;
 for(const rule of program.rules){const at=declarations.findIndex(([name])=>name===rule.owner),self=at<0?{kind:'unknown' as const}:context.entities.get(BigInt(at+1))!;const result=await context.evaluator.evaluate(rule.expression,context.scope(self));expect(['logical','unknown']).toContain(result.kind);rules++;}
 for(const fn of program.functions){await context.evaluator.call(fn.name,fn.parameters.flatMap(parameter=>parameter.names.map(()=>({kind:'unknown' as const}))));functions++;}
 expect(rules).toBe(777);expect(functions).toBe(48);
});
test('all contextual bounds execute for rational curve and surface arrays',async()=>{
 const result=await validateIfc4x3Express(await parseIfc4x3Step(models.header+models.body.join('\n')+models.footer),{},await programPromise);
 const identities=new Set(result.results.filter(row=>row.kind==='bound').map(row=>`${row.owner}.${row.rule}`));
 expect(identities.size).toBe(6);expect(result.results.filter(row=>row.kind==='bound').every(row=>row.result==='true')).toBe(true);
});
import statementCases from './statements.json';
import {ExpressParser} from './parser.ts';
for(const row of statementCases)test(`statement ${row.name}`,async()=>{
 const program={...await programPromise,functions:[new ExpressParser(row.source).function(row.name)]};
 const evaluator=new ExpressEvaluator(program),result=await evaluator.call(row.name,row.arguments.map(value=>({kind:'integer',value:BigInt(value)})));
 expect(result.kind).toBe('integer');expect(String(result.value)).toBe(row.expected);
});

import matchingCases from './matching-cases.json';
import type {ExpressValue} from './evaluator.ts';
for(const row of matchingCases)test(`matching ${row.name}`,async()=>{
 const convert=(values:string[],kind:string):ExpressValue=>({kind:'aggregate',aggregate:kind as ExpressValue['aggregate'],value:values.map(value=>value==='?'?{kind:'unknown'}:{kind:'integer',value:BigInt(value)})});
 const evaluate=()=>new ExpressEvaluator().binary(row.operator,convert(row.left,row.leftKind),convert(row.right,row.rightKind));
 if(row.error)await expect(evaluate()).rejects.toMatchObject({code:row.error});else{const value=await evaluate();expect(value.kind).toBe(row.kind);expect(String(value.value)).toBe(row.value);}
});
test('TYPEOF constructors include all entity and defined type ancestors',async()=>{
 const evaluate=(source:string)=>evaluateExpressExpression(parseExpressExpression(source));
 for(const type of ['IFCCARTESIANPOINT','IFCPOINT','IFCGEOMETRICREPRESENTATIONITEM','IFCREPRESENTATIONITEM'])expect((await evaluate(`'IFC4X3_ADD2.${type}' IN TYPEOF(IfcCartesianPoint([0.,0.,0.]))`)).value).toBe('true');
 expect((await evaluate("'IFC4X3_ADD2.IFCLENGTHMEASURE' IN TYPEOF(IfcPositiveLengthMeasure(1.))")).value).toBe('true');
});
test('membership compares entity identities instead of equal constructor attributes',async()=>{
 const evaluator=new ExpressEvaluator(),a=await evaluator.call('IFCDIRECTION',[{kind:'aggregate',value:[{kind:'real',value:1},{kind:'real',value:0}]}]),b=await evaluator.call('IFCDIRECTION',[{kind:'aggregate',value:[{kind:'real',value:1},{kind:'real',value:0}]}]);
 expect((await evaluator.binary('=',a,b)).value).toBe('true');expect((await evaluator.binary('IN',a,{kind:'aggregate',aggregate:'SET',value:[b]})).value).toBe('false');
 expect((await evaluator.binary('*',{kind:'aggregate',aggregate:'SET',value:[a]},{kind:'aggregate',aggregate:'SET',value:[b]})).value).toEqual([]);
});
import structuralCases from '../../🧪️tests/schema-cases.json';
import {validateIfc4x3Domains} from './admission.ts';
for(const row of structuralCases)test(`controlled structural ${row.name}`,async()=>{
 const entities=JSON.parse(JSON.stringify(row.entities),(key,value)=>key==='id'||key==='bits'?BigInt(value):value?.kind==='integer'||value?.kind==='reference'?{...value,value:BigInt(value.value)}:value);
 const snapshot:any={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities};
 expect((await validateIfc4x3Domains(snapshot)).map(row=>row.code)).toEqual(row.codes);
});
test('structural admission cancellation interrupts actual domain traversal',async()=>{
 const snapshot=await parseIfc4x3Step(models.header+'#1=IFCPROPERTYENUMERATION(\'Large\',(IFCLABEL(\'A\')),$);'+models.footer);
 snapshot.entities[0]!.args[1]={kind:'aggregate',value:Array.from({length:20_000},(_,i)=>({kind:'typedValue',value:{name:'IFCLABEL',items:[{kind:'string',value:String(i)}]}}))};
 const controller=new AbortController();let progress=0;
 await expect(validateIfc4x3Domains(snapshot,{signal:controller.signal,batchSize:32,progress:row=>{expect(row.phase).toBe('structure');if(++progress===2)controller.abort();}})).rejects.toMatchObject({code:'cancelled'});
 expect(progress).toBe(2);
});
test('full semantic validation forwards real structural phase cancellation',async()=>{
 const snapshot=await parseIfc4x3Step(models.header+'#1=IFCDIRECTION((1.,0.,0.));'+models.footer),controller=new AbortController();let structure=false;
 const result=await validateIfc4x3Express(snapshot,{signal:controller.signal,batchSize:1,progress:row=>{if(row.phase==='structure'){structure=true;controller.abort();}}},await programPromise);
 expect(structure).toBe(true);expect(result.complete).toBe(false);expect(result.valid).toBe(false);
});
test('complex derived redeclarations cannot erase a supplied literal',async()=>{
 const source=models.header+'#1=IFCDIMENSIONALEXPONENTS(1,0,0,0,0,0,0);#2=(IFCNAMEDUNIT(#1,.LENGTHUNIT.) IFCSIUNIT($,.METRE.));'+models.footer;
 const result=await validateIfc4x3Express(await parseIfc4x3Step(source),{},await programPromise);
 expect(result.complete).toBe(false);expect(result.valid).toBe(false);expect(result.diagnostics[0]!.code).toBe('express.complex-derived');
});
import domainCases from './domain-cases.json';
for(const row of domainCases)test(`controlled domain ${row.name}`,async()=>{
 const snapshot=await parseIfc4x3Step(models.header+row.body+models.footer);
 expect((await validateIfc4x3Domains(snapshot)).map(row=>row.code)).toEqual(row.codes);
});
test('unordered equality cannot reuse one possible match for multiple elements',async()=>{
 const integer=(value:bigint):ExpressValue=>({kind:'integer',value}),array=(value:ExpressValue[]):ExpressValue=>({kind:'aggregate',aggregate:'ARRAY',value});
 const a:ExpressValue={kind:'aggregate',aggregate:'BAG',value:[array([{kind:'unknown'},integer(1n)]),array([{kind:'unknown'},integer(1n)])]},b:ExpressValue={kind:'aggregate',aggregate:'BAG',value:[array([integer(1n),integer(1n)]),array([integer(1n),integer(2n)])]};
 expect((await new ExpressEvaluator().equal(a,b)).value).toBe('false');
});
