/** 🎛️ Handle gestures are absolute world transforms with stable anchors. */
import {expect,test} from "bun:test";
import {Matrix3,Vector2} from "three";
import {handleMatrix,handlePoints,hitHandle} from "../../🟦️.ts";
import cases from "../../🧫️fixtures/🔣️.json";
for (const sample of cases) test(`transform handle: ${sample.name}`,()=>{
  const actual=handleMatrix(sample.handle,sample.bounds as [number,number,number,number],sample.start as [number,number],sample.end as [number,number],sample.constrained,sample.centered)!;
  actual.forEach((value,index)=>expect(value).toBeCloseTo(sample.matrix[index]!,12));
  const [a,b,c,d,e,f]=actual,expected=sample.matrix;
  const output=new Vector2(7,13).applyMatrix3(new Matrix3().set(a,c,e,b,d,f,0,0,1));
  const oracle=new Vector2(7,13).applyMatrix3(new Matrix3().set(expected[0]!,expected[2]!,expected[4]!,expected[1]!,expected[3]!,expected[5]!,0,0,1));
  expect(output.x).toBeCloseTo(oracle.x,12);expect(output.y).toBeCloseTo(oracle.y,12);
});
test("handle hit radius stays constant across camera zoom",()=>{
  for (const zoom of [.25,1,4]) {
    const points=handlePoints([10,20,100,80],zoom);
    expect(hitHandle([10,20,100,80],[points[8]![0]+5/zoom,points[8]![1]],zoom)).toBe(8);
    expect(hitHandle([10,20,100,80],[points[8]![0]+20/zoom,points[8]![1]],zoom)).toBeNull();
  }
});


test("invalid handle input never publishes a matrix",()=>{
  for(const handle of [-1,1.5,9])expect(handleMatrix(handle,[0,0,10,10],[10,10],[20,20],false,false)).toBeNull();
  expect(handleMatrix(4,[0,0,10,10],[10,10],[Infinity,20],false,false)).toBeNull();
});
