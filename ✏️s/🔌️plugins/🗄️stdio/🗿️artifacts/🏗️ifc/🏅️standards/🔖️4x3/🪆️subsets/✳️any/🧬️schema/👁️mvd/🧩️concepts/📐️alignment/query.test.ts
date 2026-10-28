/** 🔎️ The business terminal clause consumes owner-qualified queries and preserves unknown discovery. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import cases from './query-cases.json';
import schema from './query.schema.json';
import reportSchema from './🔣️.json';
import {alignmentQueryFixture} from './query-fixture.ts';
import {validateIfcAlignmentTerminal} from './🟦️.ts';
import type {IfcAlignmentLayoutKind} from './types.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {ifcAttributeQuerySteps} from '../../../../../../../../📚️catalog/🔎️query/🟦️.ts';
const drain=<T>(steps:Generator<void,T,void>):T=>{for(;;){const row=steps.next();if(row.done)return row.value;}};
test('closed owner-query cases explicitly retain partial projection scope',()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);expect(cases.physicalConformance).toBe('not-evaluated');});
for(const row of cases.cases)test('business declared owner '+row.id,async()=>{
 const source=alignmentQueryFixture(row),before=structuredClone(source),result=await validateIfcAlignmentTerminal(source,{targets:[{layout:'1',kind:row.kind as IfcAlignmentLayoutKind,applicable:true}]},{batchSize:4096});
 expect(compare({status:row.expected},{status:result.status})).toEqual([]);expect<string|undefined>(result.targets[0]?.status).toBe(row.expected);expect(result.schemaConformance).toBe('not-evaluated');expect(result.viewConformance).toBe('not-evaluated');expect(result.geometryConformance).toBe('not-evaluated');expect(source).toEqual(before);expect(new Ajv({strict:true}).compile(reportSchema)(result)).toBe(true);
});
test('qualified relationship lookup borrows the exact canonical owner slot',()=>{
 const source=alignmentQueryFixture({kind:'horizontal',variant:'foreign-relationship-owner'}),entity=source.entities[1]!,query=drain(ifcAttributeQuerySteps(ifc4x3Catalog,{entity,ordinal:1},'RelatingObject','IFCRELNESTS'));
 expect(query.status).toBe('unique');expect(query.matches).toHaveLength(1);expect(query.matches[0]?.declaration.declaredBy).toBe('IfcRelNests');expect(query.matches[0]?.value).toBe(entity.args[0]);expect(query.matches[0]?.pointer).toBe('/entities/1/args/0');
 const unqualified=drain(ifcAttributeQuerySteps(ifc4x3Catalog,{entity,ordinal:1},'RelatingObject'));expect(unqualified.status).toBe('ambiguous');
 entity.complex!.push({name:'IFCUNKNOWN',args:[]});const unknown=drain(ifcAttributeQuerySteps(ifc4x3Catalog,{entity,ordinal:1},'RelatingObject','IFCRELNESTS'));expect(unknown.status).toBe('unknown');expect(unknown.matches[0]?.value).toBe(entity.args[0]);
});
test('closed terminal report admits an explicit unknown attribute diagnostic',async()=>{
 const source=alignmentQueryFixture({kind:'horizontal',variant:'baseline'}),baseline=await validateIfcAlignmentTerminal(source,{targets:[{layout:'1',kind:'horizontal',applicable:true}]}),diagnostic={code:'mvd.attribute.unknown',path:['RelatedObjects'],label:{en:'Unresolved declared owner.',de:'Ungeklärter deklarierter Eigentümer.'}},report={...baseline,status:'incomplete',targets:[{...baseline.targets[0]!,status:'incomplete',diagnostics:[diagnostic]}]};
 expect(new Ajv({strict:true}).compile(reportSchema)(report)).toBe(true);
});
