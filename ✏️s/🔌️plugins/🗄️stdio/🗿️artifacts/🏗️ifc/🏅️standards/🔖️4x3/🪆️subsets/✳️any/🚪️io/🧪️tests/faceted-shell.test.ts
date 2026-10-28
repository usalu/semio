/** 🛡️ Declared shells retain connectivity, orientation and full surface separation. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './faceted-brep-cases.json';
import shellCases from './faceted-shell-cases.json';
import schema from './faceted-shell.schema.json';
import {ifcFacetedFixture,type IfcFacetedCase} from './faceted-brep-fixtures.ts';
import {extractIfc4x3Triangles} from '../geometry.ts';
import type {IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
test('shell rejection contract is closed',()=>{const valid=new Ajv({strict:true}).compile(schema);expect(valid(shellCases)).toBe(true);expect(valid({...shellCases,publish:true})).toBe(false);});
for(const row of shellCases.cases)test('faceted shell refuses '+row.id,async()=>{
 const data:IfcFacetedCase=structuredClone(cases.cases[0]!);if(row.change==='voids')data.geometry.voids=row.voids!.map(shell=>({...shell,holes:[]}));
 if(row.change==='join'){const[x,y,z]=row.offset!;data.geometry.voids=[{...structuredClone(data.geometry),outer:data.geometry.outer.map(p=>[p[0]!+x!,p[1]!+y!]),base:data.geometry.base+z!,reverse:false}];}
 const fixture=ifcFacetedFixture(data),reference=fixture.solid.args[0];if(reference?.kind!=='reference')throw Error('authored outer reference');const shell=fixture.snapshot.entities.find(entity=>entity.id===reference.value)!,faces=shell.args[0] as Extract<IfcValue,{kind:'aggregate'}>;
 if(row.change==='open')faces.value.pop();if(row.change==='duplicate')faces.value.push(faces.value[0]!);
 if(row.change==='join'){const voids=fixture.solid.args[1] as Extract<IfcValue,{kind:'aggregate'}>,otherRef=voids.value[0];if(otherRef?.kind!=='reference')throw Error('authored second shell');const other=fixture.snapshot.entities.find(entity=>entity.id===otherRef.value)!,members=other.args[0] as Extract<IfcValue,{kind:'aggregate'}>;faces.value.push(...members.value);fixture.solid.name='IFCFACETEDBREP';fixture.solid.args=[reference];}
 const before=structuredClone(fixture.snapshot),outcome=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});expect(outcome.artifact??[]).toEqual([]);expect(outcome.diagnostics.some(d=>d.severity==='error')).toBe(true);expect(fixture.snapshot).toEqual(before);
});
