import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json";
import law from"../../../../../🧪️tests/🧫️fixtures/🎟️presented-source/🔣️.json";
test("original accepted semantic tree source declares one paid alias before same-caller return",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);expect(law.acquire).toEqual(law.return);const db=new Database(":memory:");try{expect(db.query(`SELECT json_extract(?, '$.items') AS items, json_extract(?, '$.capacity') AS capacity`).get(JSON.stringify(law.acquire),JSON.stringify(law.return))).toEqual({items:1,capacity:0})}finally{db.close()}
 const native=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(native.includes("pub fn admit_presented_tree_source(")).toBe(true);expect(native.includes("window.presented_interaction_epoch!=loan.identity.interaction")).toBe(true);expect(native.includes("window.presented_source=Some(identity)")).toBe(true);expect(native.includes("window.presented_source=None;loan.active=false")).toBe(true);expect(native.includes("self.windows.values().any(|window|window.presented_source.is_some())")).toBe(true);expect(native.includes("if self.presented_source.is_some(){return false}")).toBe(true);expect(native.includes("window.presented_source.is_some()){return UiSurfaceCloseStep::Pending")).toBe(true);
});
