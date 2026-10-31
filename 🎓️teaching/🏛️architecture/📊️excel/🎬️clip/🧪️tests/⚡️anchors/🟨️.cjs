/** ⚡ Independently validate physical leader coordinates against source quantities and the portable schema. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const clip=path.resolve(__dirname,'../..');
const read=p=>JSON.parse(fs.readFileSync(path.join(clip,p),'utf8'));
const cases=read('🧪️cases/⚡️anchors/🔣️.json');
const validate=new Ajv({strict:true}).compile(read('🧪️cases/⚡️anchors/🔣️.schema.json'));
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
const model=JSON.parse(fs.readFileSync(path.resolve(clip,'../../🏢️nakagin/🔣️.json'),'utf8'));
const physical=model.capsule.casing,nominal=model.capsule;
const near=(a,b)=>assert.ok(Math.abs(a-b)<cases.tolerance,a+' != '+b);
near(cases.reference_points.left[0],-physical.width.value/2);
near(cases.reference_points.right[0],physical.width.value/2);
near(cases.reference_points.front[1],(nominal.length.value+physical.length.value)/2);
near(cases.reference_points.back[1],(nominal.length.value-physical.length.value)/2);
near(cases.reference_points.window[1],cases.reference_points.front[1]-physical.glazing_recess_front.value);
assert.deepEqual(cases.nominal_energy_dimensions,['width','length','height'].map(k=>nominal[k].value));
for(const mutation of ['short-point','unknown-family','negative-tolerance']){
 const bad=structuredClone(cases);
 if(mutation==='short-point')bad.reference_points.left=[-1.3,2];
 if(mutation==='unknown-family')bad.families[0]='Unknown';
 if(mutation==='negative-tolerance')bad.tolerance=-1;
 assert.equal(validate(bad),false,mutation);
}
console.log('[DEBUG] Energy physical anchors match source dimensions/recess;3invalid portable contracts rejected.');
