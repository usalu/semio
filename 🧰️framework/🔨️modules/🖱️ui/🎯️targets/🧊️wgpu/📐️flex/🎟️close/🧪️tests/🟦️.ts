/** 📏️ Native retained storage depth remains independently funded beneath its original parent. */
import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import{readFileSync}from"node:fs";
import schema from"../🧬️schema/🔣️.json";import fixture from"../../../../../🧪️tests/🧫️fixtures/🧲️flex-close/🔣️.json";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("original layout close quotes the native paged depth without raising its caller",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);expect(valid({...fixture,parentDepth:0})).toBe(false);
 const db=new Database(":memory:");try{for(let i=0;i<fixture.storageDepths.length;i++){expect(db.query("SELECT ?+? AS depth").get(fixture.storageDepths[i],fixture.parentDepth)).toEqual({depth:fixture.quotedDepths[i]})}}finally{db.close()}
 const flex=readModuleTreeSource(new URL("../../🦀️.rs",import.meta.url)),mounted=readModuleTreeSource(new URL("../../../📌️mounted_layout/🦀️.rs",import.meta.url));
 expect(flex.includes("fn next_close_depth_demand")).toBe(true);expect(flex.includes("columns.next_pop_depth_demand()")).toBe(true);expect(flex.includes("columns.next_release_depth_demand()")).toBe(true);expect(flex.includes("grant.maximum_depth<depth")).toBe(true);
 expect(mounted.includes("fn next_close_depth")).toBe(true);expect(mounted.includes("owner.next_pop_depth_demand()")).toBe(true);expect(mounted.includes("owner.next_release_depth_demand()")).toBe(true);expect(mounted.includes("grant.maximum_depth<depth")).toBe(true);
});

test("original owned text cancellation retains its alias until paid witness return",()=>{const source=readModuleTreeSource(new URL("../../../📌️mounted_layout/🦀️.rs",import.meta.url));expect(source.includes("self.cancel_after_shape.as_ref()")).toBe(true);expect(source.includes("cancel_retirement")).toBe(true);expect(source.includes("original.return_alias_step(cx.original_cancel_token(),grant)")).toBe(true);expect(source.includes("self.cancel_after_shape.take() {\n            cancel.cancel_now()")).toBe(false);});
