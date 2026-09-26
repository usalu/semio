/** 🎛️ Independent Three.js bounds and Immer document oracle for selection fixtures. */
import { test, expect } from "bun:test";
import { Box2, Vector2, CubicBezierCurve } from "three";
import { produce } from "immer";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import conversions from "../../🧫️fixtures/🛤️to-path/🔣️.json";
import { shapePath } from "../../../../../🧬️schema/🧮️geometry/🔀️conversion/🟦️.ts";

test("shape conversion preserves primitive outlines and independent ellipse geometry", () => {
  const validate=new Ajv({strict:true}).compile(schema);
  expect(validate({operation:"toPath",ids:["shape"]})).toBe(true);
  for(const item of conversions) {
    const result=shapePath(item.kind,item.geometry),saved=structuredClone(item.geometry);
    expect(result).toEqual(item.segments);
    expect(item.geometry).toEqual(saved);
    let from=new Vector2();
    for(const segment of result) {
      if(segment.kind==="cubic") {
        const curve=new CubicBezierCurve(from,new Vector2(...segment.ctrl1),new Vector2(...segment.ctrl2),new Vector2(...segment.to));
        for(let step=0;step<=20;step++) {
          const point=curve.getPoint(step/20);
          expect(Math.abs(Math.hypot(point.x,point.y/(item.kind==="ellipse"?2:1))-1)).toBeLessThan(.0003);
        }
      }
      if(segment.kind!=="close") from=new Vector2(...segment.to);
    }
    const source={id:"shape",kind:"shape",name:"Original",transform:{x:5,y:6,scaleX:2,scaleY:3,rotation:.4},attributes:{opacity:.5},shapeKind:item.kind,[item.kind]:item.geometry};
    const oracle=produce(source,draft=>{ draft.kind="path"; delete (draft as Record<string,unknown>).shapeKind; delete (draft as Record<string,unknown>)[item.kind]; (draft as Record<string,unknown>).segments=result; });
    expect(oracle.id).toBe(source.id);
    expect(oracle.transform).toEqual(source.transform);
    expect(oracle.attributes).toEqual(source.attributes);
  }
});

test("selection fixture operations satisfy the command schema", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const item of fixture.cases) expect(validate({ operation: item.operation, ids: item.ids })).toBe(true);
  expect(validate({ operation: "unknown", ids: [] })).toBe(false);
});

test("selection fixture outcomes agree with Three.js and Immer", () => {
  for (const item of fixture.cases) {
    const selected = fixture.layers.filter(layer => item.ids.includes(layer.id));
    const boxes = selected.map(layer => new Box2(new Vector2(layer.x, layer.y), new Vector2(layer.x+layer.width, layer.y+layer.height)));
    const bounds = boxes.reduce((total, box) => total.union(box), new Box2());
    const output = produce(fixture.layers, draft => {
      if (item.operation === "delete") return draft.filter(layer => !item.ids.includes(layer.id));
      if (item.operation === "group") return [...draft.filter(layer => !item.ids.includes(layer.id)), { ...selected[0]!, id: "group" }];
      if (item.operation === "duplicate") return [...draft, ...selected.map(layer => ({ ...layer, id: `${layer.id}.copy` }))];
      if (item.operation === "bringToFront") return [...draft.filter(layer => !item.ids.includes(layer.id)), ...selected];
      if (item.operation === "sendToBack") return [...selected, ...draft.filter(layer => !item.ids.includes(layer.id))];
      const gap = (bounds.getSize(new Vector2()).x-selected.reduce((sum,layer) => sum+layer.width,0))/(selected.length-1);
      let cursor = bounds.min.x;
      for (const layer of draft) if (item.ids.includes(layer.id)) {
        if (item.operation === "alignLeft") layer.x = bounds.min.x;
        if (item.operation === "alignBottom") layer.y = bounds.max.y-layer.height;
        if (item.operation === "distributeHorizontal") { layer.x = cursor; cursor += layer.width+gap; }
      }
    });
    if ("bounds" in item) expect(output.map(layer => [layer.x,layer.y,layer.width,layer.height])).toEqual(item.bounds);
    if ("rootCount" in item) expect(output.length).toBe(item.rootCount);
    if ("order" in item) expect(output.map(layer => layer.id)).toEqual(item.order);
  }
});


test("repeated layer creation retains every requested instance with Immer", async () => {
  const fixture = await Bun.file(new URL("../../../➕️add-layer/🧫️fixtures/🔣️.json", import.meta.url)).json();
  let layers: { id: string; kind: string }[] = [];
  for (const kind of fixture.kinds) for (let ordinal = 0; ordinal < fixture.repetitions; ordinal++) layers = produce(layers, draft => { draft.push({ id: `${kind}:${ordinal}`, kind }); });
  expect(layers.length).toBe(30);
  expect(new Set(layers.map(layer => layer.id)).size).toBe(30);
});
