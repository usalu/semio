import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json";
import law from"./🧫️fixtures/🔣️.json";
test("original slot canonical identity and same retained output have independent token oracle",()=>{
 expect(new Ajv({strict:true,strictTuples:false}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{for(const row of law.cases){const wire=JSON.stringify(row.identity);expect(createHash("sha256").update(wire).digest("hex")).toBe(row.token);expect(db.query("SELECT json_array_length(?) AS arity, json_extract(?, '$[1]') AS node").get(wire,wire)).toEqual({arity:8,node:row.identity[1]});}}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source.includes("scalar:source.scalar()")).toBe(true);expect(source.includes("self.scalar!=source.scalar()")).toBe(true);expect(source.includes("JsonBorrowedWriteCursor")).toBe(true);expect(source.includes("NativeEncodeControl::resume")).toBe(true);expect(source.includes("self.payload.take_original()")).toBe(true);expect(source.includes("props:props.take().unwrap()")).toBe(true);expect(source.includes("close_original_retained_step(cx,demand")).toBe(true);
 console.log("[DEBUG] original slot neutral Ajv/SQLite/crypto token oracle; paid native collector/Browser/GPU claims require actual runtime replay");
});
