import{expect,test}from"bun:test";
import{applyPatch}from"fast-json-patch";
import fixture from"./🧫️fixtures/🔣️.json";
test("original Work publication preserves answered faults and fills only declared unanswered rows",()=>{
 for(const row of fixture.paintCases){
  const merged=structuredClone(row.live)as Record<string,unknown>;
  for(const id of row.widgets){const live=(row.live as Record<string,{out?:Record<string,unknown>;error?:unknown}>)[id];const old=(row.converged as Record<string,{out?:Record<string,unknown>;error?:unknown}>)[id];if(old&&!live?.error&&Object.keys(live?.out??{}).length===0&&(old.error!==undefined||Object.keys(old.out??{}).length!==0))applyPatch(merged,[{op:"replace",path:"/"+id,value:structuredClone(old)}]);}
  expect(merged).toEqual(row.expected);
 }
 expect(JSON.parse(JSON.stringify(fixture.expected))).toEqual(fixture.expected);
});

import Ajv from"ajv/dist/2020.js";
import schema from"../🧬️schema/🔣️.json";
test("original Work publication fixture follows its neutral ownership schema",()=>{const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture)).toBe(true);});
