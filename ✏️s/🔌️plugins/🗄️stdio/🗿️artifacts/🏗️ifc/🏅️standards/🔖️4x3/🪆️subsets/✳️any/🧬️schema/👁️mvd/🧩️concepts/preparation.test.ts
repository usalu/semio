/** 🎟️ Canonical preparation vectors exercise actual typed endpoint request admission before graph evaluation. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import laws from './preparation-cases.json';
import schema from './preparation.schema.json';
import {validateIfcAlignmentDirectCounterpart,type IfcAlignmentDirectRequest} from './📐️alignment/direct.ts';
import {validateIfcAlignmentTerminal,type IfcAlignmentTerminalRequest} from './📐️alignment/🟦️.ts';
import type {IfcSnapshot} from '../../📸️snapshot/🟦️.ts';
test('generic native preparation has a closed language-neutral request contract',()=>{expect(new Ajv({strict:true}).compile(schema)(laws)).toBe(true);});
for(const row of laws.cases)test('actual Direct and Terminal request admission '+row.id,async()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[]};
 const direct={targets:row.targets.map(target=>({segment:target.id,kind:target.kind,applicable:true}))}as IfcAlignmentDirectRequest,terminal={targets:row.targets.map(target=>({layout:target.id,kind:target.kind,applicable:true}))}as IfcAlignmentTerminalRequest;
 for(const run of [()=>validateIfcAlignmentDirectCounterpart(source,direct),()=>validateIfcAlignmentTerminal(source,terminal)]){if(row.admitted){const report=await run();expect(report.targets.length).toBe(row.targets.length);expect<string>(report.schemaConformance).toBe(laws.conformance);}else await expect(run()).rejects.toMatchObject({code:'mvd.request'});}
});
