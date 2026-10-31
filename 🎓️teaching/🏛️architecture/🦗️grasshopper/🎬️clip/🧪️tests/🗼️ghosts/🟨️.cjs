/** 👻 Validate the language-neutral rejected-geometry fixture with AJV. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const base=path.resolve(__dirname,'../../🧪️cases/🗼️ghosts');
const read=name=>JSON.parse(fs.readFileSync(path.join(base,name),'utf8'));
const fixture=read('🔣️.json');
const validate=new Ajv({strict:true,allErrors:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(fixture),true,JSON.stringify(validate.errors));
assert.equal(new Set(fixture.rows.map(row=>row.storey_index+':'+row.index)).size,8);
for(const mutate of [candidate=>candidate.rows[0].family='Z',candidate=>candidate.rows[0].index=8,candidate=>delete candidate.rows[0].type]){
  const candidate=structuredClone(fixture);
  mutate(candidate);
  assert.equal(validate(candidate),false);
}
console.log('Tower ghost schema accepted; 3 malformed physical-tag cases rejected.');
