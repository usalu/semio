/** ✂️ Validate portable source-facet and fragment-rendering contracts. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../../../🧪️cases/👁️occlusion/✂️fragments');
const read=name=>JSON.parse(fs.readFileSync(path.join(root,name),'utf8'));
const cases=read('🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const change of [value=>delete value.warped_quad,value=>value.transparent.items[0].opacity=1.1,value=>value.warped_quad.triangles[0][0]=4,value=>value.cases[0].items[0].polygons[0][0]=[0],value=>value.compound.unknown=true,value=>delete value.plane_lineage,value=>value.plane_lineage.normal=[1,0],value=>value.plane_lineage.epsilon=0]){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
console.log('[DEBUG] Fragment contracts accepted; eight malformed cases rejected.');
