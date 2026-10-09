import {test,expect} from "bun:test";
import Ajv from "ajv";
import {produce} from "immer";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🔣️.json";
import {applyShapeCoordinate} from "../../🦠️mutation/🟦️.ts";
import {shapeCoordinate,type ShapeCoordinateField} from "../../../../../../✳️any/🧬️schema/🔷️shape/✏️coordinates/🟦️.ts";
import type {DrawingArtifact,DrawingLayerNode} from "../../../../../../✳️any/🧬️schema/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
test("sparse coordinate mutation matches shared fixture and independent Immer output",()=>{
 const validate=new Ajv({strict:false}).compile(schema);
 const shape={id:"shape",kind:"shape",shapeKind:"rect",rect:{x:binary64(0),y:binary64(0),width:binary64(128),height:binary64(96)}} as Extract<DrawingLayerNode,{kind:"shape"}>;
 const before={schema:"drawing.document",id:"drawing",layers:[shape],assets:{}} as DrawingArtifact;
 for(const edit of fixture.edits){
  const field=edit.field as ShapeCoordinateField,mutation={layerId:"shape",field,value:edit.value};expect(validate({mutation:"setShapeCoordinate",...mutation})).toBe(true);
  const after=applyShapeCoordinate(before,mutation);const suffix=field.slice(4),key=suffix[0]!.toLowerCase()+suffix.slice(1);
  expect(after).toEqual(produce(before,draft=>{(draft.layers[0] as typeof shape).rect![key as "x"]=binary64(edit.value);}));
  expect(applyShapeCoordinate(after,{...mutation,value:shapeCoordinate(shape,field)})).toEqual(before);
 }
 for(const edit of fixture.invalid){expect(()=>applyShapeCoordinate(before,{layerId:"shape",field:edit.field as ShapeCoordinateField,value:edit.value})).toThrow();}
});
