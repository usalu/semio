/** 🧪️ Shape-coordinate twins match language-neutral fixtures and independent Immer edits. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {produce} from "immer";
import {binary64,binary64Value,type Binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type {DrawingLayerNode} from "../../../../🟦️.ts";
import {SHAPE_COORDINATE_FIELDS,shapeCoordinate,setShapeCoordinate,type ShapeCoordinateField} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import rows from "../../🧫️fixtures/🔣️.json";
const capture=(value:unknown):unknown=>typeof value==="number"?binary64(value):Array.isArray(value)?value.map(capture):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,value])=>[key,capture(value)])):value;
const query=(value:unknown):unknown=>value&&typeof value==="object"&&"bits" in value?binary64Value(value as Binary64):Array.isArray(value)?value.map(query):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,value])=>[key,query(value)])):value;
test("all shape coordinate fixture outputs match independent immutable scalar patches",()=>{
  const validate=new Ajv({strict:true}).compile(schema);
  const seen=new Set<ShapeCoordinateField>();
  for(const row of rows) {
    const field=row.field as ShapeCoordinateField,index="index" in row?row.index:undefined,shape={kind:"shape",shapeKind:row.kind,[row.kind]:capture(row.before)} as Extract<DrawingLayerNode,{kind:"shape"}>;
    const before=structuredClone(shape);
    expect(validate({field,value:row.value,...(index===undefined?{}:{index})})).toBe(true);
    if("error" in row){expect(()=>setShapeCoordinate(shape,field,index,row.value)).toThrow();expect(shape).toEqual(before);continue;}
    seen.add(field);
    const previous=shapeCoordinate(shape,field,index),after=setShapeCoordinate(shape,field,index,row.value);
    expect(query(after[row.kind as "rect"])).toEqual(row.after);expect(shape).toEqual(before);expect(shapeCoordinate(after,field,index)).toBe(row.value);
    const oracle=produce(row.before as Record<string,unknown>,draft=>{if(row.kind==="polygon"){(draft.points as number[][])[index!]![field==="polygonX"?0:1]=row.value;}else {const suffix=field.slice(row.kind.length);draft[suffix[0]!.toLowerCase()+suffix.slice(1)]=row.value;}});
    expect(query(after[row.kind as "rect"])).toEqual(oracle);
    expect(setShapeCoordinate(after,field,index,previous)).toEqual(shape);
    expect(()=>setShapeCoordinate(shape,field,index,NaN)).toThrow();
  }
  expect([...seen].sort()).toEqual([...SHAPE_COORDINATE_FIELDS].sort());
});
