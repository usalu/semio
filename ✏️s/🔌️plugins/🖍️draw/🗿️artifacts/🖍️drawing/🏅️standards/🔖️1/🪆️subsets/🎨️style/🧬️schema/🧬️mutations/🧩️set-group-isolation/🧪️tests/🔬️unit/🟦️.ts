/** 🧩️ Shared isolation mutation contract checked against Immer and Ajv. */
import {expect,test} from "bun:test";
import {produce} from "immer";
import Ajv from "ajv";
import {applyGroupIsolationEdit} from "../../🦠️mutation/🟦️.ts";
import {inverse} from "../../↩️inverse/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import before from "../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass-through-to-isolated/📸️snapshot/⬅️before/🔣️.json";
import after from "../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass-through-to-isolated/📸️snapshot/➡️after/🔣️.json";
test("isolation is an atomic group-only edit with a reversible authored default",()=>{
  const edit={layerId:"group-a",isolation:true};
  expect(new Ajv({strict:true}).compile(schema)(edit)).toBe(true);
  const result=applyGroupIsolationEdit(before,edit);
  expect(result).toEqual(after);
  expect(result).toEqual(produce(before,draft=>{Object.assign(draft.layers[0]!,{isolation:true});}));
  expect(applyGroupIsolationEdit(result,inverse(edit,false)[0]!)).toEqual(before);
  expect(before.layers[0]).not.toHaveProperty("isolation");
  for(const layerId of ["shape-a","missing"])expect(()=>applyGroupIsolationEdit(before,{...edit,layerId})).toThrow();
  for(const isolation of ["true",1,null]){expect(new Ajv({strict:true}).compile(schema)({...edit,isolation})).toBe(false);expect(()=>applyGroupIsolationEdit(before,{...edit,isolation} as never)).toThrow();}
});

import {parseDrawingLayerNode} from "../../../../../../✳️any/🧬️schema/🟦️.ts";
import {parseDrawingLayerPatch} from "../../../../../../✳️any/🧬️schema/🔺️diff/🟦️.ts";
import documentSchema from "../../../../../../✳️any/🧬️schema/🔣️.json";
test("isolation snapshot and diff guards agree with the shared schema",()=>{
  const ajv=new Ajv({strict:false,validateFormats:false}).addSchema(documentSchema);
  const validate=ajv.compile({$ref:`${documentSchema.$id}#/$defs/DrawingLayerNode`});
  for(const isolation of [false,true]){
    const layer={kind:"group",isolation,children:[]};
    expect(validate(layer)).toBe(true);expect(parseDrawingLayerNode(layer).isolation).toBe(isolation);
    expect(parseDrawingLayerPatch({isolation}).isolation).toBe(isolation);
  }
  for(const isolation of ["true",1,null]){
    const layer={kind:"group",isolation,children:[]};
    expect(validate(layer)).toBe(false);expect(()=>parseDrawingLayerNode(layer)).toThrow();
  }
  expect(()=>parseDrawingLayerPatch({isolation:"true"})).toThrow();
  console.error("[DEBUG] group isolation schema, snapshot/diff guards, and immutable mutation agree");
});
