/** 🧬 Portable fixture dimensions independently checked by strict AJV. */
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const Ajv = require('ajv/dist/2020').default;
const root = path.resolve(__dirname,'../..');
const read = file => JSON.parse(fs.readFileSync(path.join(root,file),'utf8'));
const model = read('🔣️.json');
const cases = read('🧪️cases/💊️detail/🔣️.json');
const validate = new Ajv({strict:true,allErrors:true}).compile(read('🧬️schema/🔣️.schema.json'));
assert.equal(validate(model),true,JSON.stringify(validate.errors));
for (const test of cases.invalid_details) {
  const candidate = structuredClone(model);
  candidate.capsule.detail[test.key].value = test.value;
  assert.equal(validate(candidate),false,test.key);
}
console.log('Capsule detail: authored schema accepted; '+cases.invalid_details.length+' impossible fixture dimensions rejected.');
