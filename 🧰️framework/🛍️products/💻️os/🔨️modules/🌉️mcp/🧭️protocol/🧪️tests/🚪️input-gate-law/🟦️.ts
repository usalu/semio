import{expect,test}from"bun:test";
import Ajv from"ajv";
import document from"../../../🧬️schema/🔣️.json"with{type:"json"};
import cases from"../🧫️fixtures/🚪️input-gate-law/🔣️.json"with{type:"json"};
test("the third-party validator accepts and rejects exactly the inputs the registry gate does",()=>{
 const oracle=new Ajv({strict:false,allErrors:true});
 for(const row of cases){
  const verdict=oracle.validate(document.$defs[row.shape as"CapabilitiesSearchInput"|"UiRevealInput"|"InferenceListInput"],row.input);
  expect(verdict,row.name).toBe(row.valid);
  if(row.valid===false)expect(JSON.stringify(oracle.errors),row.name).toContain(row.mentions);
 }
});
