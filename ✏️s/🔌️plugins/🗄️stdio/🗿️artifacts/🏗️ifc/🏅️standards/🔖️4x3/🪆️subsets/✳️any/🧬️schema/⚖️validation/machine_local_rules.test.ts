import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020.js';
import law from './machine-local-cases.json';
import schema from './machine-local-cases.schema.json';
import values from '../../../../../../🔎️details/🧪️tests/type-number-binary-input.json';
import {compileIfc4x3Express} from './compiler.ts';
import {IfcExpressContext} from './population.ts';
import {ifc4x3Catalog as catalog} from '../📚️catalog/🟦️.ts';
import type {IfcValue,IfcSnapshot} from '../📸️snapshot/🟦️.ts';
const compilation=compileIfc4x3Express({signal:new AbortController().signal});
function native(input:unknown):IfcValue {
 const raw=input as{kind:string,value?:unknown};
 if(raw.kind==='integer'||raw.kind==='reference')return{kind:raw.kind,value:BigInt(String(raw.value))};
 if(raw.kind==='real')return{kind:'real',value:{bits:BigInt((raw.value as{bits:string}).bits)}};
 if(raw.kind==='aggregate')return{kind:'aggregate',value:(raw.value as unknown[]).map(native)};
 return structuredClone(input)as IfcValue;
}
test('local rule contract is closed and retains honest native boundary',()=>{expect(new Ajv().compile(schema)(law)).toBe(true);expect(law.execution).toBe(false);});
for(const row of law.cases)test('issued actual local rule '+row.id,async()=>{
 const original=structuredClone(values.sources[row.source as keyof typeof values.sources]);
 const target=original.entities[0]!;
 const fixture=values.cases.find(value=>value.id===row.case),index=row.source==='binary'?1:0;
 target.args[index]=(fixture?.args[index]??{kind:'binary',value:row.case})as never;
 if(row.id==='preserved-format-unknown')target.args[0]={kind:'unset'}as never;
 const snapshot:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:original.entities.map(entity=>{
  const declaration=catalog.entities[entity.name]!;
  const args=entity.complex.length?declaration.allAttributes.map(attribute=>{const owner=entity.name===attribute.declaredBy.toUpperCase()?entity:entity.complex.find(part=>part.name===attribute.declaredBy.toUpperCase())!;const ordinal=catalog.entities[attribute.declaredBy.toUpperCase()]!.attributes.findIndex(item=>item.name===attribute.name);return native(owner.args[ordinal]);}):entity.args.map(native);
  return{id:BigInt(entity.id),name:entity.name,args,complex:[]};
 })};
 const issued=await compilation;expect(issued.complete).toBe(true);
 const context=new IfcExpressContext(snapshot,issued.program,{signal:new AbortController().signal});await context.initialize();
 const rule=issued.program.rules.find(rule=>rule.owner===row.owner&&rule.label===row.rule)!;expect(rule).toBeDefined();
 const entity=context.entities.get(1n)!;
 const self=row.source==='ratio'?entity.fields!.get('RED')!:entity;
 const result=await context.evaluator.evaluate(rule.expression,context.scope(self));
 expect(result.kind).toBe('logical');expect(result.value).toBe(row.status==='pass'?'true':row.status==='fail'?'false':'unknown');
});
