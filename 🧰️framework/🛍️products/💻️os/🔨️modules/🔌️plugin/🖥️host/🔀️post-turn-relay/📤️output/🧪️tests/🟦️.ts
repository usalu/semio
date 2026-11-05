import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
test("Source result pages preserve exact byte order until paid capsule return",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const bytes=Buffer.concat(fixture.pages.map(page=>Buffer.from(page.bytes)));expect([...bytes]).toEqual(fixture.bytes);const db=new Database(":memory:");try{expect(db.query("SELECT lower(hex(?)) AS bytes").get(bytes)).toEqual({bytes:fixture.expectedHex});for(const page of fixture.pages)expect(db.query("SELECT lower(hex(substr(?,?,?))) AS bytes").get(bytes,page.offset+1,page.bytes.length)).toEqual({bytes:Buffer.from(page.bytes).toString("hex")});}finally{db.close();}
});

test("fault result retains primary, original receipt and settlement causes independently",()=>{const db=new Database(":memory:");try{const causes=[fixture.fault.primary,fixture.fault.receipt,fixture.fault.settlement];expect(db.query("SELECT count(DISTINCT value) AS count FROM json_each(?)").get(JSON.stringify(causes))).toEqual({count:fixture.fault.expectedCauses});expect(causes.length).toBe(fixture.fault.expectedCauses);}finally{db.close();}});
