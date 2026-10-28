/** 🚆️ Actual terminal endpoint reference laws retain request rows and late unknown discovery across population preparation. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import laws from './terminal-population-cases.json';
import schema from './terminal-population.schema.json';
import {alignmentFixture} from './🧪️fixture.ts';
import {validateIfcAlignmentTerminal,type IfcAlignmentTerminalRequest} from './🟦️.ts';
test('Terminal population integration has a closed language-neutral contract',()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(laws)).toBe(true);const forged=structuredClone(laws)as unknown as Record<string,unknown>;forged.conformance='valid';expect(validate(forged)).toBe(false);});
for(const row of laws.cases)test('actual Terminal population integration '+row.id,async()=>{
 const source=alignmentFixture({kind:'horizontal',lengths:[10,0]});
 if(row.variant==='header')source.header.fileSchema=[{kind:'aggregate',value:[{kind:'string',value:'IFC4'}]}];
 if(row.variant==='id')source.entities[0]!.id=0n;
 if(row.variant==='reference')source.entities[0]!.args.push({kind:'reference',value:0n});
 if(row.variant==='unknown')source.entities.push({id:900n,name:'IFCUNKNOWN',args:[],complex:[]});
 const before=structuredClone(source),request=structuredClone({targets:laws.targets})as IfcAlignmentTerminalRequest,original=structuredClone(request),report=await validateIfcAlignmentTerminal(source,request,{batchSize:4096});
 expect<string>(report.status).toBe(row.status);expect<string[]>(report.targets.map(target=>target.status)).toEqual(row.targets);expect<IfcAlignmentTerminalRequest['targets']>(report.targets.map(({layout,kind,applicable})=>({layout,kind,applicable}))).toEqual(original.targets);expect<string[]>(report.diagnostics.map(value=>value.code)).toEqual(row.diagnostics);expect<string[]>(report.targets[0]!.diagnostics.map(value=>value.code)).toEqual(row.firstDiagnostics);expect<string>(report.schemaConformance).toBe(laws.conformance);expect<string>(report.viewConformance).toBe(laws.conformance);expect(source).toEqual(before);expect(request).toEqual(original);
});
