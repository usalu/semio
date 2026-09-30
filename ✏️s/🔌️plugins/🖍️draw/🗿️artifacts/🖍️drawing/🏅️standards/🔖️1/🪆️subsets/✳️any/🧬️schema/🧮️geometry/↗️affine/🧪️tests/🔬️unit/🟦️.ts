/** ↗️ Exact affine transforms retain shear, reflection and collapsed axes. */
import {expect,test} from "bun:test";
import {Matrix3,Vector2} from "three";
import {drawingTransformToMatrix,drawingMatrixToTransform} from "../../🟦️.ts";
import cases from "../../🧫️fixtures/🔣️.json";

for (const sample of cases) test(`affine roundtrip: ${sample.name}`, () => {
  const matrix = sample.matrix as [number,number,number,number,number,number];
  const transform = drawingMatrixToTransform(matrix);
  const actual = drawingTransformToMatrix(transform);
  const magnitude = Math.max(...matrix.slice(0,4).map(Math.abs),Number.MIN_VALUE);
  for (let index=0;index<6;index++) expect(Math.abs(actual[index]!-matrix[index]!)/Math.max(magnitude,Math.abs(matrix[index]!))).toBeLessThan(1e-12);
  const oracle = new Matrix3().set(1,0,transform.x,0,1,transform.y,0,0,1)
    .multiply(new Matrix3().makeRotation(transform.rotation))
    .multiply(new Matrix3().set(transform.scaleX,transform.shear,0,0,transform.scaleY,0,0,0,1));
  const point = new Vector2(2,-3).applyMatrix3(oracle);
  const expected = new Vector2(2,-3).applyMatrix3(new Matrix3().set(matrix[0],matrix[2],matrix[4],matrix[1],matrix[3],matrix[5],0,0,1));
  for (const axis of ["x","y"] as const) expect(Math.abs(point[axis]-expected[axis])/Math.max(magnitude,Math.abs(expected[axis]))).toBeLessThan(1e-12);
});


test("world rotation remains exact inside a nonuniformly scaled parent", () => {
  const parent=new Matrix3().set(2,1,10,0,3,20,0,0,1), original=new Matrix3().set(1,0,5,0,1,7,0,0,1);
  const worldTurn=new Matrix3().makeRotation(Math.PI/3);
  const local=parent.clone().invert().multiply(worldTurn).multiply(parent).multiply(original);
  const m=local.elements;
  const retained=drawingMatrixToTransform([m[0]!,m[1]!,m[3]!,m[4]!,m[6]!,m[7]!]);
  expect(Math.abs(retained.shear)).toBeGreaterThan(0.1);
  const output=drawingTransformToMatrix(retained);
  const actual=parent.clone().multiply(new Matrix3().set(output[0],output[2],output[4],output[1],output[3],output[5],0,0,1));
  const wanted=worldTurn.clone().multiply(parent).multiply(original);
  actual.elements.forEach((value,index)=>expect(value).toBeCloseTo(wanted.elements[index]!,12));
});


import {semioSchemaAjvV1} from "../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import mutationSchema from "../../../../../../🔀️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🧬️schema/🔣️.json";

test("transform mutation schema requires shear and accepts exact affine components", () => {
  const validate=semioSchemaAjvV1({allErrors:true}).compile(mutationSchema);
  for (const sample of cases) {
    const transform=drawingMatrixToTransform(sample.matrix as [number,number,number,number,number,number]);
    expect(validate({mutation:"updateLayerTransform",layerId:"shape",transform})).toBe(true);
    expect(validate({layerId:"shape",transform})).toBe(false);
    const {shear,...incomplete}=transform;
    expect(validate({mutation:"updateLayerTransform",layerId:"shape",transform:incomplete})).toBe(false);
  }
});
