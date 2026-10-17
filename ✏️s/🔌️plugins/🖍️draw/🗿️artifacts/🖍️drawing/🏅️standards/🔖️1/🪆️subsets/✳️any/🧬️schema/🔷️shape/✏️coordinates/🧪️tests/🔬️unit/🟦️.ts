/** 🧪️ Shape-coordinate twins match language-neutral fixtures and independent Immer edits. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {produce} from "immer";
import {binary64Value,type Binary64} from "@semio-tech/framework/value/ieee754";
import {parseDrawingLayerNode,type DrawingLayerNode} from "../../../../🟦️.ts";
import {drawingWords} from "../../../../../🧪️tests/🧫️fixtures/🟦️.ts";
import {SHAPE_COORDINATE_FIELDS,shapeCoordinate,setShapeCoordinate,type ShapeCoordinateField} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import rows from "../../../../../🧪️tests/🧫️fixtures/🔷️shape/✏️coordinates/🔣️.json";
const base={id:"shape",name:"Shape",visible:true,locked:false,opacity:1,blendMode:"normal",transform:{x:0,y:0,scaleX:1,scaleY:1,shear:0,rotation:0},attributes:{fillRule:"evenodd"}};
const shapeNode=(kind:string,geometry:unknown):Extract<DrawingLayerNode,{kind:"shape"}>=>{const node=parseDrawingLayerNode(drawingWords({...base,kind:"shape",shapeKind:kind,[kind]:geometry}));if(node.kind!=="shape")throw new Error("fixture is not a shape");return node;};
const query=(value:unknown):unknown=>value&&typeof value==="object"&&"bits" in value?binary64Value(value as Binary64):Array.isArray(value)?value.map(query):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,value])=>[key,query(value)])):value;
test("all shape coordinate fixture outputs match independent immutable scalar patches",()=>{
  const validate=new Ajv({strict:true}).compile(schema);
  const seen=new Set<ShapeCoordinateField>();
  for(const row of rows) {
    const field=row.field as ShapeCoordinateField,index="index" in row?row.index:undefined,shape=shapeNode(row.kind,row.before);
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
