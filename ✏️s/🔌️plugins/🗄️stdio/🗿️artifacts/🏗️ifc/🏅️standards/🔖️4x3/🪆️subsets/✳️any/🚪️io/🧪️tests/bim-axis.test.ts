/** 🏛️ Actual spatial exporters must emit valid paired Cartesian orientation. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './bim-axis-cases.json';
import schema from './bim-axis.schema.json';
import {ifcBimAxisFixture} from './bim-axis-fixtures.ts';
import {bimSpatialToIfc4x3,ifc4x3ToBimSpatial} from '../bim.ts';
test('BIM placement vectors obey their closed schema',()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test(`BIM exports normative ${row.id} rotation`,async()=>{
 const exported=await bimSpatialToIfc4x3(ifcBimAxisFixture(row.angle));expect(exported.artifact).toBeDefined();
 for(const axis of exported.artifact!.entities.filter(entity=>entity.name==='IFCAXIS2PLACEMENT3D'))expect(axis.args[1]?.kind==='unset').toBe(axis.args[2]?.kind==='unset');
 const imported=await ifc4x3ToBimSpatial(exported.artifact!);expect(imported.artifact).toBeDefined();const building=Object.values(imported.artifact!.buildings!)[0]!;
 expect(building.rotation).toBeCloseTo(row.angle,14);expect(building.origin).toEqual({x:1,y:2});expect(building.elevation).toBe(4);
});

