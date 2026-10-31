/** 🧭 Validate source-confirmed pose records through independent AJV. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const root=path.resolve(__dirname,'../..');
const read=file=>JSON.parse(fs.readFileSync(path.join(root,file),'utf8'));
const data=read('🧪️cases/🧭️poses/🔣️.json');
const schema=read('🧪️cases/🧭️poses/🔣️.schema.json');
const valid=new Ajv({strict:true}).compile(schema);
assert.equal(valid(data),true,JSON.stringify(valid.errors));
for(const [key,value] of [['family','Unknown'],['phase',3],['center',[0,0]],['yaw',45]]){
  const candidate=structuredClone(data);
  candidate.records[0][key]=value;
  assert.equal(valid(candidate),false,key);
}
assert.equal(data.records.length,33);
console.log('Source-pose schema accepted33 confirmed records;4 malformed fixtures rejected.');
