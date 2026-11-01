/** 🔍 Validate necessary coverage probes without treating samples as a union proof. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../../🧪️cases/👁️occlusion');
const read=name=>JSON.parse(fs.readFileSync(path.join(root,name),'utf8'));
const cases=read('🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const change of [value=>delete value.coverage_witness.expansion,value=>value.coverage_witness.expansion=0,value=>value.coverage_witness.cases[0].footprint[0]=[0],value=>value.coverage_witness.cases[0].possible='true']){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
for(const change of [value=>delete value.single_cover,value=>value.single_cover[0].expected='yes',value=>value.single_cover[0].cover[0]=[0]]){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
for(const change of [value=>delete value.single_cover_bounds,value=>value.single_cover_bounds.cases[0].skip_halfplanes='yes']){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
for(const change of [value=>delete value.overlap_candidates,value=>value.overlap_candidates.cases[0].second[0]=[0],value=>value.overlap_candidates.cases[1].inclusive[0]=[-1,0]]){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
for(const change of [value=>delete value.refinement_progress,value=>value.refinement_progress.phase='silent']){
 const invalid=structuredClone(cases);change(invalid);assert.equal(validate(invalid),false);
}
