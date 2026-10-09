import {expect,test} from "bun:test";
import {Matrix3,Vector2} from "three";
import fixture from "../../🧫️fixtures/🔣️.json";
import {affineQuad,affineQuadPoint} from "../../🟦️.ts";

test("packed glyph and image corners retain authored affine transforms",()=>{
  const corners=[[0,0],[1,0],[1,1],[0,1]] as const;
  for(const sample of fixture.cases){
    const rect=sample.rect as [number,number,number,number],m=sample.matrix as [number,number,number,number,number,number];
    const quad=affineQuad(rect,m),oracle=new Matrix3().set(m[0],m[2],m[4],m[1],m[3],m[5],0,0,1);
    corners.forEach((corner,index)=>{
      const expected=new Vector2(rect[0]+corner[0]*rect[2],rect[1]+corner[1]*rect[3]).applyMatrix3(oracle),actual=affineQuadPoint(quad,corner);
      expect(actual).toEqual(sample.corners[index]!);
      expect(actual[0]).toBeCloseTo(expected.x,8);
      expect(actual[1]).toBeCloseTo(expected.y,8);
    });
  }
  console.error(`[DEBUG] Affine quad Three.js oracle: ${fixture.cases.length} transformations; 20 mapped corners`);
});
