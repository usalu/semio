/** 📦 Validate source-bearing physical casing and reject unsupported gap compensation. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../..');
const read=file=>JSON.parse(fs.readFileSync(path.join(root,file),'utf8'));
const data=read('🔣️.json');
const schema=read('🧬️schema/🔣️.schema.json');
const cases=read('🧪️cases/📦️casing/🔣️.json');
const validate=new Ajv({strict:true}).compile(schema);
assert.equal(validate(data),true,JSON.stringify(validate.errors));
assert.deepEqual(['width','length','height'].map(key=>data.capsule.casing[key].value),cases.dimensions);
for(const key of ['width','length','height']){
  const candidate=structuredClone(data);
  candidate.capsule.casing[key].value=-1;
  assert.equal(validate(candidate),false,key);
}
const missing=structuredClone(data);
delete missing.capsule.casing;
assert.equal(validate(missing),false);
const collar=structuredClone(data);
collar.families[0].entry_standoff={value:.1,unit:'m',basis:'assumed'};
assert.equal(validate(collar),false);
console.log('Physical casing schema accepted;5invalid casing/gap contracts rejected.');
