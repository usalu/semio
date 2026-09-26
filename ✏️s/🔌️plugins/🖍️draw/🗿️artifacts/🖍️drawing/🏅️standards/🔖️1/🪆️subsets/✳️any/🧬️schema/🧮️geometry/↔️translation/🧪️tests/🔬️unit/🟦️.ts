/** ↔️ Parent-space translation agrees with independent Three.js inversion. */
import {expect,test} from "bun:test";
import {Matrix3,Vector3} from "three";
import {translate} from "../../🟦️.ts";
import cases from "../../🧫️fixtures/🔣️.json";
test("world dragging preserves local scale and rotation under nested transforms",()=>{
  for(const entry of cases){
    const result=translate(entry.transform,entry.parent,entry.delta);
    expect(result).toEqual(entry.after);
    if(!result) continue;
    const [a,b,c,d,e,f]=entry.parent;
    const matrix=new Matrix3().set(a!,c!,e!,b!,d!,f!,0,0,1).invert();
    const delta=new Vector3(entry.delta[0],entry.delta[1],0).applyMatrix3(matrix);
    expect(result[0]).toBeCloseTo(entry.transform[0]!+delta.x,10);
    expect(result[1]).toBeCloseTo(entry.transform[1]!+delta.y,10);
  }
});
