/** ✂️ Validate portable fragment input, ray expectations and operation-control cases. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const base=path.resolve(__dirname,'..');
const cases=JSON.parse(fs.readFileSync(path.join(base,'🧪️cases/🔣️.json'),'utf8'));
const schema=JSON.parse(fs.readFileSync(path.join(base,'🔣️.schema.json'),'utf8'));
const validate=new Ajv({strict:true}).compile(schema);
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const mutate of [value=>delete value.numeric_policy,value=>value.numeric_policy.source_planes_only=false,value=>value.cases[0].faces[0].vertices[0]=[0,0],value=>value.cases[0].faces[0].source=-1,value=>value.control.deep_stack_count=2,value=>value.lineage.source_planes_per_input=false,value=>value.lineage.faces.pop()]){
 const invalid=structuredClone(cases);mutate(invalid);assert.equal(validate(invalid),false);
}
console.log('[DEBUG] Seven portable BSP scenarios plus exact four-face lineage accepted; seven malformed contracts rejected.');
