/** 📋️ Copies retain the source sibling stack and refuse protected destinations. */
import {expect,test} from "bun:test";
import {hierarchy} from "d3-hierarchy";
import {duplicateLayerPlan} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
type LayerTree={readonly id:string;readonly children?:readonly LayerTree[]};
for(const row of fixture.cases)test("Duplicate placement for "+row.id,()=>{
  const before=JSON.stringify(fixture.layers);
  expect(duplicateLayerPlan(fixture.layers,row.id)).toEqual({parentId:row.parentId,index:row.index});
  const tree=hierarchy<LayerTree>({id:"root",children:fixture.layers},node=>node.children);
  const source=tree.descendants().find(node=>node.data.id===row.id)!;
  expect(duplicateLayerPlan(fixture.layers,row.id)).toEqual({parentId:source.parent===tree?null:source.parent!.data.id,index:source.parent!.children!.indexOf(source)+1});
  expect(JSON.stringify(fixture.layers)).toBe(before);
});
for(const row of fixture.invalid)test("Duplicate refuses "+row.id,()=>{expect(()=>duplicateLayerPlan(fixture.layers,row.id)).toThrow(row.error);});
