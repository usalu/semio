/** 📷️ Shared artwork framing traces checked against Three.js transformed points. */
import { expect,test } from "bun:test";
import Ajv from "ajv";
import { Box2,CubicBezierCurve,Matrix3,QuadraticBezierCurve,Vector2 } from "three";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { drawingSceneBounds,type FramingNode } from "../../🟦️.ts";
test("framing fixtures conform to their neutral schema",() => expect(new Ajv().compile(schema)(fixture)).toBe(true));
for (const item of fixture.cases) test(item.name,() => {
  const nodes = item.nodes as FramingNode[];
  const artboard = "artboard" in item ? item.artboard : undefined;
  const actual = drawingSceneBounds(artboard,nodes);
  const box = new Box2();
  if (artboard) box.set(new Vector2(0,0),new Vector2(artboard.width,artboard.height));
  for (const node of nodes.filter(node => node.visible && node.opacity > 0)) {
    const lines = node.text?.content.split(/\r\n|[\r\n]/);
    const rectangle = node.image ? [0,0,node.image.width,node.image.height] : node.text ? [0,0,Math.max(...lines!.map(line => Array.from(line).length))*node.text.size*0.6,lines!.length*node.text.size*1.2] : null;
    const points: number[][] = [];
    if (rectangle) {
      const [x,y,w,h] = rectangle as [number,number,number,number];
      points.push([x,y],[x+w,y],[x+w,y+h],[x,y+h]);
    } else {
      let current = new Vector2();
      let start = current.clone();
      for (const segment of node.segments) {
        if (segment.kind === "cubic") points.push(...new CubicBezierCurve(current,new Vector2(...segment.ctrl1),new Vector2(...segment.ctrl2),new Vector2(...segment.to)).getPoints(10000).map(point => point.toArray()));
        else if (segment.kind === "quad") points.push(...new QuadraticBezierCurve(current,new Vector2(...segment.ctrl),new Vector2(...segment.to)).getPoints(10000).map(point => point.toArray()));
        else if ("to" in segment) points.push(segment.to);
        if (segment.kind === "move") start = new Vector2(...segment.to);
        current = "to" in segment ? new Vector2(...segment.to) : start;
      }
    }
    const [a,b,c,d,e,f] = node.transform;
    const matrix = new Matrix3().set(a,c,e,b,d,f,0,0,1);
    for (const point of points) {
      const radius = (node.stroke?.width ?? 0)/2;
      for (let sample=0;sample<(radius ? 10000 : 1);sample++) {
        const angle = sample*Math.PI*2/10000;
        box.expandByPoint(new Vector2(point[0]!+radius*Math.cos(angle),point[1]!+radius*Math.sin(angle)).applyMatrix3(matrix));
      }
    }
  }
  const oracle = box.isEmpty() ? [0,0,1024,1024] : [box.min.x,box.min.y,box.max.x,box.max.y];
  actual.forEach((value,index) => { expect(value).toBeCloseTo(item.expected[index]!,8); expect(value).toBeCloseTo(oracle[index]!,5); });
});
