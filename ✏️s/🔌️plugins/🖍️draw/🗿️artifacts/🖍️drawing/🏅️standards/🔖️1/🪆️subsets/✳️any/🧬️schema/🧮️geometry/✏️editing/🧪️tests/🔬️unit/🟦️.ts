/** 🧪️ Shared node edits and independent curve evaluation. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { CubicBezierCurve, QuadraticBezierCurve, LineCurve, EllipseCurve, Vector2, Box2, Matrix3 } from "three";
import { produce } from "immer";
import { editPath, type PathEdit } from "../../🟦️.ts";
import { arcGeometry, arcPoint, segmentBounds, type Point, type Matrix } from "../../../🟦️.ts";
import boundsFixture from "../../../🧫️fixtures/🔄️arc-bounds/🔣️.json";
import type { PathSegment } from "../../../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
test("path node editing matches shared cases and independent geometry", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const item of fixture) {
    expect(validate(item.operation)).toBe(true);
    const before = item.before as PathSegment[];
    const operation = item.operation as PathEdit;
    if ("error" in item) { expect(() => editPath(before, operation)).toThrow(); continue; }
    const result = editPath(before, operation);
    const rounded = (value: unknown) => JSON.parse(JSON.stringify(value, (_, field) => typeof field === "number" ? Number(field.toFixed(10)) : field));
    expect(rounded(result)).toEqual(rounded(item.after));
    expect(editPath(editPath(result, { kind: "reverse" }), { kind: "reverse" })).toEqual(result);
    if (item.name === "edit-control") {
      const oracle = produce(before, draft => { if (draft[1]?.kind === "cubic") draft[1].ctrl1[1] = 18; });
      expect(result).toEqual(oracle);
    }
    if (item.name === "split-cubic") {
      const curve = new CubicBezierCurve(new Vector2(0,0), new Vector2(0,12), new Vector2(12,12), new Vector2(12,0));
      for (let index = 0; index <= 20; index++) {
        const t = index / 20;
        const segment = result[t <= .5 ? 1 : 2];
        if (segment?.kind !== "cubic") throw new Error("Missing cubic");
        const start = t <= .5 ? [0,0] : [6,9];
        const half = new CubicBezierCurve(new Vector2(...start), new Vector2(...segment.ctrl1), new Vector2(...segment.ctrl2), new Vector2(...segment.to));
        expect(half.getPoint(t <= .5 ? t * 2 : (t-.5)*2).distanceTo(curve.getPoint(t))).toBeLessThan(1e-10);
      }
    }
    if (item.name === "split-arc") {
      const oracle = new EllipseCurve(5,0,5,5,Math.PI,2*Math.PI,false,0);
      for (let index = 0; index <= 20; index++) {
        const t = index / 20, halfIndex = t <= .5 ? 1 : 2, segment = result[halfIndex]!, previous = result[halfIndex-1]!;
        if (segment.kind !== "arc" || previous.kind === "close") throw new Error("Missing arc");
        const arc = arcGeometry(previous.to,[segment.rx,segment.ry],segment.rotation,segment.largeArc,segment.sweep,segment.to);
        if (!arc) throw new Error("Invalid arc");
        expect(new Vector2(...arcPoint(arc,t<=.5?t*2:(t-.5)*2)).distanceTo(oracle.getPoint(t))).toBeLessThan(1e-10);
      }
    }
  }
});

test("ellipse centers preserve rotation, radius correction and sweep direction", () => {
  for (const clockwise of [false,true]) {
    const oracle = new EllipseCurve(10,20,8,4,0,Math.PI*1.5,clockwise,Math.PI/2);
    const from = oracle.getPoint(0), to = oracle.getPoint(1);
    const arc = arcGeometry([from.x,from.y],[8,4],90,!clockwise,!clockwise,[to.x,to.y]);
    expect(arc).not.toBeNull();
    for (let index = 0; index <= 20; index++) expect(new Vector2(...arcPoint(arc!, index/20)).distanceTo(oracle.getPoint(index/20))).toBeLessThan(1e-10);
  }
  const corrected = arcGeometry([0,0],[1,1],0,false,true,[10,0]);
  expect(corrected?.radii).toEqual([5,5]);
});

test("segment conversions preserve endpoints and independently evaluated curve geometry", () => {
  for (const item of fixture.filter(item => item.name.startsWith("convert-"))) {
    const before = item.before as PathSegment[], saved = structuredClone(before);
    const result = editPath(before, item.operation as PathEdit);
    expect(before).toEqual(saved);
    const start = before[0]!, source = before[1]!, segment = result[1]!;
    if (start.kind !== "move" || segment.kind !== "cubic" || source.kind === "close") throw new Error("Missing converted curve");
    expect(segment.to).toEqual(source.to);
    const curve = new CubicBezierCurve(new Vector2(...start.to), new Vector2(...segment.ctrl1), new Vector2(...segment.ctrl2), new Vector2(...segment.to));
    const original = source.kind === "quad" ? new QuadraticBezierCurve(new Vector2(...start.to),new Vector2(...source.ctrl),new Vector2(...source.to)) : new LineCurve(new Vector2(...start.to),new Vector2(...source.to));
    for (let step = 0; step <= 40; step++) {
      const t = step / 40, point = curve.getPoint(t);
      if (source.kind === "arc") expect(Math.abs(point.length()-10)).toBeLessThan(.003);
      else expect(point.distanceTo(original.getPoint(t))).toBeLessThan(1e-10);
    }
    const line = editPath(result, {kind:"convert",index:1,target:"line"});
    const oracle = produce(result,draft=>{ draft[1]={kind:"line",to:[...segment.to]}; });
    expect(line).toEqual(oracle);
  }
});

test("converted multi-quadrant arcs retain rotated ellipse geometry and sweep", () => {
  for (const clockwise of [false,true]) for (const rotation of [0,Math.PI/4]) {
    const ellipse = new EllipseCurve(10,20,8,4,0,Math.PI*1.5,clockwise,rotation);
    const start=ellipse.getPoint(0),end=ellipse.getPoint(1);
    const result=editPath([{kind:"move",to:[start.x,start.y]},{kind:"arc",rx:8,ry:4,rotation:rotation*180/Math.PI,largeArc:!clockwise,sweep:!clockwise,to:[end.x,end.y]}],{kind:"convert",index:1,target:"cubic"});
    expect(result.length).toBe(clockwise?2:4);
    let from=start;
    for(let index=1;index<result.length;index++) {
      const segment=result[index]!;
      if(segment.kind!=="cubic") throw new Error("Expected cubic pieces");
      const curve=new CubicBezierCurve(from,new Vector2(...segment.ctrl1),new Vector2(...segment.ctrl2),new Vector2(...segment.to));
      for(let step=0;step<=20;step++) {
        const point=curve.getPoint(step/20).sub(new Vector2(10,20)).rotateAround(new Vector2(),-rotation);
        expect(Math.abs(Math.hypot(point.x/8,point.y/4)-1)).toBeLessThan(.0003);
      }
      expect(curve.getTangent(0).dot(ellipse.getTangent((index-1)/(result.length-1)))).toBeGreaterThan(.999999);
      from=new Vector2(...segment.to);
    }
    expect(from.equals(end)).toBe(true);
  }
});

test("joining preserves oriented curves and matches independent contour splices", () => {
  const separated=fixture.find(item=>item.name==="join-separated-contours")!;
  const before=separated.before as PathSegment[],saved=structuredClone(before);
  const result=editPath(before,separated.operation as PathEdit);
  const oracle=produce(before,draft=>{ draft[2]={kind:"line",to:[10,0]}; });
  expect(result).toEqual(oracle);
  expect(before).toEqual(saved);
  const coincident=fixture.find(item=>item.name==="join-coincident-endpoints")!;
  expect(editPath(coincident.before as PathSegment[],coincident.operation as PathEdit)).toEqual(produce(coincident.before,draft=>{draft.splice(2,1);}));
  const reversed=fixture.find(item=>item.name==="join-reversed-contours")!;
  const joined=editPath(reversed.before as PathSegment[],reversed.operation as PathEdit),segment=joined[1]!;
  if(segment.kind!=="cubic") throw new Error("Missing joined curve");
  const original=new CubicBezierCurve(new Vector2(0,0),new Vector2(1,2),new Vector2(4,2),new Vector2(5,0));
  const curve=new CubicBezierCurve(new Vector2(5,0),new Vector2(...segment.ctrl1),new Vector2(...segment.ctrl2),new Vector2(...segment.to));
  for(let index=0;index<=20;index++) expect(curve.getPoint(index/20).distanceTo(original.getPoint(1-index/20))).toBeLessThan(1e-10);
});

test("arc extrema agree with shared fixtures and an independent ellipse", () => {
  for (const item of boundsFixture) {
    const bounds=segmentBounds(item.segment as PathSegment,item.from as Point,item.from as Point,item.matrix as Matrix);
    for (let axis=0;axis<4;axis++) expect(bounds[axis]).toBeCloseTo(item.bounds[axis]!,10);
    const oracle=new EllipseCurve(5,0,5,5,Math.PI,Math.PI*2,false,0);
    const [a,b,c,d,e,f]=item.matrix as Matrix,matrix=new Matrix3().set(a,c,e,b,d,f,0,0,1);
    const box=new Box2().setFromPoints(oracle.getPoints(1000).map(point=>point.applyMatrix3(matrix)));
    const expected=[box.min.x,box.min.y,box.max.x-box.min.x,box.max.y-box.min.y];
    for(let axis=0;axis<4;axis++) expect(bounds[axis]).toBeCloseTo(expected[axis]!,10);
  }
});
