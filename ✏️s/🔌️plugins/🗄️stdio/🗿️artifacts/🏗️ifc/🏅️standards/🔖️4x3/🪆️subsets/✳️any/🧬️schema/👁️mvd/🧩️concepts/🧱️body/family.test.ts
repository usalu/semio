/** 🧱️ Shared family vectors observe only item classes through actual concept endpoints. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import cases from './family-cases.json';
import schema from './family.schema.json';
import {validateIfcBodyTessellation} from '../🔺️tessellation/🟦️.ts';
import {validateIfcBodySweptSolid} from '../🌊️swept-solid/🟦️.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
test('closed shared family vectors never claim complete physical validity',()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);expect(cases.physicalValidity).toBe('not-claimed');});
for(const row of cases.cases)test('actual shared family '+row.id,async()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[{id:1n,name:'IFCSHAPEREPRESENTATION',args:[{kind:'unset'},{kind:'string',value:'Body'},{kind:'string',value:row.policy==='tessellation'?'Tessellation':'SweptSolid'},{kind:'aggregate',value:[{kind:'reference',value:2n}]}],complex:[]},{id:2n,name:row.names[0]!,args:[],complex:row.names.slice(1).map(name=>({name,args:[]}))}]},before=structuredClone(source),validate=row.policy==='tessellation'?validateIfcBodyTessellation:validateIfcBodySweptSolid;
 const report=await validate(source,{targets:[{representation:'1',applicable:true}]});expect(String(report.status)).toBe(row.expected);expect(report.schemaConformance).toBe('not-evaluated');expect(report.viewConformance).toBe('not-evaluated');expect(compare(before,source)).toEqual([]);
});
