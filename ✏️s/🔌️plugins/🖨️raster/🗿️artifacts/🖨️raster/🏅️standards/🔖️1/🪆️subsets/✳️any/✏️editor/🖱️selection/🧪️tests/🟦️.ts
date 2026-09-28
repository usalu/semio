/** 🖱️ New-layer selection preserves exact identifiers through the framework request wire. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {layerSelectionArgs} from "../🟦️.ts";
const validate=new Ajv().compile(schema);
for(const row of fixture.cases)test("Select created layer "+row.id,()=>{
  const args=layerSelectionArgs(row.id);expect(validate(args)).toBe(true);expect(JSON.parse(args.targets)).toEqual([row.target]);
});
for(const id of fixture.invalid)test("Reject empty selection identity",()=>{expect(()=>layerSelectionArgs(id)).toThrow();});
