import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
/** 📥️ Independent JavaScript Wasm and SQL validate the scalar and refusal before original memory mutation is implemented. */
test("original guest memory instruction retains denied address and operands",async()=>{const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const bytes=new Uint8Array(law.moduleHex.match(/../gu).map((hex:string)=>parseInt(hex,16))),oracle={instance:new WebAssembly.Instance(new WebAssembly.Module(bytes))};expect((oracle.instance.exports.invoke as()=>bigint)()).toBe(BigInt(law.expected));const db=new Database(":memory:");try{expect(db.query("SELECT ?1<?2 AS denied,2+1+1+1+1 AS vm").get(law.deniedCopyBytes,law.width)).toEqual({denied:1,vm:law.expectedInstructions})}finally{db.close()}const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("admit_memory_instruction");expect(source).toContain("plan_memory_address");});
