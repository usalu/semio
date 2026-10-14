import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("original fixture completion preserves the rejected source and closes every owner",()=>{
 const validate=new Ajv({allErrors:true,strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const row of fixture.cases){
  const pending={mutations:[{value:7}]};const state:{pending:typeof pending|null;completion:typeof pending|null;fault:string|null}={pending,completion:row.occupied?{mutations:[{value:3}]}:null,fault:null};
  const completed= row.faultCode===null?applyPatch(state,[{op:"move",from:"/pending",path:"/completion"},{op:"add",path:"/pending",value:null}],true,true).newDocument:applyPatch(state,[{op:"replace",path:"/fault",value:row.faultCode}],true,true).newDocument;
  if(row.faultCode===null){expect(completed.completion).toBe(pending);expect(completed.pending).toBeNull()}else{expect(completed.pending).toBe(pending);expect(completed.fault).toBe(row.faultCode)}
  const closed=applyPatch(completed,[{op:"replace",path:"/pending",value:null},{op:"replace",path:"/completion",value:null},{op:"replace",path:"/fault",value:null}],true,true).newDocument;expect(closed).toEqual({pending:null,completion:null,fault:null});
 }
 expect(fixture.terminalDropBytes).toBe(0);expect(fixture.sameRecipient).toBe(true);console.log("[DEBUG] original fixture pending emission source/rejection/close neutral cases=3");
});
