/** 🧩️ Shared isolation mutation contract checked against Immer and Ajv. */
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {expect,test} from "bun:test";
import {produce} from "immer";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import {applyGroupIsolationEdit} from "../../🦠️mutation/🟦️.ts";
import {inverse} from "../../↩️inverse/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import before from "../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/⬅️before/🔣️.json";
import after from "../../../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/➡️after/🔣️.json";
test("isolation is an atomic group-only edit with a reversible authored default",()=>{
  const edit={layerId:"group-a",isolation:true};
  const validate=semioSchemaAjvV1({allErrors:true}).compile(schema);
  expect(validate({mutation:"setGroupIsolation",...edit})).toBe(true);
  expect(validate(edit)).toBe(false);
  const result=applyGroupIsolationEdit(before,edit);
  expect(result).toEqual(after);
  expect(result).toEqual(produce(before,draft=>{Object.assign(draft.layers[0]!,{isolation:true});}));
  expect(applyGroupIsolationEdit(result,inverse(edit,false)[0]!)).toEqual(before);
  expect(before.layers[0]).not.toHaveProperty("isolation");
  for(const layerId of ["shape-a","missing"])expect(()=>applyGroupIsolationEdit(before,{...edit,layerId})).toThrow();
  for(const isolation of ["true",1,null]){expect(validate({mutation:"setGroupIsolation",...edit,isolation})).toBe(false);expect(()=>applyGroupIsolationEdit(before,{...edit,isolation} as never)).toThrow();}
});

import {parseDrawingLayerNode} from "../../../../../../✳️any/🧬️schema/🟦️.ts";
import {parseDrawingLayerPatch} from "../../../../../../✳️any/🧬️schema/🔺️diff/🟦️.ts";
import documentSchema from "../../../../../../✳️any/🧬️schema/🔣️.json";
test("isolation snapshot and diff guards agree with the shared schema",()=>{
  const ajv=semioSchemaAjvV1({allErrors:true}).addSchema(documentSchema);
  const validate=ajv.compile({$ref:`${documentSchema.$id}#/$defs/DrawingLayerNode`});
  for(const isolation of [false,true]){
    const layer={kind:"group",isolation,children:[]};
    const owned={...layer,id:"group",name:"Group",visible:true,locked:false,opacity:binary64(1),blendMode:"normal",transform:{x:binary64(0),y:binary64(0),scaleX:binary64(1),scaleY:binary64(1),shear:binary64(0),rotation:binary64(0)},attributes:{fillRule:"evenodd"}};
    expect(validate(layer)).toBe(true);const parsed=parseDrawingLayerNode(owned);expect(parsed.kind).toBe("group");if(parsed.kind!=="group")throw new Error("group required");expect(parsed.isolation).toBe(isolation);
    expect(parseDrawingLayerPatch({isolation}).isolation).toBe(isolation);
  }
  for(const isolation of ["true",1,null]){
    const layer={kind:"group",isolation,children:[]};
    expect(validate(layer)).toBe(false);expect(()=>parseDrawingLayerNode(layer)).toThrow();
  }
  expect(()=>parseDrawingLayerPatch({isolation:"true"})).toThrow();
});
