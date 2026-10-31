/** 🧱 Validate source-independent conformal cap coverage cases and reject malformed geometry contracts. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../../🧪️cases/🧱️panel');
const read=p=>JSON.parse(fs.readFileSync(path.join(root,p),'utf8'));
const cases=read('🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const key of ['aperture_radius','edge_incidence','directed_balance']){
 const invalid=structuredClone(cases);invalid[key]+=1;assert.equal(validate(invalid),false,key);
}
console.log('[DEBUG] Conformal panel contract accepted;3invalid physical invariants rejected.');
