/** ⭕️ One fixed-size basis norm contract supports both represented circle implementations. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {Vector3} from 'three';
import cases from './radius-cases.json';
import schema from './radius.schema.json';
test('circle basis radius contract is closed',()=>{const valid=new Ajv({strict:true}).compile(schema);expect(valid(cases)).toBe(true);expect(valid({...cases,publish:true})).toBe(false);});
for(const row of cases.cases)test('circle basis '+row.id,async()=>{const {circleBasisRadius}=await import('../🟦️.ts'),actual=circleBasisRadius(row.radius,row.u as [number,number,number],row.v as [number,number,number]);if(row.expected===null){expect(actual).toBeUndefined();return;}expect(actual).toBeDefined();expect(actual!).toBeGreaterThanOrEqual(row.expected*(1-1e-14));expect(actual!).toBeLessThanOrEqual(row.expected*(1+1e-12));let reference=0;for(let at=0;at<8;at++){const angle=at*Math.PI/4,point=new Vector3(...row.u as [number,number,number]).multiplyScalar(row.radius*Math.cos(angle)).add(new Vector3(...row.v as [number,number,number]).multiplyScalar(row.radius*Math.sin(angle)));reference=Math.max(reference,point.length());}expect(reference/row.expected).toBeCloseTo(1,12);});
