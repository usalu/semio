/** 🧪️ Coordinate frame laws checked against Three.js independently of raster models. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Matrix3,Vector3} from "three";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {reframe,compose,decompose,type AffineControls} from "../🟦️.ts";
import controlsFixture from "../🧫️fixtures/🎛️components.json";
import controlsSchema from "../🧬️schema/🎛️components.json";
import {multiply,type CompositeAffine} from "../../🟦️.ts";

const validate=new Ajv({strict:false}).compile(schema);
const validateControls=new Ajv({strict:false}).compile(controlsSchema);
const affine=(value:number[])=>value as unknown as CompositeAffine;
const matrix=(v:CompositeAffine)=>new Matrix3().set(v[0],v[2],v[4],v[1],v[3],v[5],0,0,1);
function close(actual:readonly number[],expected:readonly number[]):void {
  expect(actual.length).toBe(expected.length);
  actual.forEach((value,index)=>expect(value).toBeCloseTo(expected[index]!,10));
}
for(const row of fixture.cases)test(`Frame change: ${row.name}`,()=>{
  expect(validate(row.input)).toBe(true);
  const {source,target,transform}=row.input,before=structuredClone(row.input);
  const result=reframe(affine(transform),affine(source),affine(target));
  close(result,row.expected);
  const oracle=matrix(affine(target)).invert().multiply(matrix(affine(source))).multiply(matrix(affine(transform))).elements;
  close(result,[oracle[0]!,oracle[1]!,oracle[3]!,oracle[4]!,oracle[6]!,oracle[7]!]);
  close(reframe(result,affine(target),affine(source)),transform);
  for(const point of fixture.points){
    const original=new Vector3(point[0],point[1],1).applyMatrix3(matrix(affine(source)).multiply(matrix(affine(transform))));
    const moved=new Vector3(point[0],point[1],1).applyMatrix3(matrix(multiply(affine(target),result)));
    close(moved.toArray(),original.toArray());
  }
  expect(row.input).toEqual(before);
});
test("Frame changes reject singular, nonfinite and overflowing placements",()=>{
  const identity:CompositeAffine=[1,0,0,1,0,0];
  for(const value of [...fixture.invalid,[Infinity,0,0,1,0,0],[1,0,0,1,NaN,0]]){
    expect(()=>reframe(affine(value),identity,identity)).toThrow();
    expect(()=>reframe(identity,affine(value),identity)).toThrow();
    expect(()=>reframe(identity,identity,affine(value))).toThrow();
  }
  expect(()=>reframe([1e154,0,0,1,0,0],[1e155,0,0,1,0,0],identity)).toThrow();
});
for(const row of controlsFixture.cases)test(`Affine controls: ${row.name}`,()=>{
  expect(validateControls(row.input)).toBe(true);
  const t=row.input,result=compose(t),r=t.rotation*Math.PI/180;
  close(result,row.expected);
  const oracle=new Matrix3().set(1,0,t.x,0,1,t.y,0,0,1)
    .multiply(new Matrix3().set(Math.cos(r),-Math.sin(r),0,Math.sin(r),Math.cos(r),0,0,0,1))
    .multiply(new Matrix3().set(1,t.shearX,0,0,1,0,0,0,1))
    .multiply(new Matrix3().set(t.scaleX,0,0,0,t.scaleY,0,0,0,1)).elements;
  close(result,[oracle[0]!,oracle[1]!,oracle[3]!,oracle[4]!,oracle[6]!,oracle[7]!]);
  const components=decompose(result);
  expect(validateControls(components)).toBe(true);expect(components.scaleX).toBeGreaterThan(0);
  close(compose(components),result);
});
test("Affine controls preserve all frame-change matrices",()=>{
  for(const row of fixture.cases)close(compose(decompose(affine(row.expected))),row.expected);
});
test("Affine controls reject incomplete and unsafe transforms",()=>{
  const good:AffineControls={x:0,y:0,scaleX:1,scaleY:1,rotation:0,shearX:0};
  for(const field of Object.keys(good) as (keyof AffineControls)[]){
    const missing:Partial<AffineControls>={...good};delete missing[field];
    expect(validateControls(missing)).toBe(false);expect(()=>compose(missing as AffineControls)).toThrow();
    expect(()=>compose({...good,[field]:NaN})).toThrow();
  }
  expect(()=>compose({...good,scaleX:0})).toThrow();
  expect(()=>compose({...good,scaleY:1e-13})).toThrow();
  for(const invalid of fixture.invalid)expect(()=>decompose(affine(invalid))).toThrow();
});
