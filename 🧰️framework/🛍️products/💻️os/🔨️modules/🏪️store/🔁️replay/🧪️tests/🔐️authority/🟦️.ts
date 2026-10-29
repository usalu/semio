import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import jsonpatch from "fast-json-patch";
const fixture=JSON.parse(await readFile(new URL("🔣️.json",import.meta.url),"utf8"));
assert.equal(fixture.schema,"semio.replay-authority.v1");
let cases=0;
for(const row of fixture.cases){
 assert.equal(row.producerInstalled,false);
 const source={coldAuthority:row.authority==="cold",state:structuredClone(row.state)};
 let state=source.state,accepted=false,error:string|null=null;
 try{
  state=jsonpatch.applyPatch(source,[
   {op:"test",path:"/coldAuthority",value:true},
   {op:"replace",path:"/state/counter",value:row.nextCounter},
   {op:"replace",path:"/state/calls",value:row.state.calls+1}
  ],true,false).newDocument.state;
  accepted=true;
 }catch(refusal){
  assert.equal((refusal as {name:string}).name,"TEST_OPERATION_FAILED");
  error="UnsupportedOwner";
 }
 const actual={accepted,unchanged:JSON.stringify(state)===JSON.stringify(row.state),error,state};
 assert.deepEqual(actual,row.expected);
 console.log(JSON.stringify({name:row.name,actual}));
 cases++;
}
assert.equal(cases,2);
console.log(JSON.stringify({oracle:"fast-json-patch",cases,status:"passed"}));
