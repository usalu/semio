/** 🗂️ Closed native population laws preserve exact identities without granting conformance. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './population-cases.json';
import schema from './population.schema.json';
import {ifcConceptIndexSteps} from './🟦️.ts';
import type {IfcSnapshot} from '../../📸️snapshot/🟦️.ts';
const validate=new Ajv({strict:true}).compile(schema);
test('prepared population neutral cases have a closed language-neutral contract',()=>{expect(validate(cases)).toBe(true);expect(cases.cases).toHaveLength(8);expect(cases.conformance).toBe('not-evaluated');});
test('population case IDs preserve canonical exact u64 words',()=>{for(const word of ['01','-1','18446744073709551616','1'.repeat(21)]){const input=structuredClone(cases);input.cases[0]!.queries=[word];expect(validate(input)).toBe(false);}for(const word of ['0','9007199254740993','18446744073709551615']){const input=structuredClone(cases);input.cases[0]!.queries=[word];expect(validate(input)).toBe(true);}});
test('a population lookup outcome never fabricates ordinal or full acceptance',()=>{const input=structuredClone(cases)as unknown as Record<string,unknown>;input.conformance='valid';expect(validate(input)).toBe(false);const unique=structuredClone(cases);delete(unique.cases[1]!.expected.matches[0]as {ordinal?:number}).ordinal;expect(validate(unique)).toBe(false);const ambiguous=structuredClone(cases);(ambiguous.cases[2]!.expected.matches[0]as {ordinal?:number}).ordinal=0;expect(validate(ambiguous)).toBe(false);});
test('unavailable container cases cannot contain published lookups',()=>{const input=structuredClone(cases);input.cases[5]!.expected.matches=[{id:'1',status:'missing'}];expect(validate(input)).toBe(false);const unknown=structuredClone(cases)as unknown as Record<string,unknown>;unknown.admit=true;expect(validate(unknown)).toBe(false);});
test('prepared neutral lookup outcomes match the existing shared TypeScript source index',()=>{for(const row of cases.cases){if(row.expected.container!=='prepared')continue;const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:row.header}]}]},entities:row.entities.map(entity=>({id:BigInt(entity.id),name:entity.name,args:[],complex:[]}))},before=structuredClone(source),steps=ifcConceptIndexSteps(source);let index:ReturnType<typeof steps.next>;do{index=steps.next();}while(!index.done);const observed=row.queries.map(id=>{const value=index.value.get(BigInt(id));return value===undefined?{id,status:'missing'}:value===null?{id,status:'ambiguous'}:{id,status:'unique',ordinal:value.ordinal};});expect(observed).toEqual(row.expected.matches);expect(source).toEqual(before);}});

