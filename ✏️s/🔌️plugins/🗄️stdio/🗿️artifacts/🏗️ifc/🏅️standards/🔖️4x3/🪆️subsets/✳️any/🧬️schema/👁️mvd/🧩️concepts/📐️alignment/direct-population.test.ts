/** 🪢️ The actual selected Direct endpoint preserves labels and statuses across prepared/unavailable containers. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import laws from './direct-population-cases.json';
import schema from './direct-population.schema.json';
import {ifcAlignmentDirectFixture} from './direct-fixture.ts';
import {validateIfcAlignmentDirectCounterpart,type IfcAlignmentDirectRequest} from './direct.ts';
import type {IfcValue} from '../../../📸️snapshot/🟦️.ts';
test('Direct population integration has a closed language-neutral contract',()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(laws)).toBe(true);const forged=structuredClone(laws)as unknown as Record<string,unknown>;forged.conformance='valid';expect(validate(forged)).toBe(false);});
for(const row of laws.cases)test('actual Direct population integration '+row.id,async()=>{
 const graph=ifcAlignmentDirectFixture({id:'zero-length'}),source=graph.snapshot;
 if(row.variant==='header')source.header.fileSchema=[{kind:'aggregate',value:[{kind:'string',value:'IFC4'}]}];
 if(row.variant==='id')source.entities[0]!.id=0n;
 if(row.variant==='reference')source.entities[0]!.args.push({kind:'reference',value:0n}as IfcValue);
 if(graph.segment.kind!=='reference')throw new Error('fixture did not return its selected source reference');
 const selected=String(graph.segment.value),before=structuredClone(source),request={targets:laws.targets.map(target=>({...target,segment:target.segment==='selected'?selected:target.segment}))}as IfcAlignmentDirectRequest,original=structuredClone(request),report=await validateIfcAlignmentDirectCounterpart(source,request,{batchSize:4096});
 expect<string>(report.status).toBe(row.status);expect<string[]>(report.targets.map(target=>target.status)).toEqual(row.targets);expect<IfcAlignmentDirectRequest['targets']>(report.targets.map(({segment,kind,applicable})=>({segment,kind,applicable}))).toEqual(original.targets);expect<string[]>(report.diagnostics.map(value=>value.code)).toEqual(row.diagnostics);expect<string>(report.schemaConformance).toBe(laws.conformance);expect<string>(report.viewConformance).toBe(laws.conformance);expect(source).toEqual(before);expect(request).toEqual(original);
});
