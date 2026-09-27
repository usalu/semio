/** 🎛️ Sparse adjustment parameters preserve explicit reset and schema bounds. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../../🔺️diff/🔣️.json";
import mutationSchema from "../../🧬️mutations/🎛️change-layer-adjustment-parameter/🧬️schema/🔣️.json";
import fixture from "../../🧬️mutations/🎛️change-layer-adjustment-parameter/🧪️tests/🔣️.json";
import {parseRasterLayerPatch} from "../../🔺️diff/🟦️.ts";
const validate=new Ajv({strict:false}).compile(schema.$defs.RasterAdjustmentParameter);
const validateMutation=new Ajv({strict:false}).compile(mutationSchema);
for(const row of fixture.cases)test(`adjustment ${row.parameter} ${row.before} to ${row.after}`,()=>{
  const parameter={parameter:row.parameter,value:row.after};
  expect(validate(parameter)).toBe(true);
  expect(validateMutation({layerId:"tone",parameter:row.parameter,expected:row.before,value:row.after})).toBe(true);
  expect(parseRasterLayerPatch({adjustmentParameter:parameter}).adjustmentParameter).toEqual(parameter);
});
for(const parameter of fixture.invalid)test(`reject invalid adjustment ${JSON.stringify(parameter)}`,()=>{
  expect(validate(parameter)).toBe(false);
  expect(()=>parseRasterLayerPatch({adjustmentParameter:parameter})).toThrow();
});
test("adjustment reset must be explicit",()=>{
  expect(parseRasterLayerPatch({}).adjustmentParameter).toBeUndefined();
  expect(validate({parameter:"brightness"})).toBe(false);
  expect(()=>parseRasterLayerPatch({adjustmentParameter:{parameter:"brightness"}})).toThrow();
});
