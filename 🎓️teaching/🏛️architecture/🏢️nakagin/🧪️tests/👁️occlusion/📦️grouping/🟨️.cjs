/** 📦 Validate exact translated source-grouping fixtures. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../../../🧪️cases/👁️occlusion/📦️grouping');
const read=name=>JSON.parse(fs.readFileSync(path.join(root,name),'utf8'));
const cases=read('🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const change of [v=>v.cases[0].placements[0].axes[0]=[1],v=>v.cases[0].placements[0].family='Radial',v=>delete v.cases[0].placements[0].origin]){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
