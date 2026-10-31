/** ✂️ Validate exact cutaway source-material contracts. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../../../🧪️cases/🎨️visuals/✂️cutaway');
const read=name=>JSON.parse(fs.readFileSync(path.join(root,name),'utf8'));
const cases=read('🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const mutate of [value=>delete value.cases[0].planes,value=>value.cases[0].normal=[1,0],value=>value.cases[0].area=-1,value=>value.cases[0].unknown=true,value=>delete value.boundary_identity.positive_limit,value=>value.boundary_identity.ulp_multiplier=128,value=>value.boundary_identity.contacts[0].normal=[1,0]]){
 const invalid=structuredClone(cases);mutate(invalid);assert.equal(validate(invalid),false);
}
console.log('[DEBUG] Six cutaway contracts and boundary identity accepted; seven malformed contracts rejected.');
