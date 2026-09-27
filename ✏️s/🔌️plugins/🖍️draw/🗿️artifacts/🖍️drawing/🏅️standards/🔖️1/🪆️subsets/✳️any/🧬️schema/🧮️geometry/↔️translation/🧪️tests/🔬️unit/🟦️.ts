/** ↔️ Parent-space translation agrees with independent Three.js inversion. */
import {expect,test} from "bun:test";
import {Matrix3,Vector3} from "three";
import {translate,selectionRoots} from "../../🟦️.ts";
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

import selectionCases from "../../🧫️fixtures/🗂️selection/🔣️.json";
import {produce} from "immer";
test("selection movement updates selected ancestors exactly once",()=>{
  for(const entry of selectionCases) {
    const roots=selectionRoots(entry.paths);
    expect(roots).toEqual(entry.roots);
    const leaf=entry.leaves.map(path=>({path,world:[5,7]}));
    const own=leaf.map(item=>{const count=roots.filter(root=>root.every((index,depth)=>item.path[depth]===index)).length;return {...item,world:[5+12*count,7-3*count]};});
    const expected=produce(leaf,draft=>{for(const item of draft) { if(entry.paths.some(parent=>parent.every((index,depth)=>item.path[depth]===index))) { item.world[0]!+=12; item.world[1]!-=3; } }});
    expect(own).toEqual(expected);
  }
});
