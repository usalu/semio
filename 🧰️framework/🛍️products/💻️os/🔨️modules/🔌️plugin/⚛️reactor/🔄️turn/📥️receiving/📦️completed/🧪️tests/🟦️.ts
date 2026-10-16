import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("original Completed uses the same admitted donor and independent correlation/full-Fault oracle",()=>{
 const validate=new Ajv({strict:false}).compile({...schema,$schema:undefined});expect(validate(law)).toBe(true);
 const db=new Database(":memory:");try{const actual=db.query("SELECT json_set(json_remove(?, '$.ok', '$.output', '$.fault', '$.faultCode', '$.faultMessage'), '$.ok', json('false'), '$.output', json('null')) AS value").get(JSON.stringify(law.request)) as {value:string};expect(JSON.parse(actual.value)).toEqual(law.expected);expect(JSON.parse(JSON.stringify(law.fault))).toEqual(law.fault);const joined=db.query("SELECT ? || ? AS value").get(law.parked.partial,law.parked.terminal) as {value:string};expect(joined.value).toBe(law.parked.expected)}finally{db.close()}
 const base=new URL("../",import.meta.url);const receiver=readFileSync(new URL("../🦀️.rs",base),"utf8");const completed=readFileSync(new URL("🦀️.rs",base),"utf8");const fault=readFileSync(new URL("⚠️fault/🦀️.rs",base),"utf8");
 const hook=receiver.slice(receiver.indexOf("pub fn retain_completed"),receiver.indexOf("pub async fn advance_responses"));expect(hook.indexOf("position(Option::is_none)")).toBeLessThan(hook.indexOf("take_continuation(req)"));expect(hook.indexOf("owner.witness.admit(cx)")).toBeLessThan(hook.indexOf("donor.take()"));expect(hook.indexOf("metadata(cx)")).toBeLessThan(hook.indexOf("donor.take()"));expect(hook).not.toContain("ManuallyDrop::new(owner)");
 expect(hook).toContain("take_pending_partial(req)");expect(receiver).toContain("resolve_original(self.request.unwrap(),&mut self.request_result)");expect(completed).toContain("new_request(event:Event,partial:Vec<u8>)");
 expect(completed).toContain("grammar.step_source(&source,1,control,grant,false)");expect(completed).toContain("advance_original_step");expect(completed).toContain("original_close_demands");expect(completed).toContain("admit_original_domain_failure");expect(fault).not.toContain("from_value");expect(fault).not.toContain("clone()");
});
