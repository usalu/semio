/** 🧊 The one published released catalog cannot lose authoritative schema metadata during callbacks. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv';
import Ajv2020 from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import {ifc4x3Catalog as catalog} from '../📚️catalog/🟦️.ts';
import catalogSchema from '../../../../../../📚️catalog/🧬️schema/🔣️.json';
import cases from './catalog-cases.json';
import schema from './catalog.schema.json';
import model from './models.json';
import {compileIfc4x3Express,validateIfc4x3Express} from './index.ts';
import {parseIfc4x3Step} from '../../🚪️io/step.ts';
const source=()=>parseIfc4x3Step(model.header+'#1=IFCDIRECTION((0.,0.,0.));'+model.footer);
const baselinePromise=Promise.all([compileIfc4x3Express({batchSize:4096}),source()]).then(async([compiled,snapshot])=>({compiled,report:await validateIfc4x3Express(snapshot,{batchSize:4096},compiled.program)}));
function mutation(kind:string):[object,string,unknown]{
 const direction=catalog.entities.IFCDIRECTION!;
 if(kind==='where-rules')return[direction,'whereRules',[]];
 if(kind==='direct-attribute')return[direction.attributes[0]!,'optional',true];
 if(kind==='inherited-attribute')return[catalog.entities.IFCALIGNMENTSEGMENT!.allAttributes[0]!,'declaredBy','REMOVED'];
 if(kind==='type-domain')return[catalog.types.IFCLABEL!.typeExpr!,'name','INTEGER'];
 if(kind==='select-members')return[catalog.types.IFCVALUE!,'members',[]];
 if(kind==='function-source')return[catalog.functions,'IFCNORMALISE','REMOVED'];
 if(kind==='global-source')return[catalog.rules,Object.keys(catalog.rules)[0]!,'REMOVED'];
 if(kind==='entity-roster')return[catalog.entities,'IFCDIRECTION',undefined];
 if(kind==='type-roster')return[catalog.types,'IFCLABEL',undefined];
 if(kind==='source-provenance')return[catalog.source,'sha256','REMOVED'];
 if(kind==='schema-name')return[catalog,'schema','REMOVED'];
 return[catalog.validation,'whereRules',true];
}
test('closed catalog vectors match the actual released schema and independent named failure',async()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(cases)).toBe(true);expect(new Ajv({strict:true}).compile(catalogSchema)(catalog)).toBe(true);const {compiled,report}=await baselinePromise;expect(compiled.complete).toBe(true);expect(compiled.program.rules).toHaveLength(cases.expectedRules);expect(compiled.program.functions).toHaveLength(cases.expectedFunctions);expect(report.complete).toBe(true);expect(report.valid).toBe(false);expect(report.diagnostics.map(row=>row.owner+'.'+row.rule)).toEqual([cases.expectedFailure]);
});
for(const row of cases.cases)test('canonical catalog '+row.id,async()=>{
 const baseline=await baselinePromise,[target,key,value]=mutation(row.mutation),original=Reflect.get(target,key);let reached=false,blocked=false;
 const progress=(event:{phase:string})=>{if(event.phase===row.phase&&!reached){reached=true;blocked=!Reflect.set(target,key,value);if(!blocked&&!Reflect.set(target,key,original))throw Error('Test fixture restoration failed');}};
 const compiled=row.phase==='compile-index'?await compileIfc4x3Express({batchSize:4096,progress}):baseline.compiled,result=await validateIfc4x3Express(await source(),{batchSize:4096,progress},compiled.program);
 expect(reached).toBe(true);expect(blocked).toBe(true);expect(compiled.program.rules).toHaveLength(cases.expectedRules);expect(compiled.program.functions).toHaveLength(cases.expectedFunctions);expect(compare(baseline.report,result)).toEqual([]);
});
test('all released catalog arrays, declarations and nested domains are frozen in place',()=>{
 const stack:object[]=[catalog],seen=new Set<object>();while(stack.length){const value=stack.pop()!;if(seen.has(value))continue;seen.add(value);expect(Object.isFrozen(value)).toBe(true);for(const child of Object.values(value))if(child!==null&&typeof child==='object')stack.push(child);}expect(seen.size).toBeGreaterThan(6004);expect(()=>Array.prototype.pop.call(catalog.types.IFCVALUE!.members)).toThrow(TypeError);
});
