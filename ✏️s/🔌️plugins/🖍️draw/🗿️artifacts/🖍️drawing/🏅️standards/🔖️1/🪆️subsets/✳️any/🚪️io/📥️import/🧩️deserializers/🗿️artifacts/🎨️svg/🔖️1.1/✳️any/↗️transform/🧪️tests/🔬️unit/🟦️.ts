/** 🧪️ SVG transforms preserve authored geometry through owned decomposition and SVGLoader. */
import {expect,test} from "bun:test";
import {DOMParser} from "@xmldom/xmldom";
import {SVGLoader} from "three/addons/loaders/SVGLoader.js";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {parseEditableSvgTransform} from "../../🟦️.ts";
import {readFileSync} from "node:fs";
import {drawingTransformToMatrix} from "../../../../../../../../../../🧬️schema/🧮️geometry/↗️affine/🟦️.ts";
test("decoded native SVG transforms use the same semantic operation authority",()=>{
  for(const row of fixture.filter(row=>row.matrix!==null)){
    const actual=drawingTransformToMatrix(parseEditableSvgTransform(row.source));
    for(let i=0;i<6;i++)expect(actual[i]!).toBeCloseTo(row.matrix![i]!,10);
  }
  const native=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
  expect(native.includes("pub fn editable_svg_transform_operations")).toBe(true);
});
for(const row of fixture)test(`editable SVG transform: ${row.name}`,()=>{
  expect(new Ajv({strict:true}).compile(schema)(row.source)).toBe(true);
  if(row.matrix===null){expect(()=>parseEditableSvgTransform(row.source)).toThrow();return;}
  const actual=drawingTransformToMatrix(parseEditableSvgTransform(row.source));
  for(let i=0;i<6;i++)expect(actual[i]!).toBeCloseTo(row.matrix[i]!,10);
  const previous=Reflect.get(globalThis,"DOMParser");Reflect.set(globalThis,"DOMParser",DOMParser);
  try {
    const points=new SVGLoader().parse(`<svg xmlns="http://www.w3.org/2000/svg"><path transform="${row.source}" d="M0 0 L2 3 L7 -4"/></svg>`).paths[0]!.subPaths[0]!.getPoints();
    expect(points.length).toBe(3);
    for(const [i,[x,y]] of [[0,0],[2,3],[7,-4]].entries()) {
      expect(points[i]!.x).toBeCloseTo(actual[0]*x!+actual[2]*y!+actual[4],9);
      expect(points[i]!.y).toBeCloseTo(actual[1]*x!+actual[3]*y!+actual[5],9);
    }
  } finally {if(previous===undefined)Reflect.deleteProperty(globalThis,"DOMParser");else Reflect.set(globalThis,"DOMParser",previous);}
});
