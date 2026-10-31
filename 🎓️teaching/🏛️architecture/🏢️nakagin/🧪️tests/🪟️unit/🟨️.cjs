/** 🪟 Independent schema gate for source window hardware and authored mechanism states. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../..');
const read=file=>JSON.parse(fs.readFileSync(path.join(root,file),'utf8'));
const model=read('🔣️.json');
const cases=read('🧪️cases/🪟️unit/🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🧬️schema/🔣️.schema.json'));
assert.equal(validate(model),true,JSON.stringify(validate.errors));
for(const [key,value] of [['fold_count',1],['default_blind_fraction',1.1],['sash_limit',91],['inner_glass_inset',-.001]]){
  const candidate=structuredClone(model);
  candidate.capsule.window_unit[key].value=value;
  assert.equal(validate(candidate),false,key);
}
for(const key of ['base_diameter','cover_diameter','cover_thickness','sleeve_diameter','divider_thickness','carrier_sheet','angle_side','angle_thickness']){
  assert.equal(model.capsule.window_unit[key].value,cases.source_dimensions[key]);
  assert.equal(model.capsule.window_unit[key].basis,'sourced');
}
assert.equal(model.detail.glazing_thickness.value,cases.source_dimensions.fixed_glass_thickness);
assert.equal(model.capsule.window_unit.inner_glass_thickness.basis,'assumed');
console.log('Window unit schema and primary quantities accepted;4invalid mechanism models rejected.');
