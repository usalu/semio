/** 🧪️ Authored winding agrees with Three.js SVG triangulation. */
import {test,expect} from "bun:test";
import {DOMParser} from "@xmldom/xmldom";
import {SVGLoader} from "three/addons/loaders/SVGLoader.js";
import {ShapeUtils,Vector2} from "three";
import Ajv from "ajv";
import {parseFillRule} from "../../🟦️.ts";
import {PathHitCursor} from "../../../../🧮️geometry/🎯️picking/🟦️.ts";
import {parseEditableSvgPath} from "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🛤️path/🟦️.ts";
import cases from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
const validate=new Ajv({strict:true}).compile(schema);
for(const row of cases)test(`authored fill rule: ${row.name}`,()=>{
  expect(validate(row.rule)).toBe(true);
  const rule=parseFillRule(row.rule),segments=parseEditableSvgPath(row.path);
  const hit=new PathHitCursor([row.point[0]!,row.point[1]!],[1,0,0,1,0,0],0,.001);
  while(!hit.step(segments)){}
  expect(hit.contains(true,false,rule==="evenodd")).toBe(row.hit);
  const previous=Reflect.get(globalThis,"DOMParser");Reflect.set(globalThis,"DOMParser",DOMParser);
  try {
    const paths=new SVGLoader().parse(`<svg xmlns="http://www.w3.org/2000/svg"><path fill-rule="${rule}" d="${row.path}"/></svg>`).paths;
    const p=new Vector2(...row.point),cross=(a:Vector2,b:Vector2)=>(b.x-a.x)*(p.y-a.y)-(b.y-a.y)*(p.x-a.x);
    const oracle=paths.flatMap(path=>SVGLoader.createShapes(path)).some(shape=>{
      const {shape:contour,holes}=shape.extractPoints(16),faces=ShapeUtils.triangulateShape(contour,holes),points=[...contour,...holes.flat()];
      return faces.some(face=>{const [a,b,c]=face.map(index=>points[index]!);const signs=[cross(a!,b!),cross(b!,c!),cross(c!,a!)];return signs.every(value=>value>=0)||signs.every(value=>value<=0);});
    });
    expect(oracle).toBe(row.hit);
  } finally {if(previous===undefined)Reflect.deleteProperty(globalThis,"DOMParser");else Reflect.set(globalThis,"DOMParser",previous);}
});
test("fill rule refuses unknown values",()=>{for(const value of ["evenOdd","NONZERO","",null,0]){expect(validate(value)).toBe(false);expect(()=>parseFillRule(value)).toThrow();}});

import {applyFillRuleEdit} from "../../../../../../🎨️style/🧬️schema/🧬️mutations/🌀️set-layer-fill-rule/🦠️mutation/🟦️.ts";
import {applyPatch} from "fast-json-patch";
import before from "../../../../../../🎨️style/🧫️fixtures/🧬️mutations/🌀️set-layer-fill-rule/🌀️evenodd/📸️snapshot/⬅️before/🔣️.json";
import after from "../../../../../../🎨️style/🧫️fixtures/🧬️mutations/🌀️set-layer-fill-rule/🌀️evenodd/📸️snapshot/➡️after/🔣️.json";
test("fill-rule mutation preserves unrelated fields and has an exact inverse",()=>{
  const changed=applyFillRuleEdit(before,{layerId:"shape-a",fillRule:"nonzero"});
  expect(changed).toEqual(after);
  expect(applyPatch(structuredClone(before),[{op:"replace",path:"/layers/0/attributes/fillRule",value:"nonzero"}]).newDocument).toEqual(changed);
  expect(applyFillRuleEdit(changed,{layerId:"shape-a",fillRule:"evenodd"})).toEqual(before);
  expect(()=>applyFillRuleEdit(before,{layerId:"missing",fillRule:"nonzero"})).toThrow();
  expect(()=>applyFillRuleEdit(before,{layerId:"shape-a",fillRule:"invalid" as never})).toThrow();
});
