/** 🪢️ Released predicates consume declared source owners without inventing positional values. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './query-cases.json';
import schema from './query.schema.json';
import {validateIfcBodyTessellation} from './🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
test('declared tessellation query source cases have a closed neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('tessellation original declared slots '+row.id,async()=>{
 const args:IfcValue[]=[{kind:'unset'},{kind:'string',value:'Body'},{kind:'string',value:'Tessellation'},{kind:'aggregate',value:[{kind:'reference',value:2n}]}],complex=[{name:cases.owner,args:row.id==='flattened-complex-primary'?[]:args}];if(row.id==='duplicate-owner'||row.id==='late-unknown')complex.push({name:cases.owner,args});if(row.id==='late-unknown')complex.push({name:'UNKNOWN',args:[]});const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[{id:1n,name:'IFCSHAPEREPRESENTATION',args:row.id==='flattened-complex-primary'?args:[],complex},{id:2n,name:'IFCTRIANGULATEDFACESET',args:[]}]};
 const report=await validateIfcBodyTessellation(source,{targets:[{representation:'1',applicable:true}]});expect(String(report.status)).toBe(row.status);expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');if(row.code)expect(report.targets[0]!.diagnostics.some(diagnostic=>diagnostic.code===row.code)).toBe(true);
});
