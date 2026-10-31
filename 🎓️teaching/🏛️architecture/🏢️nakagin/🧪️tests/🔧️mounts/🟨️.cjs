/** 🔧 Validate portable fixture-contact evidence independently with AJV. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const folder=path.resolve(__dirname,'../../🧪️cases/🔧️mounts');
const read=file=>JSON.parse(fs.readFileSync(path.join(folder,file),'utf8'));
const cases=read('🔣️.json');
const validate=new Ajv({strict:true,allErrors:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
assert.equal(new Set(cases.links.map(row=>row.mount)).size,cases.links.length);
for(const mutation of [candidate=>candidate.basis='measured',candidate=>candidate.links[0].contacts=[],candidate=>candidate.minimum_volume=-1]){
  const candidate=structuredClone(cases);
  mutation(candidate);
  assert.equal(validate(candidate),false);
}
console.log('Fixture-contact portable schema accepted; 3 invalid contracts rejected.');
